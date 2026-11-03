//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 🏹 Loads: one magnitude grip per load arrow of a beam, column or slab action (area load, point load, column N and M), writing the typed `change-<parent>-action-*` kinds in one commit per drag.
//!
//! 📏️ Beams: span, slab thickness, effective width, the position, diameter and height of every stud, sheeting height and orientation, section choice. Columns: length (the buckling length), outer size, wall thickness, bar area (steel ratio), bar axis distance and the encased I-section. Slabs: span, thickness, sheeting height and orientation.

use super::beams::MAX_STUDS_DRAWN;
use super::facts::{BeamFacts, ColumnFacts, SlabFacts};
use super::kit::{base_id, entity_ref, loc};
use super::layout::{Layout, Placement, Pose, STRIP_WIDTH};
use super::model::{En1994Snapshot, SteelSection};
use super::scene::*;
use super::scene::projection_kit::handle_kit::{grid, handle, HandleSpec as Spec, field_binding as binding, round};

/// 🧱️ The unit string a section-choice write carries: the value is the catalogue section height in m and the editor resolves it to the catalogue designation.
pub const SECTION_HEIGHT_UNIT: &str = "section-height";

const STUD_DIAMETER_MIN_M: f64 = 0.010;
const STUD_DIAMETER_MAX_M: f64 = 0.025;

const CATALOGUE: [&str; 11] = ["HEB300", "HEB320", "HEB340", "HEB360", "HEB400", "HEB450", "HEB500", "HEB550", "HEB600", "HEB650", "HEB700"];

fn catalogue_heights() -> Vec<f64> {
    CATALOGUE.iter().filter_map(|d| SteelSection::from_catalogue(d)).map(|s| s.height_m).collect()
}

fn orientation(entity: &EntityRef, collection: &str, id: &str, anchor: Vec3) -> Handle {
    handle(Spec {
        entity,
        tag: "orientation",
        kind: HandleKind::CountStepper,
        anchor,
        grip: GripStyle::Diamond,
        writes: vec![binding(format!("{collection}[id={id}].sheeting.ribsParallelToBeam"), Affine::IDENTITY, "1", Some(Quantise::Int))],
        snap: SnapRule::NONE,
        limits: (0.0, 1.0),
        name: loc("Sheeting ribs parallel to the beam (1) or transverse (0)", "Profilblechrippen parallel zum Träger (1) oder quer (0)"),
    })
}

const AREA_LOAD_PA_PER_M: f64 = 2.0e4;
const POINT_LOAD_N_PER_M: f64 = 2.0e5;
const COLUMN_N_PER_M: f64 = 1.0e6;
const COLUMN_M_NM_PER_M: f64 = 2.0e5;
const LOAD_ARROW_MAX_M: f64 = 3.0;

struct Magnitude {
    field: &'static str,
    per_m: f64,
    unit: &'static str,
    step: f64,
    value: f64,
    en: &'static str,
    de: &'static str,
}

fn area_or_point(q_area_pa: f64, f_k_n: f64) -> Option<Magnitude> {
    if q_area_pa.abs() > 1e-9 {
        Some(Magnitude { field: "qAreaPa", per_m: AREA_LOAD_PA_PER_M, unit: "Pa", step: 100.0, value: q_area_pa.abs(), en: "Area load", de: "Flächenlast" })
    } else if f_k_n.abs() > 1.0 {
        Some(Magnitude { field: "fKN", per_m: POINT_LOAD_N_PER_M, unit: "N", step: 500.0, value: f_k_n.abs(), en: "Point load", de: "Einzellast" })
    } else {
        None
    }
}

/// 🏹 The magnitude grip of one load arrow: the arrow tip height above its rest height `z0` is `value / per_m`, so dragging the tip writes the characteristic load of that action (`insert-` and `remove-` of the action rows are properties-panel rows).
fn load_arrow(entity: &EntityRef, pose: &Pose, collection: &str, parent: &str, action: &str, x: f64, y: f64, z0: f64, m: &Magnitude) -> Handle {
    handle(Spec {
        entity,
        tag: &format!("action-{action}"),
        kind: HandleKind::ScalarAlong { axis: pose.dir(0.0, 0.0, 1.0) },
        anchor: pose.point(x, y, z0 + m.value / m.per_m),
        grip: GripStyle::Arrow,
        writes: vec![binding(format!("{collection}[id={parent}].actions[id={action}].{}", m.field), Affine { base: -z0 * m.per_m, k: m.per_m }, m.unit, round(m.step))],
        snap: grid(m.step / m.per_m),
        limits: (z0, z0 + LOAD_ARROW_MAX_M.max(1.5 * m.value / m.per_m)),
        name: loc(&format!("{} {action} of {parent}", m.en), &format!("{} {action} von {parent}", m.de)),
    })
}

