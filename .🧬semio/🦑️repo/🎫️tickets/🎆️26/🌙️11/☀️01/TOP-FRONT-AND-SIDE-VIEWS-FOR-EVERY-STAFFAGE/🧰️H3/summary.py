import json,sys
ROOT=r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\👤️humans"
OUT=r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H3"
L=[]
for a in sys.argv[1:]:
    d=json.load(open(f"{ROOT}\\{a}\\🔣️.json",encoding="utf8"))["asset"]
    L.append(f"##### {a} dims={d['dimensions']} pres={d['presentation']}")
    L.append("bones:")
    for b in d["rig"]["bones"]:
        L.append(f"  {b['id']:<14} p={b['parent']} x={b['x']} y={b['y']} a={b['angleDeg']} len={b['lengthM']}")
    L.append("anchors: "+", ".join(f"{x['id']}@{x.get('bone')}" for x in d["anchors"]))
    L.append("poses:")
    for p in d["poses"]:
        L.append(f"  {p['id']}: "+json.dumps({k:v.get('angleDeg') for k,v in p['bones'].items()},separators=(",",":")) + " extra=" + json.dumps({k:v for k,v in p.items() if k in ('root','ground','offset')}))
    L.append("clips:")
    for c in d["clips"]:
        L.append(f"  {c['id']} dur={c['durationS']} rootMotion={c.get('rootMotion')} keys={[(k['t'],k.get('pose')) for k in c['keys']]}")
    reps=d["representations"]
    for sc,r in reps.items():
        g=r.get("geometry")
        if not g:
            L.append(f"scale {sc} {r['status']} (no geometry) keys={list(r.keys())}"); continue
        L.append(f"scale {sc} {r['status']} " + str({k:len(v) for k,v in g.items() if isinstance(v,list)}) + f" extra={[k for k in g if not isinstance(g[k],list)]}")
        L.append("  elev ids: "+", ".join(f"{s['id']}({s['bone']},{s['kind'][0]},z{s['z']})" for s in g['elevation']))
        if g.get("plan"): L.append("  plan ids: "+", ".join(f"{s['id']}({s['bone']},{s['kind'][0]},z{s['z']})" for s in g['plan']))
open(f"{OUT}\\summary.txt","w",encoding="utf8").write("\n".join(L))
print("done")
