//! 🧱️ Walls: the wall as three utilisation zones (top, middle, bottom) cut by its openings, window and door plates, concrete lintels, the floor bearing on the top, the base and side supports, dimensions and failure glyphs.

use super::kit::*;
use super::layout::{openings, solid_rects, Layout, WallPose, EPS};
use super::model::MasonryWall;
use super::scene::*;
use super::{handles, norm, profile, reduction, SceneInput};

pub(crate) const PHI_FIELD: &str = "wall-phi";
pub(crate) const STRESS_FIELD: &str = "wall-stress";
const SLAB_THICKNESS: f64 = 0.2;
const SLAB_STUB_MAX: f64 = 3.0;
const PLATE_THICKNESS: f64 = 0.03;
pub(crate) const ZONES: [(&str, usize, &str, &str); 3] = [("zone-bottom", 2, "Bottom zone", "Unterer Bereich"), ("zone-mid", 1, "Middle zone", "Mittlerer Bereich"), ("zone-top", 0, "Top zone", "Oberer Bereich")];

pub(crate) fn compression_id(wall: &str, load_case: &str, station: usize) -> String {
    if station == 1 { format!("en1996.6.1.2.compression.mid.{wall}.{load_case}") } else { format!("en1996.6.1.2.compression.{wall}.{load_case}") }
}

fn frame_of(pose: &WallPose, origin: Vec3) -> Frame {
    Frame { origin, x: pose.axis_x(), y: pose.axis_y(), z: Vec3::Z }
}

fn upright_frame(pose: &WallPose, origin: Vec3) -> Frame {
    let (x, y) = (pose.axis_x(), Vec3::Z);
    let z = x.cross(&y);
    Frame { origin, x, y, z }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        if wall.length_m <= EPS || wall.height_m <= EPS || wall.thickness_m <= EPS {
            continue;
        }
        project_wall(input, layout, wall, pose, out);
    }
}

/// 🕳️ The solid rectangles `(x0, x1, z0, z1, depth)` of the wall with every chase and recess cut into its face: the part under a cut keeps the thickness behind the depth, overlapping cuts keep the deeper one.
fn pockets(rects: Vec<(f64, f64, f64, f64)>, cuts: &[(f64, f64, f64, f64, f64)]) -> Vec<(f64, f64, f64, f64, f64)> {
    let mut pieces: Vec<_> = rects.into_iter().map(|(x0, x1, a, b)| (x0, x1, a, b, 0.0)).collect();
    for &(c0, c1, d0, d1, depth) in cuts {
        pieces = pieces
            .into_iter()
            .flat_map(|(x0, x1, a, b, d)| {
                let (i0, i1, j0, j1) = (x0.max(c0), x1.min(c1), a.max(d0), b.min(d1));
                if i1 - i0 <= EPS || j1 - j0 <= EPS {
                    return vec![(x0, x1, a, b, d)];
                }
                [(x0, i0, a, b, d), (i1, x1, a, b, d), (i0, i1, a, j0, d), (i0, i1, j1, b, d), (i0, i1, j0, j1, d.max(depth))].into_iter().filter(|p| p.1 - p.0 > EPS && p.3 - p.2 > EPS).collect()
            })
            .collect();
    }
    pieces
}

