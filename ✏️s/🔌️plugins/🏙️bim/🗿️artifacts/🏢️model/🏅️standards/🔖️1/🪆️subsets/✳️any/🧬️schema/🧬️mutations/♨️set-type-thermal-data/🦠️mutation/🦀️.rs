//! ♨️ `set-type-thermal-data` payload. Sparsely sets the thermal data of a window type (U-value of the whole window, g-value of the glazing, frame fraction), of a curtain wall type (the same three for the whole facade) or of a door type (U-value); an assigned null clears a field. The envelope U-values and solar gains are inferred from them.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{Assigned, CurtainWallTypePatch, DoorTypePatch, WindowTypePatch};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTypeThermalData {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub u_value: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub g_value: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frame_fraction: Option<Assigned<Option<f64>>>,
}

impl SetTypeThermalData {
    /// 🩹 The sparse window type patch this payload names: every provided field, restated values included.
    pub fn window_patch(&self) -> WindowTypePatch {
        WindowTypePatch { u_value: self.u_value.clone(), g_value: self.g_value.clone(), frame_fraction: self.frame_fraction.clone(), ..Default::default() }
    }

    /// 🩹 The sparse curtain wall type patch this payload names: every provided field, restated values included.
    pub fn curtain_patch(&self) -> CurtainWallTypePatch {
        CurtainWallTypePatch { u_value: self.u_value.clone(), g_value: self.g_value.clone(), frame_fraction: self.frame_fraction.clone(), ..Default::default() }
    }

    /// 🩹 The sparse door type patch this payload names: the U-value, when provided.
    pub fn door_patch(&self) -> DoorTypePatch {
        DoorTypePatch { u_value: self.u_value.clone(), ..Default::default() }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetTypeThermalData {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "type-thermal-data", kind: "set-type-thermal-data", record: "SetTypeThermalData" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set thermal data of type \"{}\"", self.id), &format!("Thermische Daten des Typs \"{}\" setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
