"""✂️ DAG carve step 1: vcs root becomes the child lib root; store/demo/test parts move to the parent root (exact anchors, run once)."""
import os,re,sys
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/"
VCS=D+"🌿️vcs/🦀️.rs"
t=open(VCS,encoding="utf-8").read()
def cut(t,a,b=None,repl=""):
    assert t.count(a)==1,(a[:60],t.count(a))
    return t.replace(a,repl)
t=cut(t,"//! 🧬️ DAG mutation, diff, and document codecs.\nuse crate::dag_demo_host_snapshot;\n",repl="""//! 🧬️ DAG mutation, diff, and document codecs: the persisted snapshot, its diff algebra and every direct mutation leaf with all of their native representations.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub use semio_framework_os_kernel::{os_dsl, os_pack, os_spr, os_store};
""")
t=cut(t,"#[cfg(test)]\nuse ::graph::manifest::PropertyValue;\n#[cfg(test)]\nuse semio_framework_pack_json::Value;\n")
t=cut(t,"#[cfg(test)]\nuse crate::os_spr::Mutation;\n#[cfg(test)]\nuse crate::os_spr::{ArtifactId, Edit, SchemaId};\n")
t=cut(t,"use crate::os_store::document_materialize::create_document_envelope;\n#[cfg(test)]\nuse crate::os_store::store_schema::ArtifactCommand;\nuse crate::os_store::{store_schema::ArtifactEnvelope, ArtifactStore};\n")
a=t.index("/// 🌱️ The demo document seed")
b=t.index("/// 🔁️ Projects a {@link DagHostSnapshot}")
default_dag=t[a:b]
t=t[:a]+t[b:]
a=t.index("pub type DagEnvelope")
b=t.index("//#region 🔖️Dsl")
store_part=t[a:b]
t=t[:a]+t[b:]
t=cut(t,"""#[cfg(test)]
#[path = "🧪️tests/🔬️dag-vcs/🦀️.rs"]
mod dag_vcs_tests;
""")
t=cut(t,"""#[cfg(test)]
#[path = "🧪️tests/🔬️dag-direct/🦀️.rs"]
mod dag_direct_tests;

""")
open(VCS,"w",encoding="utf-8").write(t)
open("/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osnd/dag-default.txt","w",encoding="utf-8").write(default_dag)
open("/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osnd/dag-store.txt","w",encoding="utf-8").write(store_part)
print("ok")
