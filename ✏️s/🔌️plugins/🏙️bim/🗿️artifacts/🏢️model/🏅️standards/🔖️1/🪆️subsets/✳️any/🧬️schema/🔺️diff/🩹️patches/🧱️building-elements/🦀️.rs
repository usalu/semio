//! 🧱️ Sparse patches of the physical building elements: walls, curtain walls, columns, beams, slabs, ceilings, roofs, openings, stairs, railings and ramps.

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
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.wall_type, later.wall_type);
        merge_slot!(&mut self.axis, later.axis);
        merge_slot!(&mut self.location, later.location);
        merge_slot!(&mut self.base_offset, later.base_offset);
        merge_slot!(&mut self.top, later.top);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.start_join, later.start_join);
        merge_slot!(&mut self.end_join, later.end_join);
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.base_slab, later.base_slab);
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
        merge_slot!(&mut self.curtain, later.curtain);
        merge_slot!(&mut self.u, later.u);
        merge_slot!(&mut self.v, later.v);
        merge_slot!(&mut self.panel, later.panel);
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
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.curtain_wall_type, later.curtain_wall_type);
        merge_slot!(&mut self.axis, later.axis);
        merge_slot!(&mut self.base_offset, later.base_offset);
        merge_slot!(&mut self.top, later.top);
        merge_slot!(&mut self.u_grid, later.u_grid);
        merge_slot!(&mut self.v_grid, later.v_grid);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.name, later.name);
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

