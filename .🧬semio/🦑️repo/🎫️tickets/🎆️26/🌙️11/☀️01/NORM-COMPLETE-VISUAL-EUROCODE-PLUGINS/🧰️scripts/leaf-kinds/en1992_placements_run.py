"""EN 1992 placements driver (scene v4): python -I en1992_placements_run.py [--apply]. Retires the eight inline `change-{member,anchor}-placement-*` kinds and adds `change-placement-{x,y,z,yaw}`, `insert-placement`, `remove-placement` for the flat `placements` collection. Derived from en1992_run.py. One typed set-field kind per snapshot leaf, insert/remove-row kinds per collection, diff patch lanes, twins, fixtures."""
import copy, glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I, keyed as K, keyed_fixtures as X, diff as D, en1992_diff as ED

ART = "en1992"
STAGE_A = TICKET + "/🗑️generated/leaf-kinds/stage-scene-v4/en1992"
MODEL = "semio_s_artifact_norm_en1992_model"
ST = I.Style("En1992", MODEL, "semio_s_artifact_norm_en1992_diff", "En1992Snapshot", "../../../📸️snapshot/🟦️.ts", mode="lanes", opt_index=False, tagged=False)
art = K.Art(ART, "🏛️", ST, "en1992", "📐️member-fields", "semio-s-artifact-norm-en1992-mut-member-fields", "change-member-cover", None)

L = I.Link
GRADE_C = L("concrete_grades", "ConcreteGrade", "grade_id", "concrete_grade", "Concrete grade", "Betonsorte")
GRADE_R = L("reinforcement_grades", "ReinforcementGrade", "grade_id", "reinforcement_grade", "Reinforcement grade", "Betonstahlsorte")
GRADE_P = L("prestress_steels", "PrestressSteel", "grade_id", "prestress_steel", "Prestressing steel", "Spannstahl")
MEMBER = L("members", "RcMember", "member_id", "member", "Member", "Bauteil")
LAYER = L("longitudinal", "BarLayer", "layer_id", "bar_layer", "Bar layer", "Bewehrungslage")
MACT = L("actions", "LoadCaseActions", "action_id", "action", "Action", "Einwirkung")
SECT = L("section_points", "SectionPoint", "section_point_id", "section_point", "Section point", "Querschnittspunkt")
BARP = L("bars", "BarPoint", "bar_id", "bar_point", "Bar point", "Bewehrungsstab")
TENP = L("tendon_points", "TendonPoint", "tendon_point_id", "tendon_point", "Tendon point", "Spanngliedpunkt")
STUD = L("stud_rings", "StudRing", "stud_ring_id", "stud_ring", "Stud ring", "Dübelkreis")
PLAC = L("placements", "Placement", "placement_id", "placement", "Placement", "Platzierung")
ANCH = L("anchors", "Anchor", "anchor_id", "anchor", "Anchor", "Dübel")
AACT = L("actions", "LoadCaseActions", "action_id", "anchor_action", "Anchor action", "Dübeleinwirkung")

E = I.Ent
ENTS = [
    E("concrete-grade", "🧱", [GRADE_C], "Concrete grade", "Betonsorte", slug="concrete", group="concrete-grades"),
    E("reinforcement-grade", "🔩", [GRADE_R], "Reinforcement grade", "Betonstahlsorte", slug="reinforcement", group="reinforcement-grades"),
    E("prestress-steel", "🧵", [GRADE_P], "Prestressing steel", "Spannstahl", slug="prestress-steel", group="prestress-steels"),
    E("member", "🏛️", [MEMBER], "Member", "Bauteil", group="member"),
    E("bar-layer", "⭕", [MEMBER, LAYER], "Bar layer", "Bewehrungslage", group="bar-layers"),
    E("action", "🏋️", [MEMBER, MACT], "Member action", "Bauteileinwirkung", group="member-actions"),
    E("section-point", "📐️", [MEMBER, SECT], "Section point", "Querschnittspunkt", group="geometry"),
    E("bar-point", "📍", [MEMBER, BARP], "Bar point", "Bewehrungsstab", group="geometry"),
    E("tendon-point", "🧬", [MEMBER, TENP], "Tendon point", "Spanngliedpunkt", group="geometry"),
    E("stud-ring", "⭕", [MEMBER, STUD], "Stud ring", "Dübelkreis", group="geometry"),
    E("anchor", "⚓️", [ANCH], "Anchor", "Dübel", group="anchors"),
    E("anchor-action", "🏋️", [ANCH, AACT], "Anchor action", "Dübeleinwirkung", group="anchor-actions"),
    E("placement", "📍️", [PLAC], "Placement", "Platzierung", slug="placement", group="placements"),
]
BY = {e.name: e for e in ENTS}
for e in ENTS:
    e.chain = [(l.coll, l.row, l.sel) for l in e.links]
    e.patch = "".join(pascal(l.coll) for l in e.links)

