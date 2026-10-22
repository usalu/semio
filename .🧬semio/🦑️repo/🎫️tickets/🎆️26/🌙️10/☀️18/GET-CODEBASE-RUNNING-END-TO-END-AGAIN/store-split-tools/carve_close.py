import os,re,sys
R="/Users/ueli/Documents/semio"
S=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
src=S+"/⚙️engine/📥️lifted/🏪️store/🚦️cursor-admission-dispatch-and-apply-batch/🦀️.rs"
dst_dir=S+"/⚙️engine/🧯️close"
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
s=open(src,encoding='utf8').read()
a=s.index("    pub fn close_cursor_admit(")
b=s.index("    pub(super) fn close_structural_owners_terminal_is_empty(")
c=s.index("    pub fn owned_roots_terminal_is_empty(")
d=s.index("    pub async fn fold_history<")
admit=s[a:b].rstrip("\n")+"\n"
structural=s[b:c].rstrip("\n")+"\n"
roots=s[c:d].rstrip("\n")+"\n"
assert "self.close_take_backbone_retirement(grant)" in admit
assert admit.count("self.snapshot_read_leases_terminal_is_empty()")==1
admit=admit.replace("    pub fn close_cursor_admit(","    fn close_cursor_admit(",1)
admit=admit.replace("self.snapshot_read_leases_terminal_is_empty()","self.owners.snapshot_read_leases_terminal_is_empty()")
structural=structural.replace("    pub(super) fn close_structural","    fn close_structural",1)
roots=roots.replace("    pub fn owned_roots","    fn owned_roots",1)
ret="Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>"
lifted="""//! 🧯️ The close witnesses and cursor admission of one store over its component bundle.

use semio_framework_os_spr::{OpBinary, OpText};
use semio_framework_os_store_engine_live::lifted_store::{StoreParts, StorePartsRef};
use semio_framework_os_store_ledger::{ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, PendingCommandReport};
use semio_framework_os_store_read::{ArtifactStoreCursorDisposerPhase, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, retire_document_envelope};
use semio_framework_os_store_replay::{EffectiveSupersessions, PrefixSnapshot};
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::{ErasedSnapshotRetirement, FromValue, ToValue, ValueError};
use std::sync::Arc;
use super::{StoreCloseAdmission, StoreCloseWitness};

//#region 🔖️ArtifactStore
impl<P, Mutation> StoreCloseAdmission<P, Mutation> for StoreParts<'_, P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> %(ret)s {
        self.wire.close_take_backbone_retirement(grant)
    }

%(admit)s}

impl<P, Mutation> StoreCloseWitness for StorePartsRef<'_, P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P>,
{
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        self.owners.snapshot_read_leases_terminal_is_empty()
    }

%(structural)s
%(roots)s}
//#endregion 🔖️ArtifactStore
""" % dict(ret=ret,admit=admit,structural=structural,roots=roots)
root="""//! 🧯️ Close witnesses and cursor admission over the StoreParts bundle.
//!
//! 🧭️ Layering and decomposition: `PLAN-OS-KERNEL-CRATES.md` sections 6 and 7 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`).

use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ErasedSnapshotRetirement, ValueError};

#[path = "📥️lifted/🏪️store/🦀️.rs"]
mod lifted_store;

/// 🧯️ Admits the owned roots of one store to the installed close cursor one phase at a time.
pub trait StoreCloseAdmission<P, Mutation> {
    /// 🧲️ Takes the attached backbone and its inbox under the caller's retirement grant.
    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> %(ret)s;
    /// 🚪️ Moves one owned root of the addressed phase into its retirement owner.
    fn close_cursor_admit(&mut self, phase: semio_framework_os_store_read::ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> %(ret)s;
}

/// 🪹️ Reads the terminal-empty witnesses of one store's owned roots.
pub trait StoreCloseWitness {
    /// 🔎️ Whether every snapshot read lease has closed.
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool;
    /// 🫙️ Whether the structural shells hold no payload owner.
    fn close_structural_owners_terminal_is_empty(&self) -> bool;
    /// 🪦️ Whether every owned root has been detached.
    fn owned_roots_terminal_is_empty(&self) -> bool;
}
""" % dict(ret=ret)
files={dst_dir+"/🦀️.rs":root,dst_dir+"/📥️lifted/🏪️store/🦀️.rs":lifted}
newsrc=s[:a]+s[d:]
if not apply:
    for k,v in files.items(): print(k,len(v.encode()))
    print("cursor",len(s.encode()),"->",len(newsrc.encode()))
else:
    for k,v in files.items():
        os.makedirs(os.path.dirname(k),exist_ok=True); open(k,"w",encoding='utf8').write(v)
    open(src,"w",encoding='utf8').write(newsrc)
