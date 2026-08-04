#!/usr/bin/env python3
"""Scan PDF table hairlines for overlapping stub artifacts at column boundaries."""
from __future__ import annotations

import json
import sys
from pathlib import Path

import fitz

PDF = Path(r"e:\semio\mit-bestand\bericht\zwischenbericht\dist\zwischenbericht-dark.pdf")
OUT = Path(__file__).resolve().parent


def hlines(page: fitz.Page) -> list[tuple[float, float, float, float]]:
    out: list[tuple[float, float, float, float]] = []
    for d in page.get_drawings():
        r = d.get("rect")
        if not r or r.height >= 2.5 or r.width < 8:
            continue
        out.append((r.y0, r.x0, r.x1, r.width))
    out.sort()
    return out


def stubs_at_y(lines: list[tuple[float, float, float, float]], y: float, tol: float = 3.5) -> list[tuple]:
    band = [ln for ln in lines if abs(ln[0] - y) <= tol]
    if len(band) < 2:
        return []
    main = max(band, key=lambda t: t[3])
    stubs = []
    for ln in band:
        if ln is main:
            continue
        if ln[3] < main[3] * 0.45 and abs(ln[0] - main[0]) <= tol:
            stubs.append((ln, main))
    return stubs


def page_stubs(page: fitz.Page) -> int:
    lines = hlines(page)
    if len(lines) < 4:
        return 0
    ys = sorted({round(ln[0], 1) for ln in lines})
    total = 0
    for y in ys:
        total += len(stubs_at_y(lines, y))
    return total


def main() -> int:
    doc = fitz.open(PDF)
    report: dict = {"pdf": str(PDF), "pages": [], "bb22_pages": [], "total_stubs": 0}
    for i in range(len(doc)):
        page = doc[i]
        text = page.get_text()
        stubs = page_stubs(page)
        report["total_stubs"] += stubs
        if stubs:
            report["pages"].append({"page": i + 1, "stubs": stubs})
        if "BB-22" in text and "Datenfelder" in text:
            report["bb22_pages"].append(i + 1)
            w = page.rect.width
            for z in (2, 3, 4):
                clip = fitz.Rect(w * 0.05, 80, w * 0.98, min(700, page.rect.height - 40))
                pix = page.get_pixmap(matrix=fitz.Matrix(z, z), clip=clip)
                pix.save(str(OUT / f"bb22-p{i+1}-z{z}.png"))
    doc.close()
    (OUT / "stub-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 0 if report["total_stubs"] == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
