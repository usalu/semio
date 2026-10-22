import sys,os,re,json
sys.path.insert(0,os.path.dirname(__file__))
from lib import *
OS=F+"/🛍️products/💻️os"
S=OS+"/🔨️modules/🏪️store"
EPH=S+"/🫧️ephemeral";PRE=S+"/👥️presence"
PKG=PRE+"/📦️packages/🦀️rust"
XPKG=EPH+"/📦️packages/🦀️rust"
OLD=EPH+"/👥️presence-store-and-peers-publication/🦀️.rs"
NEWSTORE=PRE+"/👥️presence-store-and-peers-publication/🦀️.rs"
# ---- 1. manifest of the presence crate from the ephemeral manifest
xm=get(XPKG+"/Cargo.toml")
m=xm.replace('name = "semio-framework-os-store-ephemeral"','name = "semio-framework-os-store-presence"').replace('id = "os-store-ephemeral"','id = "os-store-presence"')
m=m.replace('[package.metadata.semio]','description = "Presence lane: the bounded peer table, the presence store, peers publication, retirement, read and rejection owners"\n\n[package.metadata.semio]',1)
m=m.replace('testkit = []','testkit = ["semio-framework-os-store-ephemeral/testkit"]')
m=m.replace('[dependencies]\n','[dependencies]\nsemio-framework-os-store-ephemeral = { path = "../../../🫧️ephemeral/📦️packages/🦀️rust" }\n',1)
put(PKG+"/Cargo.toml",m)
put(PKG+"/📋️project.json",project_json("os-store-presence"))
# ephemeral manifest: presence snapshot moves to the presence crate
sub(XPKG+"/Cargo.toml",'semio-framework-os-presence-native-snapshot = { path="../../../👥️presence/📸️snapshot/📦️packages/🦀️rust" }\n','')
# ---- 2. move the store file
s=get(OLD)
lit=re.search(r'        Ok\(ArtifactEphemeralOneItemPublication \{\n.*?\n        \}\)\n',s,re.S)
assert lit
s=s.replace(lit.group(0),'        Ok(ArtifactEphemeralOneItemPublication::preparing(operation, expected_generation, footprint, preparation, root_retirement_factory))\n')
put(NEWSTORE,s)
REMOVE.append(OLD)
# ---- 3. presence root
p=PRE+"/🦀️.rs"
r=get(p)
head_old='''use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value::ValueError;
use std::sync::Arc;
use semio_framework_value_snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
'''
assert head_old in r
head_new='''use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, ValueError, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value_snapshot_read::*;
use semio_framework_io_control::NativeSnapshotDecodeOwner;
use semio_framework_os_store_read::ArtifactStoreDisplacedSnapshotRetirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_store_one_item::*;
use protocol::{Mutation, MutationDiff, apply_operation};
use semio_framework_value::{FromValue, ToValue};
use std::{collections::BTreeMap, marker::PhantomData, sync::Arc, mem::{size_of, size_of_val}};
use semio_framework_os_store_ephemeral::{ArtifactEphemeralBaseOwner, ArtifactEphemeralBaseRead, ArtifactEphemeralOneItemAdmissionRejected, ArtifactEphemeralOneItemPreparation, ArtifactEphemeralOneItemPreparationFactory, ArtifactEphemeralOneItemPreparationRequest, ArtifactEphemeralOneItemPrepared, ArtifactEphemeralOneItemPublication};
#[cfg(test)]
use semio_framework_value_snapshot_read::presence_test_retirement;
'''
r=r.replace(head_old,head_new)
r=r.replace("//! presence read, publication io and rejection leaves read it without naming the engine.","//! presence read, publication io and rejection leaves read it without naming the engine. This crate owns the whole\n//! presence lane: the store, the peers publication and commit, the retirement owners and the rejection owner.")
r=r.rstrip("\n")+'''

//#region 🔖️Authority
#[path = "👥️presence-store-and-peers-publication/🦀️.rs"]
mod presence_store_and_peers_publication;
pub use presence_store_and_peers_publication::{PresencePeersCommit, PresencePeersPublication, PresenceStore};

#[path = "🚪️io/📬️publication/🦀️.rs"]
mod presence_publication_io;

#[path = "🔗️read/🦀️.rs"]
mod presence_peer_read;
pub use presence_peer_read::PresencePeersRead;

#[path = "🚫️rejection/🦀️.rs"]
mod presence_peer_rejection;
pub use presence_peer_rejection::PresencePeerAdmissionRejected;

pub use semio_framework_os_presence_native_snapshot::ArtifactPresenceSnapshot;

#[path = "♻️retirement/🦀️.rs"]
mod presence_retirement;
pub use presence_retirement::PresenceStoreRetirement;
//#endregion 🔖️Authority

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
'''
texts[p]=r
sub(PRE+"/♻️retirement/🧪️tests/🔬️unit/🦀️.rs","crate::ephemeral_transfer::ArtifactEphemeralTransferPreparationFactory","semio_framework_os_store_ephemeral::ephemeral_transfer::ArtifactEphemeralTransferPreparationFactory")
# ---- 4. ephemeral root
x=EPH+"/🦀️.rs"
sub(x,'#[path = "../👥️presence/🦀️.rs"]\npub mod presence;\nuse presence::{LaneItemReceipt, PRESENCE_PEER_ID_BYTES, PRESENCE_PEER_SLOTS, PresencePeersCapacityError, PresencePeersRetiredEntries, PresencePeersRetirement, PresencePeersRoot};\nuse presence::{PresencePeerEntry, PresencePeerRootCustody, presence_arc_bytes};\n\n','//#region 🔖️EphemeralLanes\n')
sub(x,'#[path = "👥️presence-store-and-peers-publication/🦀️.rs"]\nmod presence_store_and_peers_publication;\npub use presence_store_and_peers_publication::{\n    PresencePeersCommit, PresencePeersPublication, PresenceStore,\n};\n\n','')
i=get(x).index('#[path = "../👥️presence/🚪️io/📬️publication/🦀️.rs"]')
j=get(x).index('//#endregion 🔖️EphemeralLanes')
xs=get(x)
texts[x]=xs[:i]+'//#endregion 🔖️EphemeralLanes\n\n'+xs[xs.index('#[cfg(test)]\n#[global_allocator]'):]
sub(x,"//! 🫧️ Exact presence, transient and interaction publication owners.","//! 🫧️ Exact transient and interaction publication owners and the ephemeral one-item publication they share.")
# ---- 5. publication: constructor + public state-machine fields
pu=EPH+"/📤️ephemeral-one-item-publication/🦀️.rs"
for f in ["expected_generation","footprint","preparation","displaced_root_retirement","root_retirement_factory","receipt","published","cancel_requested","fault","phase"]:
    subre(pu,r'    pub\(super\) '+f+r':',"    pub "+f+":")
