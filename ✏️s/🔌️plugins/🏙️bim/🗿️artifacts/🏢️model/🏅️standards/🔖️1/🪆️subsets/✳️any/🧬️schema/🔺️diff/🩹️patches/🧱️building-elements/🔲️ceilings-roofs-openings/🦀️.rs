//! 🔲️ Envelope patches of the building elements: the ceiling, roof and opening sparse patches. See [`super`].

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::entities::*;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::*;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::*;

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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.ceiling_type, later.ceiling_type);
        merge_slot(&mut self.boundary, later.boundary);
        merge_slot(&mut self.holes, later.holes);
        merge_slot(&mut self.offset, later.offset);
        merge_slot(&mut self.slope, later.slope);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.roof_type, later.roof_type);
        merge_slot(&mut self.footprint, later.footprint);
        merge_slot(&mut self.shape, later.shape);
        merge_slot(&mut self.overhang, later.overhang);
        merge_slot(&mut self.base_offset, later.base_offset);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.host, later.host);
        merge_slot(&mut self.kind, later.kind);
        merge_slot(&mut self.offset, later.offset);
        merge_slot(&mut self.sill_override, later.sill_override);
        merge_slot(&mut self.width, later.width);
        merge_slot(&mut self.height, later.height);
        merge_slot(&mut self.flip_hand, later.flip_hand);
        merge_slot(&mut self.flip_facing, later.flip_facing);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.reveal_depth, later.reveal_depth);
        merge_slot(&mut self.reveal_material, later.reveal_material);
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
