"""🧪 Drive the live folder scanner and scoped page wire into the original paged genesis owner."""
import hashlib
import json
from pathlib import Path
import runpy
import struct
import re
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
STAGE = runpy.run_path(str(TICKET / "native-folder-reopen-stage.py"))
OUT, ROOT = (STAGE[name] for name in ("out", "root"))
PREFIX = ["repo", "tools", "exec", "--light"]
subprocess.run(PREFIX + ["git", "--", "apply", "--check", str(OUT / "native-folder-source.patch")], cwd=ROOT, check=True)
CACHE = Path(sys.argv[1]).resolve()
if not (OUT / "expected.pack").is_file() or not (OUT / "expected.spr").is_file():
    subprocess.run(PREFIX + ["python", "--", str(TICKET / "large-reopen-check.py"), str(CACHE)], cwd=ROOT, check=True)
src = (OUT / "probe.rs").read_text()
src = src.replace("input: &Path, extent:", "mut file: impl Read, extent:")
src = src.replace("    let mut file = File::open(input).unwrap();\n", "")
src = src.replace("heap <= DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES", 'heap <= DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES + if mode == "journal" { 64 } else { 0 }')
src = src.replace("    let (ready, heap) = run(Path::new(&args[1]), n(2), limits, digest, &args[7], Path::new(&args[8]));",
                  '''    let (ready, heap) = if args[7] == "journal" {
        journal(&args, n(2), limits, digest)
    } else {
        run(File::open(&args[1]).unwrap(), n(2), limits, digest, &args[7], Path::new(&args[8]))
    };''')
start = src.index('    if args[1] == "prepare" {')
end = src.index("    let n =", start)
src = src[:start] + src[end:]
src += "\n" + (TICKET / "native-folder-reopen-test.rs").read_text()
target = OUT / ("journal.exe" if sys.platform == "win32" else "journal")
probe = OUT / "journal.rs"
probe.write_text(src)
names = ("semio_framework_value", "semio_framework_value_persistent", "semio_framework_hash",
         "semio_framework_value_testing_allocator", "semio_os_vcs", "pack")
exported = list(CACHE.glob("*/*/out/lib*.rlib"))
pack_lib, = [p for p in exported if p.name.startswith("libpack-")]
deps = dict(json.loads(pack_lib.with_suffix(".rmeta.semio-deps").read_text())["dependencies"])
value, = [p for p in exported if p.name.startswith("libsemio_framework_value-") and p.stem[3:] in deps]
deps.update(json.loads(value.with_suffix(".rmeta.semio-deps").read_text())["dependencies"])
arguments = ["--edition=2021", "-O", str(probe), "-o", str(target)]
for name in names:
    matches = [p for p in exported if p.name.startswith(f"lib{name}-")]
    chosen = [p for p in matches if p.stem[3:] in deps] or matches
    assert len(chosen) == 1, (name, chosen)
    arguments += ["--extern", f"{name}={chosen[0].with_suffix('.rmeta')}", "--extern", f"{name}={chosen[0]}"]
arguments += [f"-Ldependency={directory}" for directory in sorted({p.parent for p in CACHE.glob("*/*/out/lib*")})]
toolchain = re.search(r'^channel = "([^"]+)"', (ROOT / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
response = OUT / "journal.args"
response.write_text("\n".join(arguments) + "\n")
subprocess.run(PREFIX + ["rustup", "--", "run", toolchain, "rustc", "@" + str(response)], cwd=ROOT, check=True)


def frame(kind, doc, schema, archive):
    key = doc.encode()
    metadata = schema.encode()
    body = bytes([kind]) + struct.pack("<Q", 1)
    body += struct.pack("<I", len(key)) + key + struct.pack("<I", len(metadata)) + metadata
    body += struct.pack("<Q", len(archive)) + archive + struct.pack("<Q", 0)
    hash64 = 0xcbf29ce484222325
    for byte in body:
        hash64 = ((hash64 ^ byte) * 0x100000001b3) & ((1 << 64) - 1)
    return struct.pack("<8sQQ", b"SEMIOEL1", len(body), hash64) + body


fixture = json.loads((TICKET / "native-folder-reopen-law.json").read_text())
pack = (OUT / "expected.pack").read_bytes()
spr = (OUT / "expected.spr").read_bytes()


def length_bytes(number):
    result = []
    while number >= 128:
        result.append(number % 128 | 128)
        number >>= 7
    return bytes([*result, number])


payload = b"\1" + length_bytes(len(pack)) + pack + length_bytes(len(spr)) + spr + b"\0"
assert 4 << 20 < len(payload)

folder = OUT / "native-journal"
(folder / ".semio").mkdir(parents=True, exist_ok=True)
record_path = folder / ".semio/events.semio"
schema, document = fixture["schema"], fixture["document"]
record_path.write_bytes(frame(4, "other-archive", schema, b"version-one-placeholder"))
with record_path.open("ab") as file:
    file.write(frame(4, document, schema, payload))
superseding = OUT / "replacement-record"
superseding.write_bytes(frame(1, document, schema, b"new pair"))
args = (folder, len(payload), fixture["archiveMaximum"], fixture["packMaximum"], fixture["sprMaximum"],
        hashlib.sha256(payload).hexdigest(), "journal", OUT, superseding)
subprocess.run(["repo", "capacity", "run", "--memory=64", "--", str(target), *map(str, args)], cwd=ROOT, check=True)
for label in ("pack", "spr"):
    assert (OUT / f"actual.{label}").read_bytes() == (OUT / f"expected.{label}").read_bytes()
print(f"[DEBUG] native event log -> scoped page wire -> retained field owner -> same genesis Pack pages: {len(payload)} bytes")
