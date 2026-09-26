#!/usr/bin/env python3
"""🧬️ Regenerates the EN 1995 mutation vocabulary for the redesigned TimberMember / TimberConnection.

Emits one leaf directory per mutation kind (🦀️.rs, 🔣️.json, 🧬️schema/🔣️.json, 🔺️diff, ↩️inverse,
🧪️tests/<scenario>), the aggregate `🧬️mutations/🦀️.rs`, the crate-root mounts, the field-meta table,
the text demo/unit tests, the index files (🔣️.json / 🟦️.ts / 🔗️.graphql / 🛰️.proto), the oracle
manifest catalog and the change-annex fixture snapshots.

Ticket-scoped tool — run once, then delete the `🗑️generated` folder.
"""
from __future__ import annotations

import json
import os
import re
import shutil
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
ART = REPO / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995"
ANY = ART / "🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY / "🧬️schema/🧬️mutations"
MUT_REL = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
OWNER_PREFIX = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/" + MUT_REL
SCHEMA_ID = "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutation/{kind}/schema.json"


def to_kebab(name: str) -> str:
    chars = list(name)
    out = ""
    for i, c in enumerate(chars):
        if c in "_-":
            if out and not out.endswith("-"):
                out += "-"
            continue
        if c.isupper():
            prev = chars[i - 1] if i > 0 else None
            nxt = chars[i + 1] if i + 1 < len(chars) else None
            if prev is None:
                boundary = False
            elif prev.islower() or prev.isdigit():
                boundary = True
            elif prev.isupper():
                boundary = nxt is not None and nxt.islower()
            else:
                boundary = False
            if boundary and out and not out.endswith("-"):
                out += "-"
            out += c.lower()
        else:
            out += c
    return out


# ---------------------------------------------------------------------------
# Field tables: (suffix, rust_field, camel, rtype, jtype, label_en, label_de, unit, choices, emoji, test_value)
# jtype: string | number | integer | boolean | enum:<CONST>
# ---------------------------------------------------------------------------
MEMBER_FIELDS = [
    ("LabelEn", "label_en", "labelEn", "String", "string", "Label (EN)", "Bezeichnung (EN)", None, None, "🏷️", '"Beam B1 (revised)".into()'),
    ("LabelDe", "label_de", "labelDe", "String", "string", "Label (DE)", "Bezeichnung (DE)", None, None, "🏷️", '"Träger B1 (überarbeitet)".into()'),
    ("Role", "role", "role", "MemberRole", "enum:ROLE", "Structural role", "Tragwerksrolle", None, "ROLE", "🎯️", "crate::MemberRole::Column"),
    ("StrengthClass", "strength_class", "strengthClass", "String", "string", "Strength class", "Festigkeitsklasse", None, "STRENGTH", "🛡️", '"GL32h".into()'),
    ("ServiceClass", "service_class", "serviceClass", "u8", "integer", "Service class", "Nutzungsklasse", None, "SERVICE", "🌧️", "2"),
    ("Support", "support", "support", "SupportType", "enum:SUPPORT", "Support type", "Lagerungsart", None, "SUPPORT", "📍️", "crate::SupportType::Cantilever"),
    ("B", "b_m", "bM", "f64", "number", "Width b", "Breite b", "m", None, "↔️", "0.24"),
    ("H", "h_m", "hM", "f64", "number", "Depth h", "Höhe h", "m", None, "↕️", "0.5"),
    ("Span", "span_m", "spanM", "f64", "number", "Span L", "Spannweite L", "m", None, "↔️", "7.0"),
    ("SupportLength", "support_length_m", "supportLengthM", "f64", "number", "Support length", "Auflagerlänge", "m", None, "↔️", "0.2"),
    ("BearingLength", "bearing_length_m", "bearingLengthM", "f64", "number", "Bearing length l", "Aufstandslänge l", "m", None, "↔️", "0.2"),
    ("BucklingY", "buckling_length_y_m", "bucklingLengthYM", "f64", "number", "Buckling length L_y", "Knicklänge L_y", "m", None, "↔️", "6.0"),
    ("BucklingZ", "buckling_length_z_m", "bucklingLengthZM", "f64", "number", "Buckling length L_z", "Knicklänge L_z", "m", None, "↔️", "2.0"),
    ("LateralRestraint", "lateral_restraint_spacing_m", "lateralRestraintSpacingM", "f64", "number", "Lateral restraint spacing", "Abstand seitlicher Halterungen", "m", None, "↔️", "1.0"),
    ("NotchDepth", "notch_depth_m", "notchDepthM", "f64", "number", "Notch depth h_e", "Ausklinkungstiefe h_e", "m", None, "↔️", "0.02"),
    ("NotchDistance", "notch_distance_m", "notchDistanceM", "f64", "number", "Notch distance x", "Ausklinkungsabstand x", "m", None, "↔️", "0.1"),
    ("MCrit", "m_crit_nm", "mCritNm", "f64", "number", "Critical moment M_crit", "Kippmoment M_crit", "N·m", None, "⚠️", "150000.0"),
    ("MassPerM", "mass_kg_per_m", "massKgPerM", "f64", "number", "Linear mass", "Längenbezogene Masse", "kg/m", None, "⚖️", "140.0"),
    ("MassPerM2", "mass_kg_per_m2", "massKgPerM2", "f64", "number", "Area mass", "Flächenbezogene Masse", "kg/m²", None, "⚖️", "60.0"),
    ("Damping", "damping_xi", "dampingXi", "f64", "number", "Modal damping ratio ξ", "Modales Dämpfungsmaß ξ", None, None, "🌊️", "0.02"),
    ("FireDuration", "fire_duration_s", "fireDurationS", "f64", "number", "Fire duration", "Branddauer", "s", None, "🔥️", "1800.0"),
    ("BridgeNObs", "bridge_n_obs", "bridgeNObs", "f64", "number", "Observed load cycles per year N_obs", "Beobachtete Lastzyklen pro Jahr N_obs", "1/a", None, "🌉️", "1.0e5"),
    ("BridgeTLYears", "bridge_t_l_years", "bridgeTLYears", "f64", "number", "Design working life t_L", "Nutzungsdauer t_L", "a", None, "🌉️", "50.0"),
    ("BridgeBeta", "bridge_beta", "bridgeBeta", "f64", "number", "Fatigue exponent β", "Ermüdungsexponent β", None, None, "🌉️", "5.0"),
    ("BridgeA", "bridge_a", "bridgeA", "f64", "number", "Fatigue intercept a", "Ermüdungsparameter a", None, None, "🌉️", "15.0"),
    ("BridgeB", "bridge_b", "bridgeB", "f64", "number", "Fatigue slope b", "Ermüdungsparameter b", None, None, "🌉️", "4.0"),
    ("BridgeCrowd", "bridge_crowd_per_m2", "bridgeCrowdPerM2", "f64", "number", "Pedestrian crowd density", "Fußgängerdichte", "1/m²", None, "🚶️", "1.0"),
]

MEMBER_ACTION_FIELDS = [
    ("Kind", "kind", "kind", "String", "string", "Action kind", "Einwirkungsart", None, "ACTION_KIND", "⚖️", '"imposed".into()'),
    ("Category", "category", "category", "String", "string", "Imposed load category", "Nutzlastkategorie", None, "CATEGORY", "🏢️", '"B".into()'),
    ("LoadDuration", "load_duration", "loadDuration", "String", "string", "Load-duration class", "Klasse der Lasteinwirkungsdauer", None, "LOAD_DURATION", "⏳️", '"short".into()'),
    ("QLine", "q_line_n_per_m", "qLineNPerM", "f64", "number", "Line load q_k", "Streckenlast q_k", "N/m", None, "⬇️", "3500.0"),
    ("FPoint", "f_point_n", "fPointN", "f64", "number", "Point load F_k", "Einzellast F_k", "N", None, "⬇️", "2000.0"),
    ("MK", "m_k_nm", "mKNm", "f64", "number", "Characteristic moment M_k", "Charakteristisches Moment M_k", "N·m", None, "⤴️", "12000.0"),
    ("VK", "v_k_n", "vKN", "f64", "number", "Characteristic shear V_k", "Charakteristische Querkraft V_k", "N", None, "↕️", "8000.0"),
    ("NK", "n_k_n", "nKN", "f64", "number", "Characteristic compression N_k", "Charakteristische Druckkraft N_k", "N", None, "🏋️", "5000.0"),
    ("NTK", "n_t_k_n", "nTKN", "f64", "number", "Characteristic tension N_t,k", "Charakteristische Zugkraft N_t,k", "N", None, "🏋️", "3000.0"),
    ("FC90K", "f_c90_k_n", "fC90KN", "f64", "number", "Characteristic compression perp. F_c,90,k", "Charakteristische Querdruckkraft F_c,90,k", "N", None, "🏋️", "9000.0"),
]

