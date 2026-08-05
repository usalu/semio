#!/usr/bin/env python3
"""Measure project-photo insets vs full-width cell borders (top/bottom/left)."""
from __future__ import annotations

import fitz

PDF = r"E:\semio\mit-bestand\bericht\zwischenbericht\dist\zwischenbericht.pdf"


def spans(page):
    out = []
    for b in page.get_text("dict")["blocks"]:
        if b.get("type") != 0:
            continue
        for line in b.get("lines", []):
            for s in line.get("spans", []):
                t = s["text"].strip()
                if t:
                    out.append((t, s["bbox"]))
    return out


def nearest(spans_list, text, near_y=None, near_x=None, ytol=30, xtol=120):
    best = None
    best_d = 1e9
    for t, bb in spans_list:
        if t != text:
            continue
        if near_y is not None and abs(bb[1] - near_y) > ytol:
            continue
        if near_x is not None and abs(bb[0] - near_x) > xtol:
            continue
        d = 0.0
        if near_y is not None:
            d += abs(bb[1] - near_y)
        if near_x is not None:
            d += abs(bb[0] - near_x)
        if d < best_d:
            best_d = d
            best = bb
    return best


def full_hlines(page, min_span=300):
    ys = []
    for d in page.get_drawings():
        for item in d.get("items", []):
            if item[0] != "l":
                continue
            p1, p2 = item[1], item[2]
            if abs(p1.y - p2.y) > 0.6:
                continue
            xa, xb = sorted([p1.x, p2.x])
            if xb - xa < min_span:
                continue
            ys.append(round((p1.y + p2.y) / 2, 2))
    return sorted(set(ys))


def main():
    doc = fitz.open(PDF)
    rows = []
    for i, page in enumerate(doc):
        text = page.get_text()
        if "Stadt" not in text or "Menge" not in text:
            continue
        sp = spans(page)
        full = full_hlines(page)
        imgs = [
            b["bbox"]
            for b in page.get_text("dict")["blocks"]
            if b.get("type") == 1
            and b["bbox"][2] - b["bbox"][0] > 80
            and b["bbox"][3] - b["bbox"][1] > 40
        ]
        for img in imgs:
            x0, y0, x1, y1 = img
            stadt = nearest(sp, "Stadt", near_y=y0, near_x=x1, ytol=18, xtol=260)
            if not stadt:
                continue
            menge = nearest(sp, "Menge", near_x=stadt[0], ytol=200, xtol=40)
            above = [y for y in full if y < y0]
            below = [y for y in full if y > y1]
            lefts = []
            for d in page.get_drawings():
                for item in d.get("items", []):
                    if item[0] != "l":
                        continue
                    p1, p2 = item[1], item[2]
                    if abs(p1.x - p2.x) > 0.6:
                        continue
                    x = (p1.x + p2.x) / 2
                    if x >= x0:
                        continue
                    ya, yb = sorted([p1.y, p2.y])
                    if ya <= y0 + 5 and yb >= y1 - 5:
                        lefts.append(x)
            if not above or not below or not lefts:
                continue
            top_b, bot_b, left_b = max(above), min(below), max(lefts)
            rows.append(
                {
                    "page": i + 1,
                    "gap_top": round(y0 - top_b, 2),
                    "gap_bot": round(bot_b - y1, 2),
                    "gap_left": round(x0 - left_b, 2),
                    "diff": round((y0 - top_b) - (bot_b - y1), 2),
                    "stadt_dy": round(stadt[1] - y0, 2),
                    "menge_dx": None if not menge else round(menge[0] - stadt[0], 2),
                }
            )

    n = len(rows)
    if not n:
        print("FAIL no cards")
        return
    avg = lambda k: sum(r[k] for r in rows) / n
    print(f"cards={n}")
    print(
        f"gap_top avg={avg('gap_top'):.2f} [{min(r['gap_top'] for r in rows):.2f},{max(r['gap_top'] for r in rows):.2f}]"
    )
    print(
        f"gap_bot avg={avg('gap_bot'):.2f} [{min(r['gap_bot'] for r in rows):.2f},{max(r['gap_bot'] for r in rows):.2f}]"
    )
    print(
        f"gap_left avg={avg('gap_left'):.2f} [{min(r['gap_left'] for r in rows):.2f},{max(r['gap_left'] for r in rows):.2f}]"
    )
    print(
        f"|top-bot| avg={sum(abs(r['diff']) for r in rows)/n:.2f} max={max(abs(r['diff']) for r in rows):.2f} bad(>0.75pt)={sum(1 for r in rows if abs(r['diff'])>0.75)}"
    )
    print(f"stadt_dy avg={avg('stadt_dy'):.2f} bad(>3pt)={sum(1 for r in rows if abs(r['stadt_dy'])>3)}")
    print(
        f"menge_dx bad(>1.5pt)={sum(1 for r in rows if r['menge_dx'] is not None and abs(r['menge_dx'])>1.5)}"
    )
    print("sample", rows[:3])


if __name__ == "__main__":
    main()
