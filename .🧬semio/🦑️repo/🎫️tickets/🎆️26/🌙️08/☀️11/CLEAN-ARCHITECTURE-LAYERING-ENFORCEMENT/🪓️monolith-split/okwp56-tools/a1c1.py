# 🔌 C1: the store-close port. `ArtifactStoreCloseView` and the owned disposers see the composer only through `ArtifactStoreCloseHost` (PLAN-OS-KERNEL-CRATES.md 5.3, 7.5).
import re
HOST_TRAIT = '''/// 🔌️ What the owned disposers may ask of the store being closed: the close contract is base, the engine type behind it is not named below. The composer implements it by delegating to its components.
pub trait ArtifactStoreCloseHost<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    fn close_host_member_inbox_capacity(&self) -> usize;
    fn close_host_backbone_attached(&self) -> bool;
    fn close_host_prefix_ring_is_empty(&self) -> bool;
    fn close_host_snapshot_read_leases(&self) -> &SnapshotReadLeaseRegistryOwner;
    fn close_host_initial_snapshot_retirement_factory(&self) -> Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>;
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
    fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
    fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn owned_roots_terminal_is_empty(&self) -> bool;
}

'''
HOST_IMPL = '''impl<P, Mutation> ArtifactStoreCloseHost<P, Mutation> for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_host_member_inbox_capacity(&self) -> usize { self.wire.member_inbox.capacity() }
    fn close_host_backbone_attached(&self) -> bool { self.wire.backbone.is_some() }
    fn close_host_prefix_ring_is_empty(&self) -> bool { self.live.prefix_ring.is_empty() }
    fn close_host_snapshot_read_leases(&self) -> &SnapshotReadLeaseRegistryOwner { &self.owners.snapshot_read_leases }
    fn close_host_initial_snapshot_retirement_factory(&self) -> Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>> { self.owners.initial_snapshot_retirement_factory.as_ref() }
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool { ArtifactStore::snapshot_read_leases_terminal_is_empty(self) }
    fn maintenance_retirements_terminal_is_empty(&self) -> bool { ArtifactStore::maintenance_retirements_terminal_is_empty(self) }
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { ArtifactStore::maintenance_retirements_step(self, grant) }
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { ArtifactStore::close_take_backbone_retirement(self, grant) }
    fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { ArtifactStore::close_cursor_admit(self, phase, grant) }
    fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { ArtifactStore::close_cursor_demands(self, phase, body) }
    fn owned_roots_terminal_is_empty(&self) -> bool { ArtifactStore::owned_roots_terminal_is_empty(self) }
}

'''
HOST_DELEGATES = {'close_take_backbone_retirement', 'close_cursor_demands'}

def sub1(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:70])
    return text.replace(old, new)

def patch_c1(text):
    # view
    text = sub1(text, "    store: &'a mut ArtifactStore<P, Mutation>,\n}", "    store: &'a mut dyn ArtifactStoreCloseHost<P, Mutation>,\n}")
    text = sub1(text, "if self.store.member_inbox.capacity() != 0 || self.store.backbone.is_some() {", "if self.store.close_host_member_inbox_capacity() != 0 || self.store.close_host_backbone_attached() {")
    # trait methods taking the store
    text = sub1(text, "fn terminal_is_empty(&self, store: &ArtifactStore<P, Mutation>) -> bool;", "fn terminal_is_empty(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>) -> bool;")
    text = sub1(text, "fn demands(&self, store: &ArtifactStore<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;", "fn demands(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;")
    # disposer impl
    text = sub1(text, "if self.terminal_is_empty(view.store) { return Ok(RetainedCloneStep::Complete(empty)); }", "if self.terminal_is_empty(&*view.store) { return Ok(RetainedCloneStep::Complete(empty)); }")
    text = sub1(text, "let demand = self.demands(view.store, grant.maximum_copy_bytes)?;", "let demand = self.demands(&*view.store, grant.maximum_copy_bytes)?;")
    text = sub1(text, "let factory = store.initial_snapshot_retirement_factory.as_ref().ok_or_else(", "let factory = store.close_host_initial_snapshot_retirement_factory().ok_or_else(")
    text = sub1(text, "return advance_returned_snapshot_read(&store.snapshot_read_leases, &mut self.active, factory, grant)", "return advance_returned_snapshot_read(store.close_host_snapshot_read_leases(), &mut self.active, factory, grant)")
    text = sub1(text, "ArtifactStoreCursorDisposerPhase::PrefixSnapshots if store.prefix_ring.is_empty() =>", "ArtifactStoreCursorDisposerPhase::PrefixSnapshots if store.close_host_prefix_ring_is_empty() =>")
    text = sub1(text, "ArtifactStoreCursorDisposerPhase::Backbone if store.member_inbox.capacity() == 0 && store.backbone.is_none() =>", "ArtifactStoreCursorDisposerPhase::Backbone if store.close_host_member_inbox_capacity() == 0 && !store.close_host_backbone_attached() =>")
    text = sub1(text, "fn demands(&self, store: &ArtifactStore<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {\n        if let Some(active)", "fn demands(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {\n        if let Some(active)")
    text = sub1(text, "fn terminal_is_empty(&self, store: &ArtifactStore<P, Mutation>) -> bool {\n        self.phase", "fn terminal_is_empty(&self, store: &dyn ArtifactStoreCloseHost<P, Mutation>) -> bool {\n        self.phase")
    # insert trait before the view struct and impl after the Drop impl of the cursor disposer
    k = text.index("pub struct ArtifactStoreCloseView<'a, P, Mutation>")
    text = text[:k] + HOST_TRAIT + text[k:]
    return text
