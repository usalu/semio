"""🧪 Drive the exact paid egress into the staged native filesystem append."""
import ast
import hashlib
import os
from pathlib import Path
import re
import runpy
import sqlite3
import struct
import subprocess
import sys
import tempfile

import pymupdf

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
cache = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else None
generated = ticket / "🗑️generated"
out = generated / "native-first-archive-disk"
artifacts = generated / "paged-jpeg-assets"
runpy.run_path(str(ticket / "native-first-archive-disk-stage.py"))
prefix = ["repo", "tools", "exec", "--light"]
subprocess.run(prefix + ["git", "--", "apply", "--check",
     str(generated / "native-archive-readback/native-archive-readback.patch"),
     str(generated / "native-first-genesis/native-first-genesis.patch"), str(out / "native-first-archive-disk.patch")],
     cwd=root, check=True)


def extract(source, marker, close="\n}"):
    start = source.index(marker)
    return source[start:source.index(close, start) + len(close)]


def syntax_only(text, native_serde):
    text = re.sub(r"^//!.*$|^#\[value\(.*\)\]$", "", text, flags=re.MULTILINE)
    text = re.sub(r",?\s*semio_framework_value(?:_derive)?::\w+", "", text)
    if not native_serde:
        text = re.sub(r"^ *#\[serde\(.*\)\]$", "", text, flags=re.MULTILINE)
        text = re.sub(r",?\s*serde::\w+", "", text)
    return text


live = next(root.glob("🧰*/🛍*/💻*/🔨*/🏪*/🔄*/🖥*/📥*/🔄*/🦀*.rs")).read_text()
scan_output = generated / "native-archive-readback"
live_methods = [
    extract(live, change, "\n    }") for change in (
        "    pub fn new(folder:", "    pub fn event_path(", "    fn checksum(", "    fn encode_text(",
        "    fn decode_event(", "    fn refuse_repository_root(", "    fn events_from(",
    )
]
directory_sync = live[live.index("    #[cfg(unix)]\n    pub(crate) fn sync_directory"):live.index("    /// 🩹")]
store = "\n".join([
    (scan_output / "records.rs").read_text().split("pub const DOCUMENT_ARCHIVE_MAXIMUM_BYTES")[0],
    extract(live, "pub struct FolderEventLogStorage {"), extract(live, "struct FolderEvent {"),
    extract(live, "struct FolderEventReader<'a> {"), extract(live, "impl<'a> FolderEventReader<'a> {"),
    extract(live, "fn folder_writer(folder:"), "impl FolderEventLogStorage {",
    *live_methods, directory_sync, "}",
])
protocol_cache = ast.parse((ticket / "native-first-genesis-check.py").read_text())
units = next(ast.literal_eval(node.value) for node in protocol_cache.body if isinstance(node, ast.Assign)
             and any(isinstance(target, ast.Name) and target.id == "units" for target in node.targets))
linked = bool(cache and all(next(cache.glob(f"*/{unit}/out/lib{name}-*.rlib"), None) for name, unit in units.items()))
serde_units = {"serde": "a53fc3d225d557b5", "serde_json": "7a481ed6935139d3"}
native_serde = linked and all(next(cache.glob(f"*/{unit}/out/lib{name}-*.rlib"), None) for name, unit in serde_units.items())
if native_serde:
    units.update(serde_units)
if not linked:
    hash_lib = next((root / ".cache/b/r/m/u/l").glob("*/prototype/build/*/*/out/libsemio_framework_hash-*.rlib"))
    cache = hash_lib.parents[3]
    units = {"semio_framework_hash": hash_lib.parents[1].name}
receipt = (ticket / "native-first-archive-receipt.rs").read_text()
if not native_serde:
    receipt = receipt.split("\nmod u64_decimal", 1)[0]
receipt = syntax_only(receipt, native_serde).replace("use protocol::wire::", "use crate::protocol::wire::")
sources = {
    "folder": store,
    "receipt": receipt,
    "exchange": (ticket / "native-first-archive-exchange.rs").read_text().split("\n", 1)[1].replace("use protocol::wire::", "use crate::protocol::wire::"),
    "natural_wire": syntax_only((ticket / "natural-pdf-wire-records.rs").read_text(), native_serde),
    "readback": (scan_output / "readback.rs").read_text(),
}
for label in ("scoped", "append"):
    name = "native-first-archive-" + ("disk" if label == "append" else label) + ".rs"
    sources[label] = (ticket / name).read_text().split("\n", 1)[1].replace("use semio_os_spr::channel::", "use crate::proof_wire::")
