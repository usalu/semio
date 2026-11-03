//! 🚁 Hazard and execution collections in the scene: helicopter landings with their pad, horizontal ties as tension lines, construction loads stacked on the stage strip, falling loads, the equivalent force of
//! execution imperfections and derailment cases next to a track. Every entity is bound to the check of its row and carries the drag handles of `handles`.

use super::collections::{stage_row_y, stage_slots};
use super::layout::stack_level;
use super::kit::*;
use super::model::En1991Snapshot;
use super::norm;
use super::scene::*;
use super::{handles, Ctx};

const PAD_SIDE: f64 = 6.0;
const PAD_LIFT: f64 = 0.05;
const TIE_LEVEL: f64 = 3.0;
const STRIP_DEPTH: f64 = 2.0;
const TRACK_LEVEL: f64 = 1.8;
const BLOCK_DENSITY: f64 = 2400.0;
const SWAY_SHARE: f64 = 0.04;
const MAX_EXAGGERATION: f64 = 100.0;
const BOW_SEGMENTS: usize = 12;
const RAIL_GAUGE: f64 = 1.435;
const RAIL_WIDTH: f64 = 0.07;
const TRACK_WIDTH: f64 = 1.5;
const TRACK_HALF_LENGTH: f64 = 10.0;

/// 🧊 Edge of the cube the falling mass would fill as concrete, so the glyph grows with the mass entered.
pub fn block_side(mass: f64) -> f64 {
    (mass.max(0.0) / BLOCK_DENSITY).cbrt().clamp(0.2, 2.0)
}

/// 🔍 Factor by which the tolerance is drawn larger so that the sway reaches a visible share of the member height.
pub fn exaggeration(height: f64, tolerance: f64) -> f64 {
    if tolerance > 1.0e-9 { (SWAY_SHARE * height / tolerance).clamp(1.0, MAX_EXAGGERATION) } else { 1.0 }
}

/// 🎀 A vertical ribbon of the given width through the `(x offset, z)` profile points, as one polygon.
fn ribbon(pose: &super::layout::Pose, x: f64, y: f64, width: f64, profile: &[(f64, f64)]) -> Vec<Vec3> {
    profile.iter().map(|(dx, z)| pose.at(x + dx, y, *z)).chain(profile.iter().rev().map(|(dx, z)| pose.at(x + dx + width, y, *z))).collect()
}

#[allow(clippy::too_many_arguments)]
fn flat(out: &mut Out, id: String, collection: &str, points: &[Vec3], name: LocalizedCopy, layer: &'static str, binding: Option<UtilisationBinding>, opacity: f64) {
    out.push(Spec::new(EntityKind::Surface, id, collection, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(points) } }), MaterialRole::Neutral, name, layer).bound(binding).opacity(opacity), points);
}

/// 🔖 Point load with its binding, detail text and grips.
#[allow(clippy::too_many_arguments)]
fn point(out: &mut Out, report: &norm::CheckReport, id: &str, collection: &str, at: Vec3, direction: Vec3, magnitude: f64, name: LocalizedCopy, detail: LocalizedCopy, check: &str, layer: &'static str, grips: Vec<String>) {
    out.push(
        Spec::new(EntityKind::Load, id.to_string(), collection, Geometry::Load(LoadGlyph::Point { at: at.clone(), direction, magnitude: force(magnitude), case: case(id, LoadKind::Accidental) }), load(LoadKind::Accidental), name, layer).bound(binding_where(report, |c| c.id == check)).channel(ColourChannel::Role).opacity(0.9).detail(detail).handles(grips),
        &[at],
    );
}

/// 🚁 Helicopter landings: the pad square on the roof level and the landing force pressing down at its centre.
pub fn helicopters(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for row in &d.helicopter_landings {
        if !out.own(&row.id) { continue; }
        let (check, grips) = (format!("en1991.1-7.helicopter-force.{}", row.id), handles::ids(d, layout, &row.id));
        let half = PAD_SIDE / 2.0;
        let pad: Vec<Vec3> = [(-half, -half), (half, -half), (half, half), (-half, half)].iter().map(|(dx, dy)| pose.at(row.x + dx, row.y + dy, row.height + PAD_LIFT)).collect();
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:pad", row.id), "helicopterLandings", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&pad) } }), MaterialRole::Neutral, loc("Helicopter pad", "Hubschrauberlandeplatz"), LAYER_HELICOPTER).bound(binding_where(report, |c| c.id == check)).opacity(0.7),
            &pad,
        );
        point(out, report, &row.id, "helicopterLandings", pose.at(row.x, row.y, row.height + PAD_LIFT), mul(&pose.dir(&Vec3::Z), -1.0), row.assumed_force, loc("Helicopter hard landing", "Hubschrauber-Hartlandung"), bi(|de| format!("F = {} kN, m = {} kg", num(row.assumed_force / 1000.0, 0, de), num(row.mass, 0, de))), &check, LAYER_HELICOPTER, grips);
    }
}

