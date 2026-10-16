"""✂️ OS host carve step 4: parent facade mounts, child root aliases and the package root (run once)."""
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/"
def rd(p):return open(p,encoding="utf-8").read()
def wr(p,t):open(p,"w",encoding="utf-8").write(t)
def sub(p,old,new,count=1):
    t=rd(p);assert t.count(old)==count,(p[-60:],old[:70],t.count(old));wr(p,t.replace(old,new))
ROOT=H+"🦀️.rs"
for mod,folder,krate in [("instance","🧱️instance","semio_framework_os_host_instance"),("media_export_raster","🖼️media-export-raster","semio_framework_os_host_media_raster"),("workflow","🔀️workflow","semio_framework_os_host_workflow"),("codec_abi","📡️codec-abi","semio_framework_os_host_codec_abi"),("registry","🗂️registry","semio_framework_os_host_registry")]:
    sub(ROOT,f'#[path = "{folder}/🦀️.rs"]\npub mod {mod};\n',f"pub use {krate} as {mod};\n")
sub(H+"📦️packages/🦀️rust/🦀️.rs","use semio_framework_artifact_workflow_workflow as workflow_kernel;\n\n","")
sub(H+"🔀️workflow/🦀️.rs","//! 🎬️ Workflow, VFS snapshot types, and media export registry.\n// #region workflow\n","//! 🎬️ Workflow, VFS snapshot types, and media export registry.\nextern crate semio_framework_value_derive as value_derive;\n#[cfg(test)]\nextern crate semio_framework_os_kernel as store;\n// #region workflow\n")
print("ok")
