import sys, re, os
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
from ed import *
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools/w1")
from ed import *
import needs

AA = R + "/🗂️asset-authority/🦀️.rs"
GLB = R + "/🦴️glb-decode/🦀️.rs"
JOBS = R + "/🔁️asset-decode-jobs/🦀️.rs"
ICON = EL + "/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/📥️asset/🦀️.rs"

def imports(files):
    s = needs.render(*needs.wanted(files))
    return s.replace("use crate::scenes::AdmittedSurfaceToken;", "use semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceToken;")

auth = imports([AA])
write(T + "/🗂️asset-authority/🦀️.rs", f'''//! 🗂️ Renderer asset I/O authority and exact-request asset delivery.
//!
//! 🧭️ Native and wasm32: the process-wide asset I/O statics with their request, seal and close functions, the fetch owner, the page and format structure cursors, the response prefix validators, and the request-owned icon export asset slots (`icon_export_asset`). The module files keep their place in the renderer tree and are mounted here.

{auth}

#[path = "../🧊️renderer/🗂️asset-authority/🦀️.rs"]
mod asset_authority;
pub use asset_authority::*;

#[path = "../../../🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/📥️asset/🦀️.rs"]
pub mod icon_export_asset;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
''')
dec = imports([GLB, JOBS])
write(T + "/🦴️asset-decode/🦀️.rs", f'''//! 🦴️ GLB decoder and renderer asset decode jobs.
//!
//! 🧭️ Native and wasm32: the GLB decoder with its mesh materialisation, and the asset probe, decode job and decode session that run on the renderer worker pool. The module files keep their place in the renderer tree and are mounted here.

{dec}

#[path = "../🧊️renderer/🦴️glb-decode/🦀️.rs"]
mod glb_decode;
pub use glb_decode::*;

#[path = "../🧊️renderer/🔁️asset-decode-jobs/🦀️.rs"]
mod asset_decode_jobs;
pub use asset_decode_jobs::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
''')
H = R + "/📥️mounts/🌱️wp1-foundation/🦀️.rs"
sub1(H, '''#[path = "../../🗂️asset-authority/🦀️.rs"]
mod asset_authority;
pub(crate) use asset_authority::*;

#[path = "../../🦴️glb-decode/🦀️.rs"]
mod glb_decode;
pub(crate) use glb_decode::*;

#[path = "../../🔁️asset-decode-jobs/🦀️.rs"]
mod asset_decode_jobs;
pub(crate) use asset_decode_jobs::*;
''', '''extern crate semio_framework_os_renderer_wgpu_asset_authority as asset_authority;
pub(crate) use asset_authority::*;

pub(crate) use semio_framework_os_renderer_wgpu_asset_decode::*;
''')