sources["drive"] = (ticket / "native-first-archive-drive.rs").read_text().split("\n", 1)[1].replace(
    "use semio_framework::kernel::", "use crate::protocol::wire::command_ingress::").replace(
    "use protocol::channel::", "use crate::proof_wire::").replace(
    "use protocol::AppDocumentIdentity;", "use crate::AppDocumentIdentity;").replace(
    "use store_sync::sync::FolderEventLogStorage;", "use crate::FolderEventLogStorage;")

toolchain = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
with tempfile.TemporaryDirectory(dir=out) as stage:
    stage = Path(stage)
    env = os.environ.copy()
    for label, code in sources.items():
        path = stage / (label + ".rs")
        path.write_text(code)
        env[f"SEMIO_DISK_{label.upper()}"] = str(path)
    binary = stage / ("first-archive-disk.exe" if os.name == "nt" else "first-archive-disk")
    test = "native-first-archive-disk-test.rs" if linked else "native-first-archive-sink-test.rs"
    arguments = ["--edition=2021", "--crate-name", "first_archive_disk", "-O",
                 str(ticket / test), "-o", str(binary)]
    if native_serde:
        arguments += ["--cfg", "archive_wire_serde"]
    for name, unit in units.items():
        for extension in ("rmeta", "rlib"):
            library = next(cache.glob(f"*/{unit}/out/lib{name}-*.{extension}"))
            arguments += ["--extern", f"{name}={library}"]
    directories = {file.parent for file in cache.glob("*/*/out/*")
                   if file.suffix in (".rlib", ".rmeta", ".so", ".dylib", ".dll")}
    arguments += [f"-Ldependency={directory}" for directory in sorted(directories)]
    response = stage / "args"
    response.write_text("\n".join(arguments) + "\n", encoding="utf8")
    subprocess.run(prefix + ["rustup", "--", "run", toolchain, "rustc", "@" + str(response)], cwd=root, env=env, check=True)
    workspace = stage / "store"
    pack = artifacts / "large.dct-placement.pack"
    source = pack
    if not linked:
        def varint(number):
            result = bytearray()
            while number >= 128:
                result.append((number & 127) | 128)
                number >>= 7
            return bytes(result) + bytes([number])
        source = stage / "first-genesis.semio"
        first_spr_fixture = b"schema=stdio.pdf.1.7\ndocument=pdf-document"
        source.write_bytes(b"\x01" + varint(pack.stat().st_size) + pack.read_bytes()
                           + varint(len(first_spr_fixture)) + first_spr_fixture + b"\0")
        print("Using independently framed genesis pages for the native disk proof", flush=True)
    result = subprocess.run([binary, source, workspace], cwd=root, check=True, text=True, stdout=subprocess.PIPE)
    print(result.stdout, end="")
    record = (workspace / ".semio/events.semio").read_bytes()
    assert record[:8] == b"SEMIOEL1"
    size, checksum = struct.unpack_from("<QQ", record, 8)
    payload = record[24:]
    assert size == len(payload) and payload[0] == 4
    calculated = 14695981039346656037
    for byte in payload:
        calculated = ((calculated ^ byte) * 1099511628211) % (1 << 64)
    assert calculated == checksum
    cursor = 9
    document_len, = struct.unpack_from("<I", payload, cursor)
    cursor += 4
    assert payload[cursor:cursor + document_len] == b"pdf-document"
    cursor += document_len
    schema_len, = struct.unpack_from("<I", payload, cursor)
    cursor += 4
    assert payload[cursor:cursor + schema_len] == b"s.stdio.pdf@1.7"
    cursor += schema_len
    extent, = struct.unpack_from("<Q", payload, cursor)
    cursor += 8
    archive = payload[cursor:cursor + extent]
    assert payload[cursor + extent:] == bytes(8)
    assert hashlib.sha256(archive).hexdigest() in result.stdout
    assert archive[0] == 1
    pack_len = 0
    cursor = 1
    while True:
        byte = archive[cursor]
        pack_len |= (byte & 127) << (7 * (cursor - 1))
        cursor += 1
        if byte < 128: break
    assert archive[cursor:cursor + pack_len] == pack.read_bytes()
    with sqlite3.connect(":memory:") as database:
        database.execute("create table document(id text primary key, initial_archive blob)")
        database.execute("insert into document values(?,?)", ("pdf-document", archive))
        assert database.execute("select initial_archive from document where id=?", ("pdf-document",)).fetchone()[0] == archive
    with pymupdf.open(artifacts / "native-graph.stream.pdf") as pdf:
        assert not pdf.is_repaired and pdf.page_count == 1 and pdf[0].get_images()
    print("Independent folder checksum, canonical Pack, SHA-256, SQLite and MuPDF checks passed")
