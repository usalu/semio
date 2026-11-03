//! 🔩️ Connections: the stacked timber and steel layers, the fastener grid of nails, screws, bolts or dowels with its end, edge and spacing zones a₁, a₃, a₄,
//! the governing Johansen failure mode, dimensions bound to the connection fields and a group that selects them as one verification subject.

use super::facts::{self, Grid};
use super::kit::*;
use super::layout::Layout;
use super::model::TimberConnection;
use super::schema::checks::timber as t;
use super::scene::*;
use super::{handles, norm, SceneInput};

pub(crate) const SPLITTING_TAG: &str = "splitting-plane";
pub(crate) const BLOCK_TAG: &str = "shear-block";
const ZONE_LIFT: f64 = 0.002;
const ZONE_OPACITY: f64 = 0.45;

fn fastener_kind(c: &TimberConnection, kind: &str) -> (FastenerKind, HeadShape, Option<f64>) {
    match kind {
        "nail" => (FastenerKind::Nail, HeadShape::None, None),
        "screw" => (FastenerKind::Screw, HeadShape::Countersunk, None),
        "bolt" => (FastenerKind::Bolt { class: if c.f_u_k > 450e6 { "8.8" } else { "4.6" }.into(), preload: None }, HeadShape::Hex, Some(0.001)),
        _ => (FastenerKind::Dowel, HeadShape::None, None),
    }
}

fn zone_appearance(binding: &Option<UtilisationBinding>) -> Appearance {
    let failing = binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail);
    Appearance {
        role: MaterialRole::Neutral,
        channel: if binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role },
        style: if failing { StyleState::Error } else { StyleState::Normal },
        opacity: ZONE_OPACITY,
        hatch: Some(if failing { Hatch::Diagonal } else { Hatch::Dots }),
        deformation: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn zone(c: &TimberConnection, tag: &str, name: LocalizedCopy, detail: LocalizedCopy, corners: [(f64, f64); 2], at: &dyn Fn(f64, f64, f64) -> Vec3, z: f64, binding: Option<UtilisationBinding>, handle_ids: Vec<String>, out: &mut Out) {
    let ring: Vec<Vec3> = [(corners[0].0, corners[0].1), (corners[1].0, corners[0].1), (corners[1].0, corners[1].1), (corners[0].0, corners[1].1)].iter().map(|(x, y)| at(*x, *y, z + ZONE_LIFT)).collect();
    out.push(
        Spec {
            kind: EntityKind::Surface,
            id: format!("{}:{tag}", c.id),
            collection: "connections",
            geometry: Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: Polygon3 { outer: Ring3 { points: ring.clone() }, holes: Vec::new() } } }),
            appearance: Some(zone_appearance(&binding)),
            role: MaterialRole::Neutral,
            binding,
            name,
            detail,
            layer: LAYER_ZONES,
            handles: handle_ids,
            deformation: None,
        },
        &ring,
    );
}

