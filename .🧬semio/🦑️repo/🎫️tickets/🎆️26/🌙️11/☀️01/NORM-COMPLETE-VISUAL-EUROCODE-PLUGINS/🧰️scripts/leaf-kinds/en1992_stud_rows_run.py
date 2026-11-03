"""EN 1992 stud rows driver (v14): python -I en1992_stud_rows_run.py [--apply]. Retires the six `*-stud-ring-*` kinds and adds `insert-stud-row`, `remove-stud-row`, `change-stud-row-{angle,count,spacing-first,spacing-radial,diameter}` for `members[].studRows[]` (6.4.5)."""
import copy, glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I, keyed as K, keyed_fixtures as X, diff as D, en1992_diff as ED

ART = "en1992"
STAGE_A = TICKET + "/🗑️generated/leaf-kinds/stage-stud-rows/en1992"
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
STUD = L("stud_rows", "StudRow", "stud_row_id", "stud_row", "Stud row", "Dübelreihe")
PLAC = L("placements", "Placement", "placement_id", "placement", "Placement", "Platzierung")
OPEN = L("openings", "Opening", "opening_id", "opening", "Opening", "Öffnung")
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
    E("stud-row", "🔩", [MEMBER, STUD], "Stud row", "Dübelreihe", group="geometry"),
    E("opening", "🕳️", [MEMBER, OPEN], "Opening", "Öffnung", group="geometry"),
    E("anchor", "⚓️", [ANCH], "Anchor", "Dübel", group="anchors"),
    E("anchor-action", "🏋️", [ANCH, AACT], "Anchor action", "Dübeleinwirkung", group="anchor-actions"),
    E("placement", "📍️", [PLAC], "Placement", "Platzierung", slug="placement", group="placements"),
]
BY = {e.name: e for e in ENTS}
for e in ENTS:
    e.chain = [(l.coll, l.row, l.sel) for l in e.links]
    e.patch = "".join(pascal(l.coll) for l in e.links)

FAMILIES = {
    "member": {"identity": ["label_en", "label_de", "kind", "support", "use_fem", "deflection_sensitive", "concrete_grade_id", "reinforcement_grade_id"], "placement": ["prestress_steel_id", "placement_x", "placement_y", "placement_z", "placement_yaw"], "shear": ["stirrups_diameter", "stirrups_legs", "punching_column_width", "punching_column_depth", "punching_column_position", "punching_asw", "punching_beta_method", "punching_moment_c1", "punching_moment_c2"], "prestress-fire": ["prestress_force", "prestress_area", "prestress_eccentricity", "prestress_loss_ratio", "fire_column_method", "fire_slab_system"]},
    "bar-point": {"section-bars": ["layer_id", "y", "z", "diameter"], "bar-fields": ["length", "bend"]},
    "anchor": {"capacity": ["cracked", "f_uk", "f_yk", "d", "c1", "f_ck"], "placement": ["head_diameter", "placement_x", "placement_y", "placement_z", "placement_yaw"]},
    "anchor-action": {"identity": ["kind", "category", "source"], "loads": ["g_k_line", "q_k_line", "point_force"], "resultants": ["m_k", "n_k", "v_k", "t_k", "v_k_punch"]},
    "action": {"identity": ["kind", "category", "source"], "loads": ["g_k_line", "q_k_line", "point_force", "t_k", "v_k_punch"]},
}
ENT_FAMILY = {"section-point": "section-bars", "bar-point": "section-bars", "tendon-point": "tendons-studs", "stud-row": "tendons-studs", "opening": "tendons-studs"}
ROWS = {"concrete-grade": "grades", "reinforcement-grade": "grades", "prestress-steel": "grades", "bar-layer": "member-lists", "action": "member-lists", "section-point": "member-lists", "bar-point": "member-geometry", "tendon-point": "member-geometry", "stud-row": "member-geometry", "placement": "placements", "opening": "member-geometry", "anchor-action": "anchor-lists"}


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


def model_types():
    """(struct, field) -> Rust type as the current model declares it: leaves the JSON schema types as inline string enums are closed Rust enums there."""
    text = rd(art.any + "/🧬️schema/📸️snapshot/🦀️.rs")
    out = {}
    for m in re.finditer(r"pub struct (\w+) \{(.*?)\n\}", text, re.S):
        for f in re.finditer(r"    pub (\w+): ([^,\n]+),", m.group(2)):
            out[(m.group(1), f.group(1))] = f.group(2).strip()
    return out


