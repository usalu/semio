//! 🚪️ Wires native snapshot languages and owned artifact conversion routes.
//! 📸️ Snapshot Text and Pack facets publish the literal flat domain records.

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{WiresMutation, WiresSnapshot, MINDMAP_WIRES_SCHEMA, WIRES_DIALECT};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<WiresSnapshot, export::json::v_rfc8259::any::WiresIntoJson>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::json::v_rfc8259::any::JsonIntoWires>(WIRES_DIALECT),
                    serializer_entry::<WiresSnapshot, export::txt::v_utf_8::any::WiresIntoTxt>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::txt::v_utf_8::any::TxtIntoWires>(WIRES_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&snapshot_languages()[0]), binary: Some(&snapshot_languages()[1]) },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::bare::<WiresSnapshot, WiresMutation>(MINDMAP_WIRES_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
fn snapshot_languages()->&'static[semio_framework_dsl::LanguageSpec]{
 static LANGUAGES:std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>>=std::sync::OnceLock::new();
 LANGUAGES.get_or_init(||vec![
 semio_framework_dsl::LanguageSpec{id:"reasoning.wires",extension:Some("wires"),role:semio_framework_dsl::LanguageRole::Document,grammar:Some(super::snapshot::text::COMPONENT_GRAMMAR_SEMIO),grammar_path:Some(super::snapshot::text::COMPONENT_GRAMMAR_PATH),protocol:Some(super::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),protocol_path:Some(super::snapshot::binary::COMPONENT_PROTOCOL_PATH),hooks:semio_framework_dsl::passthrough_hooks("reasoning.wires")},
 semio_framework_dsl::LanguageSpec{id:"reasoning.wires.pack",extension:None,role:semio_framework_dsl::LanguageRole::Pack,grammar:None,grammar_path:None,protocol:Some(super::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),protocol_path:Some(super::snapshot::binary::COMPONENT_PROTOCOL_PATH),hooks:semio_framework_dsl::passthrough_hooks("reasoning.wires.pack")}
 ]).as_slice()
}
