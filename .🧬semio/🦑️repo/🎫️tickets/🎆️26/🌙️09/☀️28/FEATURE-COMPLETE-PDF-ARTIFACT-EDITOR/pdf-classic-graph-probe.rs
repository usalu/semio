//! 🧪️ Checks the staged graph quote, canonical ordinary emission and sparse xref.
use writer::classic_graph_quote::{GraphLoan, GraphOverlay, GraphQuote, GraphRefusal, MAX_PREFIX, QUOTE_WORK};
use writer::classic_graph_writer::{ClassicGraphWriter, WriteRefusal};

struct CountingSink<W> { writer: W, remaining: usize, written: usize }
impl<W: Write> Write for CountingSink<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 { return Err(io::ErrorKind::WouldBlock.into()); }
        let count = self.writer.write(&bytes[..bytes.len().min(self.remaining)])?;
        self.remaining -= count;
        self.written += count;
        Ok(count)
    }
    fn flush(&mut self) -> io::Result<()> { self.writer.flush() }
}

fn run() {
    let assets = std::env::args_os().nth(1).unwrap();
    let assets = Path::new(&assets);
    let stream = b"BT /Fraw 16 Tf 18 48 Td (Pinned ordinary graph) Tj ET";
    let nested = dict(&[("Keys", PdfObject::Array(vec![PdfObject::Str(b"binary\0".to_vec()), PdfObject::Ref(ObjRef { num: 17, gen: 3 })]))]);
    let mut document = vec![
        object(1, dict(&[("Type", PdfObject::name("Catalog")), ("Pages", pdf_ref(2))])),
        object(2, dict(&[("Type", PdfObject::name("Pages")), ("Kids", PdfObject::Array(vec![pdf_ref(3)])), ("Count", PdfObject::Int(1))])),
        object(3, dict(&[
            ("Type", PdfObject::name("Page")), ("Parent", pdf_ref(2)),
            ("MediaBox", PdfObject::Array(vec![0, 0, 210, 90].into_iter().map(PdfObject::Int).collect())),
            ("Resources", dict(&[("Font", dict(&[("Fraw", pdf_ref(5))]))])),
            ("Contents", pdf_ref(4)),
        ])),
        object(4, PdfObject::Stream { dict: vec![PdfDictEntry::new("Length", PdfObject::Int(3))], data: stream.to_vec(), filters: Vec::new() }),
        object(5, dict(&[("Type", PdfObject::name("Font")), ("Subtype", PdfObject::name("Type1")), ("BaseFont", PdfObject::name("Helvetica"))])),
        object(8, dict(&[("Title", PdfObject::Str(b"Edited title".to_vec()))])),
        object(11, dict(&[("Nested", nested), ("Escaped#Key", PdfObject::Str(b"()\\payload".to_vec()))])),
        PdfIndirectObject { id: ObjRef { num: 17, gen: 3 }, value: PdfObject::Stream {
            dict: vec![PdfDictEntry::new("Type", PdfObject::name("EmbeddedFile"))],
            data: (0..2_097_177u32).map(|index| (index % 251) as u8).collect(),
            filters: Vec::new(),
        }},
    ];
    let trailer = DocumentTrailer {
        root: id(1), info: Some(id(8)), id: Some([vec![0, 1, 2], b"stable".to_vec()]), extra: vec![
            PdfDictEntry::new("Private", pdf_ref(11)), PdfDictEntry::new("Prev", PdfObject::Int(7)),
        ],
    };
    let loan = GraphLoan { version: "1.7", objects: &document, trailer: &trailer, overlay: GraphOverlay::default() };
    let mut quote = GraphQuote::start(loan, 3_000_000).unwrap();
    assert!(!quote.next(loan, QUOTE_WORK - 1).unwrap());
    for index in 0..document.len() {
        assert!(!quote.next(loan, QUOTE_WORK).unwrap(), "quote stopped at {index}");
    }
    assert!(quote.next(loan, QUOTE_WORK).unwrap());
    let extent = quote.maximum().unwrap();
    let replacement = DocumentTrailer { root: id(1), info: None, id: None, extra: Vec::new() };
    let mut emitter = ClassicGraphWriter::start(&quote, Vec::with_capacity(MAX_PREFIX)).unwrap();
    assert!(matches!(emitter.step(GraphLoan { trailer: &replacement, ..loan }, &mut io::sink(), MAX_PREFIX),
        Err(WriteRefusal::Graph(GraphRefusal::SourceChanged))));
    let bytes = emit(emitter, loan, &assets.join("graph.pdf"));
    assert!(bytes < extent);
    let oracle = writer::serialize_document(loan.version, loan.objects, loan.trailer, &WriteOptions::default());
    assert_eq!(oracle, fs::read(assets.join("graph.pdf")).unwrap());
    graft_fixture(loan, assets, false, false);
    graft_fixture(loan, assets, true, false);
    graft_fixture(loan, assets, false, true);
    let body_tail = loan.trailer.extra.len();
    assert_eq!(body_tail, 2);
    let mut narrow = GraphQuote::start(loan, 1_500_000).unwrap();
    assert!(loop {
        match narrow.next(loan, QUOTE_WORK) {
            Err(GraphRefusal::OutputExtent) => break true,
            Ok(false) => continue,
            _ => break false,
        }
    });
    let PdfObject::Stream { filters, .. } = &mut document[7].value else { unreachable!() };
    filters.push(PdfStreamFilter::Flate { predictor: None });
    document.push(object(19, PdfObject::Stream { dict: Vec::new(), data: Vec::new(), filters: vec![PdfStreamFilter::Flate { predictor: None }] }));
    let flate = GraphLoan { version: "1.7", objects: &document, trailer: &trailer, overlay: GraphOverlay::default() };
    let mut quote = GraphQuote::start(flate, 3_000_000).unwrap();
    while !quote.next(flate, QUOTE_WORK).unwrap() {}
    let writer = ClassicGraphWriter::start(&quote, Vec::with_capacity(MAX_PREFIX)).unwrap();
    assert!(emit(writer, flate, &assets.join("graph-flate.pdf")) < quote.maximum().unwrap());
    let PdfObject::Stream { filters, .. } = &mut document[7].value else { unreachable!() };
    filters.push(PdfStreamFilter::AsciiHex);
    let changed = GraphLoan { version: "1.7", objects: &document, trailer: &trailer, overlay: GraphOverlay::default() };
    let mut quote = GraphQuote::start(changed, 4_000_000).unwrap();
    assert!(loop {
        match quote.next(changed, QUOTE_WORK) {
            Err(GraphRefusal::UnsupportedStream) => break true,
            Ok(false) => continue,
            _ => break false,
        }
    });
    println!("[DEBUG] Classic graph emitted exact {} bytes below {}-byte quote", bytes, extent);
}