sub(pu,"    pub(super) fn advance_preparation(","    pub fn advance_preparation(")
sub(pu,"    pub fn operation(&self) -> semio_framework_job::OperationId {",'''    /// 🧱️ Opens a publication in its first phase around one prepared owner; the lane stores drive the public state fields.
    pub fn preparing(operation: semio_framework_job::OperationId, expected_generation: u64, footprint: ArtifactStoreOneItemFootprint, preparation: Box<dyn ArtifactEphemeralOneItemPreparation<P, Mutation>>, root_retirement_factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        use std::mem::ManuallyDrop as Held;
        Self {
            operation, expected_generation, footprint,
            preparation: Held::new(Some(preparation)),
            displaced_root_retirement: Held::new(None), root_retirement_factory: Held::new(Some(root_retirement_factory)),
            returned_read_registry: Held::new(None), returned_read_retirement: Held::new(None),
            owned_retirement_factory: Held::new(None), receipt: Held::new(None),
            retained_checkpoint: Default::default(), factory_retirement: [None, None],
            attempts: 0, published: false, cancel_requested: false, close_started: false,
            fault: Held::new(None), phase: ArtifactStoreOneItemPublicationPhase::Preparing,
        }
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {''')
# ---- 6. facade, prelude, kernel crate
sr=S+"/🦀️.rs"
sub(sr,"pub use semio_framework_os_store_ephemeral::*;\n","pub use semio_framework_os_store_ephemeral::*;\npub use semio_framework_os_store_presence::*;\npub use semio_framework_os_store_presence as presence;\n")
kc=OS+"/📦️packages/🦀️rust/Cargo.toml"
sub(kc,'semio-framework-os-store-ephemeral = { path = "../../🔨️modules/🏪️store/🫧️ephemeral/📦️packages/🦀️rust", features = ["testkit"] }\n','semio-framework-os-store-ephemeral = { path = "../../🔨️modules/🏪️store/🫧️ephemeral/📦️packages/🦀️rust", features = ["testkit"] }\nsemio-framework-os-store-presence = { path = "../../🔨️modules/🏪️store/👥️presence/📦️packages/🦀️rust", features = ["testkit"] }\n')
pr=S+"/🧪️testing/🧭️prelude"
sub(pr+"/🦀️.rs","ArtifactEphemeralOneItemPublication, InteractionStore, PresenceStore, TransientStore};","ArtifactEphemeralOneItemPublication, InteractionStore, TransientStore};\npub use semio_framework_os_store_presence::PresenceStore;")
add_dep_like(pr+"/📦️packages/🦀️rust/Cargo.toml","semio-framework-os-store-ephemeral","semio-framework-os-store-presence",PKG)
s=get(pr+"/📦️packages/🦀️rust/Cargo.toml")
texts[pr+"/📦️packages/🦀️rust/Cargo.toml"]=s.replace('semio-framework-os-store-presence = { path = "../../../../👥️presence/📦️packages/🦀️rust" }','semio-framework-os-store-presence = { path = "../../../../👥️presence/📦️packages/🦀️rust", features = ["testkit"] }')
# ---- 7. registries
insert_sorted_member(R+"/Cargo.toml",os.path.relpath(PKG,R))
add_build_owner("os-store-presence",os.path.relpath(PKG,R))
CB=OS+"/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
anchor='''    {
      "package": "semio-framework-os-store-one-item",
      "manifest": "🔨️modules/🏪️store/🎮️one-item/📦️packages/🦀️rust/Cargo.toml",'''
