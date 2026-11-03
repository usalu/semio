"""EN 1994 nested action kinds: `insert-<entity>-action` / `remove-<entity>-action` for `beams|columns|slabs[].actions`, parents addressed by id (`beamId`), insert by `index` + `action`, removal by action `id` only. Diff bodies drive the keyed `En1994<Coll>ActionsRows` lane (`lanes.py` in `../en1994-nested/` must have run first). Wires crates, root enum/KINDS/ts/json, binary tags, edit rules (`InsertItemRule` + `ID_REMOVALS`), harness, oracle, taxonomy, testkit demo cases and `✅apply` vectors. `python3 -I en1994_actions.py [--apply]`; merge mode, private stage."""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I, keyed as K

SHORT = "en1994"
DIRNAME = next(n for n in os.listdir(NORM + "/🗿️artifacts") if n.endswith(SHORT))
EMOJI = DIRNAME[: -len(SHORT)]
ROOT = NORM + "/🗿️artifacts/" + DIRNAME
ANYD = ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
STAGE_N = TICKET + "/🗑️generated/leaf-kinds/stage-en1994-nested"
TEMPLATE = "change-beam-stud-count"
ST = I.Style("En1994", "semio_s_artifact_norm_en1994_model", "semio_s_artifact_norm_en1994_diff", "En1994Snapshot", "../../../📸️snapshot/🟦️.ts", mode="lanes", opt_index=False, tagged=False, item_field=lambda e: "action", ts_prefix="")

_s = rd(ANYD + "/🧬️schema/🧬️mutations/🦀️.rs")
ANCHOR_CRATE = re.search(r"^use (semio_s_artifact_norm_en1994_mut_[a-z0-9_]+)::%s;" % TEMPLATE.replace("-", "_"), _s, re.M).group(1).replace("_", "-")
_p = re.search(r'\[workspace\.dependencies\.%s\]\npath = "([^"]+)"' % ANCHOR_CRATE, rd(ROOT + "/Cargo.toml")).group(1)
ANCHOR_DIR = _p.split("/🧬️mutations/")[1].split("/")[0]
art = K.Art(SHORT, EMOJI, ST, SHORT, ANCHOR_DIR, ANCHOR_CRATE, TEMPLATE, None)

COLLS = [
    dict(coll="beams", Coll="Beams", single="beam", row="CompositeBeam", action="CharacteristicAction", en="Beam", de="Träger", de_action="Trägereinwirkung", emoji="🏋️", ref="change-beam-action-kind"),
    dict(coll="columns", Coll="Columns", single="column", row="CompositeColumn", action="ColumnAction", en="Column", de="Stütze", de_action="Stützeneinwirkung", emoji="🏛️", ref="change-column-action-kind"),
    dict(coll="slabs", Coll="Slabs", single="slab", row="CompositeSlab", action="CharacteristicAction", en="Slab", de="Decke", de_action="Deckeneinwirkung", emoji="🧱️", ref="change-slab-action-kind"),
]

HEAD = "use crate::%(mod)s::%(rec)s;\nuse semio_s_artifact_norm_en1994_diff::{%(names)s};\nuse semio_s_artifact_norm_en1994_model::En1994Snapshot;\n\n"
LOCATE = '''    let Some((parent_index, parent)) = base.%(coll)s.iter().enumerate().find(|(_, row)| row.id == payload.%(single)s_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("%(en)s {} does not exist.", payload.%(single)s_id), [payload.%(single)s_id.clone()]);
    };
'''
WRAP = "En1994Diff { %(coll)s: Some(En1994%(Coll)sRows { modified: vec![En1994%(Coll)sPatch { index: parent_index, actions: Some(%(lane)s), ..Default::default() }], ..Default::default() }), ..Default::default() }"
INSERT = '''pub fn diff(payload: &%(rec)s, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
%(locate)s    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        let key = payload.action.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%(en)s action {key} already exists."), [key]);
    }
    if payload.index > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(%(wrap)s)
}
'''
REMOVE = '''pub fn diff(payload: &%(rec)s, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
%(locate)s    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("%(en)s action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(%(wrap)s)
}
'''


