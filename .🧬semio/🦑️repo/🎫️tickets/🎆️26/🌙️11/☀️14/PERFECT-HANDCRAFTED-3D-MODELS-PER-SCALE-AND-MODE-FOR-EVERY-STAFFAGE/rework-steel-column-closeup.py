"""🔬 Close-up candidates of components/steel-column: the shaft cut to 0.4 m at the base or the head so that `r3-sheet.ts --file` fills the frame with the plates, welds and anchors; montage of three lists by two ends per scale."""
import copy
import json
import subprocess
import sys
from pathlib import Path

import struct
import zlib

T = Path(__file__).parent
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/steel-column/🔣️.json")
SOURCE = T / "volumes/components/steel-column.json"
OUT = T / "🗑️generated/rework-steel-column"
BASE_WORDS = ("base", "stiffener", "anchor", "footing", "grout")
HEAD_WORDS = ("cap", "pad", "bearing")


def read_png(path):
    data = Path(path).read_bytes()
    position, chunks, header = 8, [], None
    while position < len(data):
        length, kind = struct.unpack(">I4s", data[position:position + 8])
        body = data[position + 8:position + 8 + length]
        if kind == b"IHDR":
            header = struct.unpack(">IIBBBBB", body)
        if kind == b"IDAT":
            chunks.append(body)
        position += 12 + length
    width, height, depth, colour = header[:4]
    channels = {2: 3, 6: 4, 0: 1}[colour]
    raw = zlib.decompress(b"".join(chunks))
    stride = width * channels
    rows, previous = [], bytearray(stride)
    for y in range(height):
        start = y * (stride + 1)
        kind = raw[start]
        row = bytearray(raw[start + 1:start + 1 + stride])
        if kind == 1:
            for i in range(channels, stride):
                row[i] = (row[i] + row[i - channels]) & 255
        elif kind == 2:
            for i in range(stride):
                row[i] = (row[i] + previous[i]) & 255
        elif kind == 3:
            for i in range(stride):
                left = row[i - channels] if i >= channels else 0
                row[i] = (row[i] + ((left + previous[i]) >> 1)) & 255
        elif kind == 4:
            for i in range(stride):
                a = row[i - channels] if i >= channels else 0
                b = previous[i]
                c = previous[i - channels] if i >= channels else 0
                e = a + b - c
                pa, pb, pc = abs(e - a), abs(e - b), abs(e - c)
                row[i] = (row[i] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(row)
        previous = row
    return width, height, channels, rows


def half_rgb(path):
    width, height, channels, rows = read_png(path)
    out = []
    for y in range(0, height, 2):
        row = rows[y]
        line = bytearray()
        for x in range(0, width, 2):
            line += row[x * channels:x * channels + 3] if channels >= 3 else bytes([row[x]] * 3)
        out.append(line)
    return out


def write_png(path, rows):
    width, height = len(rows[0]) // 3, len(rows)
    raw = b"".join(b"\x00" + bytes(row) for row in rows)
    def chunk(kind, body):
        return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body))
    Path(path).write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw, 6)) + chunk(b"IEND", b""))


def end_of(solid, by_id):
    target = by_id.get(solid.get("of"), solid)
    name = solid["id"] + " " + target["id"]
    if "mass" in name:
        return "none"
    if any(word in name for word in BASE_WORDS):
        return "base"
    if any(word in name for word in HEAD_WORDS):
        return "head"
    return "both"


def candidate(document, scale, end):
    result = copy.deepcopy(document)
    volume = result["asset"]["volumes"][scale]
    by_id = {solid["id"]: solid for solid in volume["body"]}
    for name in ("body", "thermal", "structure"):
        kept = []
        for solid in volume[name]:
            where = end_of(solid, by_id)
            if where not in (end, "both"):
                continue
            if where == "both" and "to" in solid and "from" in solid:
                if end == "base":
                    solid["to"] = 0.4
                else:
                    solid["from"] = 2.6
            kept.append(solid)
        volume[name] = kept
    return result


def main():
    bun = sys.argv[1]
    scales = sys.argv[2].split(",")
    views = sys.argv[3] if len(sys.argv) > 3 else "three-quarter"
    extra = sys.argv[4:]
    document = json.loads(CATALOG.read_text())
    document["asset"]["volumes"] = json.loads(SOURCE.read_text())["volumes"]
    OUT.mkdir(parents=True, exist_ok=True)
    for scale in scales:
        tiles = []
        for name in ("body", "thermal", "structure"):
            for end in ("base", "head"):
                path = OUT / "closeup.json"
                path.write_text(json.dumps(candidate(document, scale, end), ensure_ascii=False))
                run = subprocess.run([bun, "r3-sheet.ts", "components/steel-column", "--scale", scale, "--list", name, "--file", str(path), "--only", views, "--no-rig", "--no-labels", *extra], cwd=T, capture_output=True, text=True)
                png = [line for line in run.stdout.splitlines() if line.endswith(".png")]
                if not png:
                    print("[DEBUG] ", run.stdout[-400:], run.stderr[-400:])
                    continue
                tiles.append(half_rgb(png[-1]))
                Path(png[-1]).unlink()
        rows = []
        for index in range(0, len(tiles), 2):
            pair = tiles[index:index + 2]
            for y in range(len(pair[0])):
                rows.append(b"".join(bytes(tile[y]) for tile in pair))
        suffix = "".join(extra).replace("--", "-")
        target = OUT / f"closeup-s{scale}-{views}{suffix}.png"
        write_png(target, rows)
        print(target)


if __name__ == "__main__":
    main()
