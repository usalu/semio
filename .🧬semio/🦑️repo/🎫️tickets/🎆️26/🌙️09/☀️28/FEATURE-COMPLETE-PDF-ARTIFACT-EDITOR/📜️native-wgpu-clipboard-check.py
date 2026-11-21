"""🔎 Type-checks the real native focus crate with the exact dependencies the host build emitted."""

import json
import os
from pathlib import Path
import subprocess
import tomllib

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated/native-wgpu-save"
log = generated / "wgpu-host-governed-publication.log"
manifest = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/👆️ui-engine-focus/📦️packages/🦀️rust/Cargo.toml").resolve()
config = tomllib.loads(manifest.read_text(encoding="utf-8"))
entries = {}
directories = set()
for line in log.read_text(encoding="utf-8", errors="replace").splitlines():
    if not line.startswith('{"reason":"compiler-artifact"'):
        continue
    row = json.loads(line)
    package = row["package_id"].rsplit("#", 1)[-1].split("@")[0]
    paths = [Path(filename) for filename in row["filenames"] if Path(filename).suffix in {".rmeta",".rlib",".so",".dll",".dylib"} and Path(filename).is_file()]
    if paths:
        directories.update(path.parent for path in paths)
        entries[package] = next((path for path in paths if path.suffix == ".rmeta"),paths[0])
args = [
    "--crate-name", config["package"]["name"].replace("-","_"),
    "--edition", config["package"].get("edition","2021"), "--crate-type", "rlib",
    "--emit=metadata", "-o", str(generated / "native-wgpu-clipboard-focus.rmeta"),
    str((manifest.parent / config["lib"]["path"]).resolve()),
]
for directory in sorted(directories):
    args.append(f"-Ldependency={directory}")
dependencies = {**config["dependencies"], **config["target"]['cfg(not(target_arch = "wasm32"))']["dependencies"]}
for name, value in dependencies.items():
    package = value.get("package",name) if isinstance(value,dict) else name
    if package not in entries:
        raise RuntimeError(f"Live compiled dependency is unavailable: {package}")
    args.extend(("--extern", f"{name.replace('-','_')}={entries[package]}"))
response = generated / "native-wgpu-clipboard-focus.args"
response.write_text("\n".join(args)+"\n", encoding="utf-8")
environment = {**os.environ,"CARGO_MANIFEST_DIR":str(manifest.parent),"CARGO_PKG_NAME":config["package"]["name"], "CARGO_PKG_VERSION":"0.1.0"}
subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+str(response)],env=environment,check=True)
print(f"[DEBUG] Actual native focus crate type-checked against {len(dependencies)} compiled dependencies")
