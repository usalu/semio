use crate::*;

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_guest_refusal_host_preserves_causes_diagnostics_and_actual_vm_cancellation() {
    use actor_bindings::exports::semio::framework::codec::{SnapshotRejection, ValueRefusalKind as Wire};
    use semio_framework_value::ValueRefusalKind as Native;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/🪶️sqlite/⚠️refusal/🧫️fixtures/🔣️.json")).unwrap();
    let diagnostic = &fixture["diagnostic"];
    let diagnostics = vec![semio_framework::Diagnostic {
        code: semio_framework::FaultCode::new(diagnostic["code"].as_str().unwrap()),
        severity: semio_framework::Severity::Error,
        span: semio_framework::TextSpan::at(u32::try_from(diagnostic["line"].as_u64().unwrap()).unwrap(), u32::try_from(diagnostic["column"].as_u64().unwrap()).unwrap()),
        message: diagnostic["message"].as_str().unwrap().into(),
        expected: None,
        scope: semio_framework::FaultScope::default(),
    }];
    let kinds = [(Wire::InvalidValue, Native::InvalidValue), (Wire::Canceled, Native::Canceled), (Wire::OwnershipLimit, Native::OwnershipLimit), (Wire::AllocationFailed, Native::AllocationFailed), (Wire::WorkLimit, Native::WorkLimit), (Wire::DepthLimit, Native::DepthLimit), (Wire::UnsupportedOwner, Native::UnsupportedOwner), (Wire::InvariantViolated, Native::InvariantViolated)];
    for ((wire, native), expected) in kinds.into_iter().zip(fixture["kinds"].as_array().unwrap()) {
        assert_eq!(native.as_str(), expected.as_str().unwrap());
        let rejection = wit_snapshot_rejection(SnapshotRejection { kind: wire, message: fixture["message"].as_str().unwrap().into(), diagnostics: sqlite_wire::encode_diagnostics(&diagnostics) });
        assert_eq!(rejection.kind, native);
        let original = rejection.into_io_error().unwrap();
        assert_eq!(original.cause.kind, native);
        assert_eq!(original.cause.message, fixture["message"].as_str().unwrap());
        assert_eq!(original.diagnostics, diagnostics);
    }
    let engine = wasmtime::Engine::default();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let epoch = EpochDeadlines::new(&engine, &pool);
    let deadline = EpochDeadlineCell::default();
    let cancellation = GuestCallCancellation::default();
    let done = observe_sqlite_guest(async { 7usize }, &epoch, &deadline, |_, _| {}, &cancellation).await.unwrap();
    assert_eq!(done, 7);
    cancellation.cancel();
    assert!(matches!(observe_sqlite_guest(async { 7usize }, &epoch, &deadline, |_, _| {}, &cancellation).await, Err(TurnFault::Cancelled)));
}
