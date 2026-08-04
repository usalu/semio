#!/usr/bin/env python3
"""Detect full-width table hrules overlapping segmented hhline rows (stub root cause)."""
from __future__ import annotations

import json
import sys
from pathlib import Path

import fitz

PDF = Path(r"e:\semio\mit-bestand\bericht\zwischenbericht\dist\zwischenbericht-dark.pdf")
OUT = Path(__file__).resolve().parent
TABLE_WIDTH_MIN = 380.0
Y_CLUSTER = 1.2


def horizontals(page: fitz.Page) -> list[tuple[float, float, float, float]]:
    out: list[tuple[float, float, float, float]] = []
    for d in page.get_drawings():
        r = d.get("rect")
        if not r or r.height >= 2.5 or r.width < 4:
            continue
        out.append((r.y0, r.x0, r.x1, r.width))
    return out


def page_defects(page: fitz.Page) -> list[dict]:
    lines = horizontals(page)
    by_y: dict[float, list[tuple[float, float, float, float]]] = {}
    for ln in lines:
        yk = round(ln[0] / Y_CLUSTER) * Y_CLUSTER
        by_y.setdefault(yk, []).append(ln)
    defects: list[dict] = []
    for yk in sorted(by_y):
        band = by_y[yk]
        full = [ln for ln in band if ln[3] >= TABLE_WIDTH_MIN]
        seg = [ln for ln in band if ln[3] < TABLE_WIDTH_MIN and len(band) >= 3]
        if full and seg:
            defects.append(
                {
                    "y": round(yk, 2),
                    "full_width": round(full[0][3], 1),
                    "segments": len(seg),
                }
            )
    return defects


def main() -> int:
    doc = fitz.open(PDF)
    report: dict = {"pdf": str(PDF), "pages_with_overlap": [], "samples": {}}
    for i in range(len(doc)):
        defects = page_defects(doc[i])
        if defects:
            report["pages_with_overlap"].append({"page": i + 1, "defects": defects})
        text = doc[i].get_text()
        if "Datenfelder und Beschaffung" in text and "Angebotsbezogene" in text:
            w = doc[i].rect.width
            clip = fitz.Rect(w * 0.02, 155, w * 0.98, 235)
            for z in (2, 3, 4):
                pix = doc[i].get_pixmap(matrix=fitz.Matrix(z, z), clip=clip)
                pix.save(str(OUT / f"datenfelder-header-p{i+1}-z{z}.png"))
            report["samples"]["datenfelder_header_page"] = i + 1
    doc.close()
    (OUT / "overlap-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 1 if report["pages_with_overlap"] else 0


if __name__ == "__main__":
    sys.exit(main())