fn layers(c: &TimberConnection, o: &Vec3, g: &Grid, binding: &Option<UtilisationBinding>, out: &mut Out) -> (Vec<EntityRef>, f64) {
    let at = |x: f64, y: f64, z: f64| v3(o.x + x, o.y + y, o.z + z);
    let mut children = Vec::new();
    let mut z = 0.0;
    for (index, layer) in facts::layers(c).iter().enumerate() {
        let id = format!("{}:layer-{index}", c.id);
        let (name, detail) = if layer.steel {
            (loc(&format!("Steel plate of {}", c.label_en), &format!("Stahlblech von {}", c.label_de)), loc(&format!("steel plate {:.0} mm", layer.thickness * 1e3), &format!("Stahlblech {:.0} mm", layer.thickness * 1e3)))
        } else {
            (loc(&format!("Timber layer {} of {}", index + 1, c.label_en), &format!("Holzlage {} von {}", index + 1, c.label_de)), loc(&format!("{} {:.0} × {:.0} × {:.0} mm", c.strength_class.id(), g.length * 1e3, g.width * 1e3, layer.thickness * 1e3), &format!("{} {:.0} × {:.0} × {:.0} mm", c.strength_class.id(), g.length * 1e3, g.width * 1e3, layer.thickness * 1e3)))
        };
        let (kind, geometry, role) = if layer.steel {
            (EntityKind::Plate, Geometry::Plate(PlateGeometry { frame: Frame { origin: at(g.length / 2.0, g.width / 2.0, z), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, outline: rect(g.length, g.width), thickness: layer.thickness, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Steel)
        } else {
            (EntityKind::Solid, Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame { origin: at(g.length / 2.0, g.width / 2.0, z), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, size: Vec3::new(g.length, g.width, layer.thickness) } }), MaterialRole::Timber)
        };
        children.push(entity_ref(kind, &id, "connections"));
        out.push(
            Spec { kind, id, collection: "connections", geometry, appearance: None, role, binding: binding.clone(), name, detail, layer: LAYER_CONNECTIONS, handles: Vec::new(), deformation: None },
            &[at(0.0, 0.0, z), at(g.length, g.width, z + layer.thickness)],
        );
        z += layer.thickness;
    }
    (children, z)
}

const FORCE_FIELD: &str = "fastener-force";

/// 🔩️ The utilisation of every fastener of the grid (instance order: rows across, then the fasteners along the grain) interpolated from the `fastener-force` stations of the Johansen check.
fn fastener_force_field(input: &SceneInput, c: &TimberConnection, g: &Grid) -> Option<Vec<f64>> {
    let prefix = "en1995.8.2.";
    let check = input.report.checks.iter().find(|k| k.id.starts_with(prefix) && k.id.ends_with(&format!(".johansen.{}", c.id)) && k.station_field == FORCE_FIELD && !k.stations.is_empty())?;
    let across = |i: u32| if g.rows > 1 { f64::from(i) / f64::from(g.rows - 1) } else { 0.5 };
    let along = |j: u32| if g.per_row > 1 { f64::from(j) / f64::from(g.per_row - 1) } else { 0.5 };
    let points: Vec<[f64; 3]> = (0..g.rows).flat_map(|i| (0..g.per_row).map(move |j| [along(j), across(i), 0.0])).collect();
    Some(projection_kit::field_at_points(check, &points))
}

/// 🪓️ The splitting plane of the fastener field (EN 1995-1-1, 8.4): the vertical plane along the grain through the middle of the rows, where the force component perpendicular to the grain opens the timber; the split capacity `F_90,Rk = 14·b·√(h_e/(1 − h_e/h))` is in the detail.
fn splitting_plane(c: &TimberConnection, g: &Grid, top: f64, at: &dyn Fn(f64, f64, f64) -> Vec3, binding: Option<UtilisationBinding>, out: &mut Out) {
    let y = 0.5 * g.width;
    let ring: Vec<Vec3> = [(0.0, 0.0), (g.length, 0.0), (g.length, top), (0.0, top)].iter().map(|(x, z)| at(*x, y, *z)).collect();
    let he = g.edge + f64::from(g.rows - 1) * g.a2 / 2.0;
    let b = c.t1_m.max(1e-3);
    let rk = (g.width > he + 1e-9).then(|| t::splitting_f90_rk(b, 1.0, he.min(0.99 * g.width), g.width)).unwrap_or(0.0);
    out.push(
        Spec {
            kind: EntityKind::Surface,
            id: format!("{}:{SPLITTING_TAG}", c.id),
            collection: "connections",
            geometry: Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: Polygon3 { outer: Ring3 { points: ring.clone() }, holes: Vec::new() } } }),
            appearance: Some(Appearance { role: MaterialRole::Neutral, channel: ColourChannel::Role, style: StyleState::Normal, opacity: 0.4, hatch: Some(Hatch::Diagonal), deformation: None }),
            role: MaterialRole::Neutral,
            binding,
            name: loc("Splitting plane (8.4)", "Spaltebene (8.4)"),
            detail: loc(&format!("Plane along the grain through the row axis, h_e = {:.0} mm of h = {:.0} mm, F_90,Rk = {:.2} kN", he * 1e3, g.width * 1e3, rk / 1e3), &format!("Ebene in Faserrichtung durch die Reihenachse, h_e = {:.0} mm von h = {:.0} mm, F_90,Rk = {:.2} kN", he * 1e3, g.width * 1e3, rk / 1e3)),
            layer: LAYER_ZONES,
            handles: Vec::new(),
            deformation: None,
        },
        &ring,
    );
}

