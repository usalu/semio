//! 🔗️ Connections: the connected plate, the bolt or rivet grid, the fillet weld with its HAZ extent, and the group that selects them as one verification subject.
//!
//! 🔩️ Pattern convention: for a fastener axis along the member axis the renderer lays `nx` along world `Z` and `ny` across, so `nx` carries the bolt rows (pitch) and `ny` the bolts per row (gauge).

use super::checks::part_1_1;
use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::{handles, SceneInput};

const HAZ_SKIN: f64 = 0.0005;

fn rect(width: f64, height: f64) -> Polygon2 {
    let (w, h) = (width / 2.0, height / 2.0);
    Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)])
}

fn mm(value: f64) -> String {
    format!("{:.1} mm", value * 1e3)
}

/// 🔩️ The in-plane force of every fastener of the bolt group (rows outermost, as the grid is drawn) scaled so that the governing one equals the F_v,Ed of the reported shear check, with the index of the governing fastener; `None` without that check. The elastic distribution is [`joints::group_forces`] of the checks.
pub fn fastener_forces(report: &super::CheckReport, conn: &super::model::AluminiumConnection) -> Option<(Vec<super::checks::joints::FastenerForce>, usize)> {
    use super::checks::joints;
    let b = &conn.bolts;
    let check = report.checks.iter().find(|c| c.id == format!("en1999.8.5.bolt.{}", conn.id))?;
    let unit = joints::fastener_forces(conn.force_method, b.rows, b.bolts_per_row, b.pitch, b.gauge, 1.0, conn.load_eccentricity);
    let governing = joints::governing_fastener(&unit);
    let top = unit.get(governing)?.resultant();
    (top > 0.0 && check.computed.value.is_finite()).then(|| {
        let scale = check.computed.value / top;
        (unit.into_iter().map(|f| joints::FastenerForce { x: f.x, y: f.y, fx: f.fx * scale, fy: f.fy * scale }).collect(), governing)
    })
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for conn in &d.connections {
        if !out.own("connections", &conn.id) {
            continue;
        }
        let (Some(pose), Some(member)) = (layout.member(&conn.member_id), d.members.iter().find(|m| m.id == conn.member_id)) else { continue };
        let section = d.sections.iter().find(|s| s.id == member.section_id);
        let (depth, width) = section.map_or((0.2, 0.2), |s| (section_depth(s), section_width(s)));
        let end = along(&pose.frame.origin, &pose.frame.x, pose.length);
        let b = &conn.bolts;
        let w = &conn.welds;
        let welded = matches!(conn.kind, semio_s_artifact_norm_en1999_model::ConnectionKind::Welded | semio_s_artifact_norm_en1999_model::ConnectionKind::Combined) && w.throat > 1e-6 && w.length > 1e-6;
        let bolted = conn.kind != semio_s_artifact_norm_en1999_model::ConnectionKind::Welded && b.diameter > 1e-6 && b.rows > 0 && b.bolts_per_row > 0;
        let thickness = b.plate_thickness.max(0.004);
        let across = b.gauge * (b.bolts_per_row.max(1) - 1) as f64 + 2.0 * b.edge_distance;
        let down = b.pitch * (b.rows.max(1) - 1) as f64 + 2.0 * b.edge_distance;
        let (plate_w, plate_h) = if bolted && !welded { (across.max(0.05), down.max(0.05)) } else if bolted { (across.max(width), down.max(depth)) } else { (width, depth) };
        let binding = out.binding(input, &[conn.id.as_str()]);
        let plate_frame = Frame { origin: end.clone(), x: pose.frame.y.clone(), y: pose.frame.z.clone(), z: pose.frame.x.clone() };
        let plate_id = format!("{}:plate", conn.id);
        let bolts_id = format!("{}:bolts", conn.id);
        let weld_id = format!("{}:weld", conn.id);
        let haz_id = format!("{}:haz", conn.id);
        let mut children = vec![entity_ref(EntityKind::Plate, &plate_id, "connections")];
        let centre = along(&end, &plate_frame.z, thickness / 2.0);
        let rim = [along(&centre, &plate_frame.x, plate_w / 2.0), along(&centre, &plate_frame.x, -plate_w / 2.0), along(&centre, &plate_frame.y, plate_h / 2.0), along(&centre, &plate_frame.y, -plate_h / 2.0)];
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: plate_id,
                collection: "connections",
                geometry: Geometry::Plate(PlateGeometry { frame: plate_frame.clone(), outline: rect(plate_w, plate_h), thickness, offset: 0.0, stiffeners: Vec::new() }),
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                name: loc(&format!("Plate of {}", conn.id), &format!("Anschlussblech {}", conn.id)),
                detail: loc(&format!("{:.0} × {:.0} × {:.0} mm", plate_w * 1e3, plate_h * 1e3, thickness * 1e3), &format!("{:.0} × {:.0} × {:.0} mm", plate_w * 1e3, plate_h * 1e3, thickness * 1e3)),
                layer: LAYER_CONNECTIONS,
                handles: Vec::new(),
                deformation: None,
                style: None,
            },
            &rim,
        );
        if bolted {
            let net = super::checks::joints::net_width(across, super::checks::joints::hole_diameter(b.diameter), b.bolts_per_row, b.gauge, conn.hole_stagger);
            let from = along(&along(&centre, &plate_frame.x, -net / 2.0), &plate_frame.y, plate_h / 2.0 + 0.05);
            out.annotate(Annotation::Dimension { id: format!("{}:net-width", conn.id), from: from.clone(), to: along(&from, &plate_frame.x, net), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, net), binding: None }, &from);
        }
        if welded {
            let from = along(&along(&end, &pose.frame.z, -depth / 2.0), &pose.frame.y, -w.length / 2.0);
            let to = along(&from, &pose.frame.y, w.length);
            children.push(entity_ref(EntityKind::Fastener, &weld_id, "connections"));
            out.push(
                Spec {
                    kind: EntityKind::Fastener,
                    id: weld_id,
                    collection: "connections",
                    geometry: Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Weld { throat: w.throat, weld: WeldKind::Fillet }, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: w.throat, head: HeadShape::None, pattern: None, hole_clearance: None }),
                    role: MaterialRole::Weld,
                    binding: binding.clone(),
                    name: loc(&format!("Weld of {}", conn.id), &format!("Schweißnaht {}", conn.id)),
                    detail: loc(&format!("fillet a = {}, length {:.0} mm, filler {}", mm(w.throat), w.length * 1e3, w.filler_alloy), &format!("Kehlnaht a = {}, Länge {:.0} mm, Zusatz {}", mm(w.throat), w.length * 1e3, w.filler_alloy)),
                    layer: LAYER_CONNECTIONS,
                    handles: Vec::new(),
                    deformation: None,
                    style: None,
                },
                &[from.clone(), to],
            );
            let parent_t = b.plate_thickness.max(w.throat / 0.7);
            let b_haz = if w.haz_extent > 0.0 { w.haz_extent } else { part_1_1::haz_width(parent_t) };
            let alloy = d.materials.iter().find(|m| m.id == conn.material_id).and_then(|m| part_1_1::resolve_alloy_for_thickness(&m.designation, parent_t));
            let start = along(&end, &pose.frame.x, -b_haz);
            let outline = rect(width + 2.0 * HAZ_SKIN, depth + 2.0 * HAZ_SKIN);
            children.push(entity_ref(EntityKind::Member, &haz_id, "connections"));
            out.push(
                Spec {
                    kind: EntityKind::Member,
                    id: haz_id,
                    collection: "connections",
                    geometry: Geometry::Member(MemberGeometry {
                        axis: polyline(&[start.clone(), end.clone()]),
                        frame: Frame { origin: start.clone(), ..pose.frame.clone() },
                        section: SectionShape::Polygon { polygon: outline },
                        section_taper: None,
                        start_offset: Vec2::ZERO,
                        end_offset: Vec2::ZERO,
                        start_release: EndRelease::Rigid,
                        end_release: EndRelease::Rigid,
                        stations: Vec::new(),
                    }),
                    role: MaterialRole::Weld,
                    binding: None,
                    name: loc(&format!("Heat-affected zone of {}", conn.id), &format!("Wärmeeinflusszone von {}", conn.id)),
                    detail: loc(
                        &format!("b_haz = {}{}", mm(b_haz), alloy.map_or(String::new(), |a| format!(", ρ_o,haz = {:.2}, ρ_u,haz = {:.2}", a.rho_o_haz, a.rho_u_haz))),
                        &format!("b_haz = {}{}", mm(b_haz), alloy.map_or(String::new(), |a| format!(", ρ_o,haz = {:.2}, ρ_u,haz = {:.2}", a.rho_o_haz, a.rho_u_haz))),
                    ),
                    layer: LAYER_HAZ,
                    handles: Vec::new(),
                    deformation: None,
                    style: Some((StyleState::Normal, 0.55, Some(Hatch::Dots))),
                },
                &[start.clone(), end.clone()],
            );
            let dim_from = along(&along(&start, &pose.frame.z, depth / 2.0 + 0.1), &pose.frame.y, 0.0);
            out.annotate(
                Annotation::Dimension {
                    id: format!("{}:haz-extent", conn.id),
                    from: dim_from.clone(),
                    to: along(&dim_from, &pose.frame.x, b_haz),
                    offset: Vec3::ZERO,
                    value: Quantity::new(QuantityKind::Length, b_haz),
                    binding: Some(FieldBinding::new(format!("connections[id={}].welds.hazExtent", conn.id), Affine::IDENTITY, "m")),
                },
                &dim_from,
            );
        }
        if bolted {
            let rivet = super::checks::joints::is_rivet(&b.material);
            let d_bolt = b.diameter;
            let from = along(&end, &pose.frame.x, thickness);
            let to = along(&end, &pose.frame.x, -0.6 * d_bolt);
            children.push(entity_ref(EntityKind::Fastener, &bolts_id, "connections"));
            out.push(
                Spec {
                    kind: EntityKind::Fastener,
                    id: bolts_id,
                    collection: "connections",
                    geometry: Geometry::Fastener(FastenerGeometry {
                        kind: if rivet { FastenerKind::Dowel } else { FastenerKind::Bolt { class: b.material.clone(), preload: None } },
                        axis: Segment3 { from: from.clone(), to: to.clone() },
                        diameter: d_bolt,
                        head: if rivet { HeadShape::Round } else { HeadShape::Hex },
                        pattern: Some(Pattern::Grid { nx: b.rows.max(1), ny: b.bolts_per_row.max(1), sx: b.pitch.max(1e-3), sy: b.gauge.max(1e-3) }),
                        hole_clearance: Some(super::checks::joints::hole_diameter(d_bolt) - d_bolt),
                    }),
                    role: MaterialRole::Bolt,
                    binding: binding.clone(),
                    name: loc(&format!("{} of {}", if rivet { "Rivets" } else { "Bolts" }, conn.id), &format!("{} {}", if rivet { "Niete" } else { "Schrauben" }, conn.id)),
                    detail: loc(
                        &format!("{} × {} {}{:.0} grade {}, p1 = {}, p2 = {}, e = {}", b.rows, b.bolts_per_row, if rivet { "rivet Ø" } else { "M" }, d_bolt * 1e3, b.material, mm(b.pitch), mm(b.gauge), mm(b.edge_distance)),
                        &format!("{} × {} {}{:.0} Güte {}, p1 = {}, p2 = {}, e = {}", b.rows, b.bolts_per_row, if rivet { "Niet Ø" } else { "M" }, d_bolt * 1e3, b.material, mm(b.pitch), mm(b.gauge), mm(b.edge_distance)),
                    ),
                    layer: LAYER_CONNECTIONS,
                    handles: Vec::new(),
                    deformation: None,
                    style: None,
                },
                &[from, to],
            );
            if let Some((forces, governing)) = fastener_forces(input.report, conn) {
                let per_row = b.bolts_per_row.max(1) as usize;
                let top = forces[governing].resultant();
                let case = LoadCaseRef { id: format!("{}:fastener-forces", conn.id), kind: LoadKind::Other, psi: None };
                let spread = forces.iter().any(|f| (f.resultant() - top).abs() > 1e-9 * top);
                for (k, f) in forces.iter().enumerate() {
                    let (row, line) = (k / per_row, k % per_row);
                    let foot = along(&along(&end, &pose.frame.z, f.x), &pose.frame.y, f.y);
                    let head = along(&foot, &pose.frame.x, thickness);
                    let magnitude = f.resultant();
                    let direction = along(&along(&Vec3::ZERO, &pose.frame.z, -f.fx / magnitude.max(1e-12)), &pose.frame.y, -f.fy / magnitude.max(1e-12));
                    let text = format!("F_v,Ed {:.2} kN (bolt {}/{})", magnitude / 1e3, row, line);
                    out.push(
                        Spec {
                            kind: EntityKind::Load,
                            id: format!("{}:fastener-force:{row}-{line}", conn.id),
                            collection: "connections",
                            geometry: Geometry::Load(LoadGlyph::Point { at: head.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, magnitude), case: case.clone() }),
                            role: MaterialRole::Load { kind: LoadKind::Other },
                            binding: None,
                            name: loc(&text, &text),
                            detail: loc(&format!("{text}, share of the shear by the {} with the lever arm e = {}", super::checks::joints::force_method_label(conn.force_method).0, mm(conn.load_eccentricity)), &format!("{text}, Anteil der Querkraft nach dem {} mit dem Hebelarm e = {}", super::checks::joints::force_method_label(conn.force_method).1, mm(conn.load_eccentricity))),
                            layer: LAYER_LOADS,
                            handles: Vec::new(),
                            deformation: None,
                            style: None,
                        },
                        &[head],
                    );
                }
                if spread {
                    let f = &forces[governing];
                    let foot = along(&along(&end, &pose.frame.z, f.x), &pose.frame.y, f.y);
                    let (from, to) = (along(&foot, &pose.frame.x, thickness), along(&foot, &pose.frame.x, -0.6 * d_bolt));
                    out.push(
                        Spec {
                            kind: EntityKind::Fastener,
                            id: format!("{}:fastener-governing", conn.id),
                            collection: "connections",
                            geometry: Geometry::Fastener(FastenerGeometry {
                                kind: if rivet { FastenerKind::Dowel } else { FastenerKind::Bolt { class: b.material.clone(), preload: None } },
                                axis: Segment3 { from: from.clone(), to: to.clone() },
                                diameter: 1.25 * d_bolt,
                                head: if rivet { HeadShape::Round } else { HeadShape::Hex },
                                pattern: None,
                                hole_clearance: None,
                            }),
                            role: MaterialRole::Bolt,
                            binding: binding.clone(),
                            name: loc(&format!("Governing fastener of {}", conn.id), &format!("Maßgebendes Verbindungsmittel von {}", conn.id)),
                            detail: loc(&format!("F_v,Ed = {:.2} kN at e = {}", top / 1e3, mm(conn.load_eccentricity)), &format!("F_v,Ed = {:.2} kN bei e = {}", top / 1e3, mm(conn.load_eccentricity))),
                            layer: LAYER_CONNECTIONS,
                            handles: Vec::new(),
                            deformation: None,
                            style: Some((StyleState::Selected, 1.0, Some(Hatch::Diagonal))),
                        },
                        &[from, to],
                    );
                }
            }
        }
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: conn.id.clone(),
                collection: "connections",
                geometry: Geometry::Group { members: children },
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                name: loc(&format!("Connection {}", conn.id), &format!("Anschluss {}", conn.id)),
                detail: loc(&format!("{:?} connection on {}", conn.kind, conn.member_id), &format!("{:?}-Anschluss an {}", conn.kind, conn.member_id)),
                layer: LAYER_CONNECTIONS,
                handles: handles::ids(d, layout, &conn.id),
                deformation: None,
                style: None,
            },
            &[end.clone()],
        );
        super::members::action_glyphs(&conn.id, "connections", &conn.actions, &pose.frame, pose.length, depth, out);
    }
}
