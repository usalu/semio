//! 🧬️ Din4108 snapshot schema — complete building-envelope subject.

use crate::document::ClimateZoneDe;
use crate::{EnvelopeElement, LayerDocument, LayerSegment, ThermalBridge, ThermalZone, ZoneWindow};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.din4108", layout = "lines")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Snapshot {
    #[state(artifact)]
    pub climate_zone: ClimateZoneDe,
    #[state(artifact)]
    pub usage: String,
    #[state(artifact)]
    pub t_int_c: f64,
    #[state(artifact)]
    pub rh_int: f64,
    #[state(artifact)]
    pub has_mechanical_ventilation: bool,
    #[state(artifact)]
    pub airtightness_n50: f64,
    #[state(artifact)]
    pub bb2_details_conform: bool,
    #[dsl(table)]
    #[state(artifact)]
    pub zones: Vec<ThermalZone>,
    #[dsl(table)]
    #[state(artifact)]
    pub elements: Vec<EnvelopeElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub thermal_bridges: Vec<ThermalBridge>,
}
crate::impl_norm_artifact_record!(Din4108Snapshot, extension = "din4108", envelope_id = "norm.din4108");

impl Default for Din4108Snapshot {
    fn default() -> Self {
        Self::compliant_etics_dwelling()
    }
}

