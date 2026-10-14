//! 📇️ Layout diff schema descriptor: the handcrafted facet leaves (Rust, TypeScript, GraphQL, JSON Schema, Protobuf) of
//! the diff schema, embedded once.

/// 🔺️ The diff facet leaves of `s.layout.layout`.
pub const DIFF_FACET: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {
    rust: include_str!("../🦀️.rs"),
    typescript: include_str!("../🟦️.ts"),
    graphql: include_str!("../🔗️.graphql"),
    json_schema: include_str!("../🔣️.json"),
    proto: include_str!("../🛰️.proto"),
};
