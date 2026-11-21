from pathlib import Path
import json
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
terrain = root / "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any"
descriptor = terrain / "🧬️schema/🧭️terrain-descriptor/🦀️.rs"
scenery = terrain / "🖼️assets/🎬️demo/🌍️scenery.json"
fixture = terrain / "🧬️schema/🧪️tests/🔬️relocated-engine/🔣️overlays.json"
toolchain = re.search(r'channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text())[1]
build = Path(sys.argv[1]).resolve()

source = generated / "terrain-descriptor-probe.rs"
source.write_text("#[path = " + json.dumps(str(descriptor), ensure_ascii=False) + "]\nmod terrain_descriptor;\n" + (ticket / source.name).read_text(encoding="utf-8"), encoding="utf-8")
binary = generated / ("terrain-descriptor-probe.exe" if sys.platform == "win32" else "terrain-descriptor-probe")
args = ["--edition=2024", str(source), "-o", str(binary)]
for path in build.glob("*/*/out"):
    if any(path.glob("lib*.rlib")) or any(path.glob("*.*lib")):
        args.extend(["-L", f"dependency={path}"])
for crate in ("semio-framework-value", "semio-framework-pack-json", "semio-framework-value-derive"):
    name = crate.replace("-", "_")
    suffix = {"darwin": "dylib", "win32": "dll"}.get(sys.platform, "so") if name.endswith("_derive") else "rmeta"
    files = list((build / crate).glob(f"*/out/*{name}-*.{suffix}"))
    if not files:
        sys.exit(f"missing Rust library for {crate}")
    library = max(files, key=lambda path: path.stat().st_mtime_ns)
    args.extend(["--extern", f"{name}={library}"])
    if suffix == "rmeta":
        args.extend(["--extern", f"{name}={library.with_suffix('.rlib')}"])
response = generated / "terrain-descriptor-probe.args"
response.write_text("\n".join(args) + "\n", encoding="utf-8")
command = ["repo", "tools", "exec", "rustup", "--", "run", toolchain, "rustc", f"@{response}"]
if subprocess.run(command, cwd=root).returncode:
    sys.exit("Rust descriptor probe failed to compile")
terrain_json = subprocess.check_output([binary, scenery, fixture], text=True, encoding="utf-8")
output = generated / "terrain-scene.json"
scene = json.loads(output.read_text(encoding="utf-8"))
terrain = json.loads(terrain_json)
assert scene["terrain"] == terrain, "Rust projection differs from the independent scene reconstruction"
scene["terrain"] = terrain
output.write_text(json.dumps(scene, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] GIS Terrain descriptor matches the independent demo and imported-map fixtures")
