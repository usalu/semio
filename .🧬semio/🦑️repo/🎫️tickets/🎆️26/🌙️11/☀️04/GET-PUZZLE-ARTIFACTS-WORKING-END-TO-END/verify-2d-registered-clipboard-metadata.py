"""Check the Puzzle2D clipboard or history test metadata against the ticket's warm native graph."""

import argparse
import json
import os
import re
import shutil
import signal
import subprocess
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("generated", nargs="?", type=Path, default=Path(__file__).resolve().parent / "🗑️generated")
parser.add_argument("--history", action="store_true", help="check the dedicated history runtime test")
parser.add_argument("--reuse-fixtures", action="store_true", help="reuse the checked v1 and unit-test metadata")
parser.add_argument("--native", action="store_true", help="link the three clipboard units with the Linux memory and disk guard")
options = parser.parse_args()
if options.native and options.history:
    parser.error("--native checks the clipboard root")
if options.native and not Path("/proc/meminfo").is_file():
    parser.error("--native requires Linux process-group and memory monitoring")
generated = options.generated.resolve()
build = generated / "cargo-puzzle-clipboard-build/debug/build"
scratch = generated / ("2d-clipboard-raw-native" if options.native else "2d-clipboard-metadata")
scratch.mkdir(exist_ok=True)
deps = scratch / "deps"
deps.mkdir(exist_ok=True)
plan_name, manifest_name = (
    ("2d-history-runtime-current-plan.json", "puzzle-2d-history-edit-runtime")
    if options.history else ("2d-registered-clipboard-native-profile.json", "puzzle-2d-registered-clipboard")
)
plan = json.loads((generated / plan_name).read_text())
units = plan["units"]
test = units[plan["roots"][0]]
anchor_unit = next(units[dep["index"]] for dep in test["dependencies"] if dep["extern_crate_name"] == "semio_s_artifact_puzzle_2d_v1")
anchor_name = anchor_unit["target"]["name"]
anchor_id = "4963e948fe62720b"
anchor = build / anchor_name.replace("_", "-") / anchor_id / "out" / f"lib{anchor_name}-{anchor_id}.rmeta"

def versions(path):
    listing = subprocess.check_output(["rustc", "-Z", "ls=all", path], text=True, stderr=subprocess.DEVNULL)
    return set(re.findall(r"^\d+ (\w+)-([0-9a-f]+) hash ", listing, re.MULTILINE))

known = versions(anchor)
known.add((anchor_name, anchor_id))
all_libs = {}
for output in build.glob("*/*/out"):
    for suffix in (".rmeta", ".so", ".dylib", ".dll"):
        for path in output.glob("lib*" + suffix):
            name, fingerprint = path.stem[3:].rsplit("-", 1)
            all_libs.setdefault((name, fingerprint), []).append(path)

def link_versions(selected):
    for name, fingerprint in selected:
        for candidate in all_libs.get((name, fingerprint), []):
            artifacts = [candidate]
            if options.native and candidate.suffix == ".rmeta":
                archive = candidate.with_suffix(".rlib")
                if archive.is_file():
                    artifacts.append(archive)
            for artifact in artifacts:
                target = deps / artifact.name
                if not target.exists():
                    if os.name == "nt":
                        target.hardlink_to(artifact)
                    else:
                        target.symlink_to(artifact)

def choose(unit):
    name = unit["target"]["name"]
    known_paths = [p for pair in known if pair[0] == name for p in all_libs.get(pair, [])]
    if len(known_paths) == 1:
        return known_paths[0]
    matches = [file for (crate, _), files in all_libs.items() if crate == name for file in files]
    if len(matches) == 1:
        return matches[0]
    exact = []
    for candidate in matches:
        facts = tuple(candidate.parent.parent.joinpath("fingerprint").glob(f"lib-{name}.json"))
        if facts and json.loads(json.loads(facts[0].read_text())["features"]) != unit["features"]:
            continue
        candidate_versions = versions(candidate)
        if all(pair in known or not any(k[0] == pair[0] for k in known) for pair in candidate_versions):
            exact.append(candidate)
    if len(exact) != 1:
        raise RuntimeError((name, unit["features"], matches, exact))
    return exact[0]

