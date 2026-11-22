pub fn exercise_preparation(fixture: &std::ffi::OsStr) {
    use crate::{checked_file, NonContiguous, Segmented, Window, IMAGE_BYTES, PAID, PREFIX};
    use std::{fs::File, sync::Mutex};

    let paid = PAID;
    for cancel in [false, true] {
        let file = File::open(fixture).unwrap();
        let len = usize::try_from(file.metadata().unwrap().len()).unwrap();
        let original = Segmented { len, width: 4096, window: Mutex::new(Window {
            file, start: usize::MAX, bytes: [0; 4096], physical_reads: 0,
        }) };
        let mut prep = PdfFilePreparationCursor::new(5, IMAGE_BYTES);
        let mut held = 0;
        let mut peak = 0;
        let mut after_graph = None;
        assert!(prep.take().is_none());
        let complete = (0..100_000).find_map(|_| {
            let text_active = matches!(prep.phase, PreparePhase::Text);
            if text_active && after_graph.is_none() {
                after_graph = Some(original.window.lock().unwrap().physical_reads);
            }
            let grant = CosGrant { allocation_bytes: paid.allocation_bytes - held, ..paid };
            let receipt = prep.advance(&original, grant, cancel && text_active);
            match checked_file(receipt, &mut held, &mut peak, grant).unwrap() {
                PdfByteGraphEvent::More | PdfByteGraphEvent::Object => None,
                PdfByteGraphEvent::Complete if !cancel => Some(true),
                PdfByteGraphEvent::Cancelled if cancel => Some(false),
                unexpected => panic!("unexpected preparation result {unexpected:?}"),
            }
        }).expect("bounded preparation must settle");
        assert_eq!(original.window.lock().unwrap().physical_reads, after_graph.unwrap());

        if complete {
            let mut prepared = prep.take().unwrap();
            assert!(prep.take().is_none());
            drop(original);
            assert_eq!(prepared.header.offset(), PREFIX);
            assert_eq!(prepared.graph.objects.len(), 5);
            assert_eq!(exact_position(&prepared.graph.positions, ObjRef { num: 5, gen: 0 }), Some(4));
            assert!(prepared.trailer.iter().any(|entry| entry.key == "Root" && entry.value.as_ref() == Some(ObjRef { num: 1, gen: 0 })));
            assert!(!prepared.trailer.iter().any(|entry| entry.key == "Size"));
            let value = &prepared.graph.objects[4].value;
            assert!(matches!(value, PdfObject::Stream { data, .. } if data.len() == IMAGE_BYTES));
            settle_prepared(&mut prepared, &mut held, &mut peak, paid);
            println!("[DEBUG] Prepared five objects and inherited trailer without reading the source after graph transfer, with {peak} peak paid bytes");
        } else {
            let denied = CosGrant { release_bytes: 0, ..paid };
            assert!((0..512).any(|_| matches!(
                checked_file(prep.close_step(denied), &mut held, &mut peak, denied),
                Ok(PdfByteGraphEvent::NeedsRelease { .. })
            )));
            assert!((0..512).any(|_| checked_file(prep.close_step(paid), &mut held, &mut peak, paid).unwrap() == PdfByteGraphEvent::Closed));
            assert!(prep.take().is_none());
        }
        assert_eq!(held, 0);
    }

    let fixtures: [(&str, &[u8]); 2] = [
        ("classic", include_bytes!("$CLASSIC_INCREMENTAL_FIXTURE")),
        ("compressed", include_bytes!("$XREF_STREAM_FIXTURE")),
    ];
    for (format, bytes) in fixtures.into_iter().take(if crate::SEGMENTED_CODECS { 2 } else { 1 }) {
        let incremental = NonContiguous(bytes);
        let mut prep = PdfFilePreparationCursor::new(32, 262_144);
        let mut held = 0;
        let mut peak = 0;
        let small = CosGrant { work: 1, ..paid };
        assert!((0..50_000).any(|_| match checked_file(prep.advance(&incremental, small, false), &mut held, &mut peak, small).unwrap() {
            PdfByteGraphEvent::More | PdfByteGraphEvent::Object => false,
            PdfByteGraphEvent::Complete => true,
            unexpected => panic!("unexpected incremental preparation result {unexpected:?}"),
        }));
        let mut prepared = prep.take().unwrap();
        let info = prepared.trailer.iter().find(|entry| entry.key == "Info").unwrap().value.as_ref().unwrap();
        let metadata = prepared.graph.objects.iter().find(|object| object.id == info).unwrap();
        assert_eq!(metadata.value.dict_get("Title"), Some(&PdfObject::Text("Updated".to_owned())));
        assert!(!prepared.graph.objects.iter().any(|object| object.id.num == 3));
        settle_prepared(&mut prepared, &mut held, &mut peak, paid);
        assert_eq!(held, 0);
        println!("[DEBUG] Prepared the {format} segmented incremental fixture under one-unit grants and retired all backing");
    }
    if crate::SEGMENTED_CODECS { exercise_compressed_length(); }
}

fn exercise_compressed_length() {
    let source = crate::NonContiguous(include_bytes!("$COMPRESSED_LENGTH_FIXTURE"));
    let grant = CosGrant { work: 1, ..crate::PAID };
    let mut prep = PdfFilePreparationCursor::new(100, 1_000_000);
    let mut held = 0;
    let mut peak = 0;
    assert!((0..100_000).any(|_| match crate::checked_file(prep.advance(&source, grant, false), &mut held, &mut peak, grant).unwrap() {
        PdfByteGraphEvent::More | PdfByteGraphEvent::Object => false,
        PdfByteGraphEvent::Complete => true,
        unexpected => panic!("unexpected compressed object-stream length turn {unexpected:?}"),
    }));
    let mut prepared = prep.take().unwrap();
    let payload = b"BT /F1 12 Tf 10 70 Td (prefix\nendstream\nsuffix) Tj ET";
    for number in [4, 10] {
        let object = prepared.graph.objects.iter().find(|object| object.id == ObjRef { num: number, gen: 0 }).unwrap();
        assert!(matches!(&object.value, PdfObject::Stream { data, .. } if data == payload));
    }
    let length = prepared.graph.objects.iter().find(|object| object.id == ObjRef { num: 7, gen: 0 }).unwrap();
    assert_eq!(length.value, PdfObject::Int(payload.len() as i64));
    settle_prepared(&mut prepared, &mut held, &mut peak, grant);
    assert_eq!(held, 0);
    println!("[DEBUG] Prepared compressed indirect lengths and preserved marker text inside both stream payloads");
}

fn settle_prepared(prepared: &mut PreparedPdfFile, held: &mut usize, peak: &mut usize, grant: CosGrant) {
    let mut trail = PdfCosCursor::retire_dictionary(std::mem::take(&mut prepared.trailer));
    assert!((0..512).any(|_| {
        let receipt = trail.close_step(grant);
        assert!(receipt.work <= grant.work && receipt.released_bytes <= grant.release_bytes);
        *held -= receipt.released_bytes;
        receipt.status.unwrap() == CosStatus::Closed
    }));
    let mut retirement = ImportedGraphRetirement::default();
    assert!((0..512).any(|_| {
        let receipt = retirement.close_step(&mut prepared.graph, grant);
        crate::checked(receipt, held, peak, grant).unwrap() == GraphImportEvent::Closed
    }));
}

