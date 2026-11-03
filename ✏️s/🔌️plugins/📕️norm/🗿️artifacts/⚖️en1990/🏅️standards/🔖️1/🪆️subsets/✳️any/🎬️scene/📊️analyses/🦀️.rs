//! 📊️ Analyses: the fatigue spectrum bins, FORM reliability analyses, test series and factor-override rows of the document as bar plates in a lane behind the members. Each bar is an entity bound to its Annex check; the grip height of an editable bar is the value over a fixed scale, so a drag writes the document leaf.

use super::checks::{analysis_form, analysis_form_multi, test_design_value};
use super::handles;
use super::kit::*;
use super::layout::Layout;
use super::model::En1990Snapshot;
use super::scene::*;
use super::SceneInput;

const LANE_Y: f64 = 3.0;
const TEST_LANE_Y: f64 = 6.0;
const OVERRIDE_LANE_Y: f64 = 9.0;
const FACTOR_PER_METRE: f64 = 1.0;
const BIN_PITCH: f64 = 0.5;
const BIN_WIDTH: f64 = 0.35;
const PAIR_PITCH: f64 = 0.8;
const PAIR_WIDTH: f64 = 0.6;
const PAIR_GAP: f64 = 1.5;
const BAR_THICKNESS: f64 = 0.06;
const MIN_HEIGHT: f64 = 0.02;
const PASCAL_PER_METRE: f64 = 5.0e7;
pub(crate) const CYCLES_PER_METRE: f64 = 1.0e7;

/// 📊️ One bar of the lane: the entity it belongs to, the document leaf its grip writes (if any) and its place.
pub(crate) struct Bar {
    pub owner: String,
    pub suffix: Option<&'static str>,
    pub collection: &'static str,
    pub field: Option<&'static str>,
    pub value: f64,
    pub per_metre: f64,
    pub step: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub ghost: bool,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub subjects: Vec<String>,
}

impl Bar {
    pub(crate) fn height(&self) -> f64 {
        (self.value / self.per_metre).max(MIN_HEIGHT)
    }

    pub(crate) fn entity_id(&self) -> String {
        self.suffix.map_or_else(|| self.owner.clone(), |suffix| format!("{}:{suffix}", self.owner))
    }

    pub(crate) fn handle_tag(&self) -> Option<&'static str> {
        match (self.collection, self.suffix) {
            ("testSeries", None) => Some("mean"),
            _ => self.field,
        }
    }

    /// 📏️ The standard-deviation leaf a reliability bar carries as a second grip: the whisker above the mean.
    pub(crate) fn spread_tag(&self) -> Option<&'static str> {
        match (self.collection, self.suffix) {
            ("reliabilityAnalyses", None) => Some("resistanceSd"),
            ("reliabilityAnalyses", Some("action")) => Some("actionSd"),
            ("reliabilityAnalyses", Some("action2")) => Some("secondActionSd"),
            ("testSeries", None) => Some("sd"),
            _ => None,
        }
    }
}

impl Bar {
    /// 🎚️ The gauge tags of the leaves the main bar of a test series carries beyond its mean and spread.
    pub(crate) fn gauge_tags(&self) -> Vec<String> {
        match (self.collection, self.suffix) {
            ("testSeries", None) => handles::gauges::TEST_SERIES.iter().map(|g| format!("gauge-{}", g.tag)).collect(),
            _ => Vec::new(),
        }
    }

    /// 📏️ The further leaves a fatigue bin carries as grips: the detail category Δσ_C along the bar and the number of cycles n into the lane.
    pub(crate) fn extra_tags(&self) -> &'static [&'static str] {
        match self.collection {
            "fatigueSpectra" => &["detailCategory", "cycles"],
            _ => &[],
        }
    }
}

pub(crate) fn decade_scale(largest: f64) -> f64 {
    if largest.is_finite() && largest > 0.0 { 10f64.powf(largest.log10().ceil()) / 2.0 } else { 1.0 }
}