def read_leaves(defs, ent):
    types = model_types()
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
        rust = types.get((l.owner, ED.RUST_FIELD.get(l.path[-1], l.path[-1])), "")
        if re.fullmatch(r"[A-Z]\w+", rust) and rust != "String" and not l.ty.startswith("enum:"):
            l.ty = "enum:" + rust
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
    m["punching"] = m.get("punching") or {"columnWidth": 0.3, "columnDepth": 0.3, "columnPosition": "Interior", "asw": 0.0005, "betaMethod": "Simplified", "momentC1": 0.0, "momentC2": 0.0}
    m["prestress"] = m.get("prestress") or {"force": 1000000.0, "area": 0.0014, "eccentricity": 0.1, "lossRatio": 0.15}
    m["fire"] = m.get("fire") or {"rating": "R90", "axisDistance": 0.04, "columnMethod": "", "slabSystem": ""}
    m["stirrups"] = m.get("stirrups") or {"diameter": 0.008, "spacing": 0.15, "legs": 2}
    m["sectionPoints"] = m["sectionPoints"] or [{"id": "sp-1", "y": 0.0, "z": 0.0}, {"id": "sp-2", "y": 0.35, "z": 0.0}]
    m["bars"] = m["bars"] or [{"id": "bar-1", "layerId": m["longitudinal"][0]["id"], "y": 0.05, "z": 0.05, "diameter": 0.016, "length": 0.0, "bend": "auto"}]
    m["tendonPoints"] = m["tendonPoints"] or [{"id": "tp-1", "x": 0.0, "y": 0.1, "z": 0.2}]
    m["studRows"] = m.get("studRows") or [{"id": "sr-1", "angle": 0.0, "count": 4, "spacingFirst": 0.1, "spacingRadial": 0.15, "diameter": 0.01}]
    doc["placements"] = doc.get("placements") or [{"id": m["id"], "x": 1.0, "y": 0.5, "z": 0.0, "yaw": 0.25}]
    m["openings"] = m.get("openings") or [{"id": "op-1", "x": 0.6, "y": 0.0, "width": 0.3, "depth": 0.3}]
    return doc


RETIRE = ["change-stud-ring-%s" % f for f in ("radius", "count", "diameter")] + ["insert-stud-ring", "remove-stud-ring"]


def retire_rest(F):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = K.drop_lines(F.read(p), r"^        (SetFieldRule|InsertItemRule|RemoveItemRule) \{ path: \"members\[\]\.studRings(\[\]\.\w+)?\"[^\n]*\n")
    F.write(p, s)
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
    pairs = [("concrete-grade", "insert-concrete-grade", "remove-concrete-grade"), ("reinforcement-grade", "insert-reinforcement-grade", "remove-reinforcement-grade"), ("prestress-steel", "insert-prestress-steel", "remove-prestress-steel"), ("bar-layer", "insert-bar-layer", "remove-bar-layer"), ("action", "insert-action", "remove-action"), ("section-point", "insert-section-point", "remove-section-point"), ("bar-point", "insert-bar-point", "remove-bar-point"), ("tendon-point", "insert-tendon-point", "remove-tendon-point"), ("stud-row", "insert-stud-row", "remove-stud-row"), ("opening", "insert-opening", "remove-opening"), ("anchor-action", "insert-anchor-action", "remove-anchor-action")]
    for name, ik, rk in pairs:
        ins, rm = I.pair(BY[name], ik, rk, ST, by_index=True)
        for x in (ins, rm):
            x.group, x.family = ROWS[name], "rows"
        new_kinds += [ins, rm]
    new_kinds = [k for k in new_kinds if k.kind not in existing_kinds]
    assert len({k.kind for k in new_kinds}) == len(new_kinds), "duplicate kind names"
    all_kinds = existing_kinds + [k.kind for k in new_kinds]
    tags = K.binary_protocol_keep(art, F, new_kinds, set(RETIRE))
    existing_crates = set(re.findall(r'(semio-s-artifact-norm-en1992-mut-[a-z0-9-]+)["\]]', rd(art.root + "/Cargo.toml")))
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
    if new_kinds:
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
