"""📋 Compact listing of one list of one scale of an asset of author 10: id, bone, kind, smallest dimension in mm, placement, repeat and (for the coarse scales) whether the solid equals its 1:50 twin.

Usage: python3 rework-10-ls.py <slug> <scale> [body|thermal|structure] [filter]
"""
import json, sys
slug, scale = sys.argv[1], sys.argv[2]
lst = sys.argv[3] if len(sys.argv) > 3 else "body"
flt = sys.argv[4] if len(sys.argv) > 4 else ""
V = json.load(open(f"volumes/machinery/{slug}.json"))["volumes"]
sig = lambda s: json.dumps({k: v for k, v in s.items() if k not in ("tone", "role")}, sort_keys=True)
fine = {s["id"]: sig(s) for s in V["50"][lst]}
for s in V[scale][lst]:
    if flt and flt not in s["id"] and flt != s.get("bone"):
        continue
    if "of" in s:
        print(f"{s['id']} of={s['of']} {s.get('region','')}{s.get('member','')}/{s.get('part','')}")
        continue
    k = s["kind"]
    if k == "box":
        g = f"size={s['size']}"
    elif k == "revolve":
        p = s["profile"]
        g = f"ax={s['axis']} o={s['origin']} L={round(max(q[0] for q in p)-min(q[0] for q in p),3)} r={min(q[1] for q in p)}..{max(q[1] for q in p)} n={len(p)}"
    elif k == "extrude":
        p = s["profile"]
        g = f"{s['plane']} {s['from']}..{s['to']} bb=[{min(q[0] for q in p)},{max(q[0] for q in p)}]x[{min(q[1] for q in p)},{max(q[1] for q in p)}] n={len(p)} holes={len(s.get('holes', []))}"
    elif k == "loft":
        c = s["sections"]
        g = f"n={len(c)} {c[0]['at']}..{c[-1]['at']} r={min(min(x['rx'],x['ry']) for x in c)}..{max(max(x['rx'],x['ry']) for x in c)}"
    elif k == "capsule":
        g = f"{s['p0']}..{s['p1']} r={s['r0']},{s['r1']}"
    else:
        g = f"radii={s.get('radii')}"
    t = s.get("transform", {})
    extra = (" t=" + json.dumps(t.get("translation"))) if "translation" in t else ""
    extra += " rot" if "rotation" in t else ""
    extra += f" x{s['repeat']['count']}" if "repeat" in s else ""
    extra += " SAME" if scale != "50" and fine.get(s["id"]) == sig(s) else ""
    extra += "".join(f" {f}" for f in ("slender", "texture") if s.get(f))
    tag = s.get("region") or s.get("member") or ""
    print(f"{s['id']} [{s['bone']}] {k} {g}{extra} {s.get('role','')} {tag}/{s.get('part','')}")
