"""🧪 Validates the live native media transport with exact registered artifacts and shared fixtures."""
import json
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
workspace = Path.cwd()
out = ticket / "🗑️generated/native-wgpu-save/original-live"
out.mkdir(parents=True, exist_ok=True)
engine = workspace / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
media = engine / "🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🎞️original-media/🦀️.rs"
queue = engine / "🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/📥️original-media.rs"
source = (ticket / "📜️native-wgpu-original-law.rs").read_text(encoding="utf-8")
law = out / "law.rs"
law.write_text(source.replace('"media.rs"', json.dumps(str(media), ensure_ascii=False))
    .replace('"queue.rs"', json.dumps(str(queue), ensure_ascii=False)), encoding="utf-8")

log = ticket / "🗑️generated/native-wgpu-save/wgpu-host-clipboard-admission.log"
artifacts = {}
dirs = set()
for line in log.read_text(encoding="utf-8").splitlines():
    if not line.startswith('{"reason":"compiler-artifact"'):
        continue
    item = json.loads(line)
    libraries = [Path(name) for name in item["filenames"] if Path(name).suffix in {".rmeta", ".rlib", ".dylib", ".so", ".dll"} and Path(name).is_file()]
    dirs.update(path.parent for path in libraries)
    if libraries:
        artifacts[item["target"]["name"]] = libraries
binary = out / ("law.exe" if sys.platform == "win32" else "law")
names = ("semio_framework_kernel", "semio_framework_value")
rsp = out / "law.rsp"
args = ["--edition=2024", "--crate-name=native_original_media_law", str(law), "-o", str(binary),
    *(f"--extern={name}={path}" for name in names for path in artifacts[name] if path.suffix in {".rmeta", ".rlib"}),
    *(f"-Ldependency={path}" for path in sorted(dirs))]
rsp.write_text("\n".join(args) + "\n", encoding="utf-8")
rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
subprocess.run([*rustc, f"@{rsp}"], check=True)

fixture = json.loads((ticket / "original-media-react-cases.json").read_text(encoding="utf-8"))
png = ticket / "🗑️generated/paged-media-receiver/raw-picker.png"
loan = fixture["request"]["loan"]
for case in fixture["cases"]:
    command = ["repo", "capacity", "run", "--memory=256", "--", str(binary), str(png), case["trigger"],
        *map(str, (loan[key] for key in ("operationId", "generation", "instanceId", "documentId", "baseRevision"))),
        fixture["seq"], str(fixture["request"]["maximumFileBytes"])]
    result = subprocess.check_output(command, text=True).splitlines()
    expected = [case["expected"], ",".join(case["phase"]), ",".join(map(str, case["reads"])), ",".join(map(str, case["progress"]))]
    assert result[-4:] == expected, (case["name"], result)
    print("[DEBUG] Passed " + case["name"])
