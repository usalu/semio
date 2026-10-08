//! 📎️ BIM application schema descriptor; the app config is empty and each window owns its own persisted view state.

/// 📎️ Publishes the empty app-config facet and the non-empty shared-live presence facet.
pub fn app_schema_descriptor() -> semio_framework_schema_registry::AppSchemaDescriptor {
    semio_framework_schema_registry::AppSchemaDescriptor {
        id: "s.bim.model",
        config: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
        presence: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
