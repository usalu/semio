
def gen_program_snapshot_rust(schema_id):
    field_lines = []
    for camel, snake, rtype, state, optional, card, scalar in PERSISTENT:
        dsl = "    #[dsl(table)]\n" if card == "list" else ""
        if snake in ("meta", "project", "governance"):
            dsl = "    #[dsl(block)]\n"
        field_lines.append(dsl + "    #[state(persistent)]\n    pub " + snake + ": " + rtype + ",")
    body = "\n".join(field_lines)
    vec_defaults = "\n".join("            " + s + ": Vec::new()," for s, _ in COLLECTIONS_META)
    head = (
        "//! 🧬️ Program snapshot schema — persistent fields only.\n\n"
        "use crate::artifacts::program::registers::*;\n"
        "use schema::ArtifactSchema;\n"
        "use serde::{Deserialize, Serialize};\n\n"
        "//#region 🔖️Snapshot\n"
        "/// 📸️ Persisted architect program snapshot (persistent fields of the artifact).\n"
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]\n"
        '#[serde(rename_all = "camelCase")]\n'
        '#[dsl(extension = "architect", layout = "lines")]\n'
        '#[artifact_schema(id = "' + schema_id + '")]\n'
        "pub struct ProgramSnapshot {\n"
        + body + "\n}\n\n"
        "impl Default for ProgramSnapshot {\n"
        "    fn default() -> Self {\n"
        "        Self {\n"
        "            schema: crate::artifacts::program::ARCHITECT_PROGRAM_SCHEMA.into(),\n"
        "            meta: ProgramMeta::default(),\n"
        "            project: ProjectDefinition::default(),\n"
        + vec_defaults + "\n"
        "            governance: Governance::default(),\n"
        "        }\n"
        "    }\n"
        "}\n"
    )
    codecs = (TICKET / "🧪wave5-architect-snapshot-codecs.rs.txt").read_text()
    return head + "\n" + codecs
