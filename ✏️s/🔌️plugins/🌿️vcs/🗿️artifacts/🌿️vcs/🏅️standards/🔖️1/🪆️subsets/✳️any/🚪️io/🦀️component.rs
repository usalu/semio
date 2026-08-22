//! 🚪️ IO s.vcs (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec plus every
//! foreign hop, aggregated from the typed `Serializer<VcsSnapshot>`/`Deserializer<VcsSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Replaces the old hand-rolled
//! `ArtifactComposition`/`ComposerEntry` dispatch chain (`derived_composition`/`io_registry`)
//! outright — all io now goes exclusively through the `io_mechanism` registry (design.md rule 3).
//!
//! This root owns four native-codec facets, each relocated here verbatim from `🧬️schema/` (design.md
//! §1 CORRECTION): `📸️snapshot/📝️text` + `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack`
//! impls for `VcsSnapshot`), `🔺️diff/📝️text` + `🔺️diff/💾️binary`, `🧬️mutations/📝️text` +
//! `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for `VcsDemoMutation`), and
//! `💡️inferences/📝️text` + `💡️inferences/💾️binary` (declaration-only — inference values are computed,
//! never authored). `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair { text: None,
//! binary: None }` below leaves their `dsl::LanguageSpec` registration deferred — a real, supported
//! shape per that type's own doc, matching the stdio pilot's identical documented deviation; the
//! underlying codec impls these would point at are unchanged and independently tested either way.

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::vcs::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::vcs::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::vcs::{VcsDemoMutation, VcsSnapshot, VCS_DIALECT, VCS_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    async fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<VcsSnapshot, export::json::v_rfc8259::any::VcsIntoJson>(VCS_DIALECT),
                    deserializer_entry::<VcsSnapshot, import::json::v_rfc8259::any::JsonIntoVcs>(VCS_DIALECT),
                    serializer_entry::<VcsSnapshot, export::csv::v_rfc4180::any::VcsIntoCsv>(VCS_DIALECT),
                    deserializer_entry::<VcsSnapshot, import::csv::v_rfc4180::any::CsvIntoVcs>(VCS_DIALECT),
                    serializer_entry::<VcsSnapshot, export::xlsx::v_ecma_376::any::VcsIntoXlsx>(VCS_DIALECT),
                    deserializer_entry::<VcsSnapshot, import::xlsx::v_ecma_376::any::XlsxIntoVcs>(VCS_DIALECT),
                    serializer_entry::<VcsSnapshot, export::zip::v2_0::any::VcsIntoZip>(VCS_DIALECT),
                    deserializer_entry::<VcsSnapshot, import::zip::v2_0::any::ZipIntoVcs>(VCS_DIALECT),
                    serializer_entry::<VcsSnapshot, export::txt::v_utf_8::any::VcsIntoTxt>(VCS_DIALECT),
                    deserializer_entry::<VcsSnapshot, import::txt::v_utf_8::any::TxtIntoVcs>(VCS_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<VcsSnapshot, VcsDemoMutation>(VCS_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
