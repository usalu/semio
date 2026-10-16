"""✂️ DAG carve step 3: the parent root becomes the artifact facade (explicit re-exports, store, demo, retained, tests)."""
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osnd/"
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/"
old=open(D+"🦀️.rs",encoding="utf-8").read()
assert old.count("pub use vcs::*;")==1
store=open(SC+"dag-store.txt",encoding="utf-8").read()
default=open(SC+"dag-default.txt",encoding="utf-8").read()
store=store.replace('include_str!("../🧪️tests/🧫️fixtures/🪪️demo-ownership/🔣️.json")','include_str!("🧪️tests/🧫️fixtures/🪪️demo-ownership/🔣️.json")')
assert 'include_str!("🧪️tests/' in store
names=["ActorId","DAG_DOCUMENT_SCHEMA","ChangedNodeAbbreviation","ChangedNodeIcon","ChangedNodeName","ChangedNodeOperatorKind","ChangeNodeAbbreviation","ChangeNodeIcon","ChangeNodeName","ChangeNodeOperatorKind","ConnectNodes","CreateNode","DagDelta","DagDiff","DagEdgePatch","DagMutation","DagNodePatch","DagSnapshot","DeleteNode","DisconnectNodes","MoveNode","MovedNode","RenameNode","RenamedNode","ReorderNodes","ReplaceNodeKind","ReplaceNodeProperties","ReplacedNodeKind","ReplacedNodeProperties","ResizeNode","ResizedNode","RewrittenEdgeEndpoint","dag_document_from_host_snapshot","dag_host_snapshot_from_document","dag_index_to_wire","empty_dag_document"]
use="pub use semio_framework_artifact_infinite_dag_vcs::{\n    "+",\n    ".join(names)+",\n};\n"
new=old.replace("pub use vcs::*;\n",use)
new=new.replace('#[path = "🌿️vcs/🦀️.rs"]\npub mod vcs;\n','')
assert "mod vcs" not in new
tail='''
use crate::os_store::document_materialize::create_document_envelope;
use crate::os_store::{store_schema::ArtifactEnvelope, ArtifactStore};
'''
i=new.index("#[cfg(test)]\n#[path = \"🧪️tests/🕸️dag/🦀️.rs\"]\nmod package_tests;\n")
new=new[:i]+'''// #region 🔖️ArtifactStore
'''+default+"\n"+store.rstrip("\n")+'''
// #endregion 🔖️ArtifactStore

#[cfg(test)]
#[path = "🧪️tests/🕸️dag/🦀️.rs"]
mod package_tests;

#[cfg(test)]
#[path = "🌿️vcs/🧪️tests/🔬️dag-vcs/🦀️.rs"]
mod dag_vcs_tests;

#[cfg(test)]
#[path = "🌿️vcs/🧪️tests/🔬️dag-direct/🦀️.rs"]
mod dag_direct_tests;

#[cfg(test)]
#[path = "🌿️vcs/🧪️tests/🔬️leaf-contracts/🦀️.rs"]
mod dag_leaf_contract_tests;
'''+new[i+len("#[cfg(test)]\n#[path = \"🧪️tests/🕸️dag/🦀️.rs\"]\nmod package_tests;\n"):]
new=new.replace("use semio_framework_os_infinite_directed_presentation::*;\n","use crate::os_store::document_materialize::create_document_envelope;\nuse crate::os_store::{store_schema::ArtifactEnvelope, ArtifactStore};\nuse semio_framework_os_infinite_directed_presentation::*;\n",1)
open(D+"🦀️.rs","w",encoding="utf-8").write(new)
