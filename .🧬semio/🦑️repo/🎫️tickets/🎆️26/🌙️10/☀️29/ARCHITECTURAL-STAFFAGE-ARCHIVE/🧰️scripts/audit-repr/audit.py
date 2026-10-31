"""📐️ Read-only representation audit of the staffage catalog (stdlib only). Writes a markdown report; never touches the catalog.

@see 🧬️schema/🟦️.ts — Shape, Representation, UnavailableRepresentation
"""
import glob
import json
import os
import re
import sys
from collections import Counter, defaultdict
from datetime import date

DEFAULT_STAFFAGE = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
DEFAULT_OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/repr-audit.md"
SCALES = ["500", "200", "100", "50"]
DECISIONS = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
V1_FIELDS = {"lod", "coreTempC", "riseK", "surfaceTempC", "insulationM2KW", "periodS", "duty"}
PATH_TOKEN = re.compile(r"[A-Za-z]|[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?")
PATH_ARITY = {"M": 2, "L": 2, "T": 2, "H": 1, "V": 1, "C": 6, "S": 4, "Q": 4, "A": 7}
POINT_TOL = 1e-6


def text_ok(t):
    return isinstance(t, dict) and all(isinstance(t.get(l), str) and t[l].strip() for l in ("en", "de"))


def parse_path(d):
    toks = PATH_TOKEN.findall(d)
    sig, vals, pts, segs = [], [], 0, 0
    cmd, i = None, 0
    while i < len(toks):
        t = toks[i]
        if t.isalpha():
            cmd = t
            i += 1
            if cmd in ("Z", "z"):
                sig.append(cmd)
            continue
        if cmd is None or cmd.upper() not in PATH_ARITY:
            break
        up = cmd.upper()
        n = PATH_ARITY[up]
        if i + n > len(toks):
            break
        nums = [float(x) for x in toks[i:i + n]]
        i += n
        sig.append(cmd)
        if up == "A":
            vals += [nums[0], nums[1], nums[5], nums[6]]
            pts += 1
            segs += 1
        elif up == "C":
            vals += nums
            pts += 3
            segs += 1
        elif up in ("S", "Q"):
            vals += nums
            pts += 2
            segs += 1
        elif up == "M":
            vals += nums
            pts += 1
        else:
            vals += nums
            pts += 1
            segs += 1
        if cmd == "M":
            cmd = "L"
        elif cmd == "m":
            cmd = "l"
    return tuple(sig), vals, pts, segs


def shape_parts(s):
    k = s["kind"]
    if k == "capsule":
        return (k,), [s["x0"], s["y0"], s["x1"], s["y1"], s["r0"], s["r1"]], 2, 1
    if k == "ellipse":
        return (k,), [s["cx"], s["cy"], s["rx"], s["ry"]], 1, 1
    if k == "rect":
        return (k, "r" in s), [s["x"], s["y"], s["w"], s["h"]] + ([s["r"]] if "r" in s else []), 4, 4
    if k == "poly":
        n = len(s["points"])
        return (k, s["closed"]), [c for p in s["points"] for c in p], n, max(n - 1, 0) + (1 if s["closed"] else 0)
    if k == "path":
        sig, vals, p, g = parse_path(s["d"])
        return (k, sig), vals, p, g
    return (k,), [], 0, 0


def shape_map(rep):
    m = {}
    for vp in ("elevation", "plan"):
        for s in rep["geometry"].get(vp) or []:
            m[(vp, s["id"])] = shape_parts(s)
    return m


def uniform_factor(a, b):
    if not a or set(a) != set(b):
        return None
    ratios = []
    for key, (sa, va, _, _) in a.items():
        sb, vb, _, _ = b[key]
        if sa != sb or len(va) != len(vb):
            return None
        for x, y in zip(va, vb):
            if abs(x) < 1e-12 and abs(y) < 1e-12:
                continue
            if abs(x) < 1e-12 or abs(y) < 1e-12:
                return None
            ratios.append(y / x)
    if not ratios:
        return None
    k = ratios[0]
    return k if all(abs(r - k) <= POINT_TOL * abs(k) for r in ratios) else None


def find_v1(node, path, out):
    if isinstance(node, dict):
        for k, v in node.items():
            if k in V1_FIELDS:
                out.append(f"{path}.{k}")
            find_v1(v, f"{path}.{k}", out)
    elif isinstance(node, list):
        for v in node:
            find_v1(v, f"{path}[]", out)


