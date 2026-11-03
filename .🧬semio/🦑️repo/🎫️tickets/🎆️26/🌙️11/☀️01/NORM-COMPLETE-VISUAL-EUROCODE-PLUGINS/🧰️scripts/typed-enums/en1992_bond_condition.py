#!/usr/bin/env python3
"""One-shot: en1992 BarLayer.bond_condition String -> enum BondCondition { Good, Poor }. Every replacement asserts its match count. Usage: python3 -I en1992_bond_condition.py [--dry]"""
import pathlib, re, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
S = ROOT / "🧬️schema"
DRY = "--dry" in sys.argv
cache = {}

def text(p):
    if p not in cache: cache[p] = p.read_text(encoding="utf-8")
    return cache[p]

def sub(p, old, new, count=1):
    t = text(p)
    n = t.count(old)
    assert n == count, f"{p.relative_to(ROOT)}: expected {count} of {old[:70]!r}, found {n}"
    cache[p] = t.replace(old, new)

M = "semio_s_artifact_norm_en1992_model"
ENUM_RS = '''/// 🔗 Bond condition of a bar layer (EN 1992-1-1 8.4.2, Figure 8.2): good or poor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BondCondition {
    Good,
    Poor,
}

'''
snap = S / "📸️snapshot/🦀️.rs"
sub(snap, "/// 🔩 Reinforcing steel — editable f_yk", ENUM_RS + "/// 🔩 Reinforcing steel — editable f_yk")
sub(snap, "    pub bond_condition: String,", "    pub bond_condition: BondCondition,")
sub(snap, 'bond_condition: "good".into()', "bond_condition: BondCondition::Good", 6)

sub(S / "🔺️diff/🦀️.rs", "lap_length: f64, bond_condition: String, aggregate_size", f"lap_length: f64, bond_condition: {M}::BondCondition, aggregate_size")

TYPE_TS = 'export type BondCondition = "Good" | "Poor";\n'
PARSE_TS = 'export const parseBondCondition: NormWireReader<BondCondition> = normWireLiteral("Good", "Poor");\n'
for ts in (S / "📸️snapshot/🟦️.ts", S / "🟦️.ts"):
    sub(ts, "  bondCondition: string;", "  bondCondition: BondCondition;")
    sub(ts, "bondCondition: normWireRequired(normWireString)", "bondCondition: normWireRequired(normWireRef(() => parseBondCondition))")
    sub(ts, "export type ExposureClass =", TYPE_TS + "export type ExposureClass =")
    sub(ts, "export const parseExposureClass:", PARSE_TS + "export const parseExposureClass:")

dts = S / "🔺️diff/🟦️.ts"
sub(dts, "  bondCondition: string | null;", "  bondCondition: BondCondition | null;")
sub(dts, "bondCondition: normWireRequired(normWireNullable(normWireString))", "bondCondition: normWireRequired(normWireNullable(normWireRef(() => parseBondCondition)))")
sub(dts, "type BarLayer, type BarPoint,", "type BarLayer, type BarPoint, type BondCondition,")
sub(dts, "parseBarLayer, parseBarPoint,", "parseBarLayer, parseBarPoint, parseBondCondition,")

for gq in (S / "📸️snapshot/🔗️.graphql", S / "🔗️.graphql"):
    sub(gq, "enum DuctilityClass {", "enum BondCondition { Good Poor }\nenum DuctilityClass {")
    sub(gq, "  bondCondition: String!", "  bondCondition: BondCondition!")

for pb in (S / "📸️snapshot/🛰️.proto", S / "🛰️.proto"):
    sub(pb, "enum DuctilityClass {", "enum BondCondition { BOND_GOOD = 0; BOND_POOR = 1; }\nenum DuctilityClass {")
    sub(pb, "string bond_condition = 7;", "BondCondition bond_condition = 7;")

DEF_JSON = '''    "BondCondition": {
      "title": "BondCondition",
      "type": "string",
      "enum": [
        "Good",
        "Poor"
      ],
      "x-semio-ui": {
        "options": {
          "Good": {
            "en": "Good bond conditions",
            "de": "Gute Verbundbedingungen"
          },
          "Poor": {
            "en": "Poor bond conditions",
            "de": "Schlechte Verbundbedingungen"
          }
        }
      }
    },
'''
OLD_PROP = '''        "bondCondition": {
          "type": "string",
          "x-semio-ui": {
            "label": {
              "en": "Bond condition",
              "de": "Verbundbedingung"
            },
            "description": {
              "en": "good or poor (EN 1992-1-1 8.4.2).",
              "de": "gut oder mäßig (EN 1992-1-1 8.4.2)."
            },
            "widget": "text"
          }
        },'''
NEW_PROP = '''        "bondCondition": {
          "$ref": "#/$defs/BondCondition",
          "x-semio-ui": {
            "label": {
              "en": "Bond condition",
              "de": "Verbundbedingung"
            },
            "description": {
              "en": "Good or poor bond conditions (EN 1992-1-1 8.4.2).",
              "de": "Gute oder schlechte Verbundbedingungen (EN 1992-1-1 8.4.2)."
            }
          }
        },'''
