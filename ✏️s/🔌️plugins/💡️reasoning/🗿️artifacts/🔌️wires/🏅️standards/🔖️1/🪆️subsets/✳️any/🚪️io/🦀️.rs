//! 🚪️ Wires native snapshot languages and owned artifact conversion routes.
//! 📸️ Snapshot Text and Pack facets publish the literal flat domain records.

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{WiresMutation, WiresSnapshot, MINDMAP_WIRES_SCHEMA, WIRES_DIALECT};
    use semio_framework_os_kernel::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
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
 semio_framework_dsl::LanguageSpec{id:"reasoning.wires",extension:Some("wires"),role:semio_framework_dsl::LanguageRole::Document,grammar:Some(self::text::snapshot::COMPONENT_GRAMMAR_SEMIO),grammar_path:Some(self::text::snapshot::COMPONENT_GRAMMAR_PATH),protocol:Some(self::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),protocol_path:Some(self::binary::snapshot::COMPONENT_PROTOCOL_PATH),hooks:semio_framework_dsl::passthrough_hooks("reasoning.wires")},
 semio_framework_dsl::LanguageSpec{id:"reasoning.wires.pack",extension:None,role:semio_framework_dsl::LanguageRole::Pack,grammar:None,grammar_path:None,protocol:Some(self::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),protocol_path:Some(self::binary::snapshot::COMPONENT_PROTOCOL_PATH),hooks:semio_framework_dsl::passthrough_hooks("reasoning.wires.pack")}
 ]).as_slice()
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::diff::WiresDiff;
    use crate::schema::mutations::WiresMutation;
    use crate::schema::snapshot::WiresSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug)]
    pub struct WiresBuilderConstruction {
        snapshot: WiresSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for WiresBuilderConstruction {
        type Snapshot = WiresSnapshot;
        type Mutation = WiresMutation;
        type Diff = WiresDiff;
        fn empty() -> Self {
            Self { snapshot: crate::empty_wires_snapshot(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WiresSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
