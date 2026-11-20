"""🧊 Rebuilds the viewport Rust leaf against an existing, coherent Cargo cohort."""

import argparse
import json
from pathlib import Path
import subprocess


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("graph", type=Path)
parser.add_argument("lane", type=Path)
parser.add_argument("rustc", type=Path)
args = parser.parse_args()
units = json.loads(args.graph.read_text())["units"]
name = "semio_s_plugin_norm_app_surface_viewport3d"
target = "wasm32-wasip2"
unit = next(unit for unit in units if unit["target"]["name"] == name and unit["platform"] == target and unit["mode"] == "build")
lane = args.lane.resolve()
build = lane / target / "prototype/build"
host = lane / "prototype/build"
out = Path(__file__).resolve().parent / "🗑️generated" / "viewport-paging-libs"
out.mkdir(exist_ok=True)
flags = [unit["target"]["src_path"], f"--crate-name={name}", "--crate-type=lib",
         f"--target={target}", "--edition=2021", "--emit=metadata,link", f"--out-dir={out}",
         "-Copt-level=1", "-Cpanic=abort", "-Cmetadata=norm_viewport_paging"]
flags.extend(f"-Ldependency={path}" for path in sorted({*build.glob("*/*/out"), *host.glob("*/*/out")}))
for need in unit["dependencies"]:
    dependency = units[need["index"]]
    package = dependency["pkg_id"].split("#")[-1].split("@")[0]
    if package[0].isdigit():
        package = dependency["pkg_id"].split("#")[0].rsplit("/", 1)[-1]
    root = build if dependency["platform"] == target else host
    folder = root / package
    if dependency["target"]["kind"] == ["proc-macro"]:
        artifacts = sorted(folder.glob("*/out/*.dylib"))
    else:
        library = max(folder.glob("*/out/*.rlib"), key=lambda path: path.stat().st_mtime)
        artifacts = [library, library.with_suffix(".rmeta")]
    assert artifacts, folder
    for artifact in artifacts:
        flags.extend(("--extern", f'{need["extern_crate_name"]}={artifact}'))
response = out / "viewport3d.args"
response.write_text("\n".join(flags) + "\n")
subprocess.run([str(args.rustc.resolve()), f"@{response}"], check=True)
