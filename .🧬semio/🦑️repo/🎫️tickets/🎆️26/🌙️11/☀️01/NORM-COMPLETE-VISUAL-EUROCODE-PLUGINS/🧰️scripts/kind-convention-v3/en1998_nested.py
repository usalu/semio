"""🏢 EN 1998 nested building rows: `insert-/remove-` `member`, `storey`, `system` of `buildings[]`.

Parent selector `buildingId` (shared `NormEditRules::payload` through `InsertItemRule`/`IdRemoval`), insert keeps the optional `index` (clamped, with a `mutation.clamped` warning like `insert-storey-variable`), remove is `{buildingId, id}`.
The diff lanes already exist (`En1998BuildingPatch.{members,storeys,systems}`). Leaves live next to their siblings (`member` in `buildings`, `storey` in `buildings-storey`, `system` in `buildings-system`); no crate is added.
Layers: leaf (payload, diff, inverse, manifest, schema json/ts), mounts, mutation root (use, enum, KINDS, ts, json, proto, graphql), edit rules, testkit demo cases, text codec, binary protocol, oracle manifest, mutate harness (py, rs, feature), taxonomy, vectors.

`python -I en1998_nested.py` is idempotent (a kind whose directory exists is skipped).

📎 [sibling kind used as template](../../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-storey-variable/🦀️.rs)
"""
import copy, glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *

ANY = NORM + "/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
M = ANY + "/🧬️schema/🧬️mutations"
FX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1998/snapshot.json#/$defs/"
CHANGED = []
ROWS = [
    dict(slug="member", pas="Member", field="member", ty="En1998Member", coll="members", delta="En1998MemberDelta", crate="🏢️buildings", krate="mut_buildings", en="member", de="Bauteil", doc_en="member", doc_de="Bauteil", tag=248, example="col-c1", insert_at=1),
    dict(slug="storey", pas="Storey", field="storey", ty="En1998Storey", coll="storeys", delta="En1998StoreyDelta", crate="🏢️buildings-storey", krate="mut_buildings_storey", en="storey", de="Geschoss", doc_en="storey", doc_de="Geschoss", tag=250, example="s4", insert_at=1),
    dict(slug="system", pas="System", field="system", ty="En1998System", coll="systems", delta="En1998SystemDelta", crate="🏢️buildings-system", krate="mut_buildings_system", en="resisting system", de="Tragsystem", doc_en="lateral-force-resisting system", doc_de="Tragsystem", tag=252, example="sys-y", insert_at=1),
]
LITERAL = {"String": 'String::from("x")', "f64": "0.0", "bool": "false", "u8": "0u8", "u32": "0u32"}


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if not os.path.exists(path) or rd(path) != text:
        open(path, "w", encoding="utf-8").write(text)
        CHANGED.append(path)


def edit(path, fn):
    text = rd(path)
    out = fn(text)
    if out != text:
        open(path, "w", encoding="utf-8").write(out)
        CHANGED.append(path)


def once(text, old, new):
    assert text.count(old) == 1, (old[:100], text.count(old))
    return text.replace(old, new)


def after_once(text, anchor, add):
    return once(text, anchor, anchor + add)


def kinds():
    out = []
    for r in ROWS:
        for verb in ("insert", "remove"):
            tag = r["tag"] + (0 if verb == "insert" else 1)
            emoji = "➕️" if verb == "insert" else "➖️"
            kind = "%s-%s" % (verb, r["slug"])
            rec = verb.capitalize() + r["pas"]
            mod = kind.replace("-", "_")
            partner = ("remove" if verb == "insert" else "insert")
            out.append(dict(r, verb=verb, tag=tag, emoji=emoji, kind=kind, rec=rec, mod=mod, dirname=emoji + kind, pkind="%s-%s" % (partner, r["slug"]), pmod="%s_%s" % (partner, r["slug"]), prec=partner.capitalize() + r["pas"]))
    return out


