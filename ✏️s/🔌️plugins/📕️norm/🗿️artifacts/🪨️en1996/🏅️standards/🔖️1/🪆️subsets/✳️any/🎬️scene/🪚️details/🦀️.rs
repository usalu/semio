//! 🪚️ Details: the Annex E panel (a thin panel in front of the wall with one glyph per edge condition: dashes free, one rod simple, two rods with hatching fixed), the chases and recesses of EN 1996-1-1 8.6 as translucent cut-out boxes on the wall face and the movement joints as full-height slit markers; chases and joints are the rows of the typed collections `chases` and `movementJoints`, linked to the wall by `wallId`.

use super::scene::projection_kit::{loc, v3, binding_where as binding_by};
use super::kit::*;
use super::layout::{Layout, WallPose, EPS};
use super::scene::*;
use super::{masonry as m, profile, SceneInput};
use m::numerical_models::{Edge, Edges};

const GAP: f64 = 0.9;
const HALF: f64 = 0.012;
const FACE: f64 = 0.006;
const JOINT_WIDTH: f64 = 0.02;

fn lerp(a: (f64, f64), b: (f64, f64), t: f64) -> (f64, f64) {
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}

/// 🧭️ The glyph of one panel edge from `a` to `b` (x, z in wall coordinates) with the outward unit direction `out`.
fn edge_glyph(mesh: &mut MeshBuilder, pose: &WallPose, edge: Edge, a: (f64, f64), b: (f64, f64), out: (f64, f64)) {
    let at = |p: (f64, f64), d: f64| [p.0 + out.0 * d, -GAP, p.1 + out.1 * d];
    match edge {
        Edge::Free => {
            for i in 0..5 {
                let (t0, t1) = (f64::from(i) / 5.0, (f64::from(i) + 0.5) / 5.0);
                mesh.rod(pose, at(lerp(a, b, t0), 0.04), at(lerp(a, b, t1), 0.04), 0.6 * HALF);
            }
        }
        Edge::Simple => mesh.rod(pose, at(a, 0.04), at(b, 0.04), HALF),
        Edge::Fixed => {
            mesh.rod(pose, at(a, 0.04), at(b, 0.04), HALF);
            mesh.rod(pose, at(a, 0.10), at(b, 0.10), HALF);
            let length = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
            let n = (length / 0.25).ceil().max(1.0) as u32;
            for i in 0..=n {
                let t = f64::from(i) / f64::from(n);
                let (p, q) = (lerp(a, b, t), lerp(a, b, (t + 0.06 / length.max(EPS)).min(1.0)));
                mesh.rod(pose, at(p, 0.04), at(q, 0.10), 0.7 * HALF);
            }
        }
    }
}

fn letters(edges: Edges) -> String {
    [edges.top, edges.bottom, edges.left, edges.right].iter().map(|e| match e { Edge::Free => 'F', Edge::Simple => 'S', Edge::Fixed => 'X' }).collect()
}

/// 🧱️ The edge conditions of the Annex E panel of a wall: those of the first M-31 design case linked to the wall by its `wall` text, else the ones derived from the held sides.
fn panel_edges(input: &SceneInput, wall: &model::MasonryWall) -> Option<Edges> {
    let linked = input.document.cases.iter().filter(|row| row.entry_id == "M-31" && row.texts.iter().any(|t| t.id == "wall" && t.value == wall.id)).find_map(|row| row.texts.iter().find(|t| t.id == "edges").and_then(|t| Edges::parse(&t.value)));
    linked.or_else(|| profile::lateral(input.document, wall).map(|lat| lat.edges))
}

