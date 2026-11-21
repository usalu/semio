"""🛠️ Compile the touched reserved-job guests against an existing WASI cohort."""

import json
import os
from functools import lru_cache
from pathlib import Path
import subprocess
import sys
from urllib.parse import unquote

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
run = workspace / ".cache/b/r/m/components/runs" / sys.argv[1]
log = run / "l"
output = ticket / "🗑️generated" / ("jr-" + run.name)
output.mkdir(exist_ok=True)
units = json.loads((log / "unit-graph-prototype-00.json").read_text())["units"]
target_platform = "wasm32-wasip2"
changed = {"semio_framework_actor_job_state", "semio_plugin_reactor_context"}
artifacts = {}
build_outputs = {}


def key(package, target, features, platform):
    return package, target, tuple(sorted(features)), platform


for line in (log / "cargo-prototype-00.jsonl").open():
    try:
        event = json.loads(line)
    except ValueError:
        continue
    if event.get("reason") == "build-script-executed":
        build_outputs.setdefault(event["package_id"], []).append(Path(event["out_dir"]))
    if event.get("reason") != "compiler-artifact":
        continue
    for filename in event["filenames"]:
        artifact = Path(filename)
        if artifact.suffix not in {".rlib", ".dylib", ".so", ".dll"} or not artifact.is_file():
            continue
        platform = target_platform if target_platform in artifact.parts else None
        artifacts[key(event["package_id"], event["target"]["name"], event["features"], platform)] = artifact


def dependencies(unit):
    return [edge for edge in unit["dependencies"] if units[edge["index"]]["mode"] == "build"]


@lru_cache(None)
def affected(index):
    unit = units[index]
    return unit["platform"] == target_platform and (
        unit["target"]["name"] in changed or any(affected(edge["index"]) for edge in dependencies(unit))
    )


@lru_cache(None)
def compile_unit(index):
    unit = units[index]
    target = unit["target"]
    name = target["name"]
    identity = key(unit["pkg_id"], name, unit["features"], unit["platform"])
    if not affected(index):
        return artifacts[identity]
    library = target["crate_types"] == ["lib"]
    artifact = output / ("lib" + name + ".rlib" if library else name + ".wasm")
    depends = [(edge["extern_crate_name"], compile_unit(edge["index"])) for edge in dependencies(unit)]
    package, version = unit["pkg_id"].rsplit("#", 1)[1].split("@", 1)
    manifest = unquote(unit["pkg_id"].split("file://", 1)[1].split("#", 1)[0])
    env = dict(os.environ, CARGO_MANIFEST_DIR=manifest, CARGO_PKG_NAME=package, CARGO_PKG_VERSION=version)
    for destination in reversed(build_outputs.get(unit["pkg_id"], [])):
        if target_platform in destination.parts and destination.is_dir():
            env["OUT_DIR"] = str(destination)
            break
    flags = [
        "--edition=" + target["edition"], "--target=" + target_platform, "--crate-name=" + name,
        "--crate-type=" + ("lib" if library else "cdylib"), "-Copt-level=0", "-Cdebuginfo=0", "-Cpanic=abort",
        "-Ccodegen-units=16", "-Cmetadata=reserved-job-" + run.name, target["src_path"],
    ]
    if library:
        flags.extend(("--emit=link,metadata", "--out-dir", str(output)))
    else:
        flags.extend(("-Clink-arg=-zstack-size=8388608", "-Clink-arg=--max-memory=536870912",
                      "-Cstrip=debuginfo", "-o", str(artifact)))
    for feature in unit["features"]:
        flags.extend(("--cfg", f'feature="{feature}"'))
    for alias, path in depends:
        flags.extend(("--extern", f"{alias}={path}"))
        metadata = path.with_suffix(".rmeta")
        if metadata.is_file():
            flags.extend(("--extern", f"{alias}={metadata}"))
    flags.extend("-Ldependency=" + str(path) for path in search_paths)
    response = output / (name + ".args")
    response.write_text("\n".join(flags) + "\n")
    print(f"[DEBUG] Rebuilding {name} in {run.name}", flush=True)
    subprocess.run([compiler, "@" + str(response)], env=env, cwd=workspace, check=True)
    return artifact


selection = sys.argv[2:]
search_paths = [output, *sorted({path.parent for path in artifacts.values()})]
compiler = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
for name in selection:
    index = next(i for i, unit in enumerate(units) if unit["target"]["name"] == name
                 and unit["mode"] == "build" and unit["platform"] == target_platform)
    print(f"[DEBUG] Current artifact {compile_unit(index)}", flush=True)
