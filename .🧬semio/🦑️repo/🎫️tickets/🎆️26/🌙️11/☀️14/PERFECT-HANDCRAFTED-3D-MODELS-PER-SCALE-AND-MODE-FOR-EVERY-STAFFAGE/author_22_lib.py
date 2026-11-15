"""Author 22: product-measure thermal loop (exposed surface, C1) on top of the author 07 kit."""
import json, re, subprocess
from author_07_lib import *
import author_07_lib as L

_shrink = L.shrink
L.shrink = lambda s, dist, id=None, floor=0.00382: _shrink(s, dist, id, floor)

def pcheck(slug, scale, rules, category="humans"):
    r = subprocess.run([L.bun(), f"{L.T}/volumes-check.ts", f"{category}/{slug}", "--scale", str(scale), "--rules", rules, "--json"], cwd=L.T, capture_output=True, text=True)
    j = json.loads(r.stdout[r.stdout.index("{"):])
    return j

def msg(f):
    m = f["message"]
    return m["en"] if isinstance(m, dict) else m

def exposed_all(slug, scale):
    out = {}
    for m in pcheck(slug, scale, "volumes-thermal-coverage")["measurements"]:
        n = re.match(r"areaDeviation:(.+)", m.get("name", ""))
        e = re.search(r"exposed ([\d.]+) m2.*areaM2 ([\d.]+)", m.get("note", "") or "")
        if n and e:
            out[n.group(1)] = (float(e.group(1)), float(e.group(2)), m["value"], m["limit"])
    return out

def exposed_failures(slug, scale):
    return {k: v[0] for k, v in exposed_all(slug, scale).items() if v[2] > v[3]}

def feature_names(slug, scale, lst):
    out = []
    for f in pcheck(slug, scale, "volumes-feature")["findings"]:
        m = re.search(r'Solid "([^"]+)" of (\w+)', msg(f))
        if m and m.group(2) == lst:
            out.append(m.group(1))
    return out

def set_areas(slug, values, category="humans"):
    p = L.Builder(slug, 50, category).path
    lines = open(p).read().split("\n")
    for region, val in values.items():
        i = next(k for k, l in enumerate(lines) if l.strip() == f'"id": "{region}",' and any('"areaM2"' in x for x in lines[k:k + 14]) and k > next(j for j, x in enumerate(lines) if '"thermal": {' in x))
        j = next(k for k in range(i, i + 14) if '"areaM2"' in lines[k])
        lines[j] = re.sub(r"[\d.]+,?$", lambda m: f"{val:.3g}" + ("," if m.group(0).endswith(",") else ""), lines[j])
    open(p, "w").write("\n".join(lines))

def thermal_refs(B, extra, drop=()):
    B.thermal = thermal_from_body(B, {}, None, extra, drop)
    return B.thermal

def finish_thermal(B, extra, fix_areas=False):
    drop = set()
    for rnd in range(3):
        thermal_refs(B, extra, drop)
        write_source(B.slug, B.scale, B.body_out(), B.thermal, B.structure, B.decisions)
        bodyby = {x["id"]: x for x in B.body}
        bad = []
        for t in B.thermal:
            if "emitter" in t["id"] or t["id"] in drop or "of" in t:
                continue
            src = bodyby[t["of"]] if "of" in t else t
            if L.min_extent_rough(src) < B.feature * 1.0 and src.get("role") != "texture":
                bad.append(t["id"])
        if not bad:
            break
        drop.update(bad)
    fails = exposed_failures(B.slug, B.scale)
    return fails, drop
