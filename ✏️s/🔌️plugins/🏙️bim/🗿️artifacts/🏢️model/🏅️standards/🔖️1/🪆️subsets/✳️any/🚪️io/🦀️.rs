//! 🚪️ IO of `s.bim.model@1/*`: the native pack (binary) and DSL (text) carriers of snapshot, diff, mutations and inferences.
//! Foreign formats (IFC, glTF, SVG) arrive as `📤️export` / `📥️import` leaves of this tree.





#[path = "📤️export/🦀️.rs"]
pub mod export;



//#region 🔖️Inference
/// 🔮️ Reads the inference of `model` from the shared session (the one path every consumer takes), so an export after an edit recomputes only what the edit touched; a fault of the run is a refusal of `leaf`.
pub fn with_inferred<R>(leaf: &str, model: &semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot, read: impl FnOnce(&semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference) -> R) -> Result<R, semio_framework::io_schema::IoError> {
    use semio_framework_value::{ValueError, ValueRefusalKind};
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(None, model, read).map_err(|error| semio_framework::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, format!("{leaf}: {error}"))))
}
//#endregion 🔖️Inference

//#region 🔖️IoDeclaration
/// 🚪️ The subset's io surface: every foreign-dialect hop, one `serializer_entry` / `deserializer_entry` per leaf.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::{BIM_MODEL_DIALECT, BIM_MODEL_DOCUMENT_SCHEMA};
    use semio_framework_os_kernel::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::ModelIntoIfc2x3>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::ifc4::ModelIntoIfc4>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::io::export::gltf::ModelIntoGlb>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::ModelIntoSvg>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, export::csv::ModelIntoCsv>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::json::ModelIntoJson>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::ModelIntoSheetsPdf>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::ModelIntoGbxml>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::energy::ModelIntoEnergy>(BIM_MODEL_DIALECT),
                    serializer_entry::<ModelSnapshot, bcf::ModelIntoBcf>(BIM_MODEL_DIALECT),
                    deserializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::Ifc2x3IntoModel>(BIM_MODEL_DIALECT),
                    deserializer_entry::<ModelSnapshot, semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::Ifc4IntoModel>(BIM_MODEL_DIALECT),
                    deserializer_entry::<ModelSnapshot, bcf::BcfIntoModel>(BIM_MODEL_DIALECT),
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
            codec: store::bare_artifact_codec::<ModelSnapshot, ModelMutation>(BIM_MODEL_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration


