//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s` mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 🧭️ Zero of an `Angular` handle: the signed angle `s` (radians, right-handed about `axis`) from the reference direction `+X` projected perpendicular to the axis (`+Z` when the axis is `±X`) to `anchor − centre`; the structural-scene sources define no zero, so this is the EN 1995 convention asserted by a law test.
//!
//! 📏️ Span, section width and depth, lateral restraint spacing, bearing length, notch depth and distance and the fire duration for members; a₁ spacing, end and edge distance, fastener count, rows, diameter and layer thicknesses for connections.
//!
//! 🧱️ Typed entities: hole position and diameter, notch depth (through the mirrored effective depth), taper and apex geometry and the fastener pattern editor (a grip per fastener of the first row and column, count steppers, a₃ and a₄), all written to the entity fields in metres and radians.

use super::facts;
use super::kit::{along, base_id, entity_ref, loc, v3};
use super::layout::{is_upright, Layout, Placement};
use super::model::{CaseRow, En1995Snapshot, SupportType, TimberProduct};
use super::scene::*;
use super::scene::projection_kit::design_case::forms;
use super::scene::projection_kit::handle_kit::{grid, handle, HandleSpec as Spec, field_binding, round};
use super::schema::checks::timber::En1995Cases;

/// 🕹️ The numeric geometric fields of the typed tapers and cutouts with the grip tag (`entity:tag`, alternatives split by `|`) that drags each; the rod resistances are strengths, not geometry, and have no grip. A law test pins the registry to the model fields and to the grips the scene builds.
pub const TYPED_FIELD_GRIPS: &[(&str, &str, &str)] = &[
    ("tapers", "taperAngleRad", "alpha_deg"),
    ("tapers", "apexDepthM", "h_ap"),
    ("tapers", "innerRadiusM", "r_in"),
    ("tapers", "lamellaThicknessM", "t_lam"),
    ("tapers", "apexAngleRad", "alpha_ap_deg"),
    ("tapers", "rods", "n_rods"),
    ("cutouts", "distanceM", "x|hole_x"),
    ("cutouts", "lengthM", "notch_l|hole_d"),
    ("cutouts", "depthM", "h_ef"),
    ("cutouts", "centreHeightM", "hole_z"),
    ("cutouts", "slope", "incl"),
    ("cutouts", "rods", "n_rods"),
];

fn write(path: String, unit: &str, map: Affine, step: Option<f64>) -> FieldBinding {
    field_binding(path, map, unit, step.and_then(round))
}

