"""en1998 v9 step 2: model, diff and wire twins for MemberKind (role), liquefiableLayer (bool) and wallStoreyId (String reference). python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
LIQ_OLD = '"liquefiableLayer":{"type":"number","x-semio-ui":{"label":{"en":"Thickness of liquefiable or sensitive soil deposits (S2 indicator, 0 = none)","de":"Mächtigkeit verflüssigungsfähiger oder empfindlicher Bodenablagerungen (S2-Kennzeichen, 0 = keine)"},"widget":"stepper","precision":3,"step":0.001}}'
LIQ_NEW = '"liquefiableLayer":{"type":"boolean","x-semio-ui":{"widget":"toggle","label":{"en":"Liquefiable or sensitive soil deposits (ground type S2)","de":"Verflüssigungsfähige oder empfindliche Böden (Baugrundklasse S2)"},"description":{"en":"Yes when the ground has deposits susceptible to liquefaction or sensitive clays (EN 1998-1 Table 3.1 asks a yes/no question for S2).","de":"Ja bei verflüssigungsfähigen Böden oder empfindlichen Tonen (EN 1998-1 Tabelle 3.1 fragt für S2 ja/nein)."}}}'
WALL_OLD = '"wallStoreyId":{"type":"number","x-semio-ui":{"label":{"en":"Storey number of the wall, counted from 1 at the base (0 = every storey)","de":"Geschossnummer der Wand, ab 1 am Fußpunkt gezählt (0 = jedes Geschoss)"},"widget":"stepper","precision":3,"step":0.001}}'
WALL_NEW = '"wallStoreyId":{"type":"string","x-semio-ui":{"label":{"en":"Storey of the wall (empty = every storey)","de":"Geschoss der Wand (leer = jedes Geschoss)"},"description":{"en":"Id of a storey row of the building; empty means the wall counts in every storey.","de":"Kennung einer Geschosszeile des Gebäudes; leer heißt, die Wand zählt in jedem Geschoss."},"widget":"reference","role":"value","ref":{"kind":"storey"}}}'
ROLE_NEW = '"role":{"$ref":"#/$defs/MemberKind","x-semio-ui":{"label":{"en":"Member kind","de":"Bauteilart"},"description":{"en":"Beam, column, wall or brace; the wall fields apply to walls only.","de":"Balken, Stütze, Wand oder Verband; die Wandfelder gelten nur für Wände."}}}'
KIND_DEF = '    "MemberKind":{"title":"MemberKind","type":"string","enum":["beam","column","wall","brace"],"x-semio-ui":{"options":{"beam":{"en":"Beam","de":"Träger"},"column":{"en":"Column","de":"Stütze"},"wall":{"en":"Wall","de":"Wand"},"brace":{"en":"Brace","de":"Verband"}}},"x-semio-formats":["🔣️jsonschema","🟦️typescript","🔗️graphql"]},\n'
for js in (S / "📸️snapshot/🔣️.json", S / "🔣️.json"):
    sub(js, LIQ_OLD, LIQ_NEW); sub(js, WALL_OLD, WALL_NEW)
    sub(js, '"id":{"type":"string"},"material":{"type":"string"},"role":{"type":"string"},"detailingCompatibleWithQ"', '"id":{"type":"string"},"material":{"type":"string"},' + ROLE_NEW + ',"detailingCompatibleWithQ"')
    sub(js, '    "DeGroundCombo":{', KIND_DEF + '    "DeGroundCombo":{')
dj = S / "🔺️diff/🔣️.json"
sub(dj, '"wallStoreyId":{"anyOf":[{"type":"number"},{"type":"null"}]}', '"wallStoreyId":{"anyOf":[{"type":"string"},{"type":"null"}]}')
sub(dj, '"role":{"anyOf":[{"type":"string"},{"type":"null"}]}', '"role":{"anyOf":[{"type":"string","enum":["beam","column","wall","brace"]},{"type":"null"}]}')

TYPE_TS = 'export type MemberKind = "beam" | "column" | "wall" | "brace";\n'
PARSE_TS = 'export const parseMemberKind: NormWireReader<MemberKind> = normWireLiteral("beam", "column", "wall", "brace");\n'
for ts in (S / "📸️snapshot/🟦️.ts", S / "🟦️.ts"):
    sub(ts, "  liquefiableLayer: number;", "  liquefiableLayer: boolean;")
    sub(ts, "liquefiableLayer: normWireRequired(normWireNumber)", "liquefiableLayer: normWireRequired(normWireBoolean)")
    sub(ts, "  role: string;\n  detailingCompatibleWithQ", "  role: MemberKind;\n  detailingCompatibleWithQ")
    sub(ts, "role: normWireRequired(normWireString)", "role: normWireRequired(normWireRef(() => parseMemberKind))")
    sub(ts, "  wallStoreyId: number;", "  wallStoreyId: string;")
    sub(ts, "wallStoreyId: normWireRequired(normWireNumber)", "wallStoreyId: normWireRequired(normWireString)")
    sub(ts, 'export type DeGroundCombo =', TYPE_TS + 'export type DeGroundCombo =')
    sub(ts, 'export const parseDeGroundCombo:', PARSE_TS + 'export const parseDeGroundCombo:')
dts = S / "🔺️diff/🟦️.ts"
sub(dts, "role: (string) | null;", "role: (MemberKind) | null;")
sub(dts, "role: normWireDefault(normWireNullable(normWireString), () => null)", "role: normWireDefault(normWireNullable(normWireRef(() => parseMemberKind)), () => null)")
sub(dts, "wallStoreyId: (number) | null;", "wallStoreyId: (string) | null;")
sub(dts, "wallStoreyId: normWireDefault(normWireNullable(normWireNumber), () => null)", "wallStoreyId: normWireDefault(normWireNullable(normWireString), () => null)")
sub(dts, "type En1998VariableAction, parseEn1998Assessment,", "type En1998VariableAction, type MemberKind, parseMemberKind, parseEn1998Assessment,")
sub(S / "📸️snapshot/🔗️.graphql", "enum DeGroundCombo { AR BR CR BT CT CS }", "enum DeGroundCombo { AR BR CR BT CT CS }\nenum MemberKind { Beam Column Wall Brace }")
sub(S / "📸️snapshot/🔗️.graphql", "  liquefiableLayer: Float!", "  liquefiableLayer: Boolean!")
sub(S / "📸️snapshot/🔗️.graphql", "  material: String!\n  role: String!", "  material: String!\n  role: MemberKind!")
sub(S / "📸️snapshot/🔗️.graphql", "  wallStoreyId: Float!", "  wallStoreyId: String!")
sub(S / "🔺️diff/🔗️.graphql", "  role: String\n", "  role: MemberKind\n")
sub(S / "🔺️diff/🔗️.graphql", "  wallStoreyId: Float\n", "  wallStoreyId: String\n")
sub(S / "📸️snapshot/🛰️.proto", "  double liquefiable_layer = 12;", "  bool liquefiable_layer = 12;")
sub(S / "📸️snapshot/🛰️.proto", "  double wall_storey_id = 16;", "  string wall_storey_id = 16;")
sub(S / "🔺️diff/🛰️.proto", "  optional double wall_storey_id = 15;", "  optional string wall_storey_id = 15;")
sub(S / "🔺️diff/🦀️.rs", "role: String, detailing_compatible_with_q", "role: semio_s_artifact_norm_en1998_model::MemberKind, detailing_compatible_with_q")
sub(S / "🔺️diff/🦀️.rs", "wall_storey_id: f64 }", "wall_storey_id: String }")

R = S / "📸️snapshot/🦀️.rs"
ENUM = '''/// 🪧 Structural kind of a member row (EN 1998-1 5, 6, 7, 9): beam, column, wall or brace; the wall fields of a member apply to walls only.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum MemberKind {
    #[dsl(key = "beam")]
    #[value(rename = "beam")]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename = "beam"))]
    Beam,
    #[dsl(key = "column")]
    #[value(rename = "column")]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename = "column"))]
    Column,
    #[dsl(key = "wall")]
    #[value(rename = "wall")]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename = "wall"))]
    Wall,
    #[dsl(key = "brace")]
    #[value(rename = "brace")]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename = "brace"))]
    Brace,
}

impl MemberKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Beam => "beam",
            Self::Column => "column",
            Self::Wall => "wall",
            Self::Brace => "brace",
        }
    }
}

'''
sub(R, "/// 🌚️ Seismic site parameters (DE plateau", ENUM + "/// 🌚️ Seismic site parameters (DE plateau")
sub(R, "    pub liquefiable_layer: f64,", "    /// 💧 Ground has deposits susceptible to liquefaction or sensitive clays: ground type S2 of EN 1998-1 Table 3.1, a yes/no criterion without a thickness.\n    pub liquefiable_layer: bool,")
sub(R, "                liquefiable_layer: 0.0,", "                liquefiable_layer: false,")
sub(R, "    pub role: String,\n    pub detailing_compatible_with_q: bool,", "    pub role: MemberKind,\n    pub detailing_compatible_with_q: bool,")
sub(R, "    pub wall_storey_id: f64,", "    /// 🏬 Id of the storey row of the building the wall belongs to; empty = the wall counts in every storey. Validated against `buildings[].storeys[]`.\n    pub wall_storey_id: String,")
sub(R, 'role: "column".into()', "role: MemberKind::Column")
sub(R, 'role: "beam".into()', "role: MemberKind::Beam")
sub(R, "wall_storey_id: 1.0 }", "wall_storey_id: String::new() }", 2)
flush()
