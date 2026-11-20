"""🪛 Type-check current archive source against a finished compiler dependency cohort."""

import json
import os
from pathlib import Path
import subprocess
import sys
from urllib.parse import unquote

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
generated = ticket / "🗑️generated"
report = workspace / ".cache/b/r/m/components/runs" / sys.argv[1] / "l"
lane = workspace / ".cache/b/r/m/u/l" / sys.argv[2]
units = json.loads((report / "unit-graph-prototype-00.json").read_text())["units"]
artifacts = {}
for row in (report / "cargo-prototype-00.jsonl").read_text().splitlines():
    try:
        record = json.loads(row)
    except ValueError:
        continue
    if record.get("reason") != "compiler-artifact":
        continue
    for file in record["filenames"]:
        path = Path(file)
        if path.suffix not in (".rmeta", ".dylib", ".so", ".dll") or not path.is_file():
            continue
        platform = "wasm32-wasip2" if "wasm32-wasip2" in path.parts else None
        key = (record["package_id"], record["target"]["name"], tuple(record["features"]), platform)
        artifacts[key] = path

for name in sys.argv[3:]:
    unit = next(u for u in units if u["target"]["name"] == name and u["mode"] == "build" and u["platform"] == "wasm32-wasip2")
    target = unit["target"]
    manifest = unquote(unit["pkg_id"].split("file://", 1)[1].split("#", 1)[0])
    args = ["--edition=" + target["edition"], "--target=wasm32-wasip2", "--crate-name", name,
            "--crate-type=lib", "--emit=metadata", "-o", str(generated / f"{name}-live.rmeta"), target["src_path"]]
    args.extend("-Ldependency=" + str(path) for path in lane.glob("**/out") if "/build/" in str(path))
    for member in unit["dependencies"]:
        dependency = units[member["index"]]
        if dependency["mode"] != "build":
            continue
        key = (dependency["pkg_id"], dependency["target"]["name"], tuple(dependency["features"]), dependency["platform"])
        args.extend(("--extern", member["extern_crate_name"] + "=" + str(artifacts[key])))
    for feature in unit["features"]:
        args.extend(("--cfg", f'feature="{feature}"'))
    flags = generated / f"{name}-live.args"
    flags.write_text("\n".join(args) + "\n")
    package, version = unit["pkg_id"].rsplit("#", 1)[1].split("@", 1)
    env = dict(os.environ, CARGO_MANIFEST_DIR=manifest, CARGO_PKG_NAME=package, CARGO_PKG_VERSION=version)
    print(f"[DEBUG] Type-checking {name}", flush=True)
    subprocess.run([os.environ.get("RUSTC", "rustc"), "@" + str(flags)], check=True, env=env)
