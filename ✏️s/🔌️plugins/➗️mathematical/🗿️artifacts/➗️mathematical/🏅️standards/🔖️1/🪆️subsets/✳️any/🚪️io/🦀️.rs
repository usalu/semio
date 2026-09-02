//! 🚪️ IO s.mathematical.mathematical (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): native
//! codec plus every foreign hop, aggregated from the typed `Serializer<MathematicalSnapshot>`/
//! `Deserializer<MathematicalSnapshot>` leaves under `📥️import/🧩️deserializers`/
//! `📤️export/🧵️serializers`. Replaces the old hand-rolled `ArtifactComposition`/`ComposerEntry`
//! dispatch chain (`derived_composition`/`io_registry`) outright — all io now goes exclusively
//! through the `io_mechanism` registry (design.md rule 3). Those old modules used a non-canonical,
//! under-qualified `Dialect { artifact_kind: "s.mathematical", ... }` (missing the `.mathematical`
//! artifact segment design.md §1's `s.<plugin>.<artifact>` grammar requires) — deleted along with
//! them; `MATHEMATICAL_DIALECT` (`s.mathematical.mathematical`, defined on the artifact root) is
//! the only coordinate this file uses now.
//!
//! This root owns four native-codec facets, relocated here verbatim from `🧬️schema/` (design.md §1
//! CORRECTION): `📸️snapshot/📝️text` + `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack`
//! impls for `MathematicalSnapshot`), `🔺️diff/📝️text` + `🔺️diff/💾️binary`, `🧬️mutations/📝️text` +
//! `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for `MathematicalMutation`), and
//! `💡️inferences/📝️text` + `💡️inferences/💾️binary` (declaration-only — inference values are
//! computed, never authored). `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair {
//! text: None, binary: None }` below leaves their `dsl::LanguageSpec` registration deferred — a
//! real, supported shape per that type's own doc, matching `🎬️sequence`'s and the stdio pilot's
//! identical documented deviation; the underlying codec impls these would point at are unchanged
//! and independently tested either way.

//#region 🔖️IoDeclaration
pub async fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::mathematical::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::mathematical::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot, MATHEMATICAL_DIALECT, MATH_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    async fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<MathematicalSnapshot, export::csv::v_rfc4180::any::MathematicalIntoCsv>(MATHEMATICAL_DIALECT),
                    deserializer_entry::<MathematicalSnapshot, import::csv::v_rfc4180::any::CsvIntoMathematical>(MATHEMATICAL_DIALECT),
                    serializer_entry::<MathematicalSnapshot, export::md::v_commonmark::any::MathematicalIntoMd>(MATHEMATICAL_DIALECT),
                    deserializer_entry::<MathematicalSnapshot, import::md::v_commonmark::any::MdIntoMathematical>(MATHEMATICAL_DIALECT),
                    serializer_entry::<MathematicalSnapshot, export::json::v_rfc8259::any::MathematicalIntoJson>(MATHEMATICAL_DIALECT),
                    deserializer_entry::<MathematicalSnapshot, import::json::v_rfc8259::any::JsonIntoMathematical>(MATHEMATICAL_DIALECT),
                    serializer_entry::<MathematicalSnapshot, export::txt::v_utf_8::any::MathematicalIntoTxt>(MATHEMATICAL_DIALECT),
                    deserializer_entry::<MathematicalSnapshot, import::txt::v_utf_8::any::TxtIntoMathematical>(MATHEMATICAL_DIALECT),
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
            codec: store::ArtifactCodec::of::<MathematicalSnapshot, MathematicalMutation>(MATH_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