def entities(defs):
    out = []
    for c in COLLS:
        parent = I.Link(c["coll"], c["row"], c["single"] + "_id", c["single"], c["en"], c["de"])
        child = I.Link("actions", c["action"], "id", "action", c["en"] + " action", c["de_action"])
        ent = I.Ent(c["single"] + "-action", c["emoji"], [parent, child], c["en"] + " action", c["de_action"], slug=c["single"] + "-action", group=c["single"] + "-action-rows")
        ins, rm = I.pair(ent, "insert-%s-action" % c["single"], "remove-%s-action" % c["single"], ST, by_index=False)
        for k in (ins, rm):
            k.group, k.family, k.c = ent.group, "", c
            k.diff_rs = (lambda k: lambda st: lane_diff(k))(k)
        out.append((c, ent, ins, rm))
    return out


def lane_diff(k):
    c = k.c
    d = dict(coll=c["coll"], Coll=c["Coll"], single=c["single"], en=c["en"], rec=k.record, mod=k.module)
    d["locate"] = LOCATE % d
    if isinstance(k, I.InsertKind):
        d["lane"] = "En1994%sActionsRows { inserted: vec![En1994%sActionsInserted { index: payload.index, row: payload.action.clone() }], ..Default::default() }" % (c["Coll"], c["Coll"])
        names = "En1994%sActionsInserted, En1994%sActionsRows, En1994%sPatch, En1994%sRows, En1994Diff" % ((c["Coll"],) * 4)
        d["wrap"] = WRAP % d
        body = INSERT % d
    else:
        d["lane"] = "En1994%sActionsRows { removed: vec![En1994%sActionsRemoved { id: payload.id.clone(), index }], ..Default::default() }" % (c["Coll"], c["Coll"])
        names = "En1994%sActionsRemoved, En1994%sActionsRows, En1994%sPatch, En1994%sRows, En1994Diff" % ((c["Coll"],) * 4)
        d["wrap"] = WRAP % d
        body = REMOVE % d
    d["names"] = names
    return "//! %s `%s` diff on the keyed `actions` lane of `%s`.\n\n" % (c["emoji"], k.kind, c["coll"]) + HEAD % d + body