CONNECTION_FIELDS = [
    ("LabelEn", "label_en", "labelEn", "String", "string", "Label (EN)", "Bezeichnung (EN)", None, None, "🏷️", '"Bolt group (revised)".into()'),
    ("LabelDe", "label_de", "labelDe", "String", "string", "Label (DE)", "Bezeichnung (DE)", None, None, "🏷️", '"Schraubverbund (überarbeitet)".into()'),
    ("FastenerType", "fastener_type", "fastenerType", "String", "string", "Fastener type", "Verbindungsmittel", None, "FASTENER", "🔩️", '"screw".into()'),
    ("StrengthClass", "strength_class", "strengthClass", "String", "string", "Strength class", "Festigkeitsklasse", None, "STRENGTH", "🛡️", '"C24".into()'),
    ("ServiceClass", "service_class", "serviceClass", "u8", "integer", "Service class", "Nutzungsklasse", None, "SERVICE", "🌧️", "2"),
    ("Diameter", "diameter_m", "diameterM", "f64", "number", "Fastener diameter d", "Durchmesser d", "m", None, "↔️", "0.016"),
    ("Number", "number", "number", "u32", "integer", "Number of fasteners", "Anzahl Verbindungsmittel", None, None, "🔢️", "12"),
    ("Rows", "rows", "rows", "u32", "integer", "Rows", "Reihen", None, None, "🔢️", "3"),
    ("Spacing", "spacing_m", "spacingM", "f64", "number", "Spacing a₁", "Abstand a₁", "m", None, "↔️", "0.1"),
    ("EdgeDistance", "edge_distance_m", "edgeDistanceM", "f64", "number", "Edge distance a₄,t", "Randabstand a₄,t", "m", None, "↔️", "0.05"),
    ("EndDistance", "end_distance_m", "endDistanceM", "f64", "number", "End distance a₃,t", "Hirnholzabstand a₃,t", "m", None, "↔️", "0.1"),
    ("T1", "t1_m", "t1M", "f64", "number", "Member thickness t₁", "Bauteildicke t₁", "m", None, "↔️", "0.22"),
    ("T2", "t2_m", "t2M", "f64", "number", "Member thickness t₂", "Bauteildicke t₂", "m", None, "↔️", "0.22"),
    ("SteelPlate", "steel_plate", "steelPlate", "bool", "boolean", "Steel plate", "Stahlblech", None, "BOOL", "🔩️", "true"),
    ("SteelPlateThickness", "steel_plate_thickness_m", "steelPlateThicknessM", "f64", "number", "Steel plate thickness", "Stahlblechdicke", "m", None, "↔️", "0.008"),
    ("ShearPlanes", "shear_planes", "shearPlanes", "u32", "integer", "Shear planes", "Scherflächen", None, None, "🔢️", "2"),
    ("FUK", "f_u_k", "fUK", "f64", "number", "Fastener tensile strength f_u,k", "Zugfestigkeit f_u,k", "Pa", None, "🛡️", "500_000_000.0"),
]

CONNECTION_ACTION_FIELDS = [
    ("Kind", "kind", "kind", "String", "string", "Action kind", "Einwirkungsart", None, "ACTION_KIND", "⚖️", '"imposed".into()'),
    ("LoadDuration", "load_duration", "loadDuration", "String", "string", "Load-duration class", "Klasse der Lasteinwirkungsdauer", None, "LOAD_DURATION", "⏳️", '"short".into()'),
    ("FK", "f_k_n", "fKN", "f64", "number", "Characteristic fastener force F_k", "Charakteristische Verbindungsmittelkraft F_k", "N", None, "🔩️", "12000.0"),
]


class Leaf:
    def __init__(self, variant: str, scope: str, emoji: str, **kw):
        self.variant = variant
        self.scope = scope
        self.emoji = emoji
        self.kind = to_kebab(variant)
        self.module = "set_snapshot" if self.kind == "change-annex" else self.kind.replace("-", "_")
        self.dir = f"{emoji}{self.kind}"
        self.tag = None
        for k, v in kw.items():
            setattr(self, k, v)

    @property
    def verb(self) -> str:
        return self.kind.split("-")[0]

    @property
    def record(self) -> str:
        return {"change": "Changed", "insert": "Inserted", "remove": "Removed"}[self.verb] + self.variant[len(self.verb):]


LEAVES: list[Leaf] = []
LEAVES.append(Leaf("ChangeAnnex", "annex", "🌍️"))
LEAVES.append(Leaf("InsertMember", "member-insert", "➕️"))
LEAVES.append(Leaf("RemoveMember", "member-remove", "➖️"))
for f in MEMBER_FIELDS:
    LEAVES.append(Leaf("ChangeMember" + f[0], "member", f[9], field=f))
LEAVES.append(Leaf("InsertMemberAction", "member-action-insert", "➕️"))
LEAVES.append(Leaf("RemoveMemberAction", "member-action-remove", "➖️"))
for f in MEMBER_ACTION_FIELDS:
    LEAVES.append(Leaf("ChangeMemberAction" + f[0], "member-action", f[9], field=f))
LEAVES.append(Leaf("InsertConnection", "connection-insert", "➕️"))
LEAVES.append(Leaf("RemoveConnection", "connection-remove", "➖️"))
for f in CONNECTION_FIELDS:
    LEAVES.append(Leaf("ChangeConnection" + f[0], "connection", f[9], field=f))
LEAVES.append(Leaf("InsertConnectionAction", "connection-action-insert", "➕️"))
LEAVES.append(Leaf("RemoveConnectionAction", "connection-action-remove", "➖️"))
for f in CONNECTION_ACTION_FIELDS:
    LEAVES.append(Leaf("ChangeConnectionAction" + f[0], "connection-action", f[9], field=f))

for i, leaf in enumerate(LEAVES):
    leaf.tag = i
assert len({l.kind for l in LEAVES}) == len(LEAVES)
assert len({l.dir for l in LEAVES}) == len(LEAVES)
assert len({l.module for l in LEAVES}) == len(LEAVES)


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def display_name(kind: str) -> str:
    return " ".join(w.capitalize() if not w[0].isdigit() else w for w in kind.split("-"))


def leaf_json(leaf: Leaf) -> str:
    return json.dumps(
        {
            "schemaVersion": 1,
            "owner": f"{OWNER_PREFIX}/{leaf.dir}",
            "semanticKind": leaf.kind,
            "displayName": display_name(leaf.kind),
            "emoji": leaf.emoji,
            "aggregateVariant": leaf.variant,
            "payloadSchema": "🧬️schema/🔣️.json",
            "textOpcode": None,
            "binaryTag": leaf.tag,
            "invertibility": "explicit-mutation",
            "diffParticipation": "detect",
            "outcomeClasses": ["applied", "no-op", "rejected"],
            "composition": "atomic",
            "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
        },
        indent=2,
        ensure_ascii=False,
    ) + "\n"


ENUM_VALUES = {
    "ROLE": ["beam", "column", "floor", "bridge"],
    "SUPPORT": ["simplySupported", "cantilever", "continuousTwoSpan"],
}


def json_prop(jtype: str) -> dict:
    if jtype == "string":
        return {"type": "string"}
    if jtype == "number":
        return {"type": "number", "format": "double"}
    if jtype == "integer":
        return {"type": "integer", "minimum": 0}
    if jtype == "boolean":
        return {"type": "boolean"}
    if jtype.startswith("enum:"):
        return {"type": "string", "enum": ENUM_VALUES[jtype[5:]]}
    raise ValueError(jtype)


def schema_json(leaf: Leaf) -> str:
    props: dict[str, dict] = {}
    if leaf.scope == "annex":
        props["newAnnex"] = {"type": "string", "enum": ["En", "De"]}
    elif leaf.scope == "member-insert":
        props = {"index": {"type": "integer", "minimum": 0}, "member": {"type": "object"}}
    elif leaf.scope == "member-remove":
        props = {"index": {"type": "integer", "minimum": 0}}
    elif leaf.scope == "connection-insert":
        props = {"index": {"type": "integer", "minimum": 0}, "connection": {"type": "object"}}
    elif leaf.scope == "connection-remove":
        props = {"index": {"type": "integer", "minimum": 0}}
    elif leaf.scope == "member-action-insert":
        props = {"memberId": {"type": "string"}, "index": {"type": "integer", "minimum": 0}, "action": {"type": "object"}}
    elif leaf.scope == "member-action-remove":
        props = {"memberId": {"type": "string"}, "index": {"type": "integer", "minimum": 0}}
    elif leaf.scope == "connection-action-insert":
        props = {"connectionId": {"type": "string"}, "index": {"type": "integer", "minimum": 0}, "action": {"type": "object"}}
    elif leaf.scope == "connection-action-remove":
        props = {"connectionId": {"type": "string"}, "index": {"type": "integer", "minimum": 0}}
    elif leaf.scope == "member":
        props = {"memberId": {"type": "string"}, "newValue": json_prop(leaf.field[4])}
    elif leaf.scope == "member-action":
        props = {"memberId": {"type": "string"}, "actionId": {"type": "string"}, "newValue": json_prop(leaf.field[4])}
    elif leaf.scope == "connection":
        props = {"connectionId": {"type": "string"}, "newValue": json_prop(leaf.field[4])}
    elif leaf.scope == "connection-action":
        props = {"connectionId": {"type": "string"}, "actionId": {"type": "string"}, "newValue": json_prop(leaf.field[4])}
    else:
        raise ValueError(leaf.scope)
    return json.dumps(
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": SCHEMA_ID.format(kind=leaf.kind),
            "title": leaf.variant,
            "type": "object",
            "additionalProperties": False,
            "required": list(props.keys()),
            "properties": props,
        },
        indent=2,
        ensure_ascii=False,
    ) + "\n"


HEADER = """#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
"""

IMPL_TAIL = """    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<<En1995Mutation as protocol::Mutation<En1995Snapshot>>::Diff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> { super::inverse::inverse(self, base) }
"""


def semantics(leaf: Leaf, entity: str) -> str:
    return f'    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "{leaf.verb}", entity: "{entity}", kind: "{leaf.kind}", record: "{leaf.record}" }};\n'


def rust_value_type(rtype: str) -> str:
    return {"MemberRole": "crate::MemberRole", "SupportType": "crate::SupportType"}.get(rtype, rtype)


