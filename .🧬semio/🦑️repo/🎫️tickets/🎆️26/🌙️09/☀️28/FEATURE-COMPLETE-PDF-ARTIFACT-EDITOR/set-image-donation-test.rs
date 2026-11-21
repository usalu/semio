fn image_archive_receiving_stock() -> dsl::original_receiving::ReceivingStock {
    use semio_framework_value::RetainedCloneGrant;
    let quantum = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
        maximum_release_bytes: 1048576, maximum_depth: 4096,
    };
    let stock = RetainedCloneGrant {
        maximum_items: 16777216, maximum_copy_bytes: 134217728, maximum_capacity_bytes: 268435456,
        maximum_release_bytes: 536870912, ..quantum
    };
    dsl::original_receiving::ReceivingStock { stock, quantum, fuel: 1 << 20 }
}

fn archive_image(fixture: &serde_json::Value) -> PdfMutation {
    let id = fixture["id"].as_str().unwrap();
    let width = fixture["width"].as_u64().unwrap() as u32;
    let height = fixture["height"].as_u64().unwrap() as u32;
    let components = fixture["components"].as_u64().unwrap() as u32;
    let stride = fixture["stride"].as_u64().unwrap() as u32;
    let samples = (0..width * height * components).map(|i| (i * stride) as u8).collect();
    let mut image = if components == 3 { PdfImage::rgb8(id, width, height, samples) }
        else { PdfImage::gray8(id, width, height, samples) };
    image.decode = vec![0.5, 1.0];
    image.interpolate = true;
    image.soft_mask = Some("ArchiveAlpha".into());
    image.soft_mask_in_data = Some(1);
    image.mask = Some(PdfImageMask::ColorKey { ranges: vec![1, 2, 3, 4, 5, 6] });
    image.matte = Some(vec![0.1, 0.2, 0.3]);
    image.intent = Some("RelativeColorimetric".into());
    image.optional_content = Some("FixtureGroup".into());
    image.struct_parent = Some(17);
    image.extra = vec![PdfDictEntry { key: "Name".into(), value: PdfObject::Int(7) }];
    PdfMutation::SetImage(set_image::SetImage { image, index: Some(3) })
}

#[test]
fn original_archived_set_image_donates_into_concrete_mutation() {
    use dsl::original_receiving::receive_to_completion;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    for lane in ["image", "alpha"] {
        let expected = archive_image(&fixture[lane]);
        let frame = expected.encode_op().unwrap();
        let range = (3, 3 + frame.len());
        let source: Vec<_> = [0xff, 0, 0x89].into_iter().chain(frame.iter().copied()).chain([0xff]).collect();
        let actual = receive_to_completion::<PdfMutation>(&source, range, image_archive_receiving_stock()).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual, PdfMutation::decode_op(&frame).unwrap());
        let encoded = actual.encode_op().unwrap();
        assert_eq!(encoded, frame);
        assert_eq!(pack::crc32c(&encoded), pack::crc32c(&source[range.0..range.1]));
        assert_eq!(semio_framework_hash::sha256_hex(&encoded), semio_framework_hash::sha256_hex(&frame));
    }
}

#[test]
fn original_archived_set_image_refuses_malformed_samples_and_metadata() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let mut reader = dsl::ByteReader::new(&frame);
    let format = reader.read_u8().unwrap();
    let tag = reader.read_u8().unwrap();
    let length = reader.read_varint_u64().unwrap() as usize;
    let body = std::str::from_utf8(reader.read_bytes(length).unwrap()).unwrap();
    for row in fixture["malformed"].as_array().unwrap() {
        let source = body.replacen(row[0].as_str().unwrap(), row[1].as_str().unwrap(), 1);
        assert_ne!(source, body, "{row}");
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(format);
        writer.write_u8(tag);
        writer.write_varint_u64(source.len() as u64);
        writer.write_bytes(source.as_bytes());
        let malformed = writer.into_bytes();
        assert!(PdfMutation::decode_op(&malformed).is_err(), "{row}");
        let refused = receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(refused.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
    }
}