fn project_wall(input: &SceneInput, layout: &Layout, wall: &MasonryWall, pose: &WallPose, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let doc = input.document;
    let prof = profile::governing(doc, wall);
    let phi_check = prof.as_ref().and_then(|p| input.report.checks.iter().find(|c| c.id == compression_id(&wall.id, &p.load_case, 0) && c.station_field == PHI_FIELD && !c.stations.is_empty()));
    let wall_handles = handles::ids(doc, layout, &wall.id);
    let mut members: Vec<EntityRef> = Vec::new();
    let cuts: Vec<(f64, f64, f64, f64, f64)> = doc
        .chases
        .iter()
        .filter(|c| c.wall_id == wall.id)
        .map(|c| {
            let (x0, x1, y0, y1) = super::masonry::chase_extent(c);
            (x0, x1, y0, y1, c.depth_m.clamp(0.0, t))
        })
        .collect();
    for (tag, station, en, de) in ZONES {
        let (z0, z1) = match station {
            2 => (0.0, h / 3.0),
            1 => (h / 3.0, 2.0 * h / 3.0),
            _ => (2.0 * h / 3.0, h),
        };
        let mut mesh = MeshBuilder::default();
        for (x0, x1, a, b, depth) in pockets(solid_rects(wall, z0, z1), &cuts) {
            if t - depth > EPS {
                mesh.cuboid(pose, (x0, x1), (depth, t), (a, b));
            }
        }
        if mesh.is_empty() {
            continue;
        }
        let points = mesh.points().to_vec();
        let id = format!("{}:{tag}", wall.id);
        let section = [1.0, 0.5, 0.0][station];
        let binding = prof.as_ref().map(|p| computed_binding(compression_id(&wall.id, &p.load_case, station), phi_check.map_or(p.utilisation[station], |c| projection_kit::field_along(c, &[section])[0])));
        let mut spec = Spec::new(EntityKind::Solid, id.clone(), mesh.solid(), MaterialRole::Masonry, LAYER_MASONRY, loc(&format!("{} – {en}", wall.label_en), &format!("{} – {de}", wall.label_de)));
        let base = pose.at(0.0, 0.0, 0.0).z;
        let heights: Vec<f64> = points.iter().map(|p| ((p.z - base) / h).clamp(0.0, 1.0)).collect();
        let mut fields: Vec<ScalarField> = Vec::new();
        if let Some(c) = phi_check {
            fields.push(ScalarField { id: PHI_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: projection_kit::field_along(c, &heights) } });
            field_legend(&mut out.legends, PHI_FIELD, loc("Compression utilisation over the wall height", "Druckausnutzung über der Wandhöhe"));
        }
        if let Some(p) = prof.as_ref() {
            let (axis, origin) = (pose.axis_y(), &pose.origin);
            let values = points.iter().zip(&heights).map(|(v, u)| reduction::stress_ratio(p, t, *u, (v.x - origin.x) * axis.x + (v.y - origin.y) * axis.y - 0.5 * t)).collect();
            fields.push(ScalarField { id: STRESS_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } });
            field_legend(&mut out.legends, STRESS_FIELD, loc("Vertical stress σ / f_d under the eccentric load (e_mk)", "Vertikale Spannung σ / f_d unter der ausmittigen Last (e_mk)"));
        }
        if let Some(active) = fields.first().map(|f| f.id.clone()) {
            spec.channel = Some(ColourChannel::Fields { active, fields });
        }
        spec.detail = loc(&format!("{en}, {:.2} m long, {:.3} m thick", l, t), &format!("{de}, {:.2} m lang, {:.3} m dick", l, t));
        spec.binding = binding;
        spec.handles = wall_handles.clone();
        members.push(entity_ref(EntityKind::Solid, &id, &format!("walls[id={}]", wall.id)));
        out.push(spec, &points);
    }
    for o in openings(wall) {
        let nested = format!(".openings[id={}]", o.id);
        let window = o.z0 > EPS;
        let opening_handles = handles::ids(doc, layout, &format!("{}:{OPENING_TAG}{}", wall.id, o.id));
        let origin = pose.at(o.x0, 0.5 * t, o.z0);
        let geometry = Geometry::Plate(PlateGeometry { frame: upright_frame(pose, origin.clone()), outline: polygon2(&[(0.0, 0.0), (o.x1 - o.x0, 0.0), (o.x1 - o.x0, o.z1 - o.z0), (0.0, o.z1 - o.z0)]), thickness: PLATE_THICKNESS, offset: 0.0, stiffeners: Vec::new() });
        let id = format!("{}:{OPENING_TAG}{}", wall.id, o.id);
        let name = loc(if window { "Window" } else { "Door" }, if window { "Fenster" } else { "Tür" });
        let mut spec = Spec::new(EntityKind::Plate, id.clone(), geometry, if window { MaterialRole::Glass } else { MaterialRole::Neutral }, LAYER_OPENINGS, name);
        spec.nested = nested.clone();
        spec.detail = loc(&format!("{:.2} m wide, {:.2} m high, sill {:.2} m", o.x1 - o.x0, o.z1 - o.z0, o.z0), &format!("{:.2} m breit, {:.2} m hoch, Brüstung {:.2} m", o.x1 - o.x0, o.z1 - o.z0, o.z0));
        spec.binding = binding_of(input.report, &format!("en1996.5.4.opening.{}.{}", wall.id, o.id));
        spec.opacity = 0.5;
        spec.handles = opening_handles.clone();
        members.push(entity_ref(EntityKind::Plate, &id, &format!("walls[id={}]{nested}", wall.id)));
        out.push(spec, &[origin, pose.at(o.x1, 0.5 * t, o.z1)]);
    }
    slab(input, layout, wall, pose, prof.as_ref(), &mut members, out);
    supports(wall, pose, out);
    let binding = binding(input.report, &[&wall.id]);
    let mut group = Spec::new(EntityKind::Group, wall.id.clone(), Geometry::Group { members }, MaterialRole::Masonry, LAYER_MASONRY, loc(&wall.label_en, &wall.label_de));
    let slender = |e: f64| format!("h_ef = {:.2} m, t_ef = {:.3} m, h_ef/t_ef = {:.1}, e_mk = {:.1} mm", super::masonry::h_ef_m(wall), super::masonry::t_ef_m(wall), super::masonry::h_ef_m(wall) / super::masonry::t_ef_m(wall).max(1e-9), e * 1000.0);
    let e_mk = prof.as_ref().map_or(0.0, |p| p.e_mk);
    group.detail = loc(&format!("{:.2} m × {:.2} m × {:.3} m, {} sides held, {} openings; {}", l, h, t, wall.support_sides, wall.openings.len(), slender(e_mk)), &format!("{:.2} m × {:.2} m × {:.3} m, {} Seiten gehalten, {} Öffnungen; {}", l, h, t, wall.support_sides, wall.openings.len(), slender(e_mk)));
    group.binding = binding.clone();
    group.handles = wall_handles;
    out.push(group, &[pose.at(0.0, 0.0, 0.0), pose.at(l, t, h)]);
    annotations(input, wall, pose, &binding, out);
}

