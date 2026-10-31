//! 🧰️ Loads reference-row fixtures as the persistent snapshot the engine solves and retires it through its original owner.
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::RetainedCloneGrant;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot, Fem2dSnapshotValue};

/// 📥️ Persistent snapshot of `value`, validated by the original native receiver.
pub fn persist(value: &Fem2dSnapshotValue) -> Fem2dSnapshot {
    Fem2dSnapshot::from_reference(value.clone()).expect("fixture loads as a persistent snapshot")
}

/// ♻️ Retires `value` through its controlled owner.
pub fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().expect("controlled retirement admits the value");
    let grant = RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1048576, maximum_capacity_bytes: 256 * 1048576, maximum_release_bytes: 256 * 1048576, maximum_depth: 128 };
    while !owner.terminal_is_empty() {
        owner.step(grant).expect("funded retirement step");
    }
}

/// 🔬️ Runs `run` on the persistent form of `value` and retires it afterwards.
pub fn with<R>(value: &Fem2dSnapshotValue, run: impl FnOnce(&Fem2dSnapshot) -> R) -> R {
    let snapshot = persist(value);
    let result = run(&snapshot);
    drain(snapshot);
    result
}
