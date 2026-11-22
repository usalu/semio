"""🧪 Cross-check version-1 archive receiving with independent framing, SHA-256 and SQLite."""
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import sqlite3
import subprocess
import sys
import zlib

TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if (parent / "Cargo.toml").exists())
STAGE = runpy.run_path(str(TICKET / "large-reopen-stage.py"))
OUT = STAGE["OUT"]
PREFIX = ["repo", "tools", "exec", "--light"]
subprocess.run(PREFIX + ["git", "--", "apply", "--check", str(OUT / "reopen.patch"), str(OUT / "snapshot.patch"), str(OUT / "hydration.patch")], cwd=ROOT, check=True)
CACHE = Path(sys.argv[1]).resolve()
names = (
    "semio_framework_value", "semio_framework_value_persistent", "semio_framework_hash",
    "semio_framework_value_testing_allocator", "semio_os_spr", "semio_os_vcs", "pack",
)
exported = list(CACHE.glob("*/*/out/lib*.rlib"))
dependencies = set()
for name in ["semio_os_spr", "pack", "semio_framework_value"]:
    modules = [p for p in exported if p.name.startswith(f"lib{name}-")]
    modules = [p for p in modules if p.stem[3:] in dependencies] or modules
    assert len(modules) == 1, modules
    description = json.loads(modules[0].with_suffix(".rmeta.semio-deps").read_text())
    dependencies.update(module for module, _ in description["dependencies"])
