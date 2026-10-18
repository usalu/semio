//! 🪜️ Circulation patches of the building elements: the stair, railing and ramp sparse patches. See [`super`].

use crate::standards::v1::subsets::any::schema::snapshot::{entities::*, values::*};
use crate::standards::v1::subsets::any::schema::diff::algebra::*;

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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.start, later.start);
        merge_slot(&mut self.direction, later.direction);
        merge_slot(&mut self.width, later.width);
        merge_slot(&mut self.flight, later.flight);
        merge_slot(&mut self.top, later.top);
        merge_slot(&mut self.max_riser, later.max_riser);
        merge_slot(&mut self.min_tread, later.min_tread);
        merge_slot(&mut self.stringer, later.stringer);
        merge_slot(&mut self.nosing, later.nosing);
        merge_slot(&mut self.tread_thickness, later.tread_thickness);
        merge_slot(&mut self.riser, later.riser);
        merge_slot(&mut self.landing_depth, later.landing_depth);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.path, later.path);
        merge_slot(&mut self.height, later.height);
        merge_slot(&mut self.post_spacing, later.post_spacing);
        merge_slot(&mut self.profile, later.profile);
        merge_slot(&mut self.post_profile, later.post_profile);
        merge_slot(&mut self.baluster, later.baluster);
        merge_slot(&mut self.infill, later.infill);
        merge_slot(&mut self.material, later.material);
        merge_slot(&mut self.base_offset, later.base_offset);
        merge_slot(&mut self.host, later.host);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.name, later.name);
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
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.path, later.path);
        merge_slot(&mut self.width, later.width);
        merge_slot(&mut self.landing_start, later.landing_start);
        merge_slot(&mut self.landing_end, later.landing_end);
        merge_slot(&mut self.landing_turn, later.landing_turn);
        merge_slot(&mut self.max_slope, later.max_slope);
        merge_slot(&mut self.thickness, later.thickness);
        merge_slot(&mut self.material, later.material);
        merge_slot(&mut self.base_offset, later.base_offset);
        merge_slot(&mut self.top, later.top);
        merge_slot(&mut self.railing_left, later.railing_left);
        merge_slot(&mut self.railing_right, later.railing_right);
        merge_slot(&mut self.name, later.name);
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
