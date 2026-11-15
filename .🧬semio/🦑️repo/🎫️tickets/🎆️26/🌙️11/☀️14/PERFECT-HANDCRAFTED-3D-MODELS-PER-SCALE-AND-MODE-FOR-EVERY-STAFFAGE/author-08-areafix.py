import sys, re, json
from author_08_lib import *

def fmt(x):
    return f"{float(f'{x:.3g}'):g}"

def run(slug, apply):
    r = check("humans", slug, "50")
    fixes = {}
    for m in r["measurements"]:
        if m["name"].startswith("areaDeviation:") and not m["ok"] and m.get("note"):
            mm = re.match(r"exposed ([0-9.]+) m2", m["note"])
            reg = m["name"].split(":")[1]
            if mm:
                fixes[reg] = float(mm.group(1))
    a = load_asset("humans", slug)
    old = {x["id"]: x["areaM2"] for x in a["thermal"]["regions"]}
    plan = {k: (old[k], float(fmt(v))) for k, v in fixes.items() if v > 0 and 0.7 <= v / old[k] <= 1.45}
    skip = {k: (old[k], v) for k, v in fixes.items() if k not in plan}
    print(slug, "fix", plan, "skip", skip)
    if apply and plan:
        path = asset_file("humans", slug)
        lines = open(path).read().split("\n")
        t0 = next(i for i, l in enumerate(lines) if re.match(r'^\s*"thermal": \{', l))
        start = next(i for i in range(t0, len(lines)) if lines[i].strip() == '"regions": [')
        for rid, (o, n) in plan.items():
            i = next(i for i in range(start, len(lines)) if lines[i].strip() == f'"id": "{rid}",')
            j = next(j for j in range(i, i + 40) if lines[j].strip().startswith('"areaM2":'))
            lines[j] = re.sub(r'"areaM2": *[0-9.eE+-]+', f'"areaM2": {fmt(n)}', lines[j])
        open(path, "w").write("\n".join(lines))
    return plan

if __name__ == "__main__":
    run(sys.argv[1], "--apply" in sys.argv)
