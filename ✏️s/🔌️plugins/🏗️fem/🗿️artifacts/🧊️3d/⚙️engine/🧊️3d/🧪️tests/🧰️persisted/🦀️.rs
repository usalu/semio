//! 🧰️ Loads reference-row fixtures as the persistent snapshot the engine solves and retires it through its original owner.
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::RetainedCloneGrant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::{Fem3dSnapshot, Fem3dSnapshotValue};

/// ♻️ Retires `value` through its controlled owner.
pub fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().expect("controlled retirement admits the value");
    let grant = RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1048576, maximum_capacity_bytes: 256 * 1048576, maximum_release_bytes: 256 * 1048576, maximum_depth: 128 };
    while !owner.terminal_is_empty() {
        let (step, _) = semio_framework_value::AuthoringWallet::new(grant, 65536).run_settled(grant, 65536, |effect| owner.step(grant.maximum_copy_bytes, effect)).expect("fund retirement step").expect("retirement step admitted");
        step.result.expect("funded retirement step");
    }
}

/// 📥️ Persistent snapshot that retires through its controlled owner when the law ends, also while a failed assertion unwinds.
pub struct Persisted(Option<Fem3dSnapshot>);

impl Persisted {
    pub fn new(snapshot: Fem3dSnapshot) -> Self {
        Self(Some(snapshot))
    }
}

impl std::ops::Deref for Persisted {
    type Target = Fem3dSnapshot;
    fn deref(&self) -> &Fem3dSnapshot {
        self.0.as_ref().expect("a persisted snapshot holds its value until dropped")
    }
}

impl std::ops::DerefMut for Persisted {
    fn deref_mut(&mut self) -> &mut Fem3dSnapshot {
        self.0.as_mut().expect("a persisted snapshot holds its value until dropped")
    }
}

impl Drop for Persisted {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            drain(snapshot);
        }
    }
}

/// 📥️ Persistent snapshot of `value`, validated by the original native receiver.
pub fn persist(value: Fem3dSnapshotValue) -> Persisted {
    Persisted::new(Fem3dSnapshot::from_reference(value).expect("fixture loads as a persistent snapshot"))
}