/// 📍️ Every bar in document order: spectrum bins behind their member, then the analyses and test series after the last member.
pub(crate) fn bars(d: &En1990Snapshot, layout: &Layout) -> Vec<Bar> {
    let mut out = Vec::new();
    for member in &d.members {
        let Some(pose) = layout.member(&member.id) else { continue };
        let first = d.fatigue_spectra.iter().find(|b| b.member_id == member.id).map(|b| b.id.clone());
        for (slot, bin) in d.fatigue_spectra.iter().filter(|b| b.member_id == member.id).enumerate() {
            out.push(Bar {
                owner: bin.id.clone(),
                suffix: None,
                collection: "fatigueSpectra",
                field: Some("stressRange"),
                value: bin.stress_range,
                per_metre: PASCAL_PER_METRE,
                step: 1.0e5,
                x: pose.frame.origin.x + slot as f64 * BIN_PITCH,
                y: LANE_Y,
                width: BIN_WIDTH,
                ghost: false,
                name: loc(&format!("Spectrum bin {} of {}", bin.id, member.label_en), &format!("Kollektiv-Klasse {} von {}", bin.id, member.label_de)),
                detail: loc(&format!("Δσ = {:.1} MPa, n = {:.3e}, Δσ_C = {:.0} MPa", bin.stress_range * 1e-6, bin.cycles, bin.detail_category * 1e-6), &format!("Δσ = {:.1} MPa, n = {:.3e}, Δσ_C = {:.0} MPa", bin.stress_range * 1e-6, bin.cycles, bin.detail_category * 1e-6)),
                subjects: first.iter().cloned().collect(),
            });
        }
    }
    let mut cursor = layout.members().map(|(_, pose)| pose.frame.origin.x + pose.length).fold(0.0, f64::max) + PAIR_GAP;
    for analysis in &d.reliability_analyses {
        let scale = decade_scale(analysis.resistance_mean.abs().max(analysis.action_mean.abs()));
        let second = analysis.second_action_sd > 0.0;
        let resistance = (analysis.resistance_distribution.as_str(), analysis.resistance_mean, analysis.resistance_sd);
        let action = (analysis.action_distribution.as_str(), analysis.action_mean, analysis.action_sd);
        let beta = if second {
            analysis_form_multi(resistance, &[action, (analysis.second_action_distribution.as_str(), analysis.second_action_mean, analysis.second_action_sd)]).map_or(f64::NAN, |form| form.beta)
        } else {
            analysis_form(resistance, action).map_or(f64::NAN, |form| form.beta)
        };
        let detail = loc(&format!("β = {beta:.2}, μ_R = {:.1} kN, μ_E = {:.1} kN", analysis.resistance_mean / 1000.0, analysis.action_mean / 1000.0), &format!("β = {beta:.2}, μ_R = {:.1} kN, μ_E = {:.1} kN", analysis.resistance_mean / 1000.0, analysis.action_mean / 1000.0));
        out.push(Bar { owner: analysis.id.clone(), suffix: None, collection: "reliabilityAnalyses", field: Some("resistanceMean"), value: analysis.resistance_mean, per_metre: scale, step: scale / 1000.0, x: cursor, y: LANE_Y, width: PAIR_WIDTH, ghost: false, name: loc(&format!("Resistance of {}", analysis.id), &format!("Widerstand von {}", analysis.id)), detail: detail.clone(), subjects: vec![analysis.id.clone()] });
        out.push(Bar { owner: analysis.id.clone(), suffix: Some("action"), collection: "reliabilityAnalyses", field: Some("actionMean"), value: analysis.action_mean, per_metre: scale, step: scale / 1000.0, x: cursor + PAIR_PITCH, y: LANE_Y, width: PAIR_WIDTH, ghost: true, name: loc(&format!("Action of {}", analysis.id), &format!("Einwirkung von {}", analysis.id)), detail, subjects: Vec::new() });
        if second {
            out.push(Bar { owner: analysis.id.clone(), suffix: Some("action2"), collection: "reliabilityAnalyses", field: Some("secondActionMean"), value: analysis.second_action_mean, per_metre: scale, step: scale / 1000.0, x: cursor + 2.0 * PAIR_PITCH, y: LANE_Y, width: PAIR_WIDTH, ghost: true, name: loc(&format!("Second action of {}", analysis.id), &format!("Zweite Einwirkung von {}", analysis.id)), detail: loc(&format!("β = {beta:.2}, μ_E2 = {:.1} kN, σ_E2 = {:.1} kN", analysis.second_action_mean / 1000.0, analysis.second_action_sd / 1000.0), &format!("β = {beta:.2}, μ_E2 = {:.1} kN, σ_E2 = {:.1} kN", analysis.second_action_mean / 1000.0, analysis.second_action_sd / 1000.0)), subjects: Vec::new() });
            cursor += PAIR_PITCH;
        }
        cursor += 2.0 * PAIR_PITCH + PAIR_GAP;
    }
    let mut test_cursor = layout.members().map(|(_, pose)| pose.frame.origin.x + pose.length).fold(0.0, f64::max) + PAIR_GAP;
    for series in &d.test_series {
        let design = test_design_value(series.distribution.as_str(), series.sample_size, series.mean, series.std_dev, series.variation_known, series.conversion_factor).unwrap_or(0.0);
        let scale = decade_scale(series.mean.abs().max(series.required.abs()).max(design.abs()));
        let detail = loc(&format!("n = {}, X_d = {design:.3}, required {:.3}", series.sample_size, series.required), &format!("n = {}, X_d = {design:.3}, gefordert {:.3}", series.sample_size, series.required));
        out.push(Bar { owner: series.id.clone(), suffix: None, collection: "testSeries", field: None, value: design.max(0.0), per_metre: scale, step: scale / 1000.0, x: test_cursor, y: TEST_LANE_Y, width: PAIR_WIDTH, ghost: false, name: loc(&format!("Design value of {}", series.id), &format!("Bemessungswert von {}", series.id)), detail: detail.clone(), subjects: vec![series.id.clone()] });
        out.push(Bar { owner: series.id.clone(), suffix: Some("required"), collection: "testSeries", field: Some("required"), value: series.required, per_metre: scale, step: scale / 1000.0, x: test_cursor + PAIR_PITCH, y: TEST_LANE_Y, width: PAIR_WIDTH, ghost: true, name: loc(&format!("Required value of {}", series.id), &format!("Geforderter Wert von {}", series.id)), detail, subjects: Vec::new() });
        test_cursor += 2.0 * PAIR_PITCH + PAIR_GAP;
    }
    let verdicts = super::inferences::factor_overrides::verdicts(d);
    let deviating = super::inferences::factor_overrides::deviations(d);
    let mut override_cursor = layout.members().map(|(_, pose)| pose.frame.origin.x + pose.length).fold(0.0, f64::max) + PAIR_GAP;
    for (index, row) in d.factor_overrides.iter().enumerate() {
        let annex = super::inferences::factor_overrides::annex_value(d, row);
        let state = if verdicts[index] != super::inferences::factor_overrides::Verdict::Applied { ("not applied", "nicht angewendet") } else if deviating[index] { ("deviates from NA", "weicht vom NA ab") } else { ("equals NA", "entspricht dem NA") };
        let target = if row.target.trim().is_empty() { ("all actions", "alle Einwirkungen") } else { (row.target.trim(), row.target.trim()) };
        let detail = loc(&format!("{} on {}: {:.3}, NA {} ({})", row.factor, target.0, row.value, annex.map_or_else(|| "-".to_string(), |a| format!("{a:.3}")), state.0), &format!("{} auf {}: {:.3}, NA {} ({})", row.factor, target.1, row.value, annex.map_or_else(|| "-".to_string(), |a| format!("{a:.3}")), state.1));
        out.push(Bar { owner: row.id.clone(), suffix: None, collection: "factorOverrides", field: Some("value"), value: row.value.max(0.0), per_metre: FACTOR_PER_METRE, step: 0.01, x: override_cursor, y: OVERRIDE_LANE_Y, width: PAIR_WIDTH, ghost: !deviating[index], name: loc(&format!("Factor override {}", row.id), &format!("Faktorüberschreibung {}", row.id)), detail: detail.clone(), subjects: vec![row.id.clone()] });
        out.push(Bar { owner: row.id.clone(), suffix: Some("annex"), collection: "factorOverrides", field: None, value: annex.unwrap_or(0.0).max(0.0), per_metre: FACTOR_PER_METRE, step: 0.01, x: override_cursor + PAIR_PITCH, y: OVERRIDE_LANE_Y, width: PAIR_WIDTH, ghost: true, name: loc(&format!("National Annex value of {}", row.id), &format!("Wert des Nationalen Anhangs von {}", row.id)), detail, subjects: Vec::new() });
        override_cursor += 2.0 * PAIR_PITCH + PAIR_GAP;
    }
    out
}

