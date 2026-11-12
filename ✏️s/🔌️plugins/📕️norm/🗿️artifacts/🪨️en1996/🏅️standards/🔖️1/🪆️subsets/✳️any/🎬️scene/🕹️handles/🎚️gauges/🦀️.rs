//! 🎚️ Gauges: one scalar grip for every numeric field of a row that no geometric grip writes, listed per collection in one table. A gauge drags along `+Z` from its anchor; the field is `value + 20 * step * (s - anchor.z)` with the `step` of the field in the snapshot schema (`x-semio-ui.step`), quantised to that step and held inside its physical range. Catalogue fields (the fire class) snap to their classes. The tables are the contract of the grip coverage test, which reads the snapshot schema.
//! 🔗️ Handle laws: [`assert_handle_laws`](crate::scene::projection_kit::test_support::assert_handle_laws).

use super::super::scene::projection_kit::{loc};
use super::super::kit::{BAR_TAG, COLUMN_TAG, TENDON_TAG, TIE_TAG};
use super::super::layout::WallPose;
use super::super::model::{ConcentratedLoad, ConfinedElement, MasonryWall, WallBar, WallLoadCase, WallPrestress, WallTie};
use super::super::scene::projection_kit::handle_kit::{field_binding, handle, HandleSpec};
use super::super::scene::*;

pub const STEPS_PER_METRE: f64 = 20.0;
pub const CLASSES_PER_METRE: f64 = 60.0;
const PITCH: f64 = 0.3;
const ROW: f64 = 0.4;
const STRESS: f64 = 1.0e9;
const FORCE: f64 = 1.0e10;
const FIRE_CLASSES: &[f64] = &[30.0, 60.0, 90.0, 120.0, 180.0, 240.0];

pub struct Gauge<T> {
    pub tag: &'static str,
    pub field: &'static str,
    pub get: fn(&T) -> f64,
    pub unit: &'static str,
    pub step: f64,
    pub lo: f64,
    pub hi: f64,
    pub classes: &'static [f64],
    pub en: &'static str,
    pub de: &'static str,
}

macro_rules! gauge {
    ($tag:literal, $field:literal, $get:expr, $unit:literal, $step:expr, $lo:expr, $hi:expr, $en:literal, $de:literal) => {
        Gauge { tag: $tag, field: $field, get: $get, unit: $unit, step: $step, lo: $lo, hi: $hi, classes: &[], en: $en, de: $de }
    };
}

