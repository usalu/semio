//! 🎪 `stdio.pdf` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

use standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
use standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA;

#[path = "🧾️artifact-declaration/🦀️.rs"]
mod artifact_declaration;
pub use artifact_declaration::*;

#[path = "🌳️standards-tree/🦀️.rs"]
pub mod standards;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod bachelor_thesis {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🧪️tests/🧩️example/🦀️.rs"]
        mod bachelor_thesis_tests;
    }
}

#[path = "✏️editor-tree/🦀️.rs"]
pub mod editor;

#[path = "👁️viewer-tree/🦀️.rs"]
pub mod viewer;

#[cfg(test)]
pub(crate) fn register_sqlite_test_declaration() {
    static REGISTERED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    REGISTERED.get_or_init(|| { semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("stdio").label("PDF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(declaration(definition().unwrap()).unwrap()).try_build().unwrap(); });
}

#[test]
fn sqlite_snapshot_pdf_native_factories_publish_exact_structural_hashes() {
    let manifest: serde_json::Value = serde_json::from_str(ARTIFACT_DEFINITION_SCHEMA).unwrap();
    let hashes = native_codecs().into_iter().map(|factory| (factory.id, (factory.codec)().pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>())).collect::<Vec<_>>();
    for factory in native_codecs() {
        let codec = (factory.codec)();
        let actual = codec.pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let declared = manifest["codecs"].as_array().unwrap().iter().find(|entry| entry["native_factory"]["factory_id"] == factory.id).unwrap();
        assert_eq!(declared["native_factory"]["artifact_schema"], codec.schema);
        assert_eq!(declared["native_factory"]["pack_schema_hash"].as_str().unwrap(), actual, "owner structural hashes {hashes:?}");
    }
}

#[test]
fn sqlite_snapshot_pdf_owned_assets_describe_each_exact_record() {
    use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
    let snapshot14 = standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot { schema: "owned-fields".into(), pages: vec![standards::v1_4::subsets::base::schema::snapshot::PageDoc { width: f64::INFINITY, height: f64::from_bits(0x7ff0000000000001), text: "a\"b\nUnicode 🗄️".into() }] };
    let snapshot17 = PdfSnapshot { schema: "owned-fields".into(), declared_version: "1.7-extension".into(), ..Default::default() };
    let cases = [
        (snapshot14.print_dsl(), <standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot as ArtifactPack>::record_spec().unwrap(), include_str!("🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/📖️.grammar.semio"), include_str!("🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")),
        (snapshot17.print_dsl(), <PdfSnapshot as ArtifactPack>::record_spec().unwrap(), include_str!("🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/📖️.grammar.semio"), include_str!("🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")),
    ];
    for (text, record, grammar, protocol) in cases {
        let (envelope, body) = store::semio_format::split_text_preamble(&text).unwrap();
        let source = format!("{}\n{body}", envelope.envelope_id());
        assert!(semio_framework_dsl::Recognizer::compile(&semio_framework_dsl::parse_grammar(grammar).unwrap(), &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments").recognize(&source).unwrap(), "{source}");
        let protocol = semio_framework_dsl::parse_protocol(protocol).unwrap();
        let fields = protocol.blocks.iter().find_map(|block| match block { semio_framework_dsl::Block::Record { name, fields, .. } if name == "snapshot" => Some(fields), _ => None }).unwrap();
        assert_eq!(fields.iter().map(|field| field.name.as_str()).collect::<Vec<_>>(), record.fields.iter().map(|field| field.key.as_str()).collect::<Vec<_>>());
    }
    let wrong = store::semio_format::SemioEnvelope::from_envelope_id("stdio.pdf", store::semio_format::Component::Dsl, 1).unwrap();
    let text = snapshot17.print_dsl(); let (_, body) = store::semio_format::split_text_preamble(&text).unwrap();
    assert!(<PdfSnapshot as ArtifactDsl>::parse_dsl(&store::semio_format::wrap_text(&wrong, body)).is_err());
}
