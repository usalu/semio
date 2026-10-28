"""🗜️ Inventory a pinned primary release archive by bounded HTTP ranges without fetching content eagerly."""
import hashlib
import json
import struct
import urllib.request
from pathlib import Path
from typing import Protocol

class ArchiveOracle(Protocol):
    def range(self, start: int, length: int) -> bytes: ...

class HttpArchiveOracle:
    def __init__(self, url: str):
        self.url = url
    def range(self, start: int, length: int) -> bytes:
        if length > 8 * 1048576:
            raise ValueError("archive index range exceeds bounded admission")
        request = urllib.request.Request(self.url, headers={"Range": f"bytes={start}-{start+length-1}", "User-Agent": "semio-ifc-release-audit"})
        with urllib.request.urlopen(request, timeout=20) as response:
            if response.status != 206 or response.headers.get("Content-Range") != f"bytes {start}-{start+length-1}/{archive['bytes']}":
                raise ValueError("primary server did not admit the exact bounded archive range")
            data = response.read(length + 1)
        if len(data) != length:
            raise ValueError("archive index range length differs")
        return data

ticket = Path(__file__).resolve().parent
metadata = json.loads((ticket / "🗑️generated/released-view-resources.json").read_text(encoding="utf-8-sig"))
archive = next(row for row in metadata["observations"] if row["name"] == "IFC4.3.2.0.zip")
oracle: ArchiveOracle = HttpArchiveOracle(archive["url"])
result = dict(archive=archive, completeViewRequirementSet=False, completeArchiveIndex=False)
try:
    tail = oracle.range(archive["bytes"] - 65557, 65557)
    end = tail.rfind(b"PK\x05\x06")
    disk, central_disk, disk_count, count, size, offset, comment = struct.unpack_from("<HHHHIIH", tail, end + 4)
    if disk or central_disk or disk_count != count or count == 65535 or size == 0xffffffff or offset == 0xffffffff:
        raise ValueError("multi-disk or ZIP64 metadata requires an explicit next parser slice")
    central = oracle.range(offset, size)
    rows = []
    position = 0
    while position < len(central):
        if central[position:position+4] != b"PK\x01\x02":
            raise ValueError("invalid primary central directory signature")
        fields = struct.unpack_from("<6H3I5H2I", central, position + 4)
        flags, compressed, uncompressed, name_length, extra_length, comment_length = fields[2], fields[7], fields[8], fields[9], fields[10], fields[11]
        name = central[position+46:position+46+name_length].decode("utf-8" if flags & 2048 else "cp437")
        rows.append(dict(name=name, compressedBytes=compressed, bytes=uncompressed, localOffset=fields[-1], method=fields[3], crc32=fields[6], flags=flags))
        position += 46 + name_length + extra_length + comment_length
    if len(rows) != count:
        raise ValueError("primary directory count differs")
    candidates = [row for row in rows if row["name"].lower().endswith((".mvdxml", ".xml", ".json", ".csv", ".xmi", ".yaml", ".yml"))]
    result.update(completeArchiveIndex=True, entries=count, centralBytes=size, centralSha256=hashlib.sha256(central).hexdigest(), candidates=candidates, viewNames=[row for row in rows if any(word in row["name"].lower() for word in ("referenceview", "alignment-based", "mvd"))], rows=rows)
except Exception as error:
    result["error"] = dict(kind=type(error).__name__, message=str(error))
(ticket / "🗑️generated/released-view-zip-index.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps({key:value for key,value in result.items() if key not in ("rows", "candidates", "viewNames")}))
