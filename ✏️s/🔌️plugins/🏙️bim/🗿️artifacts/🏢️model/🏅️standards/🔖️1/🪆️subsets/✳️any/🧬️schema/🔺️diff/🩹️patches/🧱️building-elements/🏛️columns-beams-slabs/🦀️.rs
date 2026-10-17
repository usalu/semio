//! 🏛️ Structural patches of the building elements: the column, beam and slab sparse patches. See [`super`].

use super::*;

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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.column_type, later.column_type);
        merge_slot(&mut self.position, later.position);
        merge_slot(&mut self.rotation, later.rotation);
        merge_slot(&mut self.tilt, later.tilt);
        merge_slot(&mut self.base_offset, later.base_offset);
        merge_slot(&mut self.top, later.top);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.beam_type, later.beam_type);
        merge_slot(&mut self.axis, later.axis);
        merge_slot(&mut self.top_offset, later.top_offset);
        merge_slot(&mut self.end_top_offset, later.end_top_offset);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.slab_type, later.slab_type);
        merge_slot(&mut self.boundary, later.boundary);
        merge_slot(&mut self.holes, later.holes);
        merge_slot(&mut self.offset, later.offset);
        merge_slot(&mut self.slope, later.slope);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.name, later.name);
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
