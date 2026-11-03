"""EN 1997 profile and driver (macro diff, id selectors, nested lanes): python -I en1997.py [--apply]."""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import leaf as L, diff as D, wire as W, model as M, fixtures as X, diff97 as D7

SHORT = "en1997"
PRIVATE_STAGE = TICKET + "/🗑️generated/leaf-kinds-en1997-stage"
dirname = next(n for n in os.listdir(NORM + "/🗿️artifacts") if n.endswith(SHORT))
art = W.Art(SHORT, dirname[: -len(SHORT)], dirname[: -len(SHORT)], None, None)
art.anchor_crate = "semio-s-artifact-norm-en1997-mut-ground"
art.anchor_dir = next(n for n in os.listdir(art.mut) if n.endswith("ground"))
L.init(SHORT)
E = M.Ent
FO = ("footings", "SpreadFoundation", "footing_id")
PI = ("piles", "Pile", "pile_id")
AN = ("anchors", "Anchor", "anchor_id")
LT = ("lab_tests", "LabTest", "lab_test_id")
ENTS = [
    E("layer", "🥞️", [("layers", "SoilLayer", "id")], "Soil layer", "Bodenschicht"),
    E("footing", "🏛️", [("footings", "SpreadFoundation", "id")], "Footing", "Fundament"),
    E("load-case", "🏛️", [FO, ("load_cases", "FoundationLoadCase", "id")], "Footing load case", "Fundament-Lastfall"),
    E("pile", "🔄️", [("piles", "Pile", "id")], "Pile", "Pfahl"),
    E("profile", "🔄️", [PI, ("test_profiles", "PileTestProfile", "id")], "Pile test profile", "Pfahl-Probebelastungsprofil"),
    E("wall", "🗄️", [("retaining_walls", "RetainingWall", "id")], "Retaining wall", "Stützwand"),
    E("slope", "⛰️", [("slopes", "Slope", "id")], "Slope", "Böschung"),
    E("uplift", "⬆️", [("uplift_cases", "UpliftCase", "id")], "Uplift case", "Auftriebsfall"),
    E("anchor", "⚓️", [("anchors", "Anchor", "id")], "Anchor", "Anker"),
    E("anchor-test", "⚓️", [AN, ("tests", "AnchorTest", "id")], "Anchor test", "Ankerprüfung"),
    E("embedded-wall", "🧱", [("embedded_walls", "EmbeddedWall", "id")], "Embedded wall", "Eingespannte Wand", slug="emb-wall"),
    E("pile-group", "🏗️", [("pile_groups", "PileGroup", "id")], "Pile group", "Pfahlgruppe"),
    E("lab-test", "🧪️", [("lab_tests", "LabTest", "id")], "Laboratory test", "Laborversuch"),
    E("lab-point", "🧪️", [LT, ("points", "LabPoint", "id")], "Laboratory test point", "Laborversuchspunkt"),
    E("layer-water", "💧", [("layer_waters", "LayerWater", "id")], "Layer water", "Schichtwasser", slug="water"),
    E("ground-investigation", "🕳️", [("investigations", "Investigation", "id")], "Ground investigation", "Baugrunderkundung"),
    E("ground-investigation-reading", "📈", [("investigations", "Investigation", "investigation_id"), ("readings", "InvestigationReading", "id")], "Investigation reading", "Erkundungsmesswert"),
    E("characteristic-basis", "🧮", [("characteristic_bases", "CharacteristicBasis", "id")], "Characteristic value basis", "Grundlage des charakteristischen Werts"),
]
COVERED = {("layer", "phi_prime_deg"), ("layer", "oedometric_modulus"), ("footing", "width"), ("footing", "embedment"), ("pile", "length"), ("pile", "count"), ("wall", "base_width"), ("slope", "angle_deg")}
L.TARGET_DE.update({"characteristic_basis_id": "Grundlage des charakteristischen Werts", "investigation_id": "Baugrunderkundung", "id": "Zeile", "footing_id": "Fundament", "pile_id": "Pfahl", "anchor_id": "Anker", "lab_test_id": "Laborversuch"})
M.ABBREV += [("inclination", "incl"), ("pressure", "press"), ("cohesion", "coh"), ("undrained", "undr"), ("consolidation", "consol"), ("horizontal", "horiz"), ("permanent", "perm"), ("variable", "var"), ("resistance", "res"), ("subgrade", "bed"), ("negative", "neg"), ("secondary", "sec"), ("dynamic", "dyn"), ("compression", "compr")]
W.FAMILY.update({("layer", 1): "investigation", ("pile", 1): "tension", ("wall", 1): "loads", ("anchor", 1): "proof", ("embedded-wall", 1): "pressure", ("pile-group", 1): "resistance"})
SUFFIX = "/✅apply/🔣️.before.json"
M.LIMIT[0] = 255 - len(art.fix[len(SEMIO) + 1:].encode()) - 1 - len(SUFFIX.encode()) - 8
CLONE_STEM, CLONE_ROW = "En1997Layers", "SoilLayer"
RICH = art.fix + "/➖remove-footing/🧭️middle/🔣️.before.json"