impl Din4108Snapshot {
    /// 🏠 Realistic compliant ETICS dwelling (zone 2, residential) — default subject.
    pub fn compliant_etics_dwelling() -> Self {
        Self {
            climate_zone: ClimateZoneDe::Zone2,
            usage: "residential".into(),
            t_int_c: 20.0,
            rh_int: 0.50,
            has_mechanical_ventilation: true,
            airtightness_n50: 1.5,
            bb2_details_conform: true,
            zones: vec![
                ThermalZone {
                    id: "zone-living".into(),
                    floor_area_m2: 80.0,
                    heaviness: "heavy".into(),
                    night_ventilation: "moderate".into(),
                    windows: vec![
                        ZoneWindow { id: "win-south".into(), orientation: "S".into(), inclination_deg: 90.0, area_m2: 8.0, g_value: 0.50, shading_fc: 0.50 },
                        ZoneWindow { id: "win-east".into(), orientation: "E".into(), inclination_deg: 90.0, area_m2: 4.0, g_value: 0.50, shading_fc: 0.70 },
                    ],
                },
                ThermalZone {
                    id: "zone-utility".into(),
                    floor_area_m2: 12.0,
                    heaviness: "heavy".into(),
                    night_ventilation: "none".into(),
                    windows: vec![
                        ZoneWindow { id: "win-utility".into(), orientation: "N".into(), inclination_deg: 90.0, area_m2: 1.0, g_value: 0.50, shading_fc: 0.70 },
                    ],
                },
            ],
            elements: vec![
                EnvelopeElement {
                    id: "wall-north".into(),
                    kind: "wall".into(),
                    zone_id: "zone-living".into(),
                    orientation_deg: 0.0,
                    inclination_deg: 90.0,
                    adjacent: "exterior".into(),
                    area_m2: 40.0,
                    delta_u_g: 0.0,
                    delta_u_f: 0.0,
                    delta_u_r: 0.0,
                    layers: vec![
                        LayerDocument { id: "plaster-int".into(), material_id: "gypsum_plaster".into(), thickness_m: 0.015, lambda: 0.70, mu: 10.0, density: 1400.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                        LayerDocument { id: "masonry".into(), material_id: "brick".into(), thickness_m: 0.24, lambda: 0.81, mu: 10.0, density: 1800.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                        LayerDocument { id: "eps".into(), material_id: "eps".into(), thickness_m: 0.14, lambda: 0.035, mu: 40.0, density: 20.0, application_type: "WAP".into(), compressive_class: "dm".into(), water_class: "wf".into(), tensile_class: "tf".into(), acoustic_class: "sm".into(), segments: vec![] },
                        LayerDocument { id: "render".into(), material_id: "mineral_render".into(), thickness_m: 0.008, lambda: 0.70, mu: 20.0, density: 1600.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                    ],
                },
                EnvelopeElement {
                    id: "roof".into(),
                    kind: "roof".into(),
                    zone_id: "zone-living".into(),
                    orientation_deg: 180.0,
                    inclination_deg: 0.0,
                    adjacent: "exterior".into(),
                    area_m2: 80.0,
                    delta_u_g: 0.0,
                    delta_u_f: 0.0,
                    delta_u_r: 0.0,
                    layers: vec![
                        LayerDocument { id: "gypsum".into(), material_id: "gypsum_plaster".into(), thickness_m: 0.0125, lambda: 0.25, mu: 8.0, density: 900.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                        LayerDocument { id: "mineral-wool".into(), material_id: "mineral_wool".into(), thickness_m: 0.20, lambda: 0.035, mu: 1.3, density: 30.0, application_type: "DAD".into(), compressive_class: "dm".into(), water_class: "wk".into(), tensile_class: "tk".into(), acoustic_class: "sh".into(), segments: vec![] },
                        LayerDocument { id: "concrete".into(), material_id: "concrete".into(), thickness_m: 0.16, lambda: 2.1, mu: 80.0, density: 2300.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                    ],
                },


                EnvelopeElement {
                    id: "floor-ground".into(),
                    kind: "floor".into(),
                    zone_id: "zone-utility".into(),
                    orientation_deg: 0.0,
                    inclination_deg: 0.0,
                    adjacent: "ground".into(),
                    area_m2: 80.0,
                    delta_u_g: 0.0,
                    delta_u_f: 0.0,
                    delta_u_r: 0.0,
                    layers: vec![
                        LayerDocument { id: "screed".into(), material_id: "screed".into(), thickness_m: 0.05, lambda: 1.4, mu: 50.0, density: 2000.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                        LayerDocument { id: "xps".into(), material_id: "xps".into(), thickness_m: 0.10, lambda: 0.035, mu: 100.0, density: 35.0, application_type: "DEO".into(), compressive_class: "dk".into(), water_class: "wd".into(), tensile_class: "tf".into(), acoustic_class: "sm".into(), segments: vec![] },
                        LayerDocument { id: "slab".into(), material_id: "concrete".into(), thickness_m: 0.20, lambda: 2.1, mu: 80.0, density: 2300.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                    ],
                },
            ],
            thermal_bridges: vec![
                ThermalBridge { id: "tb-window-reveal".into(), psi: 0.05, length_m: 24.0, bb2_type: "categoryA".into() },
                ThermalBridge { id: "tb-eaves".into(), psi: 0.10, length_m: 20.0, bb2_type: "categoryA".into() },
            ],
        }
    }

    /// ❌️ Non-compliant envelope with multiple DIN 4108 failures.
    pub fn failing_thin_insulation() -> Self {
        let mut snap = Self::compliant_etics_dwelling();
        snap.airtightness_n50 = 4.5;
        snap.has_mechanical_ventilation = false;
        snap.bb2_details_conform = false;
        snap.thermal_bridges = vec![ThermalBridge { id: "tb-bad".into(), psi: 0.40, length_m: 50.0, bb2_type: "detailed".into() }];
        if let Some(zone) = snap.zones.first_mut() {
            zone.heaviness = "light".into();
            zone.night_ventilation = "none".into();
            zone.windows = vec![
                ZoneWindow { id: "win-south".into(), orientation: "S".into(), inclination_deg: 90.0, area_m2: 28.0, g_value: 0.70, shading_fc: 1.0 },
                ZoneWindow { id: "win-west".into(), orientation: "W".into(), inclination_deg: 90.0, area_m2: 12.0, g_value: 0.70, shading_fc: 1.0 },
            ];
        }
        if let Some(wall) = snap.elements.iter_mut().find(|e| e.id == "wall-north") {
            wall.layers = vec![
                LayerDocument { id: "concrete".into(), material_id: "concrete".into(), thickness_m: 0.20, lambda: 2.1, mu: 80.0, density: 2300.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                LayerDocument { id: "thin-eps".into(), material_id: "eps".into(), thickness_m: 0.004, lambda: 0.035, mu: 40.0, density: 20.0, application_type: "WI".into(), compressive_class: "dh".into(), water_class: "wk".into(), tensile_class: "tk".into(), acoustic_class: "sh".into(), segments: vec![] },
            ];
        }
        if let Some(roof) = snap.elements.iter_mut().find(|e| e.id == "roof") {
            if let Some(wool) = roof.layers.iter_mut().find(|l| l.id == "mineral-wool") {
                wool.thickness_m = 0.02;
            }
        }
        snap
    }

    /// 🪵 Compliant timber-frame wall subject fragment for ISO 6946 §6.7 assessments.
    pub fn with_timber_frame_wall(mut self) -> Self {
        self.elements.push(EnvelopeElement {
            id: "wall-timber-east".into(),
            kind: "wall".into(),
            zone_id: "zone-living".into(),
            orientation_deg: 90.0,
            inclination_deg: 90.0,
            adjacent: "exterior".into(),
            area_m2: 25.0,
            delta_u_g: 0.0,
            delta_u_f: 0.0,
            delta_u_r: 0.0,
            layers: vec![
                LayerDocument { id: "tf-plaster".into(), material_id: "gypsum_plaster".into(), thickness_m: 0.015, lambda: 0.70, mu: 10.0, density: 1400.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                LayerDocument {
                    id: "tf-stud-bay".into(),
                    material_id: "mineral_wool".into(),
                    thickness_m: 0.16,
                    lambda: 0.035,
                    mu: 1.3,
                    density: 30.0,
                    application_type: "WZ".into(), compressive_class: "dm".into(), water_class: "wk".into(), tensile_class: "tk".into(), acoustic_class: "sh".into(), segments: vec![
                        LayerSegment { id: "bay".into(), material_id: "mineral_wool".into(), fraction: 0.85, lambda: 0.035, mu: 1.3, density: 30.0 },
                        LayerSegment { id: "stud".into(), material_id: "softwood".into(), fraction: 0.15, lambda: 0.13, mu: 50.0, density: 500.0 },
                    ],
                },
                LayerDocument { id: "tf-osb".into(), material_id: "osb".into(), thickness_m: 0.015, lambda: 0.13, mu: 50.0, density: 650.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                LayerDocument { id: "tf-wool-ext".into(), material_id: "mineral_wool".into(), thickness_m: 0.06, lambda: 0.035, mu: 1.3, density: 30.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
                LayerDocument { id: "tf-render".into(), material_id: "mineral_render".into(), thickness_m: 0.008, lambda: 0.70, mu: 20.0, density: 1600.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] },
            ],
        });
        self
    }
}
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
/// 📤️ Canonical JSON projection of a [`Din4108Snapshot`].
pub fn encode_din4108_snapshot_json(snapshot: &Din4108Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_din4108_snapshot_json`].
pub fn decode_din4108_snapshot_json(text: &str) -> Result<Din4108Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses a committed `.dsl.semio` artifact into a [`Din4108Snapshot`].
pub fn decode_din4108_dsl(text: &str) -> Result<Din4108Snapshot, String> {
    <Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Din4108Snapshot`] to its canonical `.dsl.semio` body.
pub fn encode_din4108_dsl(snapshot: &Din4108Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`Din4108Snapshot`] from the binary `.pack.semio` envelope.
pub fn decode_din4108_pack(bytes: &[u8]) -> Result<Din4108Snapshot, String> {
    <Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`Din4108Snapshot`] to its binary `.pack.semio` envelope.
pub fn encode_din4108_pack(snapshot: &Din4108Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