fn member_handles(d: &En1995Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(m), Some(pose)) = (d.members.iter().find(|m| m.id == id), layout.member(id)) else { return Vec::new() };
    let entity = entity_ref(EntityKind::Member, id, "members");
    let f = &pose.frame;
    let spans = if m.support == SupportType::ContinuousTwoSpan { 2.0 } else { 1.0 };
    let glulam = facts::properties(m.strength_class.id()).is_some_and(|p| p.product == TimberProduct::Glulam);
    let depth_step = if glulam { 0.04 } else { 0.02 };
    let path = |leaf: &str| format!("members[id={id}].{leaf}");
    let mut out = vec![
        handle(Spec {
            entity: &entity,
            tag: "span",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&f.origin, &f.x, pose.length),
            grip: GripStyle::Arrow,
            writes: vec![write(path("spanM"), "m", Affine { base: 0.0, k: 1.0 / spans }, Some(0.01))],
            snap: grid(0.05),
            limits: (0.5, 40.0 * spans),
            name: loc(&format!("Span of {}", m.label_en), &format!("Stützweite von {}", m.label_de)),
        }),
        handle(Spec {
            entity: &entity,
            tag: "width",
            kind: HandleKind::FaceOffset { normal: f.y.clone() },
            anchor: along(&along(&f.origin, &f.x, pose.length / 2.0), &f.y, m.b_m / 2.0),
            grip: GripStyle::Diamond,
            writes: vec![write(path("bM"), "m", Affine { base: 0.0, k: 2.0 }, Some(0.02))],
            snap: grid(0.01),
            limits: (0.03, 0.4),
            name: loc(&format!("Width of {}", m.label_en), &format!("Breite von {}", m.label_de)),
        }),
        handle(Spec {
            entity: &entity,
            tag: "depth",
            kind: HandleKind::FaceOffset { normal: f.z.clone() },
            anchor: along(&along(&f.origin, &f.x, pose.length / 2.0), &f.z, m.h_m / 2.0),
            grip: GripStyle::Diamond,
            writes: vec![write(path("hM"), "m", Affine { base: 0.0, k: 2.0 }, Some(depth_step))],
            snap: grid(depth_step / 2.0),
            limits: (0.04, 1.2),
            name: loc(&format!("Depth of {}", m.label_en), &format!("Höhe von {}", m.label_de)),
        }),
        handle(Spec {
            entity: &entity,
            tag: "bearing",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&along(&f.origin, &f.x, m.bearing_length_m), &f.z, -m.h_m / 2.0),
            grip: GripStyle::Ring,
            writes: vec![write(path("bearingLengthM"), "m", Affine::IDENTITY, Some(0.005))],
            snap: grid(0.005),
            limits: (0.02, 1.0),
            name: loc(&format!("Bearing length of {}", m.label_en), &format!("Auflagerlänge von {}", m.label_de)),
        }),
    ];
    if m.lateral_restraint_spacing_m > 1e-6 && m.lateral_restraint_spacing_m < m.span_m && !is_upright(m) {
        out.push(handle(Spec {
            entity: &entity,
            tag: "restraint",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&along(&f.origin, &f.x, m.lateral_restraint_spacing_m), &f.z, m.h_m / 2.0),
            grip: GripStyle::Ring,
            writes: vec![write(path("lateralRestraintSpacingM"), "m", Affine::IDENTITY, Some(0.01))],
            snap: grid(0.05),
            limits: (0.1, m.span_m),
            name: loc(&format!("Lateral restraint spacing of {}", m.label_en), &format!("Abstand der seitlichen Halterungen von {}", m.label_de)),
        }));
    }
    if m.notch_depth_m > 1e-6 && !is_upright(m) {
        let reach = (m.bearing_length_m / 2.0 + m.notch_distance_m).max(0.02);
        out.push(handle(Spec {
            entity: &entity,
            tag: "notch-depth",
            kind: HandleKind::FaceOffset { normal: f.z.clone() },
            anchor: along(&along(&f.origin, &f.x, reach), &f.z, -m.h_m / 2.0 + m.notch_depth_m),
            grip: GripStyle::Diamond,
            writes: vec![write(path("notchDepthM"), "m", Affine { base: m.h_m / 2.0, k: 1.0 }, Some(0.005))],
            snap: grid(0.005),
            limits: (-m.h_m / 2.0 + 0.005, 0.0),
            name: loc(&format!("Notch depth of {}", m.label_en), &format!("Ausklinkungstiefe von {}", m.label_de)),
        }));
        out.push(handle(Spec {
            entity: &entity,
            tag: "notch-distance",
            kind: HandleKind::ScalarAlong { axis: f.x.clone() },
            anchor: along(&along(&f.origin, &f.x, reach), &f.z, -m.h_m / 2.0),
            grip: GripStyle::Arrow,
            writes: vec![write(path("notchDistanceM"), "m", Affine { base: -m.bearing_length_m / 2.0, k: 1.0 }, Some(0.005))],
            snap: grid(0.005),
            limits: (m.bearing_length_m / 2.0, 2.0),
            name: loc(&format!("Notch distance of {}", m.label_en), &format!("Ausklinkungsabstand von {}", m.label_de)),
        }));
    }
    if let Some(fire) = facts::properties(m.strength_class.id()).and_then(|p| facts::fire_of(d.annex, m, &p)) {
        out.push(handle(Spec {
            entity: &entity,
            tag: "fire-time",
            kind: HandleKind::FaceOffset { normal: f.z.clone() },
            anchor: along(&along(&f.origin, &f.x, pose.length / 2.0), &f.z, m.h_m / 2.0 - fire.d_char),
            grip: GripStyle::Diamond,
            writes: vec![write(path("fireDurationS"), "s", Affine { base: m.h_m / 2.0 / fire.beta_n, k: -1.0 / fire.beta_n }, Some(60.0))],
            snap: SnapRule::NONE,
            limits: (0.0, m.h_m / 2.0),
            name: loc(&format!("Fire duration of {}", m.label_en), &format!("Branddauer von {}", m.label_de)),
        }));
    }
    out
}

