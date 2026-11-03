"""EN 1994 positional `placements` lane (`En1994PlacementsRows { removed, inserted, modified }`): diff bodies for the id-addressed placement kinds and their fixtures (leaf-kinds E)."""
import copy, os, sys
sys.path.insert(0, os.path.dirname(__file__))
import index_selector as I, keyed_fixtures as X

HEAD = "use semio_s_artifact_norm_en1994_diff::{%s};\nuse semio_s_artifact_norm_en1994_model::En1994Snapshot;\n\n"

SET = """pub fn diff(payload: &%(rec)s, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.%(new)s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(index) = base.placements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Placement {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if base.placements[index].%(f)s == payload.%(new)s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The placement already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { placements: Some(En1994PlacementsRows { modified: vec![En1994PlacementsPatch { index, %(f)s: Some(payload.%(new)s), ..Default::default() }], ..Default::default() }), ..Default::default() })
}
"""

INSERT = """pub fn diff(payload: &%(rec)s, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement {key} already exists."), [key]);
    }
    if payload.index > base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.placements.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1994Diff { placements: Some(En1994PlacementsRows { inserted: vec![En1994PlacementsInserted { index: payload.index, row: payload.item.clone() }], ..Default::default() }), ..Default::default() })
}
"""

REMOVE = """pub fn diff(payload: &%(rec)s, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index >= base.placements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "placements index out of range", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { placements: Some(En1994PlacementsRows { removed: vec![payload.index], ..Default::default() }), ..Default::default() })
}
"""


def positional(kinds):
    for k in kinds:
        if isinstance(k, I.SetKind):
            imports, body = "En1994Diff, En1994PlacementsPatch, En1994PlacementsRows", SET % dict(rec=k.record, new=k.leaf.new, f=k.leaf.field)
        elif isinstance(k, I.InsertKind):
            imports, body = "En1994Diff, En1994PlacementsInserted, En1994PlacementsRows", INSERT % dict(rec=k.record)
        else:
            imports, body = "En1994Diff, En1994PlacementsRows", REMOVE % dict(rec=k.record)
        text = "//! 📍️ `%s` diff on the positional `placements` lane.\n\nuse crate::%s::%s;\n" % (k.kind, k.module, k.record) + HEAD % imports + body
        k.diff_rs = (lambda t: (lambda st: t))(text)


def fixtures(art, F, kinds, base, blank):
    for k in kinds:
        after = copy.deepcopy(base)
        diff = copy.deepcopy(blank)
        lane = {"removed": [], "inserted": [], "modified": []}
        row0 = base["placements"][0]
        new = {"id": row0["id"] + "-b", "x": 3.0, "y": 4.0, "z": 0.0, "yaw": 1.5}
        if isinstance(k, I.SetKind):
            f = k.leaf.field
            after["placements"][0][f] = 2.5
            fields = {"id": row0["id"], k.leaf.newc: 2.5}
            lane["modified"] = [dict({"index": 0, "x": None, "y": None, "z": None, "yaw": None}, **{f: 2.5})]
        elif isinstance(k, I.InsertKind):
            after["placements"].insert(1, new)
            fields = {"index": 1, "item": new}
            lane["inserted"] = [{"index": 1, "row": new}]
        else:
            after["placements"].pop(0)
            fields = {"index": 0}
            lane["removed"] = [0]
        diff["placements"] = lane
        X.write(art, F, k, base, after, fields, diff)