def walk97(leaf, var="base"):
    e, out, src = leaf.ent, [], var
    for i, (coll, _, sel) in enumerate(e.chain):
        what = e.slug if i == len(e.chain) - 1 else coll[:-1].replace("_", "-")
        out.append('    let Some(r%d) = %s.%s.iter().find(|row| row.id == payload.%s) else {\n        return protocol::MutationOutcome::error("mutation.target-missing", format!("%s {} missing", payload.%s), vec![payload.%s.clone()]);\n    };' % (i, src, coll, sel, what, sel, sel))
        src = "r%d" % i
    return out, src


def diff_rs97(leaf):
    e, P = leaf.ent, L.Pfx()
    lets, last = walk97(leaf)
    val = "payload.%s%s" % (leaf.new, "" if leaf.copy else ".clone()")
    names = {"%s%sPatch" % (P, e.patch), "%s%sRows" % (P, e.patch)}
    expr = "%s%sRows::modification(&payload.%s, %s%sPatch { %s: Some(%s), ..Default::default() })" % (P, e.patch, e.chain[-1][2], P, e.patch, leaf.field, val)
    for i in range(len(e.chain) - 2, -1, -1):
        pname = "".join(pascal(c[0]) for c in e.chain[: i + 1])
        names |= {"%s%sPatch" % (P, pname), "%s%sRows" % (P, pname)}
        expr = "%s%sRows::modification(&payload.%s, %s%sPatch { %s: %s, ..Default::default() })" % (P, pname, e.chain[i][2], P, pname, e.chain[i + 1][0], expr)
    guard = ""
    if leaf.ty == "f64":
        guard = '    if !payload.%s.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n' % leaf.new
    return '''//! 🔺️ `%(kind)s` diff: patches `%(access)s` of the addressed %(ent)s row.

use crate::%(mod)s::%(rec)s;
use %(diff)s::{%(imports)s};
use %(model)s::%(S)s;

pub fn diff(payload: &%(rec)s, base: &%(S)s) -> protocol::MutationOutcome<%(P)sDiff> {
%(guard)s%(lets)s
    if %(last)s.%(access)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The %(ent)s already has this value.");
    }
    protocol::MutationOutcome::new(%(P)sDiff { %(top)s: Some(%(expr)s), ..Default::default() })
}
''' % dict(kind=leaf.kind, access=leaf.access, ent=e.en.lower(), mod=leaf.module, rec=leaf.record, diff=L.crate_diff(), imports=", ".join(sorted(names | {P + "Diff"})), model=L.crate_model(), S=L.snap(), P=P, guard=guard, lets="\n".join(lets), last=last, new=leaf.new, top=e.chain[0][0], expr=expr)


BASE_SCHEMA_JSON = L.schema_json


def schema_json97(leaf, defs):
    j = BASE_SCHEMA_JSON(leaf, defs)
    e = leaf.ent
    for i, (coll, d, sel) in enumerate(e.chain):
        p = j["properties"][camel(sel)]
        own = i == len(e.chain) - 1
        en, de = (e.en, e.de) if own else (coll[:-1].replace("_", " ").capitalize(), L.TARGET_DE[sel])
        ui = p["x-semio-ui"]
        ui["label"] = {"en": en, "de": de}
        ui["description"] = {"en": "The %s this mutation addresses, by id." % en.lower(), "de": "Die adressierte Zeile (%s), per Kennung." % de}
        ui["ref"] = {"kind": kebab(snake(d[:1].lower() + d[1:]))}
    return j


L.walk = walk97
L.diff_rs = diff_rs97
L.schema_json = schema_json97