/// 🩹 Sparse patch of [`Column`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ColumnPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub column_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub tilt: Option<Assigned<Option<Slope>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Column> for ColumnPatch {
    fn write(&self, base: &Column) -> Column {
        Column { storey: take(&self.storey, &base.storey), column_type: take(&self.column_type, &base.column_type), position: take(&self.position, &base.position), rotation: take(&self.rotation, &base.rotation), tilt: take_assigned(&self.tilt, &base.tilt), base_offset: take(&self.base_offset, &base.base_offset), top: take(&self.top, &base.top), phase: take(&self.phase, &base.phase), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Column) -> Self {
        Self { storey: restore(&self.storey, &base.storey), column_type: restore(&self.column_type, &base.column_type), position: restore(&self.position, &base.position), rotation: restore(&self.rotation, &base.rotation), tilt: restore_assigned(&self.tilt, &base.tilt), base_offset: restore(&self.base_offset, &base.base_offset), top: restore(&self.top, &base.top), phase: restore(&self.phase, &base.phase), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.column_type, later.column_type);
        merge_slot!(&mut self.position, later.position);
        merge_slot!(&mut self.rotation, later.rotation);
        merge_slot!(&mut self.tilt, later.tilt);
        merge_slot!(&mut self.base_offset, later.base_offset);
        merge_slot!(&mut self.top, later.top);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Column) -> Self {
        Self { storey: changed(&self.storey, &base.storey), column_type: changed(&self.column_type, &base.column_type), position: changed(&self.position, &base.position), rotation: changed(&self.rotation, &base.rotation), tilt: changed_assigned(&self.tilt, &base.tilt), base_offset: changed(&self.base_offset, &base.base_offset), top: changed(&self.top, &base.top), phase: changed(&self.phase, &base.phase), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.column_type.is_some() {
            out.push("column_type".to_string());
        }
        if self.position.is_some() {
            out.push("position".to_string());
        }
        if self.rotation.is_some() {
            out.push("rotation".to_string());
        }
        if self.tilt.is_some() {
            out.push("tilt".to_string());
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
        if self.name.is_some() {
            out.push("name".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Beam`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct BeamPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub beam_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub axis: Option<Axis>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub top_offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub end_top_offset: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Beam> for BeamPatch {
    fn write(&self, base: &Beam) -> Beam {
        Beam { storey: take(&self.storey, &base.storey), beam_type: take(&self.beam_type, &base.beam_type), axis: take(&self.axis, &base.axis), top_offset: take(&self.top_offset, &base.top_offset), end_top_offset: take_assigned(&self.end_top_offset, &base.end_top_offset), phase: take(&self.phase, &base.phase), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Beam) -> Self {
        Self { storey: restore(&self.storey, &base.storey), beam_type: restore(&self.beam_type, &base.beam_type), axis: restore(&self.axis, &base.axis), top_offset: restore(&self.top_offset, &base.top_offset), end_top_offset: restore_assigned(&self.end_top_offset, &base.end_top_offset), phase: restore(&self.phase, &base.phase), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.beam_type, later.beam_type);
        merge_slot!(&mut self.axis, later.axis);
        merge_slot!(&mut self.top_offset, later.top_offset);
        merge_slot!(&mut self.end_top_offset, later.end_top_offset);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Beam) -> Self {
        Self { storey: changed(&self.storey, &base.storey), beam_type: changed(&self.beam_type, &base.beam_type), axis: changed(&self.axis, &base.axis), top_offset: changed(&self.top_offset, &base.top_offset), end_top_offset: changed_assigned(&self.end_top_offset, &base.end_top_offset), phase: changed(&self.phase, &base.phase), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.beam_type.is_some() {
            out.push("beam_type".to_string());
        }
        if self.axis.is_some() {
            out.push("axis".to_string());
        }
        if self.top_offset.is_some() {
            out.push("top_offset".to_string());
        }
        if self.end_top_offset.is_some() {
            out.push("end_top_offset".to_string());
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

/// 🩹 Sparse patch of [`Slab`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SlabPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub slab_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<Vec<Vertex>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub holes: Option<Vec<Vec<Vertex>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub slope: Option<Assigned<Option<Slope>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Slab> for SlabPatch {
    fn write(&self, base: &Slab) -> Slab {
        Slab { storey: take(&self.storey, &base.storey), slab_type: take(&self.slab_type, &base.slab_type), boundary: take(&self.boundary, &base.boundary), holes: take(&self.holes, &base.holes), offset: take(&self.offset, &base.offset), slope: take_assigned(&self.slope, &base.slope), phase: take(&self.phase, &base.phase), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Slab) -> Self {
        Self { storey: restore(&self.storey, &base.storey), slab_type: restore(&self.slab_type, &base.slab_type), boundary: restore(&self.boundary, &base.boundary), holes: restore(&self.holes, &base.holes), offset: restore(&self.offset, &base.offset), slope: restore_assigned(&self.slope, &base.slope), phase: restore(&self.phase, &base.phase), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.slab_type, later.slab_type);
        merge_slot!(&mut self.boundary, later.boundary);
        merge_slot!(&mut self.holes, later.holes);
        merge_slot!(&mut self.offset, later.offset);
        merge_slot!(&mut self.slope, later.slope);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Slab) -> Self {
        Self { storey: changed(&self.storey, &base.storey), slab_type: changed(&self.slab_type, &base.slab_type), boundary: changed(&self.boundary, &base.boundary), holes: changed(&self.holes, &base.holes), offset: changed(&self.offset, &base.offset), slope: changed_assigned(&self.slope, &base.slope), phase: changed(&self.phase, &base.phase), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.slab_type.is_some() {
            out.push("slab_type".to_string());
        }
        if self.boundary.is_some() {
            out.push("boundary".to_string());
        }
        if self.holes.is_some() {
            out.push("holes".to_string());
        }
        if self.offset.is_some() {
            out.push("offset".to_string());
        }
        if self.slope.is_some() {
            out.push("slope".to_string());
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

/// 🩹 Sparse patch of [`Ceiling`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct CeilingPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub ceiling_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<Vec<Vertex>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub holes: Option<Vec<Vec<Vertex>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub slope: Option<Assigned<Option<Slope>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Ceiling> for CeilingPatch {
    fn write(&self, base: &Ceiling) -> Ceiling {
        Ceiling { storey: take(&self.storey, &base.storey), ceiling_type: take(&self.ceiling_type, &base.ceiling_type), boundary: take(&self.boundary, &base.boundary), holes: take(&self.holes, &base.holes), offset: take(&self.offset, &base.offset), slope: take_assigned(&self.slope, &base.slope), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Ceiling) -> Self {
        Self { storey: restore(&self.storey, &base.storey), ceiling_type: restore(&self.ceiling_type, &base.ceiling_type), boundary: restore(&self.boundary, &base.boundary), holes: restore(&self.holes, &base.holes), offset: restore(&self.offset, &base.offset), slope: restore_assigned(&self.slope, &base.slope), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.ceiling_type, later.ceiling_type);
        merge_slot!(&mut self.boundary, later.boundary);
        merge_slot!(&mut self.holes, later.holes);
        merge_slot!(&mut self.offset, later.offset);
        merge_slot!(&mut self.slope, later.slope);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Ceiling) -> Self {
        Self { storey: changed(&self.storey, &base.storey), ceiling_type: changed(&self.ceiling_type, &base.ceiling_type), boundary: changed(&self.boundary, &base.boundary), holes: changed(&self.holes, &base.holes), offset: changed(&self.offset, &base.offset), slope: changed_assigned(&self.slope, &base.slope), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.ceiling_type.is_some() {
            out.push("ceiling_type".to_string());
        }
        if self.boundary.is_some() {
            out.push("boundary".to_string());
        }
        if self.holes.is_some() {
            out.push("holes".to_string());
        }
        if self.offset.is_some() {
            out.push("offset".to_string());
        }
        if self.slope.is_some() {
            out.push("slope".to_string());
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

/// 🩹 Sparse patch of [`Roof`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct RoofPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub roof_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub footprint: Option<Vec<Vertex>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub shape: Option<RoofShape>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub overhang: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Roof> for RoofPatch {
    fn write(&self, base: &Roof) -> Roof {
        Roof { storey: take(&self.storey, &base.storey), roof_type: take(&self.roof_type, &base.roof_type), footprint: take(&self.footprint, &base.footprint), shape: take(&self.shape, &base.shape), overhang: take(&self.overhang, &base.overhang), base_offset: take(&self.base_offset, &base.base_offset), phase: take(&self.phase, &base.phase), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Roof) -> Self {
        Self { storey: restore(&self.storey, &base.storey), roof_type: restore(&self.roof_type, &base.roof_type), footprint: restore(&self.footprint, &base.footprint), shape: restore(&self.shape, &base.shape), overhang: restore(&self.overhang, &base.overhang), base_offset: restore(&self.base_offset, &base.base_offset), phase: restore(&self.phase, &base.phase), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.roof_type, later.roof_type);
        merge_slot!(&mut self.footprint, later.footprint);
        merge_slot!(&mut self.shape, later.shape);
        merge_slot!(&mut self.overhang, later.overhang);
        merge_slot!(&mut self.base_offset, later.base_offset);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Roof) -> Self {
        Self { storey: changed(&self.storey, &base.storey), roof_type: changed(&self.roof_type, &base.roof_type), footprint: changed(&self.footprint, &base.footprint), shape: changed(&self.shape, &base.shape), overhang: changed(&self.overhang, &base.overhang), base_offset: changed(&self.base_offset, &base.base_offset), phase: changed(&self.phase, &base.phase), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.roof_type.is_some() {
            out.push("roof_type".to_string());
        }
        if self.footprint.is_some() {
            out.push("footprint".to_string());
        }
        if self.shape.is_some() {
            out.push("shape".to_string());
        }
        if self.overhang.is_some() {
            out.push("overhang".to_string());
        }
        if self.base_offset.is_some() {
            out.push("base_offset".to_string());
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

/// 🩹 Sparse patch of [`Opening`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct OpeningPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind: Option<OpeningKind>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sill_override: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub width: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub height: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub flip_hand: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub flip_facing: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub reveal_depth: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub reveal_material: Option<Assigned<Option<String>>>,
}

impl Patch<Opening> for OpeningPatch {
    fn write(&self, base: &Opening) -> Opening {
        Opening { host: take(&self.host, &base.host), kind: take(&self.kind, &base.kind), offset: take(&self.offset, &base.offset), sill_override: take_assigned(&self.sill_override, &base.sill_override), width: take_assigned(&self.width, &base.width), height: take_assigned(&self.height, &base.height), flip_hand: take(&self.flip_hand, &base.flip_hand), flip_facing: take(&self.flip_facing, &base.flip_facing), name: take(&self.name, &base.name), reveal_depth: take_assigned(&self.reveal_depth, &base.reveal_depth), reveal_material: take_assigned(&self.reveal_material, &base.reveal_material) }
    }
    fn restoring(&self, base: &Opening) -> Self {
        Self { host: restore(&self.host, &base.host), kind: restore(&self.kind, &base.kind), offset: restore(&self.offset, &base.offset), sill_override: restore_assigned(&self.sill_override, &base.sill_override), width: restore_assigned(&self.width, &base.width), height: restore_assigned(&self.height, &base.height), flip_hand: restore(&self.flip_hand, &base.flip_hand), flip_facing: restore(&self.flip_facing, &base.flip_facing), name: restore(&self.name, &base.name), reveal_depth: restore_assigned(&self.reveal_depth, &base.reveal_depth), reveal_material: restore_assigned(&self.reveal_material, &base.reveal_material) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.host, later.host);
        merge_slot!(&mut self.kind, later.kind);
        merge_slot!(&mut self.offset, later.offset);
        merge_slot!(&mut self.sill_override, later.sill_override);
        merge_slot!(&mut self.width, later.width);
        merge_slot!(&mut self.height, later.height);
        merge_slot!(&mut self.flip_hand, later.flip_hand);
        merge_slot!(&mut self.flip_facing, later.flip_facing);
        merge_slot!(&mut self.name, later.name);
        merge_slot!(&mut self.reveal_depth, later.reveal_depth);
        merge_slot!(&mut self.reveal_material, later.reveal_material);
    }
    fn minimal(&self, base: &Opening) -> Self {
        Self { host: changed(&self.host, &base.host), kind: changed(&self.kind, &base.kind), offset: changed(&self.offset, &base.offset), sill_override: changed_assigned(&self.sill_override, &base.sill_override), width: changed_assigned(&self.width, &base.width), height: changed_assigned(&self.height, &base.height), flip_hand: changed(&self.flip_hand, &base.flip_hand), flip_facing: changed(&self.flip_facing, &base.flip_facing), name: changed(&self.name, &base.name), reveal_depth: changed_assigned(&self.reveal_depth, &base.reveal_depth), reveal_material: changed_assigned(&self.reveal_material, &base.reveal_material) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.host.is_some() {
            out.push("host".to_string());
        }
        if self.kind.is_some() {
            out.push("kind".to_string());
        }
        if self.offset.is_some() {
            out.push("offset".to_string());
        }
        if self.sill_override.is_some() {
            out.push("sill_override".to_string());
        }
        if self.width.is_some() {
            out.push("width".to_string());
        }
        if self.height.is_some() {
            out.push("height".to_string());
        }
        if self.flip_hand.is_some() {
            out.push("flip_hand".to_string());
        }
        if self.flip_facing.is_some() {
            out.push("flip_facing".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.reveal_depth.is_some() {
            out.push("reveal_depth".to_string());
        }
        if self.reveal_material.is_some() {
            out.push("reveal_material".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Stair`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct StairPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub start: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub direction: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub flight: Option<StairFlight>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub max_riser: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub min_tread: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub stringer: Option<StairStringer>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub nosing: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub tread_thickness: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub riser: Option<RiserKind>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub landing_depth: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Stair> for StairPatch {
    fn write(&self, base: &Stair) -> Stair {
        Stair { storey: take(&self.storey, &base.storey), start: take(&self.start, &base.start), direction: take(&self.direction, &base.direction), width: take(&self.width, &base.width), flight: take(&self.flight, &base.flight), top: take(&self.top, &base.top), max_riser: take(&self.max_riser, &base.max_riser), min_tread: take(&self.min_tread, &base.min_tread), stringer: take(&self.stringer, &base.stringer), nosing: take(&self.nosing, &base.nosing), tread_thickness: take(&self.tread_thickness, &base.tread_thickness), riser: take(&self.riser, &base.riser), landing_depth: take(&self.landing_depth, &base.landing_depth), phase: take(&self.phase, &base.phase), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Stair) -> Self {
        Self { storey: restore(&self.storey, &base.storey), start: restore(&self.start, &base.start), direction: restore(&self.direction, &base.direction), width: restore(&self.width, &base.width), flight: restore(&self.flight, &base.flight), top: restore(&self.top, &base.top), max_riser: restore(&self.max_riser, &base.max_riser), min_tread: restore(&self.min_tread, &base.min_tread), stringer: restore(&self.stringer, &base.stringer), nosing: restore(&self.nosing, &base.nosing), tread_thickness: restore(&self.tread_thickness, &base.tread_thickness), riser: restore(&self.riser, &base.riser), landing_depth: restore(&self.landing_depth, &base.landing_depth), phase: restore(&self.phase, &base.phase), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.start, later.start);
        merge_slot!(&mut self.direction, later.direction);
        merge_slot!(&mut self.width, later.width);
        merge_slot!(&mut self.flight, later.flight);
        merge_slot!(&mut self.top, later.top);
        merge_slot!(&mut self.max_riser, later.max_riser);
        merge_slot!(&mut self.min_tread, later.min_tread);
        merge_slot!(&mut self.stringer, later.stringer);
        merge_slot!(&mut self.nosing, later.nosing);
        merge_slot!(&mut self.tread_thickness, later.tread_thickness);
        merge_slot!(&mut self.riser, later.riser);
        merge_slot!(&mut self.landing_depth, later.landing_depth);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Stair) -> Self {
        Self { storey: changed(&self.storey, &base.storey), start: changed(&self.start, &base.start), direction: changed(&self.direction, &base.direction), width: changed(&self.width, &base.width), flight: changed(&self.flight, &base.flight), top: changed(&self.top, &base.top), max_riser: changed(&self.max_riser, &base.max_riser), min_tread: changed(&self.min_tread, &base.min_tread), stringer: changed(&self.stringer, &base.stringer), nosing: changed(&self.nosing, &base.nosing), tread_thickness: changed(&self.tread_thickness, &base.tread_thickness), riser: changed(&self.riser, &base.riser), landing_depth: changed(&self.landing_depth, &base.landing_depth), phase: changed(&self.phase, &base.phase), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.start.is_some() {
            out.push("start".to_string());
        }
        if self.direction.is_some() {
            out.push("direction".to_string());
        }
        if self.width.is_some() {
            out.push("width".to_string());
        }
        if self.flight.is_some() {
            out.push("flight".to_string());
        }
        if self.top.is_some() {
            out.push("top".to_string());
        }
        if self.max_riser.is_some() {
            out.push("max_riser".to_string());
        }
        if self.min_tread.is_some() {
            out.push("min_tread".to_string());
        }
        if self.stringer.is_some() {
            out.push("stringer".to_string());
        }
        if self.nosing.is_some() {
            out.push("nosing".to_string());
        }
        if self.tread_thickness.is_some() {
            out.push("tread_thickness".to_string());
        }
        if self.riser.is_some() {
            out.push("riser".to_string());
        }
        if self.landing_depth.is_some() {
            out.push("landing_depth".to_string());
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

/// 🩹 Sparse patch of [`Railing`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct RailingPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Point2>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub post_spacing: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub post_profile: Option<Profile>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub baluster: Option<Assigned<Option<Baluster>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub infill: Option<Infill>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub host: Option<Assigned<Option<RailingHost>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Railing> for RailingPatch {
    fn write(&self, base: &Railing) -> Railing {
        Railing { storey: take(&self.storey, &base.storey), path: take(&self.path, &base.path), height: take(&self.height, &base.height), post_spacing: take(&self.post_spacing, &base.post_spacing), profile: take(&self.profile, &base.profile), post_profile: take(&self.post_profile, &base.post_profile), baluster: take_assigned(&self.baluster, &base.baluster), infill: take(&self.infill, &base.infill), material: take(&self.material, &base.material), base_offset: take(&self.base_offset, &base.base_offset), host: take_assigned(&self.host, &base.host), phase: take(&self.phase, &base.phase), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Railing) -> Self {
        Self { storey: restore(&self.storey, &base.storey), path: restore(&self.path, &base.path), height: restore(&self.height, &base.height), post_spacing: restore(&self.post_spacing, &base.post_spacing), profile: restore(&self.profile, &base.profile), post_profile: restore(&self.post_profile, &base.post_profile), baluster: restore_assigned(&self.baluster, &base.baluster), infill: restore(&self.infill, &base.infill), material: restore(&self.material, &base.material), base_offset: restore(&self.base_offset, &base.base_offset), host: restore_assigned(&self.host, &base.host), phase: restore(&self.phase, &base.phase), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.path, later.path);
        merge_slot!(&mut self.height, later.height);
        merge_slot!(&mut self.post_spacing, later.post_spacing);
        merge_slot!(&mut self.profile, later.profile);
        merge_slot!(&mut self.post_profile, later.post_profile);
        merge_slot!(&mut self.baluster, later.baluster);
        merge_slot!(&mut self.infill, later.infill);
        merge_slot!(&mut self.material, later.material);
        merge_slot!(&mut self.base_offset, later.base_offset);
        merge_slot!(&mut self.host, later.host);
        merge_slot!(&mut self.phase, later.phase);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Railing) -> Self {
        Self { storey: changed(&self.storey, &base.storey), path: changed(&self.path, &base.path), height: changed(&self.height, &base.height), post_spacing: changed(&self.post_spacing, &base.post_spacing), profile: changed(&self.profile, &base.profile), post_profile: changed(&self.post_profile, &base.post_profile), baluster: changed_assigned(&self.baluster, &base.baluster), infill: changed(&self.infill, &base.infill), material: changed(&self.material, &base.material), base_offset: changed(&self.base_offset, &base.base_offset), host: changed_assigned(&self.host, &base.host), phase: changed(&self.phase, &base.phase), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.path.is_some() {
            out.push("path".to_string());
        }
        if self.height.is_some() {
            out.push("height".to_string());
        }
        if self.post_spacing.is_some() {
            out.push("post_spacing".to_string());
        }
        if self.profile.is_some() {
            out.push("profile".to_string());
        }
        if self.post_profile.is_some() {
            out.push("post_profile".to_string());
        }
        if self.baluster.is_some() {
            out.push("baluster".to_string());
        }
        if self.infill.is_some() {
            out.push("infill".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        if self.base_offset.is_some() {
            out.push("base_offset".to_string());
        }
        if self.host.is_some() {
            out.push("host".to_string());
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

/// 🩹 Sparse patch of [`Ramp`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct RampPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Vertex>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub landing_start: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub landing_end: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub landing_turn: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub max_slope: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub railing_left: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub railing_right: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Ramp> for RampPatch {
    fn write(&self, base: &Ramp) -> Ramp {
        Ramp { storey: take(&self.storey, &base.storey), path: take(&self.path, &base.path), width: take(&self.width, &base.width), landing_start: take(&self.landing_start, &base.landing_start), landing_end: take(&self.landing_end, &base.landing_end), landing_turn: take(&self.landing_turn, &base.landing_turn), max_slope: take(&self.max_slope, &base.max_slope), thickness: take(&self.thickness, &base.thickness), material: take(&self.material, &base.material), base_offset: take(&self.base_offset, &base.base_offset), top: take(&self.top, &base.top), railing_left: take(&self.railing_left, &base.railing_left), railing_right: take(&self.railing_right, &base.railing_right), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Ramp) -> Self {
        Self { storey: restore(&self.storey, &base.storey), path: restore(&self.path, &base.path), width: restore(&self.width, &base.width), landing_start: restore(&self.landing_start, &base.landing_start), landing_end: restore(&self.landing_end, &base.landing_end), landing_turn: restore(&self.landing_turn, &base.landing_turn), max_slope: restore(&self.max_slope, &base.max_slope), thickness: restore(&self.thickness, &base.thickness), material: restore(&self.material, &base.material), base_offset: restore(&self.base_offset, &base.base_offset), top: restore(&self.top, &base.top), railing_left: restore(&self.railing_left, &base.railing_left), railing_right: restore(&self.railing_right, &base.railing_right), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot!(&mut self.storey, later.storey);
        merge_slot!(&mut self.path, later.path);
        merge_slot!(&mut self.width, later.width);
        merge_slot!(&mut self.landing_start, later.landing_start);
        merge_slot!(&mut self.landing_end, later.landing_end);
        merge_slot!(&mut self.landing_turn, later.landing_turn);
        merge_slot!(&mut self.max_slope, later.max_slope);
        merge_slot!(&mut self.thickness, later.thickness);
        merge_slot!(&mut self.material, later.material);
        merge_slot!(&mut self.base_offset, later.base_offset);
        merge_slot!(&mut self.top, later.top);
        merge_slot!(&mut self.railing_left, later.railing_left);
        merge_slot!(&mut self.railing_right, later.railing_right);
        merge_slot!(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Ramp) -> Self {
        Self { storey: changed(&self.storey, &base.storey), path: changed(&self.path, &base.path), width: changed(&self.width, &base.width), landing_start: changed(&self.landing_start, &base.landing_start), landing_end: changed(&self.landing_end, &base.landing_end), landing_turn: changed(&self.landing_turn, &base.landing_turn), max_slope: changed(&self.max_slope, &base.max_slope), thickness: changed(&self.thickness, &base.thickness), material: changed(&self.material, &base.material), base_offset: changed(&self.base_offset, &base.base_offset), top: changed(&self.top, &base.top), railing_left: changed(&self.railing_left, &base.railing_left), railing_right: changed(&self.railing_right, &base.railing_right), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.path.is_some() {
            out.push("path".to_string());
        }
        if self.width.is_some() {
            out.push("width".to_string());
        }
        if self.landing_start.is_some() {
            out.push("landing_start".to_string());
        }
        if self.landing_end.is_some() {
            out.push("landing_end".to_string());
        }
        if self.landing_turn.is_some() {
            out.push("landing_turn".to_string());
        }
        if self.max_slope.is_some() {
            out.push("max_slope".to_string());
        }
        if self.thickness.is_some() {
            out.push("thickness".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        if self.base_offset.is_some() {
            out.push("base_offset".to_string());
        }
        if self.top.is_some() {
            out.push("top".to_string());
        }
        if self.railing_left.is_some() {
            out.push("railing_left".to_string());
        }
        if self.railing_right.is_some() {
            out.push("railing_right".to_string());
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
