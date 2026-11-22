"""📏️ Fits `dimensions` (height, width, depth) of an asset to both guards at once: the body volume of every scale within one smallest feature of the dimension (catalog-silhouette) and every drawing of every scale within max(2 %, 15 mm) of it (validator `extent-*`). Each guard gives an interval per axis; the dimension is the middle of their intersection; when they do not meet it is the value inside the validator's interval that misses the volume guard least (reported as a defect of the drawings or volumes, never hidden).
python3 fit_dims.py <slug|file.json>… [--write]"""
import glob
import json
import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import extents as E  # noqa: E402

FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
AXES = {"heightM": (1, ["extent-height", "extent-front-height"]), "widthM": (0, ["extent-width", "extent-plan-width"]), "depthM": (2, ["extent-front-depth", "extent-plan-depth"])}


def fit(path):
    """📐️ Returns ({heightM, widthM, depthM}, [defect text]) for the asset file."""
    boxes = json.loads(subprocess.run([E.BUN, "volbox.ts", path], capture_output=True, text=True, cwd=os.path.dirname(os.path.abspath(__file__))).stdout.strip().splitlines()[-1])
    drawn = E.extents(path)
    dims, defects = {}, []
    for axis, (k, codes) in AXES.items():
        vol = [(boxes[s][k], FEATURE[s], f"volume 1:{s}") for s in boxes]
        dr = [(v, None, f"{c} 1:{s}") for c in codes for s, v in drawn.get(c, {}).items()]
        lo0, hi0 = min(v for v, _, _ in vol + dr) - 0.1, max(v for v, _, _ in vol + dr) + 0.1
        grid = [lo0 + i / 2000 for i in range(int(round((hi0 - lo0) * 2000)) + 1)]
        volume = lambda x: max(abs(v - x) - f for v, f, _ in vol)
        drawing = lambda x: max(abs(v - x) - max(0.02 * x, 0.015) for v, _, _ in dr)
        ok = [x for x in grid if volume(x) <= -0.0005 and drawing(x) <= -0.0005]
        if ok:
            dims[axis] = round((ok[0] + ok[-1]) / 2, 3)
            continue
        valid = [x for x in grid if drawing(x) <= -0.0005] or grid
        x = min(valid, key=lambda c: (round(volume(c), 6), abs(c - sum(v for v, _, _ in vol) / len(vol))))
        dims[axis] = round(x, 3)
        over = [f"{name} {v:.3f} m" for v, f, name in vol if abs(v - x) > f] + [f"{name} {v:.3f} m" for v, _, name in dr if abs(v - x) > max(0.02 * x, 0.015)]
        defects.append(f"{axis} {dims[axis]}: no value satisfies both guards; the drawings are kept within the validator and the volume guard is missed by {volume(x) * 1000:.1f} mm: {', '.join(over)}")
    return dims, defects


if __name__ == "__main__":
    write = "--write" in sys.argv
    for arg in [a for a in sys.argv[1:] if a != "--write"]:
        path = arg if arg.endswith(".json") else f"{E.CAT}/{arg}/🔣️.json"
        dims, defects = fit(path)
        doc = json.load(open(path, encoding="utf-8"))
        old = {k: doc["asset"]["dimensions"][k] for k in dims}
        print(doc["asset"]["id"], "was", old, "fit", dims, "DEFECT " + "; ".join(defects) if defects else "ok")
        if write and old != dims:
            text = open(path, encoding="utf-8").read()
            indent = 1 if text.startswith("{\n \"") else 2
            doc["asset"]["dimensions"].update(dims)
            tmp = path + ".tmp"
            with open(tmp, "w", encoding="utf-8") as f:
                json.dump(doc, f, ensure_ascii=False, indent=indent)
                f.write("\n")
            os.replace(tmp, path)
