//! 🧬️ Din4108 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_artifact_norm_contract::{norm_list_delta, norm_row_patch};

use crate::Din4108Snapshot;

//#region 🔖️Rows
norm_row_patch! {
    /// 🩹 Sparse field patch of one `ZoneWindow`.
    pub Din4108WindowPatch of crate::ZoneWindow { set { orientation: String, inclination_deg: f64, area_m2: f64, g_value: f64, shading_fc: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `ZoneWindow` list.
    pub Din4108WindowDelta { addition: Din4108WindowAddition, modification: Din4108WindowModification, row: crate::ZoneWindow, patch: Din4108WindowPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `LayerDocument`.
    pub Din4108LayerPatch of crate::LayerDocument { set { material_id: String, thickness_m: f64, lambda: f64, mu: f64, density: f64, application_type: String, compressive_class: String, water_class: String, tensile_class: String, acoustic_class: String, segments: Vec<crate::LayerSegment> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `LayerDocument` list.
    pub Din4108LayerDelta { addition: Din4108LayerAddition, modification: Din4108LayerModification, row: crate::LayerDocument, patch: Din4108LayerPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `ThermalZone`.
    pub Din4108ZonePatch of crate::ThermalZone { set { floor_area_m2: f64, heaviness: String, night_ventilation: String } nest { windows: Din4108WindowDelta } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `ThermalZone` list.
    pub Din4108ZoneDelta { addition: Din4108ZoneAddition, modification: Din4108ZoneModification, row: crate::ThermalZone, patch: Din4108ZonePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `EnvelopeElement`.
    pub Din4108ElementPatch of crate::EnvelopeElement { set { kind: String, zone_id: String, orientation_deg: f64, inclination_deg: f64, adjacent: String, area_m2: f64, delta_u_g: f64, delta_u_f: f64, delta_u_r: f64 } nest { layers: Din4108LayerDelta } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `EnvelopeElement` list.
    pub Din4108ElementDelta { addition: Din4108ElementAddition, modification: Din4108ElementModification, row: crate::EnvelopeElement, patch: Din4108ElementPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `ThermalBridge`.
    pub Din4108ThermalBridgePatch of crate::ThermalBridge { set { psi: f64, length_m: f64, bb2_type: String } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `ThermalBridge` list.
    pub Din4108ThermalBridgeDelta { addition: Din4108ThermalBridgeAddition, modification: Din4108ThermalBridgeModification, row: crate::ThermalBridge, patch: Din4108ThermalBridgePatch, key: id }
}
//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse delta for the Din4108 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Diff {
    #[state(artifact)]
    pub climate_zone: Option<crate::document::ClimateZoneDe>,
    #[state(artifact)]
    pub usage: Option<String>,
    #[state(artifact)]
    pub t_int_c: Option<f64>,
    #[state(artifact)]
    pub rh_int: Option<f64>,
    #[state(artifact)]
    pub has_mechanical_ventilation: Option<bool>,
    #[state(artifact)]
    pub airtightness_n50: Option<f64>,
    #[state(artifact)]
    pub bb2_details_conform: Option<bool>,
    #[state(artifact)]
    pub zones: Din4108ZoneDelta,
    #[state(artifact)]
    pub elements: Din4108ElementDelta,
    #[state(artifact)]
    pub thermal_bridges: Din4108ThermalBridgeDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<Din4108Snapshot> for Din4108Diff {
    fn apply(&self, base: &Din4108Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Din4108Snapshot> {
        let mut next = base.clone();
        if let Some(value) = self.climate_zone {
            next.climate_zone = value;
        }
        if let Some(value) = &self.usage {
            next.usage = value.clone();
        }
        if let Some(value) = self.t_int_c {
            next.t_int_c = value;
        }
        if let Some(value) = self.rh_int {
            next.rh_int = value;
        }
        if let Some(value) = self.has_mechanical_ventilation {
            next.has_mechanical_ventilation = value;
        }
        if let Some(value) = self.airtightness_n50 {
            next.airtightness_n50 = value;
        }
        if let Some(value) = self.bb2_details_conform {
            next.bb2_details_conform = value;
        }
        next.zones = self.zones.commit_onto(&base.zones).map_err(|error| error.under(["zones"]))?;
        next.elements = self.elements.commit_onto(&base.elements).map_err(|error| error.under(["elements"]))?;
        next.thermal_bridges = self.thermal_bridges.commit_onto(&base.thermal_bridges).map_err(|error| error.under(["thermalBridges"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.climate_zone.is_some() {
            self.climate_zone = other.climate_zone;
        }
        if other.usage.is_some() {
            self.usage = other.usage;
        }
        if other.t_int_c.is_some() {
            self.t_int_c = other.t_int_c;
        }
        if other.rh_int.is_some() {
            self.rh_int = other.rh_int;
        }
        if other.has_mechanical_ventilation.is_some() {
            self.has_mechanical_ventilation = other.has_mechanical_ventilation;
        }
        if other.airtightness_n50.is_some() {
            self.airtightness_n50 = other.airtightness_n50;
        }
        if other.bb2_details_conform.is_some() {
            self.bb2_details_conform = other.bb2_details_conform;
        }
        self.zones.absorb(other.zones);
        self.elements.absorb(other.elements);
        self.thermal_bridges.absorb(other.thermal_bridges);
    }
}

impl DiffAlgebra<Din4108Snapshot> for Din4108Diff {
    fn inverse(&self, base: &Din4108Snapshot) -> Self {
        Self {
            climate_zone: self.climate_zone.map(|_| base.climate_zone),
            usage: self.usage.as_ref().map(|_| base.usage.clone()),
            t_int_c: self.t_int_c.map(|_| base.t_int_c),
            rh_int: self.rh_int.map(|_| base.rh_int),
            has_mechanical_ventilation: self.has_mechanical_ventilation.map(|_| base.has_mechanical_ventilation),
            airtightness_n50: self.airtightness_n50.map(|_| base.airtightness_n50),
            bb2_details_conform: self.bb2_details_conform.map(|_| base.bb2_details_conform),
            zones: self.zones.inverse(&base.zones),
            elements: self.elements.inverse(&base.elements),
            thermal_bridges: self.thermal_bridges.inverse(&base.thermal_bridges),
        }
    }

    fn between(base: &Din4108Snapshot, other: &Din4108Snapshot) -> Self {
        Self {
            climate_zone: (base.climate_zone != other.climate_zone).then_some(other.climate_zone),
            usage: (base.usage != other.usage).then(|| other.usage.clone()),
            t_int_c: (base.t_int_c != other.t_int_c).then_some(other.t_int_c),
            rh_int: (base.rh_int != other.rh_int).then_some(other.rh_int),
            has_mechanical_ventilation: (base.has_mechanical_ventilation != other.has_mechanical_ventilation).then_some(other.has_mechanical_ventilation),
            airtightness_n50: (base.airtightness_n50 != other.airtightness_n50).then_some(other.airtightness_n50),
            bb2_details_conform: (base.bb2_details_conform != other.bb2_details_conform).then_some(other.bb2_details_conform),
            zones: Din4108ZoneDelta::between(&base.zones, &other.zones),
            elements: Din4108ElementDelta::between(&base.elements, &other.elements),
            thermal_bridges: Din4108ThermalBridgeDelta::between(&base.thermal_bridges, &other.thermal_bridges),
        }
    }

    fn is_empty(&self) -> bool {
        self.climate_zone.is_none()
            && self.usage.is_none()
            && self.t_int_c.is_none()
            && self.rh_int.is_none()
            && self.has_mechanical_ventilation.is_none()
            && self.airtightness_n50.is_none()
            && self.bb2_details_conform.is_none()
            && self.zones.is_empty()
            && self.elements.is_empty()
            && self.thermal_bridges.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
