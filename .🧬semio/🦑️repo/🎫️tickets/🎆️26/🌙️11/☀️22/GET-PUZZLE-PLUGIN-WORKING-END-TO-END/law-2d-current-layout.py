import json
from pathlib import Path
import struct
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
cache = generated / "law-storage-native/b/debug/build"
scratch = generated / "law-2d-current-layout"
links = scratch / "deps"
links.mkdir(parents=True, exist_ok=True)
artifacts = {}
native = set()
for path in cache.glob("*/*/out/*"):
    if path.suffix == ".a":
        native.add(path.parent)
    if not path.name.startswith("lib") or path.suffix not in (".rlib", ".so", ".rmeta"):
        continue
    link = links / path.name
    if not link.exists():
        link.symlink_to(path)
    if path.suffix == ".rmeta":
        continue
    crate = path.name[3:].rsplit("-", 1)[0]
    fingerprint = path.parent.parent / "fingerprint" / f"lib-{crate}"
    if fingerprint.exists():
        artifacts[fingerprint.read_text().strip()] = link

directory = cache / "semio-s-artifact-puzzle-2d-connect-handles-preparation"
fingerprint = max(directory.glob("*/fingerprint/test-lib-*.json"), key=lambda path: path.stat().st_mtime_ns)
metadata = json.loads(fingerprint.read_text())
source = ticket / "law-2d-current-layout.rs"
object_file = scratch / "layout.o"
rustc = root / ".cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc"
command = [rustc, source, "--edition=2021", "--crate-type=lib", "--emit=obj", "-Z", "threads=1", "-L", f"dependency={links}", "-o", object_file]
for _, alias, _, identity in metadata["deps"]:
    key = identity.to_bytes(8, "little").hex()
    if key in artifacts:
        library = artifacts[key]
        command.extend(("--extern", f"{alias}={library}"))
        metadata_file = library.with_suffix(".rmeta")
        if metadata_file.is_file():
            command.extend(("--extern", f"{alias}={metadata_file}"))
for path in sorted(native):
    command.extend(("-L", f"native={path}"))
print("[DEBUG] Compiling layout probe against the latest 2D preparation test fingerprint", flush=True)
status = subprocess.run(command, cwd=root).returncode
if status:
    raise SystemExit(status)
layout = scratch / "layout.bin"
subprocess.run(["objcopy", "--dump-section", f".rodata.LAW_LAYOUTS={layout}", object_file], check=True)
names = ("snapshot", "node", "edge", "nodes", "edges", "chunks", "UTF-8", "optional UTF-8", "String", "mutation")
words = tuple(value for (value,) in struct.iter_unpack("<Q", layout.read_bytes()))
for index, name in enumerate(names):
    print(name, words[3 * index:3 * index + 3])