pub const WALLS: &[Gauge<MasonryWall>] = &[
    gauge!("f-b", "fBPa", |w| w.f_b_pa, "Pa", 500_000.0, 0.0, STRESS, "Normalised unit strength", "Normierte Steindruckfestigkeit"),
    gauge!("unit-length", "unitLengthM", |w| w.unit_length_m, "m", 0.005, 0.0, 2.0, "Unit length", "Steinlänge"),
    gauge!("unit-width", "unitWidthM", |w| w.unit_width_m, "m", 0.005, 0.0, 2.0, "Unit width", "Steinbreite"),
    gauge!("unit-height", "unitHeightM", |w| w.unit_height_m, "m", 0.005, 0.0, 2.0, "Unit height", "Steinhöhe"),
    gauge!("mortar-strength", "mortarStrengthPa", |w| w.mortar_strength_pa, "Pa", 500_000.0, 0.0, STRESS, "Mortar compressive strength", "Mörteldruckfestigkeit"),
    gauge!("bed-joint", "bedJointThicknessM", |w| w.bed_joint_thickness_m, "m", 0.001, 0.0, 0.1, "Bed joint thickness", "Lagerfugendicke"),
    gauge!("as-vertical", "asVerticalM2", |w| w.as_vertical_m2, "m2", 1.0e-6, 0.0, 0.1, "Vertical reinforcement area", "Querschnitt der vertikalen Bewehrung"),
    gauge!("as-horizontal", "asHorizontalM2", |w| w.as_horizontal_m2, "m2", 1.0e-6, 0.0, 0.1, "Bed joint reinforcement area", "Querschnitt der Lagerfugenbewehrung"),
    gauge!("f-yd", "fYdPa", |w| w.f_yd_pa, "Pa", 5_000_000.0, 0.0, STRESS * 2.0, "Design yield strength of reinforcement", "Bemessungswert der Streckgrenze der Bewehrung"),
    Gauge { tag: "fire-rei", field: "fireReiMin", get: |w| f64::from(w.fire_rei_min), unit: "min", step: 1.0, lo: 0.0, hi: 240.0, classes: FIRE_CLASSES, en: "Required fire resistance", de: "Erforderliche Feuerwiderstandsdauer" },
    gauge!("mu", "mu", |w| w.mu, "-", 0.01, 0.0, 2.0, "Friction coefficient", "Reibungsbeiwert"),
    gauge!("density", "densityKgM3", |w| w.density_kg_m3, "kg/m3", 10.0, 0.0, 5000.0, "Masonry density", "Rohdichte des Mauerwerks"),
    gauge!("phi-infinity", "phiInfinity", |w| w.phi_infinity, "-", 0.01, 0.0, 10.0, "Final creep coefficient", "Endkriechzahl"),
    gauge!("cover", "coverM", |w| w.cover_m, "m", 0.005, 0.0, 0.5, "Cover to reinforcement", "Überdeckung der Bewehrung"),
    gauge!("deflection-limit", "deflectionLimit", |w| w.deflection_limit, "-", 10.0, 0.0, 10_000.0, "Deflection limit span divisor", "Verformungsgrenze Stützweite durch x"),
    gauge!("soil-height", "soilHeightM", |w| w.soil_height_m, "m", 0.05, 0.0, 20.0, "Retained soil height", "Höhe der Anschüttung"),
    gauge!("soil-density", "soilDensityKgM3", |w| w.soil_density_kg_m3, "kg/m3", 50.0, 0.0, 5000.0, "Soil density", "Wichte des Bodens"),
    gauge!("plan-x", "planXM", |w| w.plan_x_m, "m", 0.1, -1000.0, 1000.0, "Plan position x", "Grundrissposition x"),
    gauge!("plan-y", "planYM", |w| w.plan_y_m, "m", 0.1, -1000.0, 1000.0, "Plan position y", "Grundrissposition y"),
    gauge!("plan-angle", "planAngleDeg", |w| w.plan_angle_deg, "deg", 5.0, -180.0, 180.0, "Plan angle", "Grundrisswinkel"),
    gauge!("in-plane-shear", "inPlaneShearN", |w| w.in_plane_shear_n, "N", 1000.0, 0.0, FORCE, "In-plane shear", "Scheibenschub"),
    gauge!("in-plane-moment", "inPlaneMomentNm", |w| w.in_plane_moment_nm, "Nm", 1000.0, 0.0, FORCE, "In-plane moment", "Scheibenmoment"),
    gauge!("web-spacing", "webSpacingM", |w| w.web_spacing_m, "m", 0.05, 0.0, 100.0, "Web spacing of the shear wall", "Stegabstand der Schubwand"),
    gauge!("second-leaf", "secondLeafThicknessM", |w| w.second_leaf_thickness_m, "m", 0.005, 0.0, 1.0, "Thickness of the second leaf", "Dicke der zweiten Schale"),
    gauge!("cavity-width", "cavityWidthM", |w| w.cavity_width_m, "m", 0.005, 0.0, 1.0, "Cavity width between the leaves", "Schalenabstand der zweischaligen Wand"),
    gauge!("effective-depth", "effectiveDepthM", |w| w.effective_depth_m, "m", 0.005, 0.0, 1.0, "Effective depth of the vertical reinforcement", "Nutzhöhe der vertikalen Bewehrung"),
];

pub const LOAD_CASES: &[Gauge<WallLoadCase>] = &[
    gauge!("g-k-slab", "gKSlabN", |c| c.g_k_slab_n, "N", 1000.0, 0.0, FORCE, "Permanent slab load", "Ständige Deckenauflast"),
    gauge!("q-k-imposed", "qKImposedPa", |c| c.q_k_imposed_pa, "Pa", 50.0, 0.0, 1.0e6, "Imposed load", "Nutzlast"),
    gauge!("tributary-area", "tributaryAreaM2", |c| c.tributary_area_m2, "m2", 0.5, 0.0, 10_000.0, "Tributary floor area", "Lasteinzugsfläche"),
    gauge!("q-k-snow", "qKSnowPa", |c| c.q_k_snow_pa, "Pa", 50.0, 0.0, 1.0e6, "Snow load", "Schneelast"),
    gauge!("q-p-wind", "qPWindPa", |c| c.q_p_wind_pa, "Pa", 50.0, 0.0, 1.0e6, "Peak velocity pressure", "Böengeschwindigkeitsdruck"),
    gauge!("c-pe", "cPe", |c| c.c_pe, "-", 0.05, -10.0, 10.0, "External pressure coefficient", "Äußerer Druckbeiwert"),
    gauge!("h-k-earth", "hKEarthN", |c| c.h_k_earth_n, "N", 1000.0, 0.0, FORCE, "Earth pressure resultant", "Erddruckresultierende"),
];