fn slab(input: &SceneInput, layout: &Layout, wall: &MasonryWall, pose: &WallPose, prof: Option<&profile::Profile>, members: &mut Vec<EntityRef>, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let a = wall.slab_bearing_depth_m.clamp(0.0, t);
    if a <= EPS {
        return;
    }
    let span = wall.load_cases.first().map_or(0.0, |lc| lc.slab_span_m.clamp(0.0, 6.0));
    let stub = (0.5 * span).clamp(0.0, SLAB_STUB_MAX);
    let origin = pose.at(0.0, t - a, h + 0.5 * SLAB_THICKNESS);
    let depth = a + stub.max(0.1);
    let geometry = Geometry::Plate(PlateGeometry { frame: Frame { origin: origin.clone(), x: pose.axis_x(), y: pose.axis_y(), z: Vec3::Z }, outline: polygon2(&[(0.0, 0.0), (l, 0.0), (l, depth), (0.0, depth)]), thickness: SLAB_THICKNESS, offset: 0.0, stiffeners: Vec::new() });
    let id = format!("{}:slab-top", wall.id);
    let mut spec = Spec::new(EntityKind::Plate, id.clone(), geometry, MaterialRole::Concrete, LAYER_FLOORS, loc("Floor bearing on the wall", "Deckenauflager auf der Wand"));
    spec.detail = loc(&format!("Floor bearing {:.0} mm deep, eccentricity {:.0} mm from the wall axis", a * 1000.0, (0.5 * (t - a)).abs() * 1000.0), &format!("Deckenauflager {:.0} mm tief, Ausmitte {:.0} mm zur Wandachse", a * 1000.0, (0.5 * (t - a)).abs() * 1000.0));
    spec.binding = prof.and_then(|p| binding_of(input.report, &format!("en1996.6.1.2.eccentricity.{}.{}", wall.id, p.load_case)));
    spec.handles = handles::ids(input.document, layout, &id);
    members.push(entity_ref(EntityKind::Plate, &id, &format!("walls[id={}]", wall.id)));
    out.push(spec, &[origin, pose.at(l, t - a + depth, h + SLAB_THICKNESS)]);
}