fn annex_e_panel(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let governing = input.report.checks.iter().filter(|c| c.id.starts_with("en1996.m-31.w-rd.")).max_by(|a, b| a.utilization.total_cmp(&b.utilization)).map(|c| c.id.clone());
    for wall in &input.document.walls {
        let id = format!("{}:{ANNEX_E_TAG}", wall.id);
        if !out.wants(&id) {
            continue;
        }
        let (Some(pose), Some(edges)) = (layout.pose(&wall.id), panel_edges(input, wall)) else { continue };
        let (l, h) = (wall.length_m, wall.height_m);
        if l <= EPS || h <= EPS {
            continue;
        }
        let lat = profile::lateral(input.document, wall);
        let width = lat.as_ref().map_or(l, |lat| lat.l_ef.min(l));
        let x0 = 0.5 * (l - width);
        let mut mesh = MeshBuilder::default();
        mesh.cuboid(pose, (x0, x0 + width), (-GAP - 0.01, -GAP + 0.01), (0.0, h));
        edge_glyph(&mut mesh, pose, edges.top, (x0, h), (x0 + width, h), (0.0, 1.0));
        edge_glyph(&mut mesh, pose, edges.bottom, (x0, 0.0), (x0 + width, 0.0), (0.0, -1.0));
        edge_glyph(&mut mesh, pose, edges.left, (x0, 0.0), (x0, h), (-1.0, 0.0));
        edge_glyph(&mut mesh, pose, edges.right, (x0 + width, 0.0), (x0 + width, h), (1.0, 0.0));
        let points = mesh.points().to_vec();
        let class = m::annex_e::class_code(edges);
        let mu = lat.as_ref().map_or(1.0, |lat| lat.m1 / lat.m2.max(1e-9));
        let alpha = m::annex_e::standin_alpha1(edges, mu, h / width.max(EPS));
        let is_governing = lat.as_ref().is_some_and(|lat| governing.as_deref() == Some(format!("en1996.m-31.w-rd.{}.{}", wall.id, lat.load_case).as_str()));
        let basis = m::annex_e::basis_id(&class).unwrap_or_default();
        let mut spec = Spec::new(EntityKind::Solid, id, mesh.solid(), MaterialRole::Support, LAYER_SUPPORTS, loc("Annex E panel and edge conditions", "Anhang-E-Wand und Randbedingungen"));
        spec.opacity = if is_governing { 0.8 } else { 0.55 };
        spec.detail = loc(
            &format!("Edge conditions top, bottom, left, right: {} (F free dashes, S simple one rod, X fixed two rods with hatching); Annex E class {class}, coefficient cells not keyed (unverified basis {basis}), yield-line stand-in α₁ = {alpha:.4}{}", letters(edges), if is_governing { "; governing Annex E case of the building" } else { "" }),
            &format!("Randbedingungen oben, unten, links, rechts: {} (F frei gestrichelt, S gelenkig ein Stab, X eingespannt zwei Stäbe mit Schraffur); Anhang-E-Klasse {class}, Beiwerttabelle nicht erfasst (ungeprüfte Grundlage {basis}), Bruchlinien-Ersatzwert α₁ = {alpha:.4}{}", letters(edges), if is_governing { "; maßgebender Anhang-E-Fall des Gebäudes" } else { "" }),
        );
        if let Some(lat) = &lat {
            spec.binding = binding_of(input.report, &format!("en1996.m-31.w-rd.{}.{}", wall.id, lat.load_case));
            spec.channel = Some(ColourChannel::Utilisation);
        }
        out.push(spec, &points);
    }
}

