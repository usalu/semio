//! 🚪️ IO s.wfc.bitmap (1/✳️any) — `io() -> IoDeclaration`: the native codec plus the two foreign
//! hops this artifact really implements, aggregated from the typed `Serializer<BitmapSnapshot>`/
//! `Deserializer<BitmapSnapshot>` leaves under `📥️import/🧩️deserializers` and
//! `📤️export/🧵️serializers`. All io goes through the `io_mechanism` registry — the retired
//! `ComposerEntry`/`io_registry` channel is not used here.
//!
//! Both hops are FULL fidelity and exact inverses: the native serialization already is utf-8 DSL
//! text, and the JSON projection is the same canonical one the committed fixtures are written in.

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt", "stdio.json"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt", "stdio.json"]
}

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{BitmapMutation, BitmapSnapshot, WFC_BITMAP_DIALECT, WFC_BITMAP_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<BitmapSnapshot, export::txt::v_utf_8::any::BitmapIntoTxt>(WFC_BITMAP_DIALECT),
                    deserializer_entry::<BitmapSnapshot, import::txt::v_utf_8::any::TxtIntoBitmap>(WFC_BITMAP_DIALECT),
                    serializer_entry::<BitmapSnapshot, export::json::v_rfc8259::any::BitmapIntoJson>(WFC_BITMAP_DIALECT),
                    deserializer_entry::<BitmapSnapshot, import::json::v_rfc8259::any::JsonIntoBitmap>(WFC_BITMAP_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&crate::pilot_languages()[0]), binary: Some(&crate::pilot_languages()[2]) },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: Some(&crate::pilot_languages()[1]), binary: Some(&crate::pilot_languages()[3]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<BitmapSnapshot, BitmapMutation>(WFC_BITMAP_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