/// 🧱️ The row shear block of the fastener field (Annex A, block shear): the timber block between the end of the member, the first and last row and the whole thickness, which tears out along its two shear faces and its tension face.
fn shear_block(c: &TimberConnection, g: &Grid, top: f64, at: &dyn Fn(f64, f64, f64) -> Vec3, binding: Option<UtilisationBinding>, out: &mut Out) {
    let last_x = g.end + f64::from(g.per_row - 1) * g.a1;
    let last_y = g.edge + f64::from(g.rows - 1) * g.a2;
    let (size, centre) = (Vec3::new(last_x, (last_y - g.edge).max(1e-3), top), at(0.5 * last_x, 0.5 * (g.edge + last_y), 0.5 * top));
    let corners = [at(0.0, g.edge, 0.0), at(last_x, last_y, top)];
    let a_v = 2.0 * last_x * top;
    let a_t = (last_y - g.edge) * top;
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: format!("{}:{BLOCK_TAG}", c.id),
            collection: "connections",
            geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame { origin: centre, x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, size } }),
            appearance: Some(Appearance { role: MaterialRole::Neutral, channel: ColourChannel::Role, style: StyleState::Normal, opacity: 0.3, hatch: Some(Hatch::Dots), deformation: None }),
            role: MaterialRole::Neutral,
            binding,
            name: loc("Row shear block (Annex A)", "Reihen-Scherblock (Anhang A)"),
            detail: loc(&format!("Block {:.0} × {:.0} × {:.0} mm, shear area {:.3} m², tension area {:.3} m²", last_x * 1e3, (last_y - g.edge) * 1e3, top * 1e3, a_v, a_t), &format!("Block {:.0} × {:.0} × {:.0} mm, Scherfläche {:.3} m², Zugfläche {:.3} m²", last_x * 1e3, (last_y - g.edge) * 1e3, top * 1e3, a_v, a_t)),
            layer: LAYER_ZONES,
            handles: Vec::new(),
            deformation: None,
        },
        &corners,
    );
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for c in &d.connections {
        if !out.wants(&c.id) {
            continue;
        }
        let Some(o) = layout.connection(&c.id).cloned() else { continue };
        if c.diameter_m <= 1e-6 || c.number == 0 {
            continue;
        }
        let g = facts::grid(c);
        let at = |x: f64, y: f64, z: f64| v3(o.x + x, o.y + y, o.z + z);
        let binding = binding_of(input.report, &c.id, "");
        let (mut children, top) = layers(c, &o, &g, &binding, out);
        let first_x = g.end + f64::from(g.per_row - 1) * g.a1;
        let (from, to) = (at(first_x, g.edge, 0.0), at(first_x, g.edge, top));
        let (kind, head, clearance) = fastener_kind(c, g.kind);
        let johansen = facts::johansen(c);
        let fasteners_id = format!("{}:fasteners", c.id);
        let mode_detail = johansen.as_ref().map_or((String::new(), String::new()), |j| {
            let (mode, rk) = j.modes[j.governing];
            (format!(", governing Johansen mode {mode} with {:.2} kN per fastener and shear plane", rk / 1e3), format!(", maßgebende Johansen-Versagensform {mode} mit {:.2} kN je Verbindungsmittel und Scherfuge", rk / 1e3))
        });
        let force_field = fastener_force_field(input, c, &g);
        if force_field.is_some() {
            field_legend(&mut out.legends, FORCE_FIELD, loc("Force per fastener against its capacity", "Kraft je Verbindungsmittel gegen Tragfähigkeit"));
        }
        children.push(entity_ref(EntityKind::Fastener, &fasteners_id, "connections"));
        out.push(
            Spec {
                kind: EntityKind::Fastener,
                id: fasteners_id.clone(),
                collection: "connections",
                geometry: Geometry::Fastener(FastenerGeometry {
                    kind,
                    axis: Segment3 { from: from.clone(), to: to.clone() },
                    diameter: c.diameter_m,
                    head,
                    pattern: Some(Pattern::Grid { nx: g.rows, ny: g.per_row, sx: g.a2, sy: g.a1 }),
                    hole_clearance: clearance,
                }),
                appearance: force_field.as_ref().map(|values| Appearance {
                    role: MaterialRole::Bolt,
                    channel: ColourChannel::Fields { active: FORCE_FIELD.into(), fields: vec![ScalarField { id: FORCE_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Instance { values: values.clone() } }] },
                    style: if binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail) { StyleState::Error } else { StyleState::Normal },
                    opacity: 1.0,
                    hatch: None,
                    deformation: None,
                }),
                role: MaterialRole::Bolt,
                binding: binding.clone(),
                name: loc(&format!("Fasteners of {}", c.label_en), &format!("Verbindungsmittel von {}", c.label_de)),
                detail: loc(
                    &format!("{} × {} {} ⌀{:.1} mm, a₁ {:.0} mm, a₂ {:.0} mm{}", g.rows, g.per_row, facts::family_of(c.fastener_type), c.diameter_m * 1e3, g.a1 * 1e3, g.a2 * 1e3, mode_detail.0),
                    &format!("{} × {} {} ⌀{:.1} mm, a₁ {:.0} mm, a₂ {:.0} mm{}", g.rows, g.per_row, facts::family_of(c.fastener_type), c.diameter_m * 1e3, g.a1 * 1e3, g.a2 * 1e3, mode_detail.1),
                ),
                layer: LAYER_CONNECTIONS,
                handles: handles::tagged(d, layout, &c.id, &["number", "rows", "diameter"]),
                deformation: None,
            },
            &[from, to],
        );
        let tag = |t: &str| handles::tagged(d, layout, &c.id, &[t]);
        let z = |prefix: &str| binding_of(input.report, &c.id, prefix);
        let (last_y, last_x) = (g.edge + f64::from(g.rows - 1) * g.a2, first_x);
        zone(c, "zone-a1", loc("Spacing zone a₁, a₂", "Abstandszone a₁, a₂"), loc(&format!("fastener field, a₁ {:.0} mm", g.a1 * 1e3), &format!("Verbindungsmittelfeld, a₁ {:.0} mm", g.a1 * 1e3)), [(g.end, g.edge), (last_x, last_y)], &at, top, z("en1995.8.spacing.a1."), tag("spacing"), out);
        zone(c, "zone-a3", loc("End distance zone a₃", "Hirnholzzone a₃"), loc(&format!("end distance {:.0} mm", g.end * 1e3), &format!("Hirnholzabstand {:.0} mm", g.end * 1e3)), [(0.0, 0.0), (g.end, g.width)], &at, top, z("en1995.8.spacing.a3t."), tag("end-distance"), out);
        zone(c, "zone-a4", loc("Edge distance zone a₄", "Randzone a₄"), loc(&format!("edge distance {:.0} mm", g.edge * 1e3), &format!("Randabstand {:.0} mm", g.edge * 1e3)), [(g.end, 0.0), (g.length, g.edge)], &at, top, z("en1995.8.spacing.a4t."), tag("edge-distance"), out);
        zone(c, "zone-a4-far", loc("Opposite edge zone a₄", "Gegenüberliegende Randzone a₄"), loc(&format!("edge distance {:.0} mm", g.edge * 1e3), &format!("Randabstand {:.0} mm", g.edge * 1e3)), [(g.end, g.width - g.edge), (g.length, g.width)], &at, top, z("en1995.8.spacing.a4t."), Vec::new(), out);
        splitting_plane(c, &g, top, &at, binding.clone(), out);
        shear_block(c, &g, top, &at, binding.clone(), out);
        children.extend([SPLITTING_TAG, BLOCK_TAG].iter().map(|t| entity_ref(if *t == BLOCK_TAG { EntityKind::Solid } else { EntityKind::Surface }, &format!("{}:{t}", c.id), "connections")));
        children.extend(["zone-a1", "zone-a3", "zone-a4", "zone-a4-far"].iter().map(|t| entity_ref(EntityKind::Surface, &format!("{}:{t}", c.id), "connections")));
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: c.id.clone(),
                collection: "connections",
                geometry: Geometry::Group { members: children },
                appearance: None,
                role: MaterialRole::Bolt,
                binding: binding.clone(),
                name: loc(&c.label_en, &c.label_de),
                detail: loc(&format!("{} connection, {} shear plane(s){}", g.kind, c.shear_planes.max(1), mode_detail.0), &format!("{}-Verbindung, {} Scherfuge(n){}", g.kind, c.shear_planes.max(1), mode_detail.1)),
                layer: LAYER_CONNECTIONS,
                handles: handles::tagged(d, layout, &c.id, &["t1", "t2", "planes"]),
                deformation: None,
            },
            &[o.clone()],
        );
        annotations(input, c, &g, &johansen, &binding, &at, top, &fasteners_id, out);
    }
}