fn chases(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for chase in &input.document.chases {
        let Some(wall) = input.document.walls.iter().find(|w| w.id == chase.wall_id) else { continue };
        let id = format!("{}:{CHASE_TAG}{}", wall.id, chase.id);
        if !out.wants(&id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (x0, x1, y0, y1) = m::chase_extent(chase);
        let (x0, x1, y0, y1) = (x0.clamp(0.0, wall.length_m), x1.clamp(0.0, wall.length_m), y0.clamp(0.0, wall.height_m), y1.clamp(0.0, wall.height_m));
        if x1 - x0 <= EPS || y1 - y0 <= EPS {
            continue;
        }
        let mut mesh = MeshBuilder::default();
        mesh.cuboid(pose, (x0, x1), (-FACE, chase.depth_m.clamp(FACE, wall.thickness_m)), (y0, y1));
        let points = mesh.points().to_vec();
        let kind = m::kind_words(chase.kind);
        let mut spec = Spec::new(EntityKind::Solid, id.clone(), mesh.solid(), MaterialRole::Neutral, LAYER_OPENINGS, loc(kind.0, kind.1));
        spec.owner_path = Some(format!("chases[id={}]", chase.id));
        spec.opacity = 0.4;
        let limits = m::chase_limits(chase.kind, wall.thickness_m, chase.length_m, y1, wall.height_m, input.document.annex);
        let (limit, width) = (limits.map_or(0.0, |l| l.depth_m), limits.and_then(|l| l.width_m));
        let table = limits.map_or("8.1/8.2", |l| l.table);
        let needs = limits.is_none() || chase.depth_m > limit + 1e-9 || width.is_some_and(|w| chase.width_m > w + 1e-9);
        let width_en = width.map_or(String::new(), |w| format!(", width {:.0} mm of {:.0} mm", chase.width_m * 1000.0, w * 1000.0));
        let width_de = width.map_or(String::new(), |w| format!(", Breite {:.0} mm von {:.0} mm", chase.width_m * 1000.0, w * 1000.0));
        spec.detail = loc(&format!("{} {:.0} mm deep in a {:.0} mm wall (Table {table}: {:.0} mm without calculation, unverified){width_en}, remaining thickness {:.0} mm{}", kind.0, chase.depth_m * 1000.0, wall.thickness_m * 1000.0, limit * 1000.0, (wall.thickness_m - chase.depth_m) * 1000.0, if needs { "; calculation required" } else { "" }), &format!("{} {:.0} mm tief in einer {:.0} mm dicken Wand (Tabelle {table}: {:.0} mm ohne Nachweis, ungeprüft){width_de}, Restdicke {:.0} mm{}", kind.1, chase.depth_m * 1000.0, wall.thickness_m * 1000.0, limit * 1000.0, (wall.thickness_m - chase.depth_m) * 1000.0, if needs { "; Nachweis erforderlich" } else { "" }));
        spec.binding = binding_of_owner(input.document, input.report, &id, &[]);
        spec.channel = Some(ColourChannel::Utilisation);
        out.push(spec, &points);
    }
}

fn joints(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for joint in &input.document.movement_joints {
        let Some(wall) = input.document.walls.iter().find(|w| w.id == joint.wall_id) else { continue };
        let id = format!("{}:{JOINT_TAG}{}", wall.id, joint.id);
        if !out.wants(&id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let x = joint.position_m.clamp(0.0, wall.length_m);
        let mut mesh = MeshBuilder::default();
        mesh.cuboid(pose, (x - 0.5 * JOINT_WIDTH, x + 0.5 * JOINT_WIDTH), (-FACE, wall.thickness_m + FACE), (0.0, wall.height_m));
        let points = mesh.points().to_vec();
        let mut spec = Spec::new(EntityKind::Solid, id.clone(), mesh.solid(), MaterialRole::Neutral, LAYER_OPENINGS, loc("Movement joint", "Dehnfuge"));
        spec.owner_path = Some(format!("movementJoints[id={}]", joint.id));
        spec.opacity = 0.6;
        spec.detail = loc(&format!("Movement joint at x = {x:.2} m of {:.2} m (marker width {:.0} mm is not a design value)", wall.length_m, JOINT_WIDTH * 1000.0), &format!("Dehnfuge bei x = {x:.2} m von {:.2} m (Markerbreite {:.0} mm ist kein Bemessungswert)", wall.length_m, JOINT_WIDTH * 1000.0));
        let spacing = format!("en1996.8.5.joint-spacing.{}", wall.id);
        spec.binding = binding_by(input.report, |c| c.id == spacing || (!is_document_check(&c.id) && owner_entity(input.document, c).as_deref() == Some(id.as_str())));
        spec.channel = Some(ColourChannel::Utilisation);
        out.push(spec, &points);
    }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    annex_e_panel(input, layout, out);
    chases(input, layout, out);
    joints(input, layout, out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_letters_follow_top_bottom_left_right() {
        assert_eq!(letters(Edges::parse("SSXF").unwrap()), "SSXF");
    }

    #[test]
    fn a_free_edge_has_five_dashes_a_simple_edge_one_rod_and_a_fixed_edge_two_rods_and_hatching() {
        let pose = WallPose { origin: v3(0.0, 0.0, 0.0), yaw: 0.0 };
        let count = |edge| {
            let mut mesh = MeshBuilder::default();
            edge_glyph(&mut mesh, &pose, edge, (0.0, 0.0), (1.0, 0.0), (0.0, -1.0));
            mesh.points().len()
        };
        let (free, simple, fixed) = (count(Edge::Free), count(Edge::Simple), count(Edge::Fixed));
        assert_eq!(free, 5 * simple);
        assert!(fixed > 2 * simple, "{fixed} {simple}");
    }
}
