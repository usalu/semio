use std::{fs::File, io::{Read, Seek, SeekFrom}, sync::Mutex};
use xref::{GraphImportCursor, GraphImportEvent, GraphImportReceipt, ImportedGraphRetirement, PdfByteGraphCursor, PdfByteGraphEvent, PdfByteGraphReceipt, XrefIndex, XrefRow, XrefStreamError};

const IMAGE_SIDE: usize = $IMAGE_SIDE;
const IMAGE_BYTES: usize = IMAGE_SIDE * IMAGE_SIDE;
const AGGREGATE_LIMIT: usize = 10 * 1024 * 1024;
const PAID: CosGrant = CosGrant { work: 4096, allocation_bytes: AGGREGATE_LIMIT, release_bytes: AGGREGATE_LIMIT };

fn checked(receipt: GraphImportReceipt, held: &mut usize, peak: &mut usize, grant: CosGrant) -> Result<GraphImportEvent, XrefStreamError> {
    assert!(receipt.work <= grant.work);
    assert!(receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
    *held += receipt.allocated_bytes;
    *peak = (*peak).max(*held);
    *held = held.checked_sub(receipt.released_bytes).unwrap();
    receipt.event
}

fn close(cursor: &mut GraphImportCursor, held: &mut usize, peak: &mut usize) {
    cursor.begin_close();
    assert!((0..512).any(|_| checked(cursor.close_step(PAID), held, peak, PAID).unwrap() == GraphImportEvent::Closed));
    assert!(cursor.take().is_none());
    assert_eq!(*held, 0);
}

fn checked_file(receipt: PdfByteGraphReceipt, held: &mut usize, peak: &mut usize, grant: CosGrant) -> Result<PdfByteGraphEvent, XrefStreamError> {
    assert!(receipt.work <= grant.work);
    assert!(receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
    *held += receipt.allocated_bytes;
    *peak = (*peak).max(*held);
    *held = held.checked_sub(receipt.released_bytes).unwrap();
    receipt.event
}

fn close_file(cursor: &mut PdfByteGraphCursor, held: &mut usize, peak: &mut usize) {
    cursor.begin_close();
    assert!((0..512).any(|_| checked_file(cursor.close_step(PAID), held, peak, PAID).unwrap() == PdfByteGraphEvent::Closed));
    assert!(cursor.take(PAID).is_none());
    assert_eq!(*held, 0);
}

fn large_source(path: &std::ffi::OsStr, maximum: usize, aggregate: usize, cancel: bool) {
    let file = File::open(path).unwrap();
    let len = usize::try_from(file.metadata().unwrap().len()).unwrap();
    assert_eq!(len, $SOURCE_BYTES);
    let source = Segmented { len, width: 4096, window: Mutex::new(Window {
        file, start: usize::MAX, bytes: [0; 4096], physical_reads: 0,
    }) };
    let pdf = ByteReadRange::new(&source, PREFIX, len).unwrap();
    assert!(pdf.contiguous_bytes().is_none());
    let mut cursor = PdfByteGraphCursor::new(5, maximum);
    let mut held = 0;
    let mut peak = 0;
    let success = (0..100_000).find_map(|_| {
        let grant = CosGrant { allocation_bytes: aggregate - held, ..PAID };
        let original: &dyn ByteReadSource = if aggregate == AGGREGATE_LIMIT { &source } else { &pdf };
        let event = checked_file(cursor.advance(original, grant, false), &mut held, &mut peak, grant);
        assert!(held <= aggregate && peak <= aggregate);
        match event {
            Ok(PdfByteGraphEvent::More | PdfByteGraphEvent::Object)
                if cancel && source.window.lock().unwrap().physical_reads > 131_072 => Some(false),
            Ok(PdfByteGraphEvent::More | PdfByteGraphEvent::Object) => None,
            Ok(PdfByteGraphEvent::Complete) => {
                assert!(!cancel && maximum >= IMAGE_BYTES && aggregate == AGGREGATE_LIMIT);
                Some(true)
            }
            Ok(PdfByteGraphEvent::NeedsAllocation { bytes }) => {
                assert!(aggregate < AGGREGATE_LIMIT && held + bytes > aggregate);
                Some(false)
            }
            Err(XrefStreamError::Decode(error)) if maximum < IMAGE_BYTES => {
                assert_eq!(error.kind, ValueRefusalKind::WorkLimit);
                Some(false)
            }
            other => panic!("unexpected graph turn {other:?}"),
        }
    }).expect("graph must terminate under bounded turns");

    if success {
        let (file, released) = cursor.take(PAID).unwrap();
        held -= released;
        assert_eq!(file.origin, PREFIX);
        assert_eq!(file.header.offset(), PREFIX);
        assert!(file.trailer.get_inheritable("Root").and_then(PdfObject::as_ref) == Some(ObjRef { num: 1, gen: 0 }));
        let mut trailer = file.trailer.into_retirement();
        assert!((0..512).any(|_| {
            let receipt = trailer.advance(PAID);
            held -= receipt.released_bytes;
            receipt.status.unwrap() == CosStatus::Closed
        }));
        let mut graph = file.graph;
        assert_eq!(graph.objects.len(), 5);
        let image = graph.objects.iter().find(|obj| obj.id.num == 5).unwrap();
        let PdfObject::Stream { data, dict, filters } = &image.value else { panic!("image stream expected") };
        assert_eq!(data.len(), IMAGE_BYTES);
        assert!(filters.is_empty() && dict.iter().all(|entry| entry.key != "Length"));
        for (index, byte) in data.iter().enumerate() {
            let lower = index / IMAGE_SIDE >= IMAGE_SIDE / 2;
            let right = index % IMAGE_SIDE >= IMAGE_SIDE / 2;
            assert_eq!(*byte, [30, 100, 170, 240][usize::from(lower) * 2 + usize::from(right)]);
        }
        let mut retire = ImportedGraphRetirement::default();
        let denied = CosGrant { release_bytes: 0, ..PAID };
        assert!((0..512).any(|_| matches!(checked(retire.close_step(&mut graph, denied), &mut held, &mut peak, denied), Ok(GraphImportEvent::NeedsRelease { .. }))));
        assert!((0..512).any(|_| checked(retire.close_step(&mut graph, PAID), &mut held, &mut peak, PAID).unwrap() == GraphImportEvent::Closed));
        assert_eq!(held, 0);
        let read = source.window.lock().unwrap().physical_reads;
        println!("[DEBUG] Graph projected five objects, {IMAGE_BYTES} exact image bytes, {peak} peak paid bytes, and {read} physical bytes read");
    } else {
        if maximum < IMAGE_BYTES {
            assert!(source.window.lock().unwrap().physical_reads < 32_768);
        }
        if cancel { assert_eq!(checked_file(cursor.advance(&source, PAID, true), &mut held, &mut peak, PAID).unwrap(), PdfByteGraphEvent::Cancelled); }
        close_file(&mut cursor, &mut held, &mut peak);
    }
}

struct NonContiguous<'a>(&'a [u8]);

impl ByteReadSource for NonContiguous<'_> {
    fn byte_len(&self) -> usize { self.0.len() }
    fn byte_at(&self, at: usize) -> Option<u8> { self.0.get(at).copied() }
}