fn bar_geometry(bar: &Bar) -> (Geometry, [Vec3; 2]) {
    let height = bar.height();
    let centre = v3(bar.x, bar.y, height / 2.0);
    let outline = Polygon2::solid(vec![Vec2::new(-bar.width / 2.0, -height / 2.0), Vec2::new(bar.width / 2.0, -height / 2.0), Vec2::new(bar.width / 2.0, height / 2.0), Vec2::new(-bar.width / 2.0, height / 2.0)]);
    let frame = Frame { origin: centre, x: Vec3::X, y: Vec3::Z, z: v3(0.0, -1.0, 0.0) };
    (Geometry::Plate(PlateGeometry { frame, outline, thickness: BAR_THICKNESS, offset: 0.0, stiffeners: Vec::new() }), [v3(bar.x - bar.width / 2.0, bar.y, 0.0), v3(bar.x + bar.width / 2.0, bar.y, height)])
}

/// 📊️ The bars of the document as scene entities.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for bar in bars(input.document, layout) {
        let (geometry, points) = bar_geometry(&bar);
        let subjects: Vec<&str> = bar.subjects.iter().map(String::as_str).collect();
        let handle_ids = bar.handle_tag().into_iter().chain(bar.spread_tag()).chain(bar.extra_tags().iter().copied()).map(str::to_string).chain(bar.gauge_tags()).map(|tag| format!("{}:{tag}", bar.owner)).collect();
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: bar.entity_id(),
                collection: bar.collection,
                geometry,
                role: MaterialRole::Steel,
                binding: if subjects.is_empty() { None } else { binding(input.document, input.report, &subjects) },
                name: bar.name.clone(),
                detail: bar.detail.clone(),
                layer: LAYER_ANALYSES,
                handles: handle_ids,
                deformation: None,
                ghost: bar.ghost,
            },
            &points,
        );
    }
}
