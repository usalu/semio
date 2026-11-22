"""🗃️ Independently decode the first-genesis folder archives a mounted open wrote (physical FNV64, framing, folder hash, SQLite)."""
import argparse
import hashlib
import json
from pathlib import Path
import sqlite3
import struct

MAXIMUM = 4 * 1024 * 1024


def records(path):
    """🧱 Yield (key, schema, archive length, archive) for each physically valid kind-4 record."""
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
            value = bytes(body[cursor:cursor + length])
            assert len(value) == length
            cursor += length
            return value

        key, schema, primary, secondary = field("<I").decode(), field("<I").decode(), field("<Q"), field("<Q")
        assert not secondary and cursor == extent
        yield key, schema, primary
        at += 24 + extent
    assert at == len(raw)


def frame(archive):
    """🔬 Check the memberless version-one archive framing: two length-prefixed fields then a zero length."""
    assert archive[0] == 1
    at = 1
    sizes = []
    while True:
        value, shift = 0, 0
        while True:
            octet = archive[at]
            at += 1
            value |= (octet % 128) << shift
            if octet < 128:
                break
            shift += 7
        if value == 0 and len(sizes) == 2:
            break
        sizes.append(value)
        at += value
        assert len(sizes) <= 2
    assert at == len(archive) and len(sizes) == 2


cli = argparse.ArgumentParser()
cli.add_argument("listing", type=Path)
listing = json.loads(cli.parse_args().listing.read_text(encoding="utf8"))
assert listing["added"], "no first-genesis archive was written"
database = sqlite3.connect(":memory:")
database.execute("CREATE TABLE events(key TEXT, schema TEXT, archive BLOB)")
for name in listing["added"]:
    path = Path(name)
    database.execute("DELETE FROM events")
    rows = list(records(path))
    for _, _, archive in rows:
        frame(archive)
    database.executemany("INSERT INTO events VALUES (?, ?, ?)", rows)
    (key, schema, size), = database.execute("SELECT key, schema, length(archive) FROM events").fetchall()
    assert path.parent.parent.name == hashlib.sha256(key.encode()).hexdigest(), "folder is not the SHA-256 of the document id"
    assert schema == "stdio.pdf.1.7" and 0 < size <= MAXIMUM
    print(f"[DEBUG] {path.parent.parent.name[:12]} holds one valid kind-4 record for {key}: {schema}, {size} archive bytes")
