#!/usr/bin/env python3
"""🩹️ Anchored, idempotent: the presence lane's two retirement owners get a GENERIC bounded
framework default, so the 95 editors that declare a presence type and no owner stop failing closed
(ticket 26/09/18, slice S12 §3; S11 §3.3b counted them)."""
import pathlib
ROOT = pathlib.Path("/Users/ueli/Documents/semio")
RET = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/♻️retirement/🦀️.rs"
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"

ret = RET.read_text()
OWNERS = '''
/// 🧹️ Bounded retirement for a displaced presence root of ANY payload type — the presence twin of
/// `bounded_transient_root_retirement_factory`, whose lane has the identical shape (one displaced
/// local root, retired in grant-sized steps, priced by the payload's own DSL print).
///
/// It exists so the framework can OWN the presence lane by default. `PresenceStore::local_read`
/// fails closed without a live local retirement owner (`🏪️store/🦀️.rs:4716`) and the `ArtifactApp`
/// default returned `None`, so every app that declares a presence type and no owner refuses the
/// first command whose ephemeral leg reads local presence — 95 editors, of which four were found one
/// at a time by four different slices in two days (🏠️home, 📖️playbook, ⚙️playbook-module-procedural,
/// 🪐️space-index; ticket 26/09/18 S10 §2.9a, S11 §3.2/§3.3b). The viewer wrapper already defaulted
/// into the framework's own owners (`🔌️plugin/🦀️.rs:8003`); the editor wrapper could not, because
/// its `Self::Presence` is `V::Presence` and the only owner on offer was `NoPresence`-typed.
/// Declaring an owner explicitly still wins — this is `or_else`, not a replacement.
pub fn bounded_presence_root_retirement_factory<P>() -> Arc<dyn store::SnapshotRetirementFactory<P>>
where
    P: store::ArtifactDsl + Send + Sync + 'static,
{
    Arc::new(BoundedPresenceRootRetirementFactory::<P>(std::marker::PhantomData))
}

struct BoundedPresenceRootRetirementFactory<P>(std::marker::PhantomData<fn() -> P>);

impl<P> store::SnapshotRetirementFactory<P> for BoundedPresenceRootRetirementFactory<P>
where
    P: store::ArtifactDsl + Send + Sync + 'static,
{
    fn retire(&self, snapshot: Arc<P>) -> Box<dyn store::ErasedSnapshotRetirement> {
        let retained_bytes = store::ArtifactDsl::print_dsl(snapshot.as_ref()).len();
        Box::new(BoundedPresenceRootRetirement { root: Some(snapshot), retained_bytes })
    }
}

struct BoundedPresenceRootRetirement<P> {
    root: Option<Arc<P>>,
    retained_bytes: usize,
}

impl<P: Send + Sync + 'static> store::ErasedSnapshotRetirement for BoundedPresenceRootRetirement<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retained_bytes > maximum_bytes {
            self.retained_bytes -= maximum_bytes;
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: maximum_bytes });
        }
        if self.root.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none()
    }
}
'''
ANCHOR = '''pub fn no_presence_peer_retirement_factory() -> Arc<dyn store::SnapshotRetirementFactory<crate::NoPresence>> {
    Arc::new(NoPresenceRetirementFactory)
}
'''
changed = 0
if "bounded_presence_root_retirement_factory" not in ret:
    assert ret.count(ANCHOR) == 1, "presence retirement anchor not unique"
    ret = ret.replace(ANCHOR, ANCHOR + OWNERS)
    RET.write_text(ret)
    changed += 1

plugin = PLUGIN.read_text()
OLD_EXPORT = "pub use presence_retirement::{no_presence_local_root_retirement_factory, no_presence_peer_retirement_factory, no_presence_store_disposer, NoPresenceRetirementFactory, PresenceStoreOwnedDisposer};"
NEW_EXPORT = "pub use presence_retirement::{bounded_presence_root_retirement_factory, no_presence_local_root_retirement_factory, no_presence_peer_retirement_factory, no_presence_store_disposer, NoPresenceRetirementFactory, PresenceStoreOwnedDisposer};"
if NEW_EXPORT not in plugin:
    assert plugin.count(OLD_EXPORT) == 1, "export anchor not unique"
    plugin = plugin.replace(OLD_EXPORT, NEW_EXPORT)
    changed += 1

OLD_LOCAL = '''        /// 🧹️ Supplies exact bounded retirement for a displaced local presence root.
        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            None
        }'''
NEW_LOCAL = '''        /// 🧹️ Supplies exact bounded retirement for a displaced local presence root. The framework
        /// owns this lane by default (`bounded_presence_root_retirement_factory`) — an app that wants
        /// its own paging overrides it, but none has to declare one to be allowed to READ its own
        /// presence.
        fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
        }'''
if NEW_LOCAL not in plugin:
    assert plugin.count(OLD_LOCAL) == 1, "local factory anchor not unique"
    plugin = plugin.replace(OLD_LOCAL, NEW_LOCAL)
    changed += 1

OLD_PEER = '''        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            None
        }

        fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {'''
NEW_PEER = '''        /// 🧹️ The peer half of the same lane, framework-owned by the same default.
        fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
            Some(crate::bounded_presence_root_retirement_factory::<Self::Presence>())
        }

        fn build_transient_store_disposer() -> ArtifactDisposal<store::TransientStore<Self::Transient, Self::TransientMutation>> {'''
if NEW_PEER not in plugin:
    assert plugin.count(OLD_PEER) == 1, "peer factory anchor not unique"
    plugin = plugin.replace(OLD_PEER, NEW_PEER)
    changed += 1

PLUGIN.write_text(plugin)
print(f"edits applied: {changed}")
