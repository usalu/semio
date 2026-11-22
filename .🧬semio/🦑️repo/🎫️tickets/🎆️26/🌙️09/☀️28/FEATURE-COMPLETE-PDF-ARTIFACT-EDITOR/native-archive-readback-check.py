"""🧪 Compare bounded Rust readback against SQLite and inspect the PDF payload with MuPDF."""
import json
import os
from pathlib import Path
import re
import runpy
import sqlite3
import struct
import subprocess

import pymupdf

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
runpy.run_path(str(ticket / "native-archive-readback-stage.py"))
generated = ticket / "🗑️generated"
out = generated / "native-archive-readback"
source = generated / "pdf-segmented-source.pdf"
settings = json.loads((ticket / "native-archive-readback.json").read_text())
source_expect = json.loads((ticket / "🔣️segmented-pdf-source.json").read_text())


def run(*args, **options):
    subprocess.run(["repo", "tools", "exec", "--light", *args], cwd=root, check=True, **options)


def varint(number):
    output = bytearray()
    while number > 127:
        output.append((number & 127) | 128)
        number >>= 7
    return bytes(output) + bytes([number])


def archive(payload):
    return b"\x01" + varint(len(payload)) + payload + b"\x00\x00"


def physical(kind, key, data=b"", metadata=b"pdf", secondary=b"", trailer=b""):
    if isinstance(key, str):
        key = key.encode("utf8")
    payload = b"".join((struct.pack("<BQI", kind, 1, len(key)), key, struct.pack("<I", len(metadata)),
                        metadata, struct.pack("<Q", len(data)), data,
                        struct.pack("<Q", len(secondary)), secondary, trailer))
    checksum = 14695981039346656037
    for byte in payload:
        checksum = ((checksum ^ byte) * 1099511628211) % (1 << 64)
    return b"SEMIOEL1" + struct.pack("<QQ", len(payload), checksum) + payload


with pymupdf.open(source) as original:
    assert not original.is_repaired and original.page_count == 1
    compact = original.tobytes(deflate=True)
    with pymupdf.open(stream=compact, filetype="pdf") as restored:
        assert not restored.is_repaired and restored.page_count == 1
        page = restored[0]
        _, _, width, height, *_ = page.get_images()[0]
        assert width == height == source_expect["imageSide"]
        picture = page.get_pixmap(colorspace=pymupdf.csGRAY)
        positions = [picture.width // 4, picture.width * 3 // 4]
        assert [picture.pixel(x, y)[0] for y in positions for x in positions] == source_expect["grayscale"]

expected = archive(compact)
assert len(expected) <= settings["archiveMaximum"] < source.stat().st_size
(out / "expected.semio").write_bytes(expected)
alternate_archive = archive((generated / "paged-jpeg-assets/large.dct-placement.pack").read_bytes())
assert alternate_archive != expected

document = "pdf-document"
newest = physical(4, document, expected, metadata=("a" * 4095 + "ä").encode())
other = physical(4, "unrelated", expected)
new_archive = physical(4, document, alternate_archive)
torn = new_archive[:-1]
unsealed = bytearray(new_archive)
unsealed[16] ^= 1
large_blob = physical(2, document, source.read_bytes())
files = {
    "exact-after-large-blob": (large_blob + newest, [(2, document, None), (4, document, expected)]),
    "different-latest-archive": (newest + new_archive, [(4, document, expected), (4, document, alternate_archive)]),
    "latest-pair": (newest + physical(1, document, b"pack", secondary=b"spr"), [(4, document, expected), (1, document, None)]),
    "other-document": (other, [(4, "unrelated", expected)]),
    "short-final-record": (newest + torn, [(4, document, expected)]),
    "unsealed-final-record": (newest + unsealed, [(4, document, expected)]),
    "corrupt-interior-record": (unsealed + newest, None),
    "invalid-utf": (physical(4, b"\xf0\x80\x80", expected), None),
    "trailing-payload": (physical(4, document, expected, trailer=b"extra"), None),
    "cancelled": (large_blob + newest, [(2, document, None), (4, document, expected)]),
}

with sqlite3.connect(":memory:") as database, (out / "cases.tsv").open("w") as expected_cases:
    database.execute("create table event(kind integer, document text, body blob)")
    for case in settings["cases"]:
        name, outcome = case["name"], case["result"]
        data, logical = files[name]
        if logical is not None:
            database.execute("delete from event")
            database.executemany("insert into event values (?, ?, ?)", logical)
            row = database.execute(
                "select kind, body from event where kind in (1,4) and document=? order by rowid desc limit 1",
                (document,),
            ).fetchone()
            if outcome != "cancelled":
                assert outcome == ("equal" if row == (4, expected) else "different")
        else:
            assert outcome == "invalid"
        (out / f"{name}.semio").write_bytes(data)
        expected_cases.write(f"{name}\t{outcome}\n")
print(f"SQLite verified exact latest archive and MuPDF reopened the packed {len(compact)}-byte PDF", flush=True)

run("git", "--", "apply", "--check", out / "native-archive-readback.patch")
wire_root = next(root.glob("🧰*/🔨*/📡*"))
wire = next(wire_root.glob("🧾*/🦀*.rs")).read_text()
codec = next(wire_root.glob("⚙*/🦀*.rs")).read_text()


def extract(text, head):
    start = text.index(head)
    return text[start:text.index("\n}", start) + 2]


primitives = "\n".join([extract(codec, "pub fn write_varint_u64("),
                         extract(wire, "pub fn write_str("), extract(wire, "pub fn write_bytes(")])
(out / "primitives.rs").write_text(primitives)
env = os.environ | {f"SEMIO_ARCHIVE_{name.upper()}": str(out / f"{name}.rs")
                    for name in ("records", "legacy", "preflight", "readback", "primitives")}
channel = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
test = out / ("archive-readback-check.exe" if os.name == "nt" else "archive-readback-check")
run("rustup", "--", "run", channel, "rustc", "--edition", "2024", str(ticket / "native-archive-readback-test.rs"),
    "-o", str(test), env=env)
subprocess.run([test, out, source], cwd=root, check=True)
