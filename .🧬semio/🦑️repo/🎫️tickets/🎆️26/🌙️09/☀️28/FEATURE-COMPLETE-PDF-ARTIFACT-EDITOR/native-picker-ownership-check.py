"""🪟 Checks native shell picker arbitration against the live Rust transport and a mounted fixture."""
import json
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
out = ticket / "🗑️generated/native-picker-ownership"
out.mkdir(parents=True, exist_ok=True)
engine = Path.cwd() / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
bridge = engine / "🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🎞️original-media/🦀️.rs"
picker = next(engine.glob("🧱️elements/*Shell/*targets/*wgpu/*staging/*doc-open/*theme-file*/🎞️original-picker.rs"))
law = out / "law.rs"
law.write_text((ticket / "native-picker-ownership-law.rs").read_text(encoding="utf-8")
    .replace('"media.rs"', json.dumps(str(bridge), ensure_ascii=False)), encoding="utf-8")
worker = picker.read_text(encoding="utf-8").split("/// 🧾 Projects accepted bytes", 1)[0]
(out / "picker.rs").write_text(worker, encoding="utf-8")

log = ticket / "🗑️generated/native-wgpu-save/wgpu-host-document-picker.log"
names = ("semio_framework_kernel", "semio_framework_value", "tokio")
artifacts = {}
dirs = set()
for line in log.read_text(encoding="utf-8").splitlines():
    if line.startswith('{"reason":"compiler-artifact"'):
        entry = json.loads(line)
        libraries = [Path(name) for name in entry["filenames"] if Path(name).suffix in {".rlib", ".rmeta"}]
        dirs.update(path.parent for name in entry["filenames"] if (path := Path(name)).is_file())
        if entry["target"]["name"] in names and libraries and all(path.is_file() for path in libraries):
            artifacts[entry["target"]["name"]] = libraries

binary = out / ("law.exe" if sys.platform == "win32" else "law")
response = out / "law.rsp"
response.write_text("\n".join(["--edition=2024", "--crate-name=native_picker_ownership",
    str(law), "-o", str(binary), *(f"--extern={name}={path}" for name in names for path in artifacts[name]),
    *(f"-Ldependency={path}" for path in sorted(dirs))]) + "\n",
    encoding="utf-8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"],
    check=True)

fixture = json.loads((ticket / "native-picker-ownership.json").read_text(encoding="utf-8"))
args = [str(binary)]
for mounted in fixture["mounted"]:
    args.extend(str(mounted[key]) for key in ("instance", "controller", "document"))
for issued in fixture["issued"]:
    args.extend(str(issued[key]) for key in ("instance", "operation"))
output = subprocess.check_output(["repo", "capacity", "run", "--memory=256", "--", *args], text=True)
actual = [json.loads(line) for line in output.splitlines()[-4:]]
assert actual == list(fixture["expected"].values()), (actual, fixture["expected"])
print("[DEBUG] Passed native picker ownership, exact busy cancellation, effect delivery and reissue")