def scalar_leaf_rs(leaf: Leaf) -> str:
    f = leaf.field
    rtype = rust_value_type(f[3])
    en, de = f[5], f[6]
    if leaf.scope == "member":
        ident, entity, noun_en, noun_de = "member_id", f"member-{f[2]}", "member", "Bauteil"
        fields = f"pub member_id: String, pub new_value: {rtype}"
        label = f'protocol::LocalizedLabel::native(&format!("Change {en} of {noun_en} {{}}", self.member_id), &format!("{de} von {noun_de} {{}} ändern", self.member_id))'
        target = "vec![self.member_id.clone()]"
    elif leaf.scope == "connection":
        ident, entity, noun_en, noun_de = "connection_id", f"connection-{f[2]}", "connection", "Verbindung"
        fields = f"pub connection_id: String, pub new_value: {rtype}"
        label = f'protocol::LocalizedLabel::native(&format!("Change {en} of {noun_en} {{}}", self.connection_id), &format!("{de} von {noun_de} {{}} ändern", self.connection_id))'
        target = "vec![self.connection_id.clone()]"
    elif leaf.scope == "member-action":
        entity = f"member-action-{f[2]}"
        fields = f"pub member_id: String, pub action_id: String, pub new_value: {rtype}"
        label = f'protocol::LocalizedLabel::native(&format!("Change {en} of action {{}} on member {{}}", self.action_id, self.member_id), &format!("{de} der Einwirkung {{}} an Bauteil {{}} ändern", self.action_id, self.member_id))'
        target = "vec![self.member_id.clone(), self.action_id.clone()]"
    elif leaf.scope == "connection-action":
        entity = f"connection-action-{f[2]}"
        fields = f"pub connection_id: String, pub action_id: String, pub new_value: {rtype}"
        label = f'protocol::LocalizedLabel::native(&format!("Change {en} of action {{}} on connection {{}}", self.action_id, self.connection_id), &format!("{de} der Einwirkung {{}} an Verbindung {{}} ändern", self.action_id, self.connection_id))'
        target = "vec![self.connection_id.clone(), self.action_id.clone()]"
    else:
        raise ValueError(leaf.scope)
    return (
        f"//! {leaf.emoji} `{leaf.kind}` — changes `{f[2]}` ({en}) on one addressed {entity.split('-')[0]}.\n"
        "use crate::{En1995Mutation, En1995Snapshot};\n"
        + HEADER
        + f"pub struct {leaf.variant} {{ {fields} }}\n"
        + f"impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for {leaf.variant} {{\n"
        + semantics(leaf, entity)
        + IMPL_TAIL
        + f"    fn label(&self) -> protocol::LocalizedLabel {{ {label} }}\n"
        + f"    fn target(&self) -> Vec<String> {{ {target} }}\n"
        + "}\n"
    )


def collection_leaf_rs(leaf: Leaf) -> str:
    s = leaf.scope
    if s == "member-insert":
        return (
            f"//! {leaf.emoji} `insert-member` — inserts one timber member at an index (clamped to the list end).\n"
            "use crate::{En1995Mutation, En1995Snapshot, TimberMember};\n" + HEADER
            + "pub struct InsertMember { pub index: usize, pub member: TimberMember }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for InsertMember {\n"
            + semantics(leaf, "member") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert member {} at #{}", self.member.id, self.index), &format!("Bauteil {} an #{} einfügen", self.member.id, self.index)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.member.id.clone()] }\n}\n"
        )
    if s == "member-remove":
        return (
            f"//! {leaf.emoji} `remove-member` — removes the timber member at an index.\n"
            "use crate::{En1995Mutation, En1995Snapshot};\n" + HEADER
            + "pub struct RemoveMember { pub index: usize }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for RemoveMember {\n"
            + semantics(leaf, "member") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Remove member #{}", self.index), &format!("Bauteil #{} entfernen", self.index)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }\n}\n"
        )
    if s == "connection-insert":
        return (
            f"//! {leaf.emoji} `insert-connection` — inserts one timber connection at an index (clamped to the list end).\n"
            "use crate::{En1995Mutation, En1995Snapshot, TimberConnection};\n" + HEADER
            + "pub struct InsertConnection { pub index: usize, pub connection: TimberConnection }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for InsertConnection {\n"
            + semantics(leaf, "connection") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert connection {} at #{}", self.connection.id, self.index), &format!("Verbindung {} an #{} einfügen", self.connection.id, self.index)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.connection.id.clone()] }\n}\n"
        )
    if s == "connection-remove":
        return (
            f"//! {leaf.emoji} `remove-connection` — removes the timber connection at an index.\n"
            "use crate::{En1995Mutation, En1995Snapshot};\n" + HEADER
            + "pub struct RemoveConnection { pub index: usize }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for RemoveConnection {\n"
            + semantics(leaf, "connection") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Remove connection #{}", self.index), &format!("Verbindung #{} entfernen", self.index)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.index.to_string()] }\n}\n"
        )
    if s == "member-action-insert":
        return (
            f"//! {leaf.emoji} `insert-member-action` — inserts one characteristic action into a member's action table.\n"
            "use crate::{CharacteristicAction, En1995Mutation, En1995Snapshot};\n" + HEADER
            + "pub struct InsertMemberAction { pub member_id: String, pub index: usize, pub action: CharacteristicAction }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for InsertMemberAction {\n"
            + semantics(leaf, "member-action") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert action {} into member {} at #{}", self.action.id, self.member_id, self.index), &format!("Einwirkung {} in Bauteil {} an #{} einfügen", self.action.id, self.member_id, self.index)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.member_id.clone(), self.action.id.clone()] }\n}\n"
        )
    if s == "member-action-remove":
        return (
            f"//! {leaf.emoji} `remove-member-action` — removes the characteristic action at an index from a member's action table.\n"
            "use crate::{En1995Mutation, En1995Snapshot};\n" + HEADER
            + "pub struct RemoveMemberAction { pub member_id: String, pub index: usize }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for RemoveMemberAction {\n"
            + semantics(leaf, "member-action") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Remove action #{} from member {}", self.index, self.member_id), &format!("Einwirkung #{} aus Bauteil {} entfernen", self.index, self.member_id)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.member_id.clone(), self.index.to_string()] }\n}\n"
        )
    if s == "connection-action-insert":
        return (
            f"//! {leaf.emoji} `insert-connection-action` — inserts one characteristic action into a connection's action table.\n"
            "use crate::{ConnectionAction, En1995Mutation, En1995Snapshot};\n" + HEADER
            + "pub struct InsertConnectionAction { pub connection_id: String, pub index: usize, pub action: ConnectionAction }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for InsertConnectionAction {\n"
            + semantics(leaf, "connection-action") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Insert action {} into connection {} at #{}", self.action.id, self.connection_id, self.index), &format!("Einwirkung {} in Verbindung {} an #{} einfügen", self.action.id, self.connection_id, self.index)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.connection_id.clone(), self.action.id.clone()] }\n}\n"
        )
    if s == "connection-action-remove":
        return (
            f"//! {leaf.emoji} `remove-connection-action` — removes the characteristic action at an index from a connection's action table.\n"
            "use crate::{En1995Mutation, En1995Snapshot};\n" + HEADER
            + "pub struct RemoveConnectionAction { pub connection_id: String, pub index: usize }\n"
            + "impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for RemoveConnectionAction {\n"
            + semantics(leaf, "connection-action") + IMPL_TAIL
            + '    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Remove action #{} from connection {}", self.index, self.connection_id), &format!("Einwirkung #{} aus Verbindung {} entfernen", self.index, self.connection_id)) }\n'
            + "    fn target(&self) -> Vec<String> { vec![self.connection_id.clone(), self.index.to_string()] }\n}\n"
        )
    raise ValueError(s)


def clone_expr(rtype: str) -> str:
    return ".clone()" if rtype == "String" else ""


MEMBER_LOOKUP = """    let Some(idx) = base.members.iter().position(|item| item.id == payload.member_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown member id.", vec![payload.member_id.clone()]);
    };
"""
CONNECTION_LOOKUP = """    let Some(idx) = base.connections.iter().position(|item| item.id == payload.connection_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown connection id.", vec![payload.connection_id.clone()]);
    };
"""
MEMBER_ACTION_LOOKUP = MEMBER_LOOKUP + """    let Some(action_idx) = base.members[idx].actions.iter().position(|action| action.id == payload.action_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown member action id.", vec![payload.member_id.clone(), payload.action_id.clone()]);
    };
"""
CONNECTION_ACTION_LOOKUP = CONNECTION_LOOKUP + """    let Some(action_idx) = base.connections[idx].actions.iter().position(|action| action.id == payload.action_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown connection action id.", vec![payload.connection_id.clone(), payload.action_id.clone()]);
    };
"""
MEMBER_OUT = "    protocol::MutationOutcome::new(En1995Diff { members: Some(En1995MemberList { values: members }), ..Default::default() })\n"
CONNECTION_OUT = "    protocol::MutationOutcome::new(En1995Diff { connections: Some(En1995ConnectionList { values: connections }), ..Default::default() })\n"


