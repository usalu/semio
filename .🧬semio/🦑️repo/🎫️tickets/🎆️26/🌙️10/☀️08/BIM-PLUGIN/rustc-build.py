#!/usr/bin/env python3
"""🦀 Compile BIM with bounded backend memory and retry a corrected unit in its current Cargo graph."""
import json
import os
from pathlib import Path
import subprocess
import shutil
import tempfile
import sys
import time

compiler, *args = sys.argv[1:]
crate = args[args.index("--crate-name") + 1] if "--crate-name" in args else ""
if crate in ("semio_s_artifact_stdio_pdf_model_graphics","semio_s_artifact_bim_model_schema","semio_s_artifact_bim_model_snapshot","semio_s_artifact_bim_model_mut_any_190","semio_s_artifact_bim_model_kit","semio_s_artifact_bim_model_bim","semio_hub_bim_component"):
    args.extend(["-Zno-parallel-backend", "-Zthreads=1"])
if crate == "semio_hub_bim_component" and "wasm32-wasip2" in args:
    args.extend(["-Clink-arg=--threads=1", "-Clink-arg=--strip-all"])
if not (crate.startswith("semio_s_artifact_bim") or crate.startswith("semio_hub_bim")):
    if not crate:
        os.execv(compiler, [compiler, *args])
    scratch = Path(__file__).resolve().parent / "🗑️generated"
    for attempt in range(5):
        with tempfile.TemporaryFile(dir=scratch) as stream:
            completed = subprocess.run([compiler, *args], stderr=stream, close_fds=False)
            if completed.returncode not in (-9, -15) or attempt == 4:
                stream.seek(0)
                shutil.copyfileobj(stream, sys.stderr.buffer)
                sys.exit(completed.returncode if completed.returncode >= 0 else 128 - completed.returncode)
        if "-Zno-parallel-backend" not in args:
            args.extend(["-Zno-parallel-backend", "-Zthreads=1"])
        time.sleep(2 * (attempt + 1))
    sys.exit(128 - completed.returncode)

directory = Path(__file__).resolve().parent / "🗑️generated" / "compiler-retry"
directory.mkdir(exist_ok=True)
target = args[args.index("--target") + 1] if "--target" in args else "native"
name = "{}-{}-{}".format("w" if target.startswith("wasm") else "n", crate.rsplit("_", 1)[-1][:16], os.getpid())
report = directory / (name + ".log")
retry = directory / (name + ".retry")
failure = directory / (name + ".stop")
for attempt in range(12):
    with report.open("wb") as capture:
        completed = subprocess.run([compiler, *args], stderr=capture, close_fds=False)
    if completed.returncode == 0:
        if crate in {"semio_s_artifact_bim_model_entity_records", "semio_s_artifact_bim_model_mut_any_190", "semio_s_artifact_bim_model_snapshot", "semio_s_artifact_bim_model_ifc", "semio_s_artifact_bim_model_svg", "semio_hub_bim_component", "semio_s_artifact_bim_model_bim", "semio_s_artifact_bim_model_viewer"}:
            try:
                import resource
                rss = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
                maximum_bytes = rss if sys.platform == "darwin" else rss * 1024
            except ImportError:
                maximum_bytes = None
            (directory / (name + ".success.json")).write_text(json.dumps({
                "crate": crate, "target": target, "peakResidentBytes": maximum_bytes,
                "compiler": compiler, "arguments": args, "cwd": os.getcwd(),
                "cargo": {key: value for key, value in os.environ.items()
                          if key in ("OUT_DIR", "TARGET", "HOST") or key.startswith(("CARGO_PKG_", "CARGO_MANIFEST_"))}
            }))
        sys.stderr.buffer.write(report.read_bytes())
        report.unlink()
        break
    if completed.returncode < 0 and "-Zno-parallel-backend" not in args:
        args.extend(["-Zno-parallel-backend", "-Zthreads=1"])
        continue
    if completed.returncode in (-9, -15) and attempt < 4 and target.startswith("wasm"):
        time.sleep(3 * (attempt + 1))
        continue
    details = []
    for line in report.read_text(errors="replace").splitlines():
        try:
            diagnostic = json.loads(line)
            if diagnostic.get("level") == "error":
                details.append(diagnostic.get("rendered", diagnostic.get("message", "")))
        except (ValueError, TypeError):
            details.append(line)
    summary = directory / (name + ".error")
    summary.write_text("\n".join(details))
    (directory / (name + ".invocation.json")).write_text(json.dumps({
        "compiler": compiler, "arguments": args, "cwd": os.getcwd(),
        "cargo": {key: value for key, value in os.environ.items()
                 if key in ("OUT_DIR", "TARGET", "HOST") or key.startswith(("CARGO_PKG_", "CARGO_MANIFEST_"))},
    }))
    deadline = time.monotonic() + 900
    while time.monotonic() < deadline and not (retry.exists() or failure.exists()):
        time.sleep(1)
    if failure.exists() or not retry.exists() or attempt == 11:
        sys.stderr.buffer.write(report.read_bytes())
        sys.exit(completed.returncode if completed.returncode > 0 else 128 - completed.returncode)
    retry.unlink()
    summary.unlink(missing_ok=True)
