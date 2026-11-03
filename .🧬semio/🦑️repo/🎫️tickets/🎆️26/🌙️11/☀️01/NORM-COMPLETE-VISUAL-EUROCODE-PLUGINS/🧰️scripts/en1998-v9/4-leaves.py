"""en1998 v9 step 4: payload types of the three retyped mutation leaves (liquefiable layer bool, wall storey id string reference, member kind enum). python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
KIND = "semio_s_artifact_norm_en1998_model::MemberKind"
LIQ, WALL, ROLE = M / "✏️change-site-liquefiable-layer", M / "✏️change-member-wall-storey-id", M / "✏️change-member-role"
sub(LIQ / "🦀️.rs", "pub new_liquefiable_layer: f64,", "pub new_liquefiable_layer: bool,")
sub(LIQ / "🧬️schema/🔣️.json", '''      "type": "number",
      "x-semio-ui": {
        "role": "value",
        "label": {
          "en": "Liquefiable or sensitive soil deposits (ground type S2)",
          "de": "Verflüssigungsfähige oder empfindliche Böden (Baugrundklasse S2)"
        },
        "group": "value",
        "order": 10,
        "widget": "stepper",
        "step": 0.001,
        "precision": 3
      }''', '''      "type": "boolean",
      "x-semio-ui": {
        "role": "value",
        "label": {
          "en": "Liquefiable or sensitive soil deposits (ground type S2)",
          "de": "Verflüssigungsfähige oder empfindliche Böden (Baugrundklasse S2)"
        },
        "group": "value",
        "order": 10,
        "widget": "toggle"
      }''')
t = LIQ / "🧬️schema/🟦️.ts"
sub(t, "import { normWireInteger, normWireLiteral, normWireNumber, normWireObject,", "import { normWireBoolean, normWireInteger, normWireLiteral, normWireObject,")
sub(t, "  newLiquefiableLayer: number;", "  newLiquefiableLayer: boolean;")
sub(t, "newLiquefiableLayer: normWireRequired(normWireNumber)", "newLiquefiableLayer: normWireRequired(normWireBoolean)")

sub(WALL / "🦀️.rs", "pub new_wall_storey_id: f64,", "pub new_wall_storey_id: String,")
sub(WALL / "🔺️diff/🦀️.rs", '''    if !payload.new_wall_storey_id.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
''', "")
sub(WALL / "🔺️diff/🦀️.rs", "wall_storey_id: Some(payload.new_wall_storey_id)", "wall_storey_id: Some(payload.new_wall_storey_id.clone())")
sub(WALL / "↩️inverse/🦀️.rs", "new_wall_storey_id: r1.wall_storey_id }", "new_wall_storey_id: r1.wall_storey_id.clone() }")
sub(WALL / "🧬️schema/🔣️.json", '''      "type": "number",
      "x-semio-ui": {
        "role": "value",
        "label": {
          "en": "Storey of the wall (empty = every storey)",
          "de": "Geschoss der Wand (leer = jedes Geschoss)"
        },
        "group": "value",
        "order": 30,
        "widget": "stepper",
        "step": 0.001,
        "precision": 3
      }''', '''      "type": "string",
      "x-semio-ui": {
        "role": "value",
        "label": {
          "en": "Storey of the wall (empty = every storey)",
          "de": "Geschoss der Wand (leer = jedes Geschoss)"
        },
        "description": {
          "en": "Id of a storey row of the building; empty means the wall counts in every storey.",
          "de": "Kennung einer Geschosszeile des Gebäudes; leer heißt, die Wand zählt in jedem Geschoss."
        },
        "ref": {
          "kind": "storey"
        },
        "group": "value",
        "order": 30,
        "widget": "reference"
      }''')
t = WALL / "🧬️schema/🟦️.ts"
sub(t, "  newWallStoreyId: number;", "  newWallStoreyId: string;")
sub(t, "newWallStoreyId: normWireRequired(normWireNumber)", "newWallStoreyId: normWireRequired(normWireString)")

sub(ROLE / "🦀️.rs", "pub new_role: String,", f"pub new_role: {KIND},")
sub(ROLE / "🔺️diff/🦀️.rs", "role: Some(payload.new_role.clone())", "role: Some(payload.new_role)")
sub(ROLE / "↩️inverse/🦀️.rs", "new_role: r1.role.clone()", "new_role: r1.role")
sub(ROLE / "🧬️schema/🔣️.json", '''    "newRole": {
      "type": "string",
      "x-semio-ui": {
        "role": "value",
        "label": {
          "en": "Member role",
          "de": "Bauteilrolle"
        },
        "group": "value",
        "order": 30,
        "widget": "text"
      }''', '''    "newRole": {
      "type": "string",
      "enum": [
        "beam",
        "column",
        "wall",
        "brace"
      ],
      "x-semio-ui": {
        "role": "value",
        "label": {
          "en": "Member kind",
          "de": "Bauteilart"
        },
        "group": "value",
        "order": 30,
        "widget": "select"
      }''')
t = ROLE / "🧬️schema/🟦️.ts"
sub(t, "newRole: string;", 'newRole: "beam" | "column" | "wall" | "brace";')
sub(t, "newRole: normWireRequired(normWireString) }", 'newRole: normWireRequired(normWireLiteral("beam", "column", "wall", "brace")) }')

sub(M / "🛰️.proto", "message ChangeSiteLiquefiableLayer { /* @state artifact */ double new_liquefiable_layer = 1; }", "message ChangeSiteLiquefiableLayer { /* @state artifact */ bool new_liquefiable_layer = 1; }")
sub(M / "🛰️.proto", "double new_wall_storey_id = 3; }", "string new_wall_storey_id = 3; }")
sub(M / "🔗️.graphql", "type ChangeSiteLiquefiableLayer { newLiquefiableLayer: Float!", "type ChangeSiteLiquefiableLayer { newLiquefiableLayer: Boolean!")
sub(M / "🔗️.graphql", "newWallStoreyId: Float!", "newWallStoreyId: String!")
sub(M / "🎲️demo-cases/🦀️.rs", "new_liquefiable_layer: 1.5 }", "new_liquefiable_layer: true }")
sub(M / "🎲️demo-cases/🦀️.rs", 'new_wall_storey_id: 1.5 }', 'new_wall_storey_id: String::from("x") }')
sub(M / "🎲️demo-cases/🦀️.rs", 'new_role: String::from("beam") }', f'new_role: {KIND}::Beam }}')
sub(M / "🎲️demo-cases/🦀️.rs", 'role: String::from("x"), detail', f'role: {KIND}::Beam, detail')
sub(M / "🎲️demo-cases/🦀️.rs", "wall_storey_id: 1.0 } }),", "wall_storey_id: String::new() } }),")
flush()