def leaf_rs(k):
    ins = k["verb"] == "insert"
    fields = "    pub building_id: String,\n" + ("    pub index: Option<usize>,\n    pub %s: semio_s_artifact_norm_en1998_model::%s,\n" % (k["field"], k["ty"]) if ins else "    pub id: String,\n")
    label_en = ("Insert " if ins else "Remove ") + k["en"]
    label_de = k["de"] + (" einfügen" if ins else " entfernen")
    target = "vec![self.building_id.clone(), self.%s.id.clone()]" % k["field"] if ins else "vec![self.building_id.clone(), self.id.clone()]"
    bound = "From<crate::%s::%s>" % (k["pmod"], k["prec"])
    return '''//! %s `%s` mutation leaf.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct %s {
%s}

impl<Op> protocol::MutationKind<En1998Snapshot, Op> for %s where Op: protocol::Mutation<En1998Snapshot, Diff = semio_s_artifact_norm_en1998_diff::En1998Diff> + %s {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "%s",
        entity: "%s",
        kind: "%s",
        record: "%s",
    };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("%s", "%s")
    }
    fn target(&self) -> Vec<String> {
        %s
    }
}

impl<Op> protocol::ColdMutationKind<En1998Snapshot, Op> for %s where Op: protocol::ColdMutation<En1998Snapshot, ReferenceDiff = semio_s_artifact_norm_en1998_diff::En1998Diff> + %s {
    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<En1998Snapshot>>::ReferenceDiff> {
        crate::%s::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        crate::%s::inverse::inverse(self, base)
    }
}

//#region 🪢️Mounts
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🪢️Mounts
''' % (k["emoji"], k["kind"], k["rec"], fields, k["rec"], bound, k["verb"], k["slug"], k["kind"], k["rec"], label_en, label_de, target, k["rec"], bound, k["mod"], k["mod"])


def diff_rs(k):
    head = '''//! %s `%s` diff: %s.

use crate::%s::%s;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, %s};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &%s, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
''' % (k["emoji"], k["kind"], "inserts the %s row; an id the list holds is a `mutation.duplicate-id`" % k["en"] if k["verb"] == "insert" else "removes the addressed %s row" % k["en"], k["mod"], k["rec"], k["delta"], k["rec"])
    if k["verb"] == "insert":
        return head + '''    if r0.%(coll)s.iter().any(|existing| existing.id == payload.%(field)s.id) {
        let key = payload.%(field)s.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("%(en)s {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(r0.%(coll)s.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { %(coll)s: %(delta)s::insertion(index, payload.%(field)s.clone()), ..Default::default() }), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the %(en)s list; inserted at {index}.", payload.index.unwrap_or(index)))
}
''' % k
    return head + '''    let Some(index) = r0.%(coll)s.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("%(en)s {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { %(coll)s: %(delta)s::removal(&r0.%(coll)s, index), ..Default::default() }), ..Default::default() })
}
''' % k


def inverse_rs(k):
    head = '''//! ↩️ `%s` inverse: %s.

use crate::%s::%s;
use crate::%s::%s;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &%s, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::%s::%s> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

''' % (k["kind"], "removes the inserted row again" if k["verb"] == "insert" else "inserts the removed row back at its position", k["mod"], k["rec"], k["pmod"], k["prec"], k["rec"], k["pmod"], k["prec"])
    if k["verb"] == "insert":
        return head + '''fn previous(payload: &%(rec)s, base: &En1998Snapshot) -> Option<%(prec)s> {
    let _r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    Some(%(prec)s { building_id: payload.building_id.clone(), id: payload.%(field)s.id.clone() })
}
''' % k
    return head + '''fn previous(payload: &%(rec)s, base: &En1998Snapshot) -> Option<%(prec)s> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let (index, row) = r0.%(coll)s.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(%(prec)s { building_id: payload.building_id.clone(), index: Some(index), %(field)s: row.clone() })
}
''' % k


def kind_json(k):
    return dumps({"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/" + k["dirname"], "semanticKind": k["kind"], "displayName": k["verb"].capitalize() + " " + k["en"], "emoji": k["emoji"], "aggregateVariant": k["rec"], "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": k["tag"], "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op", "rejected"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]})


