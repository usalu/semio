//! 🔭️ `set-view` payload. Sparsely changes a view: name, storey of a plan, plane of a section or elevation, camera, cut height (an assigned null returns to the convention), depth, crop (an assigned null clears it), hidden categories, phase filter (an assigned null shows every phase), scale and detail level. The kind and the building of a view never change.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{Assigned, DetailLevel, Phase, ViewCamera, ViewCategory, ViewCrop, ViewPatch, ViewPlane};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetView {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub plane: Option<ViewPlane>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<ViewCamera>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cut_height: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop: Option<Assigned<Option<ViewCrop>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<Vec<ViewCategory>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<Assigned<Option<Phase>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<DetailLevel>,
}

impl SetView {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ViewPatch {
        ViewPatch {
            name: self.name.clone(),
            storey: self.storey.clone().map(|storey| Assigned::new(Some(storey))),
            plane: self.plane.map(|plane| Assigned::new(Some(plane))),
            camera: self.camera.map(|camera| Assigned::new(Some(camera))),
            cut_height: self.cut_height.clone(),
            depth: self.depth,
            crop: self.crop.clone(),
            hidden: self.hidden.clone(),
            phase: self.phase.clone(),
            scale: self.scale,
            detail: self.detail,
            ..Default::default()
        }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: ViewPatch) -> Self {
        Self {
            id,
            name: patch.name,
            storey: patch.storey.and_then(|storey| storey.value),
            plane: patch.plane.and_then(|plane| plane.value),
            camera: patch.camera.and_then(|camera| camera.value),
            cut_height: patch.cut_height,
            depth: patch.depth,
            crop: patch.crop,
            hidden: patch.hidden,
            phase: patch.phase,
            scale: patch.scale,
            detail: patch.detail,
        }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetView {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "view", kind: "set-view", record: "SetView" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change view \"{}\"", self.id), &format!("Ansicht \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
