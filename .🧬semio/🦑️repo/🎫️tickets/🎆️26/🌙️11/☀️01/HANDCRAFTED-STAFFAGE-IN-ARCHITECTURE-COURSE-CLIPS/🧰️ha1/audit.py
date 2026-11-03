import json, sys, os
from collections import Counter
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from shape_ext import shape_ext
BASE = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances"
def load(a): return json.load(open(os.path.join(BASE, a, "🔣️.json"), encoding="utf-8"))["asset"]
from shape_ext import shape_ext as _se
def _unused(s):
    k = s.get("kind")
    if k == "rect": return (s["x"], s["y"], s["x"] + s["w"], s["y"] + s["h"])
    if k == "poly" or k == "polyline" or "points" in s:
        xs = [p[0] for p in s.get("points", [])] or [0]; ys = [p[1] for p in s.get("points", [])] or [0]
        return (min(xs), min(ys), max(xs), max(ys))
    if k in ("circle", "ellipse"):
        r = s.get("r", 0); return (s["cx"] - r, s["cy"] - r, s["cx"] + r, s["cy"] + r)
    return None
def audit(a):
    d = load(a); out = []
    P = out.append
    P(f"=== {a} | dims {d['dimensions']} | viewpoints {d['presentation'].get('viewpoints')} default {d['presentation'].get('defaultPose')}/{d['presentation'].get('defaultClip')} scale {d['presentation'].get('defaultScale')}")
    P(f"materials: " + ", ".join(f"{m['id']}(e{m.get('emissivity')},{m.get('source')})" for m in d.get('materials', [])))
    P(f"bones: " + ", ".join(b.get('id', '?') for b in d['rig']['bones']))
    P(f"anchors: " + ", ".join(x.get('id', '?') for x in d.get('anchors', [])))
    th = d['thermal']
    P(f"env: {th.get('environment')}")
    P(f"operating: {[ (o.get('id'), [l.get('key') for l in o.get('levels',[])] ) for o in th.get('operating',[])]}")
    regs = {r['id']: r for r in th['regions']}
    P("regions: " + "; ".join(f"{r['id']}[{r.get('model')},{r.get('kind')},A{r.get('areaM2')},C{r.get('heatCapacityJM2K')},mat {r.get('material')},op {(r.get('operating') or {}).get('heatW')}]" for r in th['regions']))
    P("emitters: " + "; ".join(f"{e.get('id')}[{e.get('kind')}, {({k:v for k,v in e.items() if k in ('heatW','temperatureRiseK','volumeFlowM3S','temperatureC','exhaledTempC','operatingKey')})}]" for e in th.get('emitters', [])))
    for rep in ['50', '100', '200', '500']:
        R = d['representations'][rep]
        if R['status'] != 'authored':
            P(f"-- {rep}: {R['status']} limiting {R.get('limitingFeature',{}).get('en','')[:60]} size {R.get('limitingSizeM')}")
            continue
        P(f"-- {rep}: valid {R['validRange']} pens {R['pens']} hatches {len(R['hatches'])}")
        g = R['geometry']
        for view, shapes in g.items():
            ids = [s.get('id') for s in shapes]
            dup = [k for k, v in Counter(ids).items() if v > 1]
            exts = [shape_ext(s) for s in shapes]
            xs0 = min(e[0] for e in exts if e); xs1 = max(e[2] for e in exts if e)
            ys0 = min(e[1] for e in exts if e); ys1 = max(e[3] for e in exts if e)
            P(f"   {view}: n={len(shapes)} extent x[{xs0:.3f},{xs1:.3f}] y[{ys0:.3f},{ys1:.3f}] dup={dup}")
            P("     " + " | ".join(f"{s.get('id')}:{s.get('kind')}/{s.get('role')}/{('pen'+str(s.get('penMm'))) if 'penMm' in s else ''}/{','.join(s.get('regions',[]) or ([s['region']] if s.get('region') else []))}/{s.get('material','')}/z{s.get('z')}/t{s.get('tone')}" for s in shapes))
        used = set()
        for shapes in g.values():
            for s in shapes:
                used.update(s.get('regions', []) or [])
                if s.get('region'): used.add(s['region'])
        P(f"   regions drawn: {sorted(used)} | missing from thermal: {sorted(used - set(regs))} | thermal not drawn: {sorted(set(regs) - used)}")
        P(f"   decisions: {list(R.get('decisions',{}).keys())}")
        P(f"   validRange ok: {R['validRange']['minDenominator'] <= int(rep) <= R['validRange']['maxDenominator']}")
    for p in d['poses']:
        t = p.get('thermal', {})
        rr = set(t.get("regions", {}).keys())
        ee = set(t.get("emitters", {}).keys())
        ops = t.get('operating', [])
        P(f"pose {p['id']}: bones {len(p.get('bones',{}))} ground {p.get('ground')} contacts {len(p.get('contacts',[]))} loads {len(p.get('loads',[]))} thermalKeys {list(t.keys())} regionsBound {len(rr)}/{len(regs)} missing {sorted(set(regs)-rr)} emittersBound {sorted(ee)} ops {ops if not isinstance(ops, list) else len(ops)}")
    for c in d['clips']:
        P(f"clip {c['id']}: dur {c.get('durationS')} fps {c.get('fps')} loop {c.get('loop')} keys {len(c.get('keys',[]))} phases {[x.get('id') for x in c.get('phases',[])]} tracks {[t.get('target') or t.get('id') or t.get('kind') for t in c.get('tracks',[])]} events {[e.get('kind') or e.get('id') for e in c.get('events',[])]}")
    P(f"title: {d['title']} | tags {d.get('tags')}")
    P(f"prov refs: {d['provenance'].get('references')} | notes: {d['provenance'].get('notes',{}).get('en','')[:200]}")
    P(f"structure: {d.get('structure',{}).get('supports')}")
    return "\n".join(out)
if __name__ == "__main__":
    for a in sys.argv[1:]:
        print(audit(a)); print()
