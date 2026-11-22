"""🧱️ Check both topology module exports with one canonical run's WASI dependencies."""
import json
import shutil
from pathlib import Path
import subprocess
import sys
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
engine = root / "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine"
rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
libraries = {}
search = {}
with Path(sys.argv[1]).open() as receipts:
    for line in receipts:
        try:
            row = json.loads(line)
        except ValueError:
            continue
        if row.get("reason") != "compiler-artifact":
            continue
        for name in row["filenames"]:
            path = Path(name)
            for dependency in (path, path.with_suffix(".rmeta")):
                if dependency.suffix in {".dylib", ".so", ".dll", ".rmeta"} and dependency.is_file():
                    previous = search.get(dependency.name)
                    if previous is not None and previous != dependency:
                        raise RuntimeError(f"conflicting dependency artifact name: {dependency.name}")
                    search[dependency.name] = dependency
            if path.suffix == ".rlib" and "wasm32-wasip2" in path.parts and path.with_suffix(".rmeta").exists():
                libraries[row["target"]["name"]] = path.with_suffix(".rmeta")
with TemporaryDirectory(dir=ticket / "🗑️generated") as directory:
    scratch = Path(directory)
    def identity(path):
        stat = path.stat()
        return stat.st_dev, stat.st_ino, stat.st_size, stat.st_mtime_ns, stat.st_ctime_ns
    before = {path: identity(path) for path in search.values()}
    for path in search.values():
        shutil.copyfile(path, scratch / path.name)
    if any(identity(path) != expected for path, expected in before.items()):
        raise RuntimeError("WASI dependency artifacts changed during snapshot; retry after lane settles")
    common = [*rustc, "--edition=2021", "--target=wasm32-wasip2", "--emit=metadata", "-L", f"dependency={scratch}"]
    crate = "semio_framework_3d_brep_solid_topology"
    result = scratch / f"lib{crate}.rmeta"
    args = []
    for key in ["boolean", "construction", "curve", "representation", "surface", "tolerance", "vector", "value"]:
        name = "semio_framework_value" if key == "value" else "semio_framework_3d_brep_" + key
        args.extend(["--extern", f"{name}={scratch / libraries[name].name}"])
    subprocess.run([*common, "--crate-name", crate, "--crate-type=rlib", *args, str(engine / "🏗️solid-topology/🦀️.rs"), "-o", str(result)], check=True)
    consumer = "pub use owner::mapped_boundary::{FaceMapping,MappedBoundary,invalid_mapping};\npub use owner::{HelixWireRecipe,PolygonWireRecipe};"
    subprocess.run([*common, "--crate-name", "topology_exports", "--crate-type=rlib", "--extern", f"owner={result}", "-", "-o", str(scratch / "consumer.rmeta")], input=consumer, text=True, check=True)
print("Mapped polygon and wire recipe APIs both resolve on WASI.")
