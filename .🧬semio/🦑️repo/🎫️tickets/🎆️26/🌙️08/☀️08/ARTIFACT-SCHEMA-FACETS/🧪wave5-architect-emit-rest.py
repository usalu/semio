
def gen_program_diff_rust(schema_id):
    lines = []
    lines.append("//! 🧬️ Program diff schema — sparse field delta over the artifact.")
    lines.append("")
    lines.append("use crate::artifacts::program::registers::*;")
    lines.append("use schema::ArtifactSchema;")
    lines.append("use serde::{Deserialize, Serialize};")
    lines.append("")
    lines.append("//#region 🔖️Diff")
    lines.append("/// 🔺️ Sparse field delta for the program artifact.")
    lines.append("#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]")
    lines.append('#[serde(rename_all = "camelCase", default)]')
    lines.append(f'#[artifact_schema(id = "{schema_id}")]')
    lines.append("pub struct ProgramDiff {")
    lines.append("    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::program::schema::ProgramArtifact>>,")
    for camel, snake, rtype, state, optional, card, scalar in (PERSISTENT + UI):
        if camel in COLLECTION_CAMELS:
            lines.append(f"    #[state({state})] pub {snake}: Option<Program{pascal(snake)}Delta>,")
        elif camel == "selectedIds":
            lines.append(f"    #[state({state})] pub {snake}: Option<ProgramStringList>,")
        elif optional and rtype.startswith("Option<"):
            lines.append(f"    #[state({state})] pub {snake}: Option<{rtype}>,")
        else:
            lines.append(f"    #[state({state})] pub {snake}: Option<{rtype}>,")
    lines.append("}")
    lines.append("//#endregion 🔖️Diff")
    lines.append("")
    lines.append("//#region 🔖️DeltaHelpers")
    lines.append("/// 📋 String-list wrapper so optional list diffs stay scalar across formats.")
    lines.append("#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]")
    lines.append('#[serde(rename_all = "camelCase", default)]')
    lines.append("pub struct ProgramStringList { pub values: Vec<String>, }")
    lines.append("")
    for snake, item in COLLECTIONS_META:
        lines.append(f"/// 🧩 Identified-collection delta for `{snake}`.")
        lines.append("#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]")
        lines.append('#[serde(rename_all = "camelCase", default)]')
        lines.append(f"pub struct Program{pascal(snake)}Delta {{")
        lines.append(f"    pub added: Vec<{item}>,")
        lines.append("    pub removed: Vec<String>,")
        lines.append(f"    pub patched: Vec<Program{pascal(snake)}PatchEntry>,")
        lines.append("    pub reordered: Option<Vec<String>>,")
        lines.append("}")
        lines.append("")
        lines.append(f"/// 🩹 One patched `{item}` entry.")
        lines.append("#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]")
        lines.append('#[serde(rename_all = "camelCase")]')
        lines.append(f"pub struct Program{pascal(snake)}PatchEntry {{")
        lines.append("    pub id: String,")
        lines.append(f"    pub patch: {item}Patch,")
        lines.append("}")
        lines.append("")
    lines.append("//#endregion 🔖️DeltaHelpers")
    return "\n".join(lines) + "\n"


def gen_program_conversions(schema_id):
    pers = [f[1] for f in PERSISTENT]
    snap_fields = ", ".join(f"{s}: self.{s}.clone()" for s in pers)
    from_fields = ", ".join(f"{s}: snapshot.{s}" for s in pers)
    set_fields = "\n".join(f"        self.{s} = snapshot.{s};" for s in pers)
    ui_defaults = [
        ("selected_ids", "Vec::new()"),
        ("active_register", '"elements".into()'),
        ("adjacency_kind_filter", "None"),
        ("active_report_json", "String::new()"),
        ("search_query", "String::new()"),
        ("search_history_json", '"[]".into()'),
        ("last_result_json", "String::new()"),
        ("last_analysis_json", "String::new()"),
        ("graph_camera_x", "0.0"),
        ("graph_camera_y", "0.0"),
        ("graph_camera_zoom", "1.0"),
    ]
    default_ui = ",\n            ".join(f"{k}: {v}" for k, v in ui_defaults)
    pers_default = ",\n            ".join(f"{s}: Default::default()" for s in pers)
    return f"""
//#region 🔖️Conversions
impl Default for ProgramArtifact {{
    fn default() -> Self {{
        Self {{
            {pers_default},
            {default_ui},
        }}
    }}
}}

impl ProgramArtifact {{
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::program::ProgramSnapshot {{
        crate::artifacts::program::ProgramSnapshot {{
            {snap_fields},
        }}
    }}

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::program::ProgramSnapshot) -> Self {{
        Self {{
            {from_fields},
            ..Self::default()
        }}
    }}

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::program::ProgramSnapshot) {{
{set_fields}
    }}
}}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `{schema_id}` — fifteen handcrafted schema leaves.
pub fn program_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {{
    schema::ArtifactSchemaDescriptor {{
        id: "{schema_id}",
        artifact: schema::FacetLeaves {{
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        }},
        snapshot: schema::FacetLeaves {{
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        }},
        diff: schema::FacetLeaves {{
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        }},
    }}
}}
//#endregion 🔖️Descriptor
"""


