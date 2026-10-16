"""🧠️ Stage H: facade root, transaction oracle access through narrow accessors, authority testkit visibility."""
import os,re
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
R=f"{REPO}/{RT}"
def rd(rel): return open(f"{R}/{rel}",encoding="utf-8").read()
def wr(rel,t): open(f"{R}/{rel}","w",encoding="utf-8").write(t)
def sub1(t,old,new):
    assert t.count(old)==1,(t.count(old),old[:80]); return t.replace(old,new)
# entity accessors
e=rd("🪪️entity/🦀️.rs")
e=sub1(e,"    /// 📤️ Hands the embedder every future queued via `Context::spawn_local` since the last drain.",'''    /// 🔔️ The entities whose change notification is still queued, in queue order.
    pub fn pending_notifications(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.effects.notify.iter().copied()
    }

    /// 🔔️ The entities whose emitted event is still queued, in queue order.
    pub fn pending_emit_sources(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.effects.emit.iter().map(|(id, _)| *id)
    }

    /// 📤️ Hands the embedder every future queued via `Context::spawn_local` since the last drain.''')
wr("🪪️entity/🦀️.rs",e)
t=rd("🔄️transaction/🦀️.rs")
t=sub1(t,"for &id in runtime.store.effects.notify.iter() {","for id in runtime.store.pending_notifications() {")
t=sub1(t,"let pending_notify = runtime.store.effects.notify.iter().copied().collect();","let pending_notify = runtime.store.pending_notifications().collect();")
t=sub1(t,"let pending_emit_sources = runtime.store.effects.emit.iter().map(|(id, _)| *id).collect();","let pending_emit_sources = runtime.store.pending_emit_sources().collect();")
wr("🔄️transaction/🦀️.rs",t)
u=rd("♻️reconcile/🦀️.rs")
u=sub1(u,"    pub(crate) fn transaction_usage(&self)","    pub fn transaction_usage(&self)")
wr("♻️reconcile/🦀️.rs",u)
# facade root
old=rd("📦️packages/🦀️rust/🦀️.rs")
doc=old[:old.index("#[path = \"../../🎛️context/🦀️.rs\"]")].rstrip("\n")
EXP_EXCHANGE=["PresenceHub"]
EXP_PRES=["COMPONENT_TREE_PRODUCER_DEPTH","ComponentTree","ComponentTreeProducer","ComponentTreeProducerStep","DEFAULT_REVISION_TOLERANCE","TreeNode","UI_NATIVE_MESH_MESSAGE_BYTES","UiNativeMeshChannel","UiNativeMeshMessage","UiNativeMeshResource","UiNativeResourceLoans","UiNativeResources","is_stale_intent"]
EXP_PRIM=["SURFACE_RECONCILE_FLOOR_BYTES","SURFACE_RECONCILE_FLOOR_ITEMS","SURFACE_RECONCILE_MAX_ITEMS","SURFACE_RECONCILE_PAGE_BYTES","SurfaceReconcileLimits","surface_subtree_items"]
EXP_REC=["SURFACE_RECONCILE_ADMISSION_SLOTS","SURFACE_RECONCILE_AGGREGATE_BYTES","SurfaceReconcileJob","SurfaceReconcileJobStep","SurfaceReconcileOutputReservation","SurfaceReconcileOutputTransfer","SurfaceReconcileOutputs","SurfaceReconcilePublishedAck","SurfaceReconcilePublishedPatch","SurfaceReconcileReadyPatch","SurfaceReconcileRejected","SurfaceReconcileReservation","SurfaceReconcileTerminal","SurfaceReconciler","close_surface_reconcile_handback_one","surface_reconcile_registry_census","surface_reconcile_registry_test_guard"]
TEST_ONLY=["Command","CommandGateway","CommandId","CommandSink","CorrelationId","SinkFull","InboxOverflow","OptimisticStatus","ProjectionDelta","ProjectionInbox"]
TEST_PRES=["Context","DeferredKey","DeferredOp","DependencyTracker","DispatchOutcome","Entity","EntityId","EntityStore","HandleIntent","Present","PresentCx"]
def lst(x): return ", ".join(x)
root=doc+f'''

pub use semio_framework_ui_runtime_exchange::{{{lst(EXP_EXCHANGE)}}};
pub use semio_framework_ui_runtime_presentation::{{{lst(EXP_PRES)}}};
pub use semio_framework_ui_runtime_reconcile::{{{lst(EXP_REC)}}};
pub use semio_framework_ui_runtime_reconcile_primitives::{{{lst(EXP_PRIM)}}};

#[cfg(test)]
use semio_framework_ui_runtime_exchange::{{{lst(TEST_ONLY)}}};
#[cfg(test)]
use semio_framework_ui_runtime_presentation::{{{lst(TEST_PRES)}}};

#[cfg(test)]
#[path = "../../🔄️transaction/🦀️.rs"]
mod transaction;
#[cfg(test)]
pub use transaction::*;
'''
wr("📦️packages/🦀️rust/🦀️.rs",root)
print("ok")
