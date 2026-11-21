"""🧮 R03 area reviewer, C27 for a human: bare-skin regions take the given hand values with f = 1 (their skinFraction follows, the difference
booked against the named garment), garments are rescaled by one k so the sum of the non-exchange body regions stays; prints old, new, f,
skinFraction and the deviation against the stored 1:50 / 1:100 outer areas, and the `r5-area-apply.py` command.
Usage: python3 r5-area-c27.py <category>/<slug> [--honest] [skin:<region>=<hand>@<garment> ...] [garment:<region>=<hand> ...] [keep:<region> ...]
  skin:  a bare-skin region set to its hand value; its skinFraction change is booked against <garment>
  garment: a garment's hand target (others: target = declared); keep: a body region left out of the rescale (value fixed)
@see ./r5-area-review-report.md (Method), ./design.md (C18, C19, C27)"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib.util
s = importlib.util.spec_from_file_location("m", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r5-area-measure.py")); m = importlib.util.module_from_spec(s); s.loader.exec_module(m)

spec = sys.argv[1]
skin, garm, keep = {}, {}, set()
honest = '--honest' in sys.argv
for a in [x for x in sys.argv[2:] if x != '--honest']:
    k, v = a.split(":", 1)
    if k == "skin":
        r, rest = v.split("=")
        val, g = rest.split("@")
        skin[r] = (float(val), g)
    elif k == "garment":
        r, val = v.split("=")
        garm[r] = float(val)
    elif k == "keep":
        keep.add(v)
doc = json.load(open(m.asset_path(spec)))["asset"]
S = doc["thermal"]["body"]["skinAreaM2"]
regs = [r for r in doc["thermal"]["regions"] if r["model"] == "body" and r.get("areaBasis") != "exchange"]
d = m.measure(spec, True)
out = {sc: {} for sc in (50, 100)}
for x in d["measurements"]:
    if x["scale"] in out and x["name"].startswith("info:outerAreaM2:"):
        out[x["scale"]][x["name"][17:]] = x["value"]
total = sum(r["areaM2"] for r in regs)
new, sf = {}, {r["id"]: r["body"]["skinFraction"] for r in regs}
for r in regs:
    if r["id"] in skin:
        v, g = skin[r["id"]]
        new[r["id"]] = v
        nsf = round(v / S, 5)
        sf[g] = round(sf[g] - (nsf - sf[r["id"]]), 5)
        sf[r["id"]] = nsf
    elif r["kind"] == "skin" or r["id"] in keep:
        new[r["id"]] = r["areaM2"]
gs = [r for r in regs if r["id"] not in new]
rest = total - sum(new.values())
tg = {r["id"]: garm.get(r["id"], r["areaM2"]) for r in gs}
k = 1.0 if honest else rest / sum(tg.values())
for r in gs:
    new[r["id"]] = round(tg[r["id"]] * k, 4)
big = max(gs, key=lambda r: new[r["id"]])["id"]
if not honest:
    new[big] = round(new[big] + total - sum(new.values()), 4)
ex = sum(r["areaM2"] for r in doc["thermal"]["regions"] if r["model"] == "body" and r.get("areaBasis") == "exchange")
print(f"## {spec} skinArea {S} sum non-exchange {total:.4f} -> {sum(new.values()):.4f} k {k:.4f} {'(honest garments, --honest)' if honest else '(C27 sum fixed)'}; fcl incl. exchange {(total + ex) / S:.3f} -> {(sum(new.values()) + ex) / S:.3f}")
cmd = []
for r in regs:
    i = r["id"]
    f = new[i] / (sf[i] * S)
    dv = "  ".join(f"1:{sc} {out[sc][i]:.4f} ({(out[sc][i] - new[i]) / new[i]:+.0%})" for sc in (50, 100) if i in out[sc])
    print(f"  {i:<18} {r['kind']:<9} {r['areaM2']:<7} -> {new[i]:<7} sf {r['body']['skinFraction']} -> {sf[i]}  f {f:.2f}{' F<1!' if f < 0.995 else ''}   model {dv}")
    parts = []
    if abs(new[i] - r["areaM2"]) > 1e-9: parts.append(f"areaM2={new[i]}")
    if abs(sf[i] - r["body"]["skinFraction"]) > 1e-9: parts.append(f"skinFraction={sf[i]}")
    if parts: cmd.append(f"{i}:{','.join(parts)}")
print(f"  sum skinFraction {sum(sf.values()) + sum(r['body']['skinFraction'] for r in doc['thermal']['regions'] if r['model'] == 'body' and r.get('areaBasis') == 'exchange'):.5f}")
print("  python3 r5-area-apply.py", spec, " ".join(cmd))
