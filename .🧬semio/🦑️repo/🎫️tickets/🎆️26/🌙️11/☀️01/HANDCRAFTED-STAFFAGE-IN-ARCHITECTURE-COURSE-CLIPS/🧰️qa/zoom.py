import sys
from pathlib import Path
import pymupdf

here = Path(__file__).resolve().parent
gen = here.parent / "🗑️generated" / "qa"
ids = ["tealight", "expansion-valve", "cooking-pot", "thermostatic-radiator-valve", "gas-hob", "room-thermostat", "circulation-pump", "ceiling-diffuser"]
CW, CH = 600, 200
page = pymupdf.open().new_page(width=CW * 2, height=CH * len(ids))
for r, ident in enumerate(ids):
    for c, view in enumerate(("dark-line", "thermal")):
        f = next((gen / "svg").glob(f"*__{ident}__elevation__{view}.svg"))
        doc = pymupdf.open(str(f))
        rect = doc[0].rect
        clip = pymupdf.Rect(0, 0, rect.width, rect.height * 0.55)
        s = min((CW - 8) / clip.width, (CH - 8) / clip.height)
        pix = doc[0].get_pixmap(matrix=pymupdf.Matrix(s, s), clip=clip, alpha=False)
        x, y = c * CW, r * CH
        page.draw_rect(pymupdf.Rect(x, y, x + CW, y + CH), fill=(0.1, 0.1, 0.1))
        page.insert_image(pymupdf.Rect(x + 4, y + 4, x + 4 + pix.width, y + 4 + pix.height), pixmap=pix)
        page.insert_text((x + 6, y + 14), f"{ident} {view}", fontsize=11, color=(1, 0, 0))
page.get_pixmap(dpi=72).save(str(gen / "zoom.png"))
