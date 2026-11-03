import json, sys, os
from PIL import Image, ImageDraw

FILL = {"body": (255, 255, 255), "detail": None}
TONES = [(236, 240, 244), (244, 236, 222), (232, 226, 240), (226, 240, 230), (246, 228, 226), (226, 230, 246)]


def rng(shapes):
    xs, ys = [], []
    for s in shapes:
        if s["role"] == "ground":
            continue
        for r in s["rings"]:
            for x, y in r:
                xs.append(x)
                ys.append(y)
    if not xs:
        return None
    return (min(xs), max(xs), min(ys), max(ys))


def stats(path):
    data = json.load(open(path, encoding="utf-8"))
    for q in data:
        if "error" in q:
            print("%-22s ERROR %s" % (q["label"], q["error"][:100]))
            continue
        out = []
        for v in ("elevation", "front", "plan"):
            r = rng(q["views"][v])
            out.append("%s x%.3f..%.3f y%.3f..%.3f" % (v[0].upper(), r[0], r[1], r[2], r[3]) if r else "%s EMPTY" % v[0].upper())
        print("%-16s %s" % (q["label"], " | ".join(out)))


def panel(shapes, box, ext, ss=3):
    x0, y0, x1, y1 = box
    w, h = (x1 - x0) * ss, (y1 - y0) * ss
    img = Image.new("RGB", (w, h), (255, 255, 255))
    d = ImageDraw.Draw(img)
    if ext is None:
        return img.resize((x1 - x0, y1 - y0))
    minx, maxx, miny, maxy = ext
    sc = w / max(maxx - minx, 1e-6)
    ox = -sc * minx
    oy = h + sc * miny

    def tr(p):
        return (ox + sc * p[0], oy - sc * p[1])

    for s in sorted(shapes, key=lambda s: s["z"]):
        for r in s["rings"]:
            if len(r) < 2:
                continue
            pts = [tr(p) for p in r]
            if s["role"] == "ground":
                d.line(pts, fill=(150, 150, 150), width=ss)
                continue
            if s["closed"] and s["filled"] and len(pts) >= 3:
                d.polygon(pts, fill=TONES[int(round(s["z"] * 10)) % len(TONES)] if s["role"] == "body" else None, outline=None)
                d.line(pts + [pts[0]], fill=(10, 10, 10) if s["role"] == "body" else (160, 20, 20), width=ss if s["role"] == "body" else max(1, ss // 2), joint="curve")
            else:
                d.line(pts + ([pts[0]] if s["closed"] else []), fill=(10, 10, 10) if s["role"] == "body" else (160, 20, 20), width=ss if s["role"] == "body" else max(1, ss // 2))
    return img.resize((x1 - x0, y1 - y0), Image.LANCZOS)


def sheet(path, labels, out, pw=560, ph=380, ppm=None):
    cache = {}

    def get(p, lab):
        if p not in cache:
            cache[p] = {q["label"]: q for q in json.load(open(p, encoding="utf-8"))}
        return cache[p][lab]

    rows = []
    for l in labels:
        if "#" in l:
            p, lab = l.split("#", 1)
            rows.append((os.path.join(os.path.dirname(path), p), lab, get(os.path.join(os.path.dirname(path), p), lab)))
        else:
            rows.append((path, l, get(path, l)))
    maxw = maxh = 0.0
    for _, _, q in rows:
        for v in ("elevation", "front", "plan"):
            r = rng(q["views"][v])
            if r:
                maxw = max(maxw, r[1] - r[0])
                maxh = max(maxh, r[3] - r[2])
    ppm = ppm or min((pw - 30) / max(maxw, 1e-6), (ph - 30) / max(maxh, 1e-6))
    img = Image.new("RGB", (pw * 3, ph * len(rows)), (255, 255, 255))
    d = ImageDraw.Draw(img)
    for i, (_, lab, q) in enumerate(rows):
        for j, v in enumerate(("elevation", "front", "plan")):
            r = rng(q["views"][v])
            if r:
                cx, cy = (r[0] + r[1]) / 2, (r[2] + r[3]) / 2
                half_w, half_h = pw / 2 / ppm, ph / 2 / ppm
                ext = (cx - half_w, cx + half_w, cy - half_h, cy + half_h)
                p = panel(q["views"][v], (0, 0, pw, ph), ext)
                img.paste(p, (j * pw, i * ph))
            d.text((j * pw + 6, i * ph + 4), "%s %s" % (lab, v), fill=(0, 0, 160))
            d.rectangle((j * pw, i * ph, (j + 1) * pw - 1, (i + 1) * ph - 1), outline=(200, 200, 200))
    img.save(out)


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "stats":
        stats(sys.argv[2])
    else:
        sheet(sys.argv[2], sys.argv[4].split(","), sys.argv[3])
