"""One continuous coat silhouette: neck coat regions merged into the trunk coat so the depiction knockout hides the inner outlines at the neck base."""
import importlib.util, json, re
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/"
spec = importlib.util.spec_from_file_location("lib", T + "🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc()
def rekey(o, old, new):
    if isinstance(o, dict):
        out = {}
        for k, v in o.items():
            nk = new if k == old else k
            if nk in out and isinstance(out[nk], dict) and isinstance(v, dict): out[nk] = {**v, **out[nk]}
            elif nk not in out: out[nk] = rekey(v, old, new)
        return out
    if isinstance(o, list): return [rekey(x, old, new) for x in o]
    return o
def merge(asset, main, tgt):
    D.edit(asset, "geometry", lambda o: o.get("region") == tgt, lambda o: {**o, "region": main})
    rs = {o["id"]: o for o in D.get(asset, "thermal", lambda o: o.get("id") in (main, tgt) and "areaM2" in o)}
    if tgt not in rs: return
    m, t = rs[main], rs[tgt]; a = m["areaM2"] + t["areaM2"]
    def upd(o):
        w = lambda k: round((m[k] * m["areaM2"] + t[k] * t["areaM2"]) / a, 4)
        return {**o, "areaM2": round(a, 4), "metabolicShare": round(m["metabolicShare"] + t["metabolicShare"], 4), "characteristicLengthM": w("characteristicLengthM"), "insulationM2KW": w("insulationM2KW"), "coreTempC": w("coreTempC")}
    D.edit(asset, "thermal", lambda o: o.get("id") == main and "areaM2" in o, upd)
    D.edit(asset, "thermal", lambda o: o.get("id") == tgt and "areaM2" in o, lambda o: None)
    D.edit(asset, "poses", lambda o: tgt in json.dumps(o), lambda o: rekey(o, tgt, main))
    cur = None; sec = None
    for i, l in enumerate(D.lines):
        mm = re.match(r'^      "id": "([^"]+)",$', l)
        if mm: cur = mm.group(1)
        if cur == asset and f'region:{tgt}:' in l: D.lines[i] = l.replace(f'region:{tgt}:', f'region:{main}:')
for asset, main, tgt in (("labrador-dog", "coat-trunk", "coat-head-neck"), ("domestic-cat", "coat-trunk", "coat-head-neck"), ("roe-deer", "coat-trunk", "coat-head-neck"), ("horse", "coat-trunk", "coat-neck")): merge(asset, main, tgt)
D.save()