def analyse_scale(sc, rep):
    info = {"sc": sc, "issues": [], "kind": None, "shapes": {}, "count": 0, "pts": 0, "segs": 0, "elev": 0, "plan": 0, "pens": 0, "hatches": 0, "dec": "n/a", "vr": "n/a"}
    if rep is None:
        info["kind"] = "missing"
        info["issues"].append(f"missing:{sc}")
        return info
    info["kind"] = rep.get("status")
    if rep.get("denominator") != int(sc):
        info["issues"].append(f"denom:{sc}")
    if info["kind"] == "unavailable":
        miss = [k for k in ("reason", "limitingFeature") if not text_ok(rep.get(k))]
        lim = rep.get("limitingSizeM")
        if not isinstance(lim, (int, float)) or isinstance(lim, bool) or lim <= 0:
            miss.append("limitingSizeM")
        if not rep.get("alternatives"):
            miss.append("alternatives")
        if miss:
            info["issues"].append(f"(e) {sc} missing {','.join(miss)}")
        return info
    geom = rep.get("geometry", {})
    shapes = shape_map(rep)
    info["shapes"] = shapes
    info["elev"] = len(geom.get("elevation") or [])
    info["plan"] = len(geom.get("plan") or [])
    info["count"] = info["elev"] + info["plan"]
    info["pts"] = sum(p for _, _, p, _ in shapes.values())
    info["segs"] = sum(g for _, _, _, g in shapes.values())
    pens = {round(v, 4) for v in (rep.get("pens") or {}).values() if v is not None}
    for vp in ("elevation", "plan"):
        for s in geom.get(vp) or []:
            if "penMm" in s:
                pens.add(round(s["penMm"], 4))
    info["pens"] = len(pens)
    info["hatches"] = len(rep.get("hatches") or [])
    dec = rep.get("decisions") or {}
    missing = [k for k in DECISIONS if not text_ok(dec.get(k))]
    info["dec"] = "ok" if not missing else "missing " + ",".join(missing)
    if missing:
        info["issues"].append(f"dec:{sc} {','.join(missing)}")
    vr = rep.get("validRange") or {}
    lo, hi = vr.get("minDenominator"), vr.get("maxDenominator")
    vr_ok = isinstance(lo, (int, float)) and isinstance(hi, (int, float)) and lo <= int(sc) <= hi
    info["vr"] = "ok" if vr_ok else "bad"
    if not vr_ok:
        info["issues"].append(f"vr:{sc}")
    return info


def pair_flags(infos):
    flags = []
    for i, x in enumerate(SCALES):
        for y in SCALES[i + 1:]:
            ix, iy = infos[x], infos[y]
            if ix["kind"] != "authored" or iy["kind"] != "authored":
                continue
            if set(ix["shapes"]) == set(iy["shapes"]) and ix["shapes"]:
                flags.append(f"(a) {x}={y} same shape ids")
            k = uniform_factor(ix["shapes"], iy["shapes"])
            if k is not None:
                flags.append(f"(b) {x}~{y} " + ("identical" if abs(k - 1) <= POINT_TOL else f"k={k:.4g}"))
            if ix["pts"] > iy["pts"]:
                flags.append(f"(c) {x} has {ix['pts']} pts > {y} {iy['pts']} pts")
    a, b = infos["500"], infos["50"]
    if a["kind"] == "authored" and b["kind"] == "authored":
        r = a["count"] / b["count"] if b["count"] else float("inf")
        if r > 0.6:
            flags.append(f"(d) shape ratio 500/50 = {r:.2f}")
    return flags


def audit_asset(path):
    with open(path, encoding="utf-8") as fh:
        doc = json.load(fh)
    a = doc["asset"]
    reps = a.get("representations") or {}
    infos = {sc: analyse_scale(sc, reps.get(sc)) for sc in SCALES}
    flags = [i for sc in SCALES for i in infos[sc]["issues"]]
    flags += pair_flags(infos)
    v1 = []
    find_v1(doc, "", v1)
    if "geometry" in a:
        v1.append("asset.geometry")
    v1 = sorted({re.sub(r"\[\]", "", p).lstrip(".") for p in v1})
    if v1:
        flags.append("(f) v1 fields: " + ", ".join(v1[:6]) + (f" (+{len(v1) - 6})" if len(v1) > 6 else ""))
    return {"id": a.get("id", os.path.basename(os.path.dirname(path))), "category": a.get("category", os.path.basename(os.path.dirname(os.path.dirname(path)))), "infos": infos, "flags": flags}


def cell(info):
    if info["kind"] == "missing":
        return "MISSING"
    if info["kind"] == "unavailable":
        return "UNAVAILABLE"
    return f"{info['elev']}/{info['plan']} sh · {info['pts']} pt · {info['pens']} pen · {info['hatches']} h"


def label(flag):
    return re.split(r"[ :]", flag, maxsplit=1)[0]


def build_report(results, staffage):
    lines = []
    w = lines.append
    flagged = [r for r in results if r["flags"]]
    total_flags = Counter(label(f) for r in results for f in r["flags"])
    w("# 📐️ Representation Audit — Staffage Catalog")
    w("")
    w(f"Generated {date.today().isoformat()} by `audit.py` (read-only, stdlib). Source: `{staffage}`.")
    w("")
    w("Shape counts are `elevation/plan` shapes. `pt` counts coordinates per shape: poly = points, path = endpoints plus control points, capsule = 2, ellipse = 1 centre, rect = 4 corners. `pen` = distinct paper pens (PenSet values plus shape `penMm`). `h` = hatches.")
    w("Flag codes: `(a)` identical shape-id sets between two scales; `(b)` coordinates identical or uniformly scaled (k) between two scales; `(c)` smaller drawing scale (larger denominator) has more points than the larger one; `(d)` shape ratio 1:500 / 1:50 above 0.6; `(e)` unavailable entry missing reason/limitingFeature/limitingSizeM/alternatives; `(f)` leftover v1 fields; `dec` decisions not complete in en+de; `vr` validRange not containing its own denominator; `denom` denominator mismatch; `missing` scale not present.")
    w("")
    w("## Per-asset table")
    w("")
    w("| Asset | Category | 1:500 | 1:200 | 1:100 | 1:50 | Flags |")
    w("|---|---|---|---|---|---|---|")
    for r in results:
        cells = [cell(r["infos"][sc]) for sc in SCALES]
        fl = "; ".join(r["flags"]) if r["flags"] else "—"
        w(f"| {r['id']} | {r['category']} | " + " | ".join(cells) + f" | {fl} |")
    w("")
    w("## Flagged assets")
    w("")
    w(f"{len(flagged)} of {len(results)} assets flagged, {len(results) * 4} asset-scale entries audited.")
    w("")
    for r in flagged:
        w(f"- **{r['id']}** ({r['category']}): " + "; ".join(r["flags"]))
    w("")
    w("Flag counts by code: " + (", ".join(f"{k} {v}" for k, v in sorted(total_flags.items())) or "none"))
    w("")
    w("## Per-category totals")
    w("")
    w("| Category | Assets | Flagged | Unavailable (500/200/100/50) | Points (500/200/100/50) | Shapes (500/200/100/50) |")
    w("|---|---|---|---|---|---|")
    cats = defaultdict(list)
    for r in results:
        cats[r["category"]].append(r)
    grand = {"assets": 0, "flagged": 0}
    for cat in sorted(cats):
        rs = cats[cat]
        un = [sum(1 for r in rs if r["infos"][sc]["kind"] == "unavailable") for sc in SCALES]
        pts = [sum(r["infos"][sc]["pts"] for r in rs) for sc in SCALES]
        shp = [sum(r["infos"][sc]["count"] for r in rs) for sc in SCALES]
        fl = sum(1 for r in rs if r["flags"])
        w(f"| {cat} | {len(rs)} | {fl} | " + "/".join(map(str, un)) + " | " + "/".join(map(str, pts)) + " | " + "/".join(map(str, shp)) + " |")
        grand["assets"] += len(rs)
        grand["flagged"] += fl
    w(f"| **Total** | {grand['assets']} | {grand['flagged']} | | | |")
    w("")
    w("## Detail per asset and scale")
    w("")
    w("| Asset | Scale | Kind | Elev | Plan | Points | Segments | Pens | Hatches | Decisions en+de | validRange |")
    w("|---|---|---|---|---|---|---|---|---|---|---|")
    for r in results:
        for sc in SCALES:
            i = r["infos"][sc]
            if i["kind"] in ("authored",):
                w(f"| {r['id']} | 1:{sc} | authored | {i['elev']} | {i['plan']} | {i['pts']} | {i['segs']} | {i['pens']} | {i['hatches']} | {i['dec']} | {i['vr']} |")
            else:
                w(f"| {r['id']} | 1:{sc} | {i['kind']} | — | — | — | — | — | — | — | — |")
    return "\n".join(lines) + "\n"


def main():
    staffage = sys.argv[1] if len(sys.argv) > 1 else DEFAULT_STAFFAGE
    out = sys.argv[2] if len(sys.argv) > 2 else DEFAULT_OUT
    files = sorted(
        f for f in glob.glob(os.path.join(staffage, "🗂️catalog", "*", "*", "🔣️.json"))
        if os.path.basename(os.path.dirname(os.path.dirname(f))) != "🖨️print-profiles"
    )
    results = sorted((audit_asset(f) for f in files), key=lambda r: (r["category"], r["id"]))
    with open(out, "w", encoding="utf-8", newline="\n") as fh:
        fh.write(build_report(results, staffage))
    flagged = sum(1 for r in results if r["flags"])
    print(f"assets={len(results)} flagged={flagged} out={out}")


if __name__ == "__main__":
    main()
