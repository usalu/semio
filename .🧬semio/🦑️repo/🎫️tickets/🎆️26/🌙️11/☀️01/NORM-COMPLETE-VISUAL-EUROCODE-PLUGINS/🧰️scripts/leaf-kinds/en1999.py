"""EN 1999 profile and driver: python -I en1999.py [--apply]."""
import copy, glob, json, re, sys, shutil, os
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import leaf as L, diff as D, wire as W, fixtures as X, model as M, en1999_rows as R

art = W.Art("en1999", "🪶️", "🪶️", None, None)
art.anchor_crate = "semio-s-artifact-norm-en1999-mut-members"
art.anchor_dir = "📏️members"
L.init("en1999")
M.LIMIT[0] = 33
E = M.Ent
ENTS = [
    E("material", "⚗️", [("materials", "AluminiumMaterial", "material_id")], "Material", "Werkstoff"),
    E("section", "📐️", [("sections", "AluminiumSection", "section_id")], "Section", "Querschnitt"),
    E("section-element", "🧱️", [("sections", "AluminiumSection", "section_id"), ("elements", "PlateElement", "element_id")], "Section element", "Querschnittsteil", slug="element"),
    E("member", "🏗️", [("members", "AluminiumMember", "member_id")], "Member", "Bauteil"),
    E("member-action", "🏋️", [("members", "AluminiumMember", "member_id"), ("actions", "MemberAction", "action_id")], "Member action", "Bauteileinwirkung", slug="action"),
    E("connection", "🔗️", [("connections", "AluminiumConnection", "connection_id")], "Connection", "Anschluss", slug="conn"),
    E("connection-action", "🔗️", [("connections", "AluminiumConnection", "connection_id"), ("actions", "MemberAction", "action_id")], "Connection action", "Anschlusseinwirkung", slug="conn-action"),
    E("fire-scenario", "🔥️", [("fire_scenarios", "FireScenario", "fire_scenario_id")], "Fire scenario", "Brandszenario", slug="fire"),
    E("fatigue-detail", "🔄️", [("fatigue_details", "FatigueDetail", "fatigue_detail_id")], "Fatigue detail", "Ermüdungsdetail", slug="fatigue"),
    E("cold-formed", "❄️", [("cold_formed", "ColdFormedSheet", "cold_formed_id")], "Cold-formed sheet", "Kaltgeformtes Blech", slug="sheet"),
    E("cold-formed-action", "❄️", [("cold_formed", "ColdFormedSheet", "cold_formed_id"), ("actions", "MemberAction", "action_id")], "Cold-formed sheet action", "Einwirkung auf kaltgeformtes Blech", slug="sheet-action"),
    E("shell", "🫙", [("shells", "AluminiumShell", "shell_id")], "Shell", "Schale"),
    E("shell-action", "🫙", [("shells", "AluminiumShell", "shell_id"), ("actions", "MemberAction", "action_id")], "Shell action", "Schaleneinwirkung"),
    E("weld-run", "🧵️", [("weld_runs", "WeldRun", "weld_run_id")], "Weld run", "Schweißnaht"),
    E("outline-vertex", "🔷️", [("outline_vertices", "SectionVertex", "outline_vertex_id")], "Outline vertex", "Umrissknoten", slug="vertex"),
    E("fatigue-category", "🎯️", [("fatigue_categories", "FatigueCategory", "fatigue_category_id")], "Fatigue detail site", "Ermüdungsdetail-Stelle", slug="category"),
    E("patch-load", "⬇️", [("patch_loads", "PatchLoad", "patch_load_id")], "Web patch load", "Stegeinzellast", slug="patch"),
    E("ring-stiffener", "⭕️", [("ring_stiffeners", "RingStiffener", "ring_stiffener_id")], "Ring stiffener", "Ringsteife", slug="ring"),
    E("transverse-stiffener", "🦴️", [("transverse_stiffeners", "TransverseStiffener", "transverse_stiffener_id")], "Transverse stiffener", "Quersteife", slug="transverse"),
    E("placement", "📍️", [("placements", "Placement", "placement_id")], "Placement", "Platzierung"),
]
COVERED = {("material", "designation"), ("section-element", "thickness"), ("member", "buckling_length_z"), ("member-action", "n_k"), ("member-action", "m_y_k"), ("connection", "welds_throat"), ("connection", "bolts_rows")}
WHOLE = ["change-materials", "change-sections", "change-members", "change-connections", "change-fire-scenarios", "change-fatigue-details", "change-cold-formed", "change-shells"]
L.TARGET_DE.update({"material_id": "Werkstoff", "section_id": "Querschnitt", "element_id": "Querschnittsteil", "member_id": "Bauteil", "action_id": "Einwirkung", "connection_id": "Anschluss", "fire_scenario_id": "Brandszenario", "fatigue_detail_id": "Ermüdungsdetail", "cold_formed_id": "Kaltgeformtes Blech", "shell_id": "Schale", "weld_run_id": "Schweißnaht", "outline_vertex_id": "Umrissknoten", "placement_id": "Platzierung", "fatigue_category_id": "Ermüdungsdetail-Stelle", "patch_load_id": "Stegeinzellast", "ring_stiffener_id": "Ringsteife", "transverse_stiffener_id": "Quersteife"})
CHOICES = {("weld-run", "process"): "tig", ("weld-run", "direction"): "transverse", ("weld-run", "element_id"): "web", ("weld-run", "member_id"): "post-chs", ("outline-vertex", "section_id"): "sec-chs168", ("outline-vertex", "element_id"): "web", ("outline-vertex", "ring"): "hole", ("fatigue-category", "detail_type"): "fillet-weld", ("fatigue-category", "member_id"): "post-chs", ("patch-load", "position"): "interior", ("patch-load", "subject_id"): "post-chs", ("ring-stiffener", "shell_id"): "shell-pass", ("transverse-stiffener", "sides"): "one", ("transverse-stiffener", "girder_id"): "post-chs"}
NEW_ROWS = {
    "placements": [{"id": "purlin-1", "x": 1.0, "y": 2.0, "z": 0.5, "yaw": 0.25}],
    "weldRuns": [{"id": "weld-1", "memberId": "purlin-1", "elementId": "flange-top", "start": 0.5, "length": 2.0, "offset": 0.0, "throat": 0.004, "process": "mig", "tJunction": False, "direction": "longitudinal", "hazExtent": 0.0}],
    "fatigueCategories": [{"id": "site-1", "memberId": "purlin-1", "elementId": "", "detailCategory": "40", "detailType": "butt-weld", "locationX": 1.5, "m1": 3.4, "m2": 5.4}],
    "patchLoads": [{"id": "patch-1", "subjectId": "purlin-1", "position": "end", "x": 0.0, "bearingLength": 0.05, "webHeight": 0.0, "webAngle": 0.0, "cornerRadius": 0.004, "load": 12000.0}],
    "ringStiffeners": [{"id": "ring-1", "shellId": "shell-pass", "x": 1.0, "depth": 0.06, "thickness": 0.006, "internal": False}],
    "transverseStiffeners": [{"id": "stiffener-1", "girderId": "purlin-1", "position": 1.5, "width": 0.08, "thickness": 0.008, "sides": "two"}],
    "outlineVertices": [{"id": "v-1", "sectionId": "sec-i120", "ring": "outer", "elementId": "web", "y": 0.0, "z": 0.0}, {"id": "v-2", "sectionId": "sec-i120", "ring": "outer", "elementId": "web", "y": 0.06, "z": 0.0}, {"id": "v-3", "sectionId": "sec-i120", "ring": "outer", "elementId": "", "y": 0.06, "z": 0.12}],
}
_new_value = X.new_value


