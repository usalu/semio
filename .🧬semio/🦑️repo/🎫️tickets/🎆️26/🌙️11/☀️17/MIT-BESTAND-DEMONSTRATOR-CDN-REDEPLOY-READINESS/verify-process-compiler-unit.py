"""🏭️ Replay one Process WASI registration unit against a retained release pass."""

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys


def artifact_paths(receipt, search, codegen):
    """📦️ Expose exact release-pass artifacts through a shorter search directory."""
    artifacts = {}
    for line in receipt.open():
        if '"reason":"compiler-artifact"' not in line:
            continue
        event = json.loads(line)
        name = event["target"]["name"]
        for path in map(Path, event["filenames"]):
            if path.suffix not in {".rlib", ".rmeta", ".dylib", ".so", ".dll"} or not path.is_file():
                continue
            mounted = search / path.name
            if not mounted.is_symlink():
                mounted.symlink_to(path)
            assert mounted.resolve() == path.resolve(), mounted
            if (
                "proc-macro" in event["target"]["kind"] and path.suffix in {".dylib", ".so", ".dll"}
                or "wasm32-wasip2" in path.parts and path.suffix == (".rlib" if codegen else ".rmeta")
            ):
                artifacts.setdefault(name, set()).add(mounted)
    return artifacts


def replay(graph, receipt, role, output, codegen):
    """🔧️ Compile only the selected native surface and leave the original pass untouched."""
    crate = f"semio_hub_process_{role}_app"
    units = json.loads(graph.read_text())["units"]
    unit = next(unit for unit in units if unit["target"]["name"] == crate and unit["platform"] == "wasm32-wasip2")
    source = Path(unit["target"]["src_path"])
    output.mkdir(parents=True, exist_ok=True)
    extern_dir = output / "deps"
    extern_dir.mkdir(exist_ok=True)
    artifacts = artifact_paths(receipt, extern_dir, codegen)
    externs = []
    for edge in unit["dependencies"]:
        name = units[edge["index"]]["target"]["name"]
        available = artifacts.get(name, set())
        if len(available) != 1:
            raise ValueError(f"Expected one retained dependency for {name}, found {available}")
        externs.extend(("--extern", f"{edge['extern_crate_name']}={next(iter(available))}"))
    command = [
        "repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
        "--edition=2021", "--target", "wasm32-wasip2", "--crate-type", "lib", "--crate-name", crate,
        "--emit", "metadata,link" if codegen else "metadata", "--out-dir", str(output),
        "-L", f"dependency={extern_dir}", *externs, str(source),
    ]
    if codegen:
        command.extend(("-C", "opt-level=s", "-C", "linker-plugin-lto", "-C", "codegen-units=16", "-C", "strip=symbols", "-Z", "embed-metadata=no", "-Z", "threads=1"))
    print("[DEBUG] Replaying", crate, "with", len(externs) // 2, "retained dependencies", flush=True)
    subprocess.run(command, check=True, env=os.environ | {"CARGO_PKG_VERSION": "0.1.0"})
    if codegen and sys.platform == "darwin":
        import resource
        peak = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
        print("[DEBUG] Peak resident bytes", peak)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("graph", type=Path)
    parser.add_argument("receipt", type=Path)
    parser.add_argument("role", choices=("editor", "viewer"))
    parser.add_argument("output", type=Path)
    parser.add_argument("--codegen", action="store_true")
    arguments = parser.parse_args()
    replay(arguments.graph, arguments.receipt, arguments.role, arguments.output.resolve(), arguments.codegen)
