"""Placement kinds profile (leaf-kinds E): python -I placements.py <en1996|en1997> [--apply]. Four `change-placement-{x,y,z,yaw}` set kinds (selector `id`), `insert-placement` (index + item) and `remove-placement` (index) for an artifact whose snapshot carries the persisted `placements` collection; wiring of root enum/KINDS/ts/json/graphql/proto, text DSL (en1997), binary protocol, edit rules, harness, oracle, taxonomy, crate registration, fixtures."""
import copy, glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I, keyed as K, keyed_fixtures as X, placements94 as P94

SHORT = sys.argv[1] if len(sys.argv) > 1 and not sys.argv[1].startswith("-") else "en1997"
DIRNAME = next(n for n in os.listdir(NORM + "/🗿️artifacts") if n.endswith(SHORT))
EMOJI = DIRNAME[: -len(SHORT)]
STAGE_E = TICKET + "/🗑️generated/leaf-kinds/stage-e/" + SHORT
P = SHORT.replace("en", "En", 1)
MODEL = "semio_s_artifact_norm_%s_model" % SHORT
CFG = {
    "en1996": dict(mode="single", tagged=False, anchor_dir="🧭plan", anchor_crate="semio-s-artifact-norm-en1996-mut-plan", template="change-plan-x", ref_dir="🧭change-plan-x", sample="wall-north", second="wall-south"),
    "en1991": dict(mode="single", tagged=False, anchor_dir=None, anchor_crate=None, template="change-floor-area", ref_dir="🗺️change-floor-area", sample=None, second=None),
    "en1993": dict(mode="single", tagged=False, anchor_dir=None, anchor_crate=None, template="change-material-grade", ref_dir=None, sample=None, second=None),
    "en1995": dict(mode="single", tagged=False, anchor_dir=None, anchor_crate=None, template="change-connection-diameter", ref_dir=None, sample=None, second=None),
    "en1994": dict(mode="lanes", tagged=False, anchor_dir=None, anchor_crate=None, template="change-beam-stud-count", ref_dir=None, sample=None, second=None),
    "en1998": dict(mode="single", tagged=False, anchor_dir=None, anchor_crate=None, template="change-tower-m-rd-nm", ref_dir=None, sample=None, second=None),
    "en1997": dict(mode="lanes", tagged=True, anchor_dir=None, anchor_crate="semio-s-artifact-norm-en1997-mut-ground", template="change-slope-angle", ref_dir="⛰️change-slope-angle", sample=None, second=None),
}[SHORT]
ST = I.Style(P, MODEL, "semio_s_artifact_norm_%s_diff" % SHORT, P + "Snapshot", "../../../📸️snapshot/🟦️.ts", mode=CFG["mode"], opt_index=False, tagged=CFG["tagged"], item_field=lambda e: "item", ts_prefix=P)
ROOT = NORM + "/🗿️artifacts/" + DIRNAME
if SHORT == "en1997":
    CFG["anchor_dir"] = next(n for n in os.listdir(ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations") if n.endswith("ground"))
if SHORT in ("en1991", "en1993", "en1994", "en1995", "en1998"):
    _s = rd(ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs")
    _c = re.search(r"^use (semio_s_artifact_norm_%s_mut_[a-z0-9_]+)::%s;" % (SHORT, CFG["template"].replace("-", "_")), _s, re.M).group(1).replace("_", "-")
    CFG["anchor_crate"] = _c
    CFG["ref_dir"] = next(n for n in os.listdir(ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations") if n.endswith(CFG["template"]))
    _p = re.search(r'\[workspace\.dependencies\.%s\]\npath = "([^"]+)"' % _c, rd(ROOT + "/Cargo.toml")).group(1)
    CFG["anchor_dir"] = _p.split("/🧬️mutations/")[1].split("/")[0]
_rep = K.rep


def _rep_doc(text, old, new, count=1):
    if old == "\n}\n\npub const KINDS" and old not in text:
        m = re.search(r"\n\}\n\n(?=(?:///[^\n]*\n|#\[[^\n]*\n)*pub const KINDS)", text)
        assert m, "KINDS anchor missing"
        return text[: m.start()] + new[: -len("\n}\n\npub const KINDS")] + "\n}\n\n" + text[m.end():]
    return _rep(text, old, new, count)


K.rep = _rep_doc
art = K.Art(SHORT, EMOJI, ST, SHORT, CFG["anchor_dir"], CFG["anchor_crate"], CFG["template"], None)
PLAC = I.Link("placements", "En1998Placement" if SHORT == "en1998" else "Placement", "id", "placement", "Placement", "Platzierung")
ENT = I.Ent("placement", "📍️", [PLAC], "Placement", "Platzierung", slug="placement", group="placements")
FIELDS = ["x", "y", "z", "yaw"]


def leaves(defs):
    props = defs[PLAC.row]["properties"]
    out = []
    for f in FIELDS:
        out.append(I.Leaf(ENT, [f], "f64", props[f].get("x-semio-ui"), owner="Placement", kind="change-placement-" + f))
    return out


ROWS91 = [
    ("bridgeLanes", "BridgeLane", "bridge_lane", "lane", "bridge-lanes", "Bridge lane", "Brückenfahrstreifen", "🛣️"),
    ("constructionStages", "ConstructionStage", "construction_stage", "stage", "construction-stages", "Construction stage", "Bauphase", "🏗️"),
    ("roofSurfaces", "RoofSurface", "roof_surface", "roof-surface", "roof-surfaces", "Roof surface", "Dachfläche", "🏠"),
]
SCALAR = {"number": "f64", "integer": "u32", "string": "String", "boolean": "bool"}


def entities91(defs):
    out = []
    for coll, row, single, slug, plural, en, de, emoji in ROWS91:
        link = I.Link(snake(coll), row, "index", single, en, de)
        ent = I.Ent(slug, emoji, [link], en, de, slug=slug, group=plural)
        ls = []
        for name, p in defs[row]["properties"].items():
            if name == "id":
                continue
            ty = "u8" if name == "lm1Lane" else SCALAR[p["type"]]
            ls.append(I.Leaf(ent, [snake(name)], ty, p.get("x-semio-ui"), owner=row, kind="change-%s-%s" % (slug, kebab(snake(name)))))
        ks = [I.SetKind(l) for l in ls]
        ins, rm = I.pair(ent, "insert-" + plural, "remove-" + plural, ST, by_index=True)
        for k in ks + [ins, rm]:
            k.group, k.family = plural, ""
        out.append((ent, coll, defs[row], ks + [ins, rm]))
    if "Placement" in defs:
        pks = [I.SetKind(l) for l in leaves(defs)]
        pi, pr = I.pair(ENT, "insert-placement", "remove-placement", ST, by_index=True)
        for k in pks + [pi, pr]:
            k.group, k.family = "placements", ""
        out.append((ENT, "placements", defs["Placement"], pks + [pi, pr]))
    return out


def graphql_gap(art, F, kinds, ins):
    p = art.mut + "/🔗️.graphql"
    s = F.read(p)
    names = [k.record for k in kinds]
    if SHORT == "en1996":
        m = re.search(r"^union %sMutation = [^\n]*" % P, s, re.M)
        s = s[: m.end()] + "".join(" | " + n for n in names) + s[m.end():]
        blocks = ""
        for k in kinds:
            if isinstance(k, I.SetKind):
                blocks += "type %s {\n  id: String\n  %s: Float\n}\n\n" % (k.record, k.leaf.newc)
            elif isinstance(k, I.InsertKind):
                blocks += "type %s {\n  index: Int\n  item: String\n}\n\n" % k.record
            else:
                blocks += "type %s {\n  index: Int\n}\n\n" % k.record
        s = s.rstrip("\n") + "\n\n" + blocks.rstrip("\n") + "\n"
    else:
        last = list(re.finditer(r"^  \| \w+$", s, re.M))[-1]
        s = s[: last.end()] + "".join("\n  | " + n for n in names) + s[last.end():]
        for k in kinds:
            if isinstance(k, I.SetKind):
                line = "type %s { mutation: String! id: String! %s: Float! }\n" % (k.record, k.leaf.newc)
            elif isinstance(k, I.InsertKind):
                line = "type %s { mutation: String! index: Int! item: PlacementInput! }\n" % k.record
            else:
                line = "type %s { mutation: String! index: Int! }\n" % k.record
            s = s.rstrip("\n") + "\n" + line
        if "input PlacementInput" not in s:
            end = list(re.finditer(r"^input \w+ \{\n(?:  [^\n]*\n)*\}\n", s, re.M))[-1].end()
            s = s[:end] + "input PlacementInput {\n  id: String!\n  x: Float!\n  y: Float!\n  z: Float!\n  yaw: Float!\n}\n" + s[end:]
    F.write(p, s)


def gql_proto_generic(art, F, kinds):
    d = art.mut
    rec_t = pascal(art.template_kind)
    p = d + "/🔗️.graphql"
    g = F.read(p)
    m = re.search(r"^type %s( \{[^\n]*\}|\s*\{\n(?:[^\n]*\n)*?\})" % rec_t, g, re.M)
    single = bool(m) and "\n" not in m.group(0)
    suffix = " @state(class: ARTIFACT)" if "@state(class: ARTIFACT)" in g else ""
    bang = "!" if re.search(r"\w+: \w+!", m.group(0)) else ""
    kind_comment = bool(m) and re.search(r"\{\n  # kind:", m.group(0)) is not None
    ins_m = re.search(r"^type Insert\w+ \{\n((?:[^\n]*\n)*?)\}", g, re.M)
    names = [k.record for k in kinds]
    uni = re.search(r"^union \w+ =( [^\n]*|\n)", g, re.M)
    if uni and uni.group(1) != "\n":
        g = g[: uni.end()] + "".join(" | " + n for n in names) + g[uni.end():]
    elif uni:
        last = list(re.finditer(r"^  \| \w+$", g, re.M))[-1]
        g = g[: last.end()] + "".join("\n  | " + n for n in names) + g[last.end():]
    row_ty = (re.findall(r"\w+: (\w+)", ins_m.group(1))[-1] if ins_m else "JSON")
    ibang = "!" if ins_m and re.search(r"index: Int!", ins_m.group(1)) else ""
    for k in kinds:
        if isinstance(k, I.SetKind):
            f = [("id" if not I.is_index(k.ent.sels[0]) else "index", "String" if not I.is_index(k.ent.sels[0]) else "Int", bang), (k.leaf.newc, "Float" if k.leaf.ty == "f64" else "String" if k.leaf.ty == "String" else "Int" if k.leaf.ty in ("u32", "u8") else "Boolean", bang)]
        elif isinstance(k, I.InsertKind):
            if not ins_m and single:
                continue
            f = [("index", "Int", ibang), ("payload" if single else "item", row_ty, "!")]
        else:
            f = [("index", "Int", ibang or bang)]
        if single:
            g = g.rstrip("\n") + "\ntype %s { %s }\n" % (k.record, " ".join("%s: %s%s%s" % (n, t, b, suffix) for n, t, b in f))
        else:
            body = ('  # kind: "%s"\n' % k.kind if kind_comment else "") + "".join("  %s: %s%s%s\n" % (n, t, b, suffix) for n, t, b in f)
            g = g.rstrip("\n") + "\n\ntype %s {\n%s}\n" % (k.record, body)
    F.write(p, g)
    p = d + "/🛰️.proto"
    t = F.read(p)
    oneof = re.search(r"  oneof \w+ \{\n((?:    [^\n]*\n)*)  \}", t)
    if not oneof:
        F.write(p, t)
        return
    n = max(int(x) for x in re.findall(r"= (\d+);", oneof.group(1)))
    pm = re.search(r"^message %s( \{[^\n]*\}|\s*\{\n(?:[^\n]*\n)*?\})" % rec_t, t, re.M)
    psingle = bool(pm) and "\n" not in pm.group(0)
    state = "  // @state artifact\n" if pm and "// @state artifact" in pm.group(0) else ""
    pre = bool(pm) and re.search(r"// kind: [\w-]+\n$", t[: pm.start()]) is not None
    entries, msgs = "", ""
    for k in kinds:
        n += 1
        entries += "    %s %s = %d;\n" % (k.record, k.module, n)
        if isinstance(k, I.SetKind):
            idx = I.is_index(k.ent.sels[0])
            fl = [("uint32" if idx else "string", "index" if idx else "id"), ("double" if k.leaf.ty == "f64" else "string" if k.leaf.ty == "String" else "uint32" if k.leaf.ty in ("u32", "u8") else "bool", snake(k.leaf.newc))]
        elif isinstance(k, I.InsertKind):
            fl = [("uint32", "index"), ("bytes", "payload" if psingle else "item")]
        else:
            fl = [("uint32", "index")]
        if psingle:
            msgs += "message %s { %s }\n" % (k.record, " ".join("%s %s = %d;" % (ty, nm, i + 1) for i, (ty, nm) in enumerate(fl)))
        else:
            msgs += ("// kind: %s\n" % k.kind if pre else "") + "message %s {\n%s}\n\n" % (k.record, "".join(state + "  %s %s = %d;\n" % (ty, nm, i + 1) for i, (ty, nm) in enumerate(fl)))
    t = t[: oneof.end(1)] + entries + t[oneof.end(1):]
    F.write(p, t.rstrip("\n") + "\n" + ("\n" if not psingle else "") + msgs.rstrip("\n") + "\n")


def proto_gap(art, F, kinds):
    p = art.mut + "/🛰️.proto"
    s = F.read(p)
    if SHORT != "en1996":
        return
    nums = [int(x) for x in re.findall(r"= (\d+);\n", s[: s.index("\n  }\n}")])]
    n = max(nums)
    entries, msgs = "", ""
    for k in kinds:
        n += 1
        entries += "    %s %s = %d;\n" % (k.record, k.module, n)
        if isinstance(k, I.SetKind):
            msgs += "message %s {\n  string id = 1;\n  double %s = 2;\n}\n" % (k.record, snake(k.leaf.newc))
        elif isinstance(k, I.InsertKind):
            msgs += "message %s {\n  uint32 index = 1;\n  string item = 2;\n}\n" % k.record
        else:
            msgs += "message %s {\n  uint32 index = 1;\n}\n" % k.record
    i = s.index("  }\n}\nmessage")
    s = s[:i] + entries + s[i:]
    F.write(p, s.rstrip("\n") + "\n" + msgs)


def text_dsl(art, F, kinds, groups_of):
    if SHORT != "en1997":
        return
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    crate = groups_of[kinds[0].kind].replace("-", "_")
    mods = ", ".join(sorted(k.module for k in kinds))
    anchor = re.findall(r"^use semio_s_artifact_norm_en1997_mut_[a-z_]+::\{[^\n]*\};\n", s, re.M)[0]
    s = s.replace(anchor, anchor + "use %s::{%s};\n" % (crate, mods), 1)
    s = rep(s, "use semio_s_artifact_norm_en1997_model::{", "use semio_s_artifact_norm_en1997_model::{Placement, ", 1)
    variants = to_dsl = from_dsl = ""
    for k in kinds:
        if isinstance(k, I.SetKind):
            variants += "    %s { id: String, %s: f64 },\n" % (k.record, k.leaf.new)
            to_dsl += "        En1997Mutation::%s(payload) => En1997MutationDsl::%s { id: payload.id.clone(), %s: payload.%s },\n" % (k.record, k.record, k.leaf.new, k.leaf.new)
            from_dsl += "        En1997MutationDsl::%s { id, %s } => En1997Mutation::%s(%s::%s { id, %s }),\n" % (k.record, k.leaf.new, k.record, k.module, k.record, k.leaf.new)
        elif isinstance(k, I.InsertKind):
            variants += "    %s {\n        index: usize,\n        #[dsl(block)]\n        item: Placement,\n    },\n" % k.record
            to_dsl += "        En1997Mutation::%s(payload) => En1997MutationDsl::%s { index: payload.index, item: payload.item.clone() },\n" % (k.record, k.record)
            from_dsl += "        En1997MutationDsl::%s { index, item } => En1997Mutation::%s(%s::%s { index, item }),\n" % (k.record, k.record, k.module, k.record)
        else:
            variants += "    %s { index: usize },\n" % k.record
            to_dsl += "        En1997Mutation::%s(payload) => En1997MutationDsl::%s { index: payload.index },\n" % (k.record, k.record)
            from_dsl += "        En1997MutationDsl::%s { index } => En1997Mutation::%s(%s::%s { index }),\n" % (k.record, k.record, k.module, k.record)
    s = rep(s, "\n}\n\nimpl OpText for En1997MutationDsl", "\n" + variants.rstrip("\n") + "\n}\n\nimpl OpText for En1997MutationDsl", 1)
    a = "    }\n}\n\npub fn en1997_mutation_from_dsl"
    s = rep(s, a, to_dsl + a, 1)
    a = "    }\n}\n\nimpl OpText for En1997Mutation {"
    s = rep(s, a, from_dsl + a, 1)
    F.write(p, s)


def harness(art, F, kinds):
    h = art.harness
    p = h + "/🐍️.py"
    s = F.read(p)
    if '"%s"' % art.template_kind in s:
        last = re.findall(r'\n    "([a-z0-9-]+)",\n\]', s)[0]
        s = rep(s, '    "%s",\n]' % last, '    "%s",\n' % last + "".join('    "%s",\n' % k.kind for k in kinds) + "]", 1)
        m = list(re.finditer(r'^    "[a-z0-9-]+": \("[^"]*", "✅apply"\),\n', s, re.M))[-1]
        s = s[: m.end()] + "".join('    "%s": ("%s", "✅apply"),\n' % (k.kind, k.dirname) for k in kinds) + s[m.end():]
        F.write(p, s)
    p = h + "/🥒️.feature"
    s = F.read(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (k.kind, k.dirname) for k in kinds)
    s = re.sub(r"(      \| %s [^\n]*\n)" % re.escape(art.template_kind), lambda m: m.group(1) + rows, s)
    F.write(p, s)
    if SHORT == "en1997":
        p = h + "/🦀️.rs"
        s = F.read(p)
        s = rep(s, '    "remove-layer-water",\n', '    "remove-layer-water",\n' + "".join('    "%s",\n' % k.kind for k in kinds), 1)
        arms = ""
        for k in kinds:
            arms += '        "%s" => (\n' % k.kind + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (k.dirname, x) for x in ["before", "mutation", "after", "outcome"]) + "        ),\n"
        a = '        other => panic!("mutate-en1997-1: no committed fixture'
        s = rep(s, a, arms + a, 1)
        F.write(p, s)


def main():
    F = K.KFiles(STAGE_E)
    schema = json.loads(rd(art.any + "/🧬️schema/📸️snapshot/🔣️.json"))
    defs = schema["$defs"]
    existing = K.kinds_of(art, F)
    if SHORT == "en1991":
        ents = entities91(defs)
        new_kinds = [k for e in ents for k in e[3]]
    else:
        lv = leaves(defs)
        new_kinds = [I.SetKind(l) for l in lv]
        for k in new_kinds:
            k.family = ""
        ins, rm = I.pair(ENT, "insert-placement", "remove-placement", ST, by_index=True)
        for x in (ins, rm):
            x.group, x.family = "placements", ""
        new_kinds += [ins, rm]
        for k in new_kinds:
            k.group = "placements"
    new_kinds = [k for k in new_kinds if k.kind not in existing]
    if not new_kinds:
        print("nothing to do: all placement kinds exist")
        return
    assert len({k.kind for k in new_kinds}) == len(new_kinds)
    if SHORT == "en1994":
        P94.positional(new_kinds)
    tags = K.binary_protocol_keep(art, F, new_kinds, set())
    existing_crates = set(re.findall(r'"(semio-s-artifact-norm-%s-mut-[a-z0-9-]+)"' % SHORT, rd(art.root + "/Cargo.toml")))
    groups = K.group_kinds(art, new_kinds, existing_crates)
    names = K.create_crates(art, F, groups)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    K.register(art, F, names)
    K.leaf_files(art, F, new_kinds, defs, tags)
    K.root_wire(art, F, new_kinds, groups_of)
    K.edit_rules(art, F, new_kinds)
    if SHORT in ("en1996", "en1997"):
        graphql_gap(art, F, new_kinds, None)
        proto_gap(art, F, new_kinds)
    elif SHORT in ("en1993", "en1995", "en1998"):
        gql_proto_generic(art, F, new_kinds)
    text_dsl(art, F, new_kinds, groups_of)
    harness(art, F, new_kinds)
    K.oracle(art, F, new_kinds)
    K.taxonomy(art, F, new_kinds)
    ref = art.fix + "/" + CFG["ref_dir"] + "/✅apply"
    base = json.loads(rd(ref + "/🔣️.before.json"))
    if SHORT == "en1997":
        base = json.loads(rd(art.fix + "/➖remove-footing/🧭️middle/🔣️.before.json"))
    if SHORT == "en1994":
        diff = json.loads(rd(ref + "/🔣️.diff.json"))
        blank = {k: ({"removed": [], "inserted": [], "modified": []} if isinstance(v, dict) else None) for k, v in diff.items()}
        base["placements"] = [{"id": base["beams"][0]["id"], "x": 1.0, "y": 2.0, "z": 0.5, "yaw": 0.25}]
        P94.fixtures(art, F, new_kinds, base, blank)
        print("kinds", len(new_kinds), "crates", len(names), "files", len(F.written))
        if "--apply" in sys.argv:
            print("applied", F.apply())
        return
    if SHORT == "en1991":
        diff = json.loads(rd(ref + "/🔣️.diff.json"))
        blank = {k: ({"removed": [], "inserted": [], "moved": [], "modified": []} if isinstance(v, dict) else None) for k, v in diff.items()}
        spec = {}
        rows = {}
        for ent, coll, d, ks in ents:
            fields = [camel(n) for n in d["properties"] if n != "id"]
            spec[pascal(ent.last.single)] = (fields, [])
            row = {"id": ent.slug + "-1"}
            for n, p in d["properties"].items():
                if n != "id":
                    row[n] = {"number": 1.5, "integer": 1, "string": "x", "boolean": False}[p["type"]]
            base[coll] = [row]
            rows[ent.last.single] = row
        X.build(art, F, new_kinds, defs, base, spec, blank, lambda k, coll, b: dict(rows[k.ent.last.single], id=rows[k.ent.last.single]["id"] + "-new"))
        print("kinds", len(new_kinds), "crates", len(names), "files", len(F.written))
        if "--apply" in sys.argv:
            print("applied", F.apply())
        return
    if SHORT == "en1997":
        sample = base["footings"][0]["id"]
    else:
        sample = CFG["sample"] or next(r[0]["id"] for k, r in base.items() if k != "placements" and isinstance(r, list) and r and isinstance(r[0], dict) and "id" in r[0])
    base["placements"] = [{"id": sample, "x": 1.0, "y": 2.0, "z": 0.5, "yaw": 0.25}]
    diff = json.loads(rd(ref + "/🔣️.diff.json"))
    if CFG["mode"] == "single":
        blank = {k: ({"removed": [], "inserted": [], "moved": [], "modified": []} if isinstance(v, dict) else None) for k, v in diff.items()}
        spec = {"Placement": ([camel(f) for f in FIELDS], [])}
    else:
        blank = {k: None for k in diff}
        spec = {"Placements": ([camel(f) for f in FIELDS], [])}
    second = CFG["second"] or sample + "-b"
    X.build(art, F, new_kinds, defs, base, spec, blank, lambda k, coll, b: {"id": second, "x": 3.0, "y": 4.0, "z": 0.0, "yaw": 1.5})
    print("kinds", len(new_kinds), "crates", len(names), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


if __name__ == "__main__":
    shutil.rmtree(STAGE_E, ignore_errors=True)
    main()