def diff_rs(leaf: Leaf) -> str:
    s = leaf.scope
    head_m = f"use super::{leaf.variant};\nuse crate::diff::En1995MemberList;\nuse crate::{{En1995Diff, En1995Snapshot}};\n"
    head_c = f"use super::{leaf.variant};\nuse crate::diff::En1995ConnectionList;\nuse crate::{{En1995Diff, En1995Snapshot}};\n"
    sig = f"pub fn diff(payload: &{leaf.variant}, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {{\n"
    if s == "member":
        f = leaf.field
        return head_m + sig + MEMBER_LOOKUP + "    let mut members = base.members.clone();\n" + f"    members[idx].{f[1]} = payload.new_value{clone_expr(f[3])};\n" + MEMBER_OUT + "}\n"
    if s == "connection":
        f = leaf.field
        return head_c + sig + CONNECTION_LOOKUP + "    let mut connections = base.connections.clone();\n" + f"    connections[idx].{f[1]} = payload.new_value{clone_expr(f[3])};\n" + CONNECTION_OUT + "}\n"
    if s == "member-action":
        f = leaf.field
        return head_m + sig + MEMBER_ACTION_LOOKUP + "    let mut members = base.members.clone();\n" + f"    members[idx].actions[action_idx].{f[1]} = payload.new_value{clone_expr(f[3])};\n" + MEMBER_OUT + "}\n"
    if s == "connection-action":
        f = leaf.field
        return head_c + sig + CONNECTION_ACTION_LOOKUP + "    let mut connections = base.connections.clone();\n" + f"    connections[idx].actions[action_idx].{f[1]} = payload.new_value{clone_expr(f[3])};\n" + CONNECTION_OUT + "}\n"
    if s == "member-insert":
        return head_m + sig + "    let mut members = base.members.clone();\n    let at = payload.index.min(members.len());\n    members.insert(at, payload.member.clone());\n" + MEMBER_OUT + "}\n"
    if s == "member-remove":
        return head_m + sig + '    if payload.index >= base.members.len() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Member index {} out of range.", payload.index), Vec::<String>::new());\n    }\n    let mut members = base.members.clone();\n    members.remove(payload.index);\n' + MEMBER_OUT + "}\n"
    if s == "connection-insert":
        return head_c + sig + "    let mut connections = base.connections.clone();\n    let at = payload.index.min(connections.len());\n    connections.insert(at, payload.connection.clone());\n" + CONNECTION_OUT + "}\n"
    if s == "connection-remove":
        return head_c + sig + '    if payload.index >= base.connections.len() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Connection index {} out of range.", payload.index), Vec::<String>::new());\n    }\n    let mut connections = base.connections.clone();\n    connections.remove(payload.index);\n' + CONNECTION_OUT + "}\n"
    if s == "member-action-insert":
        return head_m + sig + MEMBER_LOOKUP + "    let mut members = base.members.clone();\n    let at = payload.index.min(members[idx].actions.len());\n    members[idx].actions.insert(at, payload.action.clone());\n" + MEMBER_OUT + "}\n"
    if s == "member-action-remove":
        return head_m + sig + MEMBER_LOOKUP + '    if payload.index >= base.members[idx].actions.len() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Member action index {} out of range.", payload.index), vec![payload.member_id.clone()]);\n    }\n    let mut members = base.members.clone();\n    members[idx].actions.remove(payload.index);\n' + MEMBER_OUT + "}\n"
    if s == "connection-action-insert":
        return head_c + sig + CONNECTION_LOOKUP + "    let mut connections = base.connections.clone();\n    let at = payload.index.min(connections[idx].actions.len());\n    connections[idx].actions.insert(at, payload.action.clone());\n" + CONNECTION_OUT + "}\n"
    if s == "connection-action-remove":
        return head_c + sig + CONNECTION_LOOKUP + '    if payload.index >= base.connections[idx].actions.len() {\n        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Connection action index {} out of range.", payload.index), vec![payload.connection_id.clone()]);\n    }\n    let mut connections = base.connections.clone();\n    connections[idx].actions.remove(payload.index);\n' + CONNECTION_OUT + "}\n"
    raise ValueError(s)


def inverse_rs(leaf: Leaf) -> str:
    s = leaf.scope
    v = leaf.variant
    head = f"use super::{v};\nuse crate::mutations::En1995Mutation;\nuse crate::En1995Snapshot;\n"
    sig = f"pub fn inverse(payload: &{v}, base: &En1995Snapshot) -> Vec<En1995Mutation> {{\n"
    if s == "member":
        f = leaf.field
        return head + sig + "    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };\n" + f"    vec![En1995Mutation::{v}({v} {{ member_id: payload.member_id.clone(), new_value: item.{f[1]}{clone_expr(f[3])} }})]\n}}\n"
    if s == "connection":
        f = leaf.field
        return head + sig + "    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };\n" + f"    vec![En1995Mutation::{v}({v} {{ connection_id: payload.connection_id.clone(), new_value: item.{f[1]}{clone_expr(f[3])} }})]\n}}\n"
    if s == "member-action":
        f = leaf.field
        return head + sig + "    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return Vec::new(); };\n" + f"    vec![En1995Mutation::{v}({v} {{ member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_value: item.{f[1]}{clone_expr(f[3])} }})]\n}}\n"
    if s == "connection-action":
        f = leaf.field
        return head + sig + "    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id).and_then(|item| item.actions.iter().find(|action| action.id == payload.action_id)) else { return Vec::new(); };\n" + f"    vec![En1995Mutation::{v}({v} {{ connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_value: item.{f[1]}{clone_expr(f[3])} }})]\n}}\n"
    if s == "member-insert":
        return f"use super::InsertMember;\nuse crate::mutations::{{remove_member, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    let at = payload.index.min(base.members.len());\n    vec![En1995Mutation::RemoveMember(remove_member::RemoveMember { index: at })]\n}\n"
    if s == "member-remove":
        return f"use super::RemoveMember;\nuse crate::mutations::{{insert_member, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    if payload.index >= base.members.len() { return Vec::new(); }\n    vec![En1995Mutation::InsertMember(insert_member::InsertMember { index: payload.index, member: base.members[payload.index].clone() })]\n}\n"
    if s == "connection-insert":
        return f"use super::InsertConnection;\nuse crate::mutations::{{remove_connection, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    let at = payload.index.min(base.connections.len());\n    vec![En1995Mutation::RemoveConnection(remove_connection::RemoveConnection { index: at })]\n}\n"
    if s == "connection-remove":
        return f"use super::RemoveConnection;\nuse crate::mutations::{{insert_connection, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    if payload.index >= base.connections.len() { return Vec::new(); }\n    vec![En1995Mutation::InsertConnection(insert_connection::InsertConnection { index: payload.index, connection: base.connections[payload.index].clone() })]\n}\n"
    if s == "member-action-insert":
        return f"use super::InsertMemberAction;\nuse crate::mutations::{{remove_member_action, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };\n    let at = payload.index.min(item.actions.len());\n    vec![En1995Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { member_id: payload.member_id.clone(), index: at })]\n}\n"
    if s == "member-action-remove":
        return f"use super::RemoveMemberAction;\nuse crate::mutations::{{insert_member_action, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    let Some(item) = base.members.iter().find(|item| item.id == payload.member_id) else { return Vec::new(); };\n    if payload.index >= item.actions.len() { return Vec::new(); }\n    vec![En1995Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { member_id: payload.member_id.clone(), index: payload.index, action: item.actions[payload.index].clone() })]\n}\n"
    if s == "connection-action-insert":
        return f"use super::InsertConnectionAction;\nuse crate::mutations::{{remove_connection_action, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };\n    let at = payload.index.min(item.actions.len());\n    vec![En1995Mutation::RemoveConnectionAction(remove_connection_action::RemoveConnectionAction { connection_id: payload.connection_id.clone(), index: at })]\n}\n"
    if s == "connection-action-remove":
        return f"use super::RemoveConnectionAction;\nuse crate::mutations::{{insert_connection_action, En1995Mutation}};\nuse crate::En1995Snapshot;\n" + sig + "    let Some(item) = base.connections.iter().find(|item| item.id == payload.connection_id) else { return Vec::new(); };\n    if payload.index >= item.actions.len() { return Vec::new(); }\n    vec![En1995Mutation::InsertConnectionAction(insert_connection_action::InsertConnectionAction { connection_id: payload.connection_id.clone(), index: payload.index, action: item.actions[payload.index].clone() })]\n}\n"
    raise ValueError(s)


# ---------------------------------------------------------------------------
# Payload constructors shared by tests / demo cases
# ---------------------------------------------------------------------------
def payload_expr(leaf: Leaf, qualified: bool) -> str:
    v = leaf.variant
    path = f"{leaf.module}::{v}" if qualified else v
    s = leaf.scope
    if s == "annex":
        return f"{path} {{ new_annex: crate::document::AnnexChoice::En }}"
    if s == "member-insert":
        return f"{path} {{ index: 99, member: crate::TimberMember {{ id: \"beam-B9\".into(), ..base.members[0].clone() }} }}"
    if s == "member-remove":
        return f"{path} {{ index: 0 }}"
    if s == "connection-insert":
        return f"{path} {{ index: 99, connection: crate::TimberConnection {{ id: \"conn-C9\".into(), ..base.connections[0].clone() }} }}"
    if s == "connection-remove":
        return f"{path} {{ index: 0 }}"
    if s == "member-action-insert":
        return f"{path} {{ member_id: base.members[0].id.clone(), index: 99, action: crate::CharacteristicAction {{ id: \"w\".into(), ..base.members[0].actions[0].clone() }} }}"
    if s == "member-action-remove":
        return f"{path} {{ member_id: base.members[0].id.clone(), index: 0 }}"
    if s == "connection-action-insert":
        return f"{path} {{ connection_id: base.connections[0].id.clone(), index: 99, action: crate::ConnectionAction {{ id: \"w\".into(), ..base.connections[0].actions[0].clone() }} }}"
    if s == "connection-action-remove":
        return f"{path} {{ connection_id: base.connections[0].id.clone(), index: 0 }}"
    val = leaf.field[10]
    if s == "member":
        return f"{path} {{ member_id: base.members[0].id.clone(), new_value: {val} }}"
    if s == "connection":
        return f"{path} {{ connection_id: base.connections[0].id.clone(), new_value: {val} }}"
    if s == "member-action":
        return f"{path} {{ member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: {val} }}"
    if s == "connection-action":
        return f"{path} {{ connection_id: base.connections[0].id.clone(), action_id: base.connections[0].actions[0].id.clone(), new_value: {val} }}"
    raise ValueError(s)


def scenario(leaf: Leaf) -> tuple[str, str]:
    """Returns (scenario id, directory name)."""
    s = leaf.scope
    if s == "annex":
        return ("switches-from-the-german-na-to-the-recommended-en-annex", "🌍️switches-from-the-german-na-to-the-recommended-en-annex")
    if s == "member-insert":
        return ("inserts-a-member-at-end", "➕️inserts-a-member-at-end")
    if s == "member-remove":
        return ("removes-the-first-member", "➖️removes-the-first-member")
    if s == "connection-insert":
        return ("inserts-a-connection-at-end", "➕️inserts-a-connection-at-end")
    if s == "connection-remove":
        return ("removes-the-first-connection", "➖️removes-the-first-connection")
    if s == "member-action-insert":
        return ("inserts-an-action-at-end-of-member", "➕️inserts-an-action-at-end-of-member")
    if s == "member-action-remove":
        return ("removes-the-first-action-of-member", "➖️removes-the-first-action-of-member")
    if s == "connection-action-insert":
        return ("inserts-an-action-at-end-of-connection", "➕️inserts-an-action-at-end-of-connection")
    if s == "connection-action-remove":
        return ("removes-the-first-action-of-connection", "➖️removes-the-first-action-of-connection")
    camel = leaf.field[2]
    return (f"sets-{camel}", f"✏️sets-{camel}")


