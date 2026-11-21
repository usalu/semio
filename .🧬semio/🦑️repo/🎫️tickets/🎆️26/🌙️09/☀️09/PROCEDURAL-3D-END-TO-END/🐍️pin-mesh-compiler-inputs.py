"""📌️ Retain one coherent set of finished compiler artifacts for the ticket's Mesh proofs."""

import collections
import json
import os
from pathlib import Path
import shutil
import sys
from tempfile import TemporaryDirectory

cache = Path(".cache/b/r/m/u/l")
ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
artifacts = collections.defaultdict(list)
for lane in cache.glob("*/prototype/build"):
    for item in lane.glob("*/*/out/*"):
        if item.suffix in (".rlib", ".dylib", ".so", ".dll"):
            artifacts[item.stem.removeprefix("lib")].append(item)


def metadata(item):
    try:
        return json.loads(item.with_suffix(".rmeta.semio-deps").read_text(encoding="utf-8"))
    except FileNotFoundError:
        return {}


def candidate(name, svh=None):
    variants = artifacts.get(name, [])
    for item in sorted(variants, key=lambda path: path.stat().st_mtime, reverse=True):
        signature = metadata(item).get("svh")
        if svh is None or signature == svh or (signature is None and item.suffix != ".rlib"):
            return item
    return None


def closure(roots):
    selected = {}
    for name, item in roots:
        selected[name] = item
        for dependency, signature in metadata(item).get("dependencies", []):
            if dependency not in artifacts:
                if dependency.startswith(("semio_", "protocol-")):
                    raise ValueError("Missing required compiler input " + dependency)
                continue
            match = candidate(dependency, signature)
            if match is None:
                raise ValueError("No matching compiler input " + dependency)
            if dependency in selected and selected[dependency] != match:
                raise ValueError("Conflicting compiler provenance for " + dependency)
            selected[dependency] = match
    return selected


engines = [name for name in artifacts if name.startswith("semio_framework_mesh_engine-")]
protocols = [name for name in artifacts if name.startswith("protocol-")]
roots_by_preference = sorted(engines, key=lambda name: candidate(name).stat().st_mtime, reverse=True)
roots_by_preference += sorted(protocols, key=lambda name: candidate(name).stat().st_mtime, reverse=True)
if len(sys.argv) > 1:
    roots_by_preference = [sys.argv[1]]
selected = None
for name in roots_by_preference:
    engine = candidate(name)
    lineage = dict(metadata(engine).get("dependencies", []))
    protocol = name if name.startswith("protocol-") else next((key for key in lineage if key.startswith("protocol-")), None)
    value = next((key for key in lineage if key.startswith("semio_framework_value-")), None)
    if protocol is None or value is None:
        continue
    roots = [(name, engine)]
    for prefix in (
        "semio_framework_job-", "semio_framework_job_testing_caller-", "serde_json-",
        "semio_framework_value_derive-", "semio_framework_dsl_record-",
        "semio_framework_value_native_owned-", "semio_framework_pack_json-",
        "pack-", "semio_framework_io_base64-", "semio_framework_value_source_hold-",
    ):
        exact = next((key for key in lineage if key.startswith(prefix)), None)
        options = [exact] if exact is not None else [key for key in artifacts if key.startswith(prefix)]
        for key in options:
            item = candidate(key, lineage.get(key))
            if item is None:
                continue
            dependencies = dict(metadata(item).get("dependencies", []))
            bound_value = next((key for key in dependencies if key.startswith("semio_framework_value-")), None)
            if bound_value is not None and (bound_value != value or dependencies[value] != lineage[value]):
                continue
            try:
                closure(roots + [(key, item)])
            except (ValueError, FileNotFoundError):
                continue
            roots.append((key, item))
            break
    try:
        selected = closure(roots)
        break
    except (ValueError, FileNotFoundError) as error:
        print("[DEBUG]", name, error)
if selected is None:
    raise SystemExit("No coherent retained Mesh compiler inputs are live")

target = generated / ("mesh-inputs-" + name.rsplit("-", 1)[-1])
if target.exists():
    raise SystemExit("A frozen set already exists: " + str(target))
with TemporaryDirectory(dir=generated) as working:
    staging = Path(working)
    for item in selected.values():
        for source in (item, item.with_suffix(".rmeta"), item.with_suffix(".rmeta.semio-deps")):
            if not source.is_file():
                continue
            before = source.stat()
            shutil.copy2(source, staging / source.name)
            after = source.stat()
            if (before.st_ino, before.st_size, before.st_mtime_ns) != (after.st_ino, after.st_size, after.st_mtime_ns):
                raise SystemExit("Compiler input changed while copying: " + source.name)
    os.replace(staging, target)
print(json.dumps({"path": str(target), "protocol": protocol, "artifacts": len(selected)}))
