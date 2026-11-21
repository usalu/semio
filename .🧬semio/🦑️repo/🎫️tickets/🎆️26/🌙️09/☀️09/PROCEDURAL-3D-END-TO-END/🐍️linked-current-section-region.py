"""🧵️ Rebuild live Section analysis, topology, and native registry against the admitted Rust artifacts."""
from collections import defaultdict
import json
import os
from pathlib import Path
import subprocess
import sys
from tempfile import TemporaryDirectory
import re

ticket = Path(__file__).resolve().parent
cache = Path(".cache/b/r/m/u/l")
managed = cache / sys.argv[1] / "prototype/build"
anchor = next((managed / "semio-framework-3d-brep-native-registry").glob("*/out/*.rlib"))
brep = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
sources = (
    brep / "💡️queries/🔎️analysis",
    brep / "⚙️engine/🏗️solid-topology",
    brep / "⚙️engine/🧠️native-registry",
)
compiler = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
artifacts = defaultdict(list)
for candidate in cache.glob("*/prototype/build/*/*/out/*"):
    if candidate.suffix in (".rlib", ".dylib", ".so", ".dll"):
        artifacts[candidate.stem.removeprefix("lib")].append(candidate)

def metadata(path):
    record = path.with_suffix(".rmeta.semio-deps")
    return json.loads(record.read_text(encoding="utf-8")) if record.is_file() else {}

lineage = dict(metadata(anchor)["dependencies"])
selected = {}
linked = []
for name, digest in lineage.items():
    if name in artifacts:
        matching = [f for f in artifacts[name] if metadata(f).get("svh") == digest or f.suffix != ".rlib"]
        if matching:
            linked.append(matching[0])
            selected[name.rsplit("-", 1)[0]] = matching[0]

def compile_source(source, crate, deps, destination, *, manifest=None):
    args = [str(source), "-A", "warnings", "--edition", "2021", "--crate-name", crate, "-o", str(destination)]
    args += ["--test"] if manifest is None else ["--crate-type=rlib", "-C", "metadata=live_section_region"]
    for directory in sorted({f.parent for f in [*linked, *selected.values(), *deps.values()]}):
        args += ["-L", "dependency=" + str(directory)]
    for alias, file in deps.items():
        args += ["--extern", alias + "=" + str(file)]
        if file.with_suffix(".rmeta").is_file():
            args += ["--extern", alias + "=" + str(file.with_suffix(".rmeta"))]
    env = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest or source.parent)}
    print("[DEBUG] Compiling " + crate, flush=True)
    subprocess.run(compiler + args, check=True, env=env)

with TemporaryDirectory(dir=ticket / "🗑️generated") as fresh:
    fresh = Path(fresh)
    for source in sources:
        manifest = source / "📦️packages/🦀️rust/Cargo.toml"
        package = manifest.read_text(encoding="utf-8")
        direct = package.split("[dependencies]", 1)[1].split("\n[", 1)[0]
        aliases = [key.replace("-", "_") for key in re.findall(r"(?m)^([\w-]+)\.workspace", direct)]
        name = re.search(r'(?m)^name = "([^"]+)"', package)[1].replace("-", "_")
        destination = fresh / ("lib" + name + "-live_section_region.rlib")
        compile_source(source / "🦀️.rs", name, {alias: selected[alias] for alias in aliases}, destination, manifest=manifest.resolve().parent)
        selected[name] = destination
    if len(sys.argv) > 2:
        binary = fresh / ("section-region.exe" if os.name == "nt" else "section-region")
        deps = [name for name in selected if name.startswith("semio_framework_3d_brep_") or name == "semio_framework_value"]
        compile_source(Path(sys.argv[2]), "linked_section_region", {alias: selected[alias] for alias in deps}, binary)
        launcher = compiler[:-1]
        subprocess.run(launcher + [str(binary), "--nocapture"], check=True, env={**os.environ, "RUST_MIN_STACK": "33554432"})
