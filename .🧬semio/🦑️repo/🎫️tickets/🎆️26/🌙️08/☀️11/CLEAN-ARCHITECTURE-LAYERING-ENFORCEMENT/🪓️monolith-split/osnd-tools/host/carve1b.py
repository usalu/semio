"""✂️ OS host carve step 1: source edits of the five new producers, the parent modules and the moved raster tests (exact anchors, run once)."""
import os,re,sys
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/"
SP="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs"
def rd(p):return open(p,encoding="utf-8").read()
def wr(p,t):open(p,"w",encoding="utf-8").write(t)
def sub(p,old,new,count=1):
    t=rd(p);assert t.count(old)==count,(p[-60:],old[:70],t.count(old));wr(p,t.replace(old,new))
sub(H+"🧪️tests/🔬️registry-unit/🦀️.rs","crate::host::resolve_kernel_future(","resolve_kernel_future(",3)
WF=H+"🔀️workflow/🦀️.rs"
sub(WF,"pub use crate::workflow_kernel::{","pub use semio_framework_artifact_workflow_workflow::{")
sub(WF,"use crate::instance::create_os_id;\nuse crate::registry::{os_app_registration, os_artifact_descriptor, OsArtifactDescriptor};\n","use semio_framework_os_host_instance::create_os_id;\nuse semio_framework_os_host_registry::{os_app_registration, os_artifact_descriptor, os_artifact_dialect, resolve_kernel_future, OsArtifactDescriptor};\n")
t=rd(WF)
n1=t.count("crate::host::resolve_kernel_future(");n2=t.count("crate::registry::os_artifact_dialect(")
t=t.replace("crate::host::resolve_kernel_future(","resolve_kernel_future(").replace("crate::registry::os_artifact_dialect(","os_artifact_dialect(").replace("pub use crate::registry::os_resource_media_capability;","pub use semio_framework_os_host_registry::os_resource_media_capability;")
wr(WF,t);print("workflow resolve sites",n1,"dialect sites",n2)
WU=H+"🧪️tests/🔬️workflow-unit/🦀️.rs"
t=rd(WU)
print("workflow-unit resolve",t.count("crate::host::resolve_kernel_future("),"registry",t.count("crate::registry::"))
t=t.replace("crate::host::resolve_kernel_future(","resolve_kernel_future(").replace("crate::registry::register_artifact_descriptor(","register_artifact_descriptor(").replace("crate::registry::os_artifact_dialect(","os_artifact_dialect(")
wr(WU,t)
WS=H+"🧪️tests/🔬️workflow-standalone/🦀️.rs"
INS=H+"🧱️instance/🦀️.rs"
sub(INS,"use crate::workflow;\n","use semio_framework_artifact_workflow_workflow::WorkflowNode;\n")
sub(INS,"nodes: &[workflow::WorkflowNode],","nodes: &[WorkflowNode],")
RAS=H+"🖼️media-export-raster/🦀️.rs"
sub(RAS,"pub use crate::workflow::{","pub use semio_framework_os_host_workflow::{")
BB=H+"🗃️document-backbone/🦀️.rs"
t=rd(BB);print("backbone resolve",t.count("crate::host::resolve_kernel_future"));wr(BB,t.replace("crate::host::resolve_kernel_future","crate::registry::resolve_kernel_future"))
t=rd(SP);print("space resolve",t.count("crate::host::resolve_kernel_future"));wr(SP,t.replace("crate::host::resolve_kernel_future","crate::registry::resolve_kernel_future").replace("(`🖥️host/🦀️.rs`) resolves them","(`🖥️host/🗂️registry/🦀️.rs`) resolves them"))
CODEC_T=H+"🧪️tests/🔬️codec-abi-unit/🦀️.rs"
sub(CODEC_T,'''        let source = include_str!("../../🦀️.rs");
        let start = source.find("pub mod codec_abi {").unwrap();
        let tests = source[start..].find("//#region 🧪️Tests").unwrap() + start;
        let production = &source[start..tests];
''','''        let source = include_str!("../../📡️codec-abi/🦀️.rs");
        let tests = source.find("//#region 🧪️Tests").unwrap();
        let production = &source[..tests];
''')
print("ok")
