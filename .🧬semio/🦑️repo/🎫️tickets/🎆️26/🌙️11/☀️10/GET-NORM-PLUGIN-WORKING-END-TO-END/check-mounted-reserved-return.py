"""🧪 Compile the mounted lifecycle receipt probe against the exact Base WASI cohort."""

import json
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
out = ticket / "🗑️generated"
run, lane_name = sys.argv[1:3]
current = sys.argv[3:] == ["current"]
lane = root / ".cache/b/r/m/u/l" / lane_name
reports = root / ".cache/b/r/m/components/runs" / run / "l"
needs = {"semio_framework_job", "semio_framework_value", "semio_plugin_mounted_owner", "semio_plugin_ownership", "serde_json"}
libraries = {}
for report in sorted(reports.glob("cargo-prototype-00*.jsonl"), key=lambda path: ("retry" in path.name, path.name)):
    with report.open() as stream:
        for line in stream:
            try:
                artifact = json.loads(line)
            except ValueError:
                continue
            if artifact.get("reason") != "compiler-artifact" or artifact["target"]["name"] not in needs:
                continue
            for name in artifact["filenames"]:
                path = Path(name)
                if path.suffix == ".rlib" and "wasm32-wasip2" in path.parts and path.is_file():
                    libraries[artifact["target"]["name"]] = path

if set(libraries) != needs:
    raise ValueError(f"Missing cohort outputs: {sorted(needs - set(libraries))}")
if current:
    for name in ("semio_plugin_mounted_owner", "semio_plugin_ownership"):
        libraries[name] = out / "mounted-rmeta" / f"lib{name}.rlib"
component = out / "mounted-reserved-return.wasm"
args = ["--edition=2021", "--target=wasm32-wasip2", "-Copt-level=1", "-Cpanic=abort",
        str(ticket / "check-mounted-reserved-return.rs"), "-o", str(component)]
if current:
    args.extend(("--cfg", "mounted_receipt_current", f"-Ldependency={out / 'mounted-rmeta'}"))
args.extend(f"-Ldependency={path}" for path in lane.glob("**/out") if "/build/" in str(path))
for name, library in libraries.items():
    for path in (library, library.with_suffix(".rmeta")):
        args.extend(("--extern", f"{name}={path}"))
fixture = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧪️tests/🧫️fixtures/reserved-return.json"
flags = out / "mounted-reserved-return.args"
env = dict(os.environ, SEMIO_RECEIPT_FIXTURE=str(fixture))
flags.write_text("\n".join(args) + "\n")
subprocess.run([os.environ["RUSTC"], "@" + str(flags)], cwd=root, env=env, check=True)
subprocess.run(["repo", "tools", "exec", "jco", "--", "run", "--jco-dir", str(out / "mounted-reserved-jco"), str(component)], cwd=root, check=True)