def fix_rows(kinds, F, base_of, ref_of):
    for k in kinds:
        c = k.c
        base = base_of[c["single"]]
        after = copy.deepcopy(base)
        parent = base[c["coll"]][0]
        acts = after[c["coll"]][0]["actions"]
        diff = {key: None for key in ref_of[c["single"]]}
        patch = {key: None for key in ref_of[c["single"]][c["coll"]]["modified"][0]}
        patch["index"] = 0
        lane = {"removed": [], "inserted": [], "modified": []}
        if isinstance(k, I.InsertKind):
            row = dict(copy.deepcopy(parent["actions"][0]), id=parent["actions"][0]["id"] + "-new")
            acts.insert(1, copy.deepcopy(row))
            fields = {"%sId" % c["single"]: parent["id"], "index": 1, "action": row}
            lane["inserted"] = [{"index": 1, "row": row}]
        else:
            acts.pop(0)
            fields = {"%sId" % c["single"]: parent["id"], "id": parent["actions"][0]["id"]}
            lane["removed"] = [{"id": parent["actions"][0]["id"], "index": 0}]
        patch["actions"] = lane
        diff[c["coll"]] = {"removed": [], "inserted": [], "modified": [patch]}
        d = art.fix + "/" + k.dirname + "/✅apply"
        F.write(d + "/🔣️.before.json", dumps(base))
        F.write(d + "/🔣️.mutation.json", dumps({k.record: fields}))
        F.write(d + "/🔣️.after.json", dumps(after))
        F.write(d + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(d + "/🔣️.diff.json", dumps(diff))


def edit_rules(F, kinds):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    ins, ids = "", ""
    for k in kinds:
        c = k.c
        path = "%s[].actions" % c["coll"]
        sel = 'SelectorKey::Id("%sId")' % c["single"]
        if isinstance(k, I.InsertKind):
            ins += '        InsertItemRule { path: "%s", kind: "%s", selectors: &[%s], index: "index", item: "action" },\n' % (path, k.kind, sel)
        else:
            ids += '        IdRemoval { path: "%s", kind: "%s", selectors: &[%s], id: "id" },\n' % (path, k.kind, sel)
    s = rep(s, "    ],\n    remove_item:", ins + "    ],\n    remove_item:", 1)
    if "ID_REMOVALS" not in s:
        s = rep(s, "use semio_s_plugin_norm_app_surface::{InsertItemRule,", "use semio_s_plugin_norm_app_surface::{IdRemoval, InsertItemRule,", 1)
        s = rep(s, "};\n\nconst CATALOGUE", "};\n\n/// 🆔️ Every collection of this family whose remove kind names the row by its id, with the parent selectors of the payload.\npub const ID_REMOVALS: &[IdRemoval] = &[\n];\n\nconst CATALOGUE", 1)
        s = rep(s, "        _ => EDIT_RULES.resolve(document, edit),", "        _ => EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit),", 1)
        s = rep(s, "/// 🎯️ Resolves an editor edit into the concrete per-leaf kind mutations it denotes through [`EDIT_RULES`]; every editable leaf owns one kind.", "/// 🎯️ Resolves an editor edit into the concrete per-leaf kind mutations it denotes through [`EDIT_RULES`]; every editable leaf owns one kind and a nested action removal is raised by [`ID_REMOVALS`].", 1)
    s = rep(s, "pub const ID_REMOVALS: &[IdRemoval] = &[\n", "pub const ID_REMOVALS: &[IdRemoval] = &[\n" + ids, 1)
    F.write(p, s)


def testkit(F, kinds):
    p = art.mut + "/🧰️testkit/🦀️.rs"
    s = F.read(p)
    if "InsertBeamAction" in s:
        return
    s = rep(s, "pub fn demo_mutation_cases() -> Vec<En1994Mutation> {\n    vec![\n", "pub fn demo_mutation_cases() -> Vec<En1994Mutation> {\n    let base = En1994Snapshot::default();\n    let beam = base.beams[0].id.clone();\n    let column = base.columns[0].id.clone();\n    let slab = base.slabs[0].id.clone();\n    let mut beam_action = base.beams[0].actions[0].clone();\n    beam_action.id = format!(\"{}-added\", beam_action.id);\n    let mut column_action = base.columns[0].actions[0].clone();\n    column_action.id = format!(\"{}-added\", column_action.id);\n    let mut slab_action = base.slabs[0].actions[0].clone();\n    slab_action.id = format!(\"{}-added\", slab_action.id);\n    vec![\n", 1)
    lines = ""
    for k in kinds:
        c = k.c
        if isinstance(k, I.InsertKind):
            lines += "        En1994Mutation::%s(%s::%s { %s_id: %s.clone(), index: 0, action: %s_action.clone() }),\n" % (k.record, k.module, k.record, c["single"], c["single"], c["single"])
        else:
            lines += "        En1994Mutation::%s(%s::%s { %s_id: %s.clone(), id: base.%s[0].actions[0].id.clone() }),\n" % (k.record, k.module, k.record, c["single"], c["single"], c["coll"])
    i = s.index("    ]\n}", s.index("pub fn demo_mutation_cases"))
    F.write(p, s[:i] + lines + s[i:])


def main():
    F = K.KFiles(STAGE_N)
    F.merge = True
    defs = json.loads(rd(art.any + "/🧬️schema/📸️snapshot/🔣️.json"))["$defs"]
    existing = set(K.kinds_of(art, F))
    ents = entities(defs)
    new_kinds = [k for _, _, i, r in ents for k in (i, r) if k.kind not in existing]
    if not new_kinds:
        print("nothing to do: all nested action kinds exist")
        return
    tags = K.binary_protocol_keep(art, F, new_kinds, set())
    existing_crates = set(re.findall(r'"(semio-s-artifact-norm-%s-mut-[a-z0-9-]+)"' % SHORT, rd(art.root + "/Cargo.toml")))
    groups = K.group_kinds(art, new_kinds, existing_crates)
    names = K.create_crates(art, F, groups)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    K.register(art, F, names)
    K.leaf_files(art, F, new_kinds, defs, tags)
    K.root_wire(art, F, new_kinds, groups_of)
    edit_rules(F, new_kinds)
    testkit(F, new_kinds)
    K.harness_rs_py(art, F, new_kinds)
    K.harness(art, F, new_kinds)
    K.oracle(art, F, new_kinds)
    K.taxonomy(art, F, new_kinds)
    base_of, ref_of = {}, {}
    for c in COLLS:
        ref = art.fix + "/" + next(n for n in os.listdir(art.fix) if n.endswith(c["ref"])) + "/✅apply"
        base_of[c["single"]] = json.loads(rd(ref + "/🔣️.before.json"))
        ref_of[c["single"]] = json.loads(rd(ref + "/🔣️.diff.json"))
    fix_rows(new_kinds, F, base_of, ref_of)
    print("kinds", len(new_kinds), "crates", len(names), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply(), "merged", len(F.merged))


if __name__ == "__main__":
    shutil.rmtree(STAGE_N, ignore_errors=True)
    main()
