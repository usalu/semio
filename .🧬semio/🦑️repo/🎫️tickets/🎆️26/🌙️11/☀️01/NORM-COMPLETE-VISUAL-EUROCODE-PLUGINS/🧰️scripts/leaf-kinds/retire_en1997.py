"""Retires the 11 whole-row `update-<row>` kinds of en1997 through every layer, rewrites `resolve` to the typed rules and extends the unit-test samples with every typed kind (run once, after en1997.py, en1997_structure.py and en1997_nested.py)."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import en1997 as E, en1997_nested as N, model as M, wire as W

art = E.art
ROWS = ["layer", "footing", "pile", "retaining-wall", "slope", "uplift-case", "anchor", "embedded-wall", "pile-group", "lab-test", "layer-water"]
KINDS = ["update-" + r for r in ROWS]
MODS = [k.replace("-", "_") for k in KINDS]
RECS = [pascal(k) for k in KINDS]
CAML = [r[:1].lower() + r[1:] for r in RECS]


def sub_lines(t, pred):
    return "".join(l for l in t.splitlines(True) if not pred(l))


def has(l, names):
    return any(re.search(r"(?<![A-Za-z0-9_-])%s(?![A-Za-z0-9_-])" % re.escape(n), l) for n in names)


def rw(path, fn):
    t = rd(path)
    u = fn(t)
    if u != t:
        open(path, "w", encoding="utf-8").write(u)


def rust_sample(l, demo_var="demo"):
    ent = l.ent
    if len(ent.chain) == 1:
        sels = "%s: %s.%s[0].id.clone()" % (ent.sels[0], demo_var, ent.chain[0][0])
    else:
        pc = ent.chain[0][0]
        sels = "%s: nested_%s.id.clone(), id: nested_%s.%s[0].id.clone()" % (ent.sels[0], pc, pc, ent.chain[1][0])
    val = {"f64": "1.5", "u32": "2", "bool": "true"}.get(l.ty, '"alt".into()')
    return "        En1997Mutation::%s(%s::%s { %s, %s: %s }),\n" % (l.record, l.module, l.record, sels, l.new, val)


def samples(leaves, ks):
    out = "fn typed_samples(demo: &semio_s_artifact_norm_en1997_model::En1997Snapshot) -> Vec<En1997Mutation> {\n"
    for coll in ["footings", "piles", "anchors", "lab_tests"]:
        nested = {"footings": "load_cases", "piles": "test_profiles", "anchors": "tests", "lab_tests": "points"}[coll]
        out += '    let nested_%s = demo.%s.iter().find(|row| !row.%s.is_empty()).expect("demo row with nested rows");\n' % (coll, coll, nested)
    out += "    vec![\n"
    for l in leaves:
        out += rust_sample(l)
    out += "        En1997Mutation::ChangeStructureId(change_structure_id::ChangeStructureId { new_structure_id: \"alt\".into() }),\n"
    for k in ks:
        ps = "nested_%s" % k.pcoll
        if k.verb == "insert":
            out += "        En1997Mutation::%s(%s::%s { %s: %s.id.clone(), index: 0, %s: { let mut row = %s.%s[0].clone(); row.id = \"new\".into(); row } }),\n" % (k.record, k.module, k.record, k.psel, ps, k.row_snake, ps, k.ncoll)
        else:
            out += "        En1997Mutation::%s(%s::%s { %s: %s.id.clone(), index: 0 }),\n" % (k.record, k.module, k.record, k.psel, ps)
    return out + "    ]\n}\n\n"


def main():
    leaves, defs = M.read_entities(art.any + "/🧬️schema/📸️snapshot/🔣️.json", E.ENTS, E.COVERED)
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M))
    new = [l for l in leaves if not l.covered and l.kind in existing]
    ks = [N.K(v, r) for r in N.NEST for v in ("insert", "remove")]
    for d in glob.glob(art.mut + "/*update-*") + glob.glob(art.fix + "/*update-*"):
        if re.sub(r"^[^a-z]+", "", os.path.basename(d)) in KINDS:
            shutil.rmtree(d)
    for crate in ["rows", "structures", "investigation"]:
        d = next(x for x in os.listdir(art.mut) if x.endswith("️" + crate))
        rw(art.mut + "/" + d + "/🦀️.rs", lambda t: re.sub(r'#\[path = "\.\./[^"]*update-[a-z-]+/🦀️\.rs"\]\npub mod update_\w+;\n', "", t))
    def root(t):
        t = sub_lines(t, lambda l: l.startswith("use ") and has(l, MODS))
        t = sub_lines(t, lambda l: re.match(r"    (%s)\(" % "|".join(RECS), l))
        return sub_lines(t, lambda l: re.match(r'    "(%s)",$' % "|".join(KINDS), l))
    rw(art.mut + "/🦀️.rs", root)
    rw(art.mut + "/🟦️.ts", lambda t: sub_lines(t, lambda l: has(l, RECS) and (l.startswith("import") or l.lstrip().startswith("|")) or re.match(r"  (%s):" % "|".join(CAML), l)))
    def js(t):
        d = json.loads(t)
        d["oneOf"] = [x for x in d["oneOf"] if not any(x["$ref"].endswith("/%s/schema.json" % k) for k in KINDS)]
        return dumps(d)
    rw(art.mut + "/🔣️.json", js)
    rw(art.mut + "/🔗️.graphql", lambda t: sub_lines(t, lambda l: re.match(r"  \| (%s)$" % "|".join(RECS), l) or re.match(r"type (%s) \{" % "|".join(RECS), l)))
    tp = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    def text(t):
        t = re.sub(r"(?m)^    (?:%s) \{\n        #\[dsl\(block\)\]\n        \w+: \w+,\n    \},\n" % "|".join(RECS), "", t)
        t = sub_lines(t, lambda l: re.match(r"        En1997Mutation(?:Dsl)?::(%s)\b" % "|".join(RECS), l))
        t = sub_lines(t, lambda l: re.match(r"        En1997MutationDsl::(%s)\b" % "|".join(RECS), l))
        t = sub_lines(t, lambda l: re.match(r"        En1997Mutation::(%s)\(" % "|".join(RECS), l))
        for m in MODS:
            t = re.sub(r"\b%s, " % m, "", t)
            t = re.sub(r", %s\b" % m, "", t)
        t = re.sub(r"(?m)^use [\w:]+::\{%s\};\n" % "|".join(MODS), "", t)
        return sub_lines(t, lambda l: l.startswith("use ") and has(l, MODS))
    rw(tp, text)
    for p in [art.any + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]:
        rw(p, lambda t: [t := re.sub(r"\b%s, " % m, "", re.sub(r", %s\b" % m, "", t)) for m in MODS][-1])
    h = art.harness
    def hrs(t):
        t = re.sub(r'        "(?:%s)" => \(\n(?:            [^\n]*\n)+        \),\n' % "|".join(KINDS), "", t)
        return sub_lines(t, lambda l: re.match(r'    "(%s)",$' % "|".join(KINDS), l))
    rw(h + "/🦀️.rs", hrs)
    rw(h + "/🐍️.py", lambda t: sub_lines(t, lambda l: re.match(r'    "(%s)"[:,]' % "|".join(KINDS), l)))
    rw(h + "/🥒️.feature", lambda t: sub_lines(t, lambda l: re.match(r"      \| (%s) +\|" % "|".join(KINDS), l)))
    def orc(t):
        o = json.loads(t)
        c, m = o["mutationCatalogs"][0], o["mutationManifests"][0]
        c["vectors"] = [v for v in c["vectors"] if v["mutationId"] not in KINDS]
        c["kinds"] = [k for k in c["kinds"] if k not in KINDS]
        m["mutations"] = [x for x in m["mutations"] if x["id"] not in KINDS]
        return dumps(o)
    rw(art.any + "/🔮️oracles/🔣️.json", orc)
    kinds = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M)
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    t = rd(p)
    open(p, "w", encoding="utf-8").write(t[: t.index("record ")] + "".join("record %s tag=%d\nfield payload bytes\n" % (k, i) for i, k in enumerate(kinds)))
    for f in glob.glob(art.mut + "/*/🔣️.json"):
        j = json.loads(rd(f))
        if j.get("semanticKind") in kinds:
            j["binaryTag"] = kinds.index(j["semanticKind"])
            open(f, "w", encoding="utf-8").write(dumps(j))
    ep = art.mut + "/🧭️edit-rules/🦀️.rs"
    def edit(t):
        t = re.sub(r"use semio_framework_value::\{DslValue, FromValue, ToValue\};\n", "", t)
        t = re.sub(r"use semio_s_plugin_norm_app_surface::\{[^}]*\};", "use semio_s_plugin_norm_app_surface::{InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};", t)
        t = re.sub(r"use crate::\{update_[^}]*\};\n", "use crate::En1997Mutation;\n", t)
        i = t.index("/// 🆔️ A collection whose remove kind")
        return t[:i] + "/// 🎯️ Resolves an editor edit into the concrete typed kind mutation its [`EDIT_RULES`] rule names; every leaf and every row list has its own rule, there is no whole-row fallback.\npub fn resolve(document: &En1997Snapshot, edit: &NormEdit) -> Result<Vec<En1997Mutation>, String> {\n    EDIT_RULES.resolve(document, edit)\n}\n"
    rw(ep, edit)
    block = samples(new, ks)
    for tp in [art.mut + "/🧪️tests/🔬️unit/🦀️.rs", art.any + "/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]:
        def tests(t):
            t = sub_lines(t, lambda l: re.match(r"        En1997Mutation::(%s)\(" % "|".join(RECS), l))
            if "fn typed_samples" not in t:
                t = rep(t, "fn every_mutation() -> Vec<En1997Mutation> {", block + "fn every_mutation() -> Vec<En1997Mutation> {", 1)
                t = rep(t, "    vec![\n        En1997Mutation::ChangeAnnex", "    let mut all = vec![\n        En1997Mutation::ChangeAnnex", 1)
                i = t.index("fn every_mutation()")
                j = t.index("\n    ]\n}\n", i)
                t = t[:j] + "\n    ];\n    all.extend(typed_samples(&demo));\n    all\n}\n" + t[j + len("\n    ]\n}\n"):]
            return t
        rw(tp, tests)
    print("kinds left", len(kinds))


main()
