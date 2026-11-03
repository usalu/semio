import json, sys, os

def t(x):
    if isinstance(x, dict):
        return x.get("en", json.dumps(x, ensure_ascii=False))
    return x

def short(s, n=160):
    s = str(s)
    return s if len(s) <= n else s[: n - 3] + "..."

def fmt(v):
    return json.dumps(v, ensure_ascii=False)

def sheet(path):
    with open(path, encoding="utf-8") as fh:
        doc = json.load(fh)
    a = doc["asset"]
    L = []
    L.append(f"# {a['id']} [{a['category']}]")
    L.append(f"title: {fmt(a['title'])}")
    L.append(f"summary.en: {a['summary']['en']}")
    L.append(f"summary.de: {a['summary']['de']}")
    L.append(f"tags: {fmt(a['tags'])}")
    L.append(f"dimensions: {fmt(a['dimensions'])}")
    L.append(f"materials: {fmt([m.get('id') for m in a['materials']])}")
    L.append(f"provenance.refs: {fmt([r.get('id') for r in a['provenance']['references']])}")
    L.append(f"provenance.notes: {fmt(a['provenance'].get('notes'))}")
    L.append(f"presentation: {fmt(a['presentation'])}")
    L.append(f"exports: {fmt(a.get('exports'))} completion: {a.get('completion')}")
    L.append("")
    r = a["rig"]
    L.append(f"## rig kind={r['kind']} root={r['root']}")
    msum = 0.0
    for b in r["bones"]:
        m = b.get("mass")
        if m:
            msum += m["kg"]
        L.append(f"- bone {b['id']} parent={b.get('parent')} x={b['x']} y={b['y']} ang={b.get('angleDeg')} len={b.get('lengthM')} joint={fmt(b.get('joint'))} mass={fmt(m)} title={fmt(b.get('title'))}")
    L.append(f"mass sum={msum:.3f} vs massKg={a['dimensions']['massKg']}")
    L.append("## anchors")
    for an in a.get("anchors", []):
        L.append(f"- {fmt(an)}")
    L.append("")
    for key, rep in a["representations"].items():
        L.append(f"## representation {key} status={rep.get('status')} validRange={fmt(rep.get('validRange'))} pens={fmt(rep.get('pens'))} hatches={fmt(rep.get('hatches'))}")
        if rep.get("status") != "authored":
            L.append(f"unavailable: {fmt(rep)}")
            continue
        g = rep.get("geometry", {})
        for view, shapes in g.items():
            L.append(f"### {view}: {len(shapes)} shapes")
            for s in shapes:
                parts = [f"id={s.get('id')}", f"kind={s.get('kind')}", f"bone={s.get('bone')}", f"role={s.get('role')}", f"z={s.get('z')}", f"filled={s.get('filled')}", f"tone={s.get('tone')}"]
                if "region" in s: parts.append(f"region={s['region']}")
                if "regions" in s: parts.append(f"regions={fmt(s['regions'])}")
                if "material" in s: parts.append(f"material={s['material']}")
                if "penMm" in s: parts.append(f"penMm={s['penMm']}")
                for k in ("x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "rot", "angleDeg", "x1", "y1", "x2", "y2", "lengthM"):
                    if k in s: parts.append(f"{k}={s[k]}")
                if "d" in s: parts.append(f"d={short(s['d'], 90)}")
                if "points" in s: parts.append(f"points[{len(s['points'])}]")
                extra = {k: v for k, v in s.items() if k not in ("id","kind","bone","role","z","filled","tone","region","regions","material","penMm","x","y","w","h","r","cx","cy","rx","ry","rot","angleDeg","x1","y1","x2","y2","lengthM","d","points")}
                if extra: parts.append(f"extra={short(fmt(extra), 200)}")
                L.append("  " + " ".join(parts))
        if "symbols" in rep:
            L.append(f"symbols: {fmt(rep['symbols'])}")
        for dk, dv in rep.get("decisions", {}).items():
            L.append(f"decision.{dk}.en: {dv.get('en')}")
            L.append(f"decision.{dk}.de: {dv.get('de')}")
        L.append("")
    th = a["thermal"]
    L.append(f"## thermal env={fmt(th['environment'])}")
    for k in ("baseMet", "body", "respiration", "passengers"):
        if k in th: L.append(f"{k}: {short(fmt(th[k]), 1200)}")
    for op in th.get("operating", []):
        L.append(f"operating {op['id']}: {fmt(op['levels'])}")
    for rg in th.get("regions", []):
        L.append(f"region {rg['id']} kind={rg['kind']} model={rg['model']} mat={rg['material']} area={rg['areaM2']} Lc={rg['characteristicLengthM']} geom={rg['geometry']} sky={rg['skyViewFactor']} Cp={rg['heatCapacityJM2K']} body={fmt(rg.get('body'))} leaf={fmt(rg.get('leaf'))} operating={fmt(rg.get('operating'))} conditioned={fmt(rg.get('conditioned'))} fixed={fmt(rg.get('fixed'))} hidden={fmt(rg.get('hidden'))} src={rg.get('source')} ref={rg.get('reference')}")
        for ly in rg.get("layers", []):
            L.append(f"    layer {ly['id']} kind={ly['kind']} mat={ly['material']} th={ly.get('thicknessM')} R={ly['resistanceM2KW']} Rv={ly.get('vapourResistanceM2PaW')} Cp={ly.get('heatCapacityJM2K')} src={ly['source']} ref={ly.get('reference')}")
    for em in th.get("emitters", []):
        L.append(f"emitter {fmt(em)}")
    L.append("")
    for p in a["poses"]:
        th2 = p.get("thermal", {})
        L.append(f"## pose {p['id']} key={p.get('key')} bones={fmt(p.get('bones'))} ground={p.get('ground')} contacts={p.get('contacts')} loads={fmt(p.get('loads'))} ballistic={p.get('ballistic')}")
        L.append(f"   thermal.regions={fmt(th2.get('regions'))}")
        L.append(f"   thermal.emitters={fmt(th2.get('emitters'))} operating={fmt(th2.get('operating'))} other={ {k:v for k,v in th2.items() if k not in ('regions','emitters','operating')} }")
        L.append(f"   en: {t(p.get('description', {}))}")
    L.append("")
    for c in a.get("clips", []):
        L.append(f"## clip {c['id']} dur={c['durationS']} fps={c['fps']} loop={c['loop']} root={c.get('rootMotion')} advance={c.get('advanceM')}")
        L.append(f"   keys: {fmt([ (k['t'], k.get('pose'), k.get('ease'), {kk:vv for kk,vv in k.items() if kk not in ('t','pose','ease')}) for k in c['keys']])}")
        L.append(f"   contacts={fmt(c.get('contacts'))}")
        L.append(f"   phases={fmt([ (p['id'], p['fromS'], p['toS']) for p in c.get('phases', [])])}")
        L.append(f"   events={fmt([ (e['kind'], e['t'], e.get('target'), t(e.get('title'))) for e in c.get('events', [])])}")
        for tr in c.get("tracks", []):
            L.append(f"   track {tr['target']} ease={tr.get('ease')} keys={fmt([(k['t'], k['v']) for k in tr['keys']])}")
        L.append(f"   en: {t(c.get('description', {}))}")
    L.append("")
    s = a["structure"]
    L.append(f"## structure {fmt(s)}")
    refs = a["provenance"]["references"]
    L.append("## references")
    for rf in refs:
        L.append(f"- {fmt(rf)}")
    return "\n".join(L)

if __name__ == "__main__":
    src = sys.argv[1]
    print(sheet(src))
