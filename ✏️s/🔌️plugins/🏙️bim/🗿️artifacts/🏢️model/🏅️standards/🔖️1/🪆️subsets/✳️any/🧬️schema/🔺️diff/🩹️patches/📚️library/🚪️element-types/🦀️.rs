//! 🚪️ Element types: the sparse patches of the wall, slab, ceiling, roof, column, beam, window, door and curtain wall types. See [`super`].

use super::*;

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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.layers, later.layers);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.layers, later.layers);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.layers, later.layers);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.layers, later.layers);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.profile, later.profile);
        merge_slot(&mut self.material, later.material);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.profile, later.profile);
        merge_slot(&mut self.material, later.material);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.width, later.width);
        merge_slot(&mut self.height, later.height);
        merge_slot(&mut self.sill, later.sill);
        merge_slot(&mut self.frame_width, later.frame_width);
        merge_slot(&mut self.frame_depth, later.frame_depth);
        merge_slot(&mut self.panes, later.panes);
        merge_slot(&mut self.material, later.material);
        merge_slot(&mut self.u_value, later.u_value);
        merge_slot(&mut self.g_value, later.g_value);
        merge_slot(&mut self.frame_fraction, later.frame_fraction);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.width, later.width);
        merge_slot(&mut self.height, later.height);
        merge_slot(&mut self.frame_width, later.frame_width);
        merge_slot(&mut self.frame_depth, later.frame_depth);
        merge_slot(&mut self.leaves, later.leaves);
        merge_slot(&mut self.swing, later.swing);
        merge_slot(&mut self.material, later.material);
        merge_slot(&mut self.u_value, later.u_value);
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
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.u_grid, later.u_grid);
        merge_slot(&mut self.v_grid, later.v_grid);
        merge_slot(&mut self.interior_mullion, later.interior_mullion);
        merge_slot(&mut self.border_mullion, later.border_mullion);
        merge_slot(&mut self.panel, later.panel);
        merge_slot(&mut self.panel_material, later.panel_material);
        merge_slot(&mut self.mullion_material, later.mullion_material);
        merge_slot(&mut self.u_value, later.u_value);
        merge_slot(&mut self.g_value, later.g_value);
        merge_slot(&mut self.frame_fraction, later.frame_fraction);
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
