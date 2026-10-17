//! 🏚️ Wall patches of the building elements: the wall, curtain panel override and curtain wall sparse patches. See [`super`].

use super::*;

/// 🩹 Sparse patch of [`Wall`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct WallPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub wall_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub axis: Option<Axis>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationLine>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub start_join: Option<Assigned<Option<EndJoin>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub end_join: Option<Assigned<Option<EndJoin>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub base_slab: Option<Assigned<Option<String>>>,
}

impl Patch<Wall> for WallPatch {
    fn write(&self, base: &Wall) -> Wall {
        Wall { storey: take(&self.storey, &base.storey), wall_type: take(&self.wall_type, &base.wall_type), axis: take(&self.axis, &base.axis), location: take(&self.location, &base.location), base_offset: take(&self.base_offset, &base.base_offset), top: take(&self.top, &base.top), phase: take(&self.phase, &base.phase), start_join: take_assigned(&self.start_join, &base.start_join), end_join: take_assigned(&self.end_join, &base.end_join), name: take(&self.name, &base.name), base_slab: take_assigned(&self.base_slab, &base.base_slab) }
    }
    fn restoring(&self, base: &Wall) -> Self {
        Self { storey: restore(&self.storey, &base.storey), wall_type: restore(&self.wall_type, &base.wall_type), axis: restore(&self.axis, &base.axis), location: restore(&self.location, &base.location), base_offset: restore(&self.base_offset, &base.base_offset), top: restore(&self.top, &base.top), phase: restore(&self.phase, &base.phase), start_join: restore_assigned(&self.start_join, &base.start_join), end_join: restore_assigned(&self.end_join, &base.end_join), name: restore(&self.name, &base.name), base_slab: restore_assigned(&self.base_slab, &base.base_slab) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.wall_type, later.wall_type);
        merge_slot(&mut self.axis, later.axis);
        merge_slot(&mut self.location, later.location);
        merge_slot(&mut self.base_offset, later.base_offset);
        merge_slot(&mut self.top, later.top);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.start_join, later.start_join);
        merge_slot(&mut self.end_join, later.end_join);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.base_slab, later.base_slab);
    }
    fn minimal(&self, base: &Wall) -> Self {
        Self { storey: changed(&self.storey, &base.storey), wall_type: changed(&self.wall_type, &base.wall_type), axis: changed(&self.axis, &base.axis), location: changed(&self.location, &base.location), base_offset: changed(&self.base_offset, &base.base_offset), top: changed(&self.top, &base.top), phase: changed(&self.phase, &base.phase), start_join: changed_assigned(&self.start_join, &base.start_join), end_join: changed_assigned(&self.end_join, &base.end_join), name: changed(&self.name, &base.name), base_slab: changed_assigned(&self.base_slab, &base.base_slab) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.wall_type.is_some() {
            out.push("wall_type".to_string());
        }
        if self.axis.is_some() {
            out.push("axis".to_string());
        }
        if self.location.is_some() {
            out.push("location".to_string());
        }
        if self.base_offset.is_some() {
            out.push("base_offset".to_string());
        }
        if self.top.is_some() {
            out.push("top".to_string());
        }
        if self.phase.is_some() {
            out.push("phase".to_string());
        }
        if self.start_join.is_some() {
            out.push("start_join".to_string());
        }
        if self.end_join.is_some() {
            out.push("end_join".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.base_slab.is_some() {
            out.push("base_slab".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`CurtainPanelOverride`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct CurtainPanelOverridePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub curtain: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub u: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub v: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub panel: Option<CurtainPanel>,
}

impl Patch<CurtainPanelOverride> for CurtainPanelOverridePatch {
    fn write(&self, base: &CurtainPanelOverride) -> CurtainPanelOverride {
        CurtainPanelOverride { curtain: take(&self.curtain, &base.curtain), u: take(&self.u, &base.u), v: take(&self.v, &base.v), panel: take(&self.panel, &base.panel) }
    }
    fn restoring(&self, base: &CurtainPanelOverride) -> Self {
        Self { curtain: restore(&self.curtain, &base.curtain), u: restore(&self.u, &base.u), v: restore(&self.v, &base.v), panel: restore(&self.panel, &base.panel) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.curtain, later.curtain);
        merge_slot(&mut self.u, later.u);
        merge_slot(&mut self.v, later.v);
        merge_slot(&mut self.panel, later.panel);
    }
    fn minimal(&self, base: &CurtainPanelOverride) -> Self {
        Self { curtain: changed(&self.curtain, &base.curtain), u: changed(&self.u, &base.u), v: changed(&self.v, &base.v), panel: changed(&self.panel, &base.panel) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.curtain.is_some() {
            out.push("curtain".to_string());
        }
        if self.u.is_some() {
            out.push("u".to_string());
        }
        if self.v.is_some() {
            out.push("v".to_string());
        }
        if self.panel.is_some() {
            out.push("panel".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`CurtainWall`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct CurtainWallPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub curtain_wall_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub axis: Option<Axis>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub u_grid: Option<Assigned<Option<CurtainGrid>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub v_grid: Option<Assigned<Option<CurtainGrid>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<CurtainWall> for CurtainWallPatch {
    fn write(&self, base: &CurtainWall) -> CurtainWall {
        CurtainWall { storey: take(&self.storey, &base.storey), curtain_wall_type: take(&self.curtain_wall_type, &base.curtain_wall_type), axis: take(&self.axis, &base.axis), base_offset: take(&self.base_offset, &base.base_offset), top: take(&self.top, &base.top), u_grid: take_assigned(&self.u_grid, &base.u_grid), v_grid: take_assigned(&self.v_grid, &base.v_grid), phase: take(&self.phase, &base.phase), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &CurtainWall) -> Self {
        Self { storey: restore(&self.storey, &base.storey), curtain_wall_type: restore(&self.curtain_wall_type, &base.curtain_wall_type), axis: restore(&self.axis, &base.axis), base_offset: restore(&self.base_offset, &base.base_offset), top: restore(&self.top, &base.top), u_grid: restore_assigned(&self.u_grid, &base.u_grid), v_grid: restore_assigned(&self.v_grid, &base.v_grid), phase: restore(&self.phase, &base.phase), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.curtain_wall_type, later.curtain_wall_type);
        merge_slot(&mut self.axis, later.axis);
        merge_slot(&mut self.base_offset, later.base_offset);
        merge_slot(&mut self.top, later.top);
        merge_slot(&mut self.u_grid, later.u_grid);
        merge_slot(&mut self.v_grid, later.v_grid);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.name, later.name);
    }
    fn minimal(&self, base: &CurtainWall) -> Self {
        Self { storey: changed(&self.storey, &base.storey), curtain_wall_type: changed(&self.curtain_wall_type, &base.curtain_wall_type), axis: changed(&self.axis, &base.axis), base_offset: changed(&self.base_offset, &base.base_offset), top: changed(&self.top, &base.top), u_grid: changed_assigned(&self.u_grid, &base.u_grid), v_grid: changed_assigned(&self.v_grid, &base.v_grid), phase: changed(&self.phase, &base.phase), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.curtain_wall_type.is_some() {
            out.push("curtain_wall_type".to_string());
        }
        if self.axis.is_some() {
            out.push("axis".to_string());
        }
        if self.base_offset.is_some() {
            out.push("base_offset".to_string());
        }
        if self.top.is_some() {
            out.push("top".to_string());
        }
        if self.u_grid.is_some() {
            out.push("u_grid".to_string());
        }
        if self.v_grid.is_some() {
            out.push("v_grid".to_string());
        }
        if self.phase.is_some() {
            out.push("phase".to_string());
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