def assert_expr(rtype: str, lhs: str, rhs: str) -> str:
    if rtype == "f64":
        return f"    assert!(({lhs} - {rhs}).abs() < 1e-12);\n"
    if rtype == "String":
        return f"    assert_eq!({lhs}, {rhs});\n"
    return f"    assert_eq!({lhs}, {rhs});\n"


def test_rs(leaf: Leaf) -> str:
    v = leaf.variant
    sid, _ = scenario(leaf)
    fn = to_kebab(sid.replace("-", "_")).replace("-", "_")
    s = leaf.scope
    lines = [
        f"//! 🧪️ `{leaf.kind}` — `{sid}`: applies, checks the written field, then replays the inverse back to BASE.\n",
        f"use crate::mutations::{leaf.module}::{v};\n",
        "use crate::mutations::En1995Mutation;\n",
        "use crate::En1995Snapshot;\n",
        "#[test]\n",
        f"fn {fn}() {{\n",
        "    let base = En1995Snapshot::compliant_building_beam();\n",
        f"    let payload = {payload_expr(leaf, False)};\n",
        f"    let outcome = <{v} as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::diff(&payload, &base);\n",
        '    assert!(outcome.worst_level().is_none(), "{:?}", outcome.messages());\n',
        '    let next = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");\n',
    ]
    if s == "member-insert":
        lines.append("    assert_eq!(next.members.len(), base.members.len() + 1);\n    assert_eq!(next.members.last().unwrap().id, \"beam-B9\");\n")
    elif s == "member-remove":
        lines.append("    assert_eq!(next.members.len(), base.members.len() - 1);\n")
    elif s == "connection-insert":
        lines.append("    assert_eq!(next.connections.len(), base.connections.len() + 1);\n    assert_eq!(next.connections.last().unwrap().id, \"conn-C9\");\n")
    elif s == "connection-remove":
        lines.append("    assert_eq!(next.connections.len(), base.connections.len() - 1);\n")
    elif s == "member-action-insert":
        lines.append("    assert_eq!(next.members[0].actions.len(), base.members[0].actions.len() + 1);\n    assert_eq!(next.members[0].actions.last().unwrap().id, \"w\");\n")
    elif s == "member-action-remove":
        lines.append("    assert_eq!(next.members[0].actions.len(), base.members[0].actions.len() - 1);\n")
    elif s == "connection-action-insert":
        lines.append("    assert_eq!(next.connections[0].actions.len(), base.connections[0].actions.len() + 1);\n    assert_eq!(next.connections[0].actions.last().unwrap().id, \"w\");\n")
    elif s == "connection-action-remove":
        lines.append("    assert_eq!(next.connections[0].actions.len(), base.connections[0].actions.len() - 1);\n")
    else:
        f = leaf.field
        rhs = "payload.new_value" + (".clone()" if f[3] == "String" else "")
        lhs = {"member": f"next.members[0].{f[1]}", "connection": f"next.connections[0].{f[1]}", "member-action": f"next.members[0].actions[0].{f[1]}", "connection-action": f"next.connections[0].actions[0].{f[1]}"}[s]
        lines.append(assert_expr(f[3], lhs, rhs))
        lines.append(f"    assert_ne!(next, base);\n")
    lines += [
        f"    let inverse = <{v} as protocol::MutationKind<En1995Snapshot, En1995Mutation>>::inverse(&payload, &base);\n",
        "    assert_eq!(inverse.len(), 1);\n",
        "    let mut restored = next.clone();\n",
        "    for step in &inverse {\n",
        "        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &restored);\n",
        '        restored = protocol::MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");\n',
        "    }\n",
        "    assert_eq!(restored, base);\n",
        f"    store::os_store::test_support::assert_op_line_round_trip(&En1995Mutation::{v}(payload));\n",
        "}\n",
    ]
    return "".join(lines)


# ---------------------------------------------------------------------------
# Aggregate 🦀️.rs
# ---------------------------------------------------------------------------
def scalar_change_expr(leaf: Leaf, owner: str, ids: str) -> str:
    f = leaf.field
    v = leaf.variant
    cmp = f"(b.{f[1]} - t.{f[1]}).abs() > f64::EPSILON" if f[3] == "f64" else f"b.{f[1]} != t.{f[1]}"
    clone = ".clone()" if f[3] == "String" else ""
    return f"                    if {cmp} {{\n                        out.push(En1995Mutation::{v}({leaf.module}::{v} {{ {ids}, new_value: t.{f[1]}{clone} }}));\n                    }}\n"


def aggregate_rs() -> str:
    uses = "".join(f"use super::{l.module};\n" for l in LEAVES)
    variants = "".join(f"    {l.variant}({l.module}::{l.variant}),\n" for l in LEAVES)
    kinds = "".join(f'    "{l.kind}",\n' for l in LEAVES)
    member_scalars = "".join(scalar_change_expr(l, "member", "member_id: t.id.clone()") for l in LEAVES if l.scope == "member")
    member_action_scalars = "".join(scalar_change_expr(l, "member-action", "member_id: member_id.clone(), action_id: t.id.clone()").replace("                    ", "                            ") for l in LEAVES if l.scope == "member-action")
    connection_scalars = "".join(scalar_change_expr(l, "connection", "connection_id: t.id.clone()") for l in LEAVES if l.scope == "connection")
    connection_action_scalars = "".join(scalar_change_expr(l, "connection-action", "connection_id: connection_id.clone(), action_id: t.id.clone()").replace("                    ", "                            ") for l in LEAVES if l.scope == "connection-action")
    return f'''//! 🧬️ En1995 artifact — document mutation dispatch for the hierarchical timber subject
//! (members ⊃ characteristic actions, connections ⊃ connection actions).

use crate::{{En1995Diff, En1995Snapshot}};

//#region 🔖️Mutations
{uses}
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1995Snapshot, diff = En1995Diff, schema = "s.norm.en1995")]
pub enum En1995Mutation {{
{variants}}}

/// 🏷️ Every kind in `#[derive(dsl::Mutations)]` declaration order — pinned by `kinds_match_the_enum_and_the_catalog`.
pub const KINDS: &[&str] = &[
{kinds}];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1995Mutation {{
    /// 🔀 Derives the ordered mutation list carrying `base` to `target` — removals (reverse index order),
    /// insertions, then one `change-*` per differing scalar, recursing into each item's action table.
    pub fn from_snapshot(base: &En1995Snapshot, target: &En1995Snapshot) -> Vec<Self> {{
        let mut out = Vec::new();
        if base.annex != target.annex {{
            out.push(En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex {{ new_annex: target.annex }}));
        }}
        if base.members != target.members {{
            let base_ids: std::collections::BTreeSet<_> = base.members.iter().map(|m| m.id.clone()).collect();
            let target_ids: std::collections::BTreeSet<_> = target.members.iter().map(|m| m.id.clone()).collect();
            for (index, m) in base.members.iter().enumerate().rev() {{
                if !target_ids.contains(&m.id) {{
                    out.push(En1995Mutation::RemoveMember(remove_member::RemoveMember {{ index }}));
                }}
            }}
            for (index, m) in target.members.iter().enumerate() {{
                if !base_ids.contains(&m.id) {{
                    out.push(En1995Mutation::InsertMember(insert_member::InsertMember {{ index, member: m.clone() }}));
                }}
            }}
            for t in &target.members {{
                if let Some(b) = base.members.iter().find(|m| m.id == t.id) {{
{member_scalars}                    if b.actions != t.actions {{
                        let member_id = t.id.clone();
                        let base_ids: std::collections::BTreeSet<_> = b.actions.iter().map(|a| a.id.clone()).collect();
                        let target_ids: std::collections::BTreeSet<_> = t.actions.iter().map(|a| a.id.clone()).collect();
                        for (index, a) in b.actions.iter().enumerate().rev() {{
                            if !target_ids.contains(&a.id) {{
                                out.push(En1995Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction {{ member_id: member_id.clone(), index }}));
                            }}
                        }}
                        for (index, a) in t.actions.iter().enumerate() {{
                            if !base_ids.contains(&a.id) {{
                                out.push(En1995Mutation::InsertMemberAction(insert_member_action::InsertMemberAction {{ member_id: member_id.clone(), index, action: a.clone() }}));
                            }}
                        }}
                        for t in &t.actions {{
                            if let Some(b) = b.actions.iter().find(|a| a.id == t.id) {{
{member_action_scalars}                            }}
                        }}
                    }}
                }}
            }}
        }}
        if base.connections != target.connections {{
            let base_ids: std::collections::BTreeSet<_> = base.connections.iter().map(|c| c.id.clone()).collect();
            let target_ids: std::collections::BTreeSet<_> = target.connections.iter().map(|c| c.id.clone()).collect();
            for (index, c) in base.connections.iter().enumerate().rev() {{
                if !target_ids.contains(&c.id) {{
                    out.push(En1995Mutation::RemoveConnection(remove_connection::RemoveConnection {{ index }}));
                }}
            }}
            for (index, c) in target.connections.iter().enumerate() {{
                if !base_ids.contains(&c.id) {{
                    out.push(En1995Mutation::InsertConnection(insert_connection::InsertConnection {{ index, connection: c.clone() }}));
                }}
            }}
            for t in &target.connections {{
                if let Some(b) = base.connections.iter().find(|c| c.id == t.id) {{
{connection_scalars}                    if b.actions != t.actions {{
                        let connection_id = t.id.clone();
                        let base_ids: std::collections::BTreeSet<_> = b.actions.iter().map(|a| a.id.clone()).collect();
                        let target_ids: std::collections::BTreeSet<_> = t.actions.iter().map(|a| a.id.clone()).collect();
                        for (index, a) in b.actions.iter().enumerate().rev() {{
                            if !target_ids.contains(&a.id) {{
                                out.push(En1995Mutation::RemoveConnectionAction(remove_connection_action::RemoveConnectionAction {{ connection_id: connection_id.clone(), index }}));
                            }}
                        }}
                        for (index, a) in t.actions.iter().enumerate() {{
                            if !base_ids.contains(&a.id) {{
                                out.push(En1995Mutation::InsertConnectionAction(insert_connection_action::InsertConnectionAction {{ connection_id: connection_id.clone(), index, action: a.clone() }}));
                            }}
                        }}
                        for t in &t.actions {{
                            if let Some(b) = b.actions.iter().find(|a| a.id == t.id) {{
{connection_action_scalars}                            }}
                        }}
                    }}
                }}
            }}
        }}
        out
    }}
}}
//#endregion 🔖️FromSnapshot

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
'''