toolchain = re.search(r'^channel = "([^"]+)"', (ROOT / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
target = OUT / ("probe.exe" if os.name == "nt" else "probe")
arguments = ["--edition=2021", "-O", str(OUT / "probe.rs"), "-o", str(target)]
for name in names:
    matches = [p for p in exported if p.name.startswith(f"lib{name}-")]
    chosen = [p for p in matches if p.stem[3:] in dependencies] or matches
    assert len(chosen) == 1, (name, chosen)
    arguments += ["--extern", f"{name}={chosen[0].with_suffix('.rmeta')}", "--extern", f"{name}={chosen[0]}"]
arguments += [f"-Ldependency={directory}" for directory in sorted({p.parent for p in CACHE.glob("*/*/out/lib*")})]
response = OUT / "rustc.args"
response.write_text("\n".join(arguments) + "\n")
subprocess.run(PREFIX + ["rustup", "--", "run", toolchain, "rustc", "@" + str(response)], cwd=ROOT, check=True)


probe_source = OUT / "typed.rs"
probe_source.write_text("""
#![allow(dead_code)]
mod snapshot;
fn admit_paged<T: semio_framework_dsl_record::receiving::DslOwnedReceivingRecord + semio_framework_dsl_record::BorrowedDslRecord>(
    source: &semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>,
    native: &semio_framework_value::NativeDecodeControl<'_>,
    grant: semio_framework_value::RetainedCloneGrant,
) {
    let _ = snapshot::OriginalSnapshotRecordReceiving::<T>::admit(source, "stdio.pdf", protocol::PackLimits::default(), native, grant);
}
""")
typed_args = ["--edition=2021", "--crate-type", "lib", "--emit=metadata", str(probe_source), "-o", str(OUT / "typed.rmeta"), *arguments[5:]]
for name in ("semio_os_semio", "semio_framework_dsl_record", "semio_framework_value_native_owned", "protocol"):
    matches = [p for p in exported if p.name.startswith(f"lib{name}-")]
    chosen = [p for p in matches if p.stem[3:] in dependencies] or matches
    assert len(chosen) == 1, (name, chosen)
    typed_args += ["--extern", f"{name}={chosen[0].with_suffix('.rmeta')}"]
response = OUT / "typed.args"
response.write_text("\n".join(typed_args) + "\n")
subprocess.run(PREFIX + ["rustup", "--", "run", toolchain, "rustc", "@" + str(response)], cwd=ROOT, check=True)


def probe(*args, stdout=None):
    return subprocess.run(["repo", "capacity", "run", "--memory=64", "--", str(target), *map(str, args)],
                          cwd=ROOT, check=True, text=True, stdout=stdout)


def make_pdf():
    width, height = 1536, 1152
    pixels = hashlib.shake_256(b"semio.document.archive.reopen.rgb/v1").digest(width * height * 3)
    encoded = zlib.compress(pixels)
    assert len(encoded) >= len(pixels)
    assert hashlib.sha256(zlib.decompress(encoded)).digest() == hashlib.sha256(pixels).digest()

    def stream(headers, body):
        return b"<< " + headers + b" /Length " + str(len(body)).encode() + b" >>\nstream\n" + body + b"\nendstream"

    rows = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 768 576] /Resources << /XObject << /Im0 5 0 R >> >> /Contents 4 0 R >>",
        stream(b"", b"q 768 0 0 576 0 0 cm /Im0 Do Q\n"),
        stream(f"/Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /FlateDecode".encode(), encoded),
    ]
    pdf = bytearray(b"%PDF-1.7\n%\xf6\xe4\xfc\xdf\n")
    offsets = []
    for index, row in enumerate(rows, 1):
        offsets.append(len(pdf))
        pdf.extend(f"{index} 0 obj\n".encode() + row + b"\nendobj\n")
    xref = len(pdf)
    pdf.extend(b"xref\n0 6\n0000000000 65535 f \n")
    pdf.extend("".join(f"{offset:010d} 00000 n \n" for offset in offsets).encode())
    pdf.extend(f"trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode())
    return pdf


def varint(length):
    output = bytearray()
    while length >= 128:
        output.append((length & 127) | 128)
        length >>= 7
    output.append(length)
    return bytes(output)


def read_varint(data, at):
    value, count = 0, 0
    while at < len(data) and count < 10:
        byte = data[at]
        at += 1
        value += (byte & 127) << (7 * count)
        count += 1
        if byte < 128:
            assert count == len(varint(value))
            return value, at
    raise ValueError("truncated varint")


raw_pdf = OUT / "entropy.pdf"
raw_pdf.write_bytes(make_pdf())
pack_path, spr_path = OUT / "expected.pack", OUT / "expected.spr"
probe("prepare", raw_pdf, pack_path, spr_path)
pack, spr = pack_path.read_bytes(), spr_path.read_bytes()
fixture = json.loads((TICKET / "large-reopen-law.json").read_text())
archive = bytes([fixture["format"]]) + varint(len(pack)) + pack + varint(len(spr)) + spr + b"\0"
assert 4 * 1024 ** 2 < len(pack) < len(archive) < fixture["archiveMaximum"]
sha = hashlib.sha256(archive)
with sqlite3.connect(OUT / "folder.sqlite") as folder:
    folder.execute("create table if not exists archive (id integer primary key, value blob)")
    folder.execute("insert or replace into archive values (?, ?)", (1, archive))
with sqlite3.connect(OUT / "folder.sqlite") as folder:
    (readback,) = folder.execute("select value from archive where id = 1").fetchone()
assert hashlib.sha256(readback).digest() == sha.digest()
pack_size, pack_start = read_varint(readback, 1)
spr_size, spr_start = read_varint(readback, pack_start + pack_size)
assert readback[pack_start:pack_start + pack_size] == pack
assert readback[spr_start:spr_start + spr_size] == spr
assert readback[spr_start + spr_size:] == b"\0"
print(f"[DEBUG] encoded archive {len(archive)}, Pack {len(pack)}, SPR {len(spr)}, SHA-256 {sha.hexdigest()}", flush=True)

maximum = 0
for row in fixture["cases"]:
    maximum_archive = fixture["archiveMaximum"]
    maximum_pack = fixture["packMaximum"]
    maximum_spr = fixture["sprMaximum"]
    candidate = readback
    expected = None
    claimed_extent = None
    change = row["change"]
    if change == "exact":
        maximum_archive, maximum_pack, maximum_spr = len(candidate), len(pack), len(spr)
    elif change == "wrongDigest":
        expected = bytes(32)
    elif change == "truncated":
        candidate = candidate[:-1]
    elif change == "earlyEof":
        claimed_extent = len(candidate)
        candidate = candidate[:8192]
    elif change == "trailing":
        candidate += b"\0"
    elif change == "nonzeroMembers":
        candidate = candidate[:-1] + b"\1"
    elif change == "overlongLength":
        candidate = candidate[:pack_start - 1] + bytes([candidate[pack_start - 1] | 128]) + b"\0" + candidate[pack_start:]
    elif change == "overArchiveLimit":
        maximum_archive = len(candidate) - 1
    elif change == "overPackLimit":
        maximum_pack = len(pack) - 1
    elif change == "overSprLimit":
        maximum_spr = len(spr) - 1
    expected = expected or hashlib.sha256(candidate).digest()
    path = OUT / "archive.semio"
    path.write_bytes(candidate)
    result = probe(path, claimed_extent or len(candidate), maximum_archive, maximum_pack, maximum_spr, expected.hex(), change, OUT, stdout=subprocess.PIPE)
    outcome, heap = result.stdout.strip().split()
    maximum = max(maximum, int(heap))
    assert outcome == row["outcome"], row
    if outcome == "ready":
        assert (OUT / "actual.pack").read_bytes() == pack
        assert (OUT / "actual.spr").read_bytes() == spr
    print(f"[DEBUG] {row['name']}: {outcome}; heap request {heap}", flush=True)
assert maximum <= fixture["pageMaximum"]
