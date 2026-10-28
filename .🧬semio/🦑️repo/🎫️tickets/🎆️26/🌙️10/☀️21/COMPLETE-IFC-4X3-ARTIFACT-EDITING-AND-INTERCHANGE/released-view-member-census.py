"""📦️ Authenticate bounded primary archive members by header, length and CRC without copying repository source."""
import hashlib
import json
import struct
import zlib
from pathlib import Path
import importlib.util

ticket = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("release_archive_oracle", ticket / "released-view-zip-index.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
index = module.result
if not index["completeArchiveIndex"]:
    raise ValueError("complete primary archive index is unavailable")

def member(name: str) -> bytes:
    rows = [row for row in index["rows"] if row["name"] == name]
    if len(rows) != 1:
        raise ValueError("primary member identity is ambiguous")
    row = rows[0]
    if row["bytes"] > 2 * 1048576 or row["compressedBytes"] > 2 * 1048576 or row["flags"] & 1:
        raise ValueError("primary member exceeds explicit admission")
    header = struct.unpack("<I5H3I2H", module.oracle.range(row["localOffset"], 30))
    if header[0] != 0x04034b50 or header[3] != row["method"]:
        raise ValueError("primary local header differs")
    data = module.oracle.range(row["localOffset"] + 30, header[9] + header[10] + row["compressedBytes"])
    if data[:header[9]].decode("utf-8" if row["flags"] & 2048 else "cp437") != name:
        raise ValueError("primary local member name differs")
    compressed = data[header[9]+header[10]:]
    if row["method"] == 8:
        decoder = zlib.decompressobj(-15)
        data = decoder.decompress(compressed, row["bytes"] + 1)
        if not decoder.eof or decoder.unused_data or decoder.unconsumed_tail:
            raise ValueError("primary compressed stream differs")
    elif row["method"] == 0:
        data = compressed
    else:
        raise ValueError("primary compression method is unsupported")
    if len(data) != row["bytes"] or zlib.crc32(data) != row["crc32"]:
        raise ValueError("primary member length or CRC differs")
    return data

rows = []
for name in ["IFC4_3/HTML/IFC4X3_ADD2.exp", "IFC4_3/index.html"]:
    data = member(name)
    row = dict(name=name, bytes=len(data), sha256=hashlib.sha256(data).hexdigest(), crcVerified=True)
    if name.endswith(".exp"):
        root = ticket.parents[6]
        source = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/IFC4X3_ADD2.exp"
        row["equalsCurrentPinnedExpress"] = data == source.read_bytes()
    rows.append(row)
output = dict(completeViewRequirementSet=False, copiedSourceFiles=False, rows=rows)
(ticket / "🗑️generated/released-view-members.json").write_text(json.dumps(output, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps(output))
