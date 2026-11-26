"""🌿️ Link the paged gbXML law using complete sized WASI archives and their actual fixture caller."""
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path
from bim_link import large_link

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
root = Path.cwd()
output = generated / "wasi-gbxml-paced"
output.mkdir(exist_ok=True)
capture = json.loads((generated / "compiler-retry/w-bim-79266.success.json").read_text())
compiler = capture["compiler"]
target = "wasm32-wasip2"
captures = [generated / name for name in ("fsb", "bs")]
name = "semio_s_artifact_bim_model_gbxml"
caller = "semio_framework_job_testing_caller"
base = root / "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
source = base / "🚪️io/📤️export/🌿️gbxml"

def metadata(archive):
    text = subprocess.check_output([compiler, "-Zls=root", str(archive)], text=True)
    owners = {
        key: (unit, contents) for key, unit, contents in re.findall(r"^\d+ (\w+)-(\w+) hash (\w+)", text, re.M)
    }
    return re.search(r"^hash (\w+)", text, re.M)[1], owners

def collect(sized):
    units = sorted((sized / target / "debug/build").glob("*/*/out")) + sorted((sized / "debug/build").glob("*/*/out"))
    files = {asset.name: asset for path in units for asset in path.glob("lib*") if asset.is_file()}
    candidates = [file for file in files.values() if file.name.startswith(f"lib{name}-") and file.suffix == ".rmeta" and file.with_suffix(".rlib").is_file()]
    newest_source = max(file.stat().st_mtime for file in source.rglob("*.rs") if "🧪️tests" not in file.parts)
    candidates = [file for file in candidates if min(file.stat().st_mtime, file.with_suffix(".rlib").stat().st_mtime) >= newest_source]
    for seed in sorted(candidates, key=lambda p: p.stat().st_mtime, reverse=True):
        _, dependencies = metadata(seed)
        if caller not in dependencies:
            continue
        exact = files.get(f"lib{caller}-{dependencies[caller][0]}.rmeta")
        if not exact or not exact.with_suffix(".rlib").is_file():
            continue
        caller_hash, caller_deps = metadata(exact)
        if caller_hash != dependencies[caller][1] or caller_deps["semio_framework_job_context"] != dependencies["semio_framework_job_context"]:
            continue
        return units, files, seed, dependencies
    return None

def first_current():
    return next((result for cache in captures if (result := collect(cache)) is not None), None)

data = first_current()
if "--watch" in sys.argv:
    print("[DEBUG] Awaiting the current sized gbXML archive and matching original job caller", flush=True)
    while data is None:
        time.sleep(5)
        data = first_current()
if data is None:
    raise SystemExit("The current sized gbXML archive and matching original job caller are unavailable")
units, files, seed, deps = data
print(f"[DEBUG] Testing the current sized WASI producer {seed.name}", flush=True)
args = [compiler, "--target", target, "--edition=2021", "-Zno-parallel-backend", "-Zthreads=1", "-Copt-level=1", "--test",
        "--crate-name", "gbxml_paged_wasi", "--cfg", 'feature="funded-tests"', "-Clink-arg=--threads=1", "-Clink-arg=--strip-all",
        "-Clink-arg=--max-memory=536870912", "-Clink-arg=-zstack-size=8388608", "-o", str(output / "gbxml-paged-tests.wasm"),
        str(ticket / "gbxml-control-native.rs")]
for path in units:
    args += ["-L", f"dependency={path}"]
for alias in (name, "semio_framework", "semio_framework_value", "semio_framework_job", "semio_os_kernel", "semio_plugin",
              "semio_s_artifact_bim_model_inferences", "semio_s_artifact_bim_model_snapshot", "semio_s_artifact_stdio_xml"):
    archive = seed if alias == name else files[f"lib{alias}-{deps[alias][0]}.rmeta"]
    args += ["--extern", f"{alias}={archive}", "--extern", f"{alias}={archive.with_suffix('.rlib')}"]
print("[DEBUG] Linking the funded WASI tests", flush=True)
with large_link():
    if subprocess.run(args).returncode:
        raise SystemExit("The funded WASI tests did not link")
if "--run" in sys.argv:
    bind = output / "js"
    jco = root / "node_modules/@bytecodealliance/jco/dist/jco.js"
    print("[DEBUG] Transpiling the freshly linked paid WASI tests", flush=True)
    if subprocess.run(["node", str(jco), "transpile", str(output / "gbxml-paged-tests.wasm"), "-o", str(bind), "--no-typescript"]).returncode:
        raise SystemExit("The funded WASI tests did not transpile")
    run = ["node", str(ticket / "run-gbxml-control.mjs"), str(bind / "gbxml-paged-tests.js"),
           "--nocapture", "--skip", "serializer_emits_the_committed_gbxml_files"]
    print("[DEBUG] Running the funded box, zoning and stack laws", flush=True)
    if subprocess.run(run).returncode:
        raise SystemExit("The funded small WASI tests failed")
    print("[DEBUG] Running the funded Family House law", flush=True)
    if subprocess.run([*run, "original_controller_emits_canonical_gbxml_pages"], env=dict(os.environ, BIM_GBXML_HOUSE="1")).returncode:
        raise SystemExit("The funded house WASI law failed")
