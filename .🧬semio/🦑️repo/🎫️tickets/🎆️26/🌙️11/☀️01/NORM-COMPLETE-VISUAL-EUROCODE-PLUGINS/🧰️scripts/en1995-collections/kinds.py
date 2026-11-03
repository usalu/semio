#!/usr/bin/env python3
"""Generates the EN 1995 mutation kinds of the typed collections of `spec.py`: `insert-<slug>`, `remove-<slug>` and one `change-<slug>-<field>` per field, grouped into crates of at most nine leaves (32-file crate cap), registered in every aggregate (root enum, KINDS, ts/json/graphql/proto twins, edit rules, binary protocol, unit and text tests, harness rs/py/feature, oracle manifest, taxonomy, workspace, build owners, lock) with fixtures. Idempotent, path-guarded.
Usage: python3 -I kinds.py [--dry] [--root SUBSET_DIR] [--owners FILE] [--taxonomy FILE] [--workspace ARTIFACT_DIR]"""
import copy, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import spec as S
from spec import ENTITIES, CHILDREN, UNIT_UI, TS_TY, RS_TY

DRY = "--dry" in sys.argv
def opt(name, default):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default
ANY = opt("--root", S.ANY)
if not ANY.endswith("/"):
    ANY += "/"
ART = opt("--workspace", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995")
OWNERS = opt("--owners", "/home/deck/semio/📋️build-owners.json")
TAXONOMY = opt("--taxonomy", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json")
MUT = ANY + "🧬️schema/🧬️mutations"
FIX = ANY + "🧪️tests/🧫️fixtures/🧬️mutations"
HARNESS = ANY + "🧪️tests/🪵️mutate-en1995-1"
REL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1995/snapshot.json#/$defs/"
SHORT = {"spacing_perpendicular_m": "spacing-perp", "spacing_parallel_m": "spacing-par", "ei_l_n_m2_per_m": "ei-l", "ei_b_n_m2_per_m": "ei-b", "lamella_thickness_m": "lamella", "rod_resistance_n": "rod-resistance", "permanent_load_pa": "permanent-load", "imposed_load_pa": "imposed-load", "taper_angle_rad": "taper-angle", "apex_angle_rad": "apex-angle", "centre_height_m": "centre-height", "mass_kg_per_m2": "mass", "damping_ratio": "damping", "load_angle_rad": "load-angle", "fire_duration_s": "fire-duration", "joist_spacing_m": "joist-spacing", "apex_depth_m": "apex-depth", "inner_radius_m": "inner-radius", "connection_id": "connection", "member_id": "member", "strength_class": "strength-class", "tension_side": "side", "offset_x_m": "offset-x", "offset_y_m": "offset-y", "end_distance_m": "end-distance", "edge_distance_m": "edge-distance", "diaphragm_force_n": "force", "diaphragm_length_m": "length", "shear_resistance_n_per_m": "shear-resistance", "bracing_capacity_n_per_m": "bracing-capacity", "stiffness_n_per_m": "stiffness", "compression_force_n": "compression", "compression_perpendicular_pa": "compression-c90"}
GROUPS = {("fastener", 0): ("fastener-links", ["connection_id", "member_id", "columns", "lines"]), ("floor", 0): ("floor-stiffness", ["member_id", "span_m", "width_m", "joist_spacing_m", "ei_l_n_m2_per_m", "ei_b_n_m2_per_m"])}


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    q = (S.ANY + p[len(ANY):]) if p.startswith(ANY) else p
    assert len(q[len("/home/deck/semio/"):].encode()) < 256, "path of 256 bytes or more: " + q
    if DRY:
        return
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def dumps(o):
    return json.dumps(o, indent=2, ensure_ascii=False) + "\n"


def rep(s, a, b):
    assert s.count(a) == 1, (s.count(a), a[:90])
    return s.replace(a, b, 1)


def after(s, anchor, add):
    assert s.count(anchor) >= 1, anchor[:90]
    i = s.rindex(anchor) + len(anchor)
    return s[:i] + add + s[i:]


def oneof_dumps(d):
    c = dict(separators=(",", ":"), ensure_ascii=False)
    rest = {k: v for k, v in d.items() if k != "oneOf"}
    head = json.dumps(rest, indent=2, ensure_ascii=False)[:-2]
    body = ",\n".join("    " + json.dumps(x, **c) for x in d["oneOf"])
    return head + ',\n  "oneOf": [\n' + body + "\n  ]\n}\n"


def rep_once(s, a, b):
    return rep(s, a, b)


def pascal(k):
    return "".join(x[:1].upper() + x[1:] for x in k.split("-"))


def kebab_field(e, fl):
    return SHORT.get(fl["name"]) or re.sub(r"_(m|rad|n|pa|s)$", "", fl["name"]).replace("_", "-")


def nested_leaves():
    out = []
    for c in CHILDREN:
        parent = next(e for e in ENTITIES if e["plural"] == c["parent"])
        base = dict(e=c, parent=parent, nest=True, group=c["slug"] + "s")
        out.append(dict(base, op="insert", kind="insert-" + c["slug"], dirname="➕️insert-" + c["slug"]))
        out.append(dict(base, op="remove", kind="remove-" + c["slug"], dirname="➖️remove-" + c["slug"]))
        for fl in c["rest"]:
            kind = "change-%s-%s" % (c["slug"], kebab_field(c, fl))
            assert len(kind) <= 40, (kind, len(kind))
            out.append(dict(base, op="change", fl=fl, kind=kind, dirname=c["emoji"] + kind))
    return out


def leaves():
    out = []
    for e in ENTITIES:
        slug = e["slug"]
        rows = e.get("rows_group", "entity-rows")
        out.append(dict(e=e, op="insert", kind="insert-" + slug, group=rows, dirname="➕️insert-" + slug))
        out.append(dict(e=e, op="remove", kind="remove-" + slug, group=rows, dirname="➖️remove-" + slug))
        names = [fl["name"] for fl in e["rest"]]
        link = GROUPS.get((slug, 0))
        if e.get("groups"):
            plan = e["groups"]
        elif link:
            first, second = link[1], [n for n in names if n not in link[1]]
            plan = [(link[0], first), ({"fastener": "fastener-distances", "floor": "floor-loads"}[slug], second)]
        else:
            plan = [(e["plural"], names)]
        for gname, fields in plan:
            for n in fields:
                fl = next(x for x in e["rest"] if x["name"] == n)
                kind = "change-%s-%s" % (slug, kebab_field(e, fl))
                assert len(kind) <= 35, (kind, len(kind))
                out.append(dict(e=e, op="change", fl=fl, kind=kind, group=gname, dirname=e["emoji"] + kind))
    out += nested_leaves()
    for k in out:
        k["rec"] = pascal(k["kind"])
        k["mod"] = k["kind"].replace("-", "_")
    return out


LEAVES = leaves()
GROUP_NAMES = []
for k in LEAVES:
    if k["group"] not in GROUP_NAMES:
        GROUP_NAMES.append(k["group"])
GROUP_DIR = {"entity-rows": "🧱️entity-rows", "tapers": "📐️tapers", "cutouts": "🕳️cutouts", "fastener-links": "🔗️fastener-links", "fastener-distances": "↔️fastener-distances", "panels": "🧱️panels", "floor-stiffness": "🏠️floor-stiffness", "floor-loads": "⚖️floor-loads"}
GROUP_DIR["panels"] = "🪵️panels"
GROUP_DIR.update({"panel-layers": "🧱️panel-layers", "fastener-positions": "📍️fastener-positions", "structure-rows": "🏗️structure-rows", "diaphragm-sheathing": "🛡️diaphragm-sheathing", "diaphragm-forces": "💪️diaphragm-forces", "truss-geometry": "🔺️truss-geometry", "truss-forces": "🔻️truss-forces", "bracing-layout": "✖️bracing-layout", "bracing-forces": "⛓️bracing-forces", "deck-geometry": "🌉️deck-geometry", "deck-prestress": "🗜️deck-prestress"})
for g in GROUP_NAMES:
    assert len(g.encode()) < 60 and g in GROUP_DIR, g
    assert sum(1 for k in LEAVES if k["group"] == g) <= 10, g
CRATE = {g: "semio-s-artifact-norm-en1995-mut-" + g for g in GROUP_NAMES}
IDENT = {g: CRATE[g].replace("-", "_") for g in GROUP_NAMES}
for k in LEAVES:
    k["crate"], k["ident"] = CRATE[k["group"]], IDENT[k["group"]]

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


def impls(k, other_mod, other_rec, record, entity, verb, label_en, label_de, target, other_crate):
    other = ("crate::%s::%s" % (other_mod, other_rec)) if other_crate == k["crate"] else ("%s::%s::%s" % (other_crate.replace("-", "_"), other_mod, other_rec))
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


def partner(k):
    e = k["e"]
    if k.get("nest"):
        if k["op"] == "change":
            return k["mod"], k["rec"], k["crate"]
        return ("remove_" if k["op"] == "insert" else "insert_") + e["slug"].replace("-", "_"), ("Remove" if k["op"] == "insert" else "Insert") + e["ent"], k["crate"]
    if k["op"] == "insert":
        return "remove_" + e["slug"], "Remove" + e["ent"], k["crate"]
    if k["op"] == "remove":
        return "insert_" + e["slug"], "Insert" + e["ent"], k["crate"]
    return k["mod"], k["rec"], k["crate"]


def leaf_rs(k):
    if k.get("nest"):
        return nested_leaf_rs(k)
    e, op = k["e"], k["op"]
    pm, pr, pc = partner(k)
    if op == "change":
        fl = k["fl"]
        body = "//! %s `%s` — changes `%s` of one addressed %s.\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n" % (e["emoji"], k["kind"], fl["name"], e["slug"]) + HEAD + "pub struct %s { pub %s_id: String, pub new_value: %s }\n" % (k["rec"], e["slug"], RS_TY[fl["ty"]])
        return body + impls(k, pm, pr, "Changed" + k["rec"][6:], e["slug"] + "-" + kebab_field(e, fl), "change", '&format!("Change %s of %s {}", self.%s_id)' % (fl["en"].lower().replace('"', ""), e["slug"], e["slug"]), '&format!("%s von %s {} ändern", self.%s_id)' % (fl["de"].replace('"', ""), e["ent"], e["slug"]), "self.%s_id.clone()" % e["slug"], pc) + TAIL
    if op == "insert":
        body = "//! ➕️ `%s` — inserts one %s at an index (clamped to the list end).\nuse semio_s_artifact_norm_en1995_model::{En1995Snapshot, %s};\n" % (k["kind"], e["slug"], e["ent"]) + HEAD + "pub struct %s { pub index: usize, pub %s: %s }\n" % (k["rec"], e["slug"], e["ent"])
        return body + impls(k, pm, pr, "Inserted" + e["ent"], e["slug"], "insert", '&format!("Insert %s {} at #{}", self.%s.id, self.index)' % (e["slug"], e["slug"]), '&format!("%s {} an #{} einfügen", self.%s.id, self.index)' % (e["ent"], e["slug"]), "self.%s.id.clone()" % e["slug"], pc) + TAIL
    body = "//! ➖️ `%s` — removes the %s at an index.\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n" % (k["kind"], e["slug"]) + HEAD + "pub struct %s { pub index: usize }\n" % k["rec"]
    return body + impls(k, pm, pr, "Removed" + e["ent"], e["slug"], "remove", '&format!("Remove %s #{}", self.index)' % e["slug"], '&format!("%s #{} entfernen", self.index)' % e["ent"], "self.index.to_string()", pc) + TAIL


def diff_rs(k):
    if k.get("nest"):
        return nested_diff_rs(k)
    e, op, rec, mod = k["e"], k["op"], k["rec"], k["mod"]
    P, pl, sl = "En1995" + e["ent"], e["plural"], e["slug"]
    if op == "change":
        fl = k["fl"]
        checks = ""
        if fl["ty"] == "f64":
            checks = '    if !payload.new_value.is_finite() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());\n    }\n'
        if fl["options"]:
            opts = ", ".join('"%s"' % o[0] for o in fl["options"])
            checks = '    if ![%s].contains(&payload.new_value.as_str()) {\n        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown value {}.", payload.new_value), vec![payload.new_value.clone()]);\n    }\n' % opts
        val = "payload.new_value.clone()" if fl["ty"] == "String" else "payload.new_value"
        return """use crate::%s::%s;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{%sDelta, %sPatch};
pub fn diff(payload: &%s, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.%s.iter().find(|item| item.id == payload.%s_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown %s id.", vec![payload.%s_id.clone()]);
    };
%s    protocol::MutationOutcome::new(En1995Diff { %s: %sDelta::modification(&row.id, %sPatch { %s: Some(%s), ..Default::default() }), ..Default::default() })
}
""" % (mod, rec, P, P, rec, pl, sl, sl, sl, checks, pl, P, P, fl["name"], val)
    if op == "insert":
        return """use crate::%s::%s;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::%sDelta;
pub fn diff(payload: &%s, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.%s.iter().any(|existing| existing.id == payload.%s.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%s id {} already exists.", payload.%s.id), [payload.%s.id.clone()]);
    }
    let at = payload.index.min(base.%s.len());
    protocol::MutationOutcome::new(En1995Diff { %s: %sDelta::insertion(at, payload.%s.clone()), ..Default::default() })
}
""" % (mod, rec, P, rec, pl, sl, e["ent"], sl, sl, pl, pl, P, sl)
    return """use crate::%s::%s;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::%sDelta;
pub fn diff(payload: &%s, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.%s.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("%s index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { %s: %sDelta::removal(&base.%s, payload.index), ..Default::default() })
}
""" % (mod, rec, P, rec, pl, e["ent"], pl, P, pl)


def inverse_rs(k):
    if k.get("nest"):
        return nested_inverse_rs(k)
    e, op, rec, mod = k["e"], k["op"], k["rec"], k["mod"]
    pl, sl = e["plural"], e["slug"]
    pm, pr, pc = partner(k)
    sig = "pub fn inverse<Op>(payload: &%s, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<%s> {\n"
    if op == "change":
        fl = k["fl"]
        v = "item.%s.clone()" % fl["name"] if fl["ty"] == "String" else "item.%s" % fl["name"]
        return "use crate::%s::%s;\n\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n%s    Ok(match base.%s.iter().find(|item| item.id == payload.%s_id) {\n        Some(item) => vec![Op::from(%s { %s_id: payload.%s_id.clone(), new_value: %s })],\n        None => Vec::new(),\n    })\n}\n" % (mod, rec, sig % (rec, "crate::%s::%s" % (mod, rec)), pl, sl, rec, sl, sl, v)
    if op == "insert":
        return "use crate::insert_%s::Insert%s;\nuse %s::remove_%s;\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n%s    let at = payload.index.min(base.%s.len());\n    Ok(vec![Op::from(remove_%s::Remove%s { index: at })])\n}\n" % (sl, e["ent"], "crate", sl, sig % ("Insert" + e["ent"], "crate::remove_%s::Remove%s" % (sl, e["ent"])), pl, sl, e["ent"])
    return "use crate::remove_%s::Remove%s;\nuse crate::insert_%s;\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n%s    Ok(match base.%s.get(payload.index) {\n        Some(item) => vec![Op::from(insert_%s::Insert%s { index: payload.index, %s: item.clone() })],\n        None => Vec::new(),\n    })\n}\n" % (sl, e["ent"], sl, sig % ("Remove" + e["ent"], "crate::insert_%s::Insert%s" % (sl, e["ent"])), pl, sl, e["ent"], sl)


def catalog_json(k):
    return {"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + k["dirname"], "semanticKind": k["kind"], "displayName": " ".join(w.capitalize() for w in k["kind"].split("-")), "emoji": k["dirname"].split(k["kind"])[0], "aggregateVariant": k["rec"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": k["tag"], "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}


def ui(label_en, label_de, group, order, **extra):
    d = {"widget": "stepper", "role": "value", "label": {"en": label_en, "de": label_de}}
    d.update(extra)
    d.update({"group": group, "order": order})
    return d


def schema_json(k):
    if k.get("nest"):
        return nested_schema_json(k)
    e, op = k["e"], k["op"]
    base = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/%s/schema.json" % k["kind"], "title": k["rec"], "type": "object", "additionalProperties": False}
    sl = e["slug"]
    if op == "change":
        fl = k["fl"]
        base["required"] = [sl + "Id", "newValue"]
        target = {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": e["ent"], "de": e["ent"]}, "description": {"en": "The %s this mutation addresses." % sl, "de": "Das Element, das diese Mutation adressiert."}, "ref": {"kind": e["ref_kind"]}, "group": "target", "order": 10}}
        if fl["ty"] == "String":
            val = {"type": "string", "x-semio-ui": {"widget": "select" if fl["options"] else "text", "role": "value", "label": {"en": fl["en"], "de": fl["de"]}, "group": "fields", "order": 20}}
            if fl["options"]:
                val["enum"] = [o[0] for o in fl["options"]]
                val["x-semio-ui"]["options"] = [o[0] for o in fl["options"]]
        elif fl["ty"] == "u32":
            val = {"type": "integer", "minimum": 0, "maximum": 4294967295, "x-semio-ui": ui(fl["en"], fl["de"], "fields", 20, step=1, precision=0)}
        else:
            u = dict(UNIT_UI[fl["unit"]])
            val = {"type": "number", "x-semio-ui": ui(fl["en"], fl["de"], "fields", 20, **u)}
        base["properties"] = {sl + "Id": target, "newValue": val}
    elif op == "insert":
        base["required"] = ["index", sl]
        base["properties"] = {"index": {"type": "integer", "minimum": 0, "x-semio-ui": ui("Position", "Position", "target", 10, description={"en": "Zero-based list position at which the record is inserted.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird."}, step=1, precision=0)}, sl: {"$ref": SNAP + e["ent"], "x-semio-ui": {"role": "value", "label": {"en": e["ent"], "de": e["ent"]}, "description": {"en": "The complete record to insert.", "de": "Der vollständige einzufügende Datensatz."}, "group": "record", "order": 20}}}
    else:
        base["required"] = ["index"]
        base["properties"] = {"index": {"type": "integer", "minimum": 0, "x-semio-ui": ui(e["ent"], e["ent"], "target", 10, description={"en": "Zero-based position of the record in its list.", "de": "Nullbasierte Position des Datensatzes in der Liste."}, step=1, precision=0)}}
    return base


def schema_ts(k):
    if k.get("nest"):
        return nested_schema_ts(k)
    e, op, rec = k["e"], k["op"], k["rec"]
    sl = e["slug"]
    head = "/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.\n * @see ./🔣️.json */\n" % (k["dirname"].split(k["kind"])[0], k["kind"], rec)
    if op == "change":
        fl = k["fl"]
        w = S.WIRE[fl["ty"]]
        imports = "normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString"
        return head + 'import { %s } from "@semio-tech/norm-js/registry/contract";\n\nexport interface %s {\n  %sId: string;\n  newValue: %s;\n}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ %sId: normWireRequired(normWireString), newValue: normWireRequired(%s) });\n' % (imports, rec, sl, TS_TY[fl["ty"]], rec, rec, rec, sl, w)
    if op == "insert":
        return head + 'import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";\nimport { parse%s, type %s } from "../../../📸️snapshot/🟦️.ts";\n\nexport interface %s {\n  index: number;\n  %s: %s;\n}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), %s: normWireRequired(parse%s) });\n' % (e["ent"], e["ent"], rec, sl, e["ent"], rec, rec, rec, sl, e["ent"])
    return head + 'import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";\n\nexport interface %s {\n  index: number;\n}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });\n' % (rec, rec, rec, rec)


def row_of(e, snap, nth=0):
    row = {"id": "%s-%d" % (e["slug"], nth + 1)}
    for fl in e["rest"]:
        if fl["ref"] == "member":
            row[fl["camel"]] = snap["members"][0]["id"]
        elif fl["ref"] == "connection":
            row[fl["camel"]] = snap["connections"][0]["id"] if snap["connections"] else "conn-C1"
        else:
            row[fl["camel"]] = fl["example"]
    return row


def changed(fl, old):
    if fl["ty"] == "String":
        if fl["options"]:
            vals = [o[0] for o in fl["options"]]
            return vals[(vals.index(old) + 1) % len(vals)] if old in vals else vals[0]
        return "changed-" + str(old)
    if fl["ty"] == "u32":
        return int(old) + 1
    return float(old) * 1.5 + 1.0


def blank(d):
    return {key: ({"removed": [], "inserted": [], "moved": [], "modified": []} if isinstance(v, dict) else None) for key, v in d.items()}


def fixture(k):
    if k.get("nest"):
        return nested_fixture(k)
    base = FIX + "/🚶️change-member-bridge-crowd/✅apply"
    snap = json.loads(rd(base + "/🔣️.before.json"))
    e, op = k["e"], k["op"]
    pl = e["plural"]
    row = row_of(e, snap)
    diff = blank(json.loads(rd(base + "/🔣️.diff.json")))
    diff.setdefault(pl, {"removed": [], "inserted": [], "moved": [], "modified": []})
    if op == "change":
        snap[pl] = [row]
        aft = copy.deepcopy(snap)
        fl = k["fl"]
        nv = changed(fl, row[fl["camel"]])
        aft[pl][0][fl["camel"]] = nv
        mut = {k["rec"]: {e["slug"] + "Id": row["id"], "newValue": nv}}
        diff[pl]["modified"] = [{"id": row["id"], "patch": {x["camel"]: (nv if x is fl else None) for x in e["rest"]}}]
    elif op == "insert":
        snap[pl] = []
        aft = copy.deepcopy(snap)
        aft[pl] = [row]
        mut = {k["rec"]: {"index": 0, e["slug"]: row}}
        diff[pl]["inserted"] = [{"index": 0, "row": row}]
    else:
        snap[pl] = [row]
        aft = copy.deepcopy(snap)
        aft[pl] = []
        mut = {k["rec"]: {"index": 0}}
        diff[pl]["removed"] = [{"id": row["id"], "index": 0}]
    return snap, mut, aft, diff


def sample(k, qualified):
    if k.get("nest"):
        return nested_sample(k)
    e = k["e"]
    q = "%s::%s::" % (k["ident"], k["mod"])
    if k["op"] == "change":
        fl = k["fl"]
        v = {"String": '"%s".into()' % (fl["options"][0][0] if fl["options"] else "x"), "u32": "1", "f64": "0.5"}[fl["ty"]]
        return "        En1995Mutation::%s(%s%s { %s_id: \"%s-1\".into(), new_value: %s })," % (k["rec"], q, k["rec"], e["slug"], e["slug"], v)
    if k["op"] == "insert":
        fields = ", ".join("%s: %s" % (fl["name"], {"String": '"x".into()', "u32": "0", "f64": "0.0"}[fl["ty"]]) for fl in e["rest"])
        return "        En1995Mutation::%s(%s%s { index: 99, %s: semio_s_artifact_norm_en1995_model::%s { id: \"%s-1\".into(), %s } })," % (k["rec"], q, k["rec"], e["slug"], e["ent"], e["slug"], fields)
    return "        En1995Mutation::%s(%s%s { index: 0 })," % (k["rec"], q, k["rec"])


def item_of(c):
    return c["plural"][:-1]


def nested_payload_fields(k):
    c, P = k["e"], k["parent"]
    item = item_of(c)
    if k["op"] == "change":
        return [(P["slug"] + "_id", "String"), (item + "_id", "String"), ("new_value", RS_TY[k["fl"]["ty"]])]
    if k["op"] == "insert":
        return [(P["slug"] + "_id", "String"), ("index", "usize"), (item, c["ent"])]
    return [(P["slug"] + "_id", "String"), ("id", "String")]


DE_NOUN = {"panel-layer": "Lage", "panel": "Platte", "fastener-position": "Lage", "fastener": "Verbindungsmittelbild"}


def nested_leaf_rs(k):
    c, P, op = k["e"], k["parent"], k["op"]
    pm, pr, pc = partner(k)
    item = item_of(c)
    fields = ", ".join("pub %s: %s" % f for f in nested_payload_fields(k))
    uses = "semio_s_artifact_norm_en1995_model::{En1995Snapshot, %s}" % c["ent"] if op == "insert" else "semio_s_artifact_norm_en1995_model::En1995Snapshot"
    pid = "self.%s_id" % P["slug"]
    if op == "change":
        fl = k["fl"]
        doc = "//! %s `%s` — changes `%s` of one addressed %s of one %s.\n" % (c["emoji"], k["kind"], fl["name"], item, P["slug"])
        label = ('&format!("Change %s of %s {} of %s {}", self.%s_id, %s)' % (fl["en"].lower().replace('"', ""), item, P["slug"], item, pid), '&format!("%s von %s {} an %s {} ändern", self.%s_id, %s)' % (fl["de"].replace('"', ""), DE_NOUN[c["slug"]], DE_NOUN[P["slug"]], item, pid))
        target = "%s.clone(), self.%s_id.clone()" % (pid, item)
        entity, verb, record = c["slug"] + "-" + kebab_field(c, fl), "change", "Changed" + k["rec"][6:]
    elif op == "insert":
        doc = "//! ➕️ `%s` — inserts one %s into the %s of one %s at an index (clamped to the list end).\n" % (k["kind"], item, c["plural"], P["slug"])
        label = ('&format!("Insert %s {} into %s {} at #{}", self.%s.id, %s, self.index)' % (item, P["slug"], item, pid), '&format!("%s {} in %s {} an #{} einfügen", self.%s.id, %s, self.index)' % (DE_NOUN[c["slug"]], DE_NOUN[P["slug"]], item, pid))
        target = "%s.clone(), self.%s.id.clone()" % (pid, item)
        entity, verb, record = c["slug"], "insert", "Inserted" + c["ent"]
    else:
        doc = "//! ➖️ `%s` — removes one %s of one %s by its id.\n" % (k["kind"], item, P["slug"])
        label = ('&format!("Remove %s {} from %s {}", self.id, %s)' % (item, P["slug"], pid), '&format!("%s {} aus %s {} entfernen", self.id, %s)' % (DE_NOUN[c["slug"]], DE_NOUN[P["slug"]], pid))
        target = "%s.clone(), self.id.clone()" % pid
        entity, verb, record = c["slug"], "remove", "Removed" + c["ent"]
    body = doc + "use %s;\n" % uses + HEAD + "pub struct %s { %s }\n" % (k["rec"], fields)
    return body + impls(k, pm, pr, record, entity, verb, label[0], label[1], target, pc) + TAIL


def nested_names(k):
    c, P = k["e"], k["parent"]
    return "En1995" + P["ent"], "En1995" + c["ent"], P["plural"], c["plural"], P["slug"], item_of(c)


def nested_diff_rs(k):
    c, P, op, rec, mod = k["e"], k["parent"], k["op"], k["rec"], k["mod"]
    PP, CP, pl, cl, ps, item = nested_names(k)
    head = "use crate::%s::%s;\nuse semio_s_artifact_norm_en1995_diff::En1995Diff;\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\nuse semio_s_artifact_norm_en1995_diff::{%sDelta, %sPatch, %sDelta%s};\n" % (mod, rec, PP, PP, CP, ", %sPatch" % CP if op == "change" else "")
    head += "pub fn diff(payload: &%s, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {\n" % rec
    head += '    let Some(parent) = base.%s.iter().find(|row| row.id == payload.%s_id) else {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown %s id.", vec![payload.%s_id.clone()]);\n    };\n' % (pl, ps, ps, ps)
    wrap = lambda inner: "    protocol::MutationOutcome::new(En1995Diff { %s: %sDelta::modification(&parent.id, %sPatch { %s: %s, ..Default::default() }), ..Default::default() })\n}\n" % (pl, PP, PP, cl, inner)
    if op == "change":
        fl = k["fl"]
        checks = ""
        if fl["ty"] == "f64":
            cond = "!payload.new_value.is_finite() || payload.new_value <= 0.0" if fl["name"] == "thickness_m" else "!payload.new_value.is_finite()"
            msg = "The value must be finite and positive." if fl["name"] == "thickness_m" else "The value must be finite."
            checks = '    if %s {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "%s", Vec::<String>::new());\n    }\n' % (cond, msg)
        if fl["options"]:
            opts = ", ".join('"%s"' % o[0] for o in fl["options"])
            checks = '    if ![%s].contains(&payload.new_value.as_str()) {\n        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown value {}.", payload.new_value), vec![payload.new_value.clone()]);\n    }\n' % opts
        val = "payload.new_value.clone()" if fl["ty"] == "String" else "payload.new_value"
        find = '    let Some(row) = parent.%s.iter().find(|row| row.id == payload.%s_id) else {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown %s id.", vec![payload.%s_id.clone(), payload.%s_id.clone()]);\n    };\n' % (cl, item, item, ps, item)
        return head + find + checks + wrap("%sDelta::modification(&row.id, %sPatch { %s: Some(%s), ..Default::default() })" % (CP, CP, fl["name"], val))
    if op == "insert":
        dup = '    if parent.%s.iter().any(|row| row.id == payload.%s.id) {\n        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%s id {} already exists.", payload.%s.id), [payload.%s.id.clone()]);\n    }\n    let at = payload.index.min(parent.%s.len());\n' % (cl, item, c["ent"], item, item, cl)
        return head + dup + wrap("%sDelta::insertion(at, payload.%s.clone())" % (CP, item))
    find = '    let Some(at) = parent.%s.iter().position(|row| row.id == payload.id) else {\n        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown %s id.", vec![payload.%s_id.clone(), payload.id.clone()]);\n    };\n' % (cl, item, ps)
    return head + find + wrap("%sDelta::removal(&parent.%s, at)" % (CP, cl))


def nested_inverse_rs(k):
    c, P, op, rec, mod = k["e"], k["parent"], k["op"], k["rec"], k["mod"]
    pm, pr, pc = partner(k)
    PP, CP, pl, cl, ps, item = nested_names(k)
    sig = "pub fn inverse<Op>(payload: &%s, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%s::%s> {\n" % (rec, pm if op != "change" else mod, pr if op != "change" else rec)
    uses = "use crate::%s::%s;\nuse crate::%s;\nuse semio_s_artifact_norm_en1995_model::En1995Snapshot;\n" % (mod, rec, pm if op != "change" else mod)
    parent = "    let Some(parent) = base.%s.iter().find(|row| row.id == payload.%s_id) else { return Ok(Vec::new()); };\n" % (pl, ps)
    if op == "change":
        fl = k["fl"]
        v = "row.%s.clone()" % fl["name"] if fl["ty"] == "String" else "row.%s" % fl["name"]
        return uses + sig + parent + "    let Some(row) = parent.%s.iter().find(|row| row.id == payload.%s_id) else { return Ok(Vec::new()); };\n    Ok(vec![Op::from(%s { %s_id: payload.%s_id.clone(), %s_id: payload.%s_id.clone(), new_value: %s })])\n}\n" % (cl, item, rec, ps, ps, item, item, v)
    if op == "insert":
        return uses + sig + "    Ok(vec![Op::from(%s::%s { %s_id: payload.%s_id.clone(), id: payload.%s.id.clone() })])\n}\n" % (pm, pr, ps, ps, item)
    return uses + sig + parent + "    let Some(at) = parent.%s.iter().position(|row| row.id == payload.id) else { return Ok(Vec::new()); };\n    Ok(vec![Op::from(%s::%s { %s_id: payload.%s_id.clone(), index: at, %s: parent.%s[at].clone() })])\n}\n" % (cl, pm, pr, ps, ps, item, cl)


def nested_target_prop(entity, kind, order):
    return {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": entity["ent"], "de": entity["ent"]}, "description": {"en": "The %s this mutation addresses." % entity["slug"], "de": "Das Element, das diese Mutation adressiert."}, "ref": {"kind": entity["ref_kind"]}, "group": "target", "order": order}}


def nested_schema_json(k):
    c, P, op = k["e"], k["parent"], k["op"]
    item = item_of(c)
    base = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/%s/schema.json" % k["kind"], "title": k["rec"], "type": "object", "additionalProperties": False}
    pid = nested_target_prop(P, k["kind"], 10)
    if op == "change":
        fl = k["fl"]
        base["required"] = [P["slug"] + "Id", camel_of(item) + "Id", "newValue"]
        if fl["ty"] == "String":
            val = {"type": "string", "x-semio-ui": {"widget": "select" if fl["options"] else "text", "role": "value", "label": {"en": fl["en"], "de": fl["de"]}, "group": "fields", "order": 30}}
            if fl["options"]:
                val["enum"] = [o[0] for o in fl["options"]]
                val["x-semio-ui"]["options"] = [o[0] for o in fl["options"]]
        else:
            val = {"type": "number", "x-semio-ui": ui(fl["en"], fl["de"], "fields", 30, **dict(UNIT_UI[fl["unit"]]))}
        base["properties"] = {P["slug"] + "Id": pid, camel_of(item) + "Id": nested_target_prop(c, k["kind"], 20), "newValue": val}
    elif op == "insert":
        base["required"] = [P["slug"] + "Id", "index", item]
        base["properties"] = {P["slug"] + "Id": pid, "index": {"type": "integer", "minimum": 0, "x-semio-ui": ui("Position", "Position", "target", 20, description={"en": "Zero-based list position at which the record is inserted.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird."}, step=1, precision=0)}, item: {"$ref": SNAP + c["ent"], "x-semio-ui": {"role": "value", "label": {"en": c["ent"], "de": c["ent"]}, "description": {"en": "The complete record to insert.", "de": "Der vollständige einzufügende Datensatz."}, "group": "record", "order": 30}}}
    else:
        base["required"] = [P["slug"] + "Id", "id"]
        base["properties"] = {P["slug"] + "Id": pid, "id": nested_target_prop(c, k["kind"], 20)}
    return base


def camel_of(s):
    return S.camel(s.replace("-", "_"))


def nested_schema_ts(k):
    c, P, op, rec = k["e"], k["parent"], k["op"], k["rec"]
    item = item_of(c)
    head = "/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.\n * @see ./🔣️.json */\n" % (k["dirname"].split(k["kind"])[0], k["kind"], rec)
    ps = P["slug"] + "Id"
    if op == "change":
        fl = k["fl"]
        imports = "normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString"
        return head + 'import { %s } from "@semio-tech/norm-js/registry/contract";\n\nexport interface %s {\n  %s: string;\n  %sId: string;\n  newValue: %s;\n}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s: normWireRequired(normWireString), %sId: normWireRequired(normWireString), newValue: normWireRequired(%s) });\n' % (imports, rec, ps, item, TS_TY[fl["ty"]], rec, rec, rec, ps, item, S.WIRE[fl["ty"]])
    if op == "insert":
        return head + 'import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";\nimport { parse%s, type %s } from "../../../📸️snapshot/🟦️.ts";\n\nexport interface %s {\n  %s: string;\n  index: number;\n  %s: %s;\n}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), %s: normWireRequired(parse%s) });\n' % (c["ent"], c["ent"], rec, ps, item, c["ent"], rec, rec, rec, ps, item, c["ent"])
    return head + 'import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";\n\nexport interface %s {\n  %s: string;\n  id: string;\n}\n\nexport const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s: normWireRequired(normWireString), id: normWireRequired(normWireString) });\n' % (rec, ps, rec, rec, rec, ps)


def parent_row_with_children(P, snap, count):
    row = row_of(P, snap)
    out = {}
    for key, val in row.items():
        out[key] = val
        if P["slug"] == "panel" and key == "strengthClass":
            out["layers"] = [{"id": "layer-%d" % (i + 1), "thicknessM": 0.04, "orientation": ["longitudinal", "transverse"][i % 2], "grade": "C24"} for i in range(count)]
    if P["slug"] == "fastener":
        out["positions"] = [{"id": "position-%d" % (i + 1), "xM": 0.1 + 0.08 * i, "yM": 0.05} for i in range(count)]
    return out


def nested_fixture(k):
    base = FIX + "/🚶️change-member-bridge-crowd/✅apply"
    snap = json.loads(rd(base + "/🔣️.before.json"))
    c, P, op = k["e"], k["parent"], k["op"]
    pl, cl, item = P["plural"], c["plural"], item_of(c)
    diff = blank(json.loads(rd(base + "/🔣️.diff.json")))
    count = 3 if op == "remove" else 2 if op == "change" else 1
    snap[pl] = [parent_row_with_children(P, snap, count)]
    aft = copy.deepcopy(snap)
    parent_id = snap[pl][0]["id"]
    ps = P["slug"] + "Id"
    new_row = {"id": "%s-%d" % (c["slug"], count + 1)}
    for fl in c["rest"]:
        new_row[fl["camel"]] = fl["example"]
    patch = {x["camel"]: None for x in P["rest"]}
    if op == "change":
        fl = k["fl"]
        row = snap[pl][0][cl][0]
        nv = changed(fl, row[fl["camel"]])
        aft[pl][0][cl][0][fl["camel"]] = nv
        mut = {k["rec"]: {ps: parent_id, camel_of(item) + "Id": row["id"], "newValue": nv}}
        child_patch = {x["camel"]: (nv if x is fl else None) for x in c["rest"]}
        patch[cl] = {"removed": [], "inserted": [], "moved": [], "modified": [{"id": row["id"], "patch": child_patch}]}
    elif op == "insert":
        aft[pl][0][cl].append(new_row)
        mut = {k["rec"]: {ps: parent_id, "index": len(snap[pl][0][cl]), item: new_row}}
        patch[cl] = {"removed": [], "inserted": [{"index": len(snap[pl][0][cl]), "row": new_row}], "moved": [], "modified": []}
    else:
        gone = aft[pl][0][cl].pop()
        mut = {k["rec"]: {ps: parent_id, "id": gone["id"]}}
        patch[cl] = {"removed": [{"id": gone["id"], "index": len(aft[pl][0][cl])}], "inserted": [], "moved": [], "modified": []}
    diff[pl]["modified"] = [{"id": parent_id, "patch": patch}]
    return snap, mut, aft, diff


def nested_sample(k):
    c, P, op = k["e"], k["parent"], k["op"]
    item = item_of(c)
    q = "%s::%s::" % (k["ident"], k["mod"])
    pid = '%s_id: "%s-1".into()' % (P["slug"], P["slug"])
    if op == "change":
        fl = k["fl"]
        v = {"String": '"%s".into()' % (fl["options"][0][0] if fl["options"] else "x"), "u32": "1", "f64": "0.5"}[fl["ty"]]
        return '        En1995Mutation::%s(%s%s { %s, %s_id: "%s-1".into(), new_value: %s }),' % (k["rec"], q, k["rec"], pid, item, c["slug"], v)
    if op == "insert":
        fields = ", ".join("%s: %s" % (fl["name"], {"String": '"x".into()', "u32": "0", "f64": "0.0"}[fl["ty"]]) for fl in c["rest"])
        return '        En1995Mutation::%s(%s%s { %s, index: 99, %s: semio_s_artifact_norm_en1995_model::%s { id: "%s-1".into(), %s } }),' % (k["rec"], q, k["rec"], pid, item, c["ent"], c["slug"], fields)
    return '        En1995Mutation::%s(%s%s { %s, id: "%s-1".into() }),' % (k["rec"], q, k["rec"], pid, c["slug"])


def nested_gq(k):
    P, c, item = k["parent"], k["e"], item_of(k["e"])
    head = "  %sId: String! @state(class: ARTIFACT)\n" % P["slug"]
    if k["op"] == "change":
        return head + "  %sId: String! @state(class: ARTIFACT)\n  newValue: %s! @state(class: ARTIFACT)\n" % (camel_of(item), {"String": "String", "f64": "Float", "u32": "Int"}[k["fl"]["ty"]])
    if k["op"] == "insert":
        return head + "  index: Int! @state(class: ARTIFACT)\n  %s: JSON! @state(class: ARTIFACT)\n" % item
    return head + "  id: String! @state(class: ARTIFACT)\n"


def nested_pr(k):
    P, c, item = k["parent"], k["e"], item_of(k["e"])
    head = "  // @state artifact\n  string %s_id = 1;\n" % P["slug"]
    if k["op"] == "change":
        return head + "  // @state artifact\n  string %s_id = 2;\n  // @state artifact\n  %s new_value = 3;\n" % (item, {"String": "string", "f64": "double", "u32": "uint32"}[k["fl"]["ty"]])
    if k["op"] == "insert":
        return head + "  // @state artifact\n  uint32 index = 2;\n  // @state artifact\n  bytes %s = 3;\n" % item
    return head + "  // @state artifact\n  string id = 2;\n"


def main():
    root_p = MUT + "/🦀️.rs"
    root = rd(root_p)
    todo = [k for k in LEAVES if '"%s",' % k["kind"] not in root]
    print("to generate:", len(todo), "of", len(LEAVES))
    if not todo:
        return
    proto_p = ANY + "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    proto = rd(proto_p)
    tag = max(int(x) for x in re.findall(r"tag=(\d+)", proto)) + 1
    for k in todo:
        k["tag"] = tag
        tag += 1
        d = MUT + "/" + k["dirname"]
        wr(d + "/🦀️.rs", leaf_rs(k))
        wr(d + "/🔺️diff/🦀️.rs", diff_rs(k))
        wr(d + "/↩️inverse/🦀️.rs", inverse_rs(k))
        wr(d + "/🧬️schema/🔣️.json", dumps(schema_json(k)))
        wr(d + "/🧬️schema/🟦️.ts", schema_ts(k))
        wr(d + "/🔣️.json", dumps(catalog_json(k)))
        snap, mut, aft, diff = fixture(k)
        fx = FIX + "/" + k["dirname"] + "/✅apply"
        wr(fx + "/🔣️.before.json", dumps(snap))
        wr(fx + "/🔣️.mutation.json", dumps(mut))
        wr(fx + "/🔣️.after.json", dumps(aft))
        wr(fx + "/🔣️.outcome.json", dumps({"status": "applied"}))
        wr(fx + "/🔣️.diff.json", dumps(diff))
    tomlp = MUT + "/📍️placements/📦️packages/🦀️rust/Cargo.toml"
    for g in GROUP_NAMES:
        ks = [k for k in LEAVES if k["group"] == g]
        cd = GROUP_DIR[g]
        wr(MUT + "/" + cd + "/📦️packages/🦀️rust/Cargo.toml", rd(tomlp).replace("mut-placements", "mut-" + g).replace("placements mutation leaves", g + " mutation leaves"))
        mounts = "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (k["dirname"], k["mod"]) for k in ks)
        pl = rd(MUT + "/📍️placements/🦀️.rs")
        pre = pl[pl.index("#![allow"): pl.index("//#region 🪢️Mounts")]
        wr(MUT + "/" + cd + "/🦀️.rs", "//! %s EN 1995 %s mutation leaves.\n\n" % (cd.split("/")[0][:2], g) + pre + "//#region 🪢️Mounts\n" + mounts + "//#endregion 🪢️Mounts\n")
        rel = REL + cd + "/📦️packages/🦀️rust"
        cp = ART + "/Cargo.toml"
        c = rd(cp)
        if rel not in c:
            pm = '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️placements/📦️packages/🦀️rust",\n'
            c = rep(c, pm, pm + '    "%s",\n' % rel)
            pw = "[workspace.dependencies.semio-s-artifact-norm-en1995-mut-placements]\npath = \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️placements/📦️packages/🦀️rust\"\n\n"
            c = rep(c, pw, pw + "[workspace.dependencies.%s]\npath = \"%s\"\n\n" % (CRATE[g], rel))
            wr(cp, c)
        for dep in (MUT + "/📦️packages/🦀️rust/Cargo.toml", ANY + "🚪️io/📦️packages/🦀️rust/Cargo.toml", ANY + "🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml"):
            t = rd(dep)
            line = "semio-s-artifact-norm-en1995-mut-placements = { workspace = true }\n"
            if CRATE[g] not in t:
                wr(dep, rep(t, line, line + CRATE[g] + " = { workspace = true }\n"))
        sp = ART + "/📦️packages/🦀️rust/📜️script.ts"
        sc = rd(sp)
        if '"%s"' % CRATE[g] not in sc:
            wr(sp, rep(sc, '"semio-s-artifact-norm-en1995-mut-placements"', '"semio-s-artifact-norm-en1995-mut-placements","%s"' % CRATE[g]))
        o = rd(OWNERS)
        if '"name": "%s"' % CRATE[g] not in o:
            m = re.search(r'    \{\n      "name": "semio-s-artifact-norm-en1995-mut-placements",\n.*?\n    \},\n', o, re.S)
            wr(OWNERS, o.replace(m.group(0), m.group(0) + m.group(0).replace("mut-placements", "mut-" + g).replace("📍️placements", cd), 1))
        lp = ART + "/Cargo.lock"
        l = rd(lp)
        if 'name = "%s"' % CRATE[g] not in l:
            m = re.search(r'\[\[package\]\]\nname = "semio-s-artifact-norm-en1995-mut-placements"\n.*?\n\n', l, re.S)
            l = l.replace(m.group(0), m.group(0) + m.group(0).replace("mut-placements", "mut-" + g), 1)
            for pkg in ("io", "io-text", "mutations"):
                am = re.search(r'name = "semio-s-artifact-norm-en1995-%s"\nversion = "0.1.0"\ndependencies = \[\n(.*?)\]' % pkg, l, re.S)
                deps = sorted(set(am.group(1).splitlines()) | {' "%s",' % CRATE[g]})
                l = l.replace(am.group(0), am.group(0).replace(am.group(1), "\n".join(deps) + "\n"), 1)
            wr(lp, l)
    s = root
    uses = "".join("use %s::%s;\n" % (k["ident"], k["mod"]) for k in todo)
    s = after(s, "use semio_s_artifact_norm_en1995_mut_placements::remove_placement;\n", uses)
    s = after(s, "    RemovePlacement(remove_placement::RemovePlacement),\n", "".join("    %s(%s::%s),\n" % (k["rec"], k["mod"], k["rec"]) for k in todo))
    s = after(s, '    "remove-placement",\n', "".join('    "%s",\n' % k["kind"] for k in todo))
    wr(root_p, s)
    p = MUT + "/🟦️.ts"
    s = rd(p)
    s = after(s, 'import { type RemovePlacement, parseRemovePlacement } from "./➖️remove-placement/🧬️schema/🟦️.ts";\n', "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (k["rec"], k["rec"], k["dirname"]) for k in todo))
    m = re.search(r"  \| \{ RemovePlacement: RemovePlacement \}(;?)\n", s)
    assert m, "ts union"
    s = s.replace(m.group(0), "  | { RemovePlacement: RemovePlacement }\n" + "".join("  | { %s: %s }\n" % (k["rec"], k["rec"]) for k in todo)[:-1] + ";\n", 1)
    s = after(s, "  RemovePlacement: parseRemovePlacement,\n", "".join("  %s: parse%s,\n" % (k["rec"], k["rec"]) for k in todo))
    wr(p, s)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    for k in todo:
        j["oneOf"].append({"type": "object", "additionalProperties": False, "required": [k["rec"]], "properties": {k["rec"]: {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/%s/schema.json" % k["kind"]}}})
    wr(p, oneof_dumps(j))
    p = MUT + "/🔗️.graphql"
    s = rd(p)
    s = after(s, "  | RemovePlacement\n", "".join("  | %s\n" % k["rec"] for k in todo))
    gq = lambda k: nested_gq(k) if k.get("nest") else {"change": "  %sId: String! @state(class: ARTIFACT)\n  newValue: %s! @state(class: ARTIFACT)\n" % (k["e"]["slug"], {"String": "String", "f64": "Float", "u32": "Int"}[k["fl"]["ty"]]) if k["op"] == "change" else "", "insert": "  index: Int! @state(class: ARTIFACT)\n  %s: JSON! @state(class: ARTIFACT)\n" % k["e"]["slug"], "remove": "  index: Int! @state(class: ARTIFACT)\n"}[k["op"]]
    wr(p, s.rstrip("\n") + "\n\n" + "\n".join("type %s {\n%s}\n" % (k["rec"], gq(k)) for k in todo))
    p = MUT + "/🛰️.proto"
    s = rd(p)
    pr = lambda k: nested_pr(k) if k.get("nest") else {"change": "  // @state artifact\n  string %s_id = 1;\n  // @state artifact\n  %s new_value = 2;\n" % (k["e"]["slug"], {"String": "string", "f64": "double", "u32": "uint32"}[k["fl"]["ty"]]) if k["op"] == "change" else "", "insert": "  // @state artifact\n  uint32 index = 1;\n  // @state artifact\n  bytes %s = 2;\n" % k["e"]["slug"], "remove": "  // @state artifact\n  uint32 index = 1;\n"}[k["op"]]
    n = int(re.search(r"remove_placement = (\d+);", s).group(1))
    top = max(int(x) for x in re.findall(r"    \w+ \w+ = (\d+);", s[s.index("  oneof payload {"):]))
    msgs = "".join("message %s {\n%s}\n\n" % (k["rec"], pr(k)) for k in todo)
    s = rep(s, "message En1995Mutation {", msgs + "message En1995Mutation {")
    s = after(s, "    RemovePlacement remove_placement = %d;\n" % n, "".join("    %s %s = %d;\n" % (k["rec"], k["mod"], top + 1 + i) for i, k in enumerate(todo)))
    wr(p, s)
    wr(proto_p, proto.rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (k["kind"], k["tag"]) for k in todo))
    p = MUT + "/🧭️edit-rules/🦀️.rs"
    s = rd(p)
    sets = "".join(('        SetFieldRule { path: "%s[].%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%sId"), SelectorKey::Id("%sId")], value: "newValue" },\n' % (k["parent"]["plural"], k["e"]["plural"], k["fl"]["camel"], k["kind"], k["parent"]["slug"], camel_of(item_of(k["e"]))) if k.get("nest") else '        SetFieldRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%sId")], value: "newValue" },\n' % (k["e"]["plural"], k["fl"]["camel"], k["kind"], k["e"]["slug"])) for k in todo if k["op"] == "change")
    s = rep(s, "    ],\n    insert_item: &[\n", sets + "    ],\n    insert_item: &[\n")
    ins = "".join(('        InsertItemRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%sId")], index: "index", item: "%s" },\n' % (k["parent"]["plural"], k["e"]["plural"], k["kind"], k["parent"]["slug"], item_of(k["e"])) if k.get("nest") else '        InsertItemRule { path: "%s", kind: "%s", selectors: &[], index: "index", item: "%s" },\n' % (k["e"]["plural"], k["kind"], k["e"]["slug"])) for k in todo if k["op"] == "insert")
    s = rep(s, "    ],\n    remove_item: &[\n", ins + "    ],\n    remove_item: &[\n")
    i = s.rindex("    ],\n};")
    s = s[:i] + "".join('        RemoveItemRule { path: "%s", kind: "%s", selectors: &[], index: "index" },\n' % (k["e"]["plural"], k["kind"]) for k in todo if k["op"] == "remove" and not k.get("nest")) + s[i:]
    idr = "".join('        IdRemoval { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("%sId")], id: "id" },\n' % (k["parent"]["plural"], k["e"]["plural"], k["kind"], k["parent"]["slug"]) for k in todo if k["op"] == "remove" and k.get("nest"))
    if idr:
        s = rep_once(s, '        IdRemoval { path: "cases", kind: "remove-case", selectors: &[], id: "id" },\n', '        IdRemoval { path: "cases", kind: "remove-case", selectors: &[], id: "id" },\n' + idr)
    wr(p, s)
    for path in (MUT + "/🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", ANY + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️op-round-trip/🦀️.rs"):
        s = rd(path)
        lines = s.split("\n")
        i = max(n for n, ln in enumerate(lines) if ln.lstrip().startswith("En1995Mutation::RemovePlacement("))
        lines[i + 1:i + 1] = [sample(k, True) for k in todo]
        wr(path, "\n".join(lines))
    p = HARNESS + "/🦀️.rs"
    s = rd(p)
    s = after(s, '    "remove-placement",\n', "".join('    "%s",\n' % k["kind"] for k in todo))
    arms = "".join('    ("%s", (\n%s    )),\n' % (k["kind"], "".join('        include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (k["dirname"], q) for q in ["before", "mutation", "after", "outcome"])) for k in todo)
    m = re.search(r'    \("remove-placement", \(\n(?:        include_str!.*\n){4}    \)\),\n', s)
    assert m
    wr(p, s.replace(m.group(0), m.group(0) + arms, 1))
    p = HARNESS + "/🐍️.py"
    s = rd(p)
    s = after(s, '    "remove-placement",\n', "".join('    "%s",\n' % k["kind"] for k in todo))
    s = after(s, '    "remove-placement": ("➖️remove-placement", "✅apply"),\n', "".join('    "%s": ("%s", "✅apply"),\n' % (k["kind"], k["dirname"]) for k in todo))
    wr(p, s)
    p = HARNESS + "/🥒️.feature"
    out = []
    for ln in rd(p).split("\n"):
        out.append(ln)
        if re.match(r"^\s+\| remove-placement +\| ➖️remove-placement +\| ✅apply +\|$", ln):
            ind = re.match(r"^(\s+)", ln).group(1)
            out.extend("%s| %s | %s | ✅apply  |" % (ind, k["kind"], k["dirname"]) for k in todo)
    wr(p, "\n".join(out))
    p = ANY + "🔮️oracles/🔣️.json"
    oj = json.loads(rd(p))
    cat, man = oj["mutationCatalogs"][0], oj["mutationManifests"][0]
    tpl = {"change": "change-member-bridge-crowd", "insert": "insert-member", "remove": "remove-member"}
    for k in todo:
        t = tpl[k["op"]]
        v = copy.deepcopy([x for x in cat["vectors"] if x["mutationId"] == t][0])
        v["mutationId"] = k["kind"]
        v["sourceMutationDirectoryName"] = k["dirname"]
        v["mutationDirectoryName"] = k["dirname"]
        v["scenarios"][0]["id"] = k["kind"]
        cat["vectors"].append(v)
        cat["kinds"].append(k["kind"])
        m = copy.deepcopy([x for x in man["mutations"] if x["id"] == t][0])
        m["id"] = k["kind"]
        m["productionDispatch"]["operation"] = k["kind"]
        m["productionDispatch"]["variant"] = k["rec"]
        man["mutations"].append(m)
    wr(p, dumps(oj))
    tj = json.loads(rd(TAXONOMY))
    last = max(i for i, x in enumerate(tj["rows"]) if x["artifact"] == "🪵️en1995")
    rows = []
    for k in todo:
        tr = copy.deepcopy([x for x in tj["rows"] if x["artifact"] == "🪵️en1995" and x["kind"] == tpl[k["op"]]][0])
        e = k["e"]
        rec = {"change": "Changed" + k["rec"][6:], "insert": "Inserted" + e["ent"], "remove": "Removed" + e["ent"]}[k["op"]]
        tr.update(aggregateVariant=k["rec"], entity=(e["slug"] + "-" + kebab_field(e, k["fl"])) if k["op"] == "change" else e["slug"], kind=k["kind"], module=k["dirname"], record=rec, source="🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/🦀️.rs" % k["dirname"], type=k["rec"])
        rows.append(tr)
    tj["rows"][last + 1:last + 1] = rows
    wr(TAXONOMY, dumps(tj))
    print("generated", len(todo))


if __name__ == "__main__":
    main()
