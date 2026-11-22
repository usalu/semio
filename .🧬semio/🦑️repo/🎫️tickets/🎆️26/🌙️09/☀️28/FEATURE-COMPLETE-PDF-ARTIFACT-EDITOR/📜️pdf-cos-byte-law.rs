use std::{cell::Cell, fs::File, io::{Read, Seek, SeekFrom}, sync::Mutex};

use links::{XrefLinkCursor, XrefLinks};
use section::{ClassicSectionCursor, ClassicSectionEvent, ClassicSectionReceipt};

fn retain(turn: ClassicSectionReceipt, held: &mut usize, grant: CosGrant, reads: usize) -> ClassicSectionEvent {
    assert!(turn.work <= grant.work && reads <= turn.work * 2);
    assert!(turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
    *held += turn.allocated_bytes;
    *held = held.checked_sub(turn.released_bytes).unwrap();
    turn.event.unwrap()
}

fn close(cursor: &mut ClassicSectionCursor, held: &mut usize) {
    cursor.begin_close();
    let unpaid = CosGrant { release_bytes: 0, ..GRANT };
    assert!((0..128).any(|_| matches!(
        retain(cursor.close_step(unpaid), held, unpaid, 0),
        ClassicSectionEvent::Progress(CosStatus::NeedsRelease { .. })
    )));
    assert!((0..256).any(|_| matches!(
        retain(cursor.close_step(GRANT), held, GRANT, 0),
        ClassicSectionEvent::Progress(CosStatus::Closed)
    )));
    assert_eq!(*held, 0);
}

fn classic_trailer() {
    let path = std::env::args_os().nth(1).expect("Original PDF path");
    for width in [1, 3, 13, 4096] {
        let file = File::open(&path).unwrap();
        let len = usize::try_from(file.metadata().unwrap().len()).unwrap();
        assert_eq!(len, $SOURCE_BYTES);
        let source = Segmented { len, width, window: Mutex::new(Window {
            file, start: usize::MAX, bytes: [0; 4096], physical_reads: 0,
        }) };
        let original = ByteReadRange::new(&source, PREFIX, len).unwrap();
        let reads = Cell::new(0);
        let read = |at| { reads.set(reads.get() + 1); original.byte_at(at) };
        let mut footer = footer::FooterCursor::new(len);
        let start = (0..128).find_map(|_| {
            if let footer::FooterStep::Complete(start) = footer.advance(&read, 1) { Some(start.unwrap()) } else { None }
        }).unwrap();
        assert_eq!(start, $XREF_OFFSET);
        let mut image_offset = None;

        for cancel in [false, true] {
            let mut cursor = ClassicSectionCursor::new(start);
            let mut held = 0;
            let mut seen = 0;
            let mut refuses = false;
            assert!((0..512).any(|_| {
                let prior = reads.get();
                let grant = if refuses { GRANT } else { CosGrant { allocation_bytes: 0, ..GRANT } };
                let turn = cursor.advance(&read, grant, false);
                match retain(turn, &mut held, grant, reads.get() - prior) {
                    ClassicSectionEvent::Progress(CosStatus::More) => cancel && held != 0,
                    ClassicSectionEvent::Progress(CosStatus::NeedsAllocation { .. }) => {
                        assert_eq!(held, 0);
                        refuses = true;
                        false
                    }
                    ClassicSectionEvent::Progress(CosStatus::Complete { .. }) => {
                        assert!(!cancel);
                        true
                    }
                    ClassicSectionEvent::Row { number, offset, generation, live } => {
                        assert_eq!(number, seen);
                        assert_eq!(live, number != 0);
                        assert_eq!(generation, if live { 0 } else { 65535 });
                        if number == 5 { image_offset = Some(offset); }
                        seen += 1;
                        false
                    }
                    _ => panic!("unexpected classic trailer receipt"),
                }
            }));
            assert!(refuses);
            assert_eq!(seen, 6);
            assert!(held != 0);
            if cancel {
                let prior = reads.get();
                assert!(matches!(
                    retain(cursor.advance(&read, GRANT, true), &mut held, GRANT, reads.get() - prior),
                    ClassicSectionEvent::Progress(CosStatus::Cancelled)
                ));
                assert_eq!(reads.get(), prior);
                assert!(cursor.ready_trailer().is_none());
            } else {
                let trailer = cursor.ready_trailer().unwrap();
                assert_eq!(trailer.len(), 2);
                assert_eq!(trailer.iter().find(|entry| entry.key == "Size").unwrap().value.as_i64(), Some(6));
                assert_eq!(trailer.iter().find(|entry| entry.key == "Root").unwrap().value.as_ref(), Some(ObjRef { num: 1, gen: 0 }));
                let mut links = XrefLinkCursor::default();
                assert_eq!(links.advance(trailer, 1), (1, None));
                assert_eq!(links.advance(trailer, 1), (1, Some(XrefLinks::default())));
            }
            close(&mut cursor, &mut held);
        }
        assert!(reads.get() < 512);
        let pulled = source.window.lock().unwrap().physical_reads;
        assert!(pulled <= 2 * 4096);
        println!("[DEBUG] Segments of {width} bytes verified typed trailer and paid cancellation with {pulled} physical bytes read");
        image_frame(&original, image_offset.unwrap());
    }
}

fn image_frame(source: &impl ByteReadSource, offset: usize) {
    let reads = Cell::new(0);
    let read = |position| { reads.set(reads.get() + 1); source.byte_at(position) };
    let mut cursor = IndirectFrameCursor::new(offset, CosLimits::default());
    let mut held = 0;
    assert!((0..512).any(|_| {
        let prior = reads.get();
        let turn = cursor.advance_bytes(&read, GRANT, false);
        assert!(turn.work <= 1 && reads.get() - prior <= turn.work * 2);
        assert!(turn.allocated_bytes <= GRANT.allocation_bytes && turn.released_bytes <= GRANT.release_bytes);
        held += turn.allocated_bytes;
        held -= turn.released_bytes;
        turn.status.unwrap() == IndirectFrameStatus::Complete
    }));
    assert!(reads.get() < 256);
    let dictionary = cursor.ready_value().unwrap();
    for (key, expected) in [("Width", 3072), ("Height", 3072), ("Length", 3072 * 3072)] {
        assert_eq!(dictionary.dict_get(key).and_then(PdfObject::as_i64), Some(expected));
    }
    let start = cursor.stream_start().unwrap();
    let image = ByteReadRange::new(source, start, 3072 * 3072).unwrap();
    assert_eq!(image.byte_at(0), Some(30));
    assert_eq!(image.byte_at(3071), Some(100));
    assert_eq!(image.byte_at(3071 * 3072), Some(170));
    assert_eq!(image.byte_at(3072 * 3072 - 1), Some(240));
    assert_eq!(image.byte_at(3072 * 3072), None);
    let mut marker = [0; 10];
    assert_eq!(source.read_bytes(start + 3072 * 3072, &mut marker), marker.len());
    assert_eq!(&marker, b"\nendstream");
    cursor.begin_close();
    assert!((0..512).any(|_| {
        let turn = cursor.close_step(GRANT);
        assert!(turn.work <= 1 && turn.released_bytes <= GRANT.release_bytes);
        held -= turn.released_bytes;
        turn.status.unwrap() == IndirectFrameStatus::Closed
    }));
    assert_eq!(held, 0);
}

fn typed_bytes() {
    let sample = b"<< /Na#6De (A\\101\\\r\nB) /Hex < 4 1 2 > /Ref 1 0 R /Value -012.50 /N null >>";
    let mut cursor = PdfCosCursor::new(0, CosLimits::default());
    let reads = Cell::new(0);
    let read = |at: usize| { reads.set(reads.get() + 1); sample.get(at).copied() };
    assert_eq!(cursor.advance_bytes(&read, CosGrant { work: 0, ..GRANT }, false).work, 0);
    assert_eq!(reads.get(), 0);
    assert!((0..512).any(|_| {
        let prior = reads.get();
        let step = cursor.advance_bytes(&read, GRANT, false);
        assert!(step.work <= 1 && reads.get() - prior <= step.work * 2);
        matches!(step.status.unwrap(), CosStatus::Complete { end } if end == sample.len())
    }));
    let value = cursor.ready_value().unwrap();
    assert_eq!(value.dict_get("Name"), Some(&PdfObject::Str(b"AAB".to_vec())));
    assert_eq!(value.dict_get("Hex"), Some(&PdfObject::Str(vec![0x41, 0x20])));
    assert_eq!(value.dict_get("Ref").and_then(PdfObject::as_ref), Some(ObjRef { num: 1, gen: 0 }));
    assert_eq!(value.dict_get("N"), Some(&PdfObject::Null));

    for (length, expected) in [(256, true), (257, false)] {
        let mut name = [b'A'; 259];
        name[0] = b'/';
        name[length + 1] = b' ';
        let mut cursor = PdfCosCursor::new(0, CosLimits::default());
        let read = |at: usize| name.get(at).copied();
        assert!((0..300).any(|_| match cursor.advance_bytes(read, GRANT, false).status {
            Ok(CosStatus::More) => false,
            Ok(CosStatus::Complete { end }) => {
                assert!(expected && end == length + 1);
                true
            }
            Err("PDF COS scalar token limit exceeded") => !expected,
            other => panic!("{other:?}"),
        }));
    }
}

fn main() {
    typed_bytes();
    classic_trailer();
}

