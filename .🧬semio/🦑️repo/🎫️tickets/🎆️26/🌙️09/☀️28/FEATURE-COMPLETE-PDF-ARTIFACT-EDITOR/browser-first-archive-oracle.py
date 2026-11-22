"""🗃️ Check the Browser journal with independent physical decoding and SQLite readback."""
import hashlib
import json
from pathlib import Path
import sqlite3
import struct

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
fixture = json.loads((ticket / "browser-first-archive-cases.json").read_text())
output = generated / "browser-first-archive/law"
result = json.loads((output / "law.json").read_text())
pack = (generated / "paged-jpeg-assets/large.dct-placement.pack").read_bytes()
spr = f"schema={fixture['schema']}\ndocument={fixture['document']}".encode()

def records(path):
    raw = path.read_bytes()
    at = 0
    while at < len(raw):
        magic, extent, expected = struct.unpack_from("<8sQQ", raw, at)
        assert magic == b"SEMIOEL1"
        body = memoryview(raw)[at + 24:at + 24 + extent]
        assert len(body) == extent
        fnv = 0xcbf29ce484222325
        for octet in body:
            fnv = ((fnv ^ octet) * 0x100000001b3) % (1 << 64)
        assert fnv == expected, "physical FNV64 differs"
        kind, timestamp = struct.unpack_from("<BQ", body)
        assert kind == 4 and timestamp > 0
        cursor = 9
        def field(code):
            nonlocal cursor
            width = struct.calcsize(code)
            length, = struct.unpack_from(code, body, cursor)
            cursor += width
            result = bytes(body[cursor:cursor + length])
            assert len(result) == length
            cursor += length
            return result
        key, schema, primary, secondary = field("<I").decode(), field("<I").decode(), field("<Q"), field("<Q")
        assert not secondary and cursor == extent
        yield key, schema, primary
        at += 24 + extent
    assert at == len(raw)

def archive_fields(archive):
    assert archive[0] == 1
    at = 1
    def length():
        nonlocal at
        value, shift = 0, 0
        while True:
            octet = archive[at]
            at += 1
            value |= (octet % 128) << shift
            if octet < 128:
                assert not shift or octet != 0
                return value
            shift += 7
    fields = []
    for _ in range(2):
        size = length()
        fields.append(archive[at:at + size])
        at += size
    assert length() == 0 and at == len(archive)
    return tuple(fields)

database = sqlite3.connect(":memory:")
database.executescript("CREATE TABLE events(key TEXT, schema TEXT, archive BLOB);")
database.executemany("INSERT INTO events VALUES (?, ?, ?)", records(output / "first/.semio/events.semio"))
assert database.execute("SELECT key, schema, length(archive) FROM events ORDER BY rowid").fetchall() == [
    ("neighbor", fixture["schema"], 6),
    (fixture["document"], fixture["schema"], fixture["extent"]),
]
archive, = database.execute("SELECT archive FROM events WHERE key = ?", (fixture["document"],)).fetchone()
assert archive_fields(archive) == (pack, spr)
assert hashlib.sha256(archive).hexdigest() == result["sha256"]
assert [key for key, _, _ in records(output / "rollback/.semio/events.semio")] == ["neighbor"]
try:
    list(records(output / "corrupt/.semio/events.semio"))
except AssertionError as error:
    assert str(error) == "physical FNV64 differs"
else:
    raise AssertionError("corrupt interior record passed the independent oracle")
assert all(result[name] == value for name, value in fixture["cases"].items())
assert result["maximumRead"] == result["maximumWrite"] == fixture["page"]
wire_output = generated / "browser-first-archive"
key_hash = hashlib.sha256(fixture["document"].encode()).hexdigest()
for name, path, count in [
    ("native", wire_output / "native-law/first/.semio/events.semio", 2),
    ("HTTP", wire_output / f"http-law/os/local-documents/{key_hash}/.semio/events.semio", 1),
]:
    database.execute("DELETE FROM events")
    database.executemany("INSERT INTO events VALUES (?, ?, ?)", records(path))
    assert database.execute("SELECT COUNT(*) FROM events").fetchone() == (count,)
    assert database.execute("SELECT archive FROM events WHERE key = ? AND schema = ?", (fixture["document"], fixture["schema"])).fetchone() == (archive,)
    print(f"[DEBUG] {name} companion record matches the independently parsed archive.")
print(f"[DEBUG] Python verified physical FNV64, archive framing, SHA-256 and rollback. SQLite returned the exact {len(archive)}-byte archive.")
