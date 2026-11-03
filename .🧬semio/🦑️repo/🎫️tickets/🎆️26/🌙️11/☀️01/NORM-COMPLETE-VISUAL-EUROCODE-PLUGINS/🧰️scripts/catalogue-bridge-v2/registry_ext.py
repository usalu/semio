"""Fork-local registration of the seven case kinds in oracle manifest, taxonomy and feature tables."""
import copy, json, re
from lib import *


def oracle(art, template_kind):
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    vt = [v for v in cat["vectors"] if v["mutationId"] == template_kind][0]
    mt = [m for m in man["mutations"] if m["id"] == template_kind][0]
    have = {v["mutationId"] for v in cat["vectors"]}
    for k in KINDS:
        if k in have:
            continue
        v = copy.deepcopy(vt)
        v["mutationId"] = k
        v["sourceMutationDirectoryName"] = v["mutationDirectoryName"] = LEAF_DIRS[k]
        cat["vectors"].append(v)
        cat["kinds"].append(k)
        m = copy.deepcopy(mt)
        m["id"] = k
        m["productionDispatch"]["operation"] = k
        m["productionDispatch"]["variant"] = RECORD[k]
        man["mutations"].append(m)
    wr(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")


def taxonomy(art):
    t = json.loads(rd(TAXONOMY))
    name = art.emoji + art.short
    rel = art.rel_mut
    last = max(i for i, x in enumerate(t["rows"]) if x["artifact"] == name)
    new = []
    for k in KINDS:
        d = re.search(r'verb: "(\w+)", entity: "([\w-]+)", kind: "[\w-]+", record: "(\w+)"', rd(art.mut + "/" + LEAF_DIRS[k] + "/🦀️.rs"))
        new.append({"aggregateVariant": RECORD[k], "artifact": name, "entity": d.group(2), "kind": k, "module": LEAF_DIRS[k], "physicalLayout": "direct", "record": d.group(3), "source": "%s/%s/%s/🦀️.rs" % (name, rel, LEAF_DIRS[k]), "standard": "🔖️1", "subset": "✳️any", "type": RECORD[k], "verb": d.group(1)})
    have = {r["kind"] for r in t["rows"] if r["artifact"] == name}
    t["rows"][last + 1: last + 1] = [r for r in new if r["kind"] not in have]
    wr(TAXONOMY, json.dumps(t, indent=2, ensure_ascii=False) + "\n")


def feature(art, harness_dir, after_kind):
    p = harness_dir + "/🥒️.feature"
    s = rd(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (k, LEAF_DIRS[k]) for k in KINDS)
    pat = re.compile(r"(      \| %s [^\n]*\n)" % re.escape(after_kind))
    assert pat.search(s), after_kind
    wr(p, pat.sub(lambda m: m.group(1) + rows, s))
