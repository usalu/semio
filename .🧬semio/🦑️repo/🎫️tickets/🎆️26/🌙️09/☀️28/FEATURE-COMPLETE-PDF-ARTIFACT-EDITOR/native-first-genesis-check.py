"""🧪 Link the real paid genesis and Original caller without rebuilding their packages."""
import hashlib
import os
from pathlib import Path
import re
import runpy
import sqlite3
import subprocess
import sys

import pymupdf

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
cache = Path(sys.argv[1]).resolve()
generated = ticket / "🗑️generated"
out = generated / "native-first-genesis"
artifacts = generated / "paged-jpeg-assets"
runpy.run_path(str(ticket / "native-first-genesis-stage.py"))
target = out / ("first-genesis-proof.exe" if os.name == "nt" else "first-genesis-proof")
channel = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
prefix = ["repo", "tools", "exec", "--light"]
subprocess.run(prefix + ["git", "--", "apply", "--check", str(out / "native-first-genesis.patch")], cwd=root, check=True)
arguments = ["--edition=2021", "--crate-name", "first_genesis_proof", "-O",
             str(ticket / "native-first-genesis-test.rs"), "-o", str(target)]
units = {
    "semio_os_vcs": "6f7cf5115505731f", "semio_os_spr": "f01b5bcdf33cda1b",
    "semio_framework_job": "597755e4dac8c197", "semio_framework_value": "27d057c5fd224439",
    "semio_framework_hash": "b96aa8103cede20f",
    "semio_framework_value_testing_allocator": "15c586bcb62e103d",
}
for name, unit in units.items():
    for lib in (next(cache.glob(f"*/{unit}/out/lib{name}-*.rmeta")), next(cache.glob(f"*/{unit}/out/lib{name}-*.rlib"))):
        arguments += ["--extern", f"{name}={lib}"]
directories = {artifact.parent for artifact in cache.glob("*/*/out/*")
               if artifact.suffix in (".rlib", ".rmeta", ".so", ".dylib", ".dll")}
arguments += [f"-Ldependency={directory}" for directory in sorted(directories)]
response = out / "arguments"
response.write_text("\n".join(arguments) + "\n", encoding="utf8")
subprocess.run(prefix + ["rustup", "--", "run", channel, "rustc", "@" + str(response)], cwd=root, check=True)
capture = out / "first.semio"
pack = artifacts / "large.dct-placement.pack"
result = subprocess.run([target, pack, capture], cwd=root, check=True, text=True, stdout=subprocess.PIPE)
print(result.stdout, end="")


def field(source, offset):
    length = 0
    bits = 0
    while True:
        byte = source[offset]
        offset += 1
        length |= (byte & 127) << bits
        bits += 7
        if byte < 128:
            return source[offset:offset + length], offset + length


encoded = capture.read_bytes()
assert encoded[0] == 1
source_pack, end = field(encoded, 1)
spr, end = field(encoded, end)
assert source_pack == pack.read_bytes() and spr and encoded[end:] == b"\0"
assert hashlib.sha256(encoded).hexdigest() in result.stdout
with sqlite3.connect(":memory:") as database:
    database.execute("create table genesis(id text primary key, archive blob)")
    database.execute("insert into genesis values (?, ?)", ("pdf-document", encoded))
    assert database.execute("select archive from genesis where id=?", ("pdf-document",)).fetchone()[0] == encoded
with pymupdf.open(artifacts / "large.stream.segmented.pdf") as pdf:
    assert not pdf.is_repaired and pdf.page_count == 1
    assert pdf[0].get_images()
print("Independent archive framing, SHA-256 and SQLite readback agree; MuPDF opened the companion PDF")
