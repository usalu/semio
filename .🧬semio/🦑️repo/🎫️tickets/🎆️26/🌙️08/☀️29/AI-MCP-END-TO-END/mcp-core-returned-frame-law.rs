use super::*;
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_plugin_interpreter::ValueType;

#[test]
fn returned_frame_quotes_actual_allocator_receipt() {
    let bytes = include_bytes!(env!("SEMIO_LINKED_RETURNED_FRAME"));
    let engine = wasmtime::Engine::default();
    let oracle = wasmtime::Module::new(&engine, bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &oracle, &[]).unwrap();
    let expected = instance.get_typed_func::<(), i64>(&mut store, "invoke").unwrap().call(&mut store, ()).unwrap();
    let module = Arc::new(CoreModule::parse(bytes).unwrap());
    let mut caller = Caller::new();
    let mut owner = caller.turn(TURN, |cx| CoreInstanceOpening::begin(&module, cx)).0.unwrap();
    for _ in 0..32 {
        if owner.resources_ready() { break; }
        caller.turn(TURN, |cx| owner.advance(&module, cx));
    }
    assert!(caller.turn(TURN, |cx| owner.begin_function_step(&module, 0, &[], cx)).0);
    for _ in 0..3 {
        assert_eq!(caller.turn(TURN, |cx| step(&mut owner, &module, cx)).0.execution.spent_fuel, Some(1));
    }
    let limited = RetainedCloneGrant { maximum_release_bytes: size_of::<ValueType>() * 2, ..TURN };
    let mut quote = None;
    for _ in 0..32 {
        let result = caller.turn(limited, |cx| {
            let call = step(&mut owner, &module, cx)?;
            assert_eq!(call.result.unwrap(), Status::Pending);
            owner.returned_frame_demand(&module, cx)
        }).0;
        if result.is_some() { quote = result; break; }
    }
    let quote = quote.expect("returned callee must hold its control allocation");
    let copy = quote.copy_bytes;
    let release = quote.release_bytes;
    assert_eq!(copy, size_of::<Vec<Value>>());
    assert!(release > limited.maximum_release_bytes);
    for denied in [
        RetainedCloneGrant { maximum_release_bytes: release - 1, ..TURN },
        RetainedCloneGrant { maximum_copy_bytes: copy - 1, ..TURN },
    ] {
        let (call, (_, _, spent)) = caller.turn(denied, |cx| {
            assert_eq!(owner.returned_frame_demand(&module, cx)?, Some(quote));
            assert!(retained_clone_grant_admits(cx.original_source_total_grant(), quote));
            assert!(!retained_clone_grant_admits(cx.retained_grant(), quote));
            step(&mut owner, &module, cx)
        });
        assert_eq!((call.result.unwrap(), call.execution.spent_fuel, spent),
            (Status::Pending, Some(0), RetainedCloneProgress::default()));
    }
    let exact = RetainedCloneGrant { maximum_copy_bytes: copy, maximum_release_bytes: release, ..TURN };
    let (call, (_, actual_release, spent)) = caller.turn(exact, |cx| step(&mut owner, &module, cx));
    assert_eq!((call.execution.spent_fuel, actual_release, spent.copied_bytes, spent.released_bytes),
        (Some(0), release, copy, release));
    for _ in 0..32 {
        if caller.turn(TURN, |cx| step(&mut owner, &module, cx)).0.result.unwrap() == Status::Complete { break; }
    }
    assert_eq!(owner.invocation_values(), Some(&[Value::I64(expected)][..]));
    for _ in 0..32 {
        if owner.terminal_is_empty() { break; }
        caller.turn(TURN, |cx| owner.close_step(&module, cx));
    }
    assert!(owner.terminal_is_empty());
    caller.retire();
}