sub(CB,anchor,'''    {
      "package": "semio-framework-os-store-presence",
      "manifest": "🔨️modules/🏪️store/👥️presence/📦️packages/🦀️rust/Cargo.toml",
      "entry": "🔨️modules/🏪️store/👥️presence/🦀️.rs",
      "maximumActiveLines": 4000,
      "forbiddenDependencies": [
        "semio-framework-os-kernel",
        "semio-framework-plugin",
        "semio-framework"
      ]
    },
'''+anchor)
CU=OS+"/🧪️tests/🧫️fixtures/📦️crate-boundaries/🧬️cuts.json"
sub(CU,'''      "canonical": "pub use semio_framework_os_store_ephemeral::*;",
      "definition": "pub struct PresenceStore<"
    },''','''      "canonical": "pub use semio_framework_os_store_ephemeral::*;",
      "definition": "pub struct TransientStore<"
    },
    {
      "producer": "semio-framework-os-store-presence",
      "consumer": "🔨️modules/🏪️store/🦀️.rs",
      "canonical": "pub use semio_framework_os_store_presence::*;",
      "definition": "pub struct PresenceStore<"
    },''')
sub(CU,'''      "producer": "semio-framework-os-presence-native-snapshot",
      "consumer": "🔨️modules/🏪️store/🫧️ephemeral/🦀️.rs",''','''      "producer": "semio-framework-os-presence-native-snapshot",
      "consumer": "🔨️modules/🏪️store/👥️presence/🦀️.rs",''')
LIB=F+"/🛍️products/🦑️repo/🔨️modules/📚️library"
sub(LIB+"/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts",'"🫧️ephemeral/👥️presence-store-and-peers-publication/🦀️.rs"','"👥️presence/👥️presence-store-and-peers-publication/🦀️.rs"')
sub(LIB+"/🧪️tests/🔬️cad-presence-retirement/🟦️.ts",'join(storeFolder, "🫧️ephemeral/👥️presence-store-and-peers-publication/🦀️.rs")','join(storeBase, "👥️presence-store-and-peers-publication/🦀️.rs")')
flush()
