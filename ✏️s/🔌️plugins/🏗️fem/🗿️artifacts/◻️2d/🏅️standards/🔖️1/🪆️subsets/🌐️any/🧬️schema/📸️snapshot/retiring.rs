use super::Fem2dSnapshot;

/// ♻️ Test-kit guard (`testkit` feature, never part of a production build): a snapshot held outside a store that retires through its controlled owner when the law ends. Production code retires snapshots through their store owner explicitly.
pub struct Fem2dSnapshotRetiring(Option<Fem2dSnapshot>);

impl Fem2dSnapshotRetiring {
    pub fn new(snapshot: Fem2dSnapshot) -> Self {
        Self(Some(snapshot))
    }
}

impl std::ops::Deref for Fem2dSnapshotRetiring {
    type Target = Fem2dSnapshot;
    fn deref(&self) -> &Fem2dSnapshot {
        self.0.as_ref().unwrap_or_else(|| unreachable!("a retiring snapshot holds its value until dropped"))
    }
}

impl Drop for Fem2dSnapshotRetiring {
    fn drop(&mut self) {
        let Some(snapshot) = self.0.take() else { return };
        let Ok(mut owner) = semio_framework_value::retirement::controlled::ControlledRetirement::new(snapshot) else { return };
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1048576, maximum_capacity_bytes: 256 * 1048576, maximum_release_bytes: 256 * 1048576, maximum_depth: 128 };
        while !owner.terminal_is_empty() && owner.step(grant).is_ok() {}
    }
}