/// 🪢 Ties: the tie as a tension line along x at the first floor with the provided force per metre.
pub fn ties(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for row in &d.tie_runs {
        if !out.own(&row.id) || !(row.span > 0.0) { continue; }
        let (check, grips) = (format!("en1991.1-7.tie-force.{}", row.id), handles::ids(d, layout, &row.id));
        let (a, b) = (pose.at(row.x, row.y, TIE_LEVEL), pose.at(row.x + row.span, row.y, TIE_LEVEL));
        let per_metre = row.provided_force / row.span;
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "tieRuns", Geometry::Load(LoadGlyph::Line { path: polyline(&[a.clone(), b.clone()]), direction: pose.dir(&Vec3::X), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, per_metre) }, case: case(&row.id, LoadKind::Accidental) }), load(LoadKind::Accidental), bi(|de| format!("{} {}", if row.kind == 1 { if de { "Innenzugglied" } else { "Internal tie" } } else if de { "Randzugglied" } else { "Peripheral tie" }, row.id)), LAYER_TIES)
                .bound(binding_where(report, |c| c.id == check)).channel(ColourChannel::Role).opacity(0.9)
                .detail(bi(|de| format!("T = {} kN, s = {} m, L = {} m", num(row.provided_force / 1000.0, 1, de), num(row.spacing, 1, de), num(row.span, 1, de)))).handles(grips),
            &[a, b],
        );
    }
}

/// 🧱️ Construction loads: one area load per row layered on the strip of its stage.
pub fn construction_loads(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    let (slots, y0) = (stage_slots(d), stage_row_y(d));
    for (index, row) in d.construction_loads.iter().enumerate() {
        if !out.own(&row.id) { continue; }
        let Some(slot) = d.construction_stages.iter().position(|s| s.id == row.stage) else { continue };
        let (x0, x1) = slots[slot];
        let (check, grips) = (format!("en1991.1-6.stage-combination.{}", row.stage), handles::ids(d, layout, &row.id));
        let z = stack_level(d, index);
        let points: Vec<Vec3> = rect2(x0, y0, x1.max(x0 + 0.05), y0 + STRIP_DEPTH).iter().map(|p| pose.at(p[0], p[1], z)).collect();
        out.push(
            Spec::new(EntityKind::Load, row.id.clone(), "constructionLoads", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: mul(&Vec3::Z, -1.0), intensity: pressure(row.magnitude), case: case(&row.id, LoadKind::Variable) }), load(LoadKind::Variable), bi(|de| format!("{} {}", if de { "Bauzustandslast" } else { "Construction load" }, row.id)), LAYER_CONSTRUCTION_LOADS)
                .bound(binding_where(report, |c| c.id == check)).opacity(0.6)
                .detail(bi(|de| format!("{} = {} kN/m², {} {}", ["Q_ca", "Q_cb", "Q_cc", "Q_cd", "Q_ce", "Q_cf"].get(usize::from(row.kind)).copied().unwrap_or("Q_c"), num(row.magnitude / 1000.0, 2, de), if de { "Bauzustand" } else { "stage" }, row.stage))).handles(grips),
            &points,
        );
    }
}

