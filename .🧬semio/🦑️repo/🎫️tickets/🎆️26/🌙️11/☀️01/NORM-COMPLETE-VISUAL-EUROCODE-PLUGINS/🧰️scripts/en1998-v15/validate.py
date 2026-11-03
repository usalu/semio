"""Validates every en1998 mutation fixture (payload, before, after, diff) and check-outcome/showcase snapshot against its schema. python -I validate.py [--summary]"""
import glob, json, os, sys, collections
import jsonschema
from referencing import Registry, Resource
ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"
FX = ANY + "🧪️tests/🧫️fixtures/"
def load(p): return json.load(open(p, encoding="utf-8"))
def kind_of(folder):
    n = os.path.basename(folder); return n[next(i for i, c in enumerate(n) if c.isascii() and c.isalpha()):]
def build():
    snap = load(ANY + "🧬️schema/📸️snapshot/🔣️.json"); diff = load(ANY + "🧬️schema/🔺️diff/🔣️.json")
    res = [(snap["$id"], Resource.from_contents(snap)), (diff["$id"], Resource.from_contents(diff))]
    kinds = {}
    for f in glob.glob(ANY + "🧬️schema/🧬️mutations/*/🧬️schema/🔣️.json"):
        s = load(f); res.append((s["$id"], Resource.from_contents(s))); kinds[kind_of(os.path.dirname(os.path.dirname(f)))] = s
    return snap, diff, kinds, Registry().with_resources(res)
def problems(schema, registry, doc):
    v = jsonschema.Draft7Validator(schema, registry=registry)
    return sorted({"/".join(str(x) for x in e.absolute_path if isinstance(x, str)) + ": " + e.message[:90] for e in v.iter_errors(doc)})
def run():
    snap, diff, kinds, reg = build(); out = collections.defaultdict(list)
    for d in glob.glob(FX + "🧬️mutations/*/*"):
        k = kind_of(os.path.dirname(d))
        for part, schema in (("mutation", kinds.get(k)), ("before", snap), ("after", snap), ("diff", diff)):
            p = d + "/🔣️." + part + ".json"
            if not os.path.exists(p): continue
            if schema is None: out[p].append("no schema for kind"); continue
            for m in problems(schema, reg, load(p)): out[p].append(m)
    for p in glob.glob(FX + "🚦️check-outcome/*/🔣️.json") + glob.glob(FX + "🎪️showcase/*.json"):
        for m in problems(snap, reg, load(p)): out[p].append(m)
    return out
if __name__ == "__main__":
    out = run(); c = collections.Counter(m for ms in out.values() for m in ms)
    print("files with problems:", len(out))
    for m, n in c.most_common(30): print(n, m)
    sys.exit(1 if out else 0)
