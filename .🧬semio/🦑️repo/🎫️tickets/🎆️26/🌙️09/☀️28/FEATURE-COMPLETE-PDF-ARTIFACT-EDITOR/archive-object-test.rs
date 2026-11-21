fn object_archive_cases() -> serde_json::Value {
    serde_json::from_str(include_str!("set-image-donation.json")).unwrap()
}

fn object_archive_frame(tag: u8, json: &str) -> Vec<u8> {
    let mut writer = dsl::ByteWriter::new();
    writer.write_u8(1);
    writer.write_u8(tag);
    writer.write_varint_u64(json.len() as u64);
    writer.write_bytes(json.as_bytes());
    writer.into_bytes()
}

fn object_archive_fixtures() -> (Vec<u8>, Vec<u8>) {
    let cases = object_archive_cases();
    let forward = object_archive_frame(cases["archive"]["tag"].as_u64().unwrap() as u8, cases["archive"]["json"].as_str().unwrap());
    let inverse = object_archive_frame(cases["objects"]["inverseTag"].as_u64().unwrap() as u8, cases["objects"]["inverseJson"].as_str().unwrap());
    (forward, inverse)
}

#[test]
fn original_archive_object_pair_preserves_raw_bytes_and_typed_role_inverse() {
    use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue};
    use dsl::original_receiving::receive_to_completion;
    let (forward, inverse) = object_archive_fixtures();
    let frozen = forward.clone();
    assert_eq!(forward.len(), 141);
    assert_eq!(&forward[..8], &[1, 6, 137, 1, b'{', b'"', b'm', b'u']);
    assert_eq!(pack::crc32c(&forward[..20]), 0x94c0df0d);
    assert_eq!(u32::from_le_bytes(forward[20..24].try_into().unwrap()), 0x4f747265);
    let mut output = Vec::new();
    for frame in [&forward, &inverse] {
        let oracle = PdfMutation::decode_op(frame).unwrap();
        let original: Vec<_> = [0xff, 0].into_iter().chain(frame.iter().copied()).chain([0xff]).collect();
        let value = receive_to_completion::<PdfMutation>(&original, (2, original.len() - 1), image_archive_receiving_stock()).unwrap();
        assert_eq!(value, oracle);
        let exact = value.encode_op().unwrap();
        assert_eq!(exact, oracle.encode_op().unwrap());
        let mut reader = dsl::ByteReader::new(frame);
        assert_eq!(reader.read_u8().unwrap(), 1);
        let _ = reader.read_u8().unwrap();
        let length = reader.read_varint_u64().unwrap() as usize;
        let json: serde_json::Value = serde_json::from_slice(reader.read_bytes(length).unwrap()).unwrap();
        assert_eq!(json["id"]["num"], 5);
        assert_eq!(reader.remaining(), 0);
        println!("[DEBUG] object frame {} bytes crc32c {:08x} sha256 {}; canonical {} bytes crc32c {:08x} sha256 {}",
            frame.len(), pack::crc32c(frame), semio_framework_hash::sha256_hex(frame),
            exact.len(), pack::crc32c(&exact), semio_framework_hash::sha256_hex(&exact));
        output.push(value);
    }
    assert_eq!(forward, frozen);
    assert_eq!(output[1].encode_op().unwrap(), inverse);
    assert!(output[0].encode_op().unwrap().len() < forward.len());
    let (PdfMutation::InsertObject(insert), PdfMutation::RemoveObject(remove)) = (&output[0], &output[1]) else { panic!("typed InsertObject and RemoveObject expected"); };
    assert_eq!(insert.id, ObjRef { num: 5, r#gen: 0 });
    assert_eq!(insert.value, PdfObject::Null);
    assert_eq!(insert.index, None);
    assert_eq!(remove.id, insert.id);
    let prior = vec![PdfAdmittedStreamRole {
        identity: PdfGraphIdentity { owner: ObjRef { num: 2, r#gen: 0 }, path: Vec::new() },
        dependencies: Vec::new(),
        value: PdfStreamRoleValue::MetadataText { text: "Neutral restored role.".into() },
    }];
    let removed = insert.admitted_stream_roles.as_ref().unwrap();
    let restored = remove.admitted_stream_roles.as_ref().unwrap();
    let intervening = removed.apply(&prior);
    assert!(intervening.is_empty());
    assert_eq!(restored, &removed.inverse(&prior));
    assert_eq!(restored.apply(&intervening), prior);
}

#[test]
fn original_archive_objects_keep_frame_and_type_refusals() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let cases = object_archive_cases();
    let (forward, _) = object_archive_fixtures();
    for row in cases["archive"]["refusals"].as_array().unwrap() {
        let malformed = if let Some(json) = row["json"].as_str() {
            object_archive_frame(row["tag"].as_u64().unwrap() as u8, json)
        } else {
            let header: Vec<u8> = serde_json::from_value(row["header"].clone()).unwrap();
            header.into_iter().chain(forward[4..].iter().copied()).collect()
        };
        assert_eq!(PdfMutation::decode_op(&malformed).is_ok(), row["coldAccepts"].as_bool().unwrap_or(false), "{row}");
        let expected = if row["kind"] == "work-limit" { ValueRefusalKind::WorkLimit } else { ValueRefusalKind::InvalidValue };
        let error = receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(error.refusal_kind(), Some(expected), "{row}");
    }
    for (tag, template, rows) in [
        (6, &cases["archive"]["json"], &cases["objects"]["invalidForward"]),
        (7, &cases["objects"]["inverseJson"], &cases["objects"]["invalidInverse"]),
    ] {
        let template = template.as_str().unwrap();
        for row in rows.as_array().unwrap() {
            let body = template.replacen(row[0].as_str().unwrap(), row[1].as_str().unwrap(), 1);
            assert_ne!(body, template);
            let malformed = object_archive_frame(tag, &body);
            assert!(PdfMutation::decode_op(&malformed).is_err(), "{row}");
            let error = receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), image_archive_receiving_stock()).unwrap_err();
            assert_eq!(error.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
        }
    }
}

#[test]
fn original_archive_objects_settle_each_storage_or_cancel() {
    use crate::standards::v1_7::subsets::base::schema::{diff::PdfIndexedItem, stream_roles::PdfAdmittedStreamRole};
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, ValueRefusalKind};
    let (forward, inverse) = object_archive_fixtures();
    for (frame, size) in [(&forward, size_of::<usize>()), (&inverse, size_of::<PdfIndexedItem<PdfAdmittedStreamRole>>())] {
        for cancel in [false, true] {
            let limits = image_archive_receiving_stock();
            let grant = limits.quantum;
            let body = grant.maximum_copy_bytes;
            let (mut cursor, admitted) = PdfMutation::admit_receiving(frame, (0, frame.len()), grant).result.unwrap().unwrap();
            assert!(cursor.demands(&frame.clone(), body).is_err());
            let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
            let mut progress = admitted;
            let active = std::cell::Cell::new(true);
            let mut checkpoint = |_| active.get();
            let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
            let mut allocations = Vec::new();
            while !cursor.is_ready() {
                let work = cursor.step_work_demand(frame, body).unwrap();
                let demand = cursor.demands(frame, body).unwrap();
                if work > 8 && demand.capacity_bytes > 0 {
                    assert_eq!(demand.capacity_bytes, size);
                    let short = RetainedCloneGrant { maximum_capacity_bytes: size - 1, ..grant };
                    let denied = wallet.run(short, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                    assert_eq!(denied.performed, 0);
                    assert_eq!(denied.actual, None);
                    assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
                    assert_eq!(cursor.demands(frame, body).unwrap(), demand);
                }
                let turn = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                let receipt = turn.output.result.unwrap();
                assert_eq!(receipt.retained_capacity_bytes, turn.actual.unwrap_or(0));
                if work > 8 && receipt.retained_capacity_bytes > 0 { allocations.push(receipt.retained_capacity_bytes); }
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
                if cancel && !allocations.is_empty() { break; }
            }
            assert_eq!(allocations, [size]);
            if cancel {
                active.set(false);
                let work = cursor.step_work_demand(frame, body).unwrap();
                let denied = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                assert_eq!(denied.performed, 0);
                assert_eq!(denied.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
                assert_eq!(cursor.take_into(&mut None, grant).work, 0);
            } else {
                let mark = PdfMutation::decode_op(&inverse).unwrap();
                let mut target = Some(mark.clone());
                let denied = cursor.take_into(&mut target, grant);
                assert_eq!(denied.work, 0);
                assert_eq!(denied.result.unwrap(), RetainedCloneProgress::default());
                assert_eq!(target.take(), Some(mark));
                assert_eq!(cursor.take_into(&mut target, grant).work, 1);
                assert_eq!(target.unwrap(), PdfMutation::decode_op(frame).unwrap());
                println!("[DEBUG] archive tag {} carried {size} typed bytes; receiving receipt {progress:?}", frame[1]);
            }
            while !cursor.terminal_is_empty() {
                let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
                let receipt = turn.result.unwrap().progress();
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
            }
            if cancel {
                assert_eq!(progress.retained_capacity_bytes, progress.released_bytes);
                println!("[DEBUG] canceled archive tag {} with {} balanced bytes", frame[1], progress.released_bytes);
            }
        }
    }
}
