use std::{cell::Cell, fs::File, io::{Read, Seek, SeekFrom}, sync::Mutex};

use classic::{ClassicXrefCursor, ClassicXrefEvent};
use index::{RevisionIndex, XrefIndex, XrefIndexReceipt, XrefIndexStatus, XrefIndexScan, XrefRow};
use offset::{XrefOffsetCursor, XrefOffsetStatus};
use probe_3::{FooterCursor, FooterStep};

/// 🧬 Supplies the model identity fields shared by the real indirect-header and offset-validation code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ObjRef { num: u32, gen: u16 }

/// 🎟️ Matches the native parser ABI used by the sparse directory in the focused link.
#[derive(Clone, Copy)]
struct CosGrant { work: usize, allocation_bytes: usize, release_bytes: usize }

fn is_ws(value: u8) -> bool { matches!(value, 0 | b'\t' | b'\n' | b'\x0c' | b'\r' | b' ') }
fn is_delim(value: u8) -> bool { b"()<>[]{}/%".contains(&value) }

const PREFIX: usize = 7;
const GRANT: CosGrant = CosGrant { work: 1, allocation_bytes: 4096, release_bytes: 4096 };

struct Window {
    file: File,
    start: usize,
    bytes: [u8; 4096],
    physical_reads: usize,
}

struct Segmented {
    len: usize,
    width: usize,
    window: Mutex<Window>,
}

impl ByteReadSource for Segmented {
    fn byte_len(&self) -> usize { PREFIX + self.len }

    fn byte_at(&self, index: usize) -> Option<u8> {
        let offset = index.checked_sub(PREFIX)?;
        if offset >= self.len { return None; }
        let mut window = self.window.lock().unwrap();
        let start = offset / self.width * self.width;
        if window.start != start {
            let Window { file, bytes, .. } = &mut *window;
            let extent = self.width.min(self.len - start);
            file.seek(SeekFrom::Start(start as u64)).unwrap();
            file.read_exact(&mut bytes[..extent]).unwrap();
            window.start = start;
            window.physical_reads += extent;
        }
        Some(window.bytes[offset - start])
    }
}

fn charge(receipt: XrefIndexReceipt, held: &mut usize, grant: CosGrant) -> XrefIndexStatus {
    assert!(receipt.work <= grant.work);
    assert!(receipt.allocated_bytes <= grant.allocation_bytes);
    assert!(receipt.released_bytes <= grant.release_bytes);
    *held += receipt.allocated_bytes;
    *held = held.checked_sub(receipt.released_bytes).unwrap();
    receipt.status.unwrap()
}

fn close_index(index: &mut XrefIndex, held: &mut usize) {
    index.begin_close();
    let unpaid = CosGrant { release_bytes: 0, ..GRANT };
    assert!(matches!(charge(index.close_step(unpaid), held, unpaid), XrefIndexStatus::NeedsRelease { .. }));
    assert_eq!(charge(index.close_step(GRANT), held, GRANT), XrefIndexStatus::Closed);
    assert_eq!(*held, 0);
}

fn classic_boundaries() {
    let bytes = b"xref % subsection\r\n1 1\n00000000000000000015 00000 n% row\r\ntrailer\n<<>>";
    for work in [1, 2, 5, 19, 256] {
        let mut parser = ClassicXrefCursor::new(0);
        let read = |at: usize| bytes.get(at).copied();
        let mut found = false;
        loop {
            let receipt = parser.advance(read, work);
            assert!(receipt.work <= work);
            match receipt.event.unwrap() {
                ClassicXrefEvent::More => {}
                ClassicXrefEvent::Entry { number, offset, generation, live } => {
                    assert!(!found);
                    assert_eq!((number, offset, generation, live), (1, 15, 0, true));
                    found = true;
                }
                ClassicXrefEvent::Trailer => break,
            }
        }
        assert!(found && bytes[parser.position()..].starts_with(b"\n<<>>"));
    }

    let bytes = b"xref\n1 1\n000000000000000000015 00000 n\ntrailer\n<<>>";
    let read = |at: usize| bytes.get(at).copied();
    let mut parser = ClassicXrefCursor::new(0);
    let receipt = parser.advance(read, bytes.len());
    assert!(matches!(receipt.event, Err("xref token exceeds the numeric field width")));
}