def reg_lock(path, names, template, dependents):
    t = rd(path)
    blocks = re.split(r"(?m)^(?=\[\[package\]\]\n)", t)
    head, pk = blocks[0], blocks[1:]
    tpl = next(b for b in pk if b.startswith('[[package]]\nname = "%s"\n' % template))
    have = {re.search(r'name = "([^"]+)"', b).group(1) for b in pk}
    for n in names:
        if n not in have:
            pk.append(tpl.replace(template, n))
    out = []
    for b in pk:
        nm = re.search(r'name = "([^"]+)"', b).group(1)
        if nm in dependents:
            deps = re.search(r"dependencies = \[\n(.*?)\]\n", b, re.S)
            items = set(re.findall(r'"([^"]+)"', deps.group(1))) | set(names)
            b = b[: deps.start(1)] + "".join(' "%s",\n' % x for x in sorted(items)) + b[deps.end(1):]
        out.append((nm, b))
    out.sort(key=lambda x: x[0])
    return head + "".join(b if b.endswith("\n\n") or i == len(out) - 1 else b + "\n" for i, (n, b) in enumerate(out))


def rule_path(l):
    return ".".join(camel(c[0]) + "[]" for c in l.ent.chain) + "." + ".".join(camel(p) for p in l.path)


def register(F, names):
    f = art.root + "/Cargo.toml"
    s = F.read(f)
    for g, (cname, d) in names.items():
        rel = os.path.relpath(d + "/📦️packages/🦀️rust", art.root)
        if '"%s"' % rel not in s:
            s = s.replace("members = [\n", 'members = [\n    "%s",\n' % rel, 1)
        if "[workspace.dependencies.%s]" % cname not in s:
            s = s.replace("[workspace.lints.rust]", '[workspace.dependencies.%s]\npath = "%s"\n\n[workspace.lints.rust]' % (cname, rel), 1)
    F.write(f, s)
    f = art.root + "/📦️packages/🦀️rust/📜️script.ts"
    s = F.read(f)
    anchor = '"%s",' % art.anchor_crate
    F.write(f, rep(s, anchor, anchor + "".join('"%s",' % c for c, _ in names.values() if '"%s"' % c not in s)))
    s = F.read(OWNERS)
    ent = re.search(r'    \{\n      "name": "%s",\n.*?\n    \},\n' % re.escape(art.anchor_crate), s, re.S).group(0)
    new = ""
    for g, (cname, d) in names.items():
        if '"name": "%s"' % cname not in s:
            new += ent.replace(art.anchor_crate, cname).replace(os.path.relpath(art.mut + "/" + art.anchor_dir + "/📦️packages/🦀️rust", SEMIO), os.path.relpath(d + "/📦️packages/🦀️rust", SEMIO))
    F.write(OWNERS, s.replace(ent, ent + new, 1))
    for pkg in [art.mut + "/📦️packages/🦀️rust/Cargo.toml", art.any + "/🚪️io/📦️packages/🦀️rust/Cargo.toml", art.any + "/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml"]:
        s = F.read(pkg)
        a = "%s = { workspace = true }\n" % art.anchor_crate
        F.write(pkg, rep(s, a, a + "".join("%s = { workspace = true }\n" % c for c, _ in names.values() if (c + " =") not in s)))


def root_twins(F, leaves, groups_of, gql=None):
    P = art.P
    s = F.read(art.mut + "/🦀️.rs")
    uses = "".join("use %s::%s;\n" % (groups_of[l.kind].replace("-", "_"), l.module) for l in leaves)
    s = rep(s, "//#endregion 🔖️Leaves", uses + "//#endregion 🔖️Leaves")
    s = rep(s, "\n}\n\npub const KINDS", "\n" + "".join("    %s(%s::%s),\n" % (l.record, l.module, l.record) for l in leaves).rstrip("\n") + "\n}\n\npub const KINDS", 1)
    s = rep(s, "\n];\n//#endregion 🔖️Mutations", "\n" + "".join('    "%s",\n' % l.kind for l in leaves).rstrip("\n") + "\n];\n//#endregion 🔖️Mutations", 1)
    F.write(art.mut + "/🦀️.rs", s)
    p = art.mut + "/🔣️.json"
    j = json.loads(F.read(p))
    for l in leaves:
        j["oneOf"].append({"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/%s/mutation/%s/schema.json" % (SHORT, l.kind)})
    F.write(p, dumps(j))