FAMILIES = {
    "member": {"identity": ["label_en", "label_de", "kind", "support", "use_fem", "deflection_sensitive", "concrete_grade_id", "reinforcement_grade_id"], "placement": ["prestress_steel_id"], "shear": ["stirrups_diameter", "stirrups_legs", "punching_column_width", "punching_column_depth", "punching_column_position", "punching_asw"], "prestress-fire": ["prestress_force", "prestress_area", "prestress_eccentricity", "prestress_loss_ratio", "fire_column_method", "fire_slab_system"]},
    "anchor": {"capacity": ["cracked", "f_uk", "f_yk", "d", "c1", "f_ck"], "placement": ["head_diameter"]},
    "anchor-action": {"identity": ["kind", "category", "source"], "loads": ["g_k_line", "q_k_line", "point_force"], "resultants": ["m_k", "n_k", "v_k", "t_k", "v_k_punch"]},
    "action": {"identity": ["kind", "category", "source"], "loads": ["g_k_line", "q_k_line", "point_force", "t_k", "v_k_punch"]},
}
ENT_FAMILY = {"section-point": "section-bars", "bar-point": "section-bars", "tendon-point": "tendons-studs", "stud-ring": "tendons-studs"}
ROWS = {"concrete-grade": "grades", "reinforcement-grade": "grades", "prestress-steel": "grades", "bar-layer": "member-lists", "action": "member-lists", "section-point": "member-lists", "bar-point": "member-geometry", "tendon-point": "member-geometry", "stud-ring": "member-geometry", "anchor-action": "anchor-lists", "placement": "placements"}


def family_of(ent, field):
    for fam, fields in FAMILIES.get(ent.name, {}).items():
        if field in fields:
            return fam
    assert ent.name not in FAMILIES, (ent.name, field)
    return ENT_FAMILY.get(ent.name, "")


ABBREV = [("reinforcement", "rebar"), ("punching", "punch"), ("column", "col"), ("deflection", "defl"), ("prestress-steel", "pre-steel"), ("anchorage", "anch"), ("eccentricity", "ecc"), ("aggregate", "agg"), ("concrete-grade", "conc-grade")]
LIMIT = 34


def shorten(kind):
    for a, b in ABBREV:
        if len(kind) <= LIMIT:
            break
        kind = kind.replace(a, b)
    assert len(kind) <= LIMIT, kind
    return kind


def ref_def(p):
    for x in p.get("oneOf", []) + p.get("anyOf", []) + ([p] if "$ref" in p else []):
        if "$ref" in x:
            return x["$ref"].split("/")[-1], ("oneOf" in p or "anyOf" in p)
    return None, False


def scalar(p):
    return {"number": "f64", "integer": "u32", "boolean": "bool", "string": "String"}[p["type"]]


def read_leaves(defs, ent):
    out = []
    for name, p in defs[ent.last.row]["properties"].items():
        if name == "id" or p.get("type") == "array":
            continue
        ref, optional = ref_def(p)
        if ref and "enum" in defs[ref]:
            if optional:
                continue
            out.append(I.Leaf(ent, [snake(name)], "enum:" + ref, p.get("x-semio-ui"), owner=ent.last.row, kind="x"))
        elif ref:
            for n2, p2 in defs[ref]["properties"].items():
                r2, _ = ref_def(p2)
                if r2 and "enum" in defs[r2]:
                    out.append(I.Leaf(ent, [snake(name), snake(n2)], "enum:" + r2, p2.get("x-semio-ui"), opt=optional, owner=ref, kind="x"))
                else:
                    out.append(I.Leaf(ent, [snake(name), snake(n2)], scalar(p2), p2.get("x-semio-ui"), opt=optional, owner=ref, kind="x"))
        else:
            out.append(I.Leaf(ent, [snake(name)], scalar(p), p.get("x-semio-ui"), owner=ent.last.row, kind="x"))
    for l in out:
        l.owner_def = l.owner
        l.kind = shorten("change-%s-%s" % (ent.slug, kebab(l.field)))
        l.set_names()
    return out


