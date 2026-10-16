import sys, os, re, json
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
from ed import *

HP = T + "/🔧️host-plumbing"
files = {
    "deadlines": T + "/⏰️deadlines/🦀️.rs",
    "native_caller": T + "/🎟️native-caller/🦀️.rs",
    "render_snapshot": T + "/📸️render-snapshot/🦀️.rs",
    "browser_original_output": T + "/🌐️browser-worker/📤️output/🦀️.rs",
    "surface_lane": T + "/📐️surface-lane/🦀️.rs",
    "input_wire": T + "/🎮️input-wire/🦀️.rs",
}
IW = files["input_wire"]
sub1(IW, "remote: crate::shell::ShellHubRemoteV1,", "remote: semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1,")
SL = files["surface_lane"]
sub1(SL, "&crate::renderer_worker_pool()", "&semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool()")
for name, f in files.items():
    t = read(f)
    t = t.replace("pub(crate) ", "pub ").replace("pub(super) ", "pub ")
    write(f, t)

def mount(name, cfg=None):
    rel = os.path.relpath(files[name], HP)
    return (f"#[cfg({cfg})]\n" if cfg else "") + f'#[path = "{rel}"]\n' + (f"#[cfg({cfg})]\n" if False else "") + f"pub mod {name};\n"

root = '''//! 🔧️ Surface lane, input wire, render snapshot, deadlines, native caller and browser output adapters.
//!
//! 🧭️ Native and wasm32: the deadline scheduler, the native original-actor caller, the render snapshot with its output loan, the browser original output adapter, the surface lane job and the browser input wire. The module files keep their place in the renderer tree and are mounted here as public modules.

extern crate semio_framework_os_renderer_wgpu_media_slots as media_slots;

'''
for name in ("deadlines", "native_caller", "render_snapshot", "browser_original_output", "surface_lane", "input_wire"):
    root += mount(name, 'not(target_arch = "wasm32")' if name == "native_caller" else None) + "\n"
root += '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n'
write(HP + "/🦀️.rs", root)

H = R + "/📥️mounts/🧵️wp3-frame-core/🦀️.rs"
t = read(H)
repl = [
 ('#[path = "../../../⏰️deadlines/🦀️.rs"]\nmod deadlines;\n', ''),
 ('#[cfg(not(target_arch="wasm32"))]\n#[path="../../../🎟️native-caller/🦀️.rs"]\nmod native_caller;\n', ''),
 ('#[path = "../../../📸️render-snapshot/🦀️.rs"]\nmod render_snapshot;\n', ''),
 ('#[path="../../../🌐️browser-worker/📤️output/🦀️.rs"]\nmod browser_original_output;\n', ''),
 ('#[path = "../../../📐️surface-lane/🦀️.rs"]\nmod surface_lane;\n', ''),
 ('#[path = "../../../🎮️input-wire/🦀️.rs"]\nmod input_wire;\n', ''),
]
for a, b in repl:
    assert t.count(a) == 1, a
    t = t.replace(a, b)
anchor = '#[path = "../../../🏠️os-host/🦀️.rs"]\nmod os_host;\n'
assert t.count(anchor) == 1
t = t.replace(anchor, 'use semio_framework_os_renderer_wgpu_host_plumbing::{browser_original_output, deadlines, input_wire, render_snapshot, surface_lane};\n#[cfg(not(target_arch = "wasm32"))]\nuse semio_framework_os_renderer_wgpu_host_plumbing::native_caller;\n' + anchor)
write(H, t)

M = HP + "/📦️packages/🦀️rust/Cargo.toml"
mt = read(M)
add = '''semio-framework-job-worker-authority = { path = "../../../../../../../../../../🔨️modules/🧵️job/👷️worker/🎟️authority/📦️packages/🦀️rust" }
'''
mt = mt.replace('semio-framework-pack-json =', add + 'semio-framework-pack-json =', 1)
tail = '''
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
semio-framework-actor = { workspace = true }
semio-framework-async = { workspace = true, features = ["entrypoint"] }
semio-framework-plugin-guest-contract = { workspace = true }

[target.'cfg(all(target_arch = "wasm32", target_os = "unknown"))'.dependencies]
semio-framework-async = { workspace = true }
'''
mt = mt.replace("\n[dev-dependencies]", tail + "\n[dev-dependencies]", 1)
write(M, mt)
r = json.load(open(SPLIT + "/ri1-tools/real.json"))
if "host-plumbing" not in r: r.append("host-plumbing")
json.dump(sorted(r), open(SPLIT + "/ri1-tools/real.json", "w"), indent=1)