/// 📍️ The position grip of a point force: dragging the foot of the arrow along the span (`xM`, one commit per drag) or across the strip (`yM`) writes the typed `change-<parent>-action-<x|y>-m` kind; the first support end is excluded because `0` means the unset mid-span default.
fn position_grip(entity: &EntityRef, pose: &Pose, collection: &str, parent: &str, action: &str, field: &'static str, axis: Vec3, anchor: Vec3, shift: f64, limits: (f64, f64)) -> Handle {
    handle(Spec {
        entity,
        tag: &format!("action-{action}-{}", field.to_lowercase().trim_end_matches('m')),
        kind: HandleKind::ScalarAlong { axis },
        anchor,
        grip: GripStyle::Diamond,
        writes: vec![binding(format!("{collection}[id={parent}].actions[id={action}].{field}"), Affine { base: -shift, k: 1.0 }, "m", round(0.05))],
        snap: grid(0.05),
        limits,
        name: loc(&format!("Position {} of point load {action} of {parent}", &field[..1]), &format!("Position {} der Einzellast {action} von {parent}", &field[..1])),
    })
}

fn staggered(index: usize, count: usize, l_total: f64) -> f64 {
    l_total * (0.15 + 0.7 * (index as f64 + 1.0) / (count as f64 + 1.0))
}

fn beam_action_handles(f: &BeamFacts, pose: &Pose, entity: &EntityRef) -> Vec<Handle> {
    let id = f.beam.id.as_str();
    let actions: Vec<_> = f.beam.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue).filter_map(|a| area_or_point(a.q_area_pa, a.f_k_n).map(|m| (a, m))).collect();
    let origin = f.layout.origin(f.layout.own);
    let span = f.beam.span_m;
    let z0 = f.depth + 0.02;
    let mut out = Vec::new();
    for (i, (a, m)) in actions.iter().enumerate() {
        let point = m.field == "fKN";
        let x = if point { f.point_x(a) } else { staggered(i, actions.len(), f.l_total) };
        out.push(load_arrow(entity, pose, "beams", id, &a.id, x, 0.0, z0, m));
        if point {
            out.push(position_grip(entity, pose, "beams", id, &a.id, "xM", pose.ex(), pose.point(x, 0.0, z0), origin, (origin + 0.01 * span, origin + span)));
        }
    }
    out
}

fn slab_action_handles(f: &SlabFacts, pose: &Pose, entity: &EntityRef) -> Vec<Handle> {
    let id = f.slab.id.as_str();
    let actions: Vec<_> = f.slab.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue).filter_map(|a| area_or_point(a.q_area_pa, a.f_k_n).map(|m| (a, m))).collect();
    let span = f.slab.span_m;
    let z0 = f.slab.concrete_thickness_m + 0.02;
    let mut out = Vec::new();
    for (i, (a, m)) in actions.iter().enumerate() {
        let point = m.field == "fKN";
        let x = if point { f.point_x(a) } else { staggered(i, actions.len(), f.l_total) };
        let y = if point { f.point_y(a, STRIP_WIDTH) } else { 0.0 };
        out.push(load_arrow(entity, pose, "slabs", id, &a.id, x, y, z0, m));
        if point {
            out.push(position_grip(entity, pose, "slabs", id, &a.id, "xM", pose.ex(), pose.point(x, y, z0), 0.0, (0.01 * span, span)));
            out.push(position_grip(entity, pose, "slabs", id, &a.id, "yM", pose.ey(), pose.point(x, y, z0), 0.0, (-STRIP_WIDTH / 2.0, STRIP_WIDTH / 2.0)));
        }
    }
    out
}

