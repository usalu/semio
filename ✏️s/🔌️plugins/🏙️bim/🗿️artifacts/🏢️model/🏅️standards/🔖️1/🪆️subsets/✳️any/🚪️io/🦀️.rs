//! 🚪️ IO of `s.bim.model@1/*`: the native pack (binary) and DSL (text) carriers of snapshot, diff, mutations and inferences.
//! Foreign formats (IFC, glTF, SVG) arrive as `📤️export` / `📥️import` leaves of this tree.

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "📤️export/🦀️.rs"]
pub mod export;

#[path = "📥️import/🦀️.rs"]
pub mod import;

//#region 🔖️IoDeclaration
/// 🚪️ The subset's io surface: every foreign-dialect hop, one `serializer_entry` / `deserializer_entry` per leaf.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::{ModelMutation, ModelSnapshot, BIM_MODEL_DIALECT, BIM_MODEL_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<ModelSnapshot, export::ifc::ModelIntoIfc2x3>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, export::gltf::ModelIntoGlb>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, export::svg::ModelIntoSvg>(BIM_MODEL_DIALECT),
                    deserializer_entry::<ModelSnapshot, import::ifc::IfcIntoModel>(BIM_MODEL_DIALECT),
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
            codec: store::ArtifactCodec::bare::<ModelSnapshot, ModelMutation>(BIM_MODEL_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