def rule_path(l):
    links = l.ent.links
    return ".".join(camel(x.coll) + "[]" for x in links) + "." + ".".join(camel(x) for x in l.path)


def new_row(k, coll, base):
    row = copy.deepcopy(coll[0])
    row["id"] = row["id"] + "-new"
    return row


def augment(doc):
    m = doc["members"][0]
    m["punching"] = m.get("punching") or {"columnWidth": 0.3, "columnDepth": 0.3, "columnPosition": "interior", "asw": 0.0005}
    m["prestress"] = m.get("prestress") or {"force": 1000000.0, "area": 0.0014, "eccentricity": 0.1, "lossRatio": 0.15}
    m["fire"] = m.get("fire") or {"rating": "R90", "axisDistance": 0.04, "columnMethod": "", "slabSystem": ""}
    m["stirrups"] = m.get("stirrups") or {"diameter": 0.008, "spacing": 0.15, "legs": 2}
    m["sectionPoints"] = m["sectionPoints"] or [{"id": "sp-1", "y": 0.0, "z": 0.0}, {"id": "sp-2", "y": 0.35, "z": 0.0}]
    m["bars"] = m["bars"] or [{"id": "bar-1", "layerId": m["longitudinal"][0]["id"], "y": 0.05, "z": 0.05, "diameter": 0.016}]
    m["tendonPoints"] = m["tendonPoints"] or [{"id": "tp-1", "x": 0.0, "y": 0.1, "z": 0.2}]
    m["studRings"] = m["studRings"] or [{"id": "sr-1", "radius": 0.2, "count": 8, "diameter": 0.01}]
    doc["placements"] = doc["placements"] or [{"id": m["id"], "x": 1.0, "y": 0.5, "z": 0.0, "yaw": 0.25}]
    return doc


def clone_paragraphs(text, head, sep="\n\n"):
    paras = text.split(sep)
    idx = [i for i, p in enumerate(paras) if p.lstrip("\n").startswith(head)]
    assert idx, head
    if any(p.lstrip("\n").startswith(head.replace("PrestressSteels", "Placements")) for p in paras):
        return text
    new = [re.sub(r"\bPrestressSteel\b", "Placement", p.replace("PrestressSteels", "Placements")) for p in (paras[i] for i in idx)]
    return sep.join(paras[: idx[-1] + 1] + new + paras[idx[-1] + 1:])


