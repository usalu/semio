//! 📚️ Sparse patches of the library records: property templates, classification systems, space conditions, families, components, MEP elements, materials and the element types.

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
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.applies_to, later.applies_to);
        merge_slot!(&mut self.properties, later.properties);
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
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.edition, later.edition);
        merge_slot!(&mut self.source, later.source);
        merge_slot!(&mut self.entries, later.entries);
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
        merge_slot!(&mut self.occupancy, later.occupancy);
        merge_slot!(&mut self.occupancy_density, later.occupancy_density);
        merge_slot!(&mut self.heating_setpoint, later.heating_setpoint);
        merge_slot!(&mut self.cooling_setpoint, later.cooling_setpoint);
        merge_slot!(&mut self.ventilation_rate, later.ventilation_rate);
        merge_slot!(&mut self.lighting_power_density, later.lighting_power_density);
        merge_slot!(&mut self.equipment_power_density, later.equipment_power_density);
        merge_slot!(&mut self.schedule, later.schedule);
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
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.category, later.category);
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
        merge_slot!(&mut self.family, later.family);
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.kind, later.kind);
        merge_slot!(&mut self.value, later.value);
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
        merge_slot!(&mut self.family, later.family);
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.shape, later.shape);
        merge_slot!(&mut self.material, later.material);
        merge_slot!(&mut self.visible, later.visible);
        merge_slot!(&mut self.offset, later.offset);
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