def emit_all():
    fields = PERSISTENT + UI
    schema_id = "s.architect.program"
    prefix = "Program"
    collections = COLLECTION_CAMELS
    for sub in ["🧬️schema", "📸️snapshot/🧬️schema", "🔺️diff/🧬️schema"]:
        (ART / sub).mkdir(parents=True, exist_ok=True)
    write(ART / "🧬️schema" / "🔣️component.json", gen_artifact_json(prefix, "architect", "program", fields, DEFS))
    write(ART / "📸️snapshot" / "🧬️schema" / "🔣️component.json", gen_snapshot_json(prefix, "architect", "program", PERSISTENT, DEFS))
    doc = json.loads(gen_diff_json(prefix, "architect", "program", fields, collections, DEFS))
    doc["properties"]["selectedIds"] = {"$ref": "#/$defs/ProgramStringList", "x-semio-state": "shared-ui"}
    doc["properties"]["adjacencyKindFilter"] = {"oneOf": [{"type": "null"}, {"$ref": "#/$defs/AdjacencyKind"}], "x-semio-state": "shared-ui"}
    doc["$defs"]["ProgramStringList"] = {
        "title": "ProgramStringList",
        "type": "object",
        "additionalProperties": False,
        "required": ["values"],
        "properties": {"values": {"type": "array", "items": {"type": "string"}}},
    }
    for snake, item in COLLECTIONS_META:
        c = camel(snake)
        pascal_c = c[0].upper() + c[1:]
        delta = f"Program{pascal_c}Delta"
        entry = f"Program{pascal_c}PatchEntry"
        doc["$defs"][f"{item}Patch"] = {"title": f"{item}Patch", "type": "object", "additionalProperties": True}
        doc["$defs"][entry] = {
            "title": entry,
            "type": "object",
            "additionalProperties": False,
            "required": ["id", "patch"],
            "properties": {"id": {"type": "string"}, "patch": {"$ref": f"#/$defs/{item}Patch"}},
        }
        doc["$defs"][delta] = {
            "title": delta,
            "type": "object",
            "additionalProperties": False,
            "required": ["added", "removed", "patched"],
            "properties": {
                "added": {"type": "array", "items": {"$ref": f"#/$defs/{item}"}},
                "removed": {"type": "array", "items": {"type": "string"}},
                "patched": {"type": "array", "items": {"$ref": f"#/$defs/{entry}"}},
                "reordered": {"type": "array", "items": {"type": "string"}},
            },
        }
    write(ART / "🔺️diff" / "🧬️schema" / "🔣️component.json", json.dumps(doc, indent=2) + "\n")
    write(ART / "🧬️schema" / "🔗️component.graphql", gen_artifact_graphql(prefix, fields))
    write(ART / "📸️snapshot" / "🧬️schema" / "🔗️component.graphql", gen_snapshot_graphql(prefix, PERSISTENT))
    write(ART / "🔺️diff" / "🧬️schema" / "🔗️component.graphql", gen_diff_graphql(prefix, fields, collections))
    write(ART / "🧬️schema" / "🟦️component.ts", gen_artifact_ts(prefix, fields))
    write(ART / "📸️snapshot" / "🧬️schema" / "🟦️component.ts", gen_snapshot_ts(prefix, PERSISTENT))
    write(ART / "🔺️diff" / "🧬️schema" / "🟦️component.ts", gen_diff_ts(prefix, fields, collections))
    write(ART / "🧬️schema" / "🛰️component.proto", gen_artifact_proto(prefix, "architect", "program", fields))
    write(ART / "📸️snapshot" / "🧬️schema" / "🛰️component.proto", gen_snapshot_proto(prefix, "architect", "program", PERSISTENT))
    write(ART / "🔺️diff" / "🧬️schema" / "🛰️component.proto", gen_diff_proto(prefix, "architect", "program", fields, collections))
    art_rs = gen_artifact_rust(prefix, schema_id, fields, "program::registers::*")
    art_rs += gen_program_conversions(schema_id)
    write(ART / "🧬️schema" / "🦀️component.rs", art_rs)
    write(ART / "📸️snapshot" / "🧬️schema" / "🦀️component.rs", gen_program_snapshot_rust(schema_id))
    write(ART / "🔺️diff" / "🧬️schema" / "🦀️component.rs", gen_program_diff_rust(schema_id))
    print("ALL ARCHITECT LEAVES GENERATED")

emit_all()