/// 🪨 Falling loads: the arrest force at the point of impact, the mass block at its drop height on a fall path above the impact zone, bound drop height and stopping distance dimensions.
pub fn falling_loads(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for row in &d.falling_loads {
        if !out.own(&row.id) { continue; }
        let (check, grips) = (format!("en1991.1-6.falling-load.{}", row.id), handles::ids(d, layout, &row.id));
        point(out, report, &row.id, "fallingLoads", pose.at(row.x, row.y, 0.0), mul(&pose.dir(&Vec3::Z), -1.0), row.assumed_force, loc("Falling load", "Herabfallende Last"), bi(|de| format!("F = {} kN, m = {} kg, h = {} m, s = {} m", num(row.assumed_force / 1000.0, 1, de), num(row.mass, 0, de), num(row.drop_height, 2, de), num(row.stopping_distance, 3, de))), &check, LAYER_FALLING, grips);
        let bound = binding_where(report, |c| c.id == check);
        out.label(format!("{}:drop", row.id), pose.at(row.x, row.y, row.drop_height.max(0.0)), bi(|de| format!("m = {} kg, h = {} m", num(row.mass, 0, de), num(row.drop_height, 2, de))), tone_of(&bound));
        let (side, height) = (block_side(row.mass), row.drop_height.max(0.0));
        let zone = (2.0 * side).max(1.0) / 2.0;
        let patch = [(-zone, -zone), (zone, -zone), (zone, zone), (-zone, zone)].map(|(dx, dy)| pose.at(row.x + dx, row.y + dy, PAD_LIFT));
        flat(out, format!("{}:impact-zone", row.id), "fallingLoads", &patch, loc("Impact zone", "Aufprallzone"), LAYER_FALLING, bound.clone(), 0.6);
        flat(out, format!("{}:fall-path", row.id), "fallingLoads", &ribbon(pose, row.x - 0.03, row.y, 0.06, &[(0.0, 0.0), (0.0, height)]), loc("Fall path", "Fallweg"), LAYER_FALLING, bound.clone(), 0.5);
        out.push(
            Spec::new(EntityKind::Solid, format!("{}:body", row.id), "fallingLoads", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([row.x - side / 2.0, row.y - side / 2.0, height]), base: polygon2(&rect2(0.0, 0.0, side, side)), height: side } }), load(LoadKind::Accidental), loc("Falling mass at the drop height", "Herabfallende Masse in Fallhöhe"), LAYER_FALLING).bound(bound).opacity(0.55),
            &[pose.at(row.x - side / 2.0, row.y - side / 2.0, height), pose.at(row.x + side / 2.0, row.y + side / 2.0, height + side)],
        );
        if height > 0.0 {
            let (from, to) = (pose.at(row.x - zone, row.y, 0.0), pose.at(row.x - zone, row.y, height));
            out.annotate(Annotation::Dimension { id: format!("{}:drop-height", row.id), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, height), binding: Some(FieldBinding::new(format!("fallingLoads[id={}].dropHeight", row.id), Affine::IDENTITY, "m")) }, &from);
        }
        if row.stopping_distance > 0.0 {
            let (from, to) = (pose.at(row.x + zone, row.y, 0.0), pose.at(row.x + zone, row.y, -row.stopping_distance));
            out.annotate(Annotation::Dimension { id: format!("{}:stopping", row.id), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, row.stopping_distance), binding: Some(FieldBinding::new(format!("fallingLoads[id={}].stoppingDistance", row.id), Affine::IDENTITY, "m")) }, &from);
        }
    }
}

/// 📐 Execution imperfections: the true member axis with the equivalent horizontal force at its top, the nominal axis and the exaggerated sway and bow shapes with their drawing factor.
pub fn imperfections(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for row in &d.execution_imperfections {
        if !out.own(&row.id) || !(row.height > 0.0) { continue; }
        let (check, grips) = (format!("en1991.1-6.execution-imperfection.{}", row.id), handles::ids(d, layout, &row.id));
        point(out, report, &row.id, "executionImperfections", pose.at(row.x + row.tolerance, row.y, row.height), pose.dir(&Vec3::X), row.assumed_force, loc("Execution imperfection", "Ausführungsimperfektion"), bi(|de| format!("H = {} kN, Δ = {} mm, h = {} m, N = {} kN", num(row.assumed_force / 1000.0, 2, de), num(row.tolerance * 1000.0, 0, de), num(row.height, 1, de), num(row.vertical_load / 1000.0, 0, de))), &check, LAYER_IMPERFECTIONS, grips);
        let (base, top) = (pose.at(row.x, row.y, 0.0), pose.at(row.x + row.tolerance, row.y, row.height));
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:axis", row.id), "executionImperfections", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&[base.clone(), top.clone(), pose.at(row.x + row.tolerance, row.y + 0.05, row.height), pose.at(row.x, row.y + 0.05, 0.0)]) } }), MaterialRole::Neutral, loc("Inclined member axis", "Geneigte Bauteilachse"), LAYER_IMPERFECTIONS).bound(binding_where(report, |c| c.id == check)).opacity(0.8),
            &[base, top],
        );
        let (k, bound) = (exaggeration(row.height, row.tolerance), binding_where(report, |c| c.id == check));
        flat(out, format!("{}:nominal", row.id), "executionImperfections", &ribbon(pose, row.x, row.y, 0.04, &[(0.0, 0.0), (0.0, row.height)]), loc("Nominal member axis", "Soll-Bauteilachse"), LAYER_IMPERFECTIONS, None, 0.35);
        flat(out, format!("{}:sway", row.id), "executionImperfections", &ribbon(pose, row.x, row.y, 0.04, &[(0.0, 0.0), (k * row.tolerance, row.height)]), loc("Sway imperfection (exaggerated)", "Schiefstellung (überhöht)"), LAYER_IMPERFECTIONS, bound.clone(), 0.85);
        let bow: Vec<(f64, f64)> = (0..=BOW_SEGMENTS).map(|i| (-k * row.tolerance * (std::f64::consts::PI * i as f64 / BOW_SEGMENTS as f64).sin(), row.height * i as f64 / BOW_SEGMENTS as f64)).collect();
        flat(out, format!("{}:bow", row.id), "executionImperfections", &ribbon(pose, row.x, row.y, 0.04, &bow), loc("Bow imperfection (exaggerated)", "Vorkrümmung (überhöht)"), LAYER_IMPERFECTIONS, bound.clone(), 0.85);
        out.label(format!("{}:exaggeration", row.id), pose.at(row.x + k * row.tolerance, row.y, row.height), bi(|de| format!("Δ = {} mm, drawn ×{}", num(row.tolerance * 1000.0, 0, de), num(k, 0, de))), tone_of(&bound));
    }
}

