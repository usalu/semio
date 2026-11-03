"""🎭 pa: cast manifest for the touched assets (thermal and dark-line, 1:50) and a PyMuPDF contact sheet - usage: casts.py manifest | sheet."""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent / "🗑️generated" / "pa" / "casts"
ASSETS = [
    ("hot-water-cylinder", "charged"),
    ("gas-boiler", "hot-water"),
    ("toaster", "toasting"),
    ("pendant-lamp", "on"),
    ("mvhr-unit", "normal"),
    ("oil-pumpjack", "mid-stroke"),
    ("panel-radiator", "comfort"),
    ("sleeping-person", "supine"),
]
VIEWS = [("thermal", "dark"), ("dark-line", "dark")]


def manifest():
    casts = {}
    for asset, pose in ASSETS:
        for view, theme in VIEWS:
            cast = {"asset": asset, "pose": pose, "view": view, "scale": 50, "theme": theme}
            if view == "thermal":
                cast["environment"] = "office-neutral"
            casts[f"{asset}-{view}"[:64]] = cast
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "manifest.json").write_text(json.dumps({"version": 1, "casts": casts}, ensure_ascii=False, indent=1), encoding="utf-8")
    print(len(casts), "casts")


def sheet():
    import fitz

    files = sorted((OUT / "out").glob("**/*.svg"))
    print(len(files), "svg")
    cell = 420
    cols = 4
    rows = (len(files) + cols - 1) // cols
    doc = fitz.open()
    page = doc.new_page(width=cols * cell, height=rows * cell)
    page.draw_rect(page.rect, color=None, fill=(0.12, 0.12, 0.14))
    for index, file in enumerate(files):
        src = fitz.open("pdf", fitz.open(str(file)).convert_to_pdf())
        r = src[0].rect
        scale = min((cell - 30) / r.width, (cell - 40) / r.height)
        w, h = r.width * scale, r.height * scale
        x, y = (index % cols) * cell + (cell - w) / 2, (index // cols) * cell + 28 + (cell - 40 - h) / 2
        page.show_pdf_page(fitz.Rect(x, y, x + w, y + h), src, 0)
        page.insert_text((x, (index // cols) * cell + 16), file.stem, fontsize=11, color=(1, 1, 1))
    page.get_pixmap(dpi=96).save(str(OUT / "sheet.png"))
    print(OUT / "sheet.png")


{"manifest": manifest, "sheet": sheet}[sys.argv[1]]()
