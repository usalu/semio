"""usage: python view.py castdir out.png width_px id[:frame] ...  (stacked vertically on dark paper)"""
import sys, os
import fitz
import numpy as np

cast, out, width = sys.argv[1], sys.argv[2], float(sys.argv[3])
tiles = []
for spec in sys.argv[4:]:
    cid, _, frame = spec.partition(":")
    f = os.path.join(cast, cid, f"{int(frame or 0):04d}.svg")
    page = fitz.open(f)[0]
    z = width / page.rect.width
    p = page.get_pixmap(matrix=fitz.Matrix(z, z), alpha=True)
    arr = np.frombuffer(p.samples, dtype=np.uint8).reshape(p.height, p.width, 4).astype(float)
    a = arr[:, :, 3:4] / 255.0
    rgb = arr[:, :, :3] * a + np.array([16, 24, 30], dtype=float) * (1 - a)
    tiles.append(rgb.astype(np.uint8))
W = max(t.shape[1] for t in tiles)
rows = []
for t in tiles:
    pad = np.full((t.shape[0] + 6, W, 3), (60, 70, 80), dtype=np.uint8)
    pad[6:, : t.shape[1]] = t
    rows.append(pad)
img = np.concatenate(rows, axis=0)
fitz.Pixmap(fitz.csRGB, img.shape[1], img.shape[0], img.tobytes(), False).save(out)
print(img.shape)
