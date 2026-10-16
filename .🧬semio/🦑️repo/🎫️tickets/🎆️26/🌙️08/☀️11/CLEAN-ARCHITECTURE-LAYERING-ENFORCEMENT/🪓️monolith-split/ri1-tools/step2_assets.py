import sys, re, os
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
sys.path.insert(1, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools/w1")
from ed import *
import needs

AA = R + "/🗂️asset-authority/🦀️.rs"
GLB = R + "/🦴️glb-decode/🦀️.rs"
JOBS = R + "/🔁️asset-decode-jobs/🦀️.rs"
ICON = EL + "/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/📥️asset/🦀️.rs"
EXPORT = EL + "/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/🦀️.rs"

validators = cut_between(JOBS, "pub(crate) fn renderer_asset_svg_prefix_is_valid(prefix: &[u8]) -> bool {", "//#endregion 📡️RendererAssetAuthority")
assert "renderer_asset_reference_prefix_is_valid" in validators and validators.count("fn ") == 3
t = read(AA)
end = "//#endregion 📡️RendererAssetAuthority"
assert t.count(end) == 1
t = t.replace(end, "\n" + validators.rstrip("\n") + "\n" + end)
assert t.count("scenes::icon_export::asset::close_all_step()") == 1
t = t.replace("scenes::icon_export::asset::close_all_step()", "crate::icon_export_asset::close_all_step()")
write(AA, t)

for f in (AA, GLB, JOBS, ICON):
    t = read(f)
    t = t.replace("pub(crate) ", "pub ").replace("pub(super) ", "pub ")
    write(f, t)

sub1(EXPORT, '#[path = "📥️asset/🦀️.rs"]\npub mod asset;', 'pub use semio_framework_os_renderer_wgpu_asset_authority::icon_export_asset as asset;')
