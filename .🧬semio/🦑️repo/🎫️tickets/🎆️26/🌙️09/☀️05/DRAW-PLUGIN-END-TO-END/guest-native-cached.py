"""🖍️ Run the repository-owned Draw lifecycle against the pinned fcc8 native cache."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


PACKAGE = "semio-hub-draw-guest-tests"
TEST = "quick::wasmtime_runtime_instance_open_settles_against_the_draw_component"
ATTESTED_FILES = (
    ("📦️component.wasm", "fcc8e11c2267be890c236bf25f06d9e378b87b4becc78bebb74d95a6bcfb10e2", 143779747),
    ("🔣️.json", "6023b1884a5af2fddfa0be7dec69d110b090d4d55e304fa20ac30e220dcd0da0", 326964),
)
AOT_NAMESPACE = "7ca493006f935a741b013e1093c9cb9cc67951d05d555fc580cab4616dd67bc7"
AOT_PACKAGE = "abda83e2795a345cac46d4f6ffcebf70677a6869c2bdca7a8f1d1a8cb650d729"
AOT_SHA256 = "8c3ca8263f991c09aee56ff8add6ea5326e6cba1ad086bac6d474bf6823e9d97"


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as file:
        for chunk in iter(lambda: file.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def expect_file(path, digest, size):
    if path.stat().st_size != size or sha256(path) != digest:
        raise RuntimeError(f"Unexpected artifact bytes at {path}")
    print(f"[DEBUG] Verified {size} bytes and SHA-256 {digest} at {path}", flush=True)


def attest(model, home):
    component, descriptor = (model / row[0] for row in ATTESTED_FILES)
    for filename, digest, size in ATTESTED_FILES:
        expect_file(model / filename, digest, size)
    receipt = json.loads((model / "🧾️receipt.json").read_text(encoding="utf-8"))
    for key, expected in {
        "pluginId": "draw", "packageId": "semio:draw", "profile": "prototype", "target": "wasm32-wasip2",
        "wasmSha256": ATTESTED_FILES[0][1], "wasmBytes": ATTESTED_FILES[0][2],
        "semanticDescriptorSha256": "d1547e6294d0ff47d507cee7c0143001b3fe75e8ce9cd2d0fede5ccf07ea3884",
    }.items():
        if receipt.get(key) != expected:
            raise RuntimeError(f"Pinned component receipt differs at {key}")
    cache = home / ".semio" / "cache" / "wasmtime" / AOT_NAMESPACE / f"{AOT_PACKAGE}.cwasm"
    expect_file(cache, AOT_SHA256, 590573904)
    return component, descriptor, cache


def compile_guest_tests(hub):
    command = [
        "repo", "cargo", "--store=shared", "--configuration=prototype", "--parallelism=1", "--",
        "test", "--locked", "-p", PACKAGE, "--lib", "--no-run", "--message-format=json-render-diagnostics",
    ]
    executables = set()
    with subprocess.Popen(command, cwd=hub, stdout=subprocess.PIPE, text=True, encoding="utf-8") as child:
        for line in child.stdout:
            print(line, end="", flush=True)
            if not line.startswith("{"):
                continue
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            target = event.get("target", {}).get("name", "").replace("_", "-")
            if event.get("reason") == "compiler-artifact" and target == PACKAGE and event.get("profile", {}).get("test") and event.get("executable"):
                executables.add(Path(event["executable"]))
        if child.wait():
            raise RuntimeError("Managed Draw guest test compilation failed")
    if len(executables) != 1:
        raise RuntimeError(f"Expected one compiled Draw guest test executable, got {executables}")
    return executables.pop()


def run_guest_tests(executable, component, descriptor, home):
    if not executable.is_file():
        raise RuntimeError(f"No Draw lifecycle test executable at {executable}")
    print(f"[DEBUG] Lifecycle executable SHA-256 {sha256(executable)} at {executable}", flush=True)
    environment = dict(
        os.environ, SEMIO_HOME=str(home), SEMIO_DRAW_GUEST_COMPONENT=str(component),
        SEMIO_DRAW_GUEST_DESCRIPTOR=str(descriptor), SEMIO_DRAW_GUEST_CACHE_ONLY="1",
        RUST_MIN_STACK=os.environ.get("RUST_MIN_STACK", "134217728"),
    )
    command = ["repo", "capacity", "run", "--memory=1536", "--", str(executable), "--exact", TEST, "--test-threads=1", "--nocapture"]
    expected = {
        "test result: ok. 1 passed",
        "Native Draw editor opened, closed and retired",
        "Native Draw viewer opened, closed and retired",
    }
    with subprocess.Popen(command, env=environment, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, encoding="utf-8") as child:
        for line in child.stdout:
            print(line, end="", flush=True)
            expected = {marker for marker in expected if marker not in line}
        if child.wait() or expected:
            raise RuntimeError(f"Draw lifecycle verification failed. Missing evidence: {sorted(expected)}")


def main():
    root = next(parent for parent in Path(__file__).resolve().parents if (parent / "🌎️hub" / "Cargo.toml").is_file())
    generated = Path(__file__).resolve().parent / "🗑️generated"
    parser = argparse.ArgumentParser(description=__doc__)
    action = parser.add_mutually_exclusive_group(required=True)
    action.add_argument("--preflight", action="store_true")
    action.add_argument("--cargo", action="store_true")
    action.add_argument("--executable", type=Path)
    parser.add_argument("--model", type=Path, default=generated / "fcc8-model")
    parser.add_argument("--home", type=Path, default=generated / "wasmtime-fcc8-production")
    arguments = parser.parse_args()
    home = arguments.home.absolute()
    component, descriptor, cache = attest(arguments.model.absolute(), home)
    if arguments.preflight:
        return
    executable = compile_guest_tests(root / "🌎️hub") if arguments.cargo else arguments.executable.absolute()
    run_guest_tests(executable, component, descriptor, home)
    expect_file(cache, AOT_SHA256, 590573904)


if __name__ == "__main__":
    main()