#[test]
fn original_archived_set_image_keeps_ordinary_grammar_for_other_shapes() {
    use dsl::original_receiving::receive_to_completion;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    for body in fixture["ordinaryGrammar"].as_array().unwrap() {
        let body = body.as_str().unwrap();
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(1);
        writer.write_u8(26);
        writer.write_varint_u64(body.len() as u64);
        writer.write_bytes(body.as_bytes());
        let frame = writer.into_bytes();
        let expected = PdfMutation::decode_op(&frame).unwrap();
        let actual = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual.encode_op().unwrap(), expected.encode_op().unwrap());
    }
}

#[test]
fn original_archived_set_image_leaves_occupied_recipient_and_donor_intact() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let expected = archive_image(&fixture["image"]);
    let frame = expected.encode_op().unwrap();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let birth_grant = semio_framework_value::RetainedCloneGrant { maximum_capacity_bytes: 0, maximum_release_bytes: 0, ..grant };
    let (mut cursor, admission) = PdfMutation::admit_receiving(&frame, (0, frame.len()), birth_grant).result.unwrap().unwrap();
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    wallet.post(admission).unwrap();
    let mut reserved_exact_backing = false;
    let mut total = admission;
    let mut peak_copy = 0;
    while !cursor.is_ready() {
        let body = grant.maximum_copy_bytes;
        let demand = cursor.demands(&frame, body).unwrap();
        peak_copy = peak_copy.max(demand.copy_bytes);
        let units = cursor.step_work_demand(&frame, body).unwrap();
        if demand.capacity_bytes == 49_152 {
            let short = semio_framework_value::RetainedCloneGrant { maximum_capacity_bytes: 49_151, ..grant };
            let denied = wallet.run(short, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
            assert_eq!(denied.performed, 0);
            assert_eq!(denied.actual, None);
            assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
            assert_eq!(cursor.demands(&frame, body).unwrap(), demand);
        }
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        reserved_exact_backing |= turn.actual == Some(49_152);
        let result = turn.output;
        assert!(result.work > 0);
        let progress = result.result.unwrap();
        wallet.post(progress).unwrap();
        total = total.checked_add(progress).unwrap();
    }
    assert!(reserved_exact_backing);
    let occupied = archive_image(&fixture["alpha"]);
    let mut target = Some(occupied.clone());
    let turn = cursor.take_into(&mut target, grant);
    assert_eq!(turn.work, 0);
    assert_eq!(turn.result.unwrap(), RetainedCloneProgress::default());
    assert_eq!(target.take().unwrap(), occupied);
    assert!(cursor.is_ready());
    let turn = cursor.take_into(&mut target, grant);
    assert_eq!(turn.work, 1);
    assert_eq!(target.unwrap(), expected);
    wallet.post(turn.result.unwrap()).unwrap();
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
    println!("[DEBUG] original RGB receive through Ready: {total:?}; peak copy quote {peak_copy}");
}

#[test]
fn original_archived_set_image_cancellation_retires_exact_backing() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, ValueRefusalKind};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let (mut cursor, admission) = PdfMutation::admit_receiving(&frame, (0, frame.len()), grant).result.unwrap().unwrap();
    let active = std::cell::Cell::new(true);
    let mut checkpoint = |_| active.get();
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    wallet.post(admission).unwrap();
    let mut total = admission;
    let body = grant.maximum_copy_bytes;
    loop {
        let units = cursor.step_work_demand(&frame, body).unwrap();
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        let progress = turn.output.result.unwrap();
        wallet.post(progress).unwrap();
        total = total.checked_add(progress).unwrap();
        if turn.actual == Some(49_152) { break; }
        assert!(!cursor.is_ready());
    }
    active.set(false);
    let units = cursor.step_work_demand(&frame, body).unwrap();
    let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
    assert_eq!(turn.performed, 0);
    assert_eq!(turn.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
    let mut output = None;
    assert_eq!(cursor.take_into(&mut output, grant).work, 0);
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();
        total = total.checked_add(progress).unwrap();
    }
    assert!(output.is_none());
    assert_eq!(total.retained_capacity_bytes, total.released_bytes);
    println!("[DEBUG] canceled original RGB released {} bytes", total.released_bytes);
}
