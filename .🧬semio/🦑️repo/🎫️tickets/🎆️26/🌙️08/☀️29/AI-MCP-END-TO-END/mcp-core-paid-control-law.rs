use super::*;

#[test]
fn paid_atomic_control_matches_wasmtime_and_receipts() {
    let cases: &[(&[u8], i32, i32, u64, u64, bool)] = include!(env!("SEMIO_LINKED_CONTROL_CASES"));
    for &(bytes, argument, expected, prefix_fuel, expected_fuel, retire_label) in cases {
        verify_transfer(bytes, argument, expected, prefix_fuel, expected_fuel, retire_label);
    }
}

fn verify_transfer(bytes: &[u8], argument: i32, expected: i32, prefix_fuel: u64, expected_fuel: u64, retire_label: bool) {
    let engine = wasmtime::Engine::default();
    let oracle = wasmtime::Module::new(&engine, bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &oracle, &[]).unwrap();
    assert_eq!(instance.get_typed_func::<i32, i32>(&mut store, "run").unwrap().call(&mut store, argument).unwrap(), expected);
    let module = Arc::new(CoreModule::parse(bytes).unwrap());
    let width = size_of::<usize>();
    let copy = (2 + usize::from(retire_label) * 10) * width;
    let retained_copy = 7 * width + size_of::<Value>();
    let exact = RetainedCloneGrant {
        maximum_items: 1, maximum_copy_bytes: copy,
        maximum_capacity_bytes: 0, maximum_release_bytes: 0,
        ..TURN
    };
    let mut direct_effects = RetainedCloneProgress::default();
    for (short, cancelled) in [(false, false), (true, false), (true, true)] {
        let mut caller = Caller::new();
        let mut owner = caller.turn(TURN, |cx| CoreInstanceOpening::begin(&module, cx)).0.unwrap();
        while !owner.resources_ready() {
            caller.turn(TURN, |cx| owner.advance(&module, cx));
        }
        assert!(caller.turn(TURN, |cx| owner.begin_function_step(&module, 0, &[Value::I32(argument)], cx)).0);
        for _ in 0..prefix_fuel {
            let call = caller.turn(TURN, |cx| step(&mut owner, &module, cx)).0;
            assert_eq!(call.execution.spent_fuel, Some(1));
            assert_eq!(call.result.unwrap(), Status::Pending);
        }
        let (blocked, (_, _, receipt)) = caller.turn(RetainedCloneGrant { maximum_items: 0, ..exact }, |cx| step(&mut owner, &module, cx));
        assert_eq!(receipt, RetainedCloneProgress::default());
        assert_eq!(blocked.execution.spent_fuel, Some(0));
        assert_eq!(blocked.result.unwrap(), Status::Pending);
        let grant = RetainedCloneGrant { maximum_copy_bytes: copy - usize::from(short), ..exact };
        let ((call, work), (_, _, receipt)) = caller.turn(grant, |cx| {
            let prior = cx.original_work_units_spent();
            let call = step(&mut owner, &module, cx)?;
            Ok((call, cx.original_work_units_spent() - prior))
        });
        let charged_copy = if !short { copy } else if grant.maximum_copy_bytes >= retained_copy { retained_copy } else { 0 };
        let item = usize::from(charged_copy > 0);
        assert_eq!(receipt, RetainedCloneProgress {
            copied_items: item, copied_bytes: charged_copy, ..Default::default()
        });
        assert_eq!(work, item as u128);
        assert_eq!(call.execution.spent_fuel, Some(item as u64));
        assert_eq!(call.result.unwrap(), Status::Pending);
        if cancelled {
            caller.turn(TURN, |cx| { cx.original_cancel_token().cancel_now(); Ok(()) });
            let (call, (_, _, receipt)) = caller.turn(exact, |cx| step(&mut owner, &module, cx));
            assert_eq!(call.result.unwrap(), Status::Cancelled);
            assert_eq!(call.execution.spent_fuel, Some(0));
            assert_eq!(receipt, RetainedCloneProgress::default());
        } else {
            let mut spent = prefix_fuel + item as u64;
            let mut effects = receipt;
            for _ in 0..40 {
                let (call, (_, _, receipt)) = caller.turn(TURN, |cx| step(&mut owner, &module, cx));
                spent += call.execution.spent_fuel.unwrap();
                effects = effects.checked_add(receipt).unwrap();
                if call.result.unwrap() == Status::Complete { break }
            }
            assert_eq!(spent, expected_fuel);
            assert_eq!(owner.invocation_values(), Some(&[Value::I32(expected)][..]));
            if short {
                assert_eq!(effects, RetainedCloneProgress {
                    copied_items: direct_effects.copied_items + 2 * usize::from(retire_label),
                    copied_bytes: direct_effects.copied_bytes + usize::from(retire_label) * (7 * width + retained_copy),
                    ..direct_effects
                });
            } else {
                direct_effects = effects;
            }
            caller.turn(TURN, |cx| { cx.original_cancel_token().cancel_now(); Ok(()) });
        }
        for _ in 0..100 {
            if owner.terminal_is_empty() { break }
            caller.turn(TURN, |cx| owner.close_step(&module, cx));
        }
        assert!(owner.terminal_is_empty());
        caller.retire();
    }
}
