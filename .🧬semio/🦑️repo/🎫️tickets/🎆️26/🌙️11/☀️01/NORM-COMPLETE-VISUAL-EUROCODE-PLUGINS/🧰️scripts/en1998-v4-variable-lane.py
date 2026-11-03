#!/usr/bin/env python3
"""EN 1998 v4: replaces the whole-Vec `variables` setter of the storey, bridge and tower patches by a keyed element lane `En1998VariableActionDelta` (diff rs + ts + json + graphql + proto), rewrites the 12 variable kinds to emit per-element removal / insertion / modification, and hand-fixes every committed diff fixture. Idempotent.
Usage: /home/deck/semio/.venv/bin/python -I en1998-v4-variable-lane.py [--dry]"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, os, pathlib, re, sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from schema_json import dump_schema

DRY = "--dry" in sys.argv
ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"
D = "🧬️schema/🔺️diff/"
M = "🧬️schema/🧬️mutations/"
V = "En1998VariableAction"
NM = lambda k: f"{V}{k}"
EMPTY = {"removed": [], "inserted": [], "moved": [], "modified": []}


def rd(p):
    return open(ANY + p, encoding="utf-8").read()


def wr(p, s):
    assert len((ANY + p).encode()) < 256 + 80, p
    if not DRY:
        open(ANY + p, "w", encoding="utf-8").write(s)


def edit(p, fn):
    s = rd(p)
    t = fn(s)
    if t != s:
        wr(p, t)
        print("  edited", p[-60:])


def sub1(s, pat, repl, flags=0):
    n = len(re.findall(pat, s, flags))
    assert n == 1, (pat[:100], n)
    return re.sub(pat, repl, s, count=1, flags=flags)


def sub3(s, pat, repl, flags=0):
    n = len(re.findall(pat, s, flags))
    assert n == 3, (pat[:100], n)
    return re.sub(pat, repl, s, flags=flags)


def diff_rs(s):
    if NM("Delta") in s:
        return s
    block = (
        "/// 🔑️ Key extractor of the foreign `En1998VariableAction` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct En1998VariableActionKey;\n\n"
        "impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998VariableAction> for En1998VariableActionKey {\n    type Key = String;\n    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998VariableAction) -> String {\n        row.id.clone()\n    }\n}\n\n"
        "protocol::row_patch! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 🩹 Sparse field patch of one `En1998VariableAction`.\n"
        "    pub En1998VariableActionPatch of semio_s_artifact_norm_en1998_model::En1998VariableAction { set { category: String, qk_n: f64 } }\n}\n\n"
        "protocol::list_delta! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 📋️ Keyed row delta of the `variables` list of a storey, bridge or tower.\n"
        "    pub En1998VariableActionDelta { removal: En1998VariableActionRemoval, insertion: En1998VariableActionInsertion, relocation: En1998VariableActionRelocation, modification: En1998VariableActionModification, row: semio_s_artifact_norm_en1998_model::En1998VariableAction, patch: En1998VariableActionPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998VariableAction>, key: String = by En1998VariableActionKey }\n}\n\n")
    s = sub1(s, r"//#region 🔖️Diff\n", lambda m: block + m.group(0))
    ty = r"variables: Vec<semio_s_artifact_norm_en1998_model::En1998VariableAction>"
    for row in ("Storey", "Bridge", "Tower"):
        pat = r"(pub En1998%sPatch of semio_s_artifact_norm_en1998_model::En1998%s \{ set \{[^}]*?)%s, ([^}]*\}) \}" % (row, row, re.escape(ty))
        m = re.search(pat, s)
        assert m, row
        s = s[:m.start()] + m.group(1) + m.group(2) + " nest { variables: En1998VariableActionDelta } }" + s[m.end():]
    return s


def diff_ts(s):
    if NM("Delta") in s:
        return s
    old = "variables: (En1998VariableAction[]) | null; "
    s = sub3(s, re.escape(old), "")
    for row in ("Storey", "Bridge", "Tower"):
        s = sub1(s, r"(export interface En1998%sPatch \{[^\n]*?) \}\n" % row, lambda m: m.group(1) + " variables: En1998VariableActionDelta; }\n")
    ifs = (f"export interface {NM('Patch')} {{ category: (string) | null; qkN: (number) | null; }}\n"
           f"export interface {NM('Removal')} {{ id: string; index: number; }}\nexport interface {NM('Insertion')} {{ index: number; row: {V}; }}\n"
           f"export interface {NM('Relocation')} {{ id: string; from: number; to: number; }}\nexport interface {NM('Modification')} {{ id: string; patch: {NM('Patch')}; }}\n"
           f"export interface {NM('Delta')} {{ removed: {NM('Removal')}[]; inserted: {NM('Insertion')}[]; moved: {NM('Relocation')}[]; modified: {NM('Modification')}[]; }}\n")
    s = sub1(s, r"export interface En1998StoreyPatch ", lambda m: ifs + m.group(0))
    pold = "variables: normWireDefault(normWireNullable(normWireArray(parseEn1998VariableAction)), () => null), "
    s = sub3(s, re.escape(pold), "")
    empty = "() => ({ removed: [], inserted: [], moved: [], modified: [] })"
    for row in ("Storey", "Bridge", "Tower"):
        s = sub1(s, r"(export const parseEn1998%sPatch: [^\n]*?) \}\);\n" % row, lambda m: m.group(1) + f", variables: normWireDefault(normWireRef(() => parse{NM('Delta')}), {empty}) }});\n")
    rdr = (f"export const parse{NM('Patch')}: NormWireReader<{NM('Patch')}> = normWireObject<{NM('Patch')}>({{ category: normWireDefault(normWireNullable(normWireString), () => null), qkN: normWireDefault(normWireNullable(normWireNumber), () => null) }});\n"
           f"export const parse{NM('Removal')}: NormWireReader<{NM('Removal')}> = normWireObject<{NM('Removal')}>({{ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) }});\n"
           f"export const parse{NM('Insertion')}: NormWireReader<{NM('Insertion')}> = normWireObject<{NM('Insertion')}>({{ index: normWireRequired(normWireInteger), row: normWireRequired(parse{V}) }});\n"
           f"export const parse{NM('Relocation')}: NormWireReader<{NM('Relocation')}> = normWireObject<{NM('Relocation')}>({{ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) }});\n"
           f"export const parse{NM('Modification')}: NormWireReader<{NM('Modification')}> = normWireObject<{NM('Modification')}>({{ id: normWireRequired(normWireString), patch: normWireRequired(parse{NM('Patch')}) }});\n"
           f"export const parse{NM('Delta')}: NormWireReader<{NM('Delta')}> = normWireObject<{NM('Delta')}>({{ " + ", ".join(f"{k}: normWireDefault(normWireArray(parse{NM(v)}), () => [])" for k, v in (("removed", "Removal"), ("inserted", "Insertion"), ("moved", "Relocation"), ("modified", "Modification"))) + " });\n")
    return sub1(s, r"export const parseEn1998StoreyPatch: ", lambda m: rdr + m.group(0))


def diff_graphql(s):
    if NM("Delta") in s:
        return s
    s = sub3(s, r"  variables: String\n", f"  variables: {NM('Delta')}!\n")
    t = (f"\ntype {NM('Delta')} {{\n  removed: [{NM('Removal')}!]!\n  inserted: [{NM('Insertion')}!]!\n  moved: [{NM('Relocation')}!]!\n  modified: [{NM('Modification')}!]!\n}}\n"
         f"\ntype {NM('Removal')} {{\n  id: String!\n  index: Int!\n}}\n\ntype {NM('Insertion')} {{\n  index: Int!\n  row: {V}!\n}}\n\ntype {NM('Relocation')} {{\n  id: String!\n  from: Int!\n  to: Int!\n}}\n\ntype {NM('Modification')} {{\n  id: String!\n  patch: {NM('Patch')}!\n}}\n"
         f"\ntype {NM('Patch')} {{\n  category: String\n  qkN: Float\n}}\n")
    return s.rstrip("\n") + "\n" + t


def diff_proto(s):
    if NM("Delta") in s:
        return s
    s = sub3(s, r"  optional bytes variables = (\d+);\n", lambda m: f"  {NM('Delta')} variables = {m.group(1)};\n")
    t = (f"\nmessage {NM('Removal')} {{\n  string id = 1;\n  uint32 index = 2;\n}}\n\nmessage {NM('Insertion')} {{\n  uint32 index = 1;\n  bytes row = 2;\n}}\n\nmessage {NM('Relocation')} {{\n  string id = 1;\n  uint32 from = 2;\n  uint32 to = 3;\n}}\n\nmessage {NM('Modification')} {{\n  string id = 1;\n  {NM('Patch')} patch = 2;\n}}\n"
         f"\nmessage {NM('Delta')} {{\n  repeated {NM('Removal')} removed = 1;\n  repeated {NM('Insertion')} inserted = 2;\n  repeated {NM('Relocation')} moved = 3;\n  repeated {NM('Modification')} modified = 4;\n}}\n"
         f"\nmessage {NM('Patch')} {{\n  optional string category = 1;\n  optional double qk_n = 2;\n}}\n")
    return s.rstrip("\n") + "\n" + t


def diff_json(s):
    d = json.loads(s)
    if NM("Delta") in d["$defs"]:
        return s
    ref = {"$ref": f"#/$defs/{NM('Delta')}"}
    for row in ("Storey", "Bridge", "Tower"):
        props = d["$defs"][f"En1998{row}Patch"]["properties"]
        assert "variables" in props
        props["variables"] = ref
    base = "https://json.schemas.assets.semio-tech.com/s/norm/en1998/snapshot.json#/$defs/"
    uint = {"type": "integer", "minimum": 0, "maximum": 4294967295}
    arr = lambda k: {"type": "array", "items": {"$ref": f"#/$defs/{NM(k)}"}}
    nul = lambda t: {"anyOf": [{"type": t}, {"type": "null"}]}
    new = {
        NM("Delta"): {"type": "object", "additionalProperties": False, "properties": {"removed": arr("Removal"), "inserted": arr("Insertion"), "moved": arr("Relocation"), "modified": arr("Modification")}},
        NM("Removal"): {"type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": uint}},
        NM("Insertion"): {"type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": uint, "row": {"$ref": base + V}}},
        NM("Relocation"): {"type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": uint, "to": uint}},
        NM("Modification"): {"type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": {"$ref": f"#/$defs/{NM('Patch')}"}}},
        NM("Patch"): {"type": "object", "additionalProperties": False, "properties": {"category": nul("string"), "qkN": nul("number")}},
    }
    defs = {}
    for k, v in d["$defs"].items():
        if k == "En1998StoreyPatch":
            defs.update(new)
        defs[k] = v
    d["$defs"] = defs
    return dump_schema(d)


ENTS = {
    "storey": dict(Pascal="Storey", rel=("building_index", "storey_index"), top=None),
    "bridge": dict(Pascal="Bridge", rel=("bridge_index",), top="bridges"),
    "tower": dict(Pascal="Tower", rel=("tower_index",), top="towers"),
}
OPS = {
    "insert": ("➕️", "insert-{e}-variable", "Insert{P}Variable"),
    "remove": ("➖️", "remove-{e}-variable", "Remove{P}Variable"),
    "category": ("✏️", "change-{e}-variable-category", "Change{P}VariableCategory"),
    "qk-n": ("✏️", "change-{e}-variable-qk-n", "Change{P}VariableQkN"),
}


def kind_diff(e, op):
    P = ENTS[e]["Pascal"]
    emoji, kind_t, rec_t = OPS[op]
    kind, rec = kind_t.format(e=e), rec_t.format(P=P)
    mod = kind.replace("-", "_")
    head = f"//! {emoji} `{kind}` diff — emits one keyed element entry in the `variables` lane of the addressed {e}; a missing target is a `mutation.target-missing`.\n\n"
    if e == "storey":
        imp = f"use semio_s_artifact_norm_en1998_diff::{{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998StoreyDelta, En1998StoreyPatch, En1998VariableActionDelta{', En1998VariableActionPatch' if op in ('category', 'qk-n') else ''}}};\n"
        find = ("    let Some(building) = base.buildings.get(payload.building_index) else {\n        return protocol::MutationOutcome::error(\"mutation.target-missing\", \"building\", Vec::<String>::new());\n    };\n"
                "    let Some(row) = building.storeys.get(payload.storey_index) else {\n        return protocol::MutationOutcome::error(\"mutation.target-missing\", \"storey\", Vec::<String>::new());\n    };\n")
        wrap = ("    let nested = En1998StoreyDelta::modification(&row.id, En1998StoreyPatch { variables: delta, ..Default::default() });\n"
                "    let patch = En1998BuildingPatch { storeys: nested, ..Default::default() };\n"
                "    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&building.id, patch), ..Default::default() })\n")
    else:
        T = P
        imp = f"use semio_s_artifact_norm_en1998_diff::{{En1998Diff, En1998{T}Delta, En1998{T}Patch, En1998VariableActionDelta{', En1998VariableActionPatch' if op in ('category', 'qk-n') else ''}}};\n"
        find = f"    let Some(row) = base.{ENTS[e]['top']}.get(payload.{e}_index) else {{\n        return protocol::MutationOutcome::error(\"mutation.target-missing\", \"{e}\", Vec::<String>::new());\n    }};\n"
        wrap = (f"    let patch = En1998{T}Patch {{ variables: delta, ..Default::default() }};\n"
                f"    protocol::MutationOutcome::new(En1998Diff {{ {ENTS[e]['top']}: En1998{T}Delta::modification(&row.id, patch), ..Default::default() }})\n")
    head += imp + "use semio_s_artifact_norm_en1998_model::En1998Snapshot;\n" + f"use crate::{mod}::{rec};\n\n"
    sig = f"pub fn diff(payload: &{rec}, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {{\n"
    if op == "insert":
        mid = ("    if row.variables.iter().any(|existing| existing.id == payload.variable.id) {\n"
               "        return protocol::MutationOutcome::fatal(\"mutation.duplicate-id\", format!(\"Variable action id {} already exists.\", payload.variable.id), [payload.variable.id.clone()]);\n    }\n"
               "    let index = payload.index.unwrap_or(usize::MAX).min(row.variables.len());\n"
               "    let delta = En1998VariableActionDelta::insertion(index, payload.variable.clone());\n")
    elif op == "remove":
        mid = ("    if payload.variable_index >= row.variables.len() {\n"
               "        return protocol::MutationOutcome::error(\"mutation.target-missing\", format!(\"variable #{}\", payload.variable_index), [payload.variable_index.to_string()]);\n    }\n"
               "    let delta = En1998VariableActionDelta::removal(&row.variables, payload.variable_index);\n")
    else:
        field, val = ("category", "payload.new_category.clone()") if op == "category" else ("qk_n", "payload.new_qk_n")
        mid = ("    let Some(variable) = row.variables.get(payload.variable_index) else {\n        return protocol::MutationOutcome::error(\"mutation.target-missing\", \"variable\", Vec::<String>::new());\n    };\n"
               f"    let delta = En1998VariableActionDelta::modification(&variable.id, En1998VariableActionPatch {{ {field}: Some({val}), ..Default::default() }});\n")
    return head + sig + find + mid + wrap + "}\n"


def kind_files():
    for e in ENTS:
        for op in OPS:
            emoji, kind_t, _ = OPS[op]
            yield M + emoji + kind_t.format(e=e) + "/🔺️diff/🦀️.rs", kind_diff(e, op)


def walk_patches(node, hits):
    if isinstance(node, dict):
        p = node.get("patch")
        if isinstance(p, dict) and "variables" in p:
            hits.append(p)
        for v in node.values():
            walk_patches(v, hits)
    elif isinstance(node, list):
        for v in node:
            walk_patches(v, hits)


def variable_delta(kind, before, mutation):
    m = mutation
    if "buildingIndex" in m:
        rows = before["buildings"][m["buildingIndex"]]["storeys"]
        row = rows[m["storeyIndex"]]
    elif "bridgeIndex" in m:
        row = before["bridges"][m["bridgeIndex"]]
    else:
        row = before["towers"][m["towerIndex"]]
    vs = row["variables"]
    d = json.loads(json.dumps(EMPTY))
    if "insert-" in kind:
        d["inserted"].append({"index": min(m.get("index") if m.get("index") is not None else len(vs), len(vs)), "row": m["variable"]})
    elif "remove-" in kind:
        d["removed"].append({"id": vs[m["variableIndex"]]["id"], "index": m["variableIndex"]})
    else:
        v = vs[m["variableIndex"]]
        patch = {"category": m.get("newCategory"), "qkN": m.get("newQkN")}
        d["modified"].append({"id": v["id"], "patch": patch})
    return d


def fixtures():
    root = ANY + "🧪️tests/🧫️fixtures/🧬️mutations/"
    n = 0
    for kind in sorted(os.listdir(root)):
        p = root + kind + "/✅apply/🔣️.diff.json"
        if not os.path.exists(p):
            continue
        t = open(p, encoding="utf-8").read()
        if '"variables"' not in t:
            continue
        d = json.loads(t)
        hits = []
        walk_patches(d, hits)
        if not hits:
            continue
        if re.search(r"-variable", kind):
            base = root + kind + "/✅apply/"
            before = json.load(open(base + "🔣️.before.json", encoding="utf-8"))
            mutation = json.load(open(base + "🔣️.mutation.json", encoding="utf-8"))
            for h in hits:
                if isinstance(h["variables"], dict):
                    continue
                if h["variables"] is None:
                    h["variables"] = json.loads(json.dumps(EMPTY))
                else:
                    h["variables"] = variable_delta(kind, before, mutation)
            out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
        else:
            if all(isinstance(h["variables"], dict) for h in hits):
                continue
            assert all(h["variables"] is None for h in hits), kind

            def expand(m):
                ind = m.group(1)
                return f'{ind}"variables": {{\n{ind}  "removed": [],\n{ind}  "inserted": [],\n{ind}  "moved": [],\n{ind}  "modified": []\n{ind}}}'
            out = re.sub(r'^( *)"variables": null', expand, t, flags=re.M)
            assert out != t, kind
        json.loads(out)
        if not DRY:
            open(p, "w", encoding="utf-8").write(out)
        n += 1
    print("  diff fixtures rewritten:", n)


def main():
    print("diff rs"); edit(D + "🦀️.rs", diff_rs)
    print("diff ts"); edit(D + "🟦️.ts", diff_ts)
    print("diff graphql"); edit(D + "🔗️.graphql", diff_graphql)
    print("diff proto"); edit(D + "🛰️.proto", diff_proto)
    print("diff json"); edit(D + "🔣️.json", diff_json)
    print("kind diffs")
    for p, s in kind_files():
        if not os.path.exists(ANY + p):
            print("  missing", p)
            continue
        edit(p, lambda old, s=s: s)
    print("fixtures"); fixtures()


if __name__ == "__main__":
    main()
