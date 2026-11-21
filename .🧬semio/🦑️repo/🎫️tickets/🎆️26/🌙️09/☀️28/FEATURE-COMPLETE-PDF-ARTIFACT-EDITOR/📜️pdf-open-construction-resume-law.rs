/// 🧪 Exercises a candidate whose Original receiving frame still owns its receipt.
#[test]
fn ready_never_escapes_an_unfinished_native_construction_receipt() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("🔣️receipt-recovery.json")).unwrap();
    let sweep = contract["clockSweep"].as_u64().unwrap() as usize;
    let value = contract["candidate"].as_u64().unwrap() as u8;
    let demand = OriginalConstructionDemand { items: 1, copy_bytes: 0, capacity_bytes: 1, release_bytes: 0, depth: 1 };
    let (quote, _) = original_construction_reserve::<Box<u8>>(demand, 1).unwrap();
    let mut exercised = 0;
    for reads in 0..=sweep {
        CLOCK_READS_LEFT.with(|left| left.set(usize::MAX));
        let (mut caller, mut decoder, mut admission) = prepared(9100 + reads as u64);
        let mut slot = None::<Box<u8>>;
        let ran = std::cell::Cell::new(false);
        CLOCK_READS_LEFT.with(|left| left.set(reads));
        let cut = caller.refreshed_turn_with_decoder(quote_grant(quote), &mut decoder, |cx| {
            admit_original_construction(cx, &mut slot, &mut admission, demand, 1, |_, actual| valued_byte_birth(value, &ran, actual))
        });
        CLOCK_READS_LEFT.with(|left| left.set(usize::MAX));
        if cut.is_err() || admission.is_failed() || slot.is_none() {
            std::mem::forget((caller, decoder, admission, slot));
            continue;
        }
        let original = **slot.as_ref().unwrap();
        assert_eq!(original, value);
        if admission.pending_recovery().is_some() {
            exercised += 1;
        }
        let mut settled = false;
        for _ in 0..32 {
            let status = caller.refreshed_turn_with_decoder(stock(), &mut decoder, |cx| {
                let result = admit_original_construction(cx, &mut slot, &mut admission, demand, 1, |_, _| panic!("born candidate may never be constructed twice"))?;
                if result == OriginalConstructionStatus::Ready {
                    assert!(admission.reset_step(cx)?, "Ready must permit a paid reset");
                }
                Ok(result)
            }).unwrap();
            if status == OriginalConstructionStatus::Ready {
                settled = true;
                break;
            }
        }
        assert!(settled, "Original receipt must settle after its clock resumes");
        assert_eq!(slot.as_deref(), Some(&value));
        retire(&mut caller, &mut slot);
        close_admission(&mut caller, &mut admission);
        finish(&mut caller, &mut decoder);
        caller.retire(stock());
    }
    assert!(exercised > 0, "the clock sweep must interrupt a born candidate before receipt closure");
}
