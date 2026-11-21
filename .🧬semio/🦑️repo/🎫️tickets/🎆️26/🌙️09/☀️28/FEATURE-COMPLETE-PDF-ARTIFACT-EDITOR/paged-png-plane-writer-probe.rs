//! 🔬️ Drives the first-party PDF writer from two independently paged decoded sample planes.
use plane_identity::{PlaneIdentity, PlaneRole, PlaneShape, PngPlanes};
use probe_paged_value::{RetainedCloneGrant, paged::PagedBytes};
use writer::plane_object::PlaneObjectWriter;
use std::path::PathBuf;

const CAPACITY: usize = 4 * 1024 * 1024;
const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 8192, maximum_release_bytes: 0, maximum_depth: 64,
};

fn paged_file(path: PathBuf) -> PagedBytes<CAPACITY> {
    let mut source = File::open(path).unwrap();
    let mut page = [0; 4096];
    let mut bytes = PagedBytes::default();
    loop {
        let count = source.read(&mut page).unwrap();
        if count == 0 { return bytes; }
        for byte in &page[..count] {
            while !bytes.retained_bytes().has_reserved_slot() { bytes.reserve_byte_backing(GRANT).unwrap(); }
            bytes.push_reserved_byte(*byte).unwrap();
        }
    }
}

fn write_short(sink: &mut ShortSink, mut bytes: &[u8]) {
    while !bytes.is_empty() {
        match sink.write(bytes) {
            Ok(count) => bytes = &bytes[count..],
            Err(error) if matches!(error.kind(), io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock) => {},
            Err(error) => panic!("{error}"),
        }
    }
}

fn stream(mut emitter: PlaneObjectWriter, source: &dyn ByteReadSource, sink: &mut ShortSink) -> PdfWriter {
    let mut at = 0;
    let mut page = [0; 4096];
    while at < source.byte_len() {
        let count = page.len().min(source.byte_len() - at);
        assert_eq!(source.read_bytes(at, &mut page[..count]), count);
        let mut fragment = &page[..count];
        while !fragment.is_empty() {
            assert!(!emitter.complete());
            match emitter.step(fragment, sink, 911) {
                Ok(count) => fragment = &fragment[count..],
                Err(error) if matches!(error.kind(), io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock) => {},
                Err(error) => panic!("{error}"),
            }
        }
        at += count;
    }
    while !emitter.complete() {
        match emitter.step(&[], sink, 911) {
            Ok(count) => assert_eq!(count, 0),
            Err(error) if matches!(error.kind(), io::ErrorKind::Interrupted | io::ErrorKind::WouldBlock) => {},
            Err(error) => panic!("{error}"),
        }
    }
    emitter.finish().ok().unwrap()
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);
    let color = paged_file(input.join("paid-rgba.color"));
    let alpha = paged_file(input.join("paid-rgba.alpha"));
    let shape = PlaneShape::new(512, 512, 3, true).unwrap();
    let view = PngPlanes::new(shape, &color, &alpha).unwrap();
    assert!(PngPlanes::new(shape, &alpha, &color).is_none());
    let mut identity = PlaneIdentity::new(shape);
    let mut block = [0; 4096];
    assert!(identity.push(PlaneRole::Alpha, &[0]).is_none());
    while identity.position().is_some() {
        assert!((1..=1513).contains(&identity.read_one(view, &mut block, 1513).unwrap()));
    }
    let facts = identity.finish().unwrap();
    let color_key = facts.reference(PlaneRole::Color).unwrap();
    let mask_key = facts.reference(PlaneRole::Alpha).unwrap();
    assert!(facts.matches(PlaneRole::Color, &color_key));
    assert!(facts.matches(PlaneRole::Alpha, &mask_key));
    assert!(!facts.matches(PlaneRole::Alpha, &color_key));
    assert_eq!(plane_identity::PlaneFacts::decode(facts.encode()), Some(facts));
    fs::write(output.join("pair.keys"), format!("{}\n{}\n", color_key.artifact_id, mask_key.artifact_id)).unwrap();

    let (mut writer, header) = PdfWriter::begin("1.7", &WriteOptions::default(), &[], &[]);
    let mut sink = ShortSink { output: File::create(output.join("planes.pdf")).unwrap(), calls: 0, written: 0 };
    write_short(&mut sink, &header);
    let objects = [
        object(1, dict(&[("Type", PdfObject::name("Catalog")), ("Pages", pdf_ref(2))])),
        object(2, dict(&[("Type", PdfObject::name("Pages")), ("Kids", PdfObject::Array(vec![pdf_ref(3)])), ("Count", PdfObject::Int(1))])),
        object(3, dict(&[
            ("Type", PdfObject::name("Page")), ("Parent", pdf_ref(2)),
            ("MediaBox", PdfObject::Array(vec![0, 0, 512, 512].into_iter().map(PdfObject::Int).collect())),
            ("Resources", dict(&[("XObject", dict(&[("Color", pdf_ref(5))]))])),
            ("Contents", pdf_ref(4)),
        ])),
        object(4, PdfObject::Stream { dict: Vec::new(), data: b"q 512 0 0 512 0 0 cm /Color Do Q".to_vec(), filters: Vec::new() }),
    ];
    for object in objects { write_short(&mut sink, &writer.object(&object)); }
    let mask = ObjRef { num: 6, gen: 9 };
    let color = writer.begin_plane_object(id(5), shape, PlaneRole::Color, Some(mask)).ok().unwrap();
    let writer = stream(color, view.loan(PlaneRole::Color), &mut sink);
    let alpha = writer.begin_plane_object(mask, shape, PlaneRole::Alpha, None).ok().unwrap();
    let writer = stream(alpha, view.loan(PlaneRole::Alpha), &mut sink);
    write_short(&mut sink, &writer.finish(&DocumentTrailer { root: id(1), info: None, id: None, extra: Vec::new() }));
    println!("[DEBUG] Classic PDF wrote {} bytes with paged color and opacity XObjects and preserved SMask generation", sink.written);
}
