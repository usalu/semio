"""Harness, oracle and taxonomy registration of the seven case kinds (flavour of EN 1997: rs KINDS + fixture_text arms, py KINDS + VECTORS, feature tables)."""
import copy, json, re
from lib import *


def register(art, template_kind, template_dir, harness_dir):
    h = harness_dir
    p = h + "/🦀️.rs"
    s = rd(p)
    i = s.index("const KINDS: &[&str] = &[")
    j = s.index("\n];", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k for k in KINDS).rstrip("\n") + s[j:]
    arms = ""
    for k in KINDS:
        arms += '        "%s" => (\n' % k + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (LEAF_DIRS[k], x) for x in ["before", "mutation", "after", "outcome"]) + "        ),\n"
    a = '        other => panic!("mutate-%s-1: no committed fixture' % art.short
    s = s.replace(a, arms + a, 1)
    n = len(re.findall(r'^    "[a-z0-9-]+",$', s[s.index("const KINDS"): s.index("\n];", s.index("const KINDS"))], re.M))
    s = re.sub(r"current \d+-kind", "current %d-kind" % n, s, 1)
    wr(p, s)
    p = h + "/🐍️.py"
    s = rd(p)
    i = s.index("KINDS = [")
    j = s.index("\n]", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k for k in KINDS).rstrip("\n") + s[j:]
    row = '    "%s": ("%s", "✅apply"),\n' % (template_kind, template_dir)
    assert row in s
    s = s.replace(row, row + "".join('    "%s": ("%s", "✅apply"),\n' % (k, LEAF_DIRS[k]) for k in KINDS), 1)
    wr(p, s)
    p = h + "/🥒️.feature"
    s = rd(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (k, LEAF_DIRS[k]) for k in KINDS)
    s, c = re.subn(r"(      \| %s \| [^\n]*\n)" % re.escape(template_kind), lambda m: m.group(1) + rows, s)
    assert c >= 1
    wr(p, s)


def oracle(art, template_kind):
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    have = {v["mutationId"] for v in cat["vectors"]}
    vt = next(v for v in cat["vectors"] if v["mutationId"] == template_kind)
    mt = next(m for m in man["mutations"] if m["id"] == template_kind)
    for k in KINDS:
        if k in have:
            continue
        v = copy.deepcopy(vt)
        v["mutationId"] = k
        v["sourceMutationDirectoryName"] = v["mutationDirectoryName"] = LEAF_DIRS[k]
        cat["vectors"].append(v)
        if "kinds" in cat:
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
    last = max(i for i, x in enumerate(t["rows"]) if x["artifact"] == name)
    have = {r["kind"] for r in t["rows"] if r["artifact"] == name}
    verb = lambda k: {"insert": "insert", "remove": "remove", "set": "set", "change": "change"}[k.split("-")[0]]
    rec = lambda k: {"insert": "Inserted", "remove": "Removed", "set": "Set", "change": "Changed"}[k.split("-")[0]] + RECORD[k][len(k.split("-")[0]):]
    new = [{"aggregateVariant": RECORD[k], "artifact": name, "entity": k.split("-", 1)[1], "kind": k, "module": LEAF_DIRS[k], "physicalLayout": "direct", "record": rec(k), "source": "%s/%s/%s/🦀️.rs" % (name, art.rel_mut, LEAF_DIRS[k]), "standard": "🔖️1", "subset": "✳️any", "type": RECORD[k], "verb": verb(k)} for k in KINDS if k not in have]
    new.sort(key=lambda r: r["module"])
    t["rows"][last + 1: last + 1] = new
    wr(TAXONOMY, json.dumps(t, indent=2, ensure_ascii=False) + "\n")