def preseed(F):
    d = art.any + "/🧬️schema/🔺️diff/"
    p = d + "🦀️.rs"
    s = F.read(p)
    if "PlacementKey" not in s:
        s = s.replace("    AnchorKey for Anchor;\n", "    AnchorKey for Anchor;\n    PlacementKey for Placement;\n", 1)
        s = s.replace("    pub anchors: Option<En1992AnchorsRows>,\n", "    pub anchors: Option<En1992AnchorsRows>,\n    #[state(artifact)]\n    pub placements: Option<En1992PlacementsRows>,\n", 1)
        s = re.sub(r"(        if let Some\(theirs\) = other\.anchors \{\n.*?\n        \}\n)", lambda m: m.group(1) + m.group(1).replace("anchors", "placements"), s, 1, flags=re.S)
        s = re.sub(r"(        if let Some\(rows\) = &self\.anchors \{\n[^\n]*\n        \}\n)", lambda m: m.group(1) + m.group(1).replace("anchors", "placements"), s, 1)
        s = s.replace(" && self.anchors.as_ref().map_or(true, |rows| rows.is_empty())", " && self.anchors.as_ref().map_or(true, |rows| rows.is_empty()) && self.placements.as_ref().map_or(true, |rows| rows.is_empty())", 1)
        s = re.sub(r"(            anchors: self\.anchors[^\n]*\n)", lambda m: m.group(1) + m.group(1).replace("anchors", "placements"), s, 1)
        F.write(p, s)
    p = d + "🟦️.ts"
    s = F.read(p)
    if "placements: En1992PlacementsRows" not in s:
        s = s.replace("  anchors: En1992AnchorsRows | null;\n", "  anchors: En1992AnchorsRows | null;\n  /** 📍️ @state artifact */\n  placements: En1992PlacementsRows | null;\n", 1)
        s = clone_paragraphs(s, "export interface En1992PrestressSteelsInserted")
        for suffix in ("Modified", "Moved", "Patch", "Removed", "Rows"):
            s = clone_paragraphs(s, "export interface En1992PrestressSteels" + suffix)
        lines = s.split("\n")
        pl = [i for i, l in enumerate(lines) if l.startswith("export const parseEn1992PrestressSteels")]
        new = [re.sub(r"\bparsePrestressSteel\b", "parsePlacement", l.replace("PrestressSteels", "Placements")) for l in (lines[i] for i in pl)]
        lines[pl[-1] + 1: pl[-1] + 1] = new
        s = "\n".join(lines)
        s = re.sub(r"(anchors: normWireDefault\(normWireNullable\(normWireRef\(\(\) => parseEn1992AnchorsRows\)\), \(\) => null\))", r"\1, placements: normWireDefault(normWireNullable(normWireRef(() => parseEn1992PlacementsRows)), () => null)", s, 1)
        F.write(p, s)
    p = d + "🔗️.graphql"
    s = F.read(p)
    if "En1992PlacementsRows" not in s:
        for suffix in ("Patch", "Removed", "Inserted", "Moved", "Modified", "Rows"):
            s = clone_paragraphs(s, "type En1992PrestressSteels" + suffix + " {")
        s = s.replace("  anchors: En1992AnchorsRows @state(class: ARTIFACT)\n", "  anchors: En1992AnchorsRows @state(class: ARTIFACT)\n  placements: En1992PlacementsRows @state(class: ARTIFACT)\n", 1)
        F.write(p, s)
    p = d + "🛰️.proto"
    s = F.read(p)
    if "En1992PlacementsRows" not in s:
        for suffix in ("Patch", "Removed", "Inserted", "Moved", "Modified", "Rows"):
            s = clone_paragraphs(s, "message En1992PrestressSteels" + suffix + " {")
        s = s.replace("  optional En1992AnchorsRows anchors = 9;\n", "  optional En1992AnchorsRows anchors = 9;\n  optional En1992PlacementsRows placements = 10;\n", 1)
        F.write(p, s)
    p = d + "🔣️.json"
    j = json.loads(F.read(p))
    if "placements" not in j["properties"]:
        j["properties"]["placements"] = {"anyOf": [{"$ref": "#/$defs/En1992PlacementsRows"}, {"type": "null"}], "x-semio-state": "artifact"}
        F.write(p, dumps(j))


RETIRE = ["change-member-placement-%s" % f for f in ("x", "y", "z", "yaw")] + ["change-anchor-placement-%s" % f for f in ("x", "y", "z", "yaw")]


def retire_rest(F):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    F.write(p, K.drop_lines(F.read(p), r"^        SetFieldRule \{ path: \"(members|anchors)\[\]\.placement(X|Y|Z|Yaw)\"[^\n]*\n"))
    p = art.harness + "/🦀️.rs"
    s = F.read(p)
    for k in RETIRE:
        s = K.drop_lines(s, r'^    "%s",\n' % k)
        s = re.sub(r'^        "%s" => \(\n(?:            [^\n]*\n)+        \),\n' % k, "", s, flags=re.M)
    F.write(p, s)
    p = art.harness + "/🐍️.py"
    s = F.read(p)
    for k in RETIRE:
        s = K.drop_lines(s, r'^    "%s",\n' % k)
        s = K.drop_lines(s, r'^    "%s": \([^\n]*\n' % k)
    F.write(p, s)


