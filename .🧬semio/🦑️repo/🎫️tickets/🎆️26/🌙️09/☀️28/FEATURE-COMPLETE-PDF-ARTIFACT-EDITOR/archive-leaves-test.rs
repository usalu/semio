fn leaf_archive_frame(tag: u8, row: &serde_json::Value) -> Vec<u8> {
    object_archive_frame(tag, row["json"].as_str().unwrap())
}

#[test]
fn original_archive_ordinary_leaves_preserve_all_six_fixture_frames() {
    use dsl::original_receiving::receive_to_completion;
    let laws = object_archive_cases();
    for pair in laws["ordinaryLeaves"]["pairs"].as_array().unwrap() {
        let tag = pair["tag"].as_u64().unwrap() as u8;
        for side in ["forward", "inverse"] {
            let row = &pair[side];
            let frame = leaf_archive_frame(tag, row);
            let enclosing: Vec<_> = [0xff, 0].into_iter().chain(frame.iter().copied()).chain([0xff]).collect();
            let decoded = receive_to_completion::<PdfMutation>(&enclosing, (2, 2 + frame.len()), image_archive_receiving_stock()).unwrap();
            assert_eq!(decoded, PdfMutation::decode_op(&frame).unwrap());
            assert_eq!(decoded.encode_op().unwrap(), frame);
            assert_eq!(row["frameBytes"].as_u64().unwrap() as usize, frame.len());
            assert_eq!(format!("{:08x}", pack::crc32c(&frame)), row["crc32c"].as_str().unwrap());
            assert_eq!(semio_framework_hash::sha256_hex(&frame), row["sha256"].as_str().unwrap());
        }
    }
}

#[test]
fn original_archive_ordinary_leaves_preserve_exact_typed_validation() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let laws = object_archive_cases();
    for row in laws["ordinaryLeaves"]["invalid"].as_array().unwrap() {
        let frame = object_archive_frame(row[0].as_u64().unwrap() as u8, row[1].as_str().unwrap());
        assert!(PdfMutation::decode_op(&frame).is_err(), "{row}");
        let refusal = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(refusal.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
    }
    for row in laws["ordinaryLeaves"]["normalized"].as_array().unwrap() {
        let frame = object_archive_frame(row[0].as_u64().unwrap() as u8, row[1].as_str().unwrap());
        let decoded = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap();
        assert_eq!(decoded, PdfMutation::decode_op(&frame).unwrap());
    }
}

#[test]
fn original_archive_ordinary_leaves_close_before_publication_and_settle() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, ValueRefusalKind};
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;
    let laws = object_archive_cases();
    let pairs = laws["ordinaryLeaves"]["pairs"].as_array().unwrap();
    let mut frames: Vec<_> = pairs.iter().flat_map(|pair| {
        let tag = pair["tag"].as_u64().unwrap() as u8;
        [leaf_archive_frame(tag, &pair["forward"]), leaf_archive_frame(tag, &pair["inverse"])]
    }).collect();
    let extra = laws["ordinaryLeaves"]["normalized"].as_array().unwrap().last().unwrap();
    frames.push(object_archive_frame(extra[0].as_u64().unwrap() as u8, extra[1].as_str().unwrap()));
    for (index, frame) in frames.iter().enumerate() {
        let mut turns = None;
        for cancel in [false, true] {
            let limits = image_archive_receiving_stock();
            let grant = limits.quantum;
            let body = grant.maximum_copy_bytes;
            let (mut cursor, mut progress) = PdfMutation::admit_receiving(frame, (0, frame.len()), grant).result.unwrap().unwrap();
            let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
            let active = std::cell::Cell::new(true);
            let mut checkpoint = |_| active.get();
            let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
            let mut count = 0;
            let mut typed = false;
            let mut added = 0;
            let mut last_work = 0;
            while !cursor.is_ready() {
                let demand = cursor.demands(frame, body).unwrap();
                let work = cursor.step_work_demand(frame, body).unwrap();
                if !typed && work == 5 {
                    let short = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant };
                    for (grant, work) in [(short, work), (grant, work - 1)] {
                        let denied = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                        assert_eq!(denied.performed, 0);
                        assert!(denied.actual.is_none());
                        assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
                        assert_eq!(cursor.demands(frame, body).unwrap(), demand);
                    }
                }
                typed |= work == 5;
                if work == 200 && demand.capacity_bytes > 0 {
                    assert_eq!(demand.capacity_bytes, size_of::<PdfDictEntry>());
                    let short = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant };
                    let declined = wallet.run(short, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                    assert_eq!(declined.performed, 0);
                    assert!(declined.actual.is_none());
                    assert_eq!(declined.output.result.unwrap(), RetainedCloneProgress::default());
                    assert_eq!(cursor.demands(frame, body).unwrap(), demand);
                }
                if cancel && Some(count + 1) == turns { active.set(false); }
                let turn = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                if !active.get() {
                    assert_eq!(turn.performed, 0);
                    assert_eq!(turn.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
                    break;
                }
                let receipt = turn.output.result.unwrap();
                if work == 200 { added += receipt.retained_capacity_bytes; }
                assert_eq!(receipt.retained_capacity_bytes, turn.actual.unwrap_or(0));
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
                count += 1;
                last_work = work;
            }
            assert!(typed);
            assert_eq!(added, if index + 1 == frames.len() { size_of::<PdfDictEntry>() } else { 0 });
            if cancel {
                assert_eq!(cursor.take_into(&mut None, grant).work, 0);
            } else {
                assert_eq!(last_work, 2);
                turns = Some(count);
                let existing = PdfMutation::decode_op(&frames[0]).unwrap();
                let mut output = Some(existing.clone());
                let denied = cursor.take_into(&mut output, grant);
                assert_eq!(denied.work, 0);
                assert_eq!(denied.result.unwrap(), RetainedCloneProgress::default());
                assert_eq!(output.take(), Some(existing));
                assert_eq!(cursor.take_into(&mut output, grant).work, 1);
                assert_eq!(output.unwrap(), PdfMutation::decode_op(frame).unwrap());
                println!("[DEBUG] ordinary archive frame {index} tag {}: {count} turns, {added} typed allocation bytes, receipt {progress:?}", frame[1]);
            }
            while !cursor.terminal_is_empty() {
                let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
                let receipt = turn.result.unwrap().progress();
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
            }
            if cancel {
                assert_eq!(progress.retained_capacity_bytes, progress.released_bytes);
                println!("[DEBUG] canceled ordinary frame {index} with {} balanced bytes", progress.released_bytes);
            }
        }
    }
}
