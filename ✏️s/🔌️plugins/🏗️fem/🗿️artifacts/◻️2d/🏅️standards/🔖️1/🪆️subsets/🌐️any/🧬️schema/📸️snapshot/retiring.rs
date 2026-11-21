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
        <Fem2dSnapshot as semio_framework_value::FromValue>::retire_decoded(snapshot);
    }
}