def main():
    F = K.KFiles(STAGE_A)
    dirs, fdirs = [], []
    for k in RETIRE:
        dirs += [os.path.basename(p) for p in glob.glob(art.mut + "/*" + k)]
        fdirs += [os.path.basename(p) for p in glob.glob(art.fix + "/*" + k)]
    assert len(dirs) == len(RETIRE) and len(fdirs) == len(RETIRE), (dirs, fdirs)
    K.retire(art, F, RETIRE, dirs, fdirs)
    retire_rest(F)
    preseed(F)
    schema = json.loads(rd(art.any + "/🧬️schema/📸️snapshot/🔣️.json"))
    defs = schema["$defs"]
    rules = rd(art.mut + "/🧭️edit-rules/🦀️.rs")
    covered_paths = set(re.findall(r'SetFieldRule \{ path: "([^"]+)"', rules))
    existing_kinds = [k for k in K.kinds_of(art, F) if k not in RETIRE]
    leaves, new_kinds = [], []
    for e in ENTS:
        for l in read_leaves(defs, e):
            leaves.append(l)
            l.covered = rule_path(l) in covered_paths
            if not l.covered:
                k = I.SetKind(l)
                k.family = family_of(e, l.field)
                new_kinds.append(k)
    pairs = [("placement", "insert-placement", "remove-placement")]
    for name, ik, rk in pairs:
        ins, rm = I.pair(BY[name], ik, rk, ST, by_index=True)
        for x in (ins, rm):
            x.group, x.family = ROWS[name], "rows"
        new_kinds += [ins, rm]
    new_kinds = [k for k in new_kinds if k.kind not in existing_kinds]
    assert len({k.kind for k in new_kinds}) == len(new_kinds), "duplicate kind names"
    tags = K.binary_protocol_keep(art, F, new_kinds, set(RETIRE))
    existing_crates = set(re.findall(r'"(semio-s-artifact-norm-en1992-mut-[a-z0-9-]+)"', rd(art.root + "/Cargo.toml")))
    groups = K.group_kinds(art, new_kinds, existing_crates)
    names = K.create_crates(art, F, groups)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    K.register(art, F, names)
    K.leaf_files(art, F, new_kinds, defs, tags)
    K.root_wire(art, F, new_kinds, groups_of)
    K.edit_rules(art, F, new_kinds)
    K.text_codec(art, F, new_kinds, groups_of)
    K.harness(art, F, new_kinds)
    K.oracle(art, F, new_kinds)
    K.taxonomy(art, F, new_kinds)
    by = {}
    for e in ENTS:
        by[e.patch] = D.Patch("En1992", e.patch, e.last.row, [], [])
    for l in leaves:
        by[l.ent.patch].fields.append(l)
    for e in ENTS:
        if len(e.links) > 1:
            by["".join(pascal(c.coll) for c in e.links[:-1])].lanes.append((e.last.coll, "En1992%sRows" % e.patch))
    for e in ENTS:
        by[e.patch].lane = e.last.coll
    dp = art.any + "/🧬️schema/🔺️diff/"
    F.write(dp + "🦀️.rs", ED.rust(F.read(dp + "🦀️.rs"), by, MODEL))
    url = "https://json.schemas.assets.semio-tech.com/s/norm/en1992/snapshot.json"
    F.write(dp + "🔣️.json", D.patch_json(F.read(dp + "🔣️.json"), "En1992", by, url, "MembersActions"))
    enums = {l.rust_ty for l in leaves if l.ty.startswith("enum:")} | {s.row_def for s in by.values()}
    F.write(dp + "🟦️.ts", ED.ts_patch(F.read(dp + "🟦️.ts"), "En1992", by, enums))
    F.write(dp + "🔗️.graphql", D.patch_blocks(F.read(dp + "🔗️.graphql"), "En1992", by, "MembersActions", "gql"))
    F.write(dp + "🛰️.proto", D.patch_blocks(F.read(dp + "🛰️.proto"), "En1992", by, "MembersActions", "proto"))
    spec = {e.patch: ([camel(l.field) for l in by[e.patch].fields], [(camel(n), r[len("En1992"): -len("Rows")]) for n, r in by[e.patch].lanes]) for e in ENTS}
    base = augment(json.loads(rd(art.fix + "/↔️change-member-width/✅apply/🔣️.before.json")))
    base["anchors"][0]["actions"] = base["anchors"][0]["actions"] or copy.deepcopy(base["members"][0]["actions"])
    top = list(json.loads(rd(art.fix + "/↔️change-member-width/✅apply/🔣️.diff.json")).keys())
    X.pad_existing(F, art, spec, "En1992")
    X.build(art, F, new_kinds, defs, base, spec, top, new_row)
    print("leaves", len(leaves), "covered", sum(l.covered for l in leaves), "new kinds", len(new_kinds), "crates", len(names), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


if __name__ == "__main__":
    shutil.rmtree(STAGE_A, ignore_errors=True)
    main()