def building_prop():
    return {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": "Building", "de": "Gebäude"}, "description": {"en": "The building this mutation addresses, by id.", "de": "Das adressierte Element (Gebäude), per Kennung."}, "ref": {"kind": "building"}, "group": "target", "order": 10}}


def schema_json(k):
    lit = k["rec"][:1].lower() + k["rec"][1:]
    props = {"mutation": {"const": lit}, "buildingId": building_prop()}
    if k["verb"] == "insert":
        props["index"] = {"type": ["integer", "null"], "minimum": 0, "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Position", "de": "Position"}, "description": {"en": "Zero-based list position at which the record is inserted. Empty appends; positions past the end append.", "de": "Nullbasierte Listenposition, an der der Datensatz eingefügt wird. Ohne Angabe am Ende; Positionen hinter dem Ende hängen an."}, "step": 1, "precision": 0, "group": "target", "order": 20}}
        props[k["field"]] = {"$ref": SNAP + k["ty"], "x-semio-ui": {"role": "value", "label": {"en": k["en"].capitalize(), "de": k["de"]}, "description": {"en": "The complete %s record to insert." % k["doc_en"], "de": "Der vollständige einzufügende Datensatz: %s." % k["doc_de"]}, "group": "record", "order": 30}}
        required = ["mutation", "buildingId", k["field"]]
    else:
        props["id"] = {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": k["en"].capitalize(), "de": k["de"]}, "description": {"en": "The %s this mutation addresses, by id." % k["doc_en"], "de": "Das adressierte Element (%s), per Kennung." % k["doc_de"]}, "ref": {"kind": k["slug"]}, "group": "target", "order": 20}}
        required = ["mutation", "buildingId", "id"]
    return dumps({"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/%s/schema.json" % k["kind"], "title": k["rec"], "type": "object", "additionalProperties": False, "required": required, "properties": props})


def schema_ts(k):
    lit = k["rec"][:1].lower() + k["rec"][1:]
    ins = k["verb"] == "insert"
    head = '''/** %s `%s` wire twin: the leaf payload `%s`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
''' % (k["emoji"], k["kind"], k["rec"])
    if ins:
        return head + '''import { type NormWireReader, normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type %(ty)s, parse%(ty)s } from "../../../📸️snapshot/🟦️.ts";

export interface %(rec)s {
  mutation: "%(lit)s";
  buildingId: string;
  index?: number | null;
  %(field)s: %(ty)s;
}

export const parse%(rec)s: NormWireReader<%(rec)s> = normWireObject<%(rec)s>({ mutation: normWireRequired(normWireLiteral("%(lit)s")), buildingId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), %(field)s: normWireRequired(parse%(ty)s) });
''' % dict(k, lit=lit)
    return head + '''import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface %(rec)s {
  mutation: "%(lit)s";
  buildingId: string;
  id: string;
}

export const parse%(rec)s: NormWireReader<%(rec)s> = normWireObject<%(rec)s>({ mutation: normWireRequired(normWireLiteral("%(lit)s")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
''' % dict(k, lit=lit)


def write_leaf(k):
    d = M + "/" + k["dirname"]
    write(d + "/🦀️.rs", leaf_rs(k))
    write(d + "/🔺️diff/🦀️.rs", diff_rs(k))
    write(d + "/↩️inverse/🦀️.rs", inverse_rs(k))
    write(d + "/🔣️.json", kind_json(k))
    write(d + "/🧬️schema/🔣️.json", schema_json(k))
    write(d + "/🧬️schema/🟦️.ts", schema_ts(k))


def mounts(ks):
    for crate in sorted({k["crate"] for k in ks}):
        mine = [k for k in ks if k["crate"] == crate]
        def fn(text, mine=mine):
            add = "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (k["dirname"], k["mod"]) for k in mine if "pub mod %s;" % k["mod"] not in text)
            return once(text, "//#endregion 🪢️Mounts", add + "//#endregion 🪢️Mounts") if add else text
        edit(M + "/" + crate + "/🦀️.rs", fn)

    def reexport(text):
        for crate, krate in (("🏢️buildings-storey", "mut_buildings_storey"), ("🏢️buildings-system", "mut_buildings_system")):
            names = [k["mod"] for k in ks if k["crate"] == crate]
            m = re.search(r"pub use semio_s_artifact_norm_en1998_%s::\{([^}]*)\};" % krate, text)
            assert m, krate
            have = [x.strip() for x in m.group(1).split(",")]
            merged = have + [n for n in names if n not in have]
            text = text.replace(m.group(0), "pub use semio_s_artifact_norm_en1998_%s::{%s};" % (krate, ", ".join(merged)))
        return text
    edit(M + "/🏢️buildings/🦀️.rs", reexport)


def root(ks):
    def rs(text):
        if "insert_member" in text:
            return text
        uses = "".join("use semio_s_artifact_norm_en1998_mut_buildings::%s;\n" % k["mod"] for k in ks)
        text = after_once(text, "use semio_s_artifact_norm_en1998_mut_buildings::remove_storey_variable;\n", uses)
        text = after_once(text, "    RemoveTowerVariable(remove_tower_variable::RemoveTowerVariable),\n", "".join("    %s(%s::%s),\n" % (k["rec"], k["mod"], k["rec"]) for k in ks))
        return after_once(text, '    "remove-tower-variable",\n', "".join('    "%s",\n' % k["kind"] for k in ks))
    edit(M + "/🦀️.rs", rs)

    def ts(text):
        if "InsertMember" in text:
            return text
        text = after_once(text, 'import { type RemoveStoreyVariable, parseRemoveStoreyVariable } from "./➖️remove-storey-variable/🧬️schema/🟦️.ts";\n', "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (k["rec"], k["rec"], k["dirname"]) for k in ks))
        text = after_once(text, "  | RemoveTowerVariable\n", "".join("  | %s\n" % k["rec"] for k in ks))
        return after_once(text, "  removeTowerVariable: parseRemoveTowerVariable,\n", "".join("  %s: parse%s,\n" % (k["rec"][:1].lower() + k["rec"][1:], k["rec"]) for k in ks))
    edit(M + "/🟦️.ts", ts)

    def js(text):
        if "insert-member" in text:
            return text
        tail = "\n    }\n  ]\n}"
        assert text.rstrip("\n").endswith(tail)
        block = ',\n    {\n      "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/%s/schema.json"\n    }'
        return text.rstrip("\n")[: -len(tail)] + "\n    }" + "".join(block % k["kind"] for k in ks) + "\n  ]\n}\n"
    edit(M + "/🔣️.json", js)

    def proto(text):
        if "message InsertMember" in text:
            return text
        msgs = ""
        for k in ks:
            if k["verb"] == "insert":
                body = "/* @state artifact */ string building_id = 1; /* @state artifact */ optional uint32 index = 2; /* @state artifact */ bytes %s = 3;" % k["field"]
            else:
                body = "/* @state artifact */ string building_id = 1; /* @state artifact */ string id = 2;"
            msgs += "// kind: %s\nmessage %s { %s }\n" % (k["kind"], k["rec"], body)
        text = after_once(text, "message RemoveStoreyVariable { /* @state artifact */ string building_id = 1; /* @state artifact */ string storey_id = 2; /* @state artifact */ string id = 3; }\n", msgs)
        number = max(int(n) for n in re.findall(r"= (\d+);\n  }\n}", text)) + 1
        entries = "".join("    %s %s = %d;\n" % (k["rec"], re.sub(r"(?<!^)([A-Z])", r"_\1", k["rec"]).lower(), number + i) for i, k in enumerate(ks))
        return once(text, "    ChangeBridgeDeckEiNm2 change_bridge_deck_ei_nm2 = 224;\n", "    ChangeBridgeDeckEiNm2 change_bridge_deck_ei_nm2 = 224;\n" + entries)
    edit(M + "/🛰️.proto", proto)

    def gql(text):
        if "type InsertMember" in text:
            return text
        text = after_once(text, "  | RemoveTowerVariable\n", "".join("  | %s\n" % k["rec"] for k in ks))
        types = ""
        for k in ks:
            if k["verb"] == "insert":
                fields = "buildingId: String! @state(class: ARTIFACT) index: Int @state(class: ARTIFACT) %s: JSON! @state(class: ARTIFACT)" % k["field"]
            else:
                fields = "buildingId: String! @state(class: ARTIFACT) id: String! @state(class: ARTIFACT)"
            types += '# kind: "%s"\ntype %s { %s }\n' % (k["kind"], k["rec"], fields)
        return text.rstrip("\n") + "\n" + types
    edit(M + "/🔗️.graphql", gql)


def edit_rules(ks):
    def fn(text):
        if '"insert-member"' in text:
            return text
        ins = "".join('        InsertItemRule { path: "buildings[].%s", kind: "%s", selectors: &[SelectorKey::Id("buildingId")], index: "index", item: "%s" },\n' % (k["coll"], k["kind"], k["field"]) for k in ks if k["verb"] == "insert")
        rem = "".join('        IdRemoval { path: "buildings[].%s", kind: "%s", selectors: &[SelectorKey::Id("buildingId")], id: "id" },\n' % (k["coll"], k["kind"]) for k in ks if k["verb"] == "remove")
        text = after_once(text, '        InsertItemRule { path: "buildings[].storeys[].variables", kind: "insert-storey-variable", selectors: &[SelectorKey::Id("buildingId"), SelectorKey::Id("storeyId")], index: "index", item: "variable" },\n', ins)
        return after_once(text, '        IdRemoval { path: "buildings[].storeys[].variables", kind: "remove-storey-variable", selectors: &[SelectorKey::Id("buildingId"), SelectorKey::Id("storeyId")], id: "id" },\n', rem)
    edit(M + "/🧭️edit-rules/🦀️.rs", fn)


def literal(ty):
    text = rd(ANY + "/🧬️schema/📸️snapshot/🦀️.rs")
    body = re.search(r"pub struct %s \{(.*?)\n\}" % ty, text, re.S).group(1)
    parts = []
    for name, kind in re.findall(r"pub (\w+): ([^,\n]+),", body):
        parts.append("%s: %s" % (name, "Vec::new()" if kind.startswith("Vec") else LITERAL[kind]))
    return "semio_s_artifact_norm_en1998_model::%s { %s }" % (ty, ", ".join(parts))


def demo_and_text(ks):
    type_uses = "".join("use semio_s_artifact_norm_en1998_mut_buildings::%s::%s;\n" % (k["mod"], k["rec"]) for k in ks)
    mod_uses = "".join("use semio_s_artifact_norm_en1998_mut_buildings::%s;\n" % k["mod"] for k in ks)

    def demo(text):
        if "InsertMember" in text:
            return text
        text = after_once(text, "use semio_s_artifact_norm_en1998_mut_buildings::insert_storey_variable::InsertStoreyVariable;\nuse semio_s_artifact_norm_en1998_mut_buildings::remove_storey_variable::RemoveStoreyVariable;\n", type_uses)
        text = after_once(text, "use semio_s_artifact_norm_en1998_mut_buildings::insert_storey_variable;\nuse semio_s_artifact_norm_en1998_mut_buildings::remove_storey_variable;\n", mod_uses)
        cases = ""
        for k in ks:
            if k["verb"] == "insert":
                cases += '        En1998Mutation::%s(%s { building_id: String::from("building-1"), index: Some(1usize), %s: %s }),\n' % (k["rec"], k["rec"], k["field"], literal(k["ty"]))
            else:
                cases += '        En1998Mutation::%s(%s { building_id: String::from("building-1"), id: String::from("%s-1") }),\n' % (k["rec"], k["rec"], k["slug"])
        anchor = '        En1998Mutation::RemoveStoreyVariable(RemoveStoreyVariable { building_id: String::from("building-1"), storey_id: String::from("storey-1"), id: String::from("variable-1") }),\n'
        return after_once(text, anchor, cases)
    edit(M + "/🎲️demo-cases/🦀️.rs", demo)

    def codec(text):
        if "InsertMember" in text:
            return text
        text = after_once(text, "use semio_s_artifact_norm_en1998_mut_buildings::insert_storey_variable::InsertStoreyVariable;\nuse semio_s_artifact_norm_en1998_mut_buildings::remove_storey_variable::RemoveStoreyVariable;\n", type_uses)
        text = after_once(text, "use semio_s_artifact_norm_en1998_mut_buildings::insert_storey_variable;\nuse semio_s_artifact_norm_en1998_mut_buildings::remove_storey_variable;\n", mod_uses)
        printed = ""
        for k in ks:
            if k["verb"] == "insert":
                printed += '        En1998Mutation::%s(p) => format!("%s building-id={} index={} %s={}", enc_json(&p.building_id), enc_json(&p.index), enc_json(&p.%s)),\n' % (k["rec"], k["kind"], k["field"], k["field"])
            else:
                printed += '        En1998Mutation::%s(p) => format!("%s building-id={} id={}", enc_json(&p.building_id), enc_json(&p.id)),\n' % (k["rec"], k["kind"])
        text = after_once(text, '        En1998Mutation::RemoveStoreyVariable(p) => format!("remove-storey-variable building-id={} storey-id={} id={}", enc_json(&p.building_id), enc_json(&p.storey_id), enc_json(&p.id)),\n', printed)
        parsed = ""
        for k in ks:
            if k["verb"] == "insert":
                parsed += '        "%s" => Ok(En1998Mutation::%s(%s { building_id: dec_json(&arg("building-id")?)?, index: opt_index(&args)?, %s: dec_json(&arg("%s")?)? })),\n' % (k["kind"], k["rec"], k["rec"], k["field"], k["field"])
            else:
                parsed += '        "%s" => Ok(En1998Mutation::%s(%s { building_id: dec_json(&arg("building-id")?)?, id: dec_json(&arg("id")?)? })),\n' % (k["kind"], k["rec"], k["rec"])
        m = re.search(r'        "remove-storey-variable" => Ok\(.*\n', text)
        assert m
        return text.replace(m.group(0), m.group(0) + parsed)
    edit(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs", codec)


def protocol(ks):
    def fn(text):
        if "record insert-member" in text:
            return text
        return text.rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (k["kind"], k["tag"]) for k in ks)
    edit(ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", fn)


def oracle(ks):
    def fn(text):
        if '"insert-member"' in text:
            return text
        for verb in ("insert", "remove"):
            tmpl = "%s-storey-variable" % verb
            emoji = "➕️" if verb == "insert" else "➖️"
            for pat in (re.compile(r'        \{\n          "mutationId": "%s",.*?\n        \},\n' % tmpl, re.S), re.compile(r'        \{\n          "id": "%s",.*?\n        \},\n' % tmpl, re.S)):
                m = pat.search(text)
                assert m, (tmpl, pat.pattern[:50])
                blocks = "".join(m.group(0).replace(tmpl, k["kind"]).replace("%sStoreyVariable" % verb.capitalize(), k["rec"]) for k in ks if k["verb"] == verb)
                text = text[: m.end()] + blocks + text[m.end():]
            line = '        "%s",\n' % tmpl
            assert text.count(line) == 1, line
            text = text.replace(line, line + "".join('        "%s",\n' % k["kind"] for k in ks if k["verb"] == verb))
        return text
    edit(ANY + "/🔮️oracles/🔣️.json", fn)


def harness(ks):
    t = ANY + "/🧪️tests/🫨️mutate-en1998-1"

    def py(text):
        if '"insert-member"' in text:
            return text
        text = after_once(text, '    "remove-storey-variable",\n', "".join('    "%s",\n' % k["kind"] for k in ks))
        return after_once(text, '    "remove-storey-variable": ("➖️remove-storey-variable", "✅apply"),\n', "".join('    "%s": ("%s", "✅apply"),\n' % (k["kind"], k["dirname"]) for k in ks))
    edit(t + "/🐍️.py", py)

    def rs(text):
        if '"insert-member"' in text:
            return text
        text = after_once(text, '    "remove-storey-variable",\n', "".join('    "%s",\n' % k["kind"] for k in ks))
        out = text
        for verb in ("insert", "remove"):
            tmpl = "%s-storey-variable" % verb
            m = re.search(r'        "%s" => \(\n.*?\n        \),\n' % tmpl, out, re.S)
            assert m, tmpl
            out = out[: m.end()] + "".join(m.group(0).replace(tmpl, k["kind"]) for k in ks if k["verb"] == verb) + out[m.end():]
        return out
    edit(t + "/🦀️.rs", rs)

    def feature(text):
        if "insert-member" in text:
            return text
        row = "      | remove-storey-variable | ➖️remove-storey-variable | ✅apply  |\n"
        assert text.count(row) == 2
        return text.replace(row, row + "".join("      | %s | %s | ✅apply  |\n" % (k["kind"], k["dirname"]) for k in ks))
    edit(t + "/🥒️.feature", feature)


def taxonomy(ks):
    t = json.loads(rd(TAXONOMY))
    name = "🫨️en1998"
    have = {r["kind"] for r in t["rows"] if r["artifact"] == name}
    at = max(i for i, r in enumerate(t["rows"]) if r["artifact"] == name)
    new = [{"aggregateVariant": k["rec"], "artifact": name, "entity": k["slug"], "kind": k["kind"], "module": k["dirname"], "physicalLayout": "direct", "record": k["rec"], "source": "%s/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/🦀️.rs" % (name, k["dirname"]), "standard": "🔖️1", "subset": "✳️any", "type": k["rec"], "verb": k["verb"]} for k in ks if k["kind"] not in have]
    if not new:
        return
    t["rows"][at + 1: at + 1] = new
    write(TAXONOMY, dumps(t))


def vectors(ks):
    src = glob.glob(FX + "/*insert-storey-variable")[0] + "/✅apply"
    base = json.loads(rd(src + "/🔣️.before.json"))
    base_diff = json.loads(rd(src + "/🔣️.diff.json"))
    for k in ks:
        d = FX + "/" + k["dirname"] + "/✅apply"
        if os.path.exists(d + "/🔣️.mutation.json"):
            continue
        lit = k["rec"][:1].lower() + k["rec"][1:]
        after = copy.deepcopy(base)
        building = next(b for b in after["buildings"] if b["id"] == "bldg-office")
        rows = building[k["coll"]]
        diff = copy.deepcopy(base_diff)
        patch = diff["buildings"]["modified"][0]["patch"]
        for lane in ("members", "storeys", "systems"):
            patch[lane] = {"removed": [], "inserted": [], "moved": [], "modified": []}
        if k["verb"] == "insert":
            row = copy.deepcopy(rows[0])
            row["id"] = rows[0]["id"] + "-2"
            at = min(k["insert_at"], len(rows))
            rows.insert(at, row)
            patch[k["coll"]]["inserted"] = [{"index": at, "row": row}]
            mutation = {"mutation": lit, "buildingId": "bldg-office", "index": at, k["field"]: row}
        else:
            at = next(i for i, r in enumerate(rows) if r["id"] == k["example"])
            rows.pop(at)
            patch[k["coll"]]["removed"] = [{"id": k["example"], "index": at}]
            mutation = {"mutation": lit, "buildingId": "bldg-office", "id": k["example"]}
        write(d + "/🔣️.before.json", dumps(base))
        write(d + "/🔣️.after.json", dumps(after))
        write(d + "/🔣️.mutation.json", dumps(mutation))
        write(d + "/🔣️.outcome.json", dumps({"status": "applied"}))
        write(d + "/🔣️.diff.json", dumps(diff))


def main():
    ks = kinds()
    for k in ks:
        write_leaf(k)
    mounts(ks)
    root(ks)
    edit_rules(ks)
    demo_and_text(ks)
    protocol(ks)
    oracle(ks)
    harness(ks)
    taxonomy(ks)
    vectors(ks)
    print(len(set(CHANGED)), "files")


if __name__ == "__main__":
    main()