def every_mutation_fn(snapshot: str = "En1995Snapshot") -> str:
    body = "".join(f"        En1995Mutation::{l.variant}({payload_expr(l, True)}),\n" for l in LEAVES)
    return f"""fn every_mutation() -> Vec<En1995Mutation> {{
    let base = {snapshot}::compliant_building_beam();
    vec![
{body}    ]
}}
"""


def unit_test_rs() -> str:
    return f"""use super::*;
use protocol::SemanticMutation;

{every_mutation_fn()}
/// 🏷️ Every declared variant labels itself and the closed vocabulary is exactly [`KINDS`].
#[test]
fn every_declared_kind_has_a_label_and_matches_kinds_len() {{
    for mutation in every_mutation() {{
        let _ = <En1995Mutation as SemanticMutation<En1995Snapshot>>::label(&mutation);
    }}
    assert_eq!(every_mutation().len(), KINDS.len());
}}

/// 🔀 `from_snapshot(base, apply(m, base))` reproduces the applied state for every variant, so the
/// editor's `set-field` / `insert-item` / `remove-item` / `set-snapshot` commands can reach each leaf.
#[test]
fn from_snapshot_reaches_every_variant() {{
    let base = En1995Snapshot::compliant_building_beam();
    for mutation in every_mutation() {{
        let outcome = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation, &base);
        assert!(outcome.worst_level().is_none(), "{{mutation:?}}: {{:?}}", outcome.messages());
        let target = protocol::MutationDiff::apply(outcome.diff(), &base).expect("applies");
        let derived = En1995Mutation::from_snapshot(&base, &target);
        assert!(!derived.is_empty() || target == base, "{{mutation:?}} left no trace for from_snapshot");
        let mut replayed = base.clone();
        for step in &derived {{
            let step_outcome = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &replayed);
            assert!(step_outcome.worst_level().is_none(), "{{step:?}}: {{:?}}", step_outcome.messages());
            replayed = protocol::MutationDiff::apply(step_outcome.diff(), &replayed).expect("derived step applies");
        }}
        assert_eq!(replayed, target, "{{mutation:?}}: from_snapshot did not reproduce the applied state via {{derived:?}}");
    }}
}}

/// 🔀 A whole-example switch (`set-snapshot`) is expressible as a mutation list.
#[test]
fn from_snapshot_carries_between_examples() {{
    let pairs = [
        (En1995Snapshot::compliant_building_beam(), En1995Snapshot::noncompliant_building()),
        (En1995Snapshot::noncompliant_building(), En1995Snapshot::compliant_bridge()),
        (En1995Snapshot::compliant_bridge(), En1995Snapshot::noncompliant_bridge()),
        (En1995Snapshot::noncompliant_bridge(), En1995Snapshot::compliant_building_beam()),
    ];
    for (base, target) in pairs {{
        let mut replayed = base.clone();
        for step in En1995Mutation::from_snapshot(&base, &target) {{
            let outcome = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&step, &replayed);
            assert!(outcome.worst_level().is_none(), "{{step:?}}: {{:?}}", outcome.messages());
            replayed = protocol::MutationDiff::apply(outcome.diff(), &replayed).expect("step applies");
        }}
        assert_eq!(replayed, target);
    }}
}}
"""


def text_unit_test_rs() -> str:
    return f"""use super::En1995Mutation;
use crate::mutations::*;

/// ⚖️ Every variant — full-coverage `OpText` round trip over the closed vocabulary.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {{
    let cases = every_mutation();
    assert_eq!(cases.len(), KINDS.len());
    for mutation in cases {{
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }}
}}

{every_mutation_fn("crate::En1995Snapshot")}"""


DEMO_KINDS = [
    "change-annex", "insert-member", "remove-member", "change-member-role", "change-member-support", "change-member-h", "change-member-b",
    "change-member-strength-class", "change-member-bridge-crowd", "insert-member-action", "remove-member-action",
    "change-member-action-load-duration", "change-member-action-q-line", "insert-connection", "remove-connection",
    "change-connection-number", "change-connection-steel-plate", "change-connection-fuk", "insert-connection-action",
    "remove-connection-action", "change-connection-action-fk",
]


def text_rs() -> str:
    demo = "".join(f"            En1995Mutation::{l.variant}(crate::mutations::{payload_expr(l, True)}),\n" for l in LEAVES if l.kind in DEMO_KINDS)
    return f"""//! ⚡️ En1995 mutations — OpText/OpBinary via JSON tokens (hierarchical timber subject).

pub use crate::artifact_schema::mutations::En1995Mutation;

use protocol::{{OpBinary, OpText}};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl OpText for En1995Mutation {{
    fn print_op(&self) -> String {{
        pack::json::to_json_string(self)
    }}
    fn parse_op(line: &str) -> Result<Self, store::TextError> {{
        pack::json::from_json_str(line).map_err(|e| store::TextError::new(e.to_string(), store::TextSpan::at(1, 1)))
    }}
}}

impl OpBinary for En1995Mutation {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        Ok(<Self as OpText>::print_op(self).into_bytes())
    }}
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        let text = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed {{ what: "utf8", offset: 0, detail: e.to_string() }})?;
        <Self as OpText>::parse_op(text).map_err(|e| protocol::ProtocolError::Malformed {{ what: "json", offset: 0, detail: e.to_string() }})
    }}
}}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
mod unit_tests {{
    use super::*;

    /// 🎬️ One representative per addressing shape — root enum, member, member action, connection, connection action.
    fn demo_mutation_cases() -> Vec<En1995Mutation> {{
        let base = crate::En1995Snapshot::compliant_building_beam();
        vec![
{demo}        ]
    }}

    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {{
        for mutation in demo_mutation_cases() {{
            let printed = <En1995Mutation as OpText>::print_op(&mutation);
            assert!(!printed.contains('\\n'), "print_op must be one line");
            let parsed = <En1995Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed: {{e}}"));
            assert_eq!(parsed, mutation);
            let encoded = <En1995Mutation as OpBinary>::encode_op(&mutation).unwrap();
            let decoded = <En1995Mutation as OpBinary>::decode_op(&encoded).unwrap();
            assert_eq!(decoded, mutation);
        }}
    }}
}}
"""


def fixture_index_rs() -> str:
    out = "//! 🧪️ Mounts every leaf's named scenario test so the whole vocabulary runs under `cargo test`.\n"
    for l in LEAVES:
        sid, sdir = scenario(l)
        out += f'#[path = "../../{l.dir}/🧪️tests/{sdir}/🦀️.rs"]\nmod tests_{l.module}_{to_kebab(sid.replace("-", "_")).replace("-", "_")};\n'
    return out


# ---------------------------------------------------------------------------
# Crate-root mounts
# ---------------------------------------------------------------------------
def mounts_block() -> str:
    ind = " " * 24
    out = ""
    for l in LEAVES:
        out += f'{ind}#[path = "."]\n{ind}pub mod {l.module} {{\n'
        out += f'{ind}    #[path = "{MUT_REL}/{l.dir}/🦀️.rs"]\n{ind}    mod component;\n'
        out += f'{ind}    #[path = "{MUT_REL}/{l.dir}/🔺️diff/🦀️.rs"]\n{ind}    pub mod diff;\n'
        out += f'{ind}    #[path = "{MUT_REL}/{l.dir}/↩️inverse/🦀️.rs"]\n{ind}    pub mod inverse;\n'
        out += f"{ind}    pub use component::*;\n{ind}}}\n"
    out += f'{ind}#[path = "{MUT_REL}/💾️binary/🦀️.rs"]\n{ind}pub mod binary;\n'
    out += f'{ind}#[path = "{MUT_REL}/📝️text/🦀️.rs"]\n{ind}pub mod text;\n'
    return out


def rewrite_crate_root() -> None:
    path = ART / "🦀️.rs"
    text = path.read_text(encoding="utf-8")
    start_marker = f'                    pub mod mutations {{\n                        #[path = "{MUT_REL}/🦀️.rs"]\n                        mod component;\n                        pub use component::*;\n'
    start = text.index(start_marker) + len(start_marker)
    end_marker = f'                        pub mod text;\n                    }}\n'
    end = text.index(end_marker, start)
    text = text[:start] + mounts_block() + text[end + len("                        pub mod text;\n"):]
    path.write_text(text, encoding="utf-8")


