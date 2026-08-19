//! 🚪️ IO s.reasoning.wires (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec
//! plus every foreign hop, aggregated from the typed `Serializer<WiresSnapshot>`/
//! `Deserializer<WiresSnapshot>` leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`.
//! Replaces the old hand-rolled `ArtifactComposition`/`ComposerEntry` dispatch chain
//! (`derived_composition`/`io_registry`) outright — all io now goes exclusively through the
//! `io_mechanism` registry (design.md rule 3).
//!
//! This root owns four native-codec facets, each relocated here verbatim from `🧬️schema/` (design.md
//! §1 CORRECTION): `📸️snapshot/📝️text` + `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack`
//! impls for `WiresSnapshot`), `🔺️diff/📝️text` + `🔺️diff/💾️binary`, `🧬️mutations/📝️text` +
//! `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for `WiresMutation`), and
//! `💡️inferences/📝️text` + `💡️inferences/💾️binary` (declaration-only — inference values are computed,
//! never authored). `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair { text: None,
//! binary: None }` below leaves their `dsl::LanguageSpec` registration deferred — a real, supported
//! shape per that type's own doc, matching the stdio pilot's identical documented deviation; the
//! underlying codec impls these would point at are unchanged and independently tested either way.

//#region 🔖️IoDeclaration
pub async fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::wires::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::wires::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::wires::{WiresMutation, WiresSnapshot, MINDMAP_WIRES_SCHEMA, WIRES_DIALECT};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    async fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<WiresSnapshot, export::csv::v_rfc4180::any::WiresIntoCsv>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::csv::v_rfc4180::any::CsvIntoWires>(WIRES_DIALECT),
                    serializer_entry::<WiresSnapshot, export::md::v_commonmark::any::WiresIntoMd>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::md::v_commonmark::any::MdIntoWires>(WIRES_DIALECT),
                    serializer_entry::<WiresSnapshot, export::json::v_rfc8259::any::WiresIntoJson>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::json::v_rfc8259::any::JsonIntoWires>(WIRES_DIALECT),
                    serializer_entry::<WiresSnapshot, export::svg::v1_1::any::WiresIntoSvg>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::svg::v1_1::any::SvgIntoWires>(WIRES_DIALECT),
                    serializer_entry::<WiresSnapshot, export::png::v1_2::any::WiresIntoPng>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::png::v1_2::any::PngIntoWires>(WIRES_DIALECT),
                    serializer_entry::<WiresSnapshot, export::txt::v_utf_8::any::WiresIntoTxt>(WIRES_DIALECT),
                    deserializer_entry::<WiresSnapshot, import::txt::v_utf_8::any::TxtIntoWires>(WIRES_DIALECT),
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
            codec: store::ArtifactCodec::of::<WiresSnapshot, WiresMutation>(MINDMAP_WIRES_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