def run_native(command, env):
    def headroom():
        memory = next(int(line.split()[1]) * 1024 for line in Path("/proc/meminfo").read_text().splitlines() if line.startswith("MemAvailable:"))
        space = shutil.disk_usage(scratch).free
        print(f"[DEBUG] native headroom memory {memory / 2**30:.2f} GiB, disk {space / 2**30:.2f} GiB", flush=True)
        return memory >= 2.5 * 2**30 and space >= 3.6 * 2**30

    if not headroom():
        return 75
    with subprocess.Popen(command, env=env, start_new_session=True) as child:
        while True:
            try:
                return child.wait(timeout=15)
            except subprocess.TimeoutExpired:
                if headroom():
                    continue
                os.killpg(child.pid, signal.SIGTERM)
                try:
                    child.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
                return 75

base = ["rustc", "--edition", "2021", "-Zthreads=1", "-Cdebuginfo=0", "-L", f"dependency={deps}"]
if options.native:
    base.extend(["-Zno-parallel-backend", "-Ccodegen-units=16"])
else:
    base.append("--emit=metadata")
patched_anchor = scratch / f"lib{anchor_name}.rmeta"

fixture_name = "semio_s_artifact_puzzle_2d_unit_tests"
fixture_unit = next(units[dep["index"]] for dep in test["dependencies"] if dep["extern_crate_name"] == fixture_name)
patched_fixture = scratch / f"lib{fixture_name}.rmeta"
patched = {anchor_name: patched_anchor, fixture_name: patched_fixture}
test_name = test["target"]["name"].replace("-", "_")
stages = [(anchor_unit, ["testkit"], patched_anchor), (fixture_unit, [], patched_fixture), (test, test["features"], scratch / f"{test_name}.rmeta")]
if options.reuse_fixtures:
    assert patched_anchor.exists() and patched_fixture.exists()
    if options.native:
        assert patched_anchor.with_suffix(".rlib").exists() and patched_fixture.with_suffix(".rlib").exists()
    stages = stages[-1:]
for unit, features, output in stages:
    externs = []
    for dependency in unit["dependencies"]:
        dep_unit = units[dependency["index"]]
        chosen = patched.get(dep_unit["target"]["name"])
        if chosen is None:
            chosen = choose(dep_unit)
        if chosen not in patched.values() and chosen.suffix == ".rmeta":
            selected = versions(chosen)
            known.update(selected)
        alias = dependency["extern_crate_name"]
        if options.native and chosen.suffix == ".rmeta":
            archive = chosen.with_suffix(".rlib")
            if not archive.is_file():
                raise RuntimeError(f"native archive absent for {alias}: {archive}")
            externs.extend(["--extern", f"{alias}={archive}"])
        externs.extend(["--extern", f"{alias}={chosen}"])
    link_versions(known)
    name = unit["target"]["name"].replace("-", "_")
    path = Path(unit["target"]["src_path"])
    manifest_dir = generated / manifest_name if unit is test else path.parent
    env = dict(os.environ, CARGO_MANIFEST_DIR=str(manifest_dir), CARGO_PKG_VERSION="0.1.0", CARGO_PKG_NAME=name)
    env.setdefault("RUST_MIN_STACK", "134217728")
    kind = ["--test"] if unit is test else ["--crate-type", "rlib"]
    command = base + kind + ["--crate-name", name] + externs
    if options.native:
        env.update(ZIG_GLOBAL_CACHE_DIR=str(scratch / "zig-global"), ZIG_LOCAL_CACHE_DIR=str(scratch / "zig-local"))
        if unit is test:
            command += ["-Zunstable-options", "-Clinker-features=+lld", "-Clink-self-contained=+linker", "-o", str(output.with_suffix(""))]
        else:
            command += [f"--emit=link={output.with_suffix('.rlib')},metadata={output}"]
    else:
        command += ["-o", str(output)]
    for feature in features:
        command += ["--cfg", f'feature="{feature}"']
    mode = "native" if options.native else "metadata"
    print(f"[DEBUG] {mode} {name} {features}", flush=True)
    command.append(str(path))
    result = run_native(command, env) if options.native else subprocess.run(command, env=env).returncode
    print(f"[DEBUG] {mode} {name} exit {result}", flush=True)
    if result:
        raise SystemExit(result)
