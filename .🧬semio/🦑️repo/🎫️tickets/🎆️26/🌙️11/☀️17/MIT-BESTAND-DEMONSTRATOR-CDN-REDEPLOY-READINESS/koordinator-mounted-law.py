"""📐️ Compile the bounded CAD ingress witness from already admitted release units."""

import argparse
import json
from pathlib import Path
import subprocess


def main():
    """🧾️ Resolve only exact compiled release receipts and avoid the Cargo build graph."""
    parser = argparse.ArgumentParser()
    parser.add_argument("cargo_receipts", type=Path)
    parser.add_argument("rustc", type=Path)
    parser.add_argument("--link", action="store_true")
    args = parser.parse_args()
    ticket = Path(__file__).resolve().parent
    output = ticket / "🗑️generated" / "koordinator-mounted-law"
    dependency = output / "deps"
    dependency.mkdir(parents=True, exist_ok=True)
    libraries = {}
    with args.cargo_receipts.open() as receipts:
        for line in receipts:
            try:
                artifact = json.loads(line)
            except ValueError:
                continue
            if artifact.get("reason") != "compiler-artifact":
                continue
            for filename in artifact.get("filenames", []):
                path = Path(filename).resolve()
                if path.suffix not in (".rlib", ".rmeta", ".dylib", ".so", ".dll") or not path.is_file():
                    continue
                link = dependency / path.name
                if not link.exists():
                    link.symlink_to(path)
                if path.suffix == ".rlib" and "wasm32-wasip2" in path.parts:
                    libraries[artifact["target"]["name"]] = link
    source = ticket / "koordinator-mounted-law.rs"
    names = ("semio_framework", "semio_framework_actor", "semio_framework_async", "semio_framework_job", "semio_framework_value", "semio_os_kernel", "semio_plugin", "semio_plugin_reactor_app", "semio_plugin_reactor_context", "semio_plugin_retained_command", "semio_plugin_ownership", "semio_hub_demonstrator")
    command = [str(args.rustc), "--edition=2021", "--target=wasm32-wasip2", "--crate-name=koordinator_mounted_law", "-Cdebuginfo=0", "-L", f"dependency={dependency}"]
    for name in names:
        library = libraries[name]
        command.extend(["--extern", f"{name}={library.with_suffix('.rmeta')}"])
        if args.link:
            command.extend(["--extern", f"{name}={library}"])
    if args.link:
        mode = ["-Copt-level=1", "-Clink-arg=--threads=1", "-Clink-arg=-z", "-Clink-arg=stack-size=67108864", "-o", output / "law.wasm"]
    else:
        mode = ["--emit=metadata", "-o", output / "law.rmeta"]
    return subprocess.run([*command, *mode, source], check=False).returncode


if __name__ == "__main__":
    raise SystemExit(main())