fn single_stream(source: &(impl ByteReadSource + ?Sized), expected: &[u8], unsupported: bool, length: Option<usize>) {
    let mut index = XrefIndex::default();
    index.insert_row(XrefRow { number: 1, entry: Some(XrefEntry::Normal { offset: 0, gen: 0 }) }, false).unwrap();
    if let Some(offset) = length {
        index.insert_row(XrefRow { number: 40, entry: Some(XrefEntry::Normal { offset, gen: 3 }) }, false).unwrap();
    }
    let mut cursor = GraphImportCursor::new(index.len(), 4096);
    let mut held = 0;
    let mut peak = 0;
    assert!((0..512).any(|_| match checked(cursor.advance(source, &index, PAID, false), &mut held, &mut peak, PAID) {
        Ok(GraphImportEvent::More | GraphImportEvent::Object) => false,
        Ok(GraphImportEvent::Complete) if !unsupported => true,
        Err(XrefStreamError::Decode(error)) if unsupported => {
            assert_eq!(error.kind, ValueRefusalKind::OwnershipLimit);
            assert_eq!(error.message, "PDF encoded stream decoder requires contiguous input");
            true
        }
        other => panic!("unexpected small source result {other:?}"),
    }));
    if unsupported { close(&mut cursor, &mut held, &mut peak); return; }
    let mut graph = cursor.take().unwrap();
    let PdfObject::Stream { data, .. } = &graph.objects[0].value else { panic!("stream expected") };
    assert_eq!(data, expected);
    let mut retire = ImportedGraphRetirement::default();
    assert!((0..512).any(|_| checked(retire.close_step(&mut graph, PAID), &mut held, &mut peak, PAID).unwrap() == GraphImportEvent::Closed));
    assert_eq!(held, 0);
}

fn main() {
    let fixture = std::env::args_os().nth(1).expect("Original PDF fixture path");
    large_source(&fixture, IMAGE_BYTES, AGGREGATE_LIMIT, false);
    large_source(&fixture, IMAGE_BYTES - 1, AGGREGATE_LIMIT, false);
    large_source(&fixture, IMAGE_BYTES, 9 * 1024 * 1024, false);
    large_source(&fixture, IMAGE_BYTES, AGGREGATE_LIMIT, true);
    let preserved = NonContiguous(b"1 0 obj << /Filter /VendorFilter /Length 3 >>\nstream\nabc\nendstream\nendobj");
    single_stream(&preserved, b"abc", false, None);
    let coded = b"1 0 obj << /Filter /ASCIIHexDecode /Length 3 >>\nstream\n41>\nendstream\nendobj";
    single_stream(coded.as_slice(), b"A", false, None);
    single_stream(&NonContiguous(coded), if SEGMENTED_CODECS { b"A" } else { b"" }, !SEGMENTED_CODECS, None);
    let indirect = b"1 0 obj << /Length 40 3 R >>\nstream\na\nendstream\nb\nendstream\nendobj\n40 3 obj\n13\nendobj";
    let offset = indirect.windows(8).position(|token| token == b"40 3 obj").unwrap();
    single_stream(&NonContiguous(indirect), b"a\nendstream\nb", false, Some(offset));
    if !SEGMENTED_CODECS {
        let encoded_xref = NonContiguous(include_bytes!("$XREF_STREAM_FIXTURE"));
        let mut cursor = PdfByteGraphCursor::new(32, 4096);
        let mut held = 0;
        let mut peak = 0;
        assert!((0..512).any(|_| checked_file(cursor.advance(&encoded_xref, PAID, false), &mut held, &mut peak, PAID).unwrap() == PdfByteGraphEvent::ReferenceRequired));
        close_file(&mut cursor, &mut held, &mut peak);
    }
    println!("[DEBUG] Verified held-byte cap, output length refusal, cancellation and raw retention");
    xref::exercise_preparation(&fixture);
    if SEGMENTED_CODECS { exercise_codecs(); }
}