# ---------------------------------------------------------------------------
# Field meta
# ---------------------------------------------------------------------------
STRENGTH_CHOICES = [
    ("C14", "C14 — softwood strength class", "C14 — Nadelholz-Festigkeitsklasse"),
    ("C16", "C16 — softwood strength class", "C16 — Nadelholz-Festigkeitsklasse"),
    ("C18", "C18 — softwood strength class", "C18 — Nadelholz-Festigkeitsklasse"),
    ("C20", "C20 — softwood strength class", "C20 — Nadelholz-Festigkeitsklasse"),
    ("C22", "C22 — softwood strength class", "C22 — Nadelholz-Festigkeitsklasse"),
    ("C24", "C24 — softwood strength class", "C24 — Nadelholz-Festigkeitsklasse"),
    ("C27", "C27 — softwood strength class", "C27 — Nadelholz-Festigkeitsklasse"),
    ("C30", "C30 — softwood strength class", "C30 — Nadelholz-Festigkeitsklasse"),
    ("C35", "C35 — softwood strength class", "C35 — Nadelholz-Festigkeitsklasse"),
    ("C40", "C40 — softwood strength class", "C40 — Nadelholz-Festigkeitsklasse"),
    ("C45", "C45 — softwood strength class", "C45 — Nadelholz-Festigkeitsklasse"),
    ("C50", "C50 — softwood strength class", "C50 — Nadelholz-Festigkeitsklasse"),
    ("GL20h", "GL20h — glulam homogeneous", "GL20h — Brettschichtholz homogen"),
    ("GL24h", "GL24h — glulam homogeneous", "GL24h — Brettschichtholz homogen"),
    ("GL28h", "GL28h — glulam homogeneous", "GL28h — Brettschichtholz homogen"),
    ("GL32h", "GL32h — glulam homogeneous", "GL32h — Brettschichtholz homogen"),
    ("GL20c", "GL20c — glulam combined", "GL20c — Brettschichtholz kombiniert"),
    ("GL24c", "GL24c — glulam combined", "GL24c — Brettschichtholz kombiniert"),
    ("GL28c", "GL28c — glulam combined", "GL28c — Brettschichtholz kombiniert"),
    ("GL32c", "GL32c — glulam combined", "GL32c — Brettschichtholz kombiniert"),
    ("LVL32", "LVL 32 — laminated veneer lumber", "LVL 32 — Furnierschichtholz"),
    ("CLT100", "CLT 100 — cross-laminated timber", "BSP 100 — Brettsperrholz"),
]


def choices_const(name: str, rows: list[tuple[str, str, str]]) -> str:
    body = "".join(f'    choice("{v}", "{en}", "{de}"),\n' for v, en, de in rows)
    return f"const {name}: &[NormFieldChoice] = &[\n{body}];\n"


def meta_row(path: str, en: str, de: str, unit: str | None, choices: str | None) -> str:
    u = f'Some("{unit}")' if unit else "None"
    c = f"Some({choices})" if choices else "None"
    return f'    ("{path}", NormFieldMeta {{ label_en: "{en}", label_de: "{de}", unit: {u}, choices: {c} }}),\n'


def field_meta_rs() -> str:
    consts = ""
    consts += choices_const("ANNEX", [("en", "EN — recommended values (CEN)", "EN — Empfohlene Werte (CEN)"), ("de", "DE — German national annex (DIN)", "DE — Deutscher Nationaler Anhang (DIN)")])
    consts += choices_const("STRENGTH", STRENGTH_CHOICES)
    consts += choices_const("SERVICE", [("1", "Service class 1 — heated interior", "Nutzungsklasse 1 — beheizter Innenraum"), ("2", "Service class 2 — sheltered exterior", "Nutzungsklasse 2 — überdachter Außenbereich"), ("3", "Service class 3 — exposed to weather", "Nutzungsklasse 3 — der Witterung ausgesetzt")])
    consts += choices_const("ROLE", [("beam", "Beam — bending member", "Träger — Biegebauteil"), ("column", "Column — compression member", "Stütze — Druckbauteil"), ("floor", "Floor — joist or panel with vibration check", "Decke — Balken oder Platte mit Schwingungsnachweis"), ("bridge", "Bridge — EN 1995-2 girder", "Brücke — Träger nach EN 1995-2")])
    consts += choices_const("SUPPORT", [("simplySupported", "Simply supported", "Einfeldträger"), ("cantilever", "Cantilever", "Kragarm"), ("continuousTwoSpan", "Continuous over two spans", "Durchlaufträger über zwei Felder")])
    consts += choices_const("ACTION_KIND", [("permanent", "Permanent (G)", "Ständig (G)"), ("imposed", "Imposed (Q)", "Nutzlast (Q)"), ("snow", "Snow (S)", "Schnee (S)"), ("wind", "Wind (W)", "Wind (W)"), ("accidental", "Accidental (A)", "Außergewöhnlich (A)")])
    consts += choices_const("CATEGORY", [("", "None — not an imposed load", "Keine — keine Nutzlast"), ("A", "A — domestic and residential", "A — Wohnflächen"), ("B", "B — offices", "B — Büroflächen"), ("C", "C — congregation areas", "C — Versammlungsflächen"), ("D", "D — shopping areas", "D — Verkaufsflächen"), ("E", "E — storage areas", "E — Lagerflächen"), ("F", "F — light vehicle traffic", "F — Leichte Fahrzeuge"), ("G", "G — medium vehicle traffic", "G — Mittelschwere Fahrzeuge"), ("H", "H — roofs not accessible", "H — Nicht begehbare Dächer")])
    consts += choices_const("LOAD_DURATION", [("permanent", "Permanent (> 10 years)", "Ständig (> 10 Jahre)"), ("long", "Long-term (6 months – 10 years)", "Lang (6 Monate – 10 Jahre)"), ("medium", "Medium-term (1 week – 6 months)", "Mittel (1 Woche – 6 Monate)"), ("short", "Short-term (< 1 week)", "Kurz (< 1 Woche)"), ("instantaneous", "Instantaneous", "Sehr kurz")])
    consts += choices_const("FASTENER", [("nail", "Nail", "Nagel"), ("screw", "Screw", "Schraube"), ("bolt", "Bolt", "Bolzen"), ("dowel", "Dowel", "Stabdübel")])
    consts += choices_const("BOOL", [("true", "Yes", "Ja"), ("false", "No", "Nein")])

    rows = ""
    rows += meta_row("annex", "National annex", "Nationaler Anhang", None, "ANNEX")
    rows += meta_row("members", "Members", "Bauteile", None, None)
    rows += meta_row("members[].id", "Member id", "Bauteil-Id", None, None)
    for f in MEMBER_FIELDS:
        rows += meta_row(f"members[].{f[2]}", f[5], f[6], f[7], f[8])
    rows += meta_row("members[].actions", "Characteristic actions", "Charakteristische Einwirkungen", None, None)
    rows += meta_row("members[].actions[].id", "Action id", "Einwirkungs-Id", None, None)
    for f in MEMBER_ACTION_FIELDS:
        rows += meta_row(f"members[].actions[].{f[2]}", f[5], f[6], f[7], f[8])
    rows += meta_row("connections", "Connections", "Verbindungen", None, None)
    rows += meta_row("connections[].id", "Connection id", "Verbindungs-Id", None, None)
    for f in CONNECTION_FIELDS:
        rows += meta_row(f"connections[].{f[2]}", f[5], f[6], f[7], f[8])
    rows += meta_row("connections[].actions", "Characteristic fastener actions", "Charakteristische Verbindungsmitteleinwirkungen", None, None)
    rows += meta_row("connections[].actions[].id", "Action id", "Einwirkungs-Id", None, None)
    for f in CONNECTION_ACTION_FIELDS:
        rows += meta_row(f"connections[].actions[].{f[2]}", f[5], f[6], f[7], f[8])

    return f"""//! 🏷️ EN 1995 NormFieldMeta — SI units + en/de labels (`[]` wildcards for list leaves, nested action tables).

use crate::app_surface::{{lookup_norm_field_meta, NormFieldChoice, NormFieldMeta}};

const fn choice(value: &'static str, label_en: &'static str, label_de: &'static str) -> NormFieldChoice {{
    NormFieldChoice {{ value, label_en, label_de }}
}}

{consts}
const TABLE: &[(&str, NormFieldMeta)] = &[
{rows}];

/// 🏷️ Exact / `[]`-wildcard metadata for the EN 1995 timber subject.
pub fn en1995_field_meta(path: &str) -> Option<NormFieldMeta> {{
    lookup_norm_field_meta(TABLE, path)
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn every_editable_leaf_has_meta_with_both_labels() {{
        for (path, meta) in TABLE {{
            assert!(!meta.label_en.is_empty() && !meta.label_de.is_empty(), "{{path}}");
            let resolved = en1995_field_meta(&path.replace("[]", "[0]")).unwrap_or_else(|| panic!("{{path}} must resolve through an index"));
            assert_eq!(resolved.label_en, meta.label_en);
        }}
        assert!(en1995_field_meta("members[3].actions[1].qLineNPerM").is_some_and(|m| m.unit == Some("N/m")));
        assert!(en1995_field_meta("connections[0].actions[0].fKN").is_some_and(|m| m.unit == Some("N")));
        assert!(en1995_field_meta("members[0].strengthClass").is_some_and(|m| m.choices.is_some_and(|c| c.iter().any(|x| x.value == "GL24h" && x.label_de.contains("Brettschichtholz")))));
        assert!(en1995_field_meta("members[0].role").is_some_and(|m| m.choices.is_some_and(|c| c.len() == 4)));
    }}
}}
"""


# ---------------------------------------------------------------------------
# Index files (🔣️.json / 🟦️.ts / 🔗️.graphql / 🛰️.proto)
# ---------------------------------------------------------------------------
def payload_fields(leaf: Leaf) -> list[tuple[str, str, str]]:
    """(camel, ts type, kind) with kind in string|number|integer|boolean|object|enum:*"""
    s = leaf.scope
    if s == "annex":
        return [("newAnnex", '"En" | "De"', "string")]
    if s == "member-insert":
        return [("index", "number", "integer"), ("member", "TimberMember", "object")]
    if s in ("member-remove", "connection-remove"):
        return [("index", "number", "integer")]
    if s == "connection-insert":
        return [("index", "number", "integer"), ("connection", "TimberConnection", "object")]
    if s == "member-action-insert":
        return [("memberId", "string", "string"), ("index", "number", "integer"), ("action", "CharacteristicAction", "object")]
    if s == "member-action-remove":
        return [("memberId", "string", "string"), ("index", "number", "integer")]
    if s == "connection-action-insert":
        return [("connectionId", "string", "string"), ("index", "number", "integer"), ("action", "ConnectionAction", "object")]
    if s == "connection-action-remove":
        return [("connectionId", "string", "string"), ("index", "number", "integer")]
    j = leaf.field[4]
    if j.startswith("enum:"):
        ts = " | ".join(f'"{v}"' for v in ENUM_VALUES[j[5:]])
    else:
        ts = {"string": "string", "number": "number", "integer": "number", "boolean": "boolean"}[j]
    ids = {"member": [("memberId", "string", "string")], "connection": [("connectionId", "string", "string")], "member-action": [("memberId", "string", "string"), ("actionId", "string", "string")], "connection-action": [("connectionId", "string", "string"), ("actionId", "string", "string")]}[s]
    return ids + [("newValue", ts, j)]


