//! 🧱️ Components and materials: the sparse patches of the component, component override, MEP element and material. See [`super`].

use super::*;

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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.family, later.family);
        merge_slot(&mut self.position, later.position);
        merge_slot(&mut self.elevation, later.elevation);
        merge_slot(&mut self.rotation, later.rotation);
        merge_slot(&mut self.mirrored, later.mirrored);
        merge_slot(&mut self.host, later.host);
        merge_slot(&mut self.system, later.system);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.component, later.component);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.value, later.value);
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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.system, later.system);
        merge_slot(&mut self.shape, later.shape);
        merge_slot(&mut self.path, later.path);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.category, later.category);
        merge_slot(&mut self.color, later.color);
        merge_slot(&mut self.density, later.density);
        merge_slot(&mut self.conductivity, later.conductivity);
        merge_slot(&mut self.specific_heat, later.specific_heat);
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
