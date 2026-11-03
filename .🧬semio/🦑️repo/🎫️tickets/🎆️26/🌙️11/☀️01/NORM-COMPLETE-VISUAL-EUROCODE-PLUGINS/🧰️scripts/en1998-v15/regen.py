"""Lossless refresh of the en1998 snapshot/diff/mutation fixtures against the current schema: adds missing required properties with the documented defaults, orders keys by schema order, fixes the typed-enum spelling of mutation payloads. Dry run by default; --apply writes. python -I regen.py [--apply]"""
import glob, json, os, sys
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import validate
from validate import ANY, FX, load, kind_of
DEFAULTS = {"enGroundType": "b", "enSpectrumType": "type1", "wallLengthM": 0.0, "wallDirectionRad": 0.0, "wallStoreyId": "", "wallStartXM": 0.0, "wallStartYM": 0.0, "wallPoissonRatio": 0.25, "effectiveStiffnessBasis": "declared", "effectiveStiffnessRatio": 1.0}
EMPTY_DELTA = lambda: {"removed": [], "inserted": [], "moved": [], "modified": []}
SNAP = load(ANY + "🧬️schema/📸️snapshot/🔣️.json")
def deref(schema, root):
    while "$ref" in schema:
        url, _, pointer = schema["$ref"].partition("#")
        if url: root = SNAP
        node = root
        for part in [x for x in pointer.split("/") if x]: node = node[part]
        schema = node
    return schema, root
def default_for(name, prop, root):
    if name in DEFAULTS: return DEFAULTS[name]
    ref = prop.get("$ref", "")
    if ref.endswith("Delta"): return EMPTY_DELTA()
    if "anyOf" in prop and any(a.get("type") == "null" for a in prop["anyOf"]): return None
    raise KeyError(name)
def conform(doc, schema, root):
    schema, root = deref(schema, root)
    if "anyOf" in schema and doc is not None:
        for a in schema["anyOf"]:
            if a.get("type") != "null": return conform(doc, a, root)
    if isinstance(doc, list) and "items" in schema: return [conform(x, schema["items"], root) for x in doc]
    if isinstance(doc, dict) and "properties" in schema:
        props = schema["properties"]; out = {k: conform(v, props[k], root) if k in props else v for k, v in doc.items()}
        names = list(props)
        for i, k in enumerate(names):
            if k in out or k not in schema.get("required", []): continue
            before = next((names[j] for j in range(i - 1, -1, -1) if names[j] in out), None)
            items = list(out.items()); at = 0 if before is None else [x for x, _ in items].index(before) + 1
            items.insert(at, (k, default_for(k, props[k], root))); out = dict(items)
        return out
    return doc
def dump(d): return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
def main(apply):
    snap, diff, kinds, reg = validate.build(); changed = []; skipped = 0
    def handle(p, schema):
        s = open(p, encoding="utf-8").read(); n = dump(conform(json.loads(s), schema, schema))
        if n != s: changed.append(p); apply and open(p, "w", encoding="utf-8").write(n)
    for d in glob.glob(FX + "🧬️mutations/*/*"):
        for part, schema in (("before", snap), ("after", snap), ("diff", diff)):
            p = d + "/🔣️." + part + ".json"
            if os.path.exists(p): handle(p, schema)
        p = d + "/🔣️.mutation.json"; m = load(p)
        if m.get("newAnnex") == "en": changed.append(p); apply and open(p, "w", encoding="utf-8").write(dump({**m, "newAnnex": "En"}))
    for p in glob.glob(FX + "🚦️check-outcome/*/🔣️.json") + glob.glob(FX + "🎪️showcase/*.json"): handle(p, snap)
    print(("applied" if apply else "dry run"), len(changed), "files")
if __name__ == "__main__": main("--apply" in sys.argv)
