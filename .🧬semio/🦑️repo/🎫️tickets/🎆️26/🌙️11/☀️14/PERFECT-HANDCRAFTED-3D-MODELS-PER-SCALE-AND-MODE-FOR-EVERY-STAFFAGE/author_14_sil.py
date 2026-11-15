"""Queries on the dumped drawing silhouettes (author-14-dump.ts): runs of the filled drawing at a station of each view."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, "🗑️generated", "author-14")


class View:
    def __init__(self, d):
        self.minX, self.minY, self.cell, self.rows = d["minX"], d["minY"], d["cell"], d["rows"]
        self.h = len(self.rows)

    def col(self, x):
        """Vertical runs [(lo, hi)] of the silhouette at paper-x `x`."""
        out = []
        j = 0
        for j, row in enumerate(self.rows):
            for a, b in row:
                if a <= x < b:
                    out.append(j)
        if not out:
            return []
        runs = []
        s = p = out[0]
        for k in out[1:]:
            if k != p + 1:
                runs.append((self.minY + s * self.cell, self.minY + (p + 1) * self.cell))
                s = k
            p = k
        runs.append((self.minY + s * self.cell, self.minY + (p + 1) * self.cell))
        return runs

    def row(self, y):
        """Horizontal runs [(lo, hi)] at paper-y `y`."""
        j = int((y - self.minY) / self.cell)
        if j < 0 or j >= self.h:
            return []
        return [tuple(r) for r in self.rows[j]]

    def extent(self):
        xs = [r[0] for row in self.rows for r in row] + [r[1] for row in self.rows for r in row]
        ys = [self.minY + j * self.cell for j, row in enumerate(self.rows) if row]
        return min(xs), max(xs), min(ys), max(ys) + self.cell


class Dump:
    def __init__(self, slug):
        d = json.load(open(os.path.join(GEN, slug + ".dump.json")))
        self.dimensions = d["dimensions"]
        self.frames = d["frames"]
        self.scales = {int(k): {v: View(x) for v, x in views.items()} for k, views in d["scales"].items()}

    def view(self, scale, name):
        return self.scales[scale][name]


if __name__ == "__main__":
    slug, scale, view = sys.argv[1], int(sys.argv[2]), sys.argv[3]
    mode = sys.argv[4]
    D = Dump(slug).view(scale, view)
    vals = [float(a) for a in sys.argv[5:]]
    print("extent", [round(v, 3) for v in D.extent()])
    for v in vals:
        print(v, [(round(a, 3), round(b, 3)) for a, b in (D.col(v) if mode == "col" else D.row(v))])
