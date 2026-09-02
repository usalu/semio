//! 🚪️ IO s.forms (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec plus every
//! foreign hop, aggregated from the typed `Serializer<FormsSnapshot>`/`Deserializer<FormsSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Replaces the old hand-rolled
//! `ArtifactComposition`/`ComposerEntry` dispatch chain (`derived_composition`/`io_registry`)
//! outright — all io now goes exclusively through the `io_mechanism` registry (design.md rule 3).
//!
//! This root owns four native-codec facets, each relocated here verbatim from `🧬️schema/` (design.md
//! §1 CORRECTION): `📸️snapshot/📝️text` + `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack`
//! impls for `FormsSnapshot`), `🔺️diff/📝️text` + `🔺️diff/💾️binary`, `🧬️mutations/📝️text` +
//! `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for `FormMutation`), and
//! `💡️inferences/📝️text` + `💡️inferences/💾️binary` (declaration-only — inference values are
//! computed, never authored). `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair
//! { text: None, binary: None }` below leaves their `dsl::LanguageSpec` registration deferred — a
//! real, supported shape per that type's own doc, matching the stdio pilot's identical documented
//! deviation (also carried over from this artifact's own pre-existing, now-orphaned
//! `pilot_languages()`, deleted alongside the old `declaration()` channel — see the artifact root).

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::forms::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::forms::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::forms::{FormMutation, FormsSnapshot, FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    async fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<FormsSnapshot, export::json::v_rfc8259::any::FormsIntoJson>(FORMS_DIALECT),
                    deserializer_entry::<FormsSnapshot, import::json::v_rfc8259::any::JsonIntoForms>(FORMS_DIALECT),
                    serializer_entry::<FormsSnapshot, export::csv::v_rfc4180::any::FormsIntoCsv>(FORMS_DIALECT),
                    deserializer_entry::<FormsSnapshot, import::csv::v_rfc4180::any::CsvIntoForms>(FORMS_DIALECT),
                    serializer_entry::<FormsSnapshot, export::xlsx::v_ecma_376::any::FormsIntoXlsx>(FORMS_DIALECT),
                    deserializer_entry::<FormsSnapshot, import::xlsx::v_ecma_376::any::XlsxIntoForms>(FORMS_DIALECT),
                    serializer_entry::<FormsSnapshot, export::zip::v2_0::any::FormsIntoZip>(FORMS_DIALECT),
                    deserializer_entry::<FormsSnapshot, import::zip::v2_0::any::ZipIntoForms>(FORMS_DIALECT),
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
            codec: store::ArtifactCodec::of::<FormsSnapshot, FormMutation>(FORMS_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
