#!/usr/bin/env python3
"""Generates the EN 1995 placement mutation kinds (`change-placement-x|y|z|yaw`, `insert-placement`, `remove-placement`) into one crate `semio-s-artifact-norm-en1995-mut-placements` (idempotent, path-guarded)."""
import copy, json, os, re

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🪵️en1995"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
CRATE = "semio-s-artifact-norm-en1995-mut-placements"
IDENT = CRATE.replace("-", "_")
CDIR = "📍️placements"
DRY = bool(os.environ.get("DRY"))
CH = "📍️"

def rd(p): return open(p, encoding="utf-8").read()
def wr(p, s):
    assert len(p[len("/home/deck/semio/"):].encode()) < 256, "path of 256 bytes or more: " + p
    if DRY: return
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)
def dumps(o): return json.dumps(o, indent=2, ensure_ascii=False) + "\n"
def rep(s, a, b):
    assert s.count(a) == 1, (s.count(a), a[:90])
    return s.replace(a, b, 1)
def after(s, anchor, add):
    assert s.count(anchor) >= 1, anchor[:90]
    i = s.index(anchor) + len(anchor)
    return s[:i] + add + s[i:]

AXES = [("x", "Origin x", "Ursprung x", "m", "ChangePlacementX"), ("y", "Origin y", "Ursprung y", "m", "ChangePlacementY"), ("z", "Origin z", "Ursprung z", "m", "ChangePlacementZ"), ("yaw", "Yaw about +Z", "Gierwinkel um +Z", "rad", "ChangePlacementYaw")]
KINDS = [dict(kind="change-placement-" + a[0], rec=a[4], mod="change_placement_" + a[0], dirname=CH + "change-placement-" + a[0], field=a[0], en=a[1], de=a[2], unit=a[3], op="change") for a in AXES]
KINDS += [dict(kind="insert-placement", rec="InsertPlacement", mod="insert_placement", dirname="➕️insert-placement", op="insert"), dict(kind="remove-placement", rec="RemovePlacement", mod="remove_placement", dirname="➖️remove-placement", op="remove")]

HEAD = """#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
"""
TAIL = """
//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
"""

def impls(k, other_mod, other_rec, record, entity, verb, label_en, label_de, target):
    other = "crate::%s::%s" % (other_mod, other_rec)
    return """impl<Op> protocol::MutationKind<En1995Snapshot, Op> for %(rec)s where Op: protocol::Mutation<En1995Snapshot, Diff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<%(other)s> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "%(verb)s", entity: "%(entity)s", kind: "%(kind)s", record: "%(record)s" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(%(len)s, %(lde)s) }
    fn target(&self) -> Vec<String> { vec![%(target)s] }
}

impl<Op> protocol::ColdMutationKind<En1995Snapshot, Op> for %(rec)s where Op: protocol::ColdMutation<En1995Snapshot, ReferenceDiff = semio_s_artifact_norm_en1995_diff::En1995Diff> + From<%(other)s> {
    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1995Snapshot>>::ReferenceDiff> { crate::%(mod)s::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%(mod)s::inverse::inverse(self, base)
    }
}
""" % dict(rec=k["rec"], other=other, verb=verb, entity=entity, kind=k["kind"], record=record, len=label_en, lde=label_de, target=target, mod=k["mod"])

def leaf_rs(k):
    op = k["op"]
    if op == "change":
        body = "//! %s `%s` — changes `%s` of one addressed placement.\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n" % (CH, k["kind"], k["field"]) + HEAD + "pub struct %s { pub placement_id: String, pub new_value: f64 }\n" % k["rec"]
        return body + impls(k, k["mod"], k["rec"], "Changed" + k["rec"][6:], "placement-" + k["field"], "change", '&format!("Change %s of placement {}", self.placement_id)' % k["en"].lower(), '&format!("%s von Platzierung {} ändern", self.placement_id)' % k["de"], "self.placement_id.clone()") + TAIL
    if op == "insert":
        body = "//! ➕️ `insert-placement` — inserts one persisted placement at an index (clamped to the list end).\nuse semio_s_artifact_norm_en1995_model::{En1995Snapshot, Placement};\n" + HEAD + "pub struct InsertPlacement { pub index: usize, pub placement: Placement }\n"
        return body + impls(k, "remove_placement", "RemovePlacement", "InsertedPlacement", "placement", "insert", '&format!("Insert placement {} at #{}", self.placement.id, self.index)', '&format!("Platzierung {} an #{} einfügen", self.placement.id, self.index)', "self.placement.id.clone()") + TAIL
    body = "//! ➖️ `remove-placement` — removes the persisted placement at an index.\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n" + HEAD + "pub struct RemovePlacement { pub index: usize }\n"
    return body + impls(k, "insert_placement", "InsertPlacement", "RemovedPlacement", "placement", "remove", '&format!("Remove placement #{}", self.index)', '&format!("Platzierung #{} entfernen", self.index)', "self.index.to_string()") + TAIL