fn column_action_handles(f: &ColumnFacts, pose: &Pose, entity: &EntityRef) -> Vec<Handle> {
    let c = &f.column;
    let id = c.id.as_str();
    let mut out = Vec::new();
    for (index, action) in c.actions.iter().enumerate() {
        let z0 = c.length_m + 0.15 * index as f64;
        let aid = action.id.as_str();
        if action.n_k_n.abs() > 1.0 {
            out.push(load_arrow(entity, pose, "columns", id, aid, 0.0, 0.0, z0, &Magnitude { field: "nKN", per_m: COLUMN_N_PER_M, unit: "N", step: 1.0e3, value: action.n_k_n.abs(), en: "Axial load", de: "Normalkraft" }));
        }
        if action.m_k_nm.abs() > 1.0 {
            out.push(handle(Spec {
                entity,
                tag: &format!("action-{aid}-m"),
                kind: HandleKind::ScalarAlong { axis: pose.dir(0.0, 1.0, 0.0) },
                anchor: pose.point(0.0, action.m_k_nm.abs() / COLUMN_M_NM_PER_M, z0 + 0.1),
                grip: GripStyle::Ring,
                writes: vec![binding(format!("columns[id={id}].actions[id={aid}].mKNm"), Affine { base: 0.0, k: COLUMN_M_NM_PER_M }, "N·m", round(500.0))],
                snap: grid(500.0 / COLUMN_M_NM_PER_M),
                limits: (0.0, LOAD_ARROW_MAX_M.max(1.5 * action.m_k_nm.abs() / COLUMN_M_NM_PER_M)),
                name: loc(&format!("Moment {aid} of {id}"), &format!("Moment {aid} von {id}")),
            }));
        }
    }
    out
}

/// 🪛️ The shear-connector grips: the transverse stud spacing of the beam and, for every stud of the persisted collection, three grips writing its own row: the position along the beam (`studs[id=..].xM`, at the foot), the shank diameter (sideways at mid height) and the height (at the head). Studs are added, duplicated and removed through the insert palette.
fn connector_handles(f: &BeamFacts, pose: &Pose, entity: &EntityRef) -> Vec<Handle> {
    let id = f.beam.id.as_str();
    let d = f.studs.max_diameter_m().max(0.006);
    let cross = if f.beam.stud_transverse_spacing_m > 1e-9 { f.beam.stud_transverse_spacing_m } else { 4.0 * d };
    let mut out = vec![handle(Spec {
        entity,
        tag: "stud-cross-spacing",
        kind: HandleKind::ScalarAlong { axis: pose.ey() },
        anchor: pose.point(f.first_station_x(), cross / 2.0, f.stud_top()),
        grip: GripStyle::Diamond,
        writes: vec![binding(format!("beams[id={id}].studTransverseSpacingM"), Affine { base: 0.0, k: 2.0 }, "m", round(0.005))],
        snap: grid(0.0025),
        limits: (0.005, (f.steel.width_m / 2.0).max(0.02)),
        name: loc(&format!("Transverse stud spacing of {id}"), &format!("Bolzenquerabstand von {id}")),
    })];
    for (row, y) in f.studs.rows.iter().zip(f.stud_ys()).take(MAX_STUDS_DRAWN) {
        let sid = row.id.as_str();
        let radius = row.diameter_m / 2.0;
        out.push(handle(Spec {
            entity,
            tag: &format!("stud-{sid}-x"),
            kind: HandleKind::ScalarAlong { axis: pose.ex() },
            anchor: pose.point(row.x_m, y, 0.0),
            grip: GripStyle::Ring,
            writes: vec![binding(format!("studs[id={sid}].xM"), Affine::IDENTITY, "m", round(0.005))],
            snap: grid(0.005),
            limits: (0.0, f.l_total),
            name: loc(&format!("Position of stud {sid}"), &format!("Position von Bolzen {sid}")),
        }));
        out.push(handle(Spec {
            entity,
            tag: &format!("stud-{sid}-d"),
            kind: HandleKind::ScalarAlong { axis: pose.ey() },
            anchor: pose.point(row.x_m, y + radius, row.height_m / 2.0),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("studs[id={sid}].diameterM"), Affine { base: -2.0 * y, k: 2.0 }, "m", round(0.001))],
            snap: grid(0.0005),
            limits: (y + STUD_DIAMETER_MIN_M / 2.0, y + STUD_DIAMETER_MAX_M / 2.0),
            name: loc(&format!("Diameter of stud {sid}"), &format!("Durchmesser von Bolzen {sid}")),
        }));
        out.push(handle(Spec {
            entity,
            tag: &format!("stud-{sid}-h"),
            kind: HandleKind::FaceOffset { normal: pose.dir(0.0, 0.0, 1.0) },
            anchor: pose.point(row.x_m, y, row.height_m),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("studs[id={sid}].heightM"), Affine::IDENTITY, "m", round(0.005))],
            snap: grid(0.005),
            limits: (0.05, (f.depth - 0.02).max(0.06)),
            name: loc(&format!("Height of stud {sid}"), &format!("Höhe von Bolzen {sid}")),
        }));
    }
    out
}