fn connection_handles(d: &En1995Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (Some(c), Some(o)) = (d.connections.iter().find(|c| c.id == id), layout.connection(id)) else { return Vec::new() };
    let entity = EntityRef::new(EntityKind::Group, id, format!("connections[id={id}]"));
    let g = facts::grid(c);
    let layers = facts::layers(c);
    let t1 = layers.first().map_or(0.0, |l| l.thickness);
    let top: f64 = layers.iter().map(|l| l.thickness).sum();
    let at = |x: f64, y: f64, z: f64| v3(o.x + x, o.y + y, o.z + z);
    let path = |leaf: &str| format!("connections[id={id}].{leaf}");
    let middle_leaf = if c.steel_plate { "steelPlateThicknessM" } else { "t2M" };
    let last_x = g.end + f64::from(g.per_row - 1) * g.a1;
    vec![
        handle(Spec { entity: &entity, tag: "spacing", kind: HandleKind::DimensionLine, anchor: at(g.end + g.a1 / 2.0, g.edge, top), grip: GripStyle::Arrow, writes: vec![write(path("spacingM"), "m", Affine::IDENTITY, Some(0.005))], snap: grid(0.005), limits: (0.01, 0.6), name: loc("Fastener spacing a₁", "Verbindungsmittelabstand a₁") }),
        handle(Spec { entity: &entity, tag: "end-distance", kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: at(g.end, g.edge, top), grip: GripStyle::Ring, writes: vec![write(path("endDistanceM"), "m", Affine::IDENTITY, Some(0.005))], snap: grid(0.005), limits: (0.01, 0.6), name: loc("End distance a₃", "Hirnholzabstand a₃") }),
        handle(Spec { entity: &entity, tag: "edge-distance", kind: HandleKind::ScalarAlong { axis: Vec3::Y }, anchor: at(last_x, g.edge, top), grip: GripStyle::Ring, writes: vec![write(path("edgeDistanceM"), "m", Affine::IDENTITY, Some(0.005))], snap: grid(0.005), limits: (0.01, 0.4), name: loc("Edge distance a₄", "Randabstand a₄") }),
        handle(Spec { entity: &entity, tag: "number", kind: HandleKind::CountStepper, anchor: at(g.length, g.width / 2.0, top), grip: GripStyle::Diamond, writes: vec![FieldBinding { path: path("number"), map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Int) }], snap: SnapRule::NONE, limits: (1.0, 60.0), name: loc("Number of fasteners", "Anzahl Verbindungsmittel") }),
        handle(Spec { entity: &entity, tag: "rows", kind: HandleKind::CountStepper, anchor: at(g.length / 2.0, g.width, top), grip: GripStyle::Diamond, writes: vec![FieldBinding { path: path("rows"), map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Int) }], snap: SnapRule::NONE, limits: (1.0, 8.0), name: loc("Number of rows", "Anzahl Reihen") }),
        handle(Spec { entity: &entity, tag: "diameter", kind: HandleKind::Radial { centre: at(last_x, g.edge, top) }, anchor: at(last_x, g.edge + c.diameter_m / 2.0, top), grip: GripStyle::Ring, writes: vec![write(path("diameterM"), "m", Affine { base: 0.0, k: 2.0 }, Some(0.001))], snap: grid(0.0005), limits: (0.001, 0.015), name: loc("Fastener diameter", "Verbindungsmitteldurchmesser") }),
        handle(Spec { entity: &entity, tag: "t1", kind: HandleKind::FaceOffset { normal: Vec3::Z }, anchor: at(g.length / 2.0, 0.0, t1), grip: GripStyle::Diamond, writes: vec![write(path("t1M"), "m", Affine::IDENTITY, Some(0.005))], snap: grid(0.005), limits: (0.01, 0.4), name: loc("Thickness t₁", "Dicke t₁") }),
        handle(Spec { entity: &entity, tag: "t2", kind: HandleKind::FaceOffset { normal: Vec3::Z }, anchor: at(g.length / 2.0, g.width, t1 + layers.get(1).map_or(0.0, |l| l.thickness)), grip: GripStyle::Diamond, writes: vec![write(path(middle_leaf), "m", Affine { base: -t1, k: 1.0 }, Some(0.001))], snap: grid(0.001), limits: (t1 + 0.002, t1 + 0.4), name: loc("Thickness t₂ or plate", "Dicke t₂ bzw. Blech") }),
        handle(Spec { entity: &entity, tag: "planes", kind: HandleKind::CountStepper, anchor: at(0.0, g.width / 2.0, top), grip: GripStyle::Diamond, writes: vec![FieldBinding { path: path("shearPlanes"), map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Int) }], snap: SnapRule::NONE, limits: (1.0, 2.0), name: loc("Shear planes", "Scherfugen") }),
    ]
}

fn case_handles(d: &En1995Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(c) = d.cases.iter().find(|c| c.id == id) else { return Vec::new() };
    let ext = |case: &CaseRow, size: forms::Size| entities::case_extras(d.annex, case, size);
    layout.case(id).map_or_else(Vec::new, |pose| forms::handles_with::<En1995Cases>(c, pose, "cases", &ext))
}