def diff_rs(k):
    op, rec, mod = k["op"], k["rec"], k["mod"]
    if op == "change":
        return """use crate::%s::%s;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995PlacementDelta, En1995PlacementPatch};
pub fn diff(payload: &%s, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(placement) = base.placements.iter().find(|item| item.id == payload.placement_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown placement id.", vec![payload.placement_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { placements: En1995PlacementDelta::modification(&placement.id, En1995PlacementPatch { %s: Some(payload.new_value), ..Default::default() }), ..Default::default() })
}
""" % (mod, rec, rec, k["field"])
    if op == "insert":
        return """use crate::insert_placement::InsertPlacement;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995PlacementDelta;
pub fn diff(payload: &InsertPlacement, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.placements.iter().any(|existing| existing.id == payload.placement.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Placement id {} already exists.", payload.placement.id), [payload.placement.id.clone()]);
    }
    let at = payload.index.min(base.placements.len());
    protocol::MutationOutcome::new(En1995Diff { placements: En1995PlacementDelta::insertion(at, payload.placement.clone()), ..Default::default() })
}
"""
    return """use crate::remove_placement::RemovePlacement;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995PlacementDelta;
pub fn diff(payload: &RemovePlacement, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.placements.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Placement index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { placements: En1995PlacementDelta::removal(&base.placements, payload.index), ..Default::default() })
}
"""

def inverse_rs(k):
    op, rec, mod = k["op"], k["rec"], k["mod"]
    sig = "pub fn inverse<Op>(payload: &%s, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%%s::%%s> {\n" % rec
    if op == "change":
        return "use crate::%s::%s;\n\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n%s    Ok(match base.placements.iter().find(|item| item.id == payload.placement_id) {\n        Some(item) => vec![Op::from(%s { placement_id: payload.placement_id.clone(), new_value: item.%s })],\n        None => Vec::new(),\n    })\n}\n" % (mod, rec, sig % (mod, rec), rec, k["field"])
    if op == "insert":
        return "use crate::insert_placement::InsertPlacement;\nuse crate::remove_placement;\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n%s    let at = payload.index.min(base.placements.len());\n    Ok(vec![Op::from(remove_placement::RemovePlacement { index: at })])\n}\n" % (sig % ("remove_placement", "RemovePlacement"))
    return "use crate::remove_placement::RemovePlacement;\nuse crate::insert_placement;\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n%s    Ok(match base.placements.get(payload.index) {\n        Some(item) => vec![Op::from(insert_placement::InsertPlacement { index: payload.index, placement: item.clone() })],\n        None => Vec::new(),\n    })\n}\n" % (sig % ("insert_placement", "InsertPlacement"))