def new_value(doc, leaf, old, defs):
    return CHOICES.get((leaf.ent.name, leaf.field)) or _new_value(doc, leaf, old, defs)


X.new_value = new_value


def rule_path(l):
    return ".".join(camel(c[0]) + "[]" for c in l.ent.chain[:-1]) + (".") * bool(l.ent.chain[:-1]) + camel(l.ent.chain[-1][0]) + "[]." + ".".join(camel(p) for p in l.path)


def pad_top(F, art, top):
    for p in glob.glob(art.fix + "/*/*/🔣️.diff.json"):
        d = json.loads(F.read(p))
        F.write(p, dumps({**d, **{k: None for k in top if k not in d}}))


def main():
    F = Files(STAGE)
    schema_path = art.any + "/🧬️schema/📸️snapshot/🔣️.json"
    leaves, defs = M.read_entities(schema_path, ENTS, COVERED)
    rs = rd(art.any + "/🧬️schema/📸️snapshot/🦀️.rs")
    leaves = [l for l in leaves if re.search(r"pub %s: " % l.path[-1], rs)]
    missing_ui = [l.kind for l in leaves if not (l.ui or {}).get("label")]
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M))
    new = [l for l in leaves if not l.covered and l.kind not in existing]
    groups, order = W.group_leaves(art, new)
    rows = R.kinds()
    groups.update(R.pseudo_group(rows))
    names = W.create_crates(art, F, groups, art.mut + "/" + art.anchor_dir)
    groups_of = {l.kind: names[g][0] for g, ls in groups.items() for l in ls}
    kinds_all = [k for k in re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M)] + [l.kind for l in new] + [k.kind for k in rows if k.kind not in existing]
    rows = [k for k in rows if k.kind not in existing]
    tags = {k: i for i, k in enumerate(kinds_all)}
    W.register(art, F, names)
    W.leaf_files(art, F, new, defs, tags)
    W.mutation_twins(art, F, new, groups_of)
    if rows:
        R.write_kinds(art, F, rows, tags)
        R.wire_twins(art, F, rows, groups_of)
        R.edit_rules(art, F, rows)
    W.edit_rules(art, F, new, rule_path, WHOLE)
    W.text_codec(art, F, new, groups_of)
    if rows:
        R.text_codec(art, F, rows, groups_of)
    W.binary_protocol(art, F, kinds_all)
    W.harness(art, F, new + rows, None)
    W.oracle(art, F, new)
    W.taxonomy(art, F, new)
    spec = D.specs("En1999", ENTS, leaves, "m")
    dp = art.any + "/🧬️schema/🔺️diff/"
    model = "semio_s_artifact_norm_en1999_model"
    t = F.read(dp + "🦀️.rs")
    F.write(dp + "🦀️.rs", D.patch_rust(t, "En1999", spec, model, "MembersActions", None))
    F.write(dp + "🔣️.json", D.patch_json(F.read(dp + "🔣️.json"), "En1999", spec, "https://json.schemas.assets.semio-tech.com/s/norm/en1999/snapshot.json", "MembersActions"))
    F.write(dp + "🟦️.ts", D.patch_ts(F.read(dp + "🟦️.ts"), "En1999", spec, "MembersActions"))
    F.write(dp + "🔗️.graphql", D.patch_blocks(F.read(dp + "🔗️.graphql"), "En1999", spec, "MembersActions", "gql"))
    F.write(dp + "🛰️.proto", D.patch_blocks(F.read(dp + "🛰️.proto"), "En1999", spec, "MembersActions", "proto"))
    base = json.loads(rd(art.fix + "/🏋️change-member-n-ed/✅apply/🔣️.before.json"))
    base.update(copy.deepcopy(NEW_ROWS))
    top = list(json.loads(rd(art.fix + "/🏋️change-member-n-ed/✅apply/🔣️.diff.json")).keys())
    top += [k for k in NEW_ROWS if k not in top]
    X.pad_existing(F, art, spec)
    pad_top(F, art, top)
    X.build(art, F, new, defs, base, spec, top)
    R.TOP_KEYS[:] = top
    if rows:
        R.fixtures(art, F, rows, base)
        R.harness_dupes(art, F, rows)
        R.taxonomy(art, F, rows)
        R.oracle(art, F, rows)
    print("leaves", len(leaves), "new", len(new), "crates", len(names), "missing labels", len(missing_ui), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


if __name__ == "__main__":
    shutil.rmtree(STAGE, ignore_errors=True)
    main()
