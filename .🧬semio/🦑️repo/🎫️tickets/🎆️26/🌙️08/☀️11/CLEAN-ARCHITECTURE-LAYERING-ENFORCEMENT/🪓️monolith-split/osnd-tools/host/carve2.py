"""✂️ OS host carve step 2: the three registrar tests move from workflow-unit to the raster crate; workflow handler registry widened to pub."""
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/"
def rd(p):return open(p,encoding="utf-8").read()
def wr(p,t):open(p,"w",encoding="utf-8").write(t)
WU=H+"🧪️tests/🔬️workflow-unit/🦀️.rs"
t=rd(WU)
a=t.index("    #[cfg(not(all(target_arch = \"wasm32\", target_env = \"p2\")))]\n    #[test]\n    fn svg_path_extraction_preserves_transformed_geometry() {")
b=t.index("    // 🚪️ `solid_exporter_and_importer_registrars_round_trip_a_box_through_step` DELETED")
moved=t[a:b]
assert moved.count("#[test]")==3,moved.count("#[test]")
wr(WU,t[:a]+t[b:])
moved=moved.replace("crate::media_export_raster::","")
moved=moved.replace("crate::host::resolve_kernel_future(","resolve_kernel_future(")
moved=moved.rstrip()+"\n"
assert "crate::" not in moved
RU=H+"🧪️tests/🔬️media-export-raster-unit/🦀️.rs"
r=rd(RU)
assert r.startswith("mod native_raster_tests {")
r="#[cfg(not(target_arch = \"wasm32\"))]\n"+r.rstrip("\n")+"\n\nmod registrar_tests {\n    use super::*;\n    use semio_framework_os_host_workflow::{export_handlers, import_handlers, os_media_handler_key};\n\n"+moved+"}\n"
wr(RU,r)
RAS=H+"🖼️media-export-raster/🦀️.rs"
old='#[cfg(all(test, not(target_arch = "wasm32")))]\ninclude!("../🧪️tests/🔬️media-export-raster-unit/🦀️.rs");'
assert rd(RAS).count(old)==1
wr(RAS,rd(RAS).replace(old,'#[cfg(test)]\ninclude!("../🧪️tests/🔬️media-export-raster-unit/🦀️.rs");'))
WF=H+"🔀️workflow/🦀️.rs"
t=rd(WF)
for old,new in [("type OsMediaExportHandler =","pub type OsMediaExportHandler ="),("type OsMediaImportHandler =","pub type OsMediaImportHandler ="),("\nfn export_handlers() ->","\n/// 🗃️ The process-wide export handler registry, shared with the registrars of the media raster producer.\npub fn export_handlers() ->"),("\nfn import_handlers() ->","\n/// 🗃️ The process-wide import handler registry, shared with the registrars of the media raster producer.\npub fn import_handlers() ->"),("\nfn os_media_handler_key(","\npub fn os_media_handler_key(")]:
    assert t.count(old)==1,old
    t=t.replace(old,new)
wr(WF,t)
print("ok")