fn beam_handles(f: &BeamFacts, pose: &Pose) -> Vec<Handle> {
    let id = f.beam.id.as_str();
    let entity = entity_ref(EntityKind::Member, id, "beams");
    let w = f.slab_width();
    let ex = pose.ex();
    let span_scale = f.beam.span_m / f.l_total.max(1e-9);
    let neighbour = f.beam.neighbour_span_m();
    let bearing = if f.beam.patch_bearing_length_m > 1e-9 { f.beam.patch_bearing_length_m } else { 0.1 };
    let heights = catalogue_heights();
    let (beff_path, beff_map) = if f.b_eff_capped { (format!("beams[id={id}].spacingM"), Affine::IDENTITY) } else { (format!("beams[id={id}].spanM"), Affine { base: -4.0 * f.steel.width_m, k: 4.0 }) };
    let mut out = vec![
        handle(Spec {
            entity: &entity,
            tag: "span",
            kind: HandleKind::ScalarAlong { axis: ex.clone() },
            anchor: pose.point(f.l_total, 0.0, -f.steel.height_m),
            grip: GripStyle::Arrow,
            writes: vec![binding(format!("beams[id={id}].spanM"), Affine { base: 0.0, k: span_scale }, "m", round(0.01))],
            snap: grid(0.05 / span_scale),
            limits: (1.0 / span_scale, 60.0 / span_scale),
            name: loc(&format!("Span of {id}"), &format!("Spannweite von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "slab-thickness",
            kind: HandleKind::FaceOffset { normal: pose.dir(0.0, 0.0, 1.0) },
            anchor: pose.point(f.l_total / 2.0, w / 2.0, f.depth),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].slabThicknessM"), Affine::IDENTITY, "m", round(0.005))],
            snap: grid(0.005),
            limits: (f.hp + 0.04, 0.5),
            name: loc(&format!("Slab thickness of {id}"), &format!("Plattendicke von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "effective-width",
            kind: HandleKind::DimensionLine,
            anchor: pose.point(f.l_total / 2.0, f.b_eff / 2.0, f.depth + 0.1),
            grip: GripStyle::Arrow,
            writes: vec![binding(beff_path, beff_map, "m", round(0.01))],
            snap: grid(0.05),
            limits: (f.steel.width_m + 0.1, 12.0),
            name: loc(&format!("Effective width of {id}"), &format!("Mitwirkende Breite von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "sheeting-height",
            kind: HandleKind::FaceOffset { normal: pose.dir(0.0, 0.0, 1.0) },
            anchor: pose.point(f.l_total * 0.2, w / 2.0, f.hp),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].sheeting.heightM"), Affine::IDENTITY, "m", round(0.005))],
            snap: grid(0.005),
            limits: (0.04, (f.depth - 0.04).max(0.05)),
            name: loc(&format!("Sheeting height of {id}"), &format!("Profilhöhe von {id}")),
        }),
        orientation(&entity, "beams", id, pose.point(f.l_total * 0.1, w / 2.0, f.hp)),
        handle(Spec {
            entity: &entity,
            tag: "adjacent-span",
            kind: HandleKind::ScalarAlong { axis: ex.clone() },
            anchor: pose.point(f.beam.span_m + neighbour, 0.0, -f.steel.height_m),
            grip: GripStyle::Arrow,
            writes: vec![binding(format!("beams[id={id}].adjacentSpanM"), Affine { base: -f.beam.span_m, k: 1.0 }, "m", round(0.01))],
            snap: grid(0.05),
            limits: (f.beam.span_m + 1.0, f.beam.span_m + 60.0),
            name: loc(&format!("Adjacent span of {id}"), &format!("Nachbarfeld von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "patch-bearing",
            kind: HandleKind::ScalarAlong { axis: ex.clone() },
            anchor: pose.point(f.l_total / 2.0 + bearing / 2.0, 0.0, -f.steel.height_m - 0.2),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].patchBearingLengthM"), Affine { base: -f.l_total, k: 2.0 }, "m", round(0.005))],
            snap: grid(0.005),
            limits: (f.l_total / 2.0 + 0.0025, f.l_total / 2.0 + 0.5),
            name: loc(&format!("Patch load bearing length of {id}"), &format!("Lasteinleitungslänge von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "bridge-lanes",
            kind: HandleKind::CountStepper,
            anchor: pose.point(f.l_total * 0.6, w / 2.0 + 0.1, f.depth + 0.1),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].bridgeLanes"), Affine::IDENTITY, "1", Some(Quantise::Int))],
            snap: SnapRule::NONE,
            limits: (0.0, 3.0),
            name: loc(&format!("Notional lanes on {id}"), &format!("Rechnerische Fahrstreifen auf {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "section",
            kind: HandleKind::FaceOffset { normal: pose.dir(0.0, 0.0, -1.0) },
            anchor: pose.point(f.l_total / 2.0, 0.0, -f.steel.height_m),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].steel.designation"), Affine::IDENTITY, SECTION_HEIGHT_UNIT, Some(Quantise::Choice { values: heights.clone() }))],
            snap: SnapRule { catalogue: heights.clone(), tolerance: 0.05, ..SnapRule::NONE },
            limits: (heights.first().copied().unwrap_or(0.3), heights.last().copied().unwrap_or(0.7)),
            name: loc(&format!("Steel section of {id} (catalogue height)"), &format!("Stahlprofil von {id} (Kataloghöhe)")),
        }),
    ];
    out.extend(connector_handles(f, pose, &entity));
    out.extend(beam_action_handles(f, pose, &entity));
    if f.bridge {
        let ey = pose.dir(0.0, 1.0, 0.0);
        out.push(handle(Spec {
            entity: &entity,
            tag: "end-zone-length",
            kind: HandleKind::ScalarAlong { axis: ex.clone() },
            anchor: pose.point(f.beam.end_zone_length_m, 0.0, f.depth + 0.05),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].endZoneLengthM"), Affine::IDENTITY, "m", round(0.01))],
            snap: grid(0.05),
            limits: (0.0, f.l_total / 2.0),
            name: loc(&format!("End zone length of {id}"), &format!("Endbereichslänge von {id}")),
        }));
        out.push(handle(Spec {
            entity: &entity,
            tag: "end-plate-width",
            kind: HandleKind::ScalarAlong { axis: ey },
            anchor: pose.point(0.0, f.beam.end_plate_width_m / 2.0, f.depth - f.beam.end_plate_height_m / 2.0),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].endPlateWidthM"), Affine { base: 0.0, k: 2.0 }, "m", round(0.01))],
            snap: grid(0.01),
            limits: (0.0, w / 2.0),
            name: loc(&format!("End plate width of {id}"), &format!("Endplattenbreite von {id}")),
        }));
        out.push(handle(Spec {
            entity: &entity,
            tag: "end-plate-height",
            kind: HandleKind::FaceOffset { normal: pose.dir(0.0, 0.0, -1.0) },
            anchor: pose.point(0.0, 0.0, f.depth - f.beam.end_plate_height_m),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("beams[id={id}].endPlateHeightM"), Affine { base: f.depth, k: 1.0 }, "m", round(0.01))],
            snap: grid(0.01),
            limits: (0.02 - f.depth, 1.0 - f.depth),
            name: loc(&format!("End plate height of {id}"), &format!("Endplattenhöhe von {id}")),
        }));
    }
    if f.hogging {
        let (origin, l) = (f.layout.origin(f.layout.own), f.beam.span_m.max(1e-9));
        let left = f.layout.cantilever || f.layout.own > 0;
        let (affine, limits) = if left { (Affine { base: -origin / l, k: 1.0 / l }, (origin, origin + 0.5 * l)) } else { (Affine { base: (origin + l) / l, k: -1.0 / l }, (origin + 0.5 * l, origin + l)) };
        let x = if left { origin + f.stiffness.cracked_fraction * l } else { origin + l - f.stiffness.cracked_fraction * l };
        out.push(handle(Spec {
            entity: &entity,
            tag: "cracked-length",
            kind: HandleKind::ScalarAlong { axis: pose.ex() },
            anchor: pose.point(x, 0.0, f.depth + 0.05),
            grip: GripStyle::Ring,
            writes: vec![binding(format!("beams[id={id}].crackedLengthFraction"), affine, "1", round(0.01))],
            snap: grid(0.01 * l),
            limits,
            name: loc(&format!("Cracked length at the internal support of {id} (EI₂)"), &format!("Gerissene Länge an der Innenstütze von {id} (EI₂)")),
        }));
    }
    out
}

fn column_handles(f: &ColumnFacts, pose: &Pose) -> Vec<Handle> {
    let c = &f.column;
    let id = c.id.as_str();
    let entity = entity_ref(EntityKind::Member, id, "columns");
    let mut out = vec![
        handle(Spec {
            entity: &entity,
            tag: "length",
            kind: HandleKind::ScalarAlong { axis: pose.dir(0.0, 0.0, 1.0) },
            anchor: pose.point(0.0, 0.0, c.length_m),
            grip: GripStyle::Arrow,
            writes: vec![binding(format!("columns[id={id}].lengthM"), Affine::IDENTITY, "m", round(0.01))],
            snap: grid(0.05),
            limits: (1.0, 30.0),
            name: loc(&format!("Length of {id}"), &format!("Länge von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "size",
            kind: HandleKind::FaceOffset { normal: pose.ex() },
            anchor: pose.point(c.outer_size_m / 2.0, 0.0, c.length_m / 2.0),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("columns[id={id}].outerSizeM"), Affine { base: 0.0, k: 2.0 }, "m", round(0.005))],
            snap: grid(0.0025),
            limits: (0.05, 0.75),
            name: loc(&format!("Outer size of {id}"), &format!("Außenmaß von {id}")),
        }),
    ];
    out.extend(column_action_handles(f, pose, &entity));
    out.push(handle(Spec {
        entity: &entity,
        tag: "introduction",
        kind: HandleKind::ScalarAlong { axis: pose.dir(0.0, 0.0, 1.0) },
        anchor: pose.point(0.0, 0.0, c.length_m * 0.9),
        grip: GripStyle::Ring,
        writes: vec![binding(format!("columns[id={id}].introducedForceN"), Affine { base: 0.0, k: 1.0e5 / c.length_m.max(1e-9) }, "N", round(1.0e3))],
        snap: grid(0.1),
        limits: (0.0, c.length_m),
        name: loc(&format!("Introduced force of {id}"), &format!("Eingeleitete Kraft von {id}")),
    }));
    let a_max = 0.06 * c.outer_size_m.powi(2);
    let per_m = 2.0 * a_max / c.outer_size_m.max(1e-9);
    out.push(handle(Spec {
        entity: &entity,
        tag: "rebar-ratio",
        kind: HandleKind::ScalarAlong { axis: pose.dir(0.0, 1.0, 0.0) },
        anchor: pose.point(0.0, c.reinforcement_as_m2 / per_m.max(1e-9), c.length_m * 0.4),
        grip: GripStyle::Ring,
        writes: vec![binding(format!("columns[id={id}].reinforcementAsM2"), Affine { base: 0.0, k: per_m }, "m2", round(1.0e-5))],
        snap: grid(0.001),
        limits: (0.0, c.outer_size_m / 2.0),
        name: loc(&format!("Steel ratio of the bars of {id} (up to 6 % of the section)"), &format!("Bewehrungsgrad der Längsbewehrung von {id} (bis 6 % des Querschnitts)")),
    }));
    if matches!(c.kind, semio_s_artifact_norm_en1994_model::ColumnKind::Encased | semio_s_artifact_norm_en1994_model::ColumnKind::PartiallyEncased) {
        let or = |v: f64, fallback: f64| if v > 1e-6 { v } else { fallback };
        let (h, b, tw, tf) = (or(c.section_height_m, c.outer_size_m * 0.5), or(c.section_width_m, c.outer_size_m * 0.5), or(c.section_tw_m, 0.01), or(c.section_tf_m, 0.015));
        let cover = or(c.rebar_cover_m, 0.05);
        let leaves: [(&str, &str, &str, &str, Vec3, Affine, f64, f64); 5] = [
            ("section-height", "sectionHeightM", "Height of the encased I-section of", "Höhe des einbetonierten I-Profils von", pose.point(h / 2.0, 0.0, c.length_m * 0.5), Affine { base: 0.0, k: 2.0 }, 0.05, c.outer_size_m),
            ("section-width", "sectionWidthM", "Flange width of the encased I-section of", "Flanschbreite des einbetonierten I-Profils von", pose.point(0.0, b / 2.0, c.length_m * 0.55), Affine { base: 0.0, k: 2.0 }, 0.05, c.outer_size_m),
            ("section-tw", "sectionTwM", "Web thickness of the encased I-section of", "Stegdicke des einbetonierten I-Profils von", pose.point(tw / 2.0, 0.0, c.length_m * 0.6), Affine { base: 0.0, k: 2.0 }, 0.003, 0.05),
            ("section-tf", "sectionTfM", "Flange thickness of the encased I-section of", "Flanschdicke des einbetonierten I-Profils von", pose.point(h / 2.0 - tf, 0.0, c.length_m * 0.65), Affine { base: h / 2.0, k: -1.0 }, 0.004, 0.08),
            ("rebar-cover", "rebarCoverM", "Axis distance of the bars of", "Achsabstand der Längsbewehrung von", pose.point(c.outer_size_m / 2.0 - cover, c.outer_size_m / 2.0 - cover, c.length_m * 0.75), Affine { base: c.outer_size_m / 2.0, k: -1.0 }, 0.02, c.outer_size_m / 2.0 - 0.01),
        ];
        for (tag, leaf, en, de, anchor, affine, lo, hi) in leaves {
            let (limits, normal) = if tag == "section-width" { ((lo / 2.0, hi / 2.0), pose.ey()) } else if affine.k < 0.0 { ((affine.base - hi, affine.base - lo), pose.ex()) } else { ((lo / 2.0, hi / 2.0), pose.ex()) };
            out.push(handle(Spec {
                entity: &entity,
                tag,
                kind: HandleKind::FaceOffset { normal },
                anchor,
                grip: GripStyle::Ring,
                writes: vec![binding(format!("columns[id={id}].{leaf}"), affine, "m", round(0.001))],
                snap: grid(0.001),
                limits,
                name: loc(&format!("{en} {id}"), &format!("{de} {id}")),
            }));
        }
    }
    if c.kind == semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled {
        let half = c.outer_size_m / 2.0;
        out.push(handle(Spec {
            entity: &entity,
            tag: "wall",
            kind: HandleKind::FaceOffset { normal: pose.ex() },
            anchor: pose.point(half - c.wall_thickness_m, 0.0, c.length_m / 4.0),
            grip: GripStyle::Ring,
            writes: vec![binding(format!("columns[id={id}].wallThicknessM"), Affine { base: half, k: -1.0 }, "m", round(0.001))],
            snap: grid(0.001),
            limits: ((half - 0.05).max(0.0), (half - 0.002).max(0.001)),
            name: loc(&format!("Wall thickness of {id}"), &format!("Wanddicke von {id}")),
        }));
    }
    out
}

fn slab_handles(f: &SlabFacts, pose: &Pose) -> Vec<Handle> {
    let s = &f.slab;
    let id = s.id.as_str();
    let entity = entity_ref(EntityKind::Plate, id, "slabs");
    let span_scale = if f.continuous { 0.5 } else { 1.0 };
    let per_width = 0.04 * s.concrete_thickness_m / (STRIP_WIDTH / 2.0);
    let mut out = vec![
        handle(Spec {
            entity: &entity,
            tag: "span",
            kind: HandleKind::ScalarAlong { axis: pose.ex() },
            anchor: pose.point(f.l_total, 0.0, 0.0),
            grip: GripStyle::Arrow,
            writes: vec![binding(format!("slabs[id={id}].spanM"), Affine { base: 0.0, k: span_scale }, "m", round(0.01))],
            snap: grid(0.05 / span_scale),
            limits: (0.5 / span_scale, 12.0 / span_scale),
            name: loc(&format!("Span of {id}"), &format!("Spannweite von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "thickness",
            kind: HandleKind::FaceOffset { normal: pose.dir(0.0, 0.0, 1.0) },
            anchor: pose.point(f.l_total / 2.0, STRIP_WIDTH / 2.0, s.concrete_thickness_m),
            grip: GripStyle::Diamond,
            writes: vec![binding(format!("slabs[id={id}].concreteThicknessM"), Affine::IDENTITY, "m", round(0.005))],
            snap: grid(0.005),
            limits: (f.hp + 0.04, 0.4),
            name: loc(&format!("Slab thickness of {id}"), &format!("Deckendicke von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "sheeting-height",
            kind: HandleKind::FaceOffset { normal: pose.dir(0.0, 0.0, 1.0) },
            anchor: pose.point(f.l_total * 0.2, STRIP_WIDTH / 2.0, f.hp),
            grip: GripStyle::Ring,
            writes: vec![binding(format!("slabs[id={id}].sheeting.heightM"), Affine::IDENTITY, "m", round(0.005))],
            snap: grid(0.005),
            limits: (0.04, (s.concrete_thickness_m - 0.04).max(0.05)),
            name: loc(&format!("Sheeting height of {id}"), &format!("Profilhöhe von {id}")),
        }),
        handle(Spec {
            entity: &entity,
            tag: "rebar-area",
            kind: HandleKind::ScalarAlong { axis: pose.dir(0.0, 1.0, 0.0) },
            anchor: pose.point(f.l_total * 0.3, s.as_m2_per_m / per_width.max(1e-12), (s.concrete_thickness_m - 0.03).max(f.hp + 0.01)),
            grip: GripStyle::Ring,
            writes: vec![binding(format!("slabs[id={id}].asM2PerM"), Affine { base: 0.0, k: per_width }, "m2/m", round(1.0e-5))],
            snap: grid(0.001),
            limits: (0.0, STRIP_WIDTH / 2.0),
            name: loc(&format!("Reinforcement A_s of {id} (up to 4 % of the slab section)"), &format!("Bewehrung A_s von {id} (bis 4 % des Deckenquerschnitts)")),
        }),
        orientation(&entity, "slabs", id, pose.point(f.l_total * 0.1, STRIP_WIDTH / 2.0, f.hp)),
    ];
    out.extend(slab_action_handles(f, pose, &entity));
    out
}

/// 🧲️ The move and rotate grips of a placed beam, column or slab strip; a drag writes `placements[id=ENTITY].x|y|yaw`.
fn moves(id: &str, collection: &str, kind: EntityKind, pose: &Pose, stored: &[Placement]) -> Vec<Handle> {
    super::scene::projection_kit::handle_kit::move_handles(&entity_ref(kind, id, collection), id, &pose.origin, stored, pose.yaw, true)
}

/// 🕹️ The handles of one document entity, anchored in the given layout.
pub fn build(document: &En1994Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let id = base_id(id);
    if let (Some(row), Some(pose)) = (document.cases.iter().find(|c| c.id == id), layout.case(id)) {
        return super::scene::projection_kit::design_case::forms::handles::<super::checks::case_catalogue::En1994Cases>(row, pose, "cases");
    }
    let Some(pose) = layout.pose(id) else { return Vec::new() };
    let stored: Vec<_> = document.placements.iter().map(|p| Placement { entity_id: p.id.clone(), origin: super::scene::Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    if let Some(beam) = document.beams.iter().find(|b| b.id == id) {
        return [beam_handles(&BeamFacts::of(document, beam), pose), moves(id, "beams", EntityKind::Member, pose, &stored)].concat();
    }
    if let Some(column) = document.columns.iter().find(|c| c.id == id) {
        return [column_handles(&ColumnFacts::of(document, column), pose), moves(id, "columns", EntityKind::Member, pose, &stored)].concat();
    }
    document.slabs.iter().find(|s| s.id == id).map(|s| [slab_handles(&SlabFacts::of(document, s), pose), moves(id, "slabs", EntityKind::Plate, pose, &stored)].concat()).unwrap_or_default()
}

fn ids_of(handles: Vec<Handle>) -> Vec<String> {
    handles.into_iter().map(|h| h.id).collect()
}

pub fn ids(_document: &En1994Snapshot, facts: &BeamFacts, pose: &Pose, id: &str) -> Vec<String> {
    ids_of([beam_handles(facts, pose), moves(id, "beams", EntityKind::Member, pose, &[])].concat())
}

pub fn column_ids(facts: &ColumnFacts, pose: &Pose, id: &str) -> Vec<String> {
    ids_of([column_handles(facts, pose), moves(id, "columns", EntityKind::Member, pose, &[])].concat())
}

pub fn slab_ids(facts: &SlabFacts, pose: &Pose, id: &str) -> Vec<String> {
    ids_of([slab_handles(facts, pose), moves(id, "slabs", EntityKind::Plate, pose, &[])].concat())
}

/// 🕹️ The handles of every selected entity (derived scene ids resolve to their document entity), in selection order, without duplicates.
pub fn handles(document: &En1994Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        for handle in build(document, &layout, &entity.id) {
            if !out.iter().any(|h| h.id == handle.id) {
                out.push(handle);
            }
        }
    }
    out
}