#[allow(clippy::too_many_arguments)]
fn annotations(input: &SceneInput, c: &TimberConnection, g: &Grid, johansen: &Option<facts::Johansen>, binding: &Option<UtilisationBinding>, at: &dyn Fn(f64, f64, f64) -> Vec3, top: f64, fasteners_id: &str, out: &mut Out) {
    let dimension = |id: &str, from: Vec3, to: Vec3, value: f64, leaf: &str| Annotation::Dimension {
        id: format!("{}:{id}", c.id),
        from,
        to,
        offset: Vec3::ZERO,
        value: Quantity::new(QuantityKind::Length, value),
        binding: Some(FieldBinding::new(format!("connections[id={}].{leaf}", c.id), Affine::IDENTITY, "m")),
    };
    let z = top + 0.05;
    let row = -0.06;
    let a1_from = at(g.end, row, z);
    out.annotate(dimension("a1", a1_from.clone(), at(g.end + g.a1, row, z), g.a1, "spacingM"), &a1_from);
    let end_from = at(0.0, row, z);
    out.annotate(dimension("a3", end_from.clone(), at(g.end, row, z), g.end, "endDistanceM"), &end_from);
    let edge_from = at(g.length + 0.06, 0.0, z);
    out.annotate(dimension("a4", edge_from.clone(), at(g.length + 0.06, g.edge, z), g.edge, "edgeDistanceM"), &edge_from);
    let label_at = at(g.length / 2.0, g.width + 0.1, z);
    let text = johansen.as_ref().map_or_else(
        || loc(&c.label_en, &c.label_de),
        |j| {
            let (mode, rk) = j.modes[j.governing];
            loc(&format!("{}: mode {mode}, F_v,Rk {:.2} kN", c.label_en, rk / 1e3), &format!("{}: Versagensform {mode}, F_v,Rk {:.2} kN", c.label_de, rk / 1e3))
        },
    );
    out.annotate(Annotation::Label { id: format!("{}:mode", c.id), anchor: label_at.clone(), text, tone: tone_of(binding) }, &label_at);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", c.id), anchor: entity_ref(EntityKind::Fastener, fasteners_id, "connections"), clause }, &label_at);
    }
    if worst(input.report, &c.id, "").is_some_and(|r| matches!(r.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning)) {
        out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", c.id), at: entity_ref(EntityKind::Fastener, fasteners_id, "connections"), mode: FailureMode::Other, extent: None }, &label_at);
    }
}
