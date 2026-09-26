//! 🧬️ Din4108 artifact schema — every field of the artifact with its state class.

use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full Din4108 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Artifact {
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
    #[state(artifact)]
    pub zones: Vec<crate::ThermalZone>,
    #[state(artifact)]
    pub elements: Vec<crate::EnvelopeElement>,
    #[state(artifact)]
    pub thermal_bridges: Vec<crate::ThermalBridge>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din4108Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::Din4108Snapshot {
        crate::Din4108Snapshot {
            climate_zone: self.climate_zone,
            usage: self.usage.clone(),
            t_int_c: self.t_int_c,
            rh_int: self.rh_int,
            has_mechanical_ventilation: self.has_mechanical_ventilation,
            airtightness_n50: self.airtightness_n50,
            bb2_details_conform: self.bb2_details_conform,
            zones: self.zones.clone(),
            elements: self.elements.clone(),
            thermal_bridges: self.thermal_bridges.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::Din4108Snapshot) -> Self {
        Self {
            climate_zone: snapshot.climate_zone,
            usage: snapshot.usage,
            t_int_c: snapshot.t_int_c,
            rh_int: snapshot.rh_int,
            has_mechanical_ventilation: snapshot.has_mechanical_ventilation,
            airtightness_n50: snapshot.airtightness_n50,
            bb2_details_conform: snapshot.bb2_details_conform,
            zones: snapshot.zones,
            elements: snapshot.elements,
            thermal_bridges: snapshot.thermal_bridges,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::Din4108Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din4108` — twenty handcrafted schema leaves.
pub fn din4108_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.din4108",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Din4108BuilderConstruction {
        snapshot: Din4108Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Din4108BuilderConstruction {
        type Snapshot = Din4108Snapshot;
        type Mutation = Din4108Mutation;
        type Diff = Din4108Diff;
        fn empty() -> Self {
            Self { snapshot: Din4108Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::Din4108Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Din4108Parts {
        pub snapshot: Option<Din4108Snapshot>,
    }

    pub struct Din4108AnalyzerAnalysis;

    impl ArtifactAnalysis for Din4108AnalyzerAnalysis {
        type Parts = Din4108Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.din4108", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Din4108Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Din4108BuilderFacets {
        construction: Din4108BuilderConstruction,
        analysis: Din4108AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Din4108ComposerComposition,
    }
    builder: Din4108Builder,
    analyzer: Din4108Analyzer,
    composer: Din4108Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure DIN 4108 / EN ISO 6946 helpers — formula library; snapshot evaluate lives in inferences.
use crate::document::{
    AnnexChoice, CheckResult, ClauseId, ClimateZoneDe, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef,
};
use crate::{EnvelopeElement, LayerDocument, ThermalBridge, ThermalZone};

/// 🧱 Interior surface resistance for walls (EN ISO 6946 / DIN 4108-2), horizontal heat flow [m²K/W].
pub const R_SI_WALL: f64 = 0.13;
/// 🧱 Exterior surface resistance (EN ISO 6946) [m²K/W].
pub const R_SE: f64 = 0.04;
/// 🧱 Interior surface resistance for roofs (upward heat flow) [m²K/W].
pub const R_SI_ROOF: f64 = 0.10;
/// 🧱 Interior surface resistance for floors (downward heat flow) [m²K/W].
pub const R_SI_FLOOR: f64 = 0.17;
/// 🦠 DIN 4108-2 mould criterion — temperature factor at interior surface.
pub const F_RSI_MINIMUM: f64 = 0.70;
/// ❄️ DIN 4108-3 winter condensed mass limit for most constructions [kg/m²].
pub const GLASER_MASS_LIMIT: f64 = 1.0;
/// ❄️ DIN 4108-3 condensed mass limit for wood / wood-based layers [kg/m²].
pub const GLASER_MASS_LIMIT_WOOD: f64 = 0.5;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn entity_path(collection: &str, id: &str) -> String {
    format!("{collection}[id={id}]")
}

fn entity_field_path(collection: &str, id: &str, field: &str) -> String {
    format!("{collection}[id={id}].{field}")
}

fn layer_field_path(element_id: &str, layer_id: &str, field: &str) -> String {
    format!("elements[id={element_id}].layers[id={layer_id}].{field}")
}

fn window_field_path(zone_id: &str, window_id: &str, field: &str) -> String {
    format!("zones[id={zone_id}].windows[id={window_id}].{field}")
}

fn bridge_field_path(bridge_id: &str, field: &str) -> String {
    format!("thermalBridges[id={bridge_id}].{field}")
}

/// ❄️ DIN 4108-3 Annex A — winter exterior climate for Glaser condensation assessment.
pub const GLASER_WINTER_T_EXT_C: f64 = -5.0;
/// ❄️ DIN 4108-3 Annex A — winter exterior relative humidity.
pub const GLASER_WINTER_RH_EXT: f64 = 0.80;
/// 🏠 DIN 4108-3 Annex A — interior design temperature for Glaser.
pub const GLASER_INTERIOR_T_C: f64 = 20.0;
/// 🏠 DIN 4108-3 Annex A — interior relative humidity for Glaser.
pub const GLASER_INTERIOR_RH: f64 = 0.50;
/// ☀️ DIN 4108-3 Annex A — summer exterior temperature (walls / floors).
pub const GLASER_SUMMER_T_EXT_C: f64 = 12.0;
/// ☀️ DIN 4108-3 Annex A — summer exterior relative humidity.
pub const GLASER_SUMMER_RH_EXT: f64 = 0.70;
/// ☀️ DIN 4108-3 Annex A — summer exterior temperature for roofs (elevated).
pub const GLASER_SUMMER_ROOF_T_EXT_C: f64 = 20.0;


fn surface_resistances(kind: &str, adjacent: &str) -> (f64, f64) {
    let r_si = match kind {
        "roof" => R_SI_ROOF,
        "floor" => R_SI_FLOOR,
        _ => R_SI_WALL,
    };
    let r_se = match adjacent {
        "ground" | "unheated" | "otherHeated" => 0.0,
        _ => R_SE,
    };
    (r_si, r_se)
}

/// 📐️ Homogeneous layer thermal resistance R = d/λ [m²K/W] (EN ISO 6946 §6.1).
pub fn layer_resistance(layer: &LayerDocument) -> f64 {
    if layer.lambda <= 0.0 {
        return f64::INFINITY;
    }
    layer.thickness_m / layer.lambda
}

/// 📐️ ISO 6946 §6.7 lower bound — isothermal planes; inhomogeneous R_i = 1/Σ(f_j/R_ij).
pub fn layer_resistance_lower(layer: &LayerDocument) -> f64 {
    if layer.segments.is_empty() {
        return layer_resistance(layer);
    }
    let mut sum = 0.0;
    for seg in &layer.segments {
        if seg.fraction <= 0.0 || seg.lambda <= 0.0 {
            continue;
        }
        let r_ij = layer.thickness_m / seg.lambda;
        sum += seg.fraction / r_ij;
    }
    if sum <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / sum
    }
}

/// 📐️ ISO 6946 §6.7 upper bound R_T,upper = 1/Σ(f_j/R_Tj) with sections from the first inhomogeneous layer.
pub fn total_resistance_upper(layers: &[LayerDocument], r_si: f64, r_se: f64) -> f64 {
    let Some(primary) = layers.iter().find(|layer| !layer.segments.is_empty()) else {
        return r_si + r_se + layers.iter().map(layer_resistance).sum::<f64>();
    };
    let mut sum_f_over_r = 0.0;
    for (j, seg) in primary.segments.iter().enumerate() {
        if seg.fraction <= 0.0 {
            continue;
        }
        let mut r_tj = r_si + r_se;
        for layer in layers {
            if layer.segments.is_empty() {
                r_tj += layer_resistance(layer);
            } else {
                let section = layer.segments.get(j).unwrap_or(seg);
                if section.lambda <= 0.0 {
                    r_tj = f64::INFINITY;
                    break;
                }
                r_tj += layer.thickness_m / section.lambda;
            }
        }
        if r_tj > 0.0 && r_tj.is_finite() {
            sum_f_over_r += seg.fraction / r_tj;
        }
    }
    if sum_f_over_r <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / sum_f_over_r
    }
}

/// 📐️ ISO 6946 §6.7 lower bound R_T,lower = R_si + Σ R_i + R_se.
pub fn total_resistance_lower(layers: &[LayerDocument], r_si: f64, r_se: f64) -> f64 {
    r_si + r_se + layers.iter().map(layer_resistance_lower).sum::<f64>()
}

/// 📐️ Total resistance R_T [m²K/W] — series for homogeneous stacks; (R_upper+R_lower)/2 for ISO 6946 §6.7.
pub fn total_resistance(layers: &[LayerDocument], r_si: f64, r_se: f64) -> f64 {
    if layers.iter().any(|layer| !layer.segments.is_empty()) {
        0.5 * (total_resistance_upper(layers, r_si, r_se) + total_resistance_lower(layers, r_si, r_se))
    } else {
        r_si + r_se + layers.iter().map(layer_resistance).sum::<f64>()
    }
}

/// 📉️ U = 1/R_T [W/(m²K)].
pub fn u_value(r_total: f64) -> f64 {
    if r_total <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / r_total
    }
}

/// 🌉️ Σ(ψ·l) [W/K].
pub fn psi_l_sum(bridges: &[ThermalBridge]) -> f64 {
    bridges.iter().map(|b| b.psi * b.length_m).sum()
}

/// 📐️ Envelope area of opaque elements [m²].
pub fn opaque_envelope_area(elements: &[EnvelopeElement]) -> f64 {
    elements.iter().filter(|e| e.kind != "window" && e.kind != "door").map(|e| e.area_m2).sum()
}

fn insulation_layer_index(layers: &[LayerDocument]) -> Option<usize> {
    layers
        .iter()
        .enumerate()
        .filter(|(_, l)| l.lambda > 0.0 && l.lambda < 0.1)
        .min_by(|a, b| a.1.lambda.partial_cmp(&b.1.lambda).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
}

fn required_insulation_thickness(layers: &[LayerDocument], r_si: f64, r_se: f64, r_min: f64) -> Option<(usize, f64)> {
    let idx = insulation_layer_index(layers)?;
    let lambda = layers[idx].lambda;
    let r_other = r_si
        + r_se
        + layers
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != idx)
            .map(|(_, l)| layer_resistance(l))
            .sum::<f64>();
    let mut d_req = (r_min - r_other).max(0.0) * lambda;
    // Nudge past f64 rounding so R lands at/above R_min after re-evaluation.
    while r_other + d_req / lambda < r_min {
        d_req = f64::from_bits(d_req.to_bits().saturating_add(1));
    }
    Some((idx, d_req))
}

// #region 🔖️Part2
pub mod part_2 {
    use super::*;

    /// 🧱 Area-weighted surface mass m′ = Σ(ρ·d) [kg/m²] for Table 3 light/heavy columns.
    pub fn surface_mass_kg_m2(layers: &[LayerDocument]) -> f64 {
        layers.iter().map(layer_surface_mass_kg_m2).sum()
    }

    /// ⚖️ Area-weighted layer surface mass (DIN 4108-2 Table 3 / ISO 6946 inhomogeneous layers).
    pub fn layer_surface_mass_kg_m2(layer: &LayerDocument) -> f64 {
        if layer.segments.is_empty() {
            layer.density * layer.thickness_m
        } else {
            layer.segments.iter().map(|seg| seg.fraction * seg.density * layer.thickness_m).sum()
        }
    }


    /// 🧱 DIN 4108-2 Table 3 representative R_min rows (residential, ≥19 °C) — shared with catalogue.
    pub const TABLE3_R_MIN_ROWS: &[(&str, &str, &str, f64)] = &[
        ("wall", "exterior", "heavy", 1.2),
        ("wall", "exterior", "light", 1.75),
        ("wall", "unheated", "heavy", 0.55),
        ("roof", "exterior", "heavy", 1.2),
        ("roof", "exterior", "light", 1.75),
        ("floor", "ground", "heavy", 0.90),
        ("floor", "ground", "light", 1.2),
        ("frameOpaque", "exterior", "heavy", 1.0),
        ("rollerShutterBox", "exterior", "heavy", 1.0),
    ];

    /// 🧱 DIN 4108-2 Table 3 R_min [m²K/W] — usage sets interior temperature class; light ≤100 kg/m².
    pub fn table3_r_min(kind: &str, adjacent: &str, usage: &str, t_int_c: f64, surface_mass_kg_m2: f64) -> f64 {
        let light = surface_mass_kg_m2 > 0.0 && surface_mass_kg_m2 <= 100.0;
        let low_temp = usage == "nonResidential" && t_int_c >= 12.0 && t_int_c < 19.0;
        match kind {
            "window" | "door" => 0.0,
            "frameOpaque" => {
                if low_temp { 0.55 } else if light { 1.4 } else { 1.0 }
            }
            "rollerShutterBox" => {
                if low_temp { 0.70 } else if light { 1.4 } else { 1.0 }
            }
            "wall" => match adjacent {
                "unheated" => {
                    if low_temp { 0.35 } else if light { 0.80 } else { 0.55 }
                }
                _ => {
                    if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
                }
            },
            "roof" => {
                if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
            }
            "floor" => match adjacent {
                "exterior" => {
                    if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
                }
                _ => {
                    if low_temp { 0.35 } else if light { 1.2 } else { 0.90 }
                }
            },
            _ => {
                if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
            }
        }
    }

    /// ☀️ Summer climate region A/B/C from heating climate zone (DIN 4108-2 Annex).
    pub fn summer_region(climate: ClimateZoneDe) -> char {
        match climate {
            ClimateZoneDe::Zone1 => 'A',
            ClimateZoneDe::Zone2 => 'B',
            ClimateZoneDe::Zone3 | ClimateZoneDe::Zone4 => 'C',
        }
    }

    /// ☀️ DIN 4108-2 Table 8 combined S_x for climate + construction + night ventilation.
    pub fn s_x_climate_construction_night(region: char, heaviness: &str, night_ventilation: &str) -> f64 {
        let night = matches!(night_ventilation, "moderate" | "high" | "strong");
        let heavy = heaviness == "heavy";
        let medium = heaviness == "medium";
        match (region, heavy, medium, night) {
            ('A', true, _, false) => 0.10,
            ('A', _, true, false) => 0.07,
            ('A', _, _, false) => 0.04,
            ('A', true, _, true) => 0.13,
            ('A', _, true, true) => 0.10,
            ('A', _, _, true) => 0.06,
            ('B', true, _, false) => 0.08,
            ('B', _, true, false) => 0.06,
            ('B', _, _, false) => 0.03,
            ('B', true, _, true) => 0.10,
            ('B', _, true, true) => 0.08,
            ('B', _, _, true) => 0.05,
            ('C', true, _, false) => 0.06,
            ('C', _, true, false) => 0.045,
            ('C', _, _, false) => 0.025,
            ('C', true, _, true) => 0.08,
            ('C', _, true, true) => 0.06,
            ('C', _, _, true) => 0.035,
            _ => 0.06,
        }
    }

    /// ☀️ DIN 4108-2 §8 solar-control glazing partial S_x,Sonnenschutzglas.
    pub fn s_x_solar_glazing(solar_control: bool) -> f64 {
        if solar_control { 0.03 } else { 0.0 }
    }

    /// ☀️ DIN 4108-2 §8 passive cooling partial S_x,bautechnische Kühlung.
    pub fn s_x_passive_cooling(passive_cooling: bool) -> f64 {
        if passive_cooling { 0.04 } else { 0.0 }
    }

    /// ☀️ DIN 4108-2 §8 S_zul = Σ S_x (Table 8 climate/construction/night + solar + passive).
    pub fn s_zul(region: char, heaviness: &str, night_ventilation: &str) -> f64 {
        s_zul_full(region, heaviness, night_ventilation, false, false)
    }

    /// ☀️ Full S_zul with solar-control and passive-cooling additives.
    pub fn s_zul_full(region: char, heaviness: &str, night_ventilation: &str, solar_control: bool, passive_cooling: bool) -> f64 {
        s_x_climate_construction_night(region, heaviness, night_ventilation)
            + s_x_solar_glazing(solar_control)
            + s_x_passive_cooling(passive_cooling)
    }

    /// ☀️ Detect solar-control glazing (mean g ≤ 0.40) for S_x,Sonnenschutzglas.
    pub fn zone_has_solar_control_glazing(zone: &ThermalZone) -> bool {
        !zone.windows.is_empty() && zone.windows.iter().all(|w| w.g_value <= 0.40)
    }

    /// ☀️ Detect passive cooling measures from night-ventilation class high/strong.
    pub fn zone_has_passive_cooling(zone: &ThermalZone) -> bool {
        matches!(zone.night_ventilation.as_str(), "high" | "strong")
    }

    /// ☀️ Orientation factor F_w (DIN 4108-2 §8 — window orientation contribution).
    pub fn orientation_factor_fw(orientation: &str) -> f64 {
        match orientation {
            "S" | "SE" | "SW" => 1.0,
            "E" | "W" => 0.90,
            "NE" | "NW" => 0.80,
            "N" => 0.70,
            _ => 1.0,
        }
    }

    /// ☀️ Inclination factor F_i (DIN 4108-2 §8 — 0° horizontal … 90° vertical).
    pub fn inclination_factor_fi(inclination_deg: f64) -> f64 {
        let a = inclination_deg.clamp(0.0, 90.0);
        0.70 + 0.30 * (a / 90.0)
    }

    /// ☀️ Orientation factor from element azimuth (degrees from north, clockwise).
    pub fn orientation_factor_from_azimuth_deg(orientation_deg: f64) -> f64 {
        let d = orientation_deg.rem_euclid(360.0);
        let label = if (45.0..135.0).contains(&d) {
            "E"
        } else if (135.0..225.0).contains(&d) {
            "S"
        } else if (225.0..315.0).contains(&d) {
            "W"
        } else {
            "N"
        };
        orientation_factor_fw(label)
    }

    /// ☀️ S_vorh = Σ(A_w · g · F_c · F_w · F_i) / A_G (DIN 4108-2 §8).
    pub fn s_vorhanden(zone: &ThermalZone) -> f64 {
        if zone.floor_area_m2 <= 0.0 {
            return f64::INFINITY;
        }
        let sum: f64 = zone
            .windows
            .iter()
            .map(|w| w.area_m2 * w.g_value * w.shading_fc * orientation_factor_fw(&w.orientation) * inclination_factor_fi(w.inclination_deg))
            .sum();
        sum / zone.floor_area_m2
    }

    /// ☀️ Additional solar entry from transparent envelope elements using orientationDeg/inclinationDeg.
    pub fn s_vorhanden_with_elements(zone: &ThermalZone, elements: &[EnvelopeElement]) -> f64 {
        if zone.floor_area_m2 <= 0.0 {
            return f64::INFINITY;
        }
        let mut sum: f64 = zone
            .windows
            .iter()
            .map(|w| w.area_m2 * w.g_value * w.shading_fc * orientation_factor_fw(&w.orientation) * inclination_factor_fi(w.inclination_deg))
            .sum();
        for el in elements.iter().filter(|e| e.zone_id == zone.id && (e.kind == "window" || e.kind == "door")) {
            let g = 0.50;
            let fc = 1.0;
            sum += el.area_m2 * g * fc * orientation_factor_from_azimuth_deg(el.orientation_deg) * inclination_factor_fi(el.inclination_deg);
        }
        sum / zone.floor_area_m2
    }


    /// 🌡 Zone transmission heat-loss coefficient H_T = Σ(A·U) [W/K] for opaque elements linked to the zone.
    pub fn zone_transmission_ht_wk(zone_id: &str, elements: &[EnvelopeElement]) -> f64 {
        elements
            .iter()
            .filter(|e| e.zone_id == zone_id && e.kind != "window" && e.kind != "door" && !e.layers.is_empty())
            .map(|e| {
                let (r_si, r_se) = surface_resistances(&e.kind, &e.adjacent);
                let u = u_value(total_resistance(&e.layers, r_si, r_se)) + e.delta_u_g + e.delta_u_f + e.delta_u_r;
                e.area_m2 * u
            })
            .sum()
    }

    /// 🌡 Zone Table 3 capacity H_T,max = Σ(A/R_min) [W/K] for the same opaque set.
    pub fn zone_transmission_ht_max_wk(zone_id: &str, elements: &[EnvelopeElement], usage: &str, t_int_c: f64) -> f64 {
        elements
            .iter()
            .filter(|e| e.zone_id == zone_id && e.kind != "window" && e.kind != "door" && !e.layers.is_empty())
            .map(|e| {
                let mass = surface_mass_kg_m2(&e.layers);
                let r_min = table3_r_min(&e.kind, &e.adjacent, usage, t_int_c, mass);
                if r_min > 0.0 {
                    e.area_m2 / r_min
                } else {
                    0.0
                }
            })
            .sum()
    }

    /// ✅️ DIN 4108-2 zone-aggregated transmission heat loss vs Table 3 capacity.
    pub fn check_zone_transmission_loss(zone: &ThermalZone, elements: &[EnvelopeElement], usage: &str, t_int_c: f64) -> CheckResult {
        let subject = SubjectRef::new(
            &zone.id,
            entity_path("zones", &zone.id),
            loc(&format!("Zone {}", zone.id), &format!("Zone {}", zone.id)),
        );
        let ht = zone_transmission_ht_wk(&zone.id, elements);
        let ht_max = zone_transmission_ht_max_wk(&zone.id, elements, usage, t_int_c);
        let mut builder = CheckResult::assess(
            format!("din4108-2.zone-ht.{}", zone.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "Table 3", "H_T"),
            subject.clone(),
            loc("Zone transmission heat-loss H_T", "Zonaler Transmissionswärmeverlust H_T"),
        )
        .annex(AnnexChoice::De);
        if ht_max <= 0.0 && ht <= 0.0 {
            builder = builder.not_applicable(loc(
                "No opaque elements assigned to this zone.",
                "Dieser Zone sind keine opaken Bauteile zugeordnet.",
            ));
        } else {
            let limit = ht_max.max(1e-9);
            builder = builder
                .utilization(Quantity::new(QuantityKind::HeatTransferCoefficient, ht), Quantity::new(QuantityKind::HeatTransferCoefficient, limit))
                .explanation(loc(
                    &format!("H_T = Σ(A·U) = {ht:.3} W/K vs Table 3 capacity {ht_max:.3} W/K for zone '{}'.", zone.id),
                    &format!("H_T = Σ(A·U) = {ht:.3} W/K gegenüber Tab.3-Kapazität {ht_max:.3} W/K für Zone '{}'.", zone.id),
                ));
            if ht > ht_max {
                if let Some(el) = elements.iter().find(|e| e.zone_id == zone.id && e.kind != "window" && e.kind != "door") {
                    let (r_si, r_se) = surface_resistances(&el.kind, &el.adjacent);
                    let mass = surface_mass_kg_m2(&el.layers);
                    let r_min = table3_r_min(&el.kind, &el.adjacent, usage, t_int_c, mass);
                    if let Some((idx, d_req)) = required_insulation_thickness(&el.layers, r_si, r_se, r_min) {
                        let layer = &el.layers[idx];
                        builder = builder.remedy(Remedy::at_least(
                            SubjectRef::new(
                                &el.id,
                                layer_field_path(&el.id, &layer.id, "thicknessM"),
                                loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                            ),
                            Quantity::length_m(layer.thickness_m),
                            Quantity::length_m(d_req),
                            loc(
                                &format!("Increase insulation on '{}' so zone '{}' H_T ≤ {ht_max:.3} W/K.", el.id, zone.id),
                                &format!("Dämmung an '{}' erhöhen, damit H_T der Zone '{}' ≤ {ht_max:.3} W/K.", el.id, zone.id),
                            ),
                        ));
                    }
                }
            }
        }
        builder.build()
    }

    /// ✅️ Minimum R per DIN 4108-2 Table 3 for one opaque element.
    pub fn check_minimum_r(element: &EnvelopeElement, usage: &str, t_int_c: f64) -> CheckResult {
        let subject = SubjectRef::new(
            &element.id,
            entity_path("elements", &element.id),
            loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
        );
        if element.kind == "window" || element.kind == "door" {
            return CheckResult::assess(
                format!("din4108-2.table3.{}", element.id),
                "DIN 4108-2",
                ClauseId::new("DIN 4108-2", "Table 3", "R_min"),
                subject,
                loc("Minimum thermal resistance (Table 3)", "Mindestwärmedurchlasswiderstand (Tabelle 3)"),
            )
            .not_applicable(loc("Transparent elements are outside Table 3 opaque R_min.", "Transparente Bauteile liegen außerhalb von Tabelle 3."))
            .annex(AnnexChoice::De)
            .build();
        }
        let (r_si, r_se) = surface_resistances(&element.kind, &element.adjacent);
        let r = total_resistance(&element.layers, r_si, r_se);
        let mass = surface_mass_kg_m2(&element.layers);
        let r_min = table3_r_min(&element.kind, &element.adjacent, usage, t_int_c, mass);
        let mut builder = CheckResult::assess(
            format!("din4108-2.table3.{}", element.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "Table 3", "R_min"),
            subject.clone(),
            loc("Minimum thermal resistance (Table 3)", "Mindestwärmedurchlasswiderstand (Tabelle 3)"),
        )
        .minimum(Quantity::thermal_resistance_m2k_w(r), Quantity::thermal_resistance_m2k_w(r_min))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!(
                "R = {r:.3} vs R_min = {r_min:.2} (mass={:.0}, usage={usage}, zone={}, orient={:.0}, incl={:.0}) for {} / {}.",
                surface_mass_kg_m2(&element.layers), element.zone_id, element.orientation_deg, element.inclination_deg, element.kind, element.adjacent
            ),
            &format!(
                "R = {r:.3} gegenüber R_min = {r_min:.2} (Masse={:.0}, Nutzung={usage}, Zone={}, Orient={:.0}, Neigung={:.0}) für {} / {}.",
                surface_mass_kg_m2(&element.layers), element.zone_id, element.orientation_deg, element.inclination_deg, element.kind, element.adjacent
            ),
        ));
        if r < r_min {
            if let Some((idx, d_req)) = required_insulation_thickness(&element.layers, r_si, r_se, r_min) {
                let layer = &element.layers[idx];
                let target = SubjectRef::new(
                    &element.id,
                    layer_field_path(&element.id, &layer.id, "thicknessM"),
                    loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                );
                builder = builder.remedy(Remedy::at_least(
                    target,
                    Quantity::length_m(layer.thickness_m),
                    Quantity::length_m(d_req),
                    loc(
                        &format!(
                            "Increase insulation layer '{}' from {:.0} mm to at least {:.0} mm so R ≥ {:.2} m²K/W.",
                            layer.id,
                            layer.thickness_m * 1000.0,
                            d_req * 1000.0,
                            r_min
                        ),
                        &format!(
                            "Dämmstärke der Schicht '{}' von {:.0} mm auf mindestens {:.0} mm erhöhen, damit R ≥ {:.2} m²K/W.",
                            layer.id,
                            layer.thickness_m * 1000.0,
                            d_req * 1000.0,
                            r_min
                        ),
                    ),
                ));
            }
        }
        builder.build()
    }

    /// ✅️ Interior surface temperature factor f_Rsi ≥ 0.70 (DIN 4108-2 mould criterion).
    pub fn check_f_rsi(element: &EnvelopeElement, t_int_c: f64, t_ext_c: f64) -> CheckResult {
        let subject = SubjectRef::new(
            &element.id,
            entity_path("elements", &element.id),
            loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
        );
        if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
            return CheckResult::assess(
                format!("din4108-2.frsi.{}", element.id),
                "DIN 4108-2",
                ClauseId::new("DIN 4108-2", "§6", "6.2"),
                subject,
                loc("Mould criterion f_Rsi", "Schimmelpilzkriterium f_Rsi"),
            )
            .not_applicable(loc("No opaque layer stack to evaluate f_Rsi.", "Kein opaker Schichtaufbau für f_Rsi."))
            .annex(AnnexChoice::De)
            .build();
        }
        let (r_si, r_se) = surface_resistances(&element.kind, &element.adjacent);
        let r_t = total_resistance(&element.layers, r_si, r_se);
        let f_rsi = if (t_int_c - t_ext_c).abs() < f64::EPSILON {
            1.0
        } else {
            1.0 - r_si / r_t
        };
        let mut builder = CheckResult::assess(
            format!("din4108-2.frsi.{}", element.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "§6", "6.2"),
            subject,
            loc("Mould criterion f_Rsi", "Schimmelpilzkriterium f_Rsi"),
        )
        .minimum(Quantity::new(QuantityKind::Dimensionless, f_rsi), Quantity::new(QuantityKind::Dimensionless, F_RSI_MINIMUM))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("f_Rsi = 1 − R_si/R_T = {f_rsi:.3} (θ_i={t_int_c:.1} °C, θ_e={t_ext_c:.1} °C)."),
            &format!("f_Rsi = 1 − R_si/R_T = {f_rsi:.3} (θ_i={t_int_c:.1} °C, θ_e={t_ext_c:.1} °C)."),
        ));
        if f_rsi < F_RSI_MINIMUM {
            let r_needed = r_si / (1.0 - F_RSI_MINIMUM);
            if let Some((idx, d_req)) = required_insulation_thickness(&element.layers, r_si, r_se, r_needed) {
                let layer = &element.layers[idx];
                builder = builder.remedy(Remedy::at_least(
                    SubjectRef::new(
                        &element.id,
                        layer_field_path(&element.id, &layer.id, "thicknessM"),
                        loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                    ),
                    Quantity::length_m(layer.thickness_m),
                    Quantity::length_m(d_req),
                    loc(
                        &format!(
                            "Increase insulation '{}' from {:.0} mm to ≥ {:.0} mm so f_Rsi ≥ 0.70.",
                            layer.id,
                            layer.thickness_m * 1000.0,
                            d_req * 1000.0
                        ),
                        &format!(
                            "Dämmstärke '{}' von {:.0} mm auf ≥ {:.0} mm erhöhen, damit f_Rsi ≥ 0,70.",
                            layer.id,
                            layer.thickness_m * 1000.0,
                            d_req * 1000.0
                        ),
                    ),
                ));
            }
        }
        builder.build()
    }

    /// ✅️ Summer heat protection S_vorh ≤ S_zul (DIN 4108-2 §8).
    pub fn check_summer_heat(zone: &ThermalZone, climate: ClimateZoneDe, elements: &[EnvelopeElement]) -> CheckResult {
        let subject = SubjectRef::new(
            &zone.id,
            entity_field_path("zones", &zone.id, "windows"),
            loc(&format!("Zone {}", zone.id), &format!("Zone {}", zone.id)),
        );
        if zone.windows.is_empty() || zone.floor_area_m2 <= 0.0 {
            return CheckResult::assess(
                format!("din4108-2.summer.{}", zone.id),
                "DIN 4108-2",
                ClauseId::new("DIN 4108-2", "§8", "8.3"),
                subject,
                loc("Summer heat protection S_vorh", "Sommerlicher Wärmeschutz S_vorh"),
            )
            .not_applicable(loc("No windows or zero floor area in zone.", "Keine Fenster oder A_G = 0 in der Zone."))
            .annex(AnnexChoice::De)
            .build();
        }
        let region = summer_region(climate);
        let s_v = s_vorhanden_with_elements(zone, elements);
        let solar = zone_has_solar_control_glazing(zone);
        let passive = zone_has_passive_cooling(zone);
        let s_z = s_zul_full(region, &zone.heaviness, &zone.night_ventilation, solar, passive);
        let mut builder = CheckResult::assess(
            format!("din4108-2.summer.{}", zone.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "§8", "8.3"),
            subject,
            loc("Summer heat protection S_vorh", "Sommerlicher Wärmeschutz S_vorh"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, s_v), Quantity::new(QuantityKind::Dimensionless, s_z))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!(
                "S_vorh = {s_v:.4}, S_zul = {s_z:.4} (region {region}, {}, night={}, windowIncl={:?}).",
                zone.heaviness, zone.night_ventilation, zone.windows.iter().map(|w| w.inclination_deg).collect::<Vec<_>>()
            ),
            &format!(
                "S_vorh = {s_v:.4}, S_zul = {s_z:.4} (Region {region}, {}, Nachtlüftung={}, FensterNeigung={:?}).",
                zone.heaviness, zone.night_ventilation, zone.windows.iter().map(|w| w.inclination_deg).collect::<Vec<_>>()
            ),
        ));
        if s_v > s_z {
            if let Some((wi, win)) = zone.windows.iter().enumerate().max_by(|a, b| {
                let wa = a.1.area_m2 * a.1.g_value * a.1.shading_fc * orientation_factor_fw(&a.1.orientation) * inclination_factor_fi(a.1.inclination_deg);
                let wb = b.1.area_m2 * b.1.g_value * b.1.shading_fc * orientation_factor_fw(&b.1.orientation) * inclination_factor_fi(b.1.inclination_deg);
                wa.partial_cmp(&wb).unwrap_or(std::cmp::Ordering::Equal)
            }) {
                let fw = orientation_factor_fw(&win.orientation);
                let fi = inclination_factor_fi(win.inclination_deg);
                let budget = s_z * zone.floor_area_m2;
                let other: f64 = zone
                    .windows
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != wi)
                    .map(|(_, w)| w.area_m2 * w.g_value * w.shading_fc * orientation_factor_fw(&w.orientation) * inclination_factor_fi(w.inclination_deg))
                    .sum();
                let allowed = (budget - other).max(0.0);
                let denom = win.area_m2 * win.g_value * fw * fi;
                let fc_req = if denom > 0.0 {
                    (allowed / denom).clamp(0.05, 1.0)
                } else {
                    win.shading_fc
                };
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &win.id,
                        window_field_path(&zone.id, &win.id, "shadingFc"),
                        loc(&format!("Window {}", win.id), &format!("Fenster {}", win.id)),
                    ),
                    Quantity::new(QuantityKind::Dimensionless, win.shading_fc),
                    Quantity::new(QuantityKind::Dimensionless, fc_req),
                    loc(
                        &format!("Reduce shading factor F_c of '{}' from {:.2} to ≤ {:.2} (or reduce g / window area).", win.id, win.shading_fc, fc_req),
                        &format!("Abminderungsfaktor F_c von '{}' von {:.2} auf ≤ {:.2} senken (oder g / Fensterfläche reduzieren).", win.id, win.shading_fc, fc_req),
                    ),
                ));
                let area_denom = win.g_value * win.shading_fc * fw * fi;
                let area_req = if area_denom > 0.0 {
                    allowed / area_denom
                } else {
                    win.area_m2
                };
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &win.id,
                        window_field_path(&zone.id, &win.id, "areaM2"),
                        loc(&format!("Window {}", win.id), &format!("Fenster {}", win.id)),
                    ),
                    Quantity::area_m2(win.area_m2),
                    Quantity::area_m2(area_req),
                    loc(
                        &format!("Reduce window area of '{}' from {:.1} m² to ≤ {:.1} m².", win.id, win.area_m2, area_req),
                        &format!("Fensterfläche von '{}' von {:.1} m² auf ≤ {:.1} m² reduzieren.", win.id, win.area_m2, area_req),
                    ),
                ));
            }
        }
        builder.build()
    }
}
// #endregion 🔖️Part2

// #region 🔖️Part3
pub mod part_3 {
    use super::*;

    /// 🌫️ Saturation vapor pressure (Magnus) [Pa], T in °C.
    pub fn saturation_vapor_pressure_pa(t_c: f64) -> f64 {
        611.2 * (17.67 * t_c / (t_c + 243.5)).exp()
    }

    /// 🧱 sd = μ · d [m].
    pub fn sd_m(layer: &LayerDocument) -> f64 {
        layer.thickness_m * layer_mu_eq(layer)
    }

    /// 💧 Area-weighted μ for inhomogeneous layers (DIN 4108-3 / DIN EN ISO 13788 practice).
    pub fn layer_mu_eq(layer: &LayerDocument) -> f64 {
        if layer.segments.is_empty() {
            layer.mu
        } else {
            layer.segments.iter().map(|seg| seg.fraction * seg.mu).sum()
        }
    }


    pub fn interface_temperatures_c(layers: &[LayerDocument], r_si: f64, r_se: f64, t_int_c: f64, t_ext_c: f64) -> Vec<f64> {
        let r_layers: Vec<f64> = layers.iter().map(layer_resistance).collect();
        let r_total = r_si + r_se + r_layers.iter().sum::<f64>();
        let mut temps = Vec::with_capacity(layers.len() + 2);
        let mut r_cum = 0.0;
        temps.push(t_int_c);
        r_cum += r_si;
        temps.push(t_int_c - (r_cum / r_total) * (t_int_c - t_ext_c));
        for r in &r_layers {
            r_cum += r;
            temps.push(t_int_c - (r_cum / r_total) * (t_int_c - t_ext_c));
        }
        temps
    }

    /// 💧️ Glaser condensed mass for one climate period [kg/m²]; positive = condensation.
    pub fn glaser_condensed_mass_kg_m2(layers: &[LayerDocument], r_si: f64, r_se: f64, t_int_c: f64, rh_int: f64, t_ext_c: f64, rh_ext: f64, hours: f64) -> f64 {
        if layers.is_empty() {
            return 0.0;
        }
        let temps = interface_temperatures_c(layers, r_si, r_se, t_int_c, t_ext_c);
        let p_int = rh_int * saturation_vapor_pressure_pa(t_int_c);
        let p_ext = rh_ext * saturation_vapor_pressure_pa(t_ext_c);
        let sd: Vec<f64> = layers.iter().map(sd_m).collect();
        let sd_total: f64 = sd.iter().sum();
        if sd_total <= 0.0 {
            return 0.0;
        }
        let mut p = Vec::with_capacity(layers.len() + 1);
        p.push(p_int);
        let mut sd_cum = 0.0;
        for s in &sd {
            sd_cum += s;
            p.push(p_int + (p_ext - p_int) * (sd_cum / sd_total));
        }
        let mut mass = 0.0;
        for i in 0..=layers.len() {
            let t = if i + 1 < temps.len() { temps[i + 1] } else { temps[temps.len() - 1] };
            let p_sat = saturation_vapor_pressure_pa(t);
            if p[i] > p_sat {
                let delta_p = p[i] - p_sat;
                let sd_left = if i == 0 { 0.1 } else { sd[..i].iter().sum::<f64>().max(0.1) };
                let sd_right = if i >= sd.len() { 0.1 } else { sd[i..].iter().sum::<f64>().max(0.1) };
                let g = delta_p * (1.0 / sd_left + 1.0 / sd_right) * 2.0e-10;
                mass += g * hours;
            }
        }
        mass
    }


    /// ❄️ Glaser winter exterior temperature from climate zone design value (DIN 4108-3 Annex A cases).
    pub fn glaser_winter_t_ext_c(climate: ClimateZoneDe) -> f64 {
        climate.design_external_temperature_c()
    }

    /// ☀️ Glaser summer exterior temperature — Annex A wall/roof cases scaled by climate summer design.
    pub fn glaser_summer_t_ext_c(climate: ClimateZoneDe, kind: &str) -> f64 {
        let summer = climate.summer_design_temperature_c();
        if kind == "roof" {
            summer - 8.0
        } else {
            summer - 16.0
        }
    }

    /// ✅️ DIN 4108-3 Glaser / periodic condensation check.
    pub fn check_glaser(element: &EnvelopeElement, t_int_c: f64, rh_int: f64, climate: ClimateZoneDe) -> CheckResult {
        let climate_label = format!("{climate:?}");
        let subject = SubjectRef::new(
            &element.id,
            entity_path("elements", &element.id),
            loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
        );
        if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
            return CheckResult::assess(
                format!("din4108-3.glaser.{}", element.id),
                "DIN 4108-3",
                ClauseId::new("DIN 4108-3", "§4", "4.3"),
                subject,
                loc("Glaser condensation mass", "Glaser-Kondensatmenge"),
            )
            .not_applicable(loc("No opaque stack for Glaser analysis.", "Kein opaker Aufbau für Glaser-Verfahren."))
            .annex(AnnexChoice::De)
            .build();
        }
        let (r_si, r_se) = surface_resistances(&element.kind, &element.adjacent);
        let t_int = if t_int_c > 0.0 { t_int_c } else { GLASER_INTERIOR_T_C };
        let rh_i = if rh_int > 0.0 { rh_int } else { GLASER_INTERIOR_RH };
        let t_ext_w = glaser_winter_t_ext_c(climate);
        let rh_ext_w = GLASER_WINTER_RH_EXT;
        let t_ext_s = glaser_summer_t_ext_c(climate, &element.kind);
        let rh_ext_s = GLASER_SUMMER_RH_EXT;
        let m_winter = glaser_condensed_mass_kg_m2(&element.layers, r_si, r_se, t_int, rh_i, t_ext_w, rh_ext_w, 90.0 * 24.0);
        let m_summer = glaser_condensed_mass_kg_m2(&element.layers, r_si, r_se, t_int, rh_i, t_ext_s, rh_ext_s, 90.0 * 24.0);
        let wood = element.layers.iter().any(|l| l.material_id.contains("wood") || l.material_id.contains("timber"));
        let limit = if wood { GLASER_MASS_LIMIT_WOOD } else { GLASER_MASS_LIMIT };
        let net = m_winter - m_summer.max(0.0);
        let condensed = m_winter.max(0.0);
        let dries = m_summer <= 0.0 || condensed <= m_summer.abs() + 1e-9;
        let mut builder = CheckResult::assess(
            format!("din4108-3.glaser.{}", element.id),
            "DIN 4108-3",
            ClauseId::new("DIN 4108-3", "§4", "4.3"),
            subject,
            loc("Glaser condensation mass", "Glaser-Kondensatmenge"),
        )
        .utilization(Quantity::new(QuantityKind::Mass, condensed), Quantity::new(QuantityKind::Mass, limit))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!(
                "Climate {climate_label} BC winter {t_ext_w:.0}°C/{rh_ext_w:.0%}, summer {t_ext_s:.0}°C/{rh_ext_s:.0%}: condensate {condensed:.3} kg/m² (limit {limit}), dry-out ok={dries}, net={net:.3}.",
            ),
            &format!(
                "Klima {climate_label} RB Winter {t_ext_w:.0}°C/{rh_ext_w:.0%}, Sommer {t_ext_s:.0}°C/{rh_ext_s:.0%}: Kondensat {condensed:.3} kg/m² (Grenze {limit}), Austrocknung ok={dries}, netto={net:.3}.",
            ),
        ));
        if condensed > limit || !dries {
            let interior_mu_idx = 0usize;
            let layer = &element.layers[interior_mu_idx];
            let sd_req = (layer.mu * layer.thickness_m * (condensed / limit).max(1.2)).max(layer.mu * layer.thickness_m);
            let mu_req = if layer.thickness_m > 0.0 { sd_req / layer.thickness_m } else { layer.mu };
            builder = builder.remedy(Remedy::at_least(
                SubjectRef::new(
                    &element.id,
                    layer_field_path(&element.id, &layer.id, "mu"),
                    loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
                ),
                Quantity::new(QuantityKind::Dimensionless, layer.mu),
                Quantity::new(QuantityKind::Dimensionless, mu_req),
                loc(
                    &format!(
                        "Raise vapour resistance of interior layer '{}' (μ from {:.1} to ≥ {:.1}, sd ≥ {:.2} m) or add a vapour barrier.",
                        layer.id, layer.mu, mu_req, sd_req
                    ),
                    &format!(
                        "Dampfdurchlasswiderstand der inneren Schicht '{}' erhöhen (μ von {:.1} auf ≥ {:.1}, sd ≥ {:.2} m) oder Dampfbremse ergänzen.",
                        layer.id, layer.mu, mu_req, sd_req
                    ),
                ),
            ));
        }
        builder.build()
    }
}
// #endregion 🔖️Part3

// #region 🔖️Part4
pub mod part_4 {
    use super::*;

    /// 📊️ DIN 4108-4 Table 1 design thermal conductivity λ [W/(m·K)] — shared with catalogue.
    pub const DESIGN_LAMBDA_ROWS: &[(&str, f64)] = &[
        ("mineral_wool", 0.035),
        ("glass_wool", 0.035),
        ("rock_wool", 0.035),
        ("eps", 0.035),
        ("eps_w", 0.035),
        ("xps", 0.035),
        ("pu", 0.028),
        ("pir", 0.028),
        ("brick", 0.81),
        ("masonry", 0.81),
        ("concrete", 2.1),
        ("gypsum_plaster", 0.70),
        ("gypsum", 0.70),
        ("mineral_render", 0.70),
        ("screed", 1.4),
        ("timber", 0.13),
        ("wood", 0.13),
        ("softwood", 0.13),
        ("osb", 0.13),
        ("aerated_concrete", 0.16),
        ("lime_sandstone", 0.99),
    ];

    /// 📊️ DIN 4108-4 design λ for a catalogue material id.
    pub fn design_lambda(material_id: &str) -> Option<f64> {
        DESIGN_LAMBDA_ROWS.iter().find(|(id, _)| *id == material_id).map(|(_, v)| *v)
    }

    /// 📊️ Catalogue material ids for referential integrity remedies.
    pub fn design_lambda_material_ids() -> Vec<String> {
        DESIGN_LAMBDA_ROWS.iter().map(|(id, _)| (*id).to_string()).collect()
    }

    /// ✅️ Declared λ of each layer must not exceed DIN 4108-4 design value.
    pub fn check_design_lambda(element: &EnvelopeElement, layer_index: usize) -> CheckResult {
        let layer = &element.layers[layer_index];
        let subject = SubjectRef::new(
            &element.id,
            layer_field_path(&element.id, &layer.id, "lambda"),
            loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
        );
        let Some(limit) = design_lambda(&layer.material_id) else {
            return CheckResult::assess(
                format!("din4108-4.lambda.{}.{}", element.id, layer.id),
                "DIN 4108-4",
                ClauseId::new("DIN 4108-4", "Table 1", "λ"),
                subject,
                loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
            )
            .not_applicable(loc(
                &format!("Material '{}' has no DIN 4108-4 row in this catalogue.", layer.material_id),
                &format!("Werkstoff '{}' ist in diesem Katalog nicht tabelliert.", layer.material_id),
            ))
            .annex(AnnexChoice::De)
            .build();
        };
        let mut builder = CheckResult::assess(
            format!("din4108-4.lambda.{}.{}", element.id, layer.id),
            "DIN 4108-4",
            ClauseId::new("DIN 4108-4", "Table 1", "λ"),
            subject.clone(),
            loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
        )
        .utilization(
            Quantity::new(QuantityKind::ThermalConductivity, layer.lambda),
            Quantity::new(QuantityKind::ThermalConductivity, limit),
        )
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("λ = {:.3} W/(m·K) vs λ_design = {:.3} for '{}'.", layer.lambda, limit, layer.material_id),
            &format!("λ = {:.3} W/(m·K) gegenüber λ_Bemessung = {:.3} für '{}'.", layer.lambda, limit, layer.material_id),
        ));
        if layer.lambda > limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                Quantity::new(QuantityKind::ThermalConductivity, layer.lambda),
                Quantity::new(QuantityKind::ThermalConductivity, limit),
                loc(
                    &format!("Reduce declared λ of '{}' from {:.3} to ≤ {:.3} W/(m·K) or change material_id.", layer.id, layer.lambda, limit),
                    &format!("Deklariertes λ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken oder materialId wechseln.", layer.id, layer.lambda, limit),
                ),
            ));
        }
        builder.build()
    }

    /// ✅️ DIN 4108-4 λ for one inhomogeneous segment (ISO 6946 §6.7 / DIN 4108-4 Table 1).
    pub fn check_segment_design_lambda(element: &EnvelopeElement, layer_index: usize, segment_index: usize) -> CheckResult {
        let layer = &element.layers[layer_index];
        let seg = &layer.segments[segment_index];
        let subject = SubjectRef::new(
            &seg.id,
            format!("elements[id={}].layers[id={}].segments[id={}].lambda", element.id, layer.id, seg.id),
            loc(&format!("Segment {}", seg.id), &format!("Abschnitt {}", seg.id)),
        );
        let Some(limit) = design_lambda(&seg.material_id) else {
            return CheckResult::assess(
                format!("din4108-4.lambda.{}.{}.{}", element.id, layer.id, seg.id),
                "DIN 4108-4",
                ClauseId::new("DIN 4108-4", "Table 1", "λ"),
                subject,
                loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
            )
            .not_applicable(loc(
                &format!("Segment material '{}' has no DIN 4108-4 row.", seg.material_id),
                &format!("Abschnittswerkstoff '{}' ist nicht tabelliert.", seg.material_id),
            ))
            .annex(AnnexChoice::De)
            .build();
        };
        let mut builder = CheckResult::assess(
            format!("din4108-4.lambda.{}.{}.{}", element.id, layer.id, seg.id),
            "DIN 4108-4",
            ClauseId::new("DIN 4108-4", "Table 1", "λ"),
            subject.clone(),
            loc("Design thermal conductivity", "Bemessungswärmeleitfähigkeit"),
        )
        .utilization(
            Quantity::new(QuantityKind::ThermalConductivity, seg.lambda),
            Quantity::new(QuantityKind::ThermalConductivity, limit),
        )
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("Segment λ = {:.3} vs λ_design = {:.3} for '{}' (ρ={:.0}, μ={:.1}).", seg.lambda, limit, seg.material_id, seg.density, seg.mu),
            &format!("Abschnitt λ = {:.3} gegenüber λ_design = {:.3} für '{}' (ρ={:.0}, μ={:.1}).", seg.lambda, limit, seg.material_id, seg.density, seg.mu),
        ));
        if seg.lambda > limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                Quantity::new(QuantityKind::ThermalConductivity, seg.lambda),
                Quantity::new(QuantityKind::ThermalConductivity, limit),
                loc(
                    &format!("Reduce segment λ of '{}' from {:.3} to ≤ {:.3} W/(m·K).", seg.id, seg.lambda, limit),
                    &format!("Abschnitts-λ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken.", seg.id, seg.lambda, limit),
                ),
            ));
        }
        builder.build()
    }
}
// #endregion 🔖️Part4

// #region 🔖️Part6
pub mod part_6 {
    use super::*;

    /// 📉️ Bridged U' = U + Σ(ψ·l)/A [W/(m²K)].
    pub fn u_with_bridges(u: f64, psi_l: f64, area_m2: f64) -> f64 {
        if area_m2 <= 0.0 {
            return f64::INFINITY;
        }
        u + psi_l / area_m2
    }

    /// ✅️ EN ISO 6946 U including §6.7 and Annex F / fastener / inverted-roof ΔU — skipped when identical to Table 3.
    pub fn check_u_value(element: &EnvelopeElement, usage: &str, t_int_c: f64) -> CheckResult {
        let subject = SubjectRef::new(
            &element.id,
            entity_path("elements", &element.id),
            loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
        );
        if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
            return CheckResult::assess(
                format!("din4108-6.u.{}", element.id),
                "EN ISO 6946",
                ClauseId::new("EN ISO 6946", "§6", "6.1"),
                subject,
                loc("Thermal transmittance U with ΔU corrections", "Wärmedurchgangskoeffizient U mit ΔU-Korrekturen"),
            )
            .not_applicable(loc("No opaque ISO 6946 stack.", "Kein opaker ISO-6946-Aufbau."))
            .annex(AnnexChoice::De)
            .build();
        }
        let inhomogeneous = element.layers.iter().any(|layer| !layer.segments.is_empty());
        let delta = element.delta_u_g + element.delta_u_f + element.delta_u_r;
        if !inhomogeneous && delta.abs() < 1e-12 {
            return CheckResult::assess(
                format!("din4108-6.u.{}", element.id),
                "EN ISO 6946",
                ClauseId::new("EN ISO 6946", "§6", "6.1"),
                subject,
                loc("Thermal transmittance U with ΔU corrections", "Wärmedurchgangskoeffizient U mit ΔU-Korrekturen"),
            )
            .not_applicable(loc(
                "Homogeneous stack without ΔU_g/ΔU_f/ΔU_r — covered by DIN 4108-2 Table 3 R_min.",
                "Homogener Aufbau ohne ΔU_g/ΔU_f/ΔU_r — durch DIN 4108-2 Tabelle 3 R_min abgedeckt.",
            ))
            .annex(AnnexChoice::De)
            .build();
        }
        let (r_si, r_se) = surface_resistances(&element.kind, &element.adjacent);
        let r = total_resistance(&element.layers, r_si, r_se);
        let u = u_value(r) + delta;
        let mass = part_2::surface_mass_kg_m2(&element.layers);
        let r_min = part_2::table3_r_min(&element.kind, &element.adjacent, usage, t_int_c, mass);
        let u_max = if r_min > 0.0 { 1.0 / r_min } else { f64::INFINITY };
        let clause = if inhomogeneous { "6.7" } else { "6.1" };
        let mut builder = CheckResult::assess(
            format!("din4108-6.u.{}", element.id),
            "EN ISO 6946",
            ClauseId::new("EN ISO 6946", "§6", clause),
            subject.clone(),
            loc("Thermal transmittance U with ΔU corrections", "Wärmedurchgangskoeffizient U mit ΔU-Korrekturen"),
        )
        .utilization(Quantity::u_value_w_m2k(u), Quantity::u_value_w_m2k(u_max))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!(
                "U+ΔU = {u:.3} (ΔU_g={:.3}/f={:.3}/r={:.3}, orient={:.0}°, incl={:.0}°, zone={}) vs U_max = {u_max:.3}.",
                element.delta_u_g, element.delta_u_f, element.delta_u_r, element.orientation_deg, element.inclination_deg, element.zone_id
            ),
            &format!(
                "U+ΔU = {u:.3} (ΔU_g={:.3}/f={:.3}/r={:.3}, Orient={:.0}°, Neigung={:.0}°, Zone={}) gegenüber U_max = {u_max:.3}.",
                element.delta_u_g, element.delta_u_f, element.delta_u_r, element.orientation_deg, element.inclination_deg, element.zone_id
            ),
        ));
        if u > u_max {
            if let Some((idx, d_req)) = required_insulation_thickness(&element.layers, r_si, r_se, r_min) {
                let layer = &element.layers[idx];
                builder = builder.remedy(Remedy::at_least(
                    SubjectRef::new(
                        &element.id,
                        layer_field_path(&element.id, &layer.id, "thicknessM"),
                        loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                    ),
                    Quantity::length_m(layer.thickness_m),
                    Quantity::length_m(d_req),
                    loc(
                        &format!("Increase insulation '{}' to ≥ {:.0} mm so U+ΔU ≤ {u_max:.3} W/(m²K).", layer.id, d_req * 1000.0),
                        &format!("Dämmstärke '{}' auf ≥ {:.0} mm erhöhen, damit U+ΔU ≤ {u_max:.3} W/(m²K).", layer.id, d_req * 1000.0),
                    ),
                ));
            }
        }
        builder.build()
    }

    /// ✅️ Building bridges allocated onto each opaque element: U′ = U + Σ(ψ·l)/A_opaque_total·share.
    pub fn check_u_prime(element: &EnvelopeElement, bridges: &[ThermalBridge], opaque_area_m2: f64, usage: &str, t_int_c: f64) -> CheckResult {
        let subject = SubjectRef::new(
            &element.id,
            entity_path("elements", &element.id),
            loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
        );
        if element.kind == "window" || element.kind == "door" || element.layers.is_empty() || element.area_m2 <= 0.0 || bridges.is_empty() || opaque_area_m2 <= 0.0 {
            return CheckResult::assess(
                format!("din4108-6.u-prime.{}", element.id),
                "DIN 4108 Bbl.2",
                ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
                subject,
                loc("Element U′ with thermal bridges", "Bauteil-U′ mit Wärmebrücken"),
            )
            .not_applicable(loc("No opaque area or no thermal bridges.", "Keine opake Fläche oder keine Wärmebrücken."))
            .annex(AnnexChoice::De)
            .build();
        }
        let (r_si, r_se) = surface_resistances(&element.kind, &element.adjacent);
        let u = u_value(total_resistance(&element.layers, r_si, r_se)) + element.delta_u_g + element.delta_u_f + element.delta_u_r;
        let share = element.area_m2 / opaque_area_m2;
        let psi_l = psi_l_sum(bridges) * share;
        let u_prime = u_with_bridges(u, psi_l, element.area_m2);
        let mass = part_2::surface_mass_kg_m2(&element.layers);
        let r_min = part_2::table3_r_min(&element.kind, &element.adjacent, usage, t_int_c, mass);
        let u_max = if r_min > 0.0 { 1.0 / r_min } else { f64::INFINITY };
        let mut builder = CheckResult::assess(
            format!("din4108-6.u-prime.{}", element.id),
            "DIN 4108 Bbl.2",
            ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
            subject.clone(),
            loc("Element U′ with thermal bridges", "Bauteil-U′ mit Wärmebrücken"),
        )
        .utilization(Quantity::u_value_w_m2k(u_prime), Quantity::u_value_w_m2k(u_max))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("U′ = {u_prime:.3} W/(m²K) (share={share:.2}) vs U_max = {u_max:.3}."),
            &format!("U′ = {u_prime:.3} W/(m²K) (Anteil={share:.2}) gegenüber U_max = {u_max:.3}."),
        ));
        if u_prime > u_max {
            if let Some(bridge) = bridges.iter().max_by(|a, b| a.psi.partial_cmp(&b.psi).unwrap_or(std::cmp::Ordering::Equal)) {
                let psi_req = (bridge.psi * (u_max / u_prime).min(1.0)).max(0.01);
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &bridge.id,
                        bridge_field_path(&bridge.id, "psi"),
                        loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                    ),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, bridge.psi),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, psi_req),
                    loc(
                        &format!("Reduce ψ of '{}' from {:.3} to ≤ {:.3} W/(m·K).", bridge.id, bridge.psi, psi_req),
                        &format!("ψ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken.", bridge.id, bridge.psi, psi_req),
                    ),
                ));
            }
        }
        builder.build()
    }
}
// #endregion 🔖️Part6

// #region 🔖️Part7
pub mod part_7 {
    use super::*;

    /// 🌬️ DIN 4108-7 n50 limit [1/h]: with / without mechanical ventilation.
    pub fn n50_limit(has_mechanical_ventilation: bool) -> f64 {
        if has_mechanical_ventilation {
            1.5
        } else {
            3.0
        }
    }

    /// ✅️ Airtightness n50 check (DIN 4108-7).
    pub fn check_airtightness(n50: f64, has_mechanical_ventilation: bool) -> CheckResult {
        let limit = n50_limit(has_mechanical_ventilation);
        let subject = SubjectRef::new("", "airtightnessN50", loc("Building airtightness", "Gebäudedichtheit"));
        let mut builder = CheckResult::assess(
            "din4108-7.n50",
            "DIN 4108-7",
            ClauseId::new("DIN 4108-7", "§4", "4.2"),
            subject.clone(),
            loc("Airtightness n50", "Luftdichtheit n50"),
        )
        .utilization(Quantity::new(QuantityKind::VentilationRate, n50), Quantity::new(QuantityKind::VentilationRate, limit))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("n50 = {n50:.2} h⁻¹ vs limit {limit:.1} h⁻¹ (mechanical ventilation = {has_mechanical_ventilation})."),
            &format!("n50 = {n50:.2} h⁻¹ gegenüber Grenzwert {limit:.1} h⁻¹ (mechanische Lüftung = {has_mechanical_ventilation})."),
        ));
        if n50 > limit {
            builder = builder.remedy(Remedy::at_most(
                subject,
                Quantity::new(QuantityKind::VentilationRate, n50),
                Quantity::new(QuantityKind::VentilationRate, limit),
                loc(
                    &format!("Achieve n50 ≤ {limit:.1} h⁻¹ (currently {n50:.2} h⁻¹) by tightening the envelope."),
                    &format!("n50 ≤ {limit:.1} h⁻¹ erreichen (aktuell {n50:.2} h⁻¹) durch dichtere Gebäudehülle."),
                ),
            ));
        }
        builder.build()
    }
}
// #endregion 🔖️Part7

// #region 🔖️BB2
pub mod bb_2 {
    use super::*;

    pub const DELTA_U_WB_CONFORM: f64 = 0.05;
    pub const DELTA_U_WB_FLAT: f64 = 0.10;

    /// 🏷️ Beiblatt 2 equivalence category wire values.
    pub const BB2_CATEGORY_A: &str = "categoryA";
    pub const BB2_CATEGORY_B: &str = "categoryB";
    pub const BB2_DETAILED: &str = "detailed";

    /// 📐️ DIN 4108 Beiblatt 2 §5 / GEG: Kategorie A → ΔU_WB = 0.05 W/(m²K).
    pub const DELTA_U_WB_CATEGORY_A: f64 = 0.05;
    /// 📐️ DIN 4108 Beiblatt 2 §5: Kategorie B / non-conforming catalogue → ΔU_WB = 0.10 W/(m²K).
    pub const DELTA_U_WB_CATEGORY_B: f64 = 0.10;
    /// 📐️ High-performance equivalent details (GEG reduced surcharge) → 0.03 W/(m²K) when all bridges are Kategorie A and details_conform.
    pub const DELTA_U_WB_CATEGORY_A_STRICT: f64 = 0.03;

    /// 🏷️ Parse bridge bb2Type into equivalence category.
    pub fn bb2_category(bb2_type: &str) -> &'static str {
        match bb2_type {
            "categoryA" | "A" | "W01" | "D01" | "conform" => BB2_CATEGORY_A,
            "categoryB" | "B" => BB2_CATEGORY_B,
            _ => BB2_DETAILED,
        }
    }

    /// 📐️ Admissible flat ΔU_WB from bridge categories + building conform flag (DIN 4108 Bbl.2 §5).
    pub fn delta_u_wb_limit(bridges: &[ThermalBridge], details_conform: bool) -> f64 {
        if bridges.is_empty() {
            return if details_conform { DELTA_U_WB_CATEGORY_A } else { DELTA_U_WB_CATEGORY_B };
        }
        let cats: Vec<&str> = bridges.iter().map(|b| bb2_category(&b.bb2_type)).collect();
        if cats.iter().any(|c| *c == BB2_DETAILED) {
            // Detailed ψ·l method: still compare Σψ·l/A against the catalogue flat for the remaining class.
            if cats.iter().all(|c| *c == BB2_DETAILED || *c == BB2_CATEGORY_A) && details_conform {
                DELTA_U_WB_CATEGORY_A
            } else {
                DELTA_U_WB_CATEGORY_B
            }
        } else if cats.iter().all(|c| *c == BB2_CATEGORY_A) {
            if details_conform { DELTA_U_WB_CATEGORY_A_STRICT } else { DELTA_U_WB_CATEGORY_A }
        } else {
            DELTA_U_WB_CATEGORY_B
        }
    }


    /// 📐️ ΔU_WB = Σ(ψ·l)/A [W/(m²K)].
    pub fn delta_u_wb(psi_l: f64, area_m2: f64) -> f64 {
        if area_m2 <= 0.0 {
            f64::INFINITY
        } else {
            psi_l / area_m2
        }
    }

    /// ✅️ Beiblatt 2 thermal-bridge equivalence.
    pub fn check_equivalence(bridges: &[ThermalBridge], area_m2: f64, details_conform: bool) -> CheckResult {
        let subject = SubjectRef::new("", "thermalBridges", loc("Thermal bridges", "Wärmebrücken"));
        if area_m2 <= 0.0 {
            return CheckResult::assess(
                "din4108-bb2.delta-u",
                "DIN 4108 Bbl.2",
                ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
                subject,
                loc("Thermal bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB"),
            )
            .not_applicable(loc("Envelope area is zero.", "Hüllfläche ist null."))
            .annex(AnnexChoice::De)
            .build();
        }
        let psi_l = psi_l_sum(bridges);
        let actual = delta_u_wb(psi_l, area_m2);
        let limit = delta_u_wb_limit(bridges, details_conform);
        let mut builder = CheckResult::assess(
            "din4108-bb2.delta-u",
            "DIN 4108 Bbl.2",
            ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
            subject,
            loc("Thermal bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB"),
        )
        .utilization(Quantity::u_value_w_m2k(actual), Quantity::u_value_w_m2k(limit))
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!(
                "ΔU_WB = {actual:.3} (limit {limit:.3}) from Σψ·l = {psi_l:.3} over A = {area_m2:.1} (conform={details_conform}, cats={:?}).",
                bridges.iter().map(|b| bb2_category(&b.bb2_type)).collect::<Vec<_>>()
            ),
            &format!(
                "ΔU_WB = {actual:.3} (Grenze {limit:.3}) aus Σψ·l = {psi_l:.3} über A = {area_m2:.1} (konform={details_conform}, Kat={:?}).",
                bridges.iter().map(|b| bb2_category(&b.bb2_type)).collect::<Vec<_>>()
            ),
        ));
        if actual > limit {
            let psi_l_max = limit * area_m2;
            if let Some((bi, bridge)) = bridges.iter().enumerate().max_by(|a, b| {
                (a.1.psi * a.1.length_m)
                    .partial_cmp(&(b.1.psi * b.1.length_m))
                    .unwrap_or(std::cmp::Ordering::Equal)
            }) {
                let other: f64 = bridges
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != bi)
                    .map(|(_, b)| b.psi * b.length_m)
                    .sum();
                let psi_req = if bridge.length_m > 0.0 {
                    ((psi_l_max - other) / bridge.length_m).max(0.0)
                } else {
                    0.0
                };
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &bridge.id,
                        bridge_field_path(&bridge.id, "psi"),
                        loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                    ),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, bridge.psi),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, psi_req),
                    loc(
                        &format!("Reduce ψ of '{}' from {:.3} to ≤ {:.3} W/(m·K), or set bb2DetailsConform with redesigned details.", bridge.id, bridge.psi, psi_req),
                        &format!("ψ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken oder bb2DetailsConform mit geänderten Details setzen.", bridge.id, bridge.psi, psi_req),
                    ),
                ));
            }
            if !details_conform {
                builder = builder.remedy(Remedy::exactly(
                    SubjectRef::new("", "bb2DetailsConform", loc("Beiblatt 2 conformity", "Beiblatt-2-Konformität")),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    loc(
                        "Redesign thermal-bridge details to Beiblatt 2 conformity (allows ΔU_WB ≤ 0.05 W/(m²K)).",
                        "Wärmebrückendetails nach Beiblatt 2 ausführen (dann ΔU_WB ≤ 0,05 W/(m²K)).",
                    ),
                ));
            }
        }
        builder.build()
    }

    /// ✅️ Per-bridge Beiblatt 2 equivalence category (DIN 4108 Bbl.2 §5).
    pub fn check_bridge_category(bridge: &ThermalBridge) -> CheckResult {
        let subject = SubjectRef::new(
            &bridge.id,
            bridge_field_path(&bridge.id, "bb2Type"),
            loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
        );
        let cat = bb2_category(&bridge.bb2_type);
        let detailed = cat == BB2_DETAILED;
        let mut builder = CheckResult::assess(
            format!("din4108-bb2.bridge.{}", bridge.id),
            "DIN 4108 Bbl.2",
            ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
            subject.clone(),
            loc("Thermal-bridge equivalence category", "Wärmebrücken-Äquivalenzkategorie"),
        )
        .utilization(
            Quantity::new(QuantityKind::Dimensionless, if detailed { 1.1 } else { 1.0 }),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
        )
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!("Bridge '{}' bb2Type='{}' → category {cat} (ψ={:.3}).", bridge.id, bridge.bb2_type, bridge.psi),
            &format!("Wärmebrücke '{}' bb2Type='{}' → Kategorie {cat} (ψ={:.3}).", bridge.id, bridge.bb2_type, bridge.psi),
        ));
        if detailed {
            builder = builder.remedy(Remedy::one_of(
                subject.clone(),
                vec![BB2_CATEGORY_A.to_string(), BB2_CATEGORY_B.to_string()],
                loc(
                    "Switch bb2Type to an equivalent Beiblatt 2 catalogue category (A or B), or keep detailed ψ with full documentation.",
                    "bb2Type auf eine äquivalente Beiblatt-2-Kategorie (A oder B) umstellen, oder detailliertes ψ mit vollständiger Dokumentation belassen.",
                ),
            ));
            if bridge.psi > 0.15 {
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &bridge.id,
                        bridge_field_path(&bridge.id, "psi"),
                        loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                    ),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, bridge.psi),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, 0.15),
                    loc(
                        &format!("Reduce ψ of '{}' from {:.3} to ≤ 0.15 W/(m·K) for detailed proof.", bridge.id, bridge.psi),
                        &format!("ψ von '{}' von {:.3} auf ≤ 0,15 W/(m·K) für den detaillierten Nachweis senken.", bridge.id, bridge.psi),
                    ),
                ));
            }
        }
        builder.build()
    }
}
// #endregion 🔖️BB2

// #region 🔖️Part10
pub mod part_10 {
    use super::*;

    /// 📋️ DIN 4108-10 Table 1 application-type codes.
    pub const APPLICATION_TYPES: &[&str] = &[
        "DAD", "DAA", "DUK", "DZ", "DI", "DEO", "DES", "WAB", "WAA", "WAP", "WZ", "WH", "WI", "WTH", "WTR", "PW", "PB",
    ];

    /// 🏷️ Required DIN 4108-10 application types from element kind / layer position / adjacent.
    pub fn required_application_types(element: &EnvelopeElement, layer_index: usize) -> &'static [&'static str] {
        let n = element.layers.len().max(1);
        let exteriorish = layer_index >= n / 2;
        match element.kind.as_str() {
            "roof" => {
                if exteriorish { &["DAD", "DAA", "DUK", "DZ"] } else { &["DI", "DZ"] }
            }
            "floor" => {
                if element.adjacent == "ground" { &["DEO", "DES", "PW"] } else { &["DEO", "DES", "DI"] }
            }
            "wall" | "frameOpaque" | "rollerShutterBox" => {
                if element.adjacent == "unheated" || element.adjacent == "otherHeated" {
                    &["WI", "WZ", "WTH"]
                } else if exteriorish {
                    &["WAP", "WAB", "WAA", "WH"]
                } else {
                    &["WI", "WZ", "WTH", "WTR"]
                }
            }
            _ => APPLICATION_TYPES,
        }
    }

    fn class_rank_compressive(code: &str) -> i32 {
        match code { "dx" => 5, "dk" => 4, "dm" => 3, "ds" => 2, "dh" => 1, _ => 0 }
    }

    fn class_rank_water(code: &str) -> i32 {
        match code { "wd" => 3, "wf" => 2, "wk" => 1, _ => 0 }
    }

    fn class_rank_tensile(code: &str) -> i32 {
        match code { "tf" => 2, "tk" => 1, _ => 0 }
    }

    fn class_rank_acoustic(code: &str) -> i32 {
        match code { "sg" => 3, "sm" => 2, "sh" => 1, _ => 0 }
    }

    fn min_compressive_for(app: &str) -> &'static str {
        match app {
            "DEO" | "DES" | "PW" | "PB" => "dk",
            "WAP" | "WAB" | "DAD" | "DAA" => "dm",
            _ => "dh",
        }
    }

    /// 💧 DIN 4108-10 Table 1 minimum water-absorption class for an application type.
    pub fn min_water_for(app: &str) -> &'static str {
        match app {
            "WAP" | "WAB" | "WAA" | "PW" | "PB" | "DUK" => "wf",
            "DEO" | "DES" => "wd",
            _ => "wk",
        }
    }

    /// 🪢 DIN 4108-10 Table 1 minimum tensile class for an application type.
    pub fn min_tensile_for(app: &str) -> &'static str {
        match app {
            "WAP" | "WAB" | "DEO" | "DES" | "PW" | "PB" => "tf",
            _ => "tk",
        }
    }

    /// 🔊 DIN 4108-10 Table 1 minimum acoustic class for an application type.
    pub fn min_acoustic_for(app: &str) -> &'static str {
        match app {
            "DES" | "DEO" => "sm",
            _ => "sh",
        }
    }

    /// ✅️ DIN 4108-10 application-type + property-class suitability (Table 1).
    pub fn check_application(element: &EnvelopeElement, layer_index: usize) -> CheckResult {
        let layer = &element.layers[layer_index];
        let subject = SubjectRef::new(
            &layer.id,
            layer_field_path(&element.id, &layer.id, "applicationType"),
            loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id)),
        );
        let insulation = layer.lambda < 0.1 && layer.thickness_m > 0.0;
        let tagged = !(layer.application_type.is_empty()
            && layer.compressive_class.is_empty()
            && layer.water_class.is_empty()
            && layer.tensile_class.is_empty()
            && layer.acoustic_class.is_empty());
        if !insulation && !tagged {
            return CheckResult::assess(
                format!("din4108-10.app.{}.{}", element.id, layer.id),
                "DIN 4108-10",
                ClauseId::new("DIN 4108-10", "Table 1", "application"),
                subject,
                loc("Insulation application type", "Anwendungsgebiet Dämmstoff"),
            )
            .not_applicable(loc(
                "Layer is not an insulation product under DIN 4108-10.",
                "Schicht ist kein Dämmstoff nach DIN 4108-10.",
            ))
            .annex(AnnexChoice::De)
            .build();
        }
        let required = required_application_types(element, layer_index);
        let ok_type = insulation && required.iter().any(|c| *c == layer.application_type.as_str());
        let ref_app = if ok_type { layer.application_type.as_str() } else { required[0] };
        let min_c = min_compressive_for(ref_app);
        let min_w = min_water_for(ref_app);
        let min_t = min_tensile_for(ref_app);
        let min_a = min_acoustic_for(ref_app);
        let c_have = class_rank_compressive(&layer.compressive_class);
        let w_have = class_rank_water(&layer.water_class);
        let t_have = class_rank_tensile(&layer.tensile_class);
        let a_have = class_rank_acoustic(&layer.acoustic_class);
        let c_need = class_rank_compressive(min_c);
        let w_need = class_rank_water(min_w);
        let t_need = class_rank_tensile(min_t);
        let a_need = class_rank_acoustic(min_a);
        let ok_c = c_have >= c_need;
        let ok_w = w_have >= w_need;
        let ok_t = t_have >= t_need;
        let ok_a = a_have >= a_need;
        let score = if ok_type { c_have + w_have + t_have + a_have } else { 0 };
        let need = c_need + w_need + t_need + a_need;
        let mut builder = CheckResult::assess(
            format!("din4108-10.app.{}.{}", element.id, layer.id),
            "DIN 4108-10",
            ClauseId::new("DIN 4108-10", "Table 1", "application"),
            subject.clone(),
            loc("Insulation application type", "Anwendungsgebiet Dämmstoff"),
        )
        .minimum(
            Quantity::new(QuantityKind::Dimensionless, score as f64),
            Quantity::new(QuantityKind::Dimensionless, need as f64),
        )
        .annex(AnnexChoice::De)
        .explanation(loc(
            &format!(
                "Application '{}' classes C/W/T/A {}/{}/{}/{} vs min {}/{}/{}/{} for {:?}.",
                layer.application_type, layer.compressive_class, layer.water_class, layer.tensile_class, layer.acoustic_class, min_c, min_w, min_t, min_a, required
            ),
            &format!(
                "Anwendung '{}' Klassen D/W/Z/A {}/{}/{}/{} gegenüber Mindest {}/{}/{}/{} für {:?}.",
                layer.application_type, layer.compressive_class, layer.water_class, layer.tensile_class, layer.acoustic_class, min_c, min_w, min_t, min_a, required
            ),
        ));
        if !insulation {
            builder = builder.remedy(Remedy::one_of(
                SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "applicationType"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
                vec![String::new()],
                loc(
                    "Clear DIN 4108-10 application tags on non-insulation layers.",
                    "DIN-4108-10-Anwendungsmarkierungen bei Nicht-Dämmstoffen leeren.",
                ),
            ));
        }
        if !ok_type {
            builder = builder.remedy(Remedy::one_of(
                SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "applicationType"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
                required.iter().map(|c| (*c).to_string()).collect(),
                loc(&format!("Set application type to one of {:?}.", required), &format!("Anwendungsgebiet auf eines von {:?} setzen.", required)),
            ));
        }
        if !ok_c {
            builder = builder.remedy(Remedy::one_of(
                SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "compressiveClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
                vec!["dh".into(), "ds".into(), "dm".into(), "dk".into(), "dx".into()],
                loc(&format!("Raise compressive class to at least '{min_c}'."), &format!("Druckfestigkeitsklasse mindestens '{min_c}' wählen.")),
            ));
        }
        if !ok_w {
            builder = builder.remedy(Remedy::one_of(
                SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "waterClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
                vec!["wk".into(), "wf".into(), "wd".into()],
                loc(&format!("Raise water class to at least '{min_w}'."), &format!("Wasseraufnahmeklasse mindestens '{min_w}' wählen.")),
            ));
        }
        if !ok_t {
            builder = builder.remedy(Remedy::one_of(
                SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "tensileClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
                vec!["tk".into(), "tf".into()],
                loc(&format!("Raise tensile class to at least '{min_t}'."), &format!("Zugfestigkeitsklasse mindestens '{min_t}' wählen.")),
            ));
        }
        if !ok_a {
            builder = builder.remedy(Remedy::one_of(
                SubjectRef::new(&layer.id, layer_field_path(&element.id, &layer.id, "acousticClass"), loc(&format!("Layer {}", layer.id), &format!("Schicht {}", layer.id))),
                vec!["sh".into(), "sm".into(), "sg".into()],
                loc(&format!("Raise acoustic class to at least '{min_a}'."), &format!("Schwingungsgruppe mindestens '{min_a}' wählen.")),
            ));
        }
        builder.build()
    }

    /// 📋 Application types published in the catalogue Table 1 view.
    pub const CATALOGUE_APPLICATION_TYPES: &[&str] = &[
        "WAP", "WAB", "WAA", "DAD", "DAA", "DEO", "DES", "WZ", "WI", "PW", "PB",
    ];

    /// 📋 Catalogue row: (application, min compressive, min water, min tensile, min acoustic) from the same helpers evaluate uses.
    pub fn application_property_row(app: &str) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
        (app, min_compressive_for(app), min_water_for(app), min_tensile_for(app), min_acoustic_for(app))
    }
}
// #endregion 🔖️Part10

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