/// 🩹 Sparse patch of [`Component`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ComponentPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub mirrored: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub host: Option<Assigned<Option<String>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub system: Option<Assigned<Option<MepSystem>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Component> for ComponentPatch {
    fn write(&self, base: &Component) -> Component {
        Component { storey: take(&self.storey, &base.storey), family: take(&self.family, &base.family), position: take(&self.position, &base.position), elevation: take(&self.elevation, &base.elevation), rotation: take(&self.rotation, &base.rotation), mirrored: take(&self.mirrored, &base.mirrored), host: take_assigned(&self.host, &base.host), system: take_assigned(&self.system, &base.system), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Component) -> Self {
        Self { storey: restore(&self.storey, &base.storey), family: restore(&self.family, &base.family), position: restore(&self.position, &base.position), elevation: restore(&self.elevation, &base.elevation), rotation: restore(&self.rotation, &base.rotation), mirrored: restore(&self.mirrored, &base.mirrored), host: restore_assigned(&self.host, &base.host), system: restore_assigned(&self.system, &base.system), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.family, later.family);
        merge_slot!(&mut self.position, later.position);
        merge_slot!(&mut self.elevation, later.elevation);
        merge_slot!(&mut self.rotation, later.rotation);
        merge_slot!(&mut self.mirrored, later.mirrored);
        merge_slot!(&mut self.host, later.host);
        merge_slot!(&mut self.system, later.system);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Component) -> Self {
        Self { storey: changed(&self.storey, &base.storey), family: changed(&self.family, &base.family), position: changed(&self.position, &base.position), elevation: changed(&self.elevation, &base.elevation), rotation: changed(&self.rotation, &base.rotation), mirrored: changed(&self.mirrored, &base.mirrored), host: changed_assigned(&self.host, &base.host), system: changed_assigned(&self.system, &base.system), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.family.is_some() {
            out.push("family".to_string());
        }
        if self.position.is_some() {
            out.push("position".to_string());
        }
        if self.elevation.is_some() {
            out.push("elevation".to_string());
        }
        if self.rotation.is_some() {
            out.push("rotation".to_string());
        }
        if self.mirrored.is_some() {
            out.push("mirrored".to_string());
        }
        if self.host.is_some() {
            out.push("host".to_string());
        }
        if self.system.is_some() {
            out.push("system".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`ComponentOverride`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ComponentOverridePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Patch<ComponentOverride> for ComponentOverridePatch {
    fn write(&self, base: &ComponentOverride) -> ComponentOverride {
        ComponentOverride { component: take(&self.component, &base.component), name: take(&self.name, &base.name), value: take(&self.value, &base.value) }
    }
    fn restoring(&self, base: &ComponentOverride) -> Self {
        Self { component: restore(&self.component, &base.component), name: restore(&self.name, &base.name), value: restore(&self.value, &base.value) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.component, later.component);
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.value, later.value);
    }
    fn minimal(&self, base: &ComponentOverride) -> Self {
        Self { component: changed(&self.component, &base.component), name: changed(&self.name, &base.name), value: changed(&self.value, &base.value) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.component.is_some() {
            out.push("component".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
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

/// 🩹 Sparse patch of [`MepElement`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct MepElementPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub system: Option<MepSystem>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub shape: Option<MepShape>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Point3>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<MepElement> for MepElementPatch {
    fn write(&self, base: &MepElement) -> MepElement {
        MepElement { storey: take(&self.storey, &base.storey), system: take(&self.system, &base.system), shape: take(&self.shape, &base.shape), path: take(&self.path, &base.path), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &MepElement) -> Self {
        Self { storey: restore(&self.storey, &base.storey), system: restore(&self.system, &base.system), shape: restore(&self.shape, &base.shape), path: restore(&self.path, &base.path), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.system, later.system);
        merge_slot!(&mut self.shape, later.shape);
        merge_slot!(&mut self.path, later.path);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &MepElement) -> Self {
        Self { storey: changed(&self.storey, &base.storey), system: changed(&self.system, &base.system), shape: changed(&self.shape, &base.shape), path: changed(&self.path, &base.path), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.system.is_some() {
            out.push("system".to_string());
        }
        if self.shape.is_some() {
            out.push("shape".to_string());
        }
        if self.path.is_some() {
            out.push("path".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Material`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct MaterialPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<MaterialCategory>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub color: Option<Rgb>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub conductivity: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub specific_heat: Option<f64>,
}

impl Patch<Material> for MaterialPatch {
    fn write(&self, base: &Material) -> Material {
        Material { name: take(&self.name, &base.name), category: take(&self.category, &base.category), color: take(&self.color, &base.color), density: take(&self.density, &base.density), conductivity: take(&self.conductivity, &base.conductivity), specific_heat: take(&self.specific_heat, &base.specific_heat) }
    }
    fn restoring(&self, base: &Material) -> Self {
        Self { name: restore(&self.name, &base.name), category: restore(&self.category, &base.category), color: restore(&self.color, &base.color), density: restore(&self.density, &base.density), conductivity: restore(&self.conductivity, &base.conductivity), specific_heat: restore(&self.specific_heat, &base.specific_heat) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.category, later.category);
        merge_slot!(&mut self.color, later.color);
        merge_slot!(&mut self.density, later.density);
        merge_slot!(&mut self.conductivity, later.conductivity);
        merge_slot!(&mut self.specific_heat, later.specific_heat);
    }
    fn minimal(&self, base: &Material) -> Self {
        Self { name: changed(&self.name, &base.name), category: changed(&self.category, &base.category), color: changed(&self.color, &base.color), density: changed(&self.density, &base.density), conductivity: changed(&self.conductivity, &base.conductivity), specific_heat: changed(&self.specific_heat, &base.specific_heat) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.category.is_some() {
            out.push("category".to_string());
        }
        if self.color.is_some() {
            out.push("color".to_string());
        }
        if self.density.is_some() {
            out.push("density".to_string());
        }
        if self.conductivity.is_some() {
            out.push("conductivity".to_string());
        }
        if self.specific_heat.is_some() {
            out.push("specific_heat".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`WallType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct WallTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl Patch<WallType> for WallTypePatch {
    fn write(&self, base: &WallType) -> WallType {
        WallType { name: take(&self.name, &base.name), layers: take(&self.layers, &base.layers) }
    }
    fn restoring(&self, base: &WallType) -> Self {
        Self { name: restore(&self.name, &base.name), layers: restore(&self.layers, &base.layers) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.layers, later.layers);
    }
    fn minimal(&self, base: &WallType) -> Self {
        Self { name: changed(&self.name, &base.name), layers: changed(&self.layers, &base.layers) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.layers.is_some() {
            out.push("layers".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`SlabType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SlabTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl Patch<SlabType> for SlabTypePatch {
    fn write(&self, base: &SlabType) -> SlabType {
        SlabType { name: take(&self.name, &base.name), layers: take(&self.layers, &base.layers) }
    }
    fn restoring(&self, base: &SlabType) -> Self {
        Self { name: restore(&self.name, &base.name), layers: restore(&self.layers, &base.layers) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.layers, later.layers);
    }
    fn minimal(&self, base: &SlabType) -> Self {
        Self { name: changed(&self.name, &base.name), layers: changed(&self.layers, &base.layers) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.layers.is_some() {
            out.push("layers".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`CeilingType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct CeilingTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl Patch<CeilingType> for CeilingTypePatch {
    fn write(&self, base: &CeilingType) -> CeilingType {
        CeilingType { name: take(&self.name, &base.name), layers: take(&self.layers, &base.layers) }
    }
    fn restoring(&self, base: &CeilingType) -> Self {
        Self { name: restore(&self.name, &base.name), layers: restore(&self.layers, &base.layers) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.layers, later.layers);
    }
    fn minimal(&self, base: &CeilingType) -> Self {
        Self { name: changed(&self.name, &base.name), layers: changed(&self.layers, &base.layers) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.layers.is_some() {
            out.push("layers".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`RoofType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct RoofTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

impl Patch<RoofType> for RoofTypePatch {
    fn write(&self, base: &RoofType) -> RoofType {
        RoofType { name: take(&self.name, &base.name), layers: take(&self.layers, &base.layers) }
    }
    fn restoring(&self, base: &RoofType) -> Self {
        Self { name: restore(&self.name, &base.name), layers: restore(&self.layers, &base.layers) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.layers, later.layers);
    }
    fn minimal(&self, base: &RoofType) -> Self {
        Self { name: changed(&self.name, &base.name), layers: changed(&self.layers, &base.layers) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.layers.is_some() {
            out.push("layers".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`ColumnType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ColumnTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl Patch<ColumnType> for ColumnTypePatch {
    fn write(&self, base: &ColumnType) -> ColumnType {
        ColumnType { name: take(&self.name, &base.name), profile: take(&self.profile, &base.profile), material: take(&self.material, &base.material) }
    }
    fn restoring(&self, base: &ColumnType) -> Self {
        Self { name: restore(&self.name, &base.name), profile: restore(&self.profile, &base.profile), material: restore(&self.material, &base.material) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.profile, later.profile);
        merge_slot!(&mut self.material, later.material);
    }
    fn minimal(&self, base: &ColumnType) -> Self {
        Self { name: changed(&self.name, &base.name), profile: changed(&self.profile, &base.profile), material: changed(&self.material, &base.material) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.profile.is_some() {
            out.push("profile".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`BeamType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct BeamTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

impl Patch<BeamType> for BeamTypePatch {
    fn write(&self, base: &BeamType) -> BeamType {
        BeamType { name: take(&self.name, &base.name), profile: take(&self.profile, &base.profile), material: take(&self.material, &base.material) }
    }
    fn restoring(&self, base: &BeamType) -> Self {
        Self { name: restore(&self.name, &base.name), profile: restore(&self.profile, &base.profile), material: restore(&self.material, &base.material) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.profile, later.profile);
        merge_slot!(&mut self.material, later.material);
    }
    fn minimal(&self, base: &BeamType) -> Self {
        Self { name: changed(&self.name, &base.name), profile: changed(&self.profile, &base.profile), material: changed(&self.material, &base.material) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.profile.is_some() {
            out.push("profile".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`WindowType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct WindowTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sill: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub frame_depth: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub panes: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub u_value: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub g_value: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub frame_fraction: Option<Assigned<Option<f64>>>,
}

impl Patch<WindowType> for WindowTypePatch {
    fn write(&self, base: &WindowType) -> WindowType {
        WindowType { name: take(&self.name, &base.name), width: take(&self.width, &base.width), height: take(&self.height, &base.height), sill: take(&self.sill, &base.sill), frame_width: take(&self.frame_width, &base.frame_width), frame_depth: take(&self.frame_depth, &base.frame_depth), panes: take(&self.panes, &base.panes), material: take(&self.material, &base.material), u_value: take_assigned(&self.u_value, &base.u_value), g_value: take_assigned(&self.g_value, &base.g_value), frame_fraction: take_assigned(&self.frame_fraction, &base.frame_fraction) }
    }
    fn restoring(&self, base: &WindowType) -> Self {
        Self { name: restore(&self.name, &base.name), width: restore(&self.width, &base.width), height: restore(&self.height, &base.height), sill: restore(&self.sill, &base.sill), frame_width: restore(&self.frame_width, &base.frame_width), frame_depth: restore(&self.frame_depth, &base.frame_depth), panes: restore(&self.panes, &base.panes), material: restore(&self.material, &base.material), u_value: restore_assigned(&self.u_value, &base.u_value), g_value: restore_assigned(&self.g_value, &base.g_value), frame_fraction: restore_assigned(&self.frame_fraction, &base.frame_fraction) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.width, later.width);
        merge_slot!(&mut self.height, later.height);
        merge_slot!(&mut self.sill, later.sill);
        merge_slot!(&mut self.frame_width, later.frame_width);
        merge_slot!(&mut self.frame_depth, later.frame_depth);
        merge_slot!(&mut self.panes, later.panes);
        merge_slot!(&mut self.material, later.material);
        merge_slot!(&mut self.u_value, later.u_value);
        merge_slot!(&mut self.g_value, later.g_value);
        merge_slot!(&mut self.frame_fraction, later.frame_fraction);
    }
    fn minimal(&self, base: &WindowType) -> Self {
        Self { name: changed(&self.name, &base.name), width: changed(&self.width, &base.width), height: changed(&self.height, &base.height), sill: changed(&self.sill, &base.sill), frame_width: changed(&self.frame_width, &base.frame_width), frame_depth: changed(&self.frame_depth, &base.frame_depth), panes: changed(&self.panes, &base.panes), material: changed(&self.material, &base.material), u_value: changed_assigned(&self.u_value, &base.u_value), g_value: changed_assigned(&self.g_value, &base.g_value), frame_fraction: changed_assigned(&self.frame_fraction, &base.frame_fraction) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.width.is_some() {
            out.push("width".to_string());
        }
        if self.height.is_some() {
            out.push("height".to_string());
        }
        if self.sill.is_some() {
            out.push("sill".to_string());
        }
        if self.frame_width.is_some() {
            out.push("frame_width".to_string());
        }
        if self.frame_depth.is_some() {
            out.push("frame_depth".to_string());
        }
        if self.panes.is_some() {
            out.push("panes".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        if self.u_value.is_some() {
            out.push("u_value".to_string());
        }
        if self.g_value.is_some() {
            out.push("g_value".to_string());
        }
        if self.frame_fraction.is_some() {
            out.push("frame_fraction".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`DoorType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct DoorTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub frame_depth: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub leaves: Option<DoorLeaves>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub swing: Option<Swing>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub u_value: Option<Assigned<Option<f64>>>,
}

impl Patch<DoorType> for DoorTypePatch {
    fn write(&self, base: &DoorType) -> DoorType {
        DoorType { name: take(&self.name, &base.name), width: take(&self.width, &base.width), height: take(&self.height, &base.height), frame_width: take(&self.frame_width, &base.frame_width), frame_depth: take(&self.frame_depth, &base.frame_depth), leaves: take(&self.leaves, &base.leaves), swing: take(&self.swing, &base.swing), material: take(&self.material, &base.material), u_value: take_assigned(&self.u_value, &base.u_value) }
    }
    fn restoring(&self, base: &DoorType) -> Self {
        Self { name: restore(&self.name, &base.name), width: restore(&self.width, &base.width), height: restore(&self.height, &base.height), frame_width: restore(&self.frame_width, &base.frame_width), frame_depth: restore(&self.frame_depth, &base.frame_depth), leaves: restore(&self.leaves, &base.leaves), swing: restore(&self.swing, &base.swing), material: restore(&self.material, &base.material), u_value: restore_assigned(&self.u_value, &base.u_value) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.width, later.width);
        merge_slot!(&mut self.height, later.height);
        merge_slot!(&mut self.frame_width, later.frame_width);
        merge_slot!(&mut self.frame_depth, later.frame_depth);
        merge_slot!(&mut self.leaves, later.leaves);
        merge_slot!(&mut self.swing, later.swing);
        merge_slot!(&mut self.material, later.material);
        merge_slot!(&mut self.u_value, later.u_value);
    }
    fn minimal(&self, base: &DoorType) -> Self {
        Self { name: changed(&self.name, &base.name), width: changed(&self.width, &base.width), height: changed(&self.height, &base.height), frame_width: changed(&self.frame_width, &base.frame_width), frame_depth: changed(&self.frame_depth, &base.frame_depth), leaves: changed(&self.leaves, &base.leaves), swing: changed(&self.swing, &base.swing), material: changed(&self.material, &base.material), u_value: changed_assigned(&self.u_value, &base.u_value) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.width.is_some() {
            out.push("width".to_string());
        }
        if self.height.is_some() {
            out.push("height".to_string());
        }
        if self.frame_width.is_some() {
            out.push("frame_width".to_string());
        }
        if self.frame_depth.is_some() {
            out.push("frame_depth".to_string());
        }
        if self.leaves.is_some() {
            out.push("leaves".to_string());
        }
        if self.swing.is_some() {
            out.push("swing".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        if self.u_value.is_some() {
            out.push("u_value".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`CurtainWallType`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct CurtainWallTypePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub u_grid: Option<CurtainGrid>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub v_grid: Option<CurtainGrid>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub interior_mullion: Option<Profile>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub border_mullion: Option<Profile>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub panel: Option<CurtainPanel>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub panel_material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub mullion_material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub u_value: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub g_value: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub frame_fraction: Option<Assigned<Option<f64>>>,
}

impl Patch<CurtainWallType> for CurtainWallTypePatch {
    fn write(&self, base: &CurtainWallType) -> CurtainWallType {
        CurtainWallType { name: take(&self.name, &base.name), u_grid: take(&self.u_grid, &base.u_grid), v_grid: take(&self.v_grid, &base.v_grid), interior_mullion: take(&self.interior_mullion, &base.interior_mullion), border_mullion: take(&self.border_mullion, &base.border_mullion), panel: take(&self.panel, &base.panel), panel_material: take(&self.panel_material, &base.panel_material), mullion_material: take(&self.mullion_material, &base.mullion_material), u_value: take_assigned(&self.u_value, &base.u_value), g_value: take_assigned(&self.g_value, &base.g_value), frame_fraction: take_assigned(&self.frame_fraction, &base.frame_fraction) }
    }
    fn restoring(&self, base: &CurtainWallType) -> Self {
        Self { name: restore(&self.name, &base.name), u_grid: restore(&self.u_grid, &base.u_grid), v_grid: restore(&self.v_grid, &base.v_grid), interior_mullion: restore(&self.interior_mullion, &base.interior_mullion), border_mullion: restore(&self.border_mullion, &base.border_mullion), panel: restore(&self.panel, &base.panel), panel_material: restore(&self.panel_material, &base.panel_material), mullion_material: restore(&self.mullion_material, &base.mullion_material), u_value: restore_assigned(&self.u_value, &base.u_value), g_value: restore_assigned(&self.g_value, &base.g_value), frame_fraction: restore_assigned(&self.frame_fraction, &base.frame_fraction) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.u_grid, later.u_grid);
        merge_slot!(&mut self.v_grid, later.v_grid);
        merge_slot!(&mut self.interior_mullion, later.interior_mullion);
        merge_slot!(&mut self.border_mullion, later.border_mullion);
        merge_slot!(&mut self.panel, later.panel);
        merge_slot!(&mut self.panel_material, later.panel_material);
        merge_slot!(&mut self.mullion_material, later.mullion_material);
        merge_slot!(&mut self.u_value, later.u_value);
        merge_slot!(&mut self.g_value, later.g_value);
        merge_slot!(&mut self.frame_fraction, later.frame_fraction);
    }
    fn minimal(&self, base: &CurtainWallType) -> Self {
        Self { name: changed(&self.name, &base.name), u_grid: changed(&self.u_grid, &base.u_grid), v_grid: changed(&self.v_grid, &base.v_grid), interior_mullion: changed(&self.interior_mullion, &base.interior_mullion), border_mullion: changed(&self.border_mullion, &base.border_mullion), panel: changed(&self.panel, &base.panel), panel_material: changed(&self.panel_material, &base.panel_material), mullion_material: changed(&self.mullion_material, &base.mullion_material), u_value: changed_assigned(&self.u_value, &base.u_value), g_value: changed_assigned(&self.g_value, &base.g_value), frame_fraction: changed_assigned(&self.frame_fraction, &base.frame_fraction) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.u_grid.is_some() {
            out.push("u_grid".to_string());
        }
        if self.v_grid.is_some() {
            out.push("v_grid".to_string());
        }
        if self.interior_mullion.is_some() {
            out.push("interior_mullion".to_string());
        }
        if self.border_mullion.is_some() {
            out.push("border_mullion".to_string());
        }
        if self.panel.is_some() {
            out.push("panel".to_string());
        }
        if self.panel_material.is_some() {
            out.push("panel_material".to_string());
        }
        if self.mullion_material.is_some() {
            out.push("mullion_material".to_string());
        }
        if self.u_value.is_some() {
            out.push("u_value".to_string());
        }
        if self.g_value.is_some() {
            out.push("g_value".to_string());
        }
        if self.frame_fraction.is_some() {
            out.push("frame_fraction".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