for js in (S / "📸️snapshot/🔣️.json", S / "🔣️.json"):
    sub(js, OLD_PROP, NEW_PROP)
    sub(js, '    "ExposureClass": {\n', DEF_JSON + '    "ExposureClass": {\n')

L = S / "🧬️mutations/⭕️change-bar-layer-bond-condition"
lr = L / "🦀️.rs"
sub(lr, "use semio_s_artifact_norm_en1992_model::En1992Snapshot;", "use semio_s_artifact_norm_en1992_model::BondCondition;\nuse semio_s_artifact_norm_en1992_model::En1992Snapshot;")
sub(lr, "pub new_bond_condition: String,", "pub new_bond_condition: BondCondition,")
sub(L / "🔺️diff/🦀️.rs", "bond_condition: Some(payload.new_bond_condition.clone())", "bond_condition: Some(payload.new_bond_condition)")
sub(L / "↩️inverse/🦀️.rs", "new_bond_condition: r1.bond_condition.clone()", "new_bond_condition: r1.bond_condition")
sub(L / "🧬️schema/🟦️.ts", "  newBondCondition: string;", "  newBondCondition: BondCondition;")
sub(L / "🧬️schema/🟦️.ts", "newBondCondition: normWireRequired(normWireString)", "newBondCondition: normWireRequired(parseBondCondition)")
sub(L / "🧬️schema/🟦️.ts", 'import { normWireObject,', 'import { type BondCondition, parseBondCondition } from "../../../📸️snapshot/🟦️.ts";\nimport { normWireObject,')
sub(L / "🧬️schema/🔗️.graphql", "  newBondCondition: String!", "  newBondCondition: String!")
sub(L / "🧬️schema/🛰️.proto", "  string newBondCondition = 3;", "  string newBondCondition = 3;")
sub(L / "🧬️schema/🔣️.json", '''    "newBondCondition": {
      "type": "string",
      "x-semio-ui": {
        "label": {
          "en": "Bond condition",
          "de": "Verbundbedingung"
        },
        "widget": "text",
        "role": "value",''', '''    "newBondCondition": {
      "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1992/snapshot.json#/$defs/BondCondition",
      "x-semio-ui": {
        "label": {
          "en": "Bond condition",
          "de": "Verbundbedingung"
        },
        "role": "value",''')

IO = ROOT / "🚪️io/🪶️sqlite/📸️snapshot"
sr = IO / "🦀️.rs"
sub(sr, "DuctilityClass, FireRating, TightnessClass};", "DuctilityClass, FireRating, TightnessClass, BondCondition};")
sub(sr, "fn tightness(a:TightnessClass)", 'fn bond_condition(a:BondCondition)->&\'static str{match a{BondCondition::Good=>"Good",BondCondition::Poor=>"Poor"}}\nfn read_bond_condition(a:&str)->Result<BondCondition, ValueError>{match a{"Good"=>Ok(BondCondition::Good),"Poor"=>Ok(BondCondition::Poor),_=>Err(invalid("EN1992 bond condition differs"))}}\nfn tightness(a:TightnessClass)')
sub(sr, "Cell::Text(&a.bond_condition)", "Cell::Text(bond_condition(a.bond_condition))")
sub(sr, "bond_condition:text(a,9,c)?", "bond_condition:read_bond_condition(text(a,9,c)?)?")
st = IO / "🟦️.ts"
sub(st, "DuctilityClass,FireRating,TightnessClass}from", "DuctilityClass,FireRating,TightnessClass,BondCondition}from")
sub(st, "bondCondition:string;", "bondCondition:BondCondition;")
sub(st, "const KINDS:readonly MemberKind[]=", 'const BONDS:readonly BondCondition[]=["Good","Poor"];\nconst KINDS:readonly MemberKind[]=')
sub(st, "a.lapLength,a.bondCondition,", "a.lapLength,choice(a.bondCondition,BONDS),") if text(st).count("a.lapLength,a.bondCondition,") else None
sub(st, "bondCondition:artifactSqliteText(a,9)", "bondCondition:choice(artifactSqliteText(a,9),BONDS)")
sub(IO / "🗄️.sql", "bond_condition TEXT NOT NULL,", "bond_condition TEXT NOT NULL CHECK(bond_condition IN ('Good','Poor')),")

CK = S / "💡️inferences/⚖️checks"
P = f"{M}::BondCondition::Poor"
sub(CK / "📋️member-evaluation/🔥️fire-anchorage-bridge-and-liquid-checks/🦀️.rs", 'layer.bond_condition == "poor"', f"layer.bond_condition == {P}")
sub(CK / "📋️member-evaluation/🔥️fire-anchorage-bridge-and-liquid-checks/🦀️.rs", 'layer.bond_condition != "poor"', f"layer.bond_condition != {P}")
sub(CK / "📋️member-evaluation/🧷️entity-cases/🦀️.rs", 'if layer.bond_condition == "poor" { "poor" } else { "good" }', f'if layer.bond_condition == {P} {{ "poor" }} else {{ "good" }}')
sub(CK / "🧪️tests/📍️reach/🦀️.rs", 'layer.bond_condition != "poor"', f"layer.bond_condition != {P}")

if not DRY:
    for p, t in cache.items(): p.write_text(t, encoding="utf-8")
print(("dry " if DRY else "") + f"ok: {len(cache)} files")
