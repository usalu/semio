"""🪣️ Wave L (design §23, cross-editor gap of 2026-10-06 01:17, raster family): the framework's own shared schema documents
are in EVERY instance's input-schema resolver — publishing them is never a plugin's duty.

Cause: the runtime resolver (`manifest::registered_input_schema_document`) reads two process-wide lists — the `schema://`
export registry and the documents an app's own leaves declare (`Mutation::INPUT_SCHEMA_DOCUMENTS`, registered by the app
constructor). The framework's shared documents entered neither on their own: `framework/value/schema.json` was published by
nobody (the manifest reader embeds it only for its numeric-transport shape test), and the store's and io vocabulary's
documents were published only by a plugin ASSEMBLY (`PluginRuntimeRegistry::publish_declared_catalogs`), so an instance
constructed without one (every registryless law fixture, the acceptance harness) held none of them.

Fix, one registration per layer:
- framework (`🛂️manifest`): `FRAMEWORK_INPUT_SCHEMA_DOCUMENTS` (the value schema) is part of the resolver itself.
- OS (`🔌️plugin`): `PluginRuntimeRegistry::publish_framework_schema_documents` (store child / owner / link / blob + the io
  vocabulary; exact duplicates are tolerated by the registry) runs at EVERY app construction; the plugin assembly calls the
  same function.
- Law `an_input_that_references_a_shared_framework_schema_resolves_on_any_instance`.

Always present afterwards: `framework/value/schema.json` (every process); `framework/io/schema.json`,
`os/store/child/schema.json`, `os/store/child/owner/schema.json`, `os/store/link/schema.json`, `os/store/blob/schema.json`
(every instance).

Loaded by `🧪️s5-runtime-land.py`.
"""

FW = "🧰️framework/🔨️modules"
OSM = "🧰️framework/🛍️products/💻️os/🔨️modules"
MANIFEST = f"{FW}/🛂️manifest/🦀️.rs"
PLG = f"{OSM}/🔌️plugin/🦀️.rs"
LAW = f"{OSM}/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs"

MANIFEST_RS = [
    (
        """/// (`semio_framework_schema_registry::registered_referenced_schema_documents`): the document whose `$id` is `id`, or `None` when no plugin
/// published it.
""",
        """/// (`semio_framework_schema_registry::registered_referenced_schema_documents`), and the framework's own shared documents
/// ([`FRAMEWORK_INPUT_SCHEMA_DOCUMENTS`], held without any registration): the document whose `$id` is `id`, or `None`
/// when nobody published it.
""",
    ),
    (
        """    texts.extend(semio_framework_schema_registry::registered_referenced_schema_documents().into_iter().filter(|text| text.contains(id)));
""",
        """    texts.extend(semio_framework_schema_registry::registered_referenced_schema_documents().into_iter().filter(|text| text.contains(id)));
    texts.extend(FRAMEWORK_INPUT_SCHEMA_DOCUMENTS.into_iter().filter(|text| text.contains(id)));
""",
    ),
    (
        """const INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON: &str = include_str!("../🌱️value/🧬️schema/🔣️.json");
""",
        """const INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON: &str = include_str!("../🌱️value/🧬️schema/🔣️.json");

/// 🪣️ The shared schema documents of the framework's own namespace an input schema of any plugin may `$ref`:
/// [`registered_input_schema_document`] always holds them, so publishing them is never a plugin's duty (design §23).
/// Today the framework value schema (`framework/value/schema.json`); the OS adds its own at every app construction.
pub const FRAMEWORK_INPUT_SCHEMA_DOCUMENTS: [&str; 1] = [INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON];
""",
    ),
]

PLG_RS = [
    (
        """        /// 📌️ Publishes the store's and io vocabulary's own schema documents, the declared shared schema documents, and the
""",
        """        /// 🪣️ Publishes the schema documents the OS itself owns — the store's document model (a composed child, its owner
        /// stamp, a link, a blob) and the io vocabulary — into the OS-wide catalogs: what an input schema of any plugin may
        /// `$ref` without publishing it. Every app construction runs it (the catalog tolerates exact duplicates), so an
        /// instance resolves those references whether or not a plugin assembly ran in its process (design §23).
        pub(crate) fn publish_framework_schema_documents() -> Result<(), ::semio_framework_schema_registry::SchemaExportRegistryError> {
            store::register_store_schema_exports()?;
            store::io_schema::register_io_schema_exports()
        }

        /// 📌️ Publishes the store's and io vocabulary's own schema documents, the declared shared schema documents, and the
""",
    ),
    (
        """            store::register_store_schema_exports().map_err(documents_error)?;
            store::io_schema::register_io_schema_exports().map_err(documents_error)?;
""",
        """            Self::publish_framework_schema_documents().map_err(documents_error)?;
""",
    ),
    (
        """            for documents in <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMA_DOCUMENTS {
                semio_framework_schema_registry::register_referenced_schema_documents(documents);
            }
""",
        """            PluginRuntimeRegistry::publish_framework_schema_documents().expect("the OS's own schema documents never conflict");
            for documents in <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMA_DOCUMENTS {
                semio_framework_schema_registry::register_referenced_schema_documents(documents);
            }
""",
    ),
]

LAW_RS = [
    (
        """//#region 🎚️HistoryFilter
""",
        """//#region 🪣️SharedSchemaDocuments
/// 🪣️ A payload schema whose one input references the framework value schema (its numeric transport).
const SHARED_REFERENCE_SCHEMA: &str = r#"{"type":"object","additionalProperties":false,"required":["opacity"],"properties":{"opacity":{"$ref":"https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport","x-semio-ui":{"label":{"en":"Opacity","de":"Deckkraft"}}}}}"#;

/// ⚖️ LAW (design §23): the framework's own shared schema documents are in every instance's input-schema resolver — a
/// registryless instance, for which no plugin assembly published anything, holds the framework value schema, the io
/// vocabulary and the store's document model (child, owner, link, blob); an input that references the value schema reads
/// as one input and the draft editor renders its control.
#[semio_framework_async_macros::async_test]
async fn an_input_that_references_a_shared_framework_schema_resolves_on_any_instance() {
    let fixture = fixture();
    let mut app = seeded_app(&fixture).await;
    for id in ["framework/value/schema.json", "framework/io/schema.json", "os/store/child/schema.json", "os/store/child/owner/schema.json", "os/store/link/schema.json", "os/store/blob/schema.json"] {
        let id = format!("https://json.schemas.assets.semio-tech.com/{id}");
        assert!(semio_framework::registered_input_schema_document(&id).is_some(), "{id} is always in the runtime resolver");
    }
    let inputs = semio_framework::mutation_input_defs(SHARED_REFERENCE_SCHEMA, &semio_framework::registered_input_schema_document).unwrap_or_else(|error| panic!("the reference resolves: {error:?}"));
    assert_eq!(inputs.len(), 1, "the referenced value is one input: {inputs:?}");
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 0 })).await;
    let editor = app.time_travel.editor_mut().expect("the draft editor");
    editor.inputs = inputs;
    editor.value = dsl(&serde_json::json!({ "opacity": 0.5 }));
    let history = render_history(&mut app, Locale::De).await;
    let row = find_node(&history, "framework.history.editor.input.opacity").unwrap_or_else(|| panic!("the input's control: {history}"));
    assert!(history.to_string().contains("Deckkraft"), "the control carries the input's own label: {row}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new()).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}
//#endregion 🪣️SharedSchemaDocuments

//#region 🎚️HistoryFilter
""",
    ),
]


def files(_root):
    return {MANIFEST: MANIFEST_RS, PLG: PLG_RS, LAW: LAW_RS}