pub const CONCENTRATED: &[Gauge<ConcentratedLoad>] = &[
    gauge!("force", "forceN", |c| c.force_n, "N", 1000.0, 0.0, FORCE, "Concentrated force", "Einzellast"),
    gauge!("bearing-area", "bearingAreaM2", |c| c.bearing_area_m2, "m2", 0.0001, 0.0, 10.0, "Loaded area", "Belastete Fläche"),
    gauge!("distance-a1", "distanceA1M", |c| c.distance_a1_m, "m", 0.05, 0.0, 100.0, "Distance to the wall end", "Abstand vom Wandende"),
];

pub const TIES: &[Gauge<WallTie>] = &[gauge!("capacity", "capacityN", |t| t.capacity_n, "N", 100.0, 0.0, FORCE, "Tie design resistance", "Bemessungswiderstand des Ankers")];

pub const PRESTRESS: &[Gauge<WallPrestress>] = &[
    gauge!("force", "forceN", |p| p.force_n, "N", 1000.0, 0.0, FORCE, "Initial prestressing force", "Anfängliche Vorspannkraft"),
    gauge!("area", "areaM2", |p| p.area_m2, "m2", 1.0e-5, 0.0, 1.0, "Tendon area", "Spannstahlfläche"),
    gauge!("f-pk", "fPkPa", |p| p.f_pk_pa, "Pa", 50_000_000.0, 0.0, STRESS * 4.0, "Tendon strength", "Spannstahlfestigkeit"),
    gauge!("loss", "lossRatio", |p| p.loss_ratio, "-", 0.01, 0.0, 1.0, "Prestress loss ratio", "Spannkraftverlust"),
];

pub const CONFINED: &[Gauge<ConfinedElement>] = &[gauge!("as", "asM2", |c| c.as_m2, "m2", 1.0e-5, 0.0, 1.0, "Column reinforcement area", "Stützenbewehrung")];

pub const BARS: &[Gauge<WallBar>] = &[gauge!("diameter", "diameterM", |b| b.diameter_m, "m", 0.002, 0.0, 0.1, "Bar diameter", "Stabdurchmesser")];

/// 🚫 The numeric schema fields no viewport grip writes, with the reason: `z` of a placement has no drag (a wall stands on the ground plane, the move grips write `x`, `y` and `yaw`); `value` of a case number is written by the grips of its catalogue form, which the case laws check per catalogue entry.
pub const EXEMPT: &[(&str, &str)] = &[("Placement", "z"), ("CaseNumber", "value")];

/// 🎚️ The gauges whose step differs from the schema step: `cPe` is a coefficient near one, but its schema stepper is the copy of a list position stepper (step 1), so a gauge of that step could not set it; the schema tool has to correct the step.
pub const STEP_OVERRIDES: &[(&str, &str)] = &[("WallLoadCase", "cPe")];

/// 🎚️ The schema definition, the field and the schema step of every gauge: the contract of the grip coverage test.
pub fn coverage() -> Vec<(&'static str, &'static str, f64, bool)> {
    fn rows<T>(def: &'static str, table: &[Gauge<T>]) -> Vec<(&'static str, &'static str, f64, bool)> {
        table.iter().map(|g| (def, g.field, g.step, !g.classes.is_empty())).collect()
    }
    [rows("MasonryWall", WALLS), rows("WallLoadCase", LOAD_CASES), rows("ConcentratedLoad", CONCENTRATED), rows("WallTie", TIES), rows("WallPrestress", PRESTRESS), rows("ConfinedElement", CONFINED), rows("WallBar", BARS)].concat()
}

/// 🎚️ The gauges of one row as handles: a rack of diamonds along the wall direction at height `z`, centred at `x_mid`, `head` the document path of the row and `scope` the tag part that keeps the ids of several rows apart.
#[allow(clippy::too_many_arguments)]
pub fn rack<T>(table: &[Gauge<T>], row: &T, head: &str, scope: &str, entity: &EntityRef, label: &str, at: impl Fn(f64) -> Vec3, out: &mut Vec<Handle>) {
    let n = table.len() as f64;
    for (i, g) in table.iter().enumerate() {
        let value = (g.get)(row);
        let anchor = at((i as f64 - (n - 1.0) / 2.0) * PITCH);
        let k = if g.classes.is_empty() { STEPS_PER_METRE * g.step } else { CLASSES_PER_METRE };
        let (lo, hi) = (g.lo.min(value), g.hi.max(value));
        let s = |v: f64| anchor.z + (v - value) / k;
        let quantise = if g.classes.is_empty() { Quantise::Round { step: g.step } } else { Quantise::Choice { values: g.classes.to_vec() } };
        out.push(handle(HandleSpec {
            entity,
            tag: &format!("{scope}gauge-{}", g.tag),
            kind: HandleKind::ScalarAlong { axis: Vec3::Z },
            anchor: anchor.clone(),
            grip: GripStyle::Diamond,
            writes: vec![field_binding(format!("{head}.{}", g.field), Affine { base: value - k * anchor.z, k }, g.unit, Some(quantise))],
            snap: SnapRule::NONE,
            limits: (s(lo), s(hi)),
            name: loc(&format!("{} ({label})", g.en), &format!("{} ({label})", g.de)),
        }));
    }
}

