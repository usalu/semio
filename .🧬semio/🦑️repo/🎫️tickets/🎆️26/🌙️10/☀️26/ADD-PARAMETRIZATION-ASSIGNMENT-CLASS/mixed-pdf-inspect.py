"""🔎 Inspect candidate first-page vector objects for a geometric PDF oracle."""
from pathlib import Path
import pdfplumber
p=Path(__file__).parent/"🗑️generated/compiled/parametrization-de.pdf"
with pdfplumber.open(p) as pdf:
    page=pdf.pages[0]
    print(page.extract_text())
    for c in page.curves:
        if 100<c["top"]<550:
            print("CURVE",tuple(round(c[k],2) for k in ["x0","top","x1","bottom"]),len(c["pts"]),c.get("path",[])[:2])
    for r in page.rects:
        if 100<r["top"]<550:
            print("RECT",tuple(round(r[k],2) for k in ["x0","top","x1","bottom"]))
