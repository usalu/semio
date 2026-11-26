"""🧪️ Recompile exactly one cached WASI member-wire unit with its authored component profile."""

import argparse
import json
import os
from pathlib import Path
import resource
import subprocess
import sys
import time

TICKET = Path(__file__).resolve().parent
REPO = next(parent for parent in TICKET.parents if (parent / "AGENTS.md").is_file())
OUT = TICKET / "🗑️generated/wire-measure"
NAME = "semio-s-artifact-draw-drawing-editor-member-wire"
CRATE = NAME.replace("-", "_")
DECODE = "semio-draw-member-decode"
TARGET = "wasm32-wasip2"
PROFILE = "prototype"
TOOL = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]


def plan(lane, graph, codegen, name=NAME):
    units = json.loads(graph.read_text())["units"]
    unit = next(unit for unit in units if unit["target"]["name"] == CRATE and unit["platform"] == TARGET)
    profile = unit["profile"]
    expected = {
        "name": PROFILE, "opt_level": "s", "lto": "off", "codegen_units": 16,
        "debuginfo": 0, "incremental": False, "debug_assertions": True, "overflow_checks": True,
    }
    if any(profile[key] != value for key, value in expected.items()):
        raise ValueError(f"Observed profile no longer matches the baseline: {profile}")
    fingerprints = {}
    outputs = set()
    for directory in (lane / f"{TARGET}/{PROFILE}/build", lane / f"{PROFILE}/build"):
        for stamp in directory.glob("*/*/fingerprint/lib-*.json"):
            fingerprint = stamp.with_suffix("")
            if not fingerprint.is_file():
                continue
            artifacts = stamp.parent.parent / "out"
            if artifacts.is_dir():
                outputs.add(artifacts)
                fingerprints[fingerprint.read_text().strip()] = stamp
    cached = [stamp for stamp in fingerprints.values() if stamp.name == f"lib-{CRATE}.json"]
    if len(cached) != 1:
        raise ValueError(f"Expected the unique cached WASI member wire owner, got {cached}")
    fingerprint = json.loads(cached[0].read_text())
    source = Path(unit["target"]["src_path"]).resolve()
    manifest = Path(unit["target"]["src_path"]).parent.parent.parent
    crate = name.replace("-", "_")
    if name == DECODE:
        directory = source.parent / "📥️decode"
        source = directory / "🦀️.rs"
        manifest = directory / "📦️packages/🦀️rust"
    assert (manifest / "Cargo.toml").is_file(), manifest
    suffix = f"-cgu{codegen}"
    args = [
        str(source), "--edition=2021", "--crate-name", crate, "--crate-type=lib", f"--target={TARGET}",
        "--emit=metadata,link", "-Copt-level=s", "-Clto=off", f"-Ccodegen-units={codegen}",
        "-Cembed-bitcode=no", "-Cdebuginfo=0", "-Cdebug-assertions=yes", "-Coverflow-checks=yes",
        "-Cstrip=symbols", "-Zthreads=1", "-Zno-parallel-backend",
        f"--out-dir={OUT}", f"-Cextra-filename={suffix}",
    ]
    args += [f"-Ldependency={path}" for path in sorted(outputs)]
    for _, alias, _, digest in fingerprint["deps"]:
        if name == DECODE and alias not in {"semio_framework_pack_json", "semio_framework_value",
                                            "semio_framework_value_native_owned", "semio_s_artifact_draw_drawing_mutations"}:
            continue
        owner = fingerprints[digest.to_bytes(8, "little").hex()]
        artifact_name = owner.stem.removeprefix("lib-")
        names = [path for path in (owner.parent.parent / "out").glob(f"lib{artifact_name}-*")
                 if path.suffix in {".rmeta", ".rlib", ".dylib", ".so", ".dll"}]
        if not names:
            raise FileNotFoundError(owner)
        args += [f"--extern={alias}={path}" for path in sorted(names)]
    return args, manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lane", type=Path)
    parser.add_argument("graph", type=Path)
    parser.add_argument("--codegen-units", type=int, choices=(16, 32, 64), default=16)
    parser.add_argument("--part", choices=("wire", "decode", "split-wire"), default="wire")
    parser.add_argument("--run", action="store_true")
    options = parser.parse_args()
    codegen = options.codegen_units
    name = DECODE if options.part == "decode" else NAME
    crate = name.replace("-", "_")
    args, manifest = plan(options.lane.resolve(), options.graph.resolve(), codegen, name)
    if options.part == "split-wire":
        decode = DECODE.replace("-", "_")
        libraries = [OUT / f"lib{decode}-cgu{codegen}.{extension}" for extension in ("rmeta", "rlib")]
        if not all(path.is_file() for path in libraries):
            raise FileNotFoundError("The matching isolated decode unit must finish first")
        args.extend(f"--extern={decode}={path}" for path in libraries)
    print(f"{name} {TARGET} {PROFILE} opt=s lto=off codegen-units={codegen}, {len(args)} arguments")
    if not options.run:
        return 0
    OUT.mkdir(exist_ok=True)
    label = f"{options.part}-cgu{codegen}"
    response = OUT / f"{label}.args"
    response.write_text("\n".join(args) + "\n")
    environment = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest), "CARGO_PKG_NAME": name}
    started = time.monotonic()
    with (OUT / f"{label}.log").open("w") as log:
        run = subprocess.run([*TOOL, f"@{response}"], cwd=REPO, env=environment, stdout=log, stderr=log)
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    rss = usage.ru_maxrss * (1 if sys.platform == "darwin" else 1024)
    peak = {"v": 1, "key": f"{name}@{crate}|{TARGET}|{PROFILE}|lib", "peakRssBytes": rss,
            "atMs": time.time_ns() // 1_000_000, "elapsedSeconds": round(time.monotonic()-started, 3),
            "codegenUnits": codegen, "status": run.returncode}
    print(json.dumps(peak))
    if run.returncode == 0:
        (OUT / f"{label}.jsonl").write_text(json.dumps(peak) + "\n")
    return run.returncode


if __name__ == "__main__":
    sys.exit(main())
