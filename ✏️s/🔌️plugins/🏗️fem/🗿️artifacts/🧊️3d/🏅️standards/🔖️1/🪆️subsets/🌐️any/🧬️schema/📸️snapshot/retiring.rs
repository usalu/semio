use super::Fem3dSnapshot;

/// ♻️ Test-kit guard (`testkit` feature, never part of a production build): a snapshot held outside a store that retires through its controlled owner when the law ends. Production code retires snapshots through their store owner explicitly.
pub struct Fem3dSnapshotRetiring(Option<Fem3dSnapshot>);

impl Fem3dSnapshotRetiring {
    pub fn new(snapshot: Fem3dSnapshot) -> Self {
        Self(Some(snapshot))
    }
}

impl std::ops::Deref for Fem3dSnapshotRetiring {
    type Target = Fem3dSnapshot;
    fn deref(&self) -> &Fem3dSnapshot {
        self.0.as_ref().unwrap_or_else(|| unreachable!("a retiring snapshot holds its value until dropped"))
    }
}

impl Drop for Fem3dSnapshotRetiring {
    fn drop(&mut self) {
        let Some(snapshot) = self.0.take() else { return };
        <Fem3dSnapshot as semio_framework_value::FromValue>::retire_decoded(snapshot);
    }
}
