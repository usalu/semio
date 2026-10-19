//! 🏷️ Templates and families: the sparse patches of the property template, classification system, space conditions, family, family parameter and family solid. See [`super`].

use super::*;

/// 🩹 Sparse patch of [`PropertyTemplate`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct PropertyTemplatePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<Vec<TemplateTarget>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<PropertyDef>>,
}

impl Patch<PropertyTemplate> for PropertyTemplatePatch {
    fn write(&self, base: &PropertyTemplate) -> PropertyTemplate {
        PropertyTemplate { name: take(&self.name, &base.name), applies_to: take(&self.applies_to, &base.applies_to), properties: take(&self.properties, &base.properties) }
    }
    fn restoring(&self, base: &PropertyTemplate) -> Self {
        Self { name: restore(&self.name, &base.name), applies_to: restore(&self.applies_to, &base.applies_to), properties: restore(&self.properties, &base.properties) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.applies_to, later.applies_to);
        merge_slot(&mut self.properties, later.properties);
    }
    fn minimal(&self, base: &PropertyTemplate) -> Self {
        Self { name: changed(&self.name, &base.name), applies_to: changed(&self.applies_to, &base.applies_to), properties: changed(&self.properties, &base.properties) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.applies_to.is_some() {
            out.push("applies_to".to_string());
        }
        if self.properties.is_some() {
            out.push("properties".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`ClassificationSystem`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ClassificationSystemPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub source: Option<Assigned<Option<String>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<ClassificationItem>>,
}

impl Patch<ClassificationSystem> for ClassificationSystemPatch {
    fn write(&self, base: &ClassificationSystem) -> ClassificationSystem {
        ClassificationSystem { name: take(&self.name, &base.name), edition: take(&self.edition, &base.edition), source: take_assigned(&self.source, &base.source), entries: take(&self.entries, &base.entries) }
    }
    fn restoring(&self, base: &ClassificationSystem) -> Self {
        Self { name: restore(&self.name, &base.name), edition: restore(&self.edition, &base.edition), source: restore_assigned(&self.source, &base.source), entries: restore(&self.entries, &base.entries) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.edition, later.edition);
        merge_slot(&mut self.source, later.source);
        merge_slot(&mut self.entries, later.entries);
    }
    fn minimal(&self, base: &ClassificationSystem) -> Self {
        Self { name: changed(&self.name, &base.name), edition: changed(&self.edition, &base.edition), source: changed_assigned(&self.source, &base.source), entries: changed(&self.entries, &base.entries) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.edition.is_some() {
            out.push("edition".to_string());
        }
        if self.source.is_some() {
            out.push("source".to_string());
        }
        if self.entries.is_some() {
            out.push("entries".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`SpaceConditions`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SpaceConditionsPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub occupancy: Option<Assigned<Option<String>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub occupancy_density: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub heating_setpoint: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub cooling_setpoint: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub ventilation_rate: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub lighting_power_density: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub equipment_power_density: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Assigned<Option<String>>>,
}

impl Patch<SpaceConditions> for SpaceConditionsPatch {
    fn write(&self, base: &SpaceConditions) -> SpaceConditions {
        SpaceConditions { occupancy: take_assigned(&self.occupancy, &base.occupancy), occupancy_density: take_assigned(&self.occupancy_density, &base.occupancy_density), heating_setpoint: take_assigned(&self.heating_setpoint, &base.heating_setpoint), cooling_setpoint: take_assigned(&self.cooling_setpoint, &base.cooling_setpoint), ventilation_rate: take_assigned(&self.ventilation_rate, &base.ventilation_rate), lighting_power_density: take_assigned(&self.lighting_power_density, &base.lighting_power_density), equipment_power_density: take_assigned(&self.equipment_power_density, &base.equipment_power_density), schedule: take_assigned(&self.schedule, &base.schedule) }
    }
    fn restoring(&self, base: &SpaceConditions) -> Self {
        Self { occupancy: restore_assigned(&self.occupancy, &base.occupancy), occupancy_density: restore_assigned(&self.occupancy_density, &base.occupancy_density), heating_setpoint: restore_assigned(&self.heating_setpoint, &base.heating_setpoint), cooling_setpoint: restore_assigned(&self.cooling_setpoint, &base.cooling_setpoint), ventilation_rate: restore_assigned(&self.ventilation_rate, &base.ventilation_rate), lighting_power_density: restore_assigned(&self.lighting_power_density, &base.lighting_power_density), equipment_power_density: restore_assigned(&self.equipment_power_density, &base.equipment_power_density), schedule: restore_assigned(&self.schedule, &base.schedule) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.occupancy, later.occupancy);
        merge_slot(&mut self.occupancy_density, later.occupancy_density);
        merge_slot(&mut self.heating_setpoint, later.heating_setpoint);
        merge_slot(&mut self.cooling_setpoint, later.cooling_setpoint);
        merge_slot(&mut self.ventilation_rate, later.ventilation_rate);
        merge_slot(&mut self.lighting_power_density, later.lighting_power_density);
        merge_slot(&mut self.equipment_power_density, later.equipment_power_density);
        merge_slot(&mut self.schedule, later.schedule);
    }
    fn minimal(&self, base: &SpaceConditions) -> Self {
        Self { occupancy: changed_assigned(&self.occupancy, &base.occupancy), occupancy_density: changed_assigned(&self.occupancy_density, &base.occupancy_density), heating_setpoint: changed_assigned(&self.heating_setpoint, &base.heating_setpoint), cooling_setpoint: changed_assigned(&self.cooling_setpoint, &base.cooling_setpoint), ventilation_rate: changed_assigned(&self.ventilation_rate, &base.ventilation_rate), lighting_power_density: changed_assigned(&self.lighting_power_density, &base.lighting_power_density), equipment_power_density: changed_assigned(&self.equipment_power_density, &base.equipment_power_density), schedule: changed_assigned(&self.schedule, &base.schedule) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.occupancy.is_some() {
            out.push("occupancy".to_string());
        }
        if self.occupancy_density.is_some() {
            out.push("occupancy_density".to_string());
        }
        if self.heating_setpoint.is_some() {
            out.push("heating_setpoint".to_string());
        }
        if self.cooling_setpoint.is_some() {
            out.push("cooling_setpoint".to_string());
        }
        if self.ventilation_rate.is_some() {
            out.push("ventilation_rate".to_string());
        }
        if self.lighting_power_density.is_some() {
            out.push("lighting_power_density".to_string());
        }
        if self.equipment_power_density.is_some() {
            out.push("equipment_power_density".to_string());
        }
        if self.schedule.is_some() {
            out.push("schedule".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Family`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct FamilyPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<FamilyCategory>,
}

impl Patch<Family> for FamilyPatch {
    fn write(&self, base: &Family) -> Family {
        Family { name: take(&self.name, &base.name), category: take(&self.category, &base.category) }
    }
    fn restoring(&self, base: &Family) -> Self {
        Self { name: restore(&self.name, &base.name), category: restore(&self.category, &base.category) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.category, later.category);
    }
    fn minimal(&self, base: &Family) -> Self {
        Self { name: changed(&self.name, &base.name), category: changed(&self.category, &base.category) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.category.is_some() {
            out.push("category".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`FamilyParameter`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct FamilyParameterPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ParameterKind>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Patch<FamilyParameter> for FamilyParameterPatch {
    fn write(&self, base: &FamilyParameter) -> FamilyParameter {
        FamilyParameter { family: take(&self.family, &base.family), name: take(&self.name, &base.name), kind: take(&self.kind, &base.kind), value: take(&self.value, &base.value) }
    }
    fn restoring(&self, base: &FamilyParameter) -> Self {
        Self { family: restore(&self.family, &base.family), name: restore(&self.name, &base.name), kind: restore(&self.kind, &base.kind), value: restore(&self.value, &base.value) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.family, later.family);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.kind, later.kind);
        merge_slot(&mut self.value, later.value);
    }
    fn minimal(&self, base: &FamilyParameter) -> Self {
        Self { family: changed(&self.family, &base.family), name: changed(&self.name, &base.name), kind: changed(&self.kind, &base.kind), value: changed(&self.value, &base.value) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.family.is_some() {
            out.push("family".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.kind.is_some() {
            out.push("kind".to_string());
        }
        if self.value.is_some() {
            out.push("value".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`FamilySolid`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct FamilySolidPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub shape: Option<SolidShape>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub visible: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<ExprPoint3>,
}

impl Patch<FamilySolid> for FamilySolidPatch {
    fn write(&self, base: &FamilySolid) -> FamilySolid {
        FamilySolid { family: take(&self.family, &base.family), name: take(&self.name, &base.name), shape: take(&self.shape, &base.shape), material: take(&self.material, &base.material), visible: take(&self.visible, &base.visible), offset: take(&self.offset, &base.offset) }
    }
    fn restoring(&self, base: &FamilySolid) -> Self {
        Self { family: restore(&self.family, &base.family), name: restore(&self.name, &base.name), shape: restore(&self.shape, &base.shape), material: restore(&self.material, &base.material), visible: restore(&self.visible, &base.visible), offset: restore(&self.offset, &base.offset) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.family, later.family);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.shape, later.shape);
        merge_slot(&mut self.material, later.material);
        merge_slot(&mut self.visible, later.visible);
        merge_slot(&mut self.offset, later.offset);
    }
    fn minimal(&self, base: &FamilySolid) -> Self {
        Self { family: changed(&self.family, &base.family), name: changed(&self.name, &base.name), shape: changed(&self.shape, &base.shape), material: changed(&self.material, &base.material), visible: changed(&self.visible, &base.visible), offset: changed(&self.offset, &base.offset) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.family.is_some() {
            out.push("family".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.shape.is_some() {
            out.push("shape".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        if self.visible.is_some() {
            out.push("visible".to_string());
        }
        if self.offset.is_some() {
            out.push("offset".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