/// 🚆 Derailment cases: the track with its rails at the entered distance, the derailed vehicle position, the design forces F_dx and F_dy at 1.8 m on the structure and the bound distance.
pub fn derailments(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    for row in &d.derailment_cases {
        if !out.own(&row.id) { continue; }
        let (check, grips) = (format!("en1991.1-7.derailment-force.{}", row.id), handles::ids(d, layout, &row.id));
        point(out, report, &row.id, "derailmentCases", pose.at(row.x, row.y, TRACK_LEVEL), pose.dir(&Vec3::X), row.assumed_force, loc("Derailment force", "Entgleisungskraft"), bi(|de| format!("F_dx = {} kN, F_dy = {} kN, d = {} m", num(row.design_force_x / 1000.0, 0, de), num(row.design_force_y / 1000.0, 0, de), num(row.distance, 1, de))), &check, LAYER_DERAILMENT, grips);
        let (a, b) = (pose.at(row.x - 10.0, row.y - row.distance, 0.0), pose.at(row.x + 10.0, row.y - row.distance, 0.0));
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:track", row.id), "derailmentCases", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&[a.clone(), b.clone(), pose.at(row.x + 10.0, row.y - row.distance + 1.5, 0.0), pose.at(row.x - 10.0, row.y - row.distance + 1.5, 0.0)]) } }), MaterialRole::Neutral, loc("Track", "Gleis"), LAYER_DERAILMENT).bound(binding_where(report, |c| c.id == check)).opacity(0.7),
            &[a, b],
        );
        let bound = binding_where(report, |c| c.id == check);
        let axis_y = row.y - row.distance + TRACK_WIDTH / 2.0;
        for (tag, side) in [("left", -1.0), ("right", 1.0)] {
            let y = axis_y + side * RAIL_GAUGE / 2.0 - RAIL_WIDTH / 2.0;
            let rail = [(row.x - TRACK_HALF_LENGTH, y), (row.x + TRACK_HALF_LENGTH, y), (row.x + TRACK_HALF_LENGTH, y + RAIL_WIDTH), (row.x - TRACK_HALF_LENGTH, y + RAIL_WIDTH)].map(|(px, py)| pose.at(px, py, 0.03));
            flat(out, format!("{}:rail-{tag}", row.id), "derailmentCases", &rail, loc("Rail", "Schiene"), LAYER_DERAILMENT, None, 0.9);
        }
        let marker = [(-0.4, -0.4), (0.4, -0.4), (0.4, 0.4), (-0.4, 0.4)].map(|(dx, dy)| pose.at(row.x + dx, axis_y + dy, 0.04));
        flat(out, format!("{}:position", row.id), "derailmentCases", &marker, loc("Derailed vehicle on the track", "Entgleistes Fahrzeug auf dem Gleis"), LAYER_DERAILMENT, bound.clone(), 0.8);
        let hit = pose.at(row.x, row.y, TRACK_LEVEL);
        for (tag, direction, magnitude, en, de) in [("fdx", Vec3::X, row.design_force_x, "Design force F_dx along the track", "Bemessungskraft F_dx in Gleisrichtung"), ("fdy", Vec3::Y, row.design_force_y, "Design force F_dy across the track", "Bemessungskraft F_dy quer zum Gleis")] {
            out.push(
                Spec::new(EntityKind::Load, format!("{}:{tag}", row.id), "derailmentCases", Geometry::Load(LoadGlyph::Point { at: hit.clone(), direction: pose.dir(&direction), magnitude: force(magnitude), case: case(&format!("{}-{tag}", row.id), LoadKind::Accidental) }), load(LoadKind::Accidental), loc(en, de), LAYER_DERAILMENT).bound(bound.clone()),
                &[hit.clone()],
            );
        }
        let (from, to) = (pose.at(row.x, axis_y, 0.0), pose.at(row.x, row.y, 0.0));
        out.annotate(Annotation::Dimension { id: format!("{}:distance", row.id), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, row.distance), binding: Some(FieldBinding::new(format!("derailmentCases[id={}].distance", row.id), Affine::IDENTITY, "m")) }, &from);
    }
}

