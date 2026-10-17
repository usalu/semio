"""🧰 rw2t step A3: scene-text-vfs gets the testkit feature its moved helpers are gated by; the Scenes companion enables it."""
from ed import *
T_ = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
replace(T_ + "/📁️scene-text-vfs/📦️packages/🦀️rust/Cargo.toml", [
 ('[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n', '[lib]\npath = "../../🦀️.rs"\n\n[features]\ntestkit = ["semio-framework-os-renderer-wgpu-contracts/testkit", "semio-framework-os-renderer-wgpu-scene-state/testkit"]\n\n[dependencies]\n', 1),
])
replace(T_ + "/🧷️engine-attach-tests/📦️packages/🦀️rust/Cargo.toml", [
 ('semio-framework-os-renderer-wgpu-scene-text-vfs = { path = "../../../📁️scene-text-vfs/📦️packages/🦀️rust" }', 'semio-framework-os-renderer-wgpu-scene-text-vfs = { path = "../../../📁️scene-text-vfs/📦️packages/🦀️rust", features = ["testkit"] }', 1),
])