fn support(wall: &str, tag: &str, at: Vec3, pose: &WallPose, kind: SupportKind, name: LocalizedCopy, out: &mut Out) {
    let mut spec = Spec::new(EntityKind::Support, format!("{wall}:{tag}"), Geometry::Support(SupportGeometry { kind, frame: frame_of(pose, at.clone()), at: at.clone() }), MaterialRole::Support, LAYER_SUPPORTS, name);
    spec.detail = spec.name.clone();
    out.push(spec, &[at]);
}

fn supports(wall: &MasonryWall, pose: &WallPose, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    support(&wall.id, "support-base", pose.at(0.5 * l, 0.5 * t, 0.0), pose, SupportKind::Pinned, loc("Base bearing", "Fußlager"), out);
    if wall.support_sides >= 3 {
        support(&wall.id, "support-left", pose.at(0.0, 0.5 * t, 0.5 * h), pose, SupportKind::Roller { normal: pose.axis_x() }, loc("Held vertical edge, left", "Gehaltener Rand links"), out);
    }
    if wall.support_sides >= 4 {
        support(&wall.id, "support-right", pose.at(l, 0.5 * t, 0.5 * h), pose, SupportKind::Roller { normal: pose.axis_x() }, loc("Held vertical edge, right", "Gehaltener Rand rechts"), out);
    }
}

fn failure_mode(check_id: &str) -> FailureMode {
    if check_id.contains("shear") {
        FailureMode::Sliding
    } else if check_id.contains("slenderness") {
        FailureMode::FlexuralBuckling
    } else if check_id.contains("compression") || check_id.contains("concentrated") || check_id.contains(".area.") {
        FailureMode::Crushing
    } else {
        FailureMode::Other
    }
}

fn annotations(input: &SceneInput, wall: &MasonryWall, pose: &WallPose, binding: &Option<UtilisationBinding>, out: &mut Out) {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let id = &wall.id;
    let path = |field: &str| format!("walls[id={id}].{field}");
    let bound = |field: &str| Some(FieldBinding::new(path(field), Affine::IDENTITY, "m"));
    let dim = |name: &str, from: Vec3, to: Vec3, value: f64, field: &str| Annotation::Dimension { id: format!("{id}:{name}"), from, to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, value), binding: bound(field) };
    let below = pose.at(0.0, -0.3, -0.25);
    out.annotate(dim("length", below.clone(), pose.at(l, -0.3, -0.25), l, "lengthM"), &below);
    let side = pose.at(-0.3, -0.3, 0.0);
    out.annotate(dim("height", side.clone(), pose.at(-0.3, -0.3, h), h, "heightM"), &side);
    let edge = pose.at(l + 0.25, 0.0, h);
    out.annotate(dim("thickness", edge.clone(), pose.at(l + 0.25, t, h), t, "thicknessM"), &edge);
    let label_at = pose.at(0.5 * l, 0.5 * t, h + 0.6);
    out.annotate(Annotation::Label { id: format!("{id}:label"), anchor: label_at.clone(), text: loc(&wall.label_en, &wall.label_de), tone: tone_of(binding) }, &label_at);
    let middle = pose.at(0.5 * l, 0.5 * t, 0.5 * h);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{id}:clause"), anchor: wall_ref(id), clause }, &middle);
    }
    let zone = EntityRef::new(EntityKind::Solid, format!("{id}:zone-mid"), format!("walls[id={id}]"));
    for check in input.report.checks.iter().filter(|c| c.subject.entity_id == *id && c.status == norm::CheckStatus::Fail) {
        out.annotate(Annotation::FailureGlyph { id: format!("{id}:failure:{}", check.id), at: zone.clone(), mode: failure_mode(&check.id), extent: None }, &middle);
    }
}