/// 🧱️ The grips of a typed entity: those of the case row it is drawn as, re-pointed from the case inputs to the entity (or member, connection) fields they stand for; inputs without a field of their own lose their grip.
fn entity_handles(d: &En1995Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let Some(o) = layout.entity(id) else { return Vec::new() };
    let tables = super::schema::checks::timber::ENTITY_TABLES;
    let Some((table, index)) = tables.iter().find_map(|table| super::schema::checks::timber::entity_ids(d, table).iter().position(|row| row == id).map(|index| (*table, index))) else { return Vec::new() };
    let Some((row, homes)) = super::schema::checks::timber::drawing_row(d, table, index) else { return Vec::new() };
    let entity = EntityRef::new(EntityKind::Group, id, format!("{table}[id={id}]"));
    let prefix = format!("cases[id={id}].numbers[id=");
    entities::row_handles(d.annex, &row, o, entity)
        .into_iter()
        .filter_map(|mut handle| {
            let key = handle.writes.first()?.path.strip_prefix(&prefix)?.split(']').next()?.to_string();
            let home = homes.iter().find(|home| home.key == key && home.scale != 0.0)?;
            for write in &mut handle.writes {
                let (base, k) = home.stored(write.map.base, write.map.k);
                write.path = home.path.clone();
                write.map = Affine { base, k };
                if let Some(unit) = home.unit() {
                    write.unit = unit.into();
                }
            }
            Some(handle)
        })
        .collect()
}

/// 🆔️ The ids of the handles of the typed entity `id` whose tag is one of `tags`.
pub(crate) fn tagged_entity(document: &En1995Snapshot, layout: &Layout, id: &str, tags: &[&str]) -> Vec<String> {
    entity_handles(document, layout, id).into_iter().map(|h| h.id).filter(|h| h.rsplit(':').next().is_some_and(|tag| tags.contains(&tag))).collect()
}

/// 🧲️ Translate grips of every placeable entity; a drag writes `placements[id=ENTITY].x|y`, and `PlacementCommit::of_writes` turns the first drag of an entity without a row into one insert (one undo step).
fn move_handles(d: &En1995Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let (entity, anchor) = if let Some(pose) = layout.member(id) {
        (entity_ref(EntityKind::Member, id, "members"), pose.frame.origin.clone())
    } else if let Some(origin) = layout.connection(id) {
        (EntityRef::new(EntityKind::Group, id, format!("connections[id={id}]")), origin.clone())
    } else if let Some(origin) = layout.entity(id) {
        (EntityRef::new(EntityKind::Group, id, format!("entities[id={id}]")), origin.clone())
    } else {
        return Vec::new();
    };
    let stored: Vec<_> = d.placements.iter().map(|p| super::scene::projection_kit::Placement { entity_id: p.id.clone(), origin: Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    let base_of = |leaf: &str| super::scene::projection_kit::grip_base(&stored, &anchor, id, leaf);
    [("move-x", Vec3::X, base_of("x"), "x", "Move along X", "Entlang X verschieben"), ("move-y", Vec3::Y, base_of("y"), "y", "Move along Y", "Entlang Y verschieben")]
        .into_iter()
        .map(|(tag, axis, base, leaf, en, de)| {
            handle(Spec {
                entity: &entity,
                tag,
                kind: HandleKind::Translate { axis },
                anchor: anchor.clone(),
                grip: GripStyle::Ring,
                writes: vec![write(format!("placements[id={id}].{leaf}"), "m", Affine { base, k: 1.0 }, Some(0.01))],
                snap: grid(0.05),
                limits: (base - 1.0e3, base + 1.0e3),
                name: loc(&format!("{en} ({id})"), &format!("{de} ({id})")),
            })
        })
        .collect()
}

/// 🕹️ The handles of one document entity, anchored in the given layout.
pub(crate) fn build(document: &En1995Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    let id = base_id(id);
    [member_handles(document, layout, id), connection_handles(document, layout, id), case_handles(document, layout, id), entity_handles(document, layout, id), move_handles(document, layout, id)].concat()
}

/// 🆔️ The ids of the handles of `id` whose tag is one of `tags`.
pub(crate) fn tagged(document: &En1995Snapshot, layout: &Layout, id: &str, tags: &[&str]) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).filter(|h| h.rsplit(':').next().is_some_and(|tag| tags.contains(&tag))).collect()
}

/// 🕹️ The handles of every selected entity (derived scene ids resolve to their document entity), in selection order, without duplicates.
pub fn handles(document: &En1995Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
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
