"""🧪 Links the paid paged placement law against the original Pack read and write implementations."""

import json
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/raw-png"
CACHE = Path(sys.argv[1]).resolve()
wasm = "--wasm" in sys.argv
metadata = "--metadata" in sys.argv
host = CACHE.parents[2] / "prototype/build" if wasm else CACHE
OUT.mkdir(parents=True, exist_ok=True)
[pack] = CACHE.glob("*/*/out/libpack-*.rlib")
deps = dict(json.loads(pack.with_suffix(".rmeta.semio-deps").read_bytes())["dependencies"])
owners = ["pack", "semio_framework_value", "semio_framework_value_persistent",
          "semio_framework_hash", "semio_framework_pack_format", "semio_framework_artifact_reference", "semio_framework_pack_source"]
binary = OUT / ("placement.exe" if sys.platform == "win32" else "placement")
source = "raw-png-placement-native.rs"
mode = ["--test"]
if wasm:
    source = "raw-png-placement-typed.rs"
    binary = OUT / "placement-typed.wasm"
    mode = ["--target=wasm32-wasip2"]
    if metadata:
        binary = OUT / "placement-typed.rmeta"
        mode.append("--emit=metadata")
    owners.extend(("semio_plugin", "semio_os_spr", "semio_os_vcs",
        "semio_s_artifact_stdio_pdf_editor_document", "semio_s_artifact_stdio_pdf_mut_family_8"))
args = ["--edition=2021", *mode, "--crate-name=raw_png_placement", "-Copt-level=1",
        "-Awarnings", str(TICKET / source), "-o", str(binary)]
for name in owners:
    candidates = list(CACHE.glob(f"*/*/out/lib{name}-*.rlib"))
    if len(candidates) != 1:
        candidates = [file for file in candidates if file.name[3:-5] in deps]
    [path] = candidates
    args.extend(("--extern", f"{name}={path.with_suffix('.rmeta')}"))
    if not metadata:
        args.extend(("--extern", f"{name}={path}"))
dependencies = {path.parent for root in {CACHE, host} for path in root.glob("*/*/out/*") if path.suffix in (".rlib", ".rmeta", ".dll", ".so", ".dylib")}
metadata_path = OUT / "type/deps"
if metadata:
    assert metadata_path.is_dir(), "run raw-png-placement-typecheck.py first"
    args.append(f"-Ldependency={metadata_path}")
else:
    if wasm and metadata_path.is_dir():
        args.append(f"-Ldependency={metadata_path}")
    args.extend(f"-Ldependency={path}" for path in sorted(dependencies))
rsp = OUT / f"{binary.name}.rsp"
rsp.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{rsp}"], check=True)
if metadata:
    print("[DEBUG] typed placement/history law passed WASI metadata type-check")
elif wasm:
    subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(binary)], check=True)
else:
    subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(binary), "--nocapture"], check=True)