def index_json() -> str:
    return json.dumps(
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": "https://json.schemas.assets.semio-tech.com/s/norm/en1995/mutations.json",
            "title": "En1995Mutation",
            "oneOf": [{"$ref": SCHEMA_ID.format(kind=l.kind)} for l in LEAVES],
        },
        indent=2,
        ensure_ascii=False,
    ) + "\n"


def index_ts() -> str:
    out = "/** 🧬️ En1995 document mutations — externally tagged union mirroring `En1995Mutation` (WASM wiring). */\n\nimport type { CharacteristicAction, ConnectionAction, TimberConnection, TimberMember } from \"../📸️snapshot/🟦️.ts\";\n\n"
    for l in LEAVES:
        out += f"export interface {l.variant} {{\n" + "".join(f"  {c}: {t};\n" for c, t, _ in payload_fields(l)) + "}\n\n"
    out += "export type En1995Mutation =\n" + "\n".join(f"  | {{ {l.variant}: {l.variant} }}" for l in LEAVES) + ";\n"
    return out


def index_graphql() -> str:
    gql = {"string": "String!", "number": "Float!", "integer": "Int!", "boolean": "Boolean!", "object": "JSON!"}
    out = "# 🧬️ EN 1995 mutation schema — hierarchical timber subject (members ⊃ actions, connections ⊃ actions).\n\nunion En1995Mutation =\n" + "\n".join(("    " if i == 0 else "  | ") + l.variant for i, l in enumerate(LEAVES)) + "\n\n"
    for l in LEAVES:
        out += f"type {l.variant} {{\n"
        for c, _, k in payload_fields(l):
            out += f"  {c}: {gql['string' if k.startswith('enum:') else k]} @state(class: ARTIFACT)\n"
        out += "}\n\n"
    return out.rstrip("\n") + "\n"


def index_proto() -> str:
    pb = {"string": "string", "number": "double", "integer": "uint32", "boolean": "bool", "object": "bytes"}
    out = "// 🧬️ EN 1995 mutation schema — hierarchical timber subject (members ⊃ actions, connections ⊃ actions).\nsyntax = \"proto3\";\npackage semio.s.norm.en1995.mutation;\n\n"
    for l in LEAVES:
        out += f"message {l.variant} {{\n"
        for i, (c, _, k) in enumerate(payload_fields(l), start=1):
            snake = re.sub(r"(?<!^)(?=[A-Z])", "_", c).lower()
            out += f"  // @state artifact\n  {pb['string' if k.startswith('enum:') else k]} {snake} = {i};\n"
        out += "}\n\n"
    out += "message En1995Mutation {\n  oneof payload {\n" + "".join(f"    {l.variant} {l.module if l.module != 'set_snapshot' else 'change_annex'} = {i};\n" for i, l in enumerate(LEAVES, start=1)) + "  }\n}\n"
    return out


# ---------------------------------------------------------------------------
# Oracle manifest
# ---------------------------------------------------------------------------
def rewrite_oracles() -> None:
    path = ANY / "🔮️oracles/🔣️.json"
    doc = json.loads(path.read_text(encoding="utf-8"))
    vectors = []
    mutations = []
    for l in LEAVES:
        sid, sdir = scenario(l)
        vectors.append({"mutationId": l.kind, "sourceMutationDirectoryName": l.dir, "mutationDirectoryName": l.dir, "scenarios": [{"id": sid, "directoryName": sdir}]})
        mutations.append(
            {
                "id": l.kind,
                "capability": "en1995-1-mutate",
                "payloadSchema": "🧬️.schema.json",
                "outcomes": ["applied", "no-op", "rejected"],
                "productionDispatch": {"operation": l.kind, "bridgeVersion": 1, "variant": l.variant},
                "oracleRequirements": [{"capability": "en1995-1-mutate", "qualifyingKind": "verified-native-second-implementation"}],
            }
        )
    for catalog in doc["mutationCatalogs"]:
        if catalog["id"] == "en1995-1-any":
            catalog["vectors"] = vectors
    for manifest in doc["mutationManifests"]:
        if manifest["artifact"] == "s.norm.en1995":
            manifest["mutations"] = mutations
    path.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


# ---------------------------------------------------------------------------
# change-annex fixture snapshots (serde camelCase of compliant_building_beam)
# ---------------------------------------------------------------------------
def building_beam(annex: str) -> dict:
    def action(id_, kind, category, dur, q):
        return {"id": id_, "kind": kind, "category": category, "loadDuration": dur, "qLineNPerM": q, "fPointN": 0.0, "mKNm": 0.0, "vKN": 0.0, "nKN": 0.0, "nTKN": 0.0, "fC90KN": 0.0}

    return {
        "annex": annex,
        "members": [
            {
                "id": "beam-B1",
                "labelEn": "Main glulam floor beam B1",
                "labelDe": "Hauptträger BSH Decke B1",
                "role": "floor",
                "strengthClass": "GL28h",
                "serviceClass": 1,
                "support": "simplySupported",
                "bM": 0.2,
                "hM": 0.4,
                "spanM": 5.0,
                "supportLengthM": 0.15,
                "bearingLengthM": 0.15,
                "bucklingLengthYM": 5.0,
                "bucklingLengthZM": 1.5,
                "lateralRestraintSpacingM": 1.5,
                "notchDepthM": 0.0,
                "notchDistanceM": 0.0,
                "mCritNm": 180000.0,
                "massKgPerM": 120.0,
                "massKgPerM2": 0.0,
                "dampingXi": 0.015,
                "fireDurationS": 0.0,
                "bridgeNObs": 0.0,
                "bridgeTLYears": 0.0,
                "bridgeBeta": 0.0,
                "bridgeA": 0.0,
                "bridgeB": 0.0,
                "bridgeCrowdPerM2": 0.0,
                "actions": [action("g", "permanent", "", "permanent", 2500.0), action("q", "imposed", "A", "medium", 3000.0)],
            }
        ],
        "connections": [
            {
                "id": "conn-C1",
                "labelEn": "Bolt group at support",
                "labelDe": "Schraubverbund am Auflager",
                "fastenerType": "bolt",
                "strengthClass": "GL28h",
                "serviceClass": 1,
                "diameterM": 0.012,
                "number": 8,
                "rows": 2,
                "spacingM": 0.09,
                "edgeDistanceM": 0.048,
                "endDistanceM": 0.096,
                "t1M": 0.2,
                "t2M": 0.2,
                "steelPlate": False,
                "steelPlateThicknessM": 0.0,
                "shearPlanes": 1,
                "fUK": 400000000.0,
                "actions": [
                    {"id": "g", "kind": "permanent", "loadDuration": "permanent", "fKN": 8000.0},
                    {"id": "q", "kind": "imposed", "loadDuration": "medium", "fKN": 10000.0},
                ],
            }
        ],
    }


def rewrite_fixture_snapshots() -> None:
    root = ANY / "🧫️fixtures/🧬️mutations/🌍️change-annex/🌍️switches-from-the-german-na-to-the-recommended-en-annex/📸️snapshot"
    write(root / "⬅️before/🔣️.json", json.dumps(building_beam("De"), indent=2, ensure_ascii=False) + "\n")
    write(root / "➡️after/🔣️.json", json.dumps(building_beam("En"), indent=2, ensure_ascii=False) + "\n")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main() -> None:
    keep = {l.dir for l in LEAVES}
    removed = []
    for entry in sorted(MUT.iterdir()):
        if entry.is_dir() and entry.name not in keep and re.search(r"(change|insert|remove)-", entry.name):
            shutil.rmtree(entry)
            removed.append(entry.name)
    for l in LEAVES:
        d = MUT / l.dir
        if l.scope == "annex":
            meta = json.loads((d / "🔣️.json").read_text(encoding="utf-8"))
            assert meta["binaryTag"] == l.tag and meta["semanticKind"] == l.kind
            continue
        tests = d / "🧪️tests"
        if tests.exists():
            shutil.rmtree(tests)
        write(d / "🦀️.rs", scalar_leaf_rs(l) if l.scope in ("member", "connection", "member-action", "connection-action") else collection_leaf_rs(l))
        write(d / "🔣️.json", leaf_json(l))
        write(d / "🧬️schema/🔣️.json", schema_json(l))
        write(d / "🔺️diff/🦀️.rs", diff_rs(l))
        write(d / "↩️inverse/🦀️.rs", inverse_rs(l))
        _, sdir = scenario(l)
        write(tests / sdir / "🦀️.rs", test_rs(l))
    write(MUT / "🦀️.rs", aggregate_rs())
    write(MUT / "🧪️tests/🔬️unit/🦀️.rs", unit_test_rs())
    write(MUT / "🧪️tests/🔬️fixture/🦀️.rs", fixture_index_rs())
    write(MUT / "📝️text/🦀️.rs", text_rs())
    write(MUT / "📝️text/🧪️tests/🔬️unit/🦀️.rs", text_unit_test_rs())
    write(MUT / "🔣️.json", index_json())
    write(MUT / "🟦️.ts", index_ts())
    write(MUT / "🔗️.graphql", index_graphql())
    write(MUT / "🛰️.proto", index_proto())
    write(ANY / "✏️editor/🏷️field-meta/🦀️.rs", field_meta_rs())
    rewrite_crate_root()
    rewrite_oracles()
    rewrite_fixture_snapshots()
    print(f"leaves: {len(LEAVES)}")
    print("removed:", ", ".join(removed))
    for l in LEAVES:
        print(f"{l.tag:3d} {l.variant:36s} {l.kind:44s} {l.dir}")


if __name__ == "__main__":
    main()