def text_codec(F, leaves, groups_of):
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    uses = ""
    for c in sorted({groups_of[l.kind] for l in leaves}):
        uses += "use %s::{%s};\n" % (c.replace("-", "_"), ", ".join(sorted(l.module for l in leaves if groups_of[l.kind] == c)))
    s = rep(s, "use semio_s_artifact_norm_en1997_model::{", uses + "use semio_s_artifact_norm_en1997_model::{", 1)
    enums = sorted({l.rust_ty for l in leaves if l.ty.startswith("enum:")} - set(re.findall(r"\b[A-Z]\w+", re.search(r"use semio_s_artifact_norm_en1997_model::\{([^}]*)\}", s).group(1))))
    if enums:
        s = s.replace("use semio_s_artifact_norm_en1997_model::{", "use semio_s_artifact_norm_en1997_model::{" + ", ".join(enums) + ", ", 1)
    variants = to_dsl = from_dsl = ""
    for l in leaves:
        f = [(x, "String") for x in l.ent.sels] + [(l.new, l.rust_ty)]
        variants += "    %s { %s },\n" % (l.record, ", ".join("%s: %s" % x for x in f))
        to_dsl += "        En1997Mutation::%s(payload) => En1997MutationDsl::%s { %s },\n" % (l.record, l.record, ", ".join("%s: payload.%s%s" % (n, n, ".clone()" if t == "String" else "") for n, t in f))
        from_dsl += "        En1997MutationDsl::%s { %s } => En1997Mutation::%s(%s::%s { %s }),\n" % (l.record, ", ".join(n for n, _ in f), l.record, l.module, l.record, ", ".join(n for n, _ in f))
    s = rep(s, "\n}\n\nimpl OpText for En1997MutationDsl", "\n" + variants.rstrip("\n") + "\n}\n\nimpl OpText for En1997MutationDsl", 1)
    a = "    }\n}\n\npub fn en1997_mutation_from_dsl"
    s = rep(s, a, to_dsl + a, 1)
    a = "    }\n}\n\nimpl OpText for En1997Mutation {"
    s = rep(s, a, from_dsl + a, 1)
    F.write(p, s)


def harness(F, leaves):
    h = art.harness
    p = h + "/🦀️.rs"
    s = F.read(p)
    last = re.findall(r'\n    "([a-z0-9-]+)",\n\];', s)[0]
    s = rep(s, '    "%s",\n];' % last, '    "%s",\n' % last + "".join('    "%s",\n' % l.kind for l in leaves) + "];", 1)
    arms = ""
    for l in leaves:
        arms += '        "%s" => (\n' % l.kind + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (l.dirname, x) for x in ["before", "mutation", "after", "outcome"]) + "        ),\n"
    a = '        other => panic!("mutate-en1997-1: no committed fixture'
    s = rep(s, a, arms + a, 1)
    F.write(p, s)
    p = h + "/🐍️.py"
    s = F.read(p)
    s = rep(s, '    "%s",\n]' % last, '    "%s",\n' % last + "".join('    "%s",\n' % l.kind for l in leaves) + "]", 1)
    m = re.search(r'    "%s": \("[^"]*", "✅apply"\),\n' % last, s)
    s = s[: m.end()] + "".join('    "%s": ("%s", "✅apply"),\n' % (l.kind, l.dirname) for l in leaves) + s[m.end():]
    F.write(p, s)
    p = h + "/🥒️.feature"
    s = F.read(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (l.kind, l.dirname) for l in leaves)
    s = re.sub(r"(      \| %s [^\n]*✅apply[^\n]*\n)" % re.escape(last), lambda m: m.group(1) + rows, s)
    F.write(p, s)


