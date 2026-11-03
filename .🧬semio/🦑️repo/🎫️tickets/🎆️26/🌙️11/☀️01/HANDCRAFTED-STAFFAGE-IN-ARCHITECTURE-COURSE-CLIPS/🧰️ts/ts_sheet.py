"""ts_sheet.py: exports moderate-source thermal casts with and without thermalSpanK and lays before (top row) and after (bottom row) onto one sheet."""
import json
import re
import subprocess
import sys
from pathlib import Path

import fitz

HERE = Path(__file__).resolve().parent
G = HERE.parent / "🗑️generated" / "ts"
W = HERE.parents[7]
EXPORTER = W / "🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts"
CASES = {
    "laptop": ("laptop", {"pose": "open"}, "indoor-heated-winter"),
    "person": ("standing-adult", {"clip": "idle-breathing"}, "indoor-heated-winter"),
    "tealight": ("tealight", {"pose": "lit"}, "indoor-heated-winter"),
    "toaster": ("toaster", {"pose": "toasting"}, "indoor-heated-winter"),
    "seated": ("seated-office-worker", {"clip": "typing"}, "indoor-warm-summer"),
    "radiator": ("panel-radiator", {"pose": "rated"}, "indoor-heated-winter"),
}


def manifest():
    casts = {}
    for name, (asset, how, env) in CASES.items():
        base = {"asset": asset, **how, "view": "thermal", "environment": env, "theme": "dark"}
        casts[f"{name}-before"] = base
        casts[f"{name}-after"] = {**base, "thermalSpanK": 25}
    return {"version": 1, "casts": casts}


def export():
    G.mkdir(parents=True, exist_ok=True)
    path = G / "sheet-manifest.json"
    path.write_text(json.dumps(manifest(), indent=1), encoding="utf-8")
    subprocess.run(["repo", "tools", "exec", "bun", "--", str(EXPORTER), "--manifest", str(path), "--out", str(G / "cast"), "--lang", "en"], check=True, cwd=W)


def size_mm(svg):
    m = re.search(r'width="([\d.]+)mm" height="([\d.]+)mm"', svg)
    return float(m.group(1)), float(m.group(2))


def sheet():
    zoom, gap = 4.0, 14
    k = 72 / 25.4 * zoom
    names = list(CASES)
    cells = {}
    for n in names:
        for phase in ("before", "after"):
            svg = (G / "cast" / f"{n}-{phase}" / "0000.svg").read_text(encoding="utf-8")
            w, h = size_mm(svg)
            cells[n, phase] = (w * k, h * k)
    cw = [max(cells[n, "before"][0], cells[n, "after"][0]) for n in names]
    rh = [max(cells[n, p][1] for n in names) for p in ("before", "after")]
    doc = fitz.open()
    page = doc.new_page(width=sum(cw) + gap * (len(names) + 1), height=sum(rh) + gap * 3 + 40)
    page.draw_rect(page.rect, color=None, fill=(0.06, 0.08, 0.1))
    y = gap
    for p, hh in zip(("before", "after"), rh):
        x = gap
        for n, w in zip(names, cw):
            src = fitz.open(G / "cast" / f"{n}-{p}" / "0000.svg")
            pdf = fitz.open("pdf", src.convert_to_pdf())
            ww, h = cells[n, p]
            page.show_pdf_page(fitz.Rect(x, y, x + ww, y + h), pdf, 0)
            page.insert_text((x, y + hh + 14), f"{n} {p}", fontsize=10, color=(0.8, 0.8, 0.8))
            x += w + gap
        y += hh + gap + 20
    out = G / "sheet-span.png"
    page.get_pixmap(dpi=72).save(out)
    print(out)


if __name__ == "__main__":
    if "--skip-export" not in sys.argv:
        export()
    sheet()