fn main() {
    classic_boundaries();
    let path = std::env::args_os().nth(1).expect("Original PDF path");
    for width in [1, 3, 13, 4096] {
        let file = File::open(&path).unwrap();
        let len = usize::try_from(file.metadata().unwrap().len()).unwrap();
        assert_eq!(len, $SOURCE_BYTES);
        let source = Segmented { len, width, window: Mutex::new(Window {
            file, start: usize::MAX, bytes: [0; 4096], physical_reads: 0,
        }) };
        let original = ByteReadRange::new(&source, PREFIX, len).unwrap();
        assert!(ByteReadRange::new(&source, PREFIX, len + 1).is_none());
        assert!(ByteReadRange::new(&source, usize::MAX, 1).is_none());
        assert_eq!(original.byte_at(len), None);
        let read_count = Cell::new(0);
        let read = |at| { read_count.set(read_count.get() + 1); original.byte_at(at) };

        let mut footer = FooterCursor::new(len);
        let start = (0..128).find_map(|_| {
            if let FooterStep::Complete(start) = footer.advance(&read, 1) { Some(start.unwrap()) } else { None }
        }).unwrap();
        assert_eq!(start, $XREF_OFFSET);
        let mut classic = ClassicXrefCursor::new(start);
        let mut pending = RevisionIndex::default();
        let mut held = 0;
        let mut seen = 0;
        let prior = read_count.get();
        assert_eq!(classic.advance(&read, 0).work, 0);
        assert_eq!(read_count.get(), prior);
        let mut reached_trailer = false;
        for _ in 0..256 {
            let before = read_count.get();
            let receipt = classic.advance(&read, GRANT.work);
            assert!(receipt.work <= GRANT.work);
            assert_eq!(read_count.get() - before, receipt.work);
            match receipt.event.unwrap() {
                ClassicXrefEvent::More => {}
                ClassicXrefEvent::Trailer => { reached_trailer = true; break; }
                ClassicXrefEvent::Entry { number, offset, generation, live } => {
                    assert_eq!(number, seen);
                    assert_eq!(live, number != 0);
                    assert_eq!(generation, if live { 0 } else { 65535 });
                    if number == 1 { assert_eq!(offset, $CATALOG_OFFSET); }
                    seen += 1;
                    let entry = live.then_some(XrefEntry::Normal { offset, gen: generation });
                    assert!(pending.offer(XrefRow { number, entry }, false));
                    if number == 0 {
                        let unpaid = CosGrant { allocation_bytes: 0, ..GRANT };
                        assert!(matches!(charge(pending.insert_step(false, unpaid), &mut held, unpaid), XrefIndexStatus::NeedsAllocation { .. }));
                    }
                    assert!((0..256).any(|_| charge(pending.insert_step(false, GRANT), &mut held, GRANT) == XrefIndexStatus::Inserted));
                }
            }
        }
        assert!(reached_trailer);
        assert_eq!(seen, 6);
        assert_eq!(original.byte_at(classic.position()), Some(b'\n'));
        assert!(pending.begin_merge(false));
        assert!((0..256).any(|_| charge(pending.merge_step(GRANT), &mut held, GRANT) == XrefIndexStatus::Inserted));
        let mut index = pending.take().unwrap();
        assert_eq!(index.len(), 5);
        assert!(index.is_free(0));

        let mut offsets = XrefOffsetCursor::default();
        let prior = read_count.get();
        assert_eq!(offsets.advance(&read, &index, 0, false), (0, XrefOffsetStatus::More));
        assert_eq!(read_count.get(), prior);
        assert!((0..128).any(|_| {
            let before = read_count.get();
            let (work, status) = offsets.advance(&read, &index, GRANT.work, false);
            assert!(work <= GRANT.work && read_count.get() - before <= work);
            match status { XrefOffsetStatus::Complete => true, XrefOffsetStatus::More => false, other => panic!("{other:?}") }
        }));
        close_index(&mut index, &mut held);

        let mut index = XrefIndex::default();
        assert!(index.offer(XrefRow { number: 1, entry: Some(XrefEntry::Normal { offset: $CATALOG_OFFSET, gen: 1 }) }));
        assert!((0..64).any(|_| charge(index.advance(GRANT, false), &mut held, GRANT) == XrefIndexStatus::Inserted));
        let mut offsets = XrefOffsetCursor::default();
        assert!((0..64).any(|_| {
            match offsets.advance(&read, &index, GRANT.work, false).1 {
                XrefOffsetStatus::More => false,
                XrefOffsetStatus::Invalid { id, offset } => {
                    assert_eq!((id, offset), (ObjRef { num: 1, gen: 1 }, $CATALOG_OFFSET));
                    true
                }
                other => panic!("{other:?}"),
            }
        }));
        let prior = read_count.get();
        let mut cancelled = XrefOffsetCursor::default();
        assert_eq!(cancelled.advance(&read, &index, GRANT.work, true), (0, XrefOffsetStatus::Cancelled));
        assert_eq!(read_count.get(), prior);
        close_index(&mut index, &mut held);

        let truncated = |position| (position < start + 4).then(|| original.byte_at(position)).flatten();
        let mut parser = ClassicXrefCursor::new(start);
        for _ in 0..4 { assert!(parser.advance(truncated, 1).event.is_ok()); }
        let receipt = parser.advance(truncated, 1);
        assert_eq!(receipt.work, 1);
        assert!(receipt.event.is_err());
        let physical_reads = source.window.lock().unwrap().physical_reads;
        assert!(physical_reads <= 3 * 4096 && read_count.get() < 256);
        println!("[DEBUG] Segments of {width} bytes validated five objects and a free entry with {physical_reads} physical bytes read");
    }
}

