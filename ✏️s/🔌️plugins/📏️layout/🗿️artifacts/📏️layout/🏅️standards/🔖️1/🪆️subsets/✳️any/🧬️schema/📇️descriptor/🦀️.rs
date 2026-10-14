//! 📇️ Layout schema descriptor: the handcrafted facet leaves of the artifact, snapshot and mutation schemas, with the
//! diff facet from its own descriptor crate.

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.layout.layout` — twenty handcrafted schema leaves.
pub fn layout_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.layout.layout",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("../🦀️.rs"), typescript: include_str!("../🟦️.ts"), graphql: include_str!("../🔗️.graphql"), json_schema: include_str!("../🔣️.json"), proto: include_str!("../🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../📸️snapshot/🦀️.rs"),
            typescript: include_str!("../📸️snapshot/🟦️.ts"),
            graphql: include_str!("../📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("../📸️snapshot/🔣️.json"),
            proto: include_str!("../📸️snapshot/🛰️.proto"),
        },
        diff: semio_s_artifact_layout_layout_descriptor_diff::DIFF_FACET,
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../🧬️mutations/🦀️.rs"),
            typescript: include_str!("../🧬️mutations/🟦️.ts"),
            graphql: include_str!("../🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("../🧬️mutations/🔣️.json"),
            proto: include_str!("../🧬️mutations/🛰️.proto"),
        },
    }
}
