//! 🌡️ `set-space-conditions` payload. Sparsely sets the thermal conditions of a space (occupancy type and density, heating and cooling set points, ventilation, lighting and equipment power density, schedule profile); the first use creates the record of the space, an assigned null clears a field. The envelope, the areas and the U-values are inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{Assigned, SpaceConditionsPatch};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSpaceConditions {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub occupancy: Option<Assigned<Option<String>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub occupancy_density: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub heating_setpoint: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub cooling_setpoint: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ventilation_rate: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lighting_power_density: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub equipment_power_density: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Assigned<Option<String>>>,
}

impl SetSpaceConditions {
    /// 🌡️ The payload that states every field of `record` (an unstated field as an assigned null): applied to a space without conditions it recreates the record exactly.
    pub fn stating(id: &str, record: &crate::SpaceConditions) -> Self {
        Self {
            id: id.to_string(),
            occupancy: Some(Assigned::new(record.occupancy.clone())),
            occupancy_density: Some(Assigned::new(record.occupancy_density.clone())),
            heating_setpoint: Some(Assigned::new(record.heating_setpoint.clone())),
            cooling_setpoint: Some(Assigned::new(record.cooling_setpoint.clone())),
            ventilation_rate: Some(Assigned::new(record.ventilation_rate.clone())),
            lighting_power_density: Some(Assigned::new(record.lighting_power_density.clone())),
            equipment_power_density: Some(Assigned::new(record.equipment_power_density.clone())),
            schedule: Some(Assigned::new(record.schedule.clone())),
        }
    }
}

impl SetSpaceConditions {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SpaceConditionsPatch {
        SpaceConditionsPatch { occupancy: self.occupancy.clone(), occupancy_density: self.occupancy_density.clone(), heating_setpoint: self.heating_setpoint.clone(), cooling_setpoint: self.cooling_setpoint.clone(), ventilation_rate: self.ventilation_rate.clone(), lighting_power_density: self.lighting_power_density.clone(), equipment_power_density: self.equipment_power_density.clone(), schedule: self.schedule.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SpaceConditionsPatch) -> Self {
        Self { id, occupancy: patch.occupancy, occupancy_density: patch.occupancy_density, heating_setpoint: patch.heating_setpoint, cooling_setpoint: patch.cooling_setpoint, ventilation_rate: patch.ventilation_rate, lighting_power_density: patch.lighting_power_density, equipment_power_density: patch.equipment_power_density, schedule: patch.schedule }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetSpaceConditions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "space-conditions", kind: "set-space-conditions", record: "SetSpaceConditions" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set conditions of space \"{}\"", self.id), &format!("Bedingungen des Raums \"{}\" setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
