"""🧪️ W2-W-norm-3: authors the seven EN 1998 `remove-<entity>` leaves that make every `insert-<entity>` invertible.

Every EN 1998 insert except `insert-building` returned an EMPTY inverse, so an inserted bridge, assessed element, silo,
tank, foundation, retaining wall or tower could never be undone. This writes each missing leaf on the exact template of
`➖️remove-building` (struct, descriptor, payload schema with en/de `x-semio-ui`, diff, inverse) and points the matching
insert's inverse at it. Idempotent: an existing file is rewritten with the same bytes.
"""
import json
import os

ROOT = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
OWNER = ROOT
LEAVES = [
    # entity, field, record, insert dir, insert module, insert struct, display, en, de, en noun, de position phrase, tag
    ("bridge", "bridges", "bridge", "🌉insert-bridge", "insert_bridge", "InsertBridge", "Remove bridge", "Remove bridge", "Brücke entfernen", "Bridge", "Brücke", "Zero-based position of the bridge in its list.", "Nullbasierte Position der Brücke in der Liste.", 22),
    ("assessment", "assessments", "assessment", "🔧insert-assessment", "insert_assessment", "InsertAssessment", "Remove assessment", "Remove assessed element", "Bewertetes Bauteil entfernen", "Assessed element", "Bewertetes Bauteil", "Zero-based position of the assessed element in its list.", "Nullbasierte Position des bewerteten Bauteils in der Liste.", 23),
    ("silo", "silos", "silo", "🫙insert-silo", "insert_silo", "InsertSilo", "Remove silo", "Remove silo", "Silo entfernen", "Silo", "Silo", "Zero-based position of the silo in its list.", "Nullbasierte Position des Silos in der Liste.", 24),
    ("tank", "tanks", "tank", "🛢insert-tank", "insert_tank", "InsertTank", "Remove tank", "Remove tank", "Tank entfernen", "Tank", "Tank", "Zero-based position of the tank in its list.", "Nullbasierte Position des Tanks in der Liste.", 25),
    ("foundation", "foundations", "foundation", "🪨insert-foundation", "insert_foundation", "InsertFoundation", "Remove foundation", "Remove foundation", "Gründung entfernen", "Foundation", "Gründung", "Zero-based position of the foundation in its list.", "Nullbasierte Position der Gründung in der Liste.", 26),
    ("retaining-wall", "retaining_walls", "wall", "🧱️insert-retaining-wall", "insert_retaining_wall", "InsertRetainingWall", "Remove retaining wall", "Remove retaining wall", "Stützwand entfernen", "Retaining wall", "Stützwand", "Zero-based position of the retaining wall in its list.", "Nullbasierte Position der Stützwand in der Liste.", 27),
    ("tower", "towers", "tower", "🗼insert-tower", "insert_tower", "InsertTower", "Remove tower", "Remove tower", "Turm entfernen", "Tower", "Turm", "Zero-based position of the tower in its list.", "Nullbasierte Position des Turms in der Liste.", 28),
]


def pascal(kebab):
    return "".join(part.capitalize() for part in kebab.split("-"))


def camel(kebab):
    text = pascal(kebab)
    return text[0].lower() + text[1:]


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as handle:
        handle.write(text)


for entity, field, record, insert_dir, insert_module, insert_struct, display, en, de, noun_en, noun_de, desc_en, desc_de, tag in LEAVES:
    kind = "remove-" + entity
    struct = pascal(kind)
    leaf = f"{ROOT}/➖️{kind}"
    write(f"{leaf}/🦀️.rs", f'''//! ➖️ `{kind}` mutation leaf.

use crate::{{En1998Mutation, En1998Snapshot}};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct {struct} {{
    pub index: usize,
}}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for {struct} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{
        verb: "remove",
        entity: "{entity}",
        kind: "{kind}",
        record: "{struct}",
    }};

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {{
        super::diff::diff(self, base)
    }}
    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {{
        super::inverse::inverse(self, base)
    }}
    fn label(&self) -> protocol::LocalizedLabel {{
        protocol::LocalizedLabel::native("{en}", "{de}")
    }}
    fn target(&self) -> Vec<String> {{
        vec![self.index.to_string()]
    }}
}}
''')
    write(f"{leaf}/🔺️diff/🦀️.rs", f'''//! Diff for `{kind}`.
use super::{struct};
use crate::{{En1998Diff, En1998Snapshot}};

pub fn diff(payload: &{struct}, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {{
    if payload.index >= base.{field}.len() {{
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{entity.replace('-', ' ')} #{{}}", payload.index), [payload.index.to_string()]);
    }}
    let mut items = base.{field}.clone();
    items.remove(payload.index);
    protocol::MutationOutcome::new(En1998Diff {{ {field}: Some(items), ..Default::default() }})
}}
''')
    write(f"{leaf}/↩️inverse/🦀️.rs", f'''//! Inverse for `{kind}`.
use super::{struct};
use crate::{{En1998Mutation, En1998Snapshot}};
use crate::standards::v1::subsets::any::schema::mutations::{insert_module};

pub fn inverse(payload: &{struct}, base: &En1998Snapshot) -> Vec<En1998Mutation> {{
    match base.{field}.get(payload.index) {{
        Some(item) => vec![En1998Mutation::{insert_struct}({insert_module}::{insert_struct} {{ index: payload.index, {record}: item.clone() }})],
        None => Vec::new(),
    }}
}}
''')
    write(f"{leaf}/🔣️.json", json.dumps({
        "schemaVersion": 1,
        "owner": f"{OWNER}/➖️{kind}",
        "semanticKind": kind,
        "displayName": display,
        "emoji": "➖️",
        "aggregateVariant": struct,
        "payloadSchema": "🧬️schema/🔣️.json",
        "textOpcode": None,
        "binaryTag": tag,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": ["applied", "no-op", "rejected"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
    }, ensure_ascii=False, indent=2) + "\n")
    write(f"{leaf}/🧬️schema/🔣️.json", json.dumps({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": f"https://json.schemas.assets.semio-tech.com/s/norm/en1998/mutation/{kind}/schema.json",
        "title": struct,
        "type": "object",
        "additionalProperties": False,
        "required": ["mutation", "index"],
        "properties": {
            "mutation": {"const": camel(kind)},
            "index": {
                "type": "integer",
                "minimum": 0,
                "x-semio-ui": {
                    "widget": "stepper",
                    "role": "value",
                    "label": {"en": noun_en, "de": noun_de},
                    "description": {"en": desc_en, "de": desc_de},
                    "step": 1,
                    "precision": 0,
                    "group": "target",
                    "order": 10,
                },
            },
        },
    }, ensure_ascii=False, indent=2) + "\n")
    write(f"{ROOT}/{insert_dir}/↩️inverse/🦀️.rs", f'''//! Inverse for `insert-{entity}`.
use super::{insert_struct};
use crate::{{En1998Mutation, En1998Snapshot}};
use crate::standards::v1::subsets::any::schema::mutations::{kind.replace('-', '_')};

pub fn inverse(payload: &{insert_struct}, base: &En1998Snapshot) -> Vec<En1998Mutation> {{
    vec![En1998Mutation::{struct}({kind.replace('-', '_')}::{struct} {{ index: payload.index.min(base.{field}.len()) }})]
}}
''')
    print("wrote", kind)