def catalog_json(k, tag):
    return {"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + k["dirname"], "semanticKind": k["kind"], "displayName": " ".join(w.capitalize() for w in k["kind"].split("-")), "emoji": k["dirname"].split(k["kind"])[0], "aggregateVariant": k["rec"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}

def ui(label_en, label_de, group, order, **extra):
    d = {"widget": "stepper", "role": "value", "label": {"en": label_en, "de": label_de}}
    d.update(extra); d.update({"group": group, "order": order})
    return d

def schema_json(k):
    op = k["op"]; base = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/%s/schema.json" % k["kind"], "title": k["rec"], "type": "object", "additionalProperties": False}
    if op == "change":
        base["required"] = ["placementId", "newValue"]
        base["properties"] = {"placementId": {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": "Placement", "de": "Platzierung"}, "description": {"en": "The persisted placement this mutation addresses.", "de": "Die persistierte Platzierung, die diese Mutation adressiert."}, "ref": {"kind": "placement"}, "group": "target", "order": 10}}, "newValue": {"type": "number", "x-semio-ui": ui(k["en"], k["de"], "pose", 20, unit=k["unit"], step=0.01, precision=3)}}
    elif op == "insert":
        base["required"] = ["index", "placement"]
        base["properties"] = {"index": {"type": "integer", "minimum": 0, "x-semio-ui": ui("Position", "Position", "target", 10, description={"en": "Zero-based list position at which the record is inserted.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird."}, step=1, precision=0)}, "placement": {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/snapshot.json#/$defs/Placement", "x-semio-ui": {"role": "value", "label": {"en": "Placement", "de": "Platzierung"}, "description": {"en": "The complete placement record to insert.", "de": "Der vollständige einzufügende Platzierungsdatensatz."}, "group": "record", "order": 20}}}
    else:
        base["required"] = ["index"]
        base["properties"] = {"index": {"type": "integer", "minimum": 0, "x-semio-ui": ui("Placement", "Platzierung", "target", 10, description={"en": "Zero-based position of the placement in its list.", "de": "Nullbasierte Position der Platzierung in der Liste."}, step=1, precision=0)}}
    return base

def schema_ts(k):
    op, rec = k["op"], k["rec"]
    head = "/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.\n * @see ./🔣️.json */\n" % (k["dirname"].split(k["kind"])[0], k["kind"], rec)
    if op == "change":
        return head + 'import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";\n\nexport interface %s {\n  placementId: string;\n  newValue: number;\n}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ placementId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });\n' % (rec, rec, rec, rec)
    if op == "insert":
        return head + 'import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";\nimport { parsePlacement, type Placement } from "../../../📸️snapshot/🟦️.ts";\n\nexport interface InsertPlacement {\n  index: number;\n  placement: Placement;\n}\n\nexport const parseInsertPlacement: NormWireReader<InsertPlacement> = normWireObject<InsertPlacement>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), placement: normWireRequired(parsePlacement) });\n'
    return head + 'import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";\n\nexport interface RemovePlacement {\n  index: number;\n}\n\nexport const parseRemovePlacement: NormWireReader<RemovePlacement> = normWireObject<RemovePlacement>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });\n'

PLACEMENT = lambda i, v=0.0: {"id": i, "x": v, "y": v, "z": v, "yaw": v}

def blank(d):
    return {key: ({"removed": [], "inserted": [], "moved": [], "modified": []} if isinstance(v, dict) else None) for key, v in d.items()}

def fixture(k):
    base = FIX + "/🚶️change-member-bridge-crowd/✅apply"
    snap = json.loads(rd(base + "/🔣️.before.json"))
    pid = snap["members"][0]["id"]
    snap["placements"] = [PLACEMENT(pid, 1.0)]
    aft = copy.deepcopy(snap)
    diff = blank(json.loads(rd(base + "/🔣️.diff.json")))
    op = k["op"]
    if op == "change":
        nv = 2.5
        aft["placements"][0][k["field"]] = nv
        mut = {k["rec"]: {"placementId": pid, "newValue": nv}}
        diff["placements"]["modified"] = [{"id": pid, "patch": {f: (nv if f == k["field"] else None) for f in ("x", "y", "z", "yaw")}}]
    elif op == "insert":
        row = PLACEMENT("beam-B9", 0.5)
        aft["placements"].append(row)
        mut = {k["rec"]: {"index": 1, "placement": row}}
        diff["placements"]["inserted"] = [{"index": 1, "row": row}]
    else:
        aft["placements"] = []
        mut = {k["rec"]: {"index": 0}}
        diff["placements"]["removed"] = [{"id": pid, "index": 0}]
    return snap, mut, aft, diff

SAMPLES = {
    "change-placement-x": "ChangePlacementX { placement_id: \"%s\".into(), new_value: 0.5 }",
    "change-placement-y": "ChangePlacementY { placement_id: \"%s\".into(), new_value: 0.5 }",
    "change-placement-z": "ChangePlacementZ { placement_id: \"%s\".into(), new_value: 0.5 }",
    "change-placement-yaw": "ChangePlacementYaw { placement_id: \"%s\".into(), new_value: 0.5 }",
}

def sample(k, qualified):
    q = (IDENT + "::" + k["mod"] + "::") if qualified else (k["mod"] + "::")
    if k["op"] == "change":
        return "        En1995Mutation::%s(%s%s { placement_id: \"beam-B1\".into(), new_value: 0.5 })," % (k["rec"], q, k["rec"])
    if k["op"] == "insert":
        return "        En1995Mutation::InsertPlacement(%sInsertPlacement { index: 99, placement: semio_s_artifact_norm_en1995_model::Placement { id: \"beam-B1\".into(), x: 0.0, y: 0.0, z: 0.0, yaw: 0.0 } })," % q
    return "        En1995Mutation::RemovePlacement(%sRemovePlacement { index: 0 })," % q

def main():
    root_p = MUT + "/🦀️.rs"
    root = rd(root_p)
    todo = [k for k in KINDS if '"%s",' % k["kind"] not in root]
    print("to generate:", len(todo))
    if not todo: return
    proto_p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    proto = rd(proto_p)
    tag = max(int(x) for x in re.findall(r"tag=(\d+)", proto)) + 1
    for k in todo:
        k["tag"] = tag; tag += 1
        d = MUT + "/" + k["dirname"]
        wr(d + "/🦀️.rs", leaf_rs(k)); wr(d + "/🔺️diff/🦀️.rs", diff_rs(k)); wr(d + "/↩️inverse/🦀️.rs", inverse_rs(k))
        wr(d + "/🧬️schema/🔣️.json", dumps(schema_json(k))); wr(d + "/🧬️schema/🟦️.ts", schema_ts(k)); wr(d + "/🔣️.json", dumps(catalog_json(k, k["tag"])))
        snap, mut, aft, diff = fixture(k)
        fx = FIX + "/" + k["dirname"] + "/✅apply"
        wr(fx + "/🔣️.before.json", dumps(snap)); wr(fx + "/🔣️.mutation.json", dumps(mut)); wr(fx + "/🔣️.after.json", dumps(aft)); wr(fx + "/🔣️.outcome.json", dumps({"status": "applied"})); wr(fx + "/🔣️.diff.json", dumps(diff))
    tomlp = MUT + "/🌉️footbridges/📦️packages/🦀️rust/Cargo.toml"
    wr(MUT + "/" + CDIR + "/📦️packages/🦀️rust/Cargo.toml", rd(tomlp).replace("mut-footbridges", "mut-placements").replace("footbridges mutation leaves", "placements mutation leaves"))
    mounts = "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (k["dirname"], k["mod"]) for k in KINDS)
    fb = rd(MUT + "/🌉️footbridges/🦀️.rs")
    pre = fb[fb.index("#![allow"): fb.index("//#region 🪢️Mounts")]
    wr(MUT + "/" + CDIR + "/🦀️.rs", "//! 📍️ EN 1995 placements mutation leaves.\n\n" + pre + "//#region 🪢️Mounts\n" + mounts + "//#endregion 🪢️Mounts\n")
    rel = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + CDIR + "/📦️packages/🦀️rust"
    cp = ART + "/Cargo.toml"; c = rd(cp)
    if rel not in c:
        fbm = "    \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️footbridges/📦️packages/🦀️rust\",\n"
        c = rep(c, fbm, fbm + "    \"%s\",\n" % rel)
        fbw = "[workspace.dependencies.semio-s-artifact-norm-en1995-mut-footbridges]\npath = \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️footbridges/📦️packages/🦀️rust\"\n\n"
        c = rep(c, fbw, fbw + "[workspace.dependencies.%s]\npath = \"%s\"\n\n" % (CRATE, rel))
        wr(cp, c)
    for dep in (MUT + "/📦️packages/🦀️rust/Cargo.toml", ANY + "/🚪️io/📦️packages/🦀️rust/Cargo.toml", ANY + "/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml"):
        t = rd(dep); line = "semio-s-artifact-norm-en1995-mut-footbridges = { workspace = true }\n"
        if CRATE not in t: wr(dep, rep(t, line, line + CRATE + " = { workspace = true }\n"))
    sp = ART + "/📦️packages/🦀️rust/📜️script.ts"; sc = rd(sp)
    if '"%s"' % CRATE not in sc: wr(sp, rep(sc, '"semio-s-artifact-norm-en1995-mut-footbridges"', '"semio-s-artifact-norm-en1995-mut-footbridges","%s"' % CRATE))
    op = "/home/deck/semio/📋️build-owners.json"; o = rd(op)
    if '"name": "%s"' % CRATE not in o:
        m = re.search(r'    \{\n      "name": "semio-s-artifact-norm-en1995-mut-footbridges",\n.*?\n    \},\n', o, re.S)
        wr(op, o.replace(m.group(0), m.group(0) + m.group(0).replace("mut-footbridges", "mut-placements").replace("🌉️footbridges", CDIR), 1))
    lp = ART + "/Cargo.lock"; l = rd(lp)
    if 'name = "%s"' % CRATE not in l:
        m = re.search(r'\[\[package\]\]\nname = "semio-s-artifact-norm-en1995-mut-footbridges"\n.*?\n\n', l, re.S)
        l = l.replace(m.group(0), m.group(0) + m.group(0).replace("mut-footbridges", "mut-placements"), 1)
        for pkg in ("io", "io-text", "mutations"):
            am = re.search(r'name = "semio-s-artifact-norm-en1995-%s"\nversion = "0.1.0"\ndependencies = \[\n(.*?)\]' % pkg, l, re.S)
            deps = sorted(set(am.group(1).splitlines()) | {' "%s",' % CRATE})
            l = l.replace(am.group(0), am.group(0).replace(am.group(1), "\n".join(deps) + "\n"), 1)
        wr(lp, l)
    s = root
    uses = "".join("use %s::%s;\n" % (IDENT, k["mod"]) for k in todo)
    s = after(s, "use semio_s_artifact_norm_en1995_mut_design_cases::set_case_text;\n", uses)
    s = after(s, "    SetCaseText(set_case_text::SetCaseText),\n", "".join("    %s(%s::%s),\n" % (k["rec"], k["mod"], k["rec"]) for k in todo))
    s = after(s, '    "change-case-text",\n', "".join('    "%s",\n' % k["kind"] for k in todo))
    wr(root_p, s)
    p = MUT + "/🟦️.ts"; s = rd(p)
    s = after(s, 'import { type SetCaseText, parseSetCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";\n', "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (k["rec"], k["rec"], k["dirname"]) for k in todo))
    last = "  | { ChangeConnectionActionFK: ChangeConnectionActionFK };\n"
    s = rep(s, last, "  | { ChangeConnectionActionFK: ChangeConnectionActionFK }\n" + "".join("  | { %s: %s }\n" % (k["rec"], k["rec"]) for k in todo)[:-1] + ";\n")
    s = after(s, "  SetCaseText: parseSetCaseText,\n", "".join("  %s: parse%s,\n" % (k["rec"], k["rec"]) for k in todo))
    wr(p, s)
    p = MUT + "/🔣️.json"; j = json.loads(rd(p))
    for k in todo: j["oneOf"].append({"type": "object", "additionalProperties": False, "required": [k["rec"]], "properties": {k["rec"]: {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/%s/schema.json" % k["kind"]}}})
    wr(p, dumps(j))
    p = MUT + "/🔗️.graphql"; s = rd(p)
    s = after(s, "  | SetCaseText\n", "".join("  | %s\n" % k["rec"] for k in todo))
    gq = {"change": "  placementId: String! @state(class: ARTIFACT)\n  newValue: Float! @state(class: ARTIFACT)\n", "insert": "  index: Int! @state(class: ARTIFACT)\n  placement: JSON! @state(class: ARTIFACT)\n", "remove": "  index: Int! @state(class: ARTIFACT)\n"}
    wr(p, s.rstrip("\n") + "\n\n" + "\n".join("type %s {\n%s}\n" % (k["rec"], gq[k["op"]]) for k in todo))
    p = MUT + "/🛰️.proto"; s = rd(p)
    pr = {"change": "  // @state artifact\n  string placement_id = 1;\n  // @state artifact\n  double new_value = 2;\n", "insert": "  // @state artifact\n  uint32 index = 1;\n  // @state artifact\n  bytes placement = 2;\n", "remove": "  // @state artifact\n  uint32 index = 1;\n"}
    n = int(re.search(r"set_case_text = (\d+);", s).group(1))
    msgs = "".join("message %s {\n%s}\n\n" % (k["rec"], pr[k["op"]]) for k in todo)
    s = rep(s, "message En1995Mutation {", msgs + "message En1995Mutation {")
    s = after(s, "    SetCaseText set_case_text = %d;\n" % n, "".join("    %s %s = %d;\n" % (k["rec"], k["mod"], n + 1 + i) for i, k in enumerate(todo)))
    wr(p, s)
    wr(proto_p, proto.rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (k["kind"], k["tag"]) for k in todo))
    p = MUT + "/🧭️edit-rules/🦀️.rs"; s = rd(p)
    sets = "".join('        SetFieldRule { path: "placements[].%s", kind: "change-placement-%s", selectors: &[SelectorKey::Id("placementId")], value: "newValue" },\n' % (a[0], a[0]) for a in AXES if any(k["kind"] == "change-placement-" + a[0] for k in todo))
    if sets: s = rep(s, "    ],\n    insert_item: &[\n", sets + "    ],\n    insert_item: &[\n")
    if any(k["kind"] == "insert-placement" for k in todo): s = rep(s, "    ],\n    remove_item: &[\n", '        InsertItemRule { path: "placements", kind: "insert-placement", selectors: &[], index: "index", item: "placement" },\n    ],\n    remove_item: &[\n')
    if any(k["kind"] == "remove-placement" for k in todo):
        i = s.rindex("    ],\n};")
        s = s[:i] + '        RemoveItemRule { path: "placements", kind: "remove-placement", selectors: &[], index: "index" },\n' + s[i:]
    wr(p, s)
    for path, anchor in ((MUT + "/🧪️tests/🔬️unit/🦀️.rs", False), (ANY + "/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", True), (ANY + "/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️op-round-trip/🦀️.rs", True)):
        s = rd(path)
        lines = s.split("\n")
        i = max(n for n, ln in enumerate(lines) if ln.lstrip().startswith("En1995Mutation::SetCaseText("))
        lines[i + 1:i + 1] = [sample(k, True) for k in todo]
        wr(path, "\n".join(lines))
    H = ANY + "/🧪️tests/🪵️mutate-en1995-1"
    p = H + "/🦀️.rs"; s = rd(p)
    s = after(s, '    "change-case-text",\n', "".join('    "%s",\n' % k["kind"] for k in todo))
    arms = "".join('    ("%s", (\n%s    )),\n' % (k["kind"], "".join('        include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (k["dirname"], q) for q in ["before", "mutation", "after", "outcome"])) for k in todo)
    m = re.search(r'    \("change-case-text", \(\n(?:        include_str!.*\n){4}    \)\),\n', s)
    s = s.replace(m.group(0), m.group(0) + arms, 1)
    wr(p, s)
    p = H + "/🐍️.py"; s = rd(p)
    s = after(s, '    "change-case-text",\n', "".join('    "%s",\n' % k["kind"] for k in todo))
    s = after(s, '    "change-case-text": ("🔤️change-case-text", "✅apply"),\n', "".join('    "%s": ("%s", "✅apply"),\n' % (k["kind"], k["dirname"]) for k in todo))
    wr(p, s)
    p = H + "/🥒️.feature"; s = rd(p)
    out = []
    for ln in s.split("\n"):
        out.append(ln)
        m = re.match(r"^(\s+\| )change-case-text( +\| )🔤️change-case-text( +\| )✅apply( +\|)$", ln)
        if m:
            for k in todo: out.append("%s%s%s%s%s✅apply%s" % (m.group(1), k["kind"], m.group(2), k["dirname"], m.group(3), m.group(4)))
    wr(p, "\n".join(out))
    p = ANY + "/🔮️oracles/🔣️.json"; oj = json.loads(rd(p))
    cat = oj["mutationCatalogs"][0]; man = oj["mutationManifests"][0]
    tpl = {"change": "change-member-bridge-crowd", "insert": "insert-member", "remove": "remove-member"}
    for k in todo:
        t = tpl[k["op"]]
        v = copy.deepcopy([x for x in cat["vectors"] if x["mutationId"] == t][0])
        v["mutationId"] = k["kind"]; v["sourceMutationDirectoryName"] = k["dirname"]; v["mutationDirectoryName"] = k["dirname"]; v["scenarios"][0]["id"] = k["kind"]
        cat["vectors"].append(v); cat["kinds"].append(k["kind"])
        m = copy.deepcopy([x for x in man["mutations"] if x["id"] == t][0])
        m["id"] = k["kind"]; m["productionDispatch"]["operation"] = k["kind"]; m["productionDispatch"]["variant"] = k["rec"]
        man["mutations"].append(m)
    wr(p, dumps(oj))
    tp = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"; tj = json.loads(rd(tp))
    last = max(i for i, x in enumerate(tj["rows"]) if x["artifact"] == "🪵️en1995")
    rows = []
    for k in todo:
        tr = copy.deepcopy([x for x in tj["rows"] if x["artifact"] == "🪵️en1995" and x["kind"] == tpl[k["op"]]][0])
        rec = {"change": "Changed" + k["rec"][6:], "insert": "InsertedPlacement", "remove": "RemovedPlacement"}[k["op"]]
        tr.update(aggregateVariant=k["rec"], entity=("placement-" + k["field"]) if k["op"] == "change" else "placement", kind=k["kind"], module=k["dirname"], record=rec, source="🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/🦀️.rs" % k["dirname"], type=k["rec"])
        rows.append(tr)
    tj["rows"][last + 1:last + 1] = rows
    wr(tp, dumps(tj))
    print("generated", len(todo))

if __name__ == "__main__":
    main()
