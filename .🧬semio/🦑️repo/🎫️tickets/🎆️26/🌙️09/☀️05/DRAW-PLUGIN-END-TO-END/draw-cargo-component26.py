import hashlib
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys


PACKAGE = "semio-hub-draw-component"
OUTPUT = "semio_hub_draw_component.wasm"
MANIFEST = Path("🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml")
TARGET = "wasm32-wasip2"
PROFILE = "prototype"
COMPONENT_HEADER = b"\0asm\x0d\0\x01\0"
MAX_BYTES = 512 * 1024**2


def command():
    return [
        "repo", "capacity", "run", "--memory=6144", "--cpu=1", "--jobs=1", "--",
        "repo", "cargo", "--store=shared", "--configuration=prototype",
        "--parallelism=1", "--lang=en", "--", "build", "--manifest-path", str(MANIFEST),
        "-p", PACKAGE, "--lib", "--target", TARGET, "--profile", PROFILE,
        "--locked", "--offline", "--color", "never", "--message-format=json-render-diagnostics",
    ]


def produced_artifact(messages, workspace):
    manifest = (workspace / MANIFEST).resolve(strict=True)
    lane_root = (workspace / ".cache/b/r/m/u/l").resolve()
    matching, previous_finish = [], None
    for line in messages:
        if not line.startswith("{"):
            continue
        try:
            row = json.loads(line)
        except ValueError:
            continue
        reason = row.get("reason")
        if reason == "build-finished":
            previous_finish = (row.get("success"), matching)
            matching = []
            continue
        if reason != "compiler-artifact" or row.get("manifest_path") != str(manifest):
            continue
        target = row.get("target", {})
        if target.get("name") != PACKAGE.replace("-", "_") or target.get("crate_types") != ["cdylib"]:
            continue
        if row.get("profile", {}).get("test") is not False or not isinstance(row.get("fresh"), bool):
            raise ValueError("Expected the owner's normal non-test compiler artifact")
        wasm = [Path(path) for path in row.get("filenames", []) if Path(path).name == OUTPUT]
        if len(wasm) != 1 or not wasm[0].is_absolute():
            raise ValueError("Expected one declared Draw output in the compiler artifact row")
        path = wasm[0]
        if not path.resolve().is_relative_to(lane_root) or path.parts[-3:-1] != (TARGET, PROFILE):
            raise ValueError("Draw output is outside the managed prototype WASI target lane")
        matching.append((path, row))
    if matching or not previous_finish or previous_finish[0] is not True or len(previous_finish[1]) != 1:
        raise ValueError("Expected one exact Draw compiler artifact followed by a successful Cargo completion")
    return previous_finish[1][0]


def file_state(info):
    return (info.st_dev, info.st_ino, info.st_mode, info.st_size, info.st_mtime_ns, info.st_ctime_ns)


def keep_component(source, destination):
    before = source.lstat()
    if not stat.S_ISREG(before.st_mode) or not len(COMPONENT_HEADER) < before.st_size <= MAX_BYTES:
        raise ValueError("Expected a bounded regular WASI component")
    digest = hashlib.sha256()
    written = 0
    pending = destination.with_suffix(".pending")
    try:
        with source.open("rb") as handle, pending.open("xb") as output:
            if file_state(before) != file_state(os.fstat(handle.fileno())):
                raise ValueError("Compiler artifact identity changed before reading")
            first = handle.read(len(COMPONENT_HEADER))
            if first != COMPONENT_HEADER:
                raise ValueError("Draw compiler output is not a WebAssembly component")
            block = first
            while block:
                digest.update(block)
                output.write(block)
                written += len(block)
                if written > before.st_size:
                    raise ValueError("Draw compiler output grew during capture")
                block = handle.read(1024**2)
            if written != before.st_size or file_state(before) != file_state(os.fstat(handle.fileno())):
                raise ValueError("Draw compiler output changed during capture")
        if file_state(before) != file_state(source.lstat()):
            raise ValueError("Draw compiler output changed before capture commit")
        pending.rename(destination)
        return digest.hexdigest(), written
    finally:
        if pending.exists():
            pending.unlink()


def build(run_name):
    if not re.fullmatch(r"[a-z][a-z0-9-]{0,27}", run_name):
        raise ValueError("Expected a short, unique ticket run name")
    ticket = Path(__file__).resolve().parent
    output = ticket / "🗑️generated" / run_name
    output.mkdir(parents=True)
    cargo_json = output / "cargo.jsonl"
    cargo_stderr = output / "cargo.log"
    argv = command()
    environment = {
        **os.environ,
        "SEMIO_LANGUAGE": "en",
        "SEMIO_BUILD_MODE": PROFILE,
        "SEMIO_HOST_CAPACITY": "on",
        "SEMIO_OFFLOAD": "never",
        "SEMIO_COMPILER_PARALLELISM": "1",
    }
    print(f"[DEBUG] Draw Cargo output: {output}", flush=True)
    with cargo_json.open("xb") as messages, cargo_stderr.open("xb") as diagnostics:
        result = subprocess.run(argv, env=environment, stdout=messages, stderr=diagnostics)
    if result.returncode:
        raise SystemExit(result.returncode if result.returncode > 0 else 128 - result.returncode)
    with cargo_json.open(encoding="utf-8") as messages:
        source, row = produced_artifact(messages, Path.cwd())
    sha256, size = keep_component(source, output / "component.wasm")
    record = {
        "kind": "draw-cargo-component-diagnostic",
        "command": argv,
        "cargoExit": result.returncode,
        "artifact": row,
        "source": str(source),
        "filename": "component.wasm",
        "sha256": sha256,
        "bytes": size,
    }
    with (output / "artifact.json").open("x", encoding="utf-8") as receipt:
        receipt.write(json.dumps(record, ensure_ascii=False, indent=2) + "\n")
    print(f"[DEBUG] Draw component SHA-256 {sha256}, {size} bytes", flush=True)


if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "plan":
        print(json.dumps(command(), ensure_ascii=False))
    elif len(sys.argv) == 3 and sys.argv[1] == "build":
        build(sys.argv[2])
    else:
        raise SystemExit("Usage: draw-cargo-component26.py plan | build <ticket-run-name>")
