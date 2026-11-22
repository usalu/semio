use super::*;

#[test]
fn paid_control_growth_matches_wasmtime_and_receipts() {
    let bytes = include_bytes!(env!("SEMIO_LINKED_LABEL_PROGRAM"));
    let input = env!("SEMIO_LINKED_LABEL_INPUT").parse::<i32>().unwrap();
    let expected = env!("SEMIO_LINKED_LABEL_EXPECTED").parse::<i32>().unwrap();
    let peak = env!("SEMIO_LINKED_LABEL_PEAK").parse::<usize>().unwrap();
    let engine = wasmtime::Engine::default();
    let oracle = wasmtime::Module::new(&engine, bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &oracle, &[]).unwrap();
    assert_eq!(instance.get_typed_func::<i32, i32>(&mut store, "run").unwrap().call(&mut store, input).unwrap(), expected);
    let module = Arc::new(CoreModule::parse(bytes).unwrap());
    for cancelled in [false, true] {
        let mut caller = Caller::new();
        let mut owner = caller.turn(TURN, |cx| CoreInstanceOpening::begin(&module, cx)).0.unwrap();
        for _ in 0..100 {
            if owner.resources_ready() { break; }
            caller.turn(TURN, |cx| owner.advance(&module, cx));
        }
        assert!(owner.resources_ready());
        let (opened, (_, _, receipt)) = caller.turn(TURN, |cx| owner.begin_function_step(&module, 0, &[Value::I32(input)], cx));
        assert!(opened);
        let mut live = receipt.retained_capacity_bytes;
        let mut scalar = |grant| {
            let (call, (_, _, physical)) = caller.turn(grant, |cx| step(&mut owner, &module, cx));
            live += physical.retained_capacity_bytes;
            live -= physical.released_bytes;
            (call.result.unwrap(), call.execution.spent_fuel.unwrap(), physical)
        };
        let (_, used, first) = scalar(TURN);
        assert_eq!(used, 1);
        let width = first.released_bytes;
        assert!(width > 0 && first.retained_capacity_bytes > 2 * width);
        let types = first.retained_capacity_bytes - 2 * width;
        let write = first.copied_bytes - width;
        assert_eq!(write, types + width + std::mem::size_of::<Value>());
        assert_eq!(scalar(TURN).1, 1);
        for refused in [
            RetainedCloneGrant { maximum_copy_bytes: write + 2 * width - 1, ..TURN },
            RetainedCloneGrant { maximum_capacity_bytes: types + 3 * width - 1, ..TURN },
            RetainedCloneGrant { maximum_release_bytes: 2 * width - 1, ..TURN },
        ] {
            assert_eq!(scalar(refused), (Status::Pending, 0, RetainedCloneProgress::default()));
        }
        let tight = if cancelled {
            RetainedCloneGrant { maximum_release_bytes: 3 * width, ..TURN }
        } else {
            RetainedCloneGrant { maximum_capacity_bytes: types + 3 * width, ..TURN }
        };
        let (status, used, third) = scalar(tight);
        assert_eq!((status, used, third.copied_bytes, third.retained_capacity_bytes, third.released_bytes),
            (Status::Pending, 1, write + 2 * width, types + 3 * width, 2 * width));
        let mut capacity = 3;
        let mut relocated = width + third.released_bytes;
        for depth in 3..peak {
            let (status, used, physical) = scalar(TURN);
            assert_eq!((status, used), (Status::Pending, 1));
            let growing = capacity == depth;
            let moved = if growing { capacity * width } else { 0 };
            if growing { capacity = (physical.retained_capacity_bytes - types) / width; }
            assert_eq!((physical.copied_bytes, physical.retained_capacity_bytes, physical.released_bytes),
                (write + moved, if growing { capacity * width + types } else { types }, moved));
            relocated += moved;
        }
        eprintln!("[DEBUG] paid Core labels: peak={peak}, capacity={capacity}, moved={relocated}, label width={width}");
        if cancelled {
            caller.turn(TURN, |cx| { cx.original_cancel_token().cancel_now(); Ok(()) });
            let (call, (_, _, physical)) = caller.turn(TURN, |cx| step(&mut owner, &module, cx));
            assert_eq!((call.result.unwrap(), call.execution.spent_fuel, physical),
                (Status::Cancelled, Some(0), RetainedCloneProgress::default()));
        } else {
            for _ in 0..500 {
                if scalar(TURN).0 == Status::Complete { break; }
            }
            assert_eq!(owner.invocation_values(), Some(&[Value::I32(expected)][..]));
        }
        for _ in 0..500 {
            let (done, (_, _, physical)) = caller.turn(TURN, |cx| owner.close_invocation_step(&module, cx));
            assert_eq!(physical.retained_capacity_bytes, 0);
            live -= physical.released_bytes;
            if done { break; }
        }
        assert_eq!(live, 0);
        for _ in 0..100 {
            if owner.terminal_is_empty() { break; }
            caller.turn(TURN, |cx| owner.close_step(&module, cx));
        }
        assert!(owner.terminal_is_empty());
        caller.retire();
        assert!(relocated < 2 * peak * width, "labels moved {relocated} bytes at depth {peak}");
    }
}
