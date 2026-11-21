#[test]
fn original_archived_set_image_metadata_uses_exact_funded_allocations() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let expected = archive_image(&fixture["image"]);
    let frame = expected.encode_op().unwrap();
    let expected_allocations: Vec<_> = fixture["metadata"]["allocations"].as_array().unwrap().iter().map(|row| {
        let element = match row["element"].as_str().unwrap() {
            "f64" => size_of::<f64>(),
            "u32" => size_of::<u32>(),
            "PdfDictEntry" => size_of::<PdfDictEntry>(),
            _ => panic!("unknown metadata corpus element"),
        };
        row["count"].as_u64().unwrap() as usize * element
    }).collect();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let body = grant.maximum_copy_bytes;
    let (mut cursor, admitted) = PdfMutation::admit_receiving(&frame, (0, frame.len()), grant).result.unwrap().unwrap();
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    let mut progress = admitted;
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut seen = Vec::new();
    while !cursor.is_ready() {
        let units = cursor.step_work_demand(&frame, body).unwrap();
        let demand = cursor.demands(&frame, body).unwrap();
        if units > 8 && demand.capacity_bytes > 0 {
            let short = semio_framework_value::RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant };
            let denied = wallet.run(short, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
            assert_eq!(denied.performed, 0);
            assert_eq!(denied.actual, None);
            assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
            assert_eq!(cursor.demands(&frame, body).unwrap(), demand);
        }
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        let receipt = turn.output.result.unwrap();
        assert!(receipt.copied_items > 0 && turn.performed > 0);
        assert_eq!(receipt.retained_capacity_bytes, turn.actual.unwrap_or(0));
        if units > 8 && demand.capacity_bytes > 0 {
            assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
            seen.push(demand.capacity_bytes);
        }
        wallet.post(receipt).unwrap();
        progress = progress.checked_add(receipt).unwrap();
    }
    assert_eq!(seen, expected_allocations);
    println!("[DEBUG] paid metadata allocation bytes {seen:?}; original receiving receipt {progress:?}");
    let mut value = None;
    assert_eq!(cursor.take_into(&mut value, grant).work, 1);
    assert_eq!(value.unwrap(), expected);
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
}

#[test]
fn original_archived_set_image_cancels_after_paid_metadata_placement() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, ValueRefusalKind};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let body = grant.maximum_copy_bytes;
    let (mut cursor, admitted) = PdfMutation::admit_receiving(&frame, (0, frame.len()), grant).result.unwrap().unwrap();
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    let active = std::cell::Cell::new(true);
    let mut checkpoint = |_| active.get();
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut total = admitted;
    let mut seen = 0;
    let end = fixture["metadata"]["allocations"].as_array().unwrap().len();
    while seen < end {
        let units = cursor.step_work_demand(&frame, body).unwrap();
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        let receipt = turn.output.result.unwrap();
        if units > 8 && receipt.retained_capacity_bytes > 0 { seen += 1; }
        total = total.checked_add(receipt).unwrap();
        wallet.post(receipt).unwrap();
        assert!(!cursor.is_ready());
    }
    active.set(false);
    let units = cursor.step_work_demand(&frame, body).unwrap();
    let refused = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
    assert_eq!(refused.performed, 0);
    assert_eq!(refused.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
    assert!(!cursor.is_ready());
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        let receipt = turn.result.unwrap().progress();
        total = total.checked_add(receipt).unwrap();
        wallet.post(receipt).unwrap();
    }
    assert_eq!(total.retained_capacity_bytes, total.released_bytes);
    println!("[DEBUG] paid metadata cancellation retired {} bytes", total.released_bytes);
}

#[test]
fn original_archived_set_image_metadata_matches_cold_type_refusals() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let mut reader = dsl::ByteReader::new(&frame);
    let format = reader.read_u8().unwrap();
    let tag = reader.read_u8().unwrap();
    let payload_length = reader.read_varint_u64().unwrap() as usize;
    let body = std::str::from_utf8(reader.read_bytes(payload_length).unwrap()).unwrap();
    for row in fixture["metadata"]["invalid"].as_array().unwrap() {
        let malformed = body.replacen(row[0].as_str().unwrap(), row[1].as_str().unwrap(), 1);
        assert_ne!(malformed, body, "{row}");
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(format);
        writer.write_u8(tag);
        writer.write_varint_u64(malformed.len() as u64);
        writer.write_bytes(malformed.as_bytes());
        let frame = writer.into_bytes();
        assert!(PdfMutation::decode_op(&frame).is_err(), "{row}");
        let error = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(error.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
    }
}