fn emit(mut emitter: ClassicGraphWriter, loan: GraphLoan<'_>, output_file: &Path) -> usize {
    let file = File::create(output_file).unwrap();
    let mut output = CountingSink { writer: file, remaining: 127, written: 0 };
    for attempt in 0..100_000 {
        output.remaining = if attempt % 41 == 4 { 0 } else { 127 };
        let turn = match emitter.step(loan, &mut output, MAX_PREFIX) {
            Ok(turn) => turn,
            Err(WriteRefusal::Sink(error)) if error.kind() == io::ErrorKind::WouldBlock => continue,
            Err(error) => panic!("{error:?}"),
        };
        assert!(turn.prepared <= MAX_PREFIX && turn.emitted <= 127);
        if turn.done { break; }
        assert!(attempt < 99_999);
    }
    assert_eq!(emitter.into_buffer().capacity(), MAX_PREFIX);
    output.written
}

fn main() { run(); }

fn graft_fixture(loan: GraphLoan<'_>, assets: &Path, base_without_info: bool, clear: bool) {
    use ordinary_graft::{GRAFT_WORK, OrdinaryGraft};
    let mut control = loan.trailer.clone();
    if base_without_info { control.info = None; }
    let draft = GraphLoan { trailer: &control, ..loan };
    let snapshot = PdfSnapshot {
        pages: vec![PdfPage { rotate: 90 }],
        info: PdfInfo { title: (!clear).then(|| "Paid updated title".to_owned()), ..Default::default() },
    };
    let grant = lexer::CosGrant { work: GRAFT_WORK, allocation_bytes: 8192, release_bytes: 8192 };
    let mut graft = OrdinaryGraft::default();
    assert!(!graft.step(draft, &snapshot, grant, 8192).unwrap().ready);
    let tight = lexer::CosGrant { allocation_bytes: 0, ..grant };
    assert!(graft.step(draft, &snapshot, tight, 0).unwrap().needs.is_some());
    let page_step = graft.step(draft, &snapshot, grant, 8192).unwrap();
    assert!(!page_step.ready && (1..8192).contains(&page_step.allocated));
    if !clear {
        assert!(graft.step(draft, &snapshot, tight, 0).unwrap().needs.is_some());
    }
    let info_step = graft.step(draft, &snapshot, grant, 8192).unwrap();
    assert!(info_step.ready && info_step.allocated < 8192);
    let rows = graft.into_rows().ok().unwrap();
    control.info = rows.info_ref;
    let output = GraphLoan { trailer: &control, overlay: rows.overlay(), ..loan };
    assert!(std::ptr::eq(output.lookup(ObjRef { num: 17, gen: 3 }).unwrap(), loan.lookup(ObjRef { num: 17, gen: 3 }).unwrap()));
    assert!(std::ptr::eq(output.lookup(id(11)).unwrap(), loan.lookup(id(11)).unwrap()));
    if clear {
        assert!(output.lookup(id(8)).is_none());
        let mut reused = control.clone();
        reused.extra.push(PdfDictEntry::new("PrivateInfo", pdf_ref(8)));
        let still_referenced = GraphLoan { trailer: &reused, ..output };
        let mut quote = GraphQuote::start(still_referenced, 3_000_000).unwrap();
        loop {
            match quote.next(still_referenced, QUOTE_WORK) {
                Err(GraphRefusal::MissingReference) => break,
                Ok(false) => continue,
                _ => panic!("graft with a still-referenced Info object must not emit"),
            }
        }
    }
    let mut quote = GraphQuote::start(output, 3_000_000).unwrap();
    while !quote.next(output, QUOTE_WORK).unwrap() {}
    let writer = ClassicGraphWriter::start(&quote, Vec::with_capacity(MAX_PREFIX)).unwrap();
    let name = if clear { "graph-graft-clear.pdf" } else if base_without_info { "graph-graft-add.pdf" } else { "graph-graft-replace.pdf" };
    assert!(emit(writer, output, &assets.join(name)) < quote.maximum().unwrap());
}
