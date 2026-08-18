//! 🚪️ IO s.sequence (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec plus
//! every foreign hop, aggregated from the typed `Serializer<SequenceSnapshot>`/
//! `Deserializer<SequenceSnapshot>` leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`.
//! Replaces the old hand-rolled `ArtifactComposition`/`ComposerEntry` dispatch chain outright — all
//! io now goes exclusively through the `io_mechanism` registry (design.md rule 3).

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::sequence::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::sequence::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::sequence::{SequenceMutation, SequenceSnapshot, SEQUENCE_DIALECT, SEQUENCE_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<SequenceSnapshot, export::csv::v_rfc4180::any::SequenceIntoCsv>(SEQUENCE_DIALECT),
                    deserializer_entry::<SequenceSnapshot, import::csv::v_rfc4180::any::CsvIntoSequence>(SEQUENCE_DIALECT),
                    serializer_entry::<SequenceSnapshot, export::md::v_commonmark::any::SequenceIntoMd>(SEQUENCE_DIALECT),
                    deserializer_entry::<SequenceSnapshot, import::md::v_commonmark::any::MdIntoSequence>(SEQUENCE_DIALECT),
                    serializer_entry::<SequenceSnapshot, export::json::v_rfc8259::any::SequenceIntoJson>(SEQUENCE_DIALECT),
                    deserializer_entry::<SequenceSnapshot, import::json::v_rfc8259::any::JsonIntoSequence>(SEQUENCE_DIALECT),
                    serializer_entry::<SequenceSnapshot, export::txt::v_utf_8::any::SequenceIntoTxt>(SEQUENCE_DIALECT),
                    deserializer_entry::<SequenceSnapshot, import::txt::v_utf_8::any::TxtIntoSequence>(SEQUENCE_DIALECT),
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
            codec: store::ArtifactCodec::of::<SequenceSnapshot, SequenceMutation>(SEQUENCE_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
