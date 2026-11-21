use super::*;
use output_stage::{TURN, caller, close_construction, finish, page, retire};
use semio_framework_job::{OriginalJobDecodeOwner, OriginalJobFixtureCaller};
use semio_plugin_decl_app::media_contract::MediaConstruction;
use semio_plugin_decl_media::ArtifactOutputChunks;

struct OutputDriver {
    caller: OriginalJobFixtureCaller,
    decoder: OriginalJobDecodeOwner,
    chunks: ArtifactOutputChunks,
    construction: MediaConstruction,
    page: Option<Vec<u8>>,
}

impl OutputDriver {
    fn new(maximum: usize) -> Self {
        let mut caller = caller();
        let mut decoder = caller.admitted_decoder();
        caller.retain_dispatch().unwrap();
        let mut chunks = None;
        assert!(caller.turn_with_decoder(TURN, &mut decoder, |cx| ArtifactOutputChunks::original_birth_admit(maximum, &mut chunks, cx)).unwrap());
        Self { caller, decoder, chunks: chunks.unwrap(), construction: MediaConstruction::default(), page: None }
    }

    fn lend<R>(&mut self, write: impl FnOnce(&mut dyn Write, usize) -> R) -> R {
        if self.page.is_none() {
            for attempt in 0..64 {
                let ready = self.caller.turn_with_decoder(TURN, &mut self.decoder, |cx| page::admit_pdf_save_page(&mut self.page, &mut self.construction, cx)).unwrap();
                if ready { break; }
                assert!(attempt < 63);
            }
        }
        let page = self.page.as_mut().unwrap();
        let result = self.caller.turn_with_decoder(TURN, &mut self.decoder, |cx| page::write_pdf_save_turn(page, cx, write)).unwrap().unwrap();
        if page.len() == ArtifactOutputChunks::CHUNK_BYTES { self.push(false); }
        result
    }

    fn push(&mut self, finished: bool) {
        assert!(self.caller.turn_with_decoder(TURN, &mut self.decoder, |cx| page::donate_pdf_save_page(&mut self.page, finished, &self.chunks, None, cx)).unwrap());
        assert!(self.page.is_none());
    }

    fn bytes(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let copied = self.lend(|sink, _| sink.write(bytes)).unwrap();
            assert_ne!(copied, 0);
            bytes = &bytes[copied..];
        }
    }

    fn deliver(mut self, mut destination: impl Write) -> std::io::Result<usize> {
        if self.page.is_some() { self.push(true); }
        assert!(self.chunks.take_chunk().is_err());
        assert!(self.caller.turn_with_decoder(TURN, &mut self.decoder, |cx| self.chunks.seal_original(cx)).unwrap());
        let mut count = 0;
        let mut terminal = false;
        while !terminal {
            let mut chunk = None;
            assert!(self.caller.turn_with_decoder(TURN, &mut self.decoder, |cx| self.chunks.take_chunk_original(&mut chunk, &mut terminal, cx)).unwrap());
            if let Some(chunk) = chunk {
                destination.write_all(&chunk)?;
                count += chunk.len();
                retire(&mut self.caller, &mut self.decoder, chunk);
            }
        }
        assert_eq!(self.chunks.bytes(), count);
        close_construction(&mut self.caller, &mut self.decoder, &mut self.construction);
        retire(&mut self.caller, &mut self.decoder, self.chunks);
        finish(self.caller, &mut self.decoder);
        Ok(count)
    }
}

#[test]
fn writes_a_native_pdf_into_the_actual_segmented_save_queue() {
    let fixture = std::env::var_os("SEMIO_PDF_SAVE_PAGE_FIXTURE").unwrap();
    let expected = Path::new(&fixture);
    let dir = expected.parent().unwrap();
    let jpeg = File::open(dir.join("large.jpg")).unwrap();
    let length = jpeg.metadata().unwrap().len() as usize;
    let source = FileSource { file: Mutex::new(jpeg), length };
    let color = PdfDctImageColor::Rgb { transform: Some(true) };
    let mut identity = PdfDctResourceIdentity::new(color.color_transform()).unwrap();
    let mut bytes = [0; PAGED_BYTES_CHUNK_BYTES];
    while !identity.complete(length) {
        let offset = identity.position();
        let count = (length - offset).min(bytes.len());
        assert_eq!(source.read_bytes(offset, &mut bytes[..count]), count);
        identity.append(&bytes[..count]);
    }
    let view = PdfDctImageView::from_admitted(&source, (1024, 1024), color, identity.digest()).unwrap();
    let total = expected.metadata().unwrap().len() as usize;
    let mut output = OutputDriver::new(total);
    let (mut writer, header) = PdfWriter::begin("1.7", &WriteOptions::default(), &[], &[]);
    output.bytes(&header);
    let objects = [
        object(1, dict(&[("Type", PdfObject::name("Catalog")), ("Pages", pdf_ref(2))])),
        object(2, dict(&[("Type", PdfObject::name("Pages")), ("Kids", PdfObject::Array(vec![pdf_ref(3)])), ("Count", PdfObject::Int(1))])),
        object(3, dict(&[
            ("Type", PdfObject::name("Page")), ("Parent", pdf_ref(2)),
            ("MediaBox", PdfObject::Array(vec![0, 0, 1024, 1024].into_iter().map(PdfObject::Int).collect())),
            ("Resources", dict(&[("XObject", dict(&[("Im", pdf_ref(5))]))])),
            ("Contents", pdf_ref(4)),
        ])),
        object(4, PdfObject::Stream { dict: Vec::new(), data: b"q 1024 0 0 1024 0 0 cm /Im Do Q".to_vec(), filters: Vec::new() }),
    ];
    for object in objects { output.bytes(&writer.object(&object)); }
    let mut image = writer.begin_dct_object(id(5), view).ok().unwrap();
    for attempt in 0..2000 {
        let turn = output.lend(|sink, _| image.step(view, sink, 8192));
        let done = turn.result.unwrap();
        assert!(turn.counts.read <= 4096 && turn.counts.written <= 4096);
        if done { break; }
        assert!(attempt < 1999);
    }
    let mut writer = image.finish().ok().unwrap();
    output.bytes(&writer.object(&object(6, dict(&[("Producer", PdfObject::Str(b"Bounded PDF writer probe".to_vec()))]))));
    output.bytes(&writer.finish(&DocumentTrailer { root: id(1), info: Some(id(6)), id: None, extra: Vec::new() }));
    let actual = expected.with_extension("segmented.pdf");
    let out = File::create(&actual).unwrap();
    assert_eq!(output.deliver(out).unwrap(), total);
    let mut actual = std::io::BufReader::new(File::open(actual).unwrap());
    let mut expected_pdf = std::io::BufReader::new(File::open(expected).unwrap());
    let mut checked = [0; PAGED_BYTES_CHUNK_BYTES];
    for offset in (0..total).step_by(bytes.len()) {
        let count = (total - offset).min(bytes.len());
        actual.read_exact(&mut bytes[..count]).unwrap();
        expected_pdf.read_exact(&mut checked[..count]).unwrap();
        assert_eq!(&bytes[..count], &checked[..count]);
    }
    println!("[DEBUG] Native stream writer produced the exact {total}-byte reference through the mounted Save output queue");
}
