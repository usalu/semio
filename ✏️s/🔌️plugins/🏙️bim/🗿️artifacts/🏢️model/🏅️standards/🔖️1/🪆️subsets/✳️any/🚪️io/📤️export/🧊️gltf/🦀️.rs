//! 🧊️ `s.bim.model@1/*` → `s.stdio.gltf@2.0/*`: the element solids of a [`ModelSnapshot`] as a binary glTF 2.0 scene.
//! One node per element (named by its name or id, `extras` carry id, kind and storey) under the site, building and storey nodes; one mesh per element with one primitive per material;
//! PBR materials from the model materials (translucent glazing); Z-up model coordinates converted to glTF's Y-up. The solids are the ones of the `element-solids` inference.
//! 🔖 `IoFidelity::Lossy`: glTF has no slot for parametric constraints, types, layers, property sets or quantities; geometry, hierarchy, names and colours are exact.
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html

use crate::ModelSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

#[path = "🧱️codec/🦀️.rs"]
pub mod codec;
#[path = "🧬️document/🦀️.rs"]
pub mod document;
#[path = "🎨️materials/🦀️.rs"]
pub mod materials;
#[path = "🌳️scene/🦀️.rs"]
pub mod scene;
#[path = "📏️projection/🦀️.rs"]
pub mod projection;

/// 🪪️ The glTF 2.0 dialect this leaf writes.
pub const GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId::ANY };

/// 🧊️ The glTF model of `model` plus a note per element that could not be placed: the inference comes from the shared session.
pub fn model_to_gltf(model: &ModelSnapshot) -> Result<(document::GltfModel, Vec<String>), String> {
    crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(None, model, |inferred| scene::build(model, inferred)).map_err(|error| error.to_string())
}

/// 📤️ The GLB bytes of `model` plus a note per element that could not be placed.
pub fn export_glb(model: &ModelSnapshot) -> Result<(Vec<u8>, Vec<String>), String> {
    let (gltf, notes) = model_to_gltf(model)?;
    Ok((gltf.to_glb()?, notes))
}

//#region 🔖️Serializer
/// 🧊️ The glTF serializer of the BIM model.
pub struct ModelIntoGlb;

impl Serializer<ModelSnapshot> for ModelIntoGlb {
    const INTO: Dialect = GLTF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let (bytes, notes) = export_glb(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ModelIntoGlb: {message}"))))?;
        if bytes.len() > u32::MAX as usize {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ModelIntoGlb: the scene exceeds the 4 GiB limit of a binary glTF")));
        }
        let diagnostics = notes
            .into_iter()
            .map(|note| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new("bim.gltf.export.skipped"), severity: semio_framework_diagnostic::Severity::Warning, span: Default::default(), message: note, expected: None, scope: Default::default() })
            .collect();
        Ok(IoOutcome { value: IoPayload::Binary(bytes), diagnostics })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
