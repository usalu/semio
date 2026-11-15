"""Reviewer 3: C27 sum-preserving redistribution among the CLOTHING regions of a human subject (bare skin keeps f = 1, hats keep their one-sided value, skinFraction and metabolicShare stay).
Gate: all clothing regions measured; sum declared / sum measured in [0.80, 1.20]; no region measured/declared > 3 or < 0.4; some region deviates beyond 30 % after the rescale; every f stays in [1.0, 1.6].
Prints the table and writes 🗑️generated/areareview3/c27.json (slug -> rows); areareview3-decisions.py reads BODY from it. Usage: python3 areareview3-c27apply.py"""
import glob, json, os
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
M = json.load(open(os.path.join(T, "🗑️generated", "areareview3", "measure.json")))
out = {}
for p in sorted(glob.glob(ROOT + "/*humans/*/🔣️.json")):
    slug = p.split("/")[-2]
    a = json.load(open(p))["asset"]
    S = a["thermal"]["body"]["skinAreaM2"]
    regs = [r for r in a["thermal"]["regions"] if r.get("body")]
    sk = sum(r["body"]["skinFraction"] for r in regs)
    C = [r for r in regs if r["kind"] in ("clothing", "surface") and r["id"] not in ("hat",)]
    if not C or any(M.get(slug, {}).get(r["id"], {}).get("outer") in (None, 0) for r in C):
        print(slug, "skip: no clothing set or unmeasured"); continue
    sd = sum(r["areaM2"] for r in C); mm = {r["id"]: M[slug][r["id"]]["outer"] for r in C}; so = sum(mm.values())
    fac = sd / so
    ratios = [mm[r["id"]] / r["areaM2"] for r in C]
    if not .8 <= fac <= 1.2 or max(ratios) > 3 or min(ratios) < .4:
        print(f"{slug} skip: factor {fac:.2f} ratios {min(ratios):.2f}-{max(ratios):.2f} (overlaid or buried solids: model not sound)"); continue
    floor = {r["id"]: r["body"]["skinFraction"] / sk * S for r in C}
    new = {r["id"]: mm[r["id"]] * fac for r in C}
    fixed = {}
    for _ in range(6):
        low = [i for i in new if i not in fixed and new[i] < floor[i]]
        if not low: break
        for i in low: fixed[i] = floor[i]
        rest = sd - sum(fixed.values()); mrest = sum(mm[i] for i in new if i not in fixed)
        for i in new:
            new[i] = fixed[i] if i in fixed else mm[i] * rest / mrest
    dev = [new[r["id"]] / r["areaM2"] for r in C]
    if not any(abs(x - 1) > .3 for x in dev):
        print(f"{slug} none: no clothing segment deviates beyond 30 % (factor {fac:.2f})"); continue
    fmax = max(new[r["id"]] / floor[r["id"]] for r in C)
    if fmax > 1.6:
        print(f"{slug} skip: f would reach {fmax:.2f}"); continue
    rows = []
    for r in C:
        n = round(new[r["id"]], 4)
        rows.append([r["id"], r["areaM2"], n, round(mm[r["id"]], 4), round(floor[r["id"]], 4)])
    d = round(sd - sum(x[2] for x in rows), 4)
    rows[max(range(len(rows)), key=lambda i: rows[i][2])][2] = round(rows[max(range(len(rows)), key=lambda i: rows[i][2])][2] + d, 4)
    out[slug] = {"rows": rows, "sumBefore": round(sd, 4), "sumAfter": round(sum(x[2] for x in rows), 4), "factor": round(fac, 3), "skinAreaM2": S}
    print(f"{slug} REDISTRIBUTE factor {fac:.2f} sum {sd:.4f} -> {out[slug]['sumAfter']}: " + ", ".join(f"{x[0]} {x[1]}->{x[2]}" for x in rows))
json.dump(out, open(os.path.join(T, "🗑️generated", "areareview3", "c27.json"), "w"), indent=1, ensure_ascii=False)