/// 🎚️ The rack of a wall and one rack per load case above it, standing above the wall head.
pub fn of_wall(wall: &MasonryWall, pose: &WallPose) -> Vec<Handle> {
    let entity = EntityRef::new(EntityKind::Group, wall.id.clone(), format!("walls[id={}]", wall.id));
    let top = ((wall.height_m + 0.4) / 0.05).ceil() * 0.05;
    let head = format!("walls[id={}]", wall.id);
    let mut out = Vec::new();
    rack(WALLS, wall, &head, "", &entity, &wall.id, |dx| pose.at(0.5 * wall.length_m + dx, 0.5 * wall.thickness_m, top), &mut out);
    for (j, lc) in wall.load_cases.iter().enumerate() {
        let z = top + ROW * (j as f64 + 1.0);
        rack(LOAD_CASES, lc, &format!("{head}.loadCases[id={}]", lc.id), &format!("lc-{}-", lc.id), &entity, &format!("{} {}", wall.id, lc.id), |dx| pose.at(0.5 * wall.length_m + dx, 0.5 * wall.thickness_m, z), &mut out);
    }
    out
}

/// 🎚️ The rack of one concentrated load, above the wall head beside its bearing.
pub fn of_concentrated(wall: &MasonryWall, pose: &WallPose, lc: &WallLoadCase, c: &ConcentratedLoad, entity: &EntityRef) -> Vec<Handle> {
    let mut out = Vec::new();
    rack(CONCENTRATED, c, &format!("walls[id={}].loadCases[id={}].concentrated[id={}]", wall.id, lc.id, c.id), "", entity, &c.id, |dx| pose.at(c.position_m + dx, 0.5 * wall.thickness_m, wall.height_m + 0.8), &mut out);
    out
}

/// 🎚️ The rack of the member scene entity `<wall>:<tag>` (tie, tendon, confining column or bar); empty for every other tag.
pub fn of_member(wall: &MasonryWall, pose: &WallPose, tag: &str) -> Vec<Handle> {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let entity = |kind: EntityKind, collection: &str, row: &str| EntityRef::new(kind, format!("{}:{tag}", wall.id), format!("walls[id={}].{collection}[id={row}]", wall.id));
    let head = |collection: &str, row: &str| format!("walls[id={}].{collection}[id={row}]", wall.id);
    let mut out = Vec::new();
    if let Some(row) = tag.strip_prefix(TIE_TAG) {
        if let Some(x) = wall.ties.iter().find(|x| x.id == row) {
            rack(TIES, x, &head("ties", row), "", &entity(EntityKind::Fastener, "ties", row), row, |dx| pose.at(x.position_m + dx, 0.5 * t, (x.level_m + 0.3).min(h + 0.3)), &mut out);
        }
    } else if let Some(row) = tag.strip_prefix(TENDON_TAG) {
        if let Some(x) = wall.prestress.iter().find(|x| x.id == row) {
            rack(PRESTRESS, x, &head("prestress", row), "", &entity(EntityKind::Fastener, "prestress", row), row, |dx| pose.at(x.position_m + dx, 0.5 * t, h + 0.3), &mut out);
        }
    } else if let Some(row) = tag.strip_prefix(COLUMN_TAG) {
        if let Some(x) = wall.confined.iter().find(|x| x.id == row) {
            rack(CONFINED, x, &head("confined", row), "", &entity(EntityKind::Solid, "confined", row), row, |dx| pose.at(x.position_m + dx, 0.5 * t, h + 0.3), &mut out);
        }
    } else if let Some(row) = tag.strip_prefix(BAR_TAG) {
        if let Some(x) = wall.bars.iter().find(|x| x.id == row) {
            rack(BARS, x, &head("bars", row), "", &entity(EntityKind::Fastener, "bars", row), row, |dx| pose.at(0.5 * l + dx, 0.5 * t, h + 0.3), &mut out);
        }
    }
    out
}
