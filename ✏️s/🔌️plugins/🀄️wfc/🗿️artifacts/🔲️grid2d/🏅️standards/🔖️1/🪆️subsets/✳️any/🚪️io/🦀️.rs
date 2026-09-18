//! 🚪️ IO s.wfc.grid2d (1/✳️any) — `io() -> IoDeclaration`: the native codec plus every foreign hop,
//! aggregated from the typed `Serializer<Grid2dSnapshot>`/`Deserializer<Grid2dSnapshot>` leaves
//! under `📥️import/🧩️deserializers` / `📤️export/🧵️serializers`. All io goes exclusively through the
//! `io_mechanism` registry — the retired `ArtifactComposition`/`ComposerEntry` chain is never used.

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.txt"]
}

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{Grid2dMutation, Grid2dSnapshot, WFC_GRID2D_DIALECT, WFC_GRID2D_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    /// 🗣️ The five `dsl::LanguageSpec`s this subset declares, built once and leaked because
    /// `dsl::passthrough_hooks` is not `const fn`. Indices: 0=document 1=op 2=diff 3=pack 4=spr.
    fn languages() -> &'static [dsl::LanguageSpec; 5] {
        use crate::standards::v1::subsets::any::schema::{mutations, snapshot};
        static LANGUAGES: OnceLock<[dsl::LanguageSpec; 5]> = OnceLock::new();
        LANGUAGES.get_or_init(|| {
            [
                dsl::LanguageSpec {
                    id: "wfc.grid2d",
                    extension: Some("wfcgrid2d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid2d"),
                },
                dsl::LanguageSpec {
                    id: "wfc.grid2d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid2d.op"),
                },
                dsl::LanguageSpec { id: "wfc.grid2d.diff", extension: None, role: dsl::LanguageRole::Diff, grammar: None, grammar_path: None, protocol: None, protocol_path: None, hooks: dsl::passthrough_hooks("wfc.grid2d.diff") },
                dsl::LanguageSpec {
                    id: "wfc.grid2d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid2d.pack"),
                },
                dsl::LanguageSpec {
                    id: "wfc.grid2d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.grid2d.spr"),
                },
            ]
        })
    }

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<Grid2dSnapshot, export::json::v_rfc8259::any::Grid2dIntoJson>(WFC_GRID2D_DIALECT),
                    deserializer_entry::<Grid2dSnapshot, import::json::v_rfc8259::any::JsonIntoGrid2d>(WFC_GRID2D_DIALECT),
                    serializer_entry::<Grid2dSnapshot, export::txt::v_utf_8::any::Grid2dIntoTxt>(WFC_GRID2D_DIALECT),
                    deserializer_entry::<Grid2dSnapshot, import::txt::v_utf_8::any::TxtIntoGrid2d>(WFC_GRID2D_DIALECT),
                ]
            })
            .as_slice()
    }

    let langs = languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Grid2dSnapshot, Grid2dMutation>(WFC_GRID2D_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
