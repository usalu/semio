"""🧩 Type-check the complete mounted owner with the current receipt contract."""

import json
import os
from pathlib import Path
import subprocess
import sys
from urllib.parse import unquote

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
out = ticket / "🗑️generated" / "mounted-rmeta"
out.mkdir(exist_ok=True)
run, lane_id = sys.argv[1:3]
lane = root / ".cache/b/r/m/u/l" / lane_id
logs = root / ".cache/b/r/m/components/runs" / run / "l"
units = json.loads((logs / "unit-graph-prototype-00.json").read_text())["units"]
artifacts = {}


def key(package, name, features, platform):
    return package, name, tuple(features), platform


for log in sorted(logs.glob("cargo-prototype-00*.jsonl"), key=lambda p: ("retry" in p.name, p.name)):
    with log.open() as stream:
        for line in stream:
            try:
                row = json.loads(line)
            except ValueError:
                continue
            if row.get("reason") != "compiler-artifact":
                continue
            for raw in row["filenames"]:
                path = Path(raw)
                if path.suffix not in {".rmeta", ".dylib", ".so", ".dll"} or not path.is_file():
                    continue
                platform = "wasm32-wasip2" if "wasm32-wasip2" in path.parts else None
                artifacts[key(row["package_id"], row["target"]["name"], row["features"], platform)] = path

link = "--link" in sys.argv[3:]
names = [name for name in sys.argv[3:] if name != "--link"] or ["semio_plugin_mounted_owner", "semio_plugin_ownership"]
for name in names:
    unit = next(row for row in units if row["target"]["name"] == name
                and row["mode"] == "build" and row["platform"] == "wasm32-wasip2")
    target = unit["target"]
    package, version = unit["pkg_id"].rsplit("#", 1)[1].split("@", 1)
    manifest = unquote(unit["pkg_id"].split("file://", 1)[1].split("#", 1)[0])
    result = out / f"lib{name}.rmeta"
    args = ["--edition=" + target["edition"], "--target=wasm32-wasip2", "--crate-name", name,
            "--crate-type=rlib", "-Cmetadata=mounted_receipt_check", "-Copt-level=1", "-Cpanic=abort",
            target["src_path"], f"-Ldependency={out}"]
    emissions = f"metadata={result}"
    if link:
        emissions += f",link={result.with_suffix('.rlib')}"
    args.append(f"--emit={emissions}")
    args.extend(f"-Ldependency={folder}" for folder in lane.glob("**/out") if "/build/" in str(folder))
    for edge in unit["dependencies"]:
        dep = units[edge["index"]]
        if dep["mode"] != "build":
            continue
        path = artifacts[key(dep["pkg_id"], dep["target"]["name"], dep["features"], dep["platform"])]
        args.extend(("--extern", edge["extern_crate_name"] + "=" + str(path)))
    for feature in unit["features"]:
        args.extend(("--cfg", f"feature={json.dumps(feature)}"))
    flags = out / f"{name}.args"
    flags.write_text("\n".join(args) + "\n")
    env = dict(os.environ, CARGO_MANIFEST_DIR=manifest, CARGO_PKG_NAME=package, CARGO_PKG_VERSION=version)
    print(f"[DEBUG] Type-checking full {name}", flush=True)
    subprocess.run([os.environ["RUSTC"], "@" + str(flags)], check=True, env=env)
    artifacts[key(unit["pkg_id"], name, unit["features"], unit["platform"])] = result