def oracle(F, leaves):
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(F.read(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    vt = [v for v in cat["vectors"] if v["mutationId"] == "change-slope-angle"][0]
    mt = [m for m in man["mutations"] if m["id"] == "change-slope-angle"][0]
    for l in leaves:
        v = copy.deepcopy(vt)
        v["mutationId"] = l.kind
        v["sourceMutationDirectoryName"] = v["mutationDirectoryName"] = l.dirname
        cat["vectors"].append(v)
        cat["kinds"].append(l.kind)
        m = copy.deepcopy(mt)
        m["id"] = l.kind
        m["productionDispatch"]["operation"] = l.kind
        m["productionDispatch"]["variant"] = l.record
        man["mutations"].append(m)
    F.write(p, dumps(o))


def diff_twins(F, leaves, ents):
    spec = D.specs("En1997", ents, leaves, "m")
    by = {}
    for e in ents:
        s = spec[e.patch]
        s.lane = e.chain[-1][0]
        s.parents = [spec["".join(pascal(c[0]) for c in e.chain[:-1])]] if len(e.chain) > 1 else []
        by[e.patch] = s
    dp = art.any + "/🧬️schema/🔺️diff/"
    model = "semio_s_artifact_norm_en1997_model"
    url = "https://json.schemas.assets.semio-tech.com/s/norm/en1997/snapshot.json"
    F.write(dp + "🦀️.rs", D7.patch_rust(F.read(dp + "🦀️.rs"), "En1997", by, model, CLONE_STEM, CLONE_ROW))
    F.write(dp + "🔣️.json", D7.patch_json(F.read(dp + "🔣️.json"), by, url, CLONE_STEM, CLONE_ROW))
    return spec


def pick(doc, chain):
    cur, ids = doc, []
    rows = doc[camel(chain[0][0])]
    if len(chain) == 1:
        return [rows[0]["id"]]
    for r in rows:
        if r.get(camel(chain[1][0])):
            return [r["id"], r[camel(chain[1][0])][0]["id"]]
    raise AssertionError("no nested row for " + chain[1][0])


def fixtures(F, leaves, defs, base, top):
    for l in leaves:
        ent = l.ent
        ids = pick(base, ent.chain)
        row = base
        for (coll, _, _), i in zip(ent.chain, ids):
            row = next(r for r in row[camel(coll)] if r["id"] == i)
        old = row[camel(l.path[-1])]
        val = X.new_value(base, l, old, defs)
        after = copy.deepcopy(base)
        ar = after
        for (coll, _, _), i in zip(ent.chain, ids):
            ar = next(r for r in ar[camel(coll)] if r["id"] == i)
        ar[camel(l.path[-1])] = val
        mut = {"mutation": l.record[:1].lower() + l.record[1:]}
        for sel, i in zip(ent.sels, ids):
            mut[camel(sel)] = i
        mut[l.newc] = val
        patch = {camel(l.field): val}
        rows = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": ids[-1], "patch": patch}]}
        for depth in range(len(ent.chain) - 2, -1, -1):
            patch = {camel(ent.chain[depth + 1][0]): rows}
            rows = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": ids[depth], "patch": patch}]}
        diff = {k: None for k in top}
        diff[camel(ent.chain[0][0])] = rows
        b = art.fix + "/" + l.dirname + "/✅apply"
        F.write(b + "/🔣️.before.json", dumps(base))
        F.write(b + "/🔣️.mutation.json", dumps(mut))
        F.write(b + "/🔣️.after.json", dumps(after))
        F.write(b + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(b + "/🔣️.diff.json", dumps(diff))


def main():
    F = Files(PRIVATE_STAGE, merge="--overwrite" not in sys.argv)
    schema_path = art.any + "/🧬️schema/📸️snapshot/🔣️.json"
    leaves, defs = M.read_entities(schema_path, ENTS, COVERED)
    rs = rd(art.any + "/🧬️schema/📸️snapshot/🦀️.rs")
    leaves = [l for l in leaves if re.search(r"pub %s: " % l.path[-1], rs)]
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M))
    new = [l for l in leaves if not l.covered and l.kind not in existing]
    only = os.environ.get("LEAF_ONLY")
    if only:
        new = [l for l in new if l.kind in only.split(",")]
    print("new kinds", [l.kind for l in new])
    groups, order = W.group_leaves(art, new)
    names = W.create_crates(art, F, groups, art.mut + "/" + art.anchor_dir)
    groups_of = {l.kind: names[g][0] for g, ls in groups.items() for l in ls}
    kinds_all = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M) + [l.kind for l in new]
    tags = {k: i for i, k in enumerate(kinds_all)}
    register(F, names)
    W.leaf_files(art, F, new, defs, tags)
    root_twins(F, new, groups_of)
    W.edit_rules(art, F, new, rule_path, [])
    text_codec(F, new, groups_of)
    W.binary_protocol(art, F, kinds_all)
    harness(F, new)
    oracle(F, new)
    W.taxonomy(art, F, new)
    spec = diff_twins(F, leaves, ENTS)
    base = json.loads(rd(RICH))
    seeds = os.path.join(os.path.dirname(os.path.abspath(__file__)), "seeds-en1997.json")
    for collection, row in (json.loads(rd(seeds)) if os.path.exists(seeds) else {}).items():
        base[collection] = base.get(collection) or [row]
    top = list(json.loads(rd(art.fix + "/⛰️change-slope-angle/✅apply/🔣️.diff.json")).keys())
    fixtures(F, new, defs, base, top)
    print("leaves", len(leaves), "new", len(new), "crates", len(names), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply(), "merged into existing", len(F.merged))
    return F


if __name__ == "__main__":
    shutil.rmtree(PRIVATE_STAGE, ignore_errors=True)
    main()
