import os, pathlib, sys

toml = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml")
lock = pathlib.Path("/Users/ueli/Documents/semio/Cargo.lock")
t = toml.read_text()
l = lock.read_text()
anchor = 'semio-framework-os-kernel = { path = "../../../../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel" }\n'
assert t.count(anchor) == 1
t = t.replace(anchor, anchor + '# 👕️ The replication contract\'s pure peer-overlay derivation (`peers_for_window`, `canvas_point_to_screen`, …) the wgpu\n# canvas-presence twin (`🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu`) paints peers with. Workspace-internal.\nreplication = { path = "../../../../../../../../../🔨️modules/📡️replication/📦️packages/🦀️rust", package = "semio-framework-replication" }\n', 1)
entry = 'name = "semio-framework-os-renderer-wgpu"\nversion = "0.1.0"\ndependencies = [\n'
start = l.index(entry)
end = l.index("]\n", start)
block = l[start:end]
old = ' "semio-framework-plugin-host",\n'
assert block.count(old) == 1
block = block.replace(old, old + ' "semio-framework-replication",\n')
l = l[:start] + block + l[end:]
toml.with_name("Cargo.toml.wg8").write_text(t)
lock.with_name("Cargo.lock.wg8").write_text(l)
os.replace(toml.with_name("Cargo.toml.wg8"), toml)
os.replace(lock.with_name("Cargo.lock.wg8"), lock)
print("swapped")
