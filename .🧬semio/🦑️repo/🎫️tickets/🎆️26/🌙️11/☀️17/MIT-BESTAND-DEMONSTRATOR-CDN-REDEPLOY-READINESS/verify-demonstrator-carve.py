"""🎪️ Recheck the Demonstrator surface owners against a coherent retained cargo lane."""
import argparse
import os
from pathlib import Path
import shlex
import subprocess
import sys


def cargo_arguments(receipt):
    """🧾️ Read the original outer crate's exact argument list from the cargo failure."""
    with receipt.open() as stream:
        row = next(line for line in stream if "process didn't exit successfully" in line and "semio_hub_demonstrator --edition" in line)
    return shlex.split(row.partition(" `")[2].partition("` (")[0])


def available_dependencies(args, output):
    """📚️ Shorten the coherent compiler search path through scratch symlinks."""
    search = [Path(value.removeprefix("dependency=")) for key, value in zip(args, args[1:]) if key == "-L" and value.startswith("dependency=")]
    direct = {name: Path(path) for key, pair in zip(args, args[1:]) if key == "--extern" for name, path in [pair.split("=", 1)]}
    for directory in search:
        for path in directory.glob("*"):
            if path.suffix not in {".rlib", ".rmeta", ".dylib", ".so"}:
                continue
            link = output / path.name
            if not link.is_symlink():
                link.symlink_to(path)
            name = path.name.split("-")[0].removeprefix("lib")
            if name in {"semio_framework_hash", "semio_framework_tool_run"} and path.suffix == ".rmeta" and "wasm32-wasip2" in path.parts:
                assert name not in direct or direct[name] == path, name
                direct[name] = path
    return direct


def compile_unit(source, name, externs, output, codegen=False):
    """⚙️ Compile a WASI unit with the recorded dependency identities."""
    command = [
        "repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
        "rustc", "--edition=2021", "--target", "wasm32-wasip2", "--crate-type", "lib",
        "--emit", "metadata,link" if codegen else "metadata", "--out-dir", str(output), "-L", "dependency=" + str(output),
        str(source), "--crate-name", name,
    ]
    if codegen:
        command += ["-C", "opt-level=s", "-C", "linker-plugin-lto", "-C", "codegen-units=16", "-C", "strip=symbols", "-Z", "embed-metadata=no", "-Z", "threads=1"]
    command += [arg for key, path in externs.items() for arg in ("--extern", f"{key}={path}")]
    print("[DEBUG] WASI codegen" if codegen else "[DEBUG] WASI metadata", name, flush=True)
    subprocess.run(command, check=True, env=os.environ | {"CARGO_PKG_VERSION": "0.1.0", "CARGO_MANIFEST_DIR": str(source.parent.resolve())})
    if codegen and sys.platform == "darwin":
        import resource
        print("[DEBUG] Maximum child resident bytes", resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss, flush=True)
    return output / ("lib" + name + ".rmeta")


def main(receipt, output, router_only, codegen):
    """🧩️ Check every mounted surface first, then the outer routing composition."""
    root = Path("🌎️hub/🧩️compositions/🎪️demonstrator")
    output.mkdir(parents=True, exist_ok=True)
    direct = available_dependencies(cargo_arguments(receipt), output)
    common = ["semio_framework", "semio_framework_dispatch_macros", "semio_framework_hash", "semio_framework_job", "semio_framework_tool_run", "semio_framework_value", "semio_os_kernel", "semio_plugin"]
    artifacts = {
        "playground-editor": "demonstrator_playground",
        "playground-viewer": "demonstrator_playground",
        "process-editor": "process_process3d_editor",
        "process-viewer": "process_process3d_viewer",
        "sourcing-editor": "sourcing_curation",
        "sourcing-viewer": "sourcing_curation",
    }
    for surface, artifact in artifacts.items():
        role = "✏️" if surface.endswith("editor") else "👁️"
        name = "semio_hub_demonstrator_" + surface.replace("-", "_") + "_app"
        path = root / "🗃️apps" / (role + surface) / "🦀️.rs"
        dependencies = {key: direct[key] for key in [*common, "semio_s_artifact_" + artifact]}
        if codegen and codegen != "router":
            if codegen == surface:
                compile_unit(path, name, dependencies, output, codegen=True)
                return
        elif router_only or codegen == "router":
            direct[name] = output / ("lib" + name + ".rmeta")
        else:
            direct[name] = compile_unit(path, name, dependencies, output)
    compile_unit(root / "📦️packages/🦀️rust/🦀️.rs", "semio_hub_demonstrator_router", direct, output, codegen=codegen == "router")
    print("[DEBUG] WASI router verification passed")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("receipt", type=Path)
    parser.add_argument("output", type=Path)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--router-only", action="store_true")
    mode.add_argument("--codegen", choices=("playground-editor", "playground-viewer", "process-editor", "process-viewer", "sourcing-editor", "sourcing-viewer", "router"))
    args = parser.parse_args()
    main(args.receipt, args.output.resolve(), args.router_only, args.codegen)
