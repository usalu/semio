"""🔍 Small-leaved lime author: silhouettes of two scales of the kit tree laid over each other in elevation, front and plan (grey both, red only the coarser, blue only the finer), as a ruler for the cross-scale agreement.

@see ./rework/assets/vegetation--small-leaved-lime.md
"""
import struct
import zlib

CELL = 0.05
VIEWS = {"elevation": (0, 1), "front": (2, 1), "plan": (0, 2)}


def raster(meshes, axes, lo, size):
    grid = [bytearray(size[0]) for _ in range(size[1])]
    a, b = axes
    for mesh in meshes:
        pts = [((p[a] - lo[0]) / CELL, (p[b] - lo[1]) / CELL) for p in mesh.v]
        for i, j, k in mesh.f:
            (x0, y0), (x1, y1), (x2, y2) = pts[i], pts[j], pts[k]
            det = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0)
            if abs(det) < 1e-12:
                continue
            for y in range(max(0, int(min(y0, y1, y2))), min(size[1] - 1, int(max(y0, y1, y2))) + 1):
                row = grid[y]
                for x in range(max(0, int(min(x0, x1, x2))), min(size[0] - 1, int(max(x0, x1, x2))) + 1):
                    px, py = x + 0.5, y + 0.5
                    u = ((px - x0) * (y2 - y0) - (x2 - x0) * (py - y0)) / det
                    v = ((x1 - x0) * (py - y0) - (px - x0) * (y1 - y0)) / det
                    if u >= -0.02 and v >= -0.02 and u + v <= 1.02:
                        row[x] = 1
    return grid


def png(path, rows, width):
    raw = b"".join(b"\x00" + bytes(row) for row in rows)
    chunk = lambda tag, data: struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
    with open(path, "wb") as f:
        f.write(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, len(rows), 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw, 6)) + chunk(b"IEND", b""))


def compare(tree, fine, coarse, out):
    built = {50: tree.build(50)}
    pts = [p for s in built[50][0] + built[50][1] for p in s["mesh"].v]
    box = (tuple(min(p[k] for p in pts) for k in range(3)), tuple(max(p[k] for p in pts) for k in range(3)))
    for scale in (fine, coarse):
        if scale not in built:
            built[scale] = tree.build(scale, box=box)
    strips = []
    for name, axes in VIEWS.items():
        lo = (box[0][axes[0]] - 0.5, box[0][axes[1]] - 0.5)
        size = (int((box[1][axes[0]] - box[0][axes[0]] + 1.0) / CELL) + 1, int((box[1][axes[1]] - box[0][axes[1]] + 1.0) / CELL) + 1)
        g = {s: raster([x["mesh"] for x in built[s][0] + built[s][1]], axes, lo, size) for s in (fine, coarse)}
        both = only_c = only_f = 0
        rows = []
        for y in range(size[1] - 1, -1, -1):
            row = bytearray()
            for x in range(size[0]):
                f, c = g[fine][y][x], g[coarse][y][x]
                both += f & c
                only_c += c & ~f & 1
                only_f += f & ~c & 1
                row += b"\xb0\xb0\xb0" if f and c else b"\xe0\x20\x20" if c else b"\x20\x40\xe0" if f else b"\xff\xff\xff"
            rows.append(row)
        print(f"[DEBUG] 1:{coarse} against 1:{fine} {name}: IoU {both / max(1, both + only_c + only_f):.3f}, only coarse {only_c * CELL * CELL:.2f} m2, only fine {only_f * CELL * CELL:.2f} m2")
        strips.append((rows, size[0]))
    height = max(len(r) for r, _ in strips)
    width = sum(w for _, w in strips)
    rows = []
    for y in range(height):
        row = bytearray()
        for strip, w in strips:
            row += strip[y] if y < len(strip) else b"\xff" * (3 * w)
        rows.append(row)
    png(out, rows, width)
    print(out)
