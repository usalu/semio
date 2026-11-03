//! 🏛️ Annex blocks in 3D: the ring stiffeners of a silo shell as annular plates, the corrugated web as an extruded wave band behind its panel and the orthotropic deck as a deck plate on trapezoidal troughs (trough depth is illustrative).

use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::SceneInput;
use std::f64::consts::TAU;

const WAVES: usize = 3;
const TROUGHS: usize = 5;
const TROUGH_DEPTH_RATIO: f64 = 0.8;
const WEB_OFFSET: f64 = 1.0;

fn rings(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for silo in d.silo_shells.iter().filter(|s| s.ring_spacing > 1e-6 && s.ring_area > 0.0 && s.radius > 1e-6 && s.depth > 1e-6 && s.thickness > 1e-6) {
        if !out.own("siloShells", &silo.id) {
            continue;
        }
        let o = layout.origin(&silo.id);
        let height = if silo.ring_offset > 1e-6 { 2.0 * silo.ring_offset } else { silo.ring_area.sqrt() };
        let thickness = silo.ring_area / height;
        let inner = silo.radius + silo.thickness / 2.0;
        let count = ((silo.depth - 1e-9) / silo.ring_spacing).floor() as usize;
        for k in 1..=count {
            let z = k as f64 * silo.ring_spacing;
            out.push(
                Spec {
                    kind: EntityKind::Shell,
                    id: format!("{}:ring-{k}", silo.id),
                    collection: "siloShells",
                    geometry: Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: o.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(inner, z), Vec2::new(inner + height, z)] }, sweep: TAU }, thickness }),
                    role: MaterialRole::Steel,
                    binding: binding(input.report, &[silo.id.as_str()]),
                    name: loc(&format!("Ring stiffener {k} of {}", silo.id), &format!("Ringsteife {k} von {}", silo.id)),
                    detail: loc(&format!("z = {z:.2} m, A_r = {:.0} mm², spacing {:.2} m", silo.ring_area * 1e6, silo.ring_spacing), &format!("z = {z:.2} m, A_r = {:.0} mm², Abstand {:.2} m", silo.ring_area * 1e6, silo.ring_spacing)),
                    layer: LAYER_SHELLS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[v3(o.x - inner - height, o.y - inner - height, o.z + z), v3(o.x + inner + height, o.y + inner + height, o.z + z)],
            );
        }
    }
}

fn wave_band(p: &super::model::PlatedPanel) -> Vec<Vec2> {
    let (a0, a1, depth, t) = (p.corrugation_flat, p.corrugation_projection, p.corrugation_depth, p.thickness);
    let pitch = 2.0 * (a0 + a1);
    let offsets = [(0.0, depth / 2.0), (a0 / 2.0, depth / 2.0), (a0 / 2.0 + a1, -depth / 2.0), (a0 / 2.0 + a1 + a0, -depth / 2.0), (a0 / 2.0 + 2.0 * a1 + a0, depth / 2.0)];
    let mut centre: Vec<Vec2> = (0..WAVES).flat_map(|k| offsets.iter().map(move |(x, y)| Vec2::new(k as f64 * pitch + x, *y))).collect();
    centre.push(Vec2::new(WAVES as f64 * pitch, depth / 2.0));
    let upper = centre.iter().map(|q| Vec2::new(q.x, q.y + t / 2.0));
    let lower = centre.iter().rev().map(|q| Vec2::new(q.x, q.y - t / 2.0));
    upper.chain(lower).collect()
}

fn corrugated_webs(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for panel in d.plated_panels.iter().filter(|p| p.corrugation_flat > 1e-6 && p.corrugation_depth > 1e-6 && p.web_height > 1e-6 && p.thickness > 1e-6) {
        if !out.own("platedPanels", &panel.id) {
            continue;
        }
        let o = layout.origin(&panel.id);
        let pitch = 2.0 * (panel.corrugation_flat + panel.corrugation_projection);
        let origin = v3(o.x, o.y + panel.b + WEB_OFFSET, o.z);
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: format!("{}:web", panel.id),
                collection: "platedPanels",
                geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: origin.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, outline: Polygon2::solid(wave_band(panel)), thickness: panel.web_height, offset: 0.0, stiffeners: Vec::new() }),
                role: MaterialRole::Steel,
                binding: binding(input.report, &[panel.id.as_str()]),
                name: loc(&format!("Corrugated web of {}", panel.id), &format!("Trapezsteg von {}", panel.id)),
                detail: loc(&format!("a0 {:.0}, a1 {:.0}, d {:.0} mm, t {:.1} mm, h_w {:.2} m; {WAVES} waves shown", panel.corrugation_flat * 1e3, panel.corrugation_projection * 1e3, panel.corrugation_depth * 1e3, panel.thickness * 1e3, panel.web_height), &format!("a0 {:.0}, a1 {:.0}, d {:.0} mm, t {:.1} mm, h_w {:.2} m; {WAVES} Wellen dargestellt", panel.corrugation_flat * 1e3, panel.corrugation_projection * 1e3, panel.corrugation_depth * 1e3, panel.thickness * 1e3, panel.web_height)),
                layer: LAYER_PLATES,
                handles: Vec::new(),
                deformation: None,
            },
            &[origin.clone(), v3(origin.x + WAVES as f64 * pitch, origin.y + panel.corrugation_depth, origin.z + panel.web_height)],
        );
    }
}

fn trough_outline(width: f64, depth: f64, wall: f64) -> Vec<Vec2> {
    let (a, bottom, k) = (width / 2.0, width / 4.0, 1.5);
    vec![
        Vec2::new(-a, 0.0),
        Vec2::new(-bottom, -depth),
        Vec2::new(bottom, -depth),
        Vec2::new(a, 0.0),
        Vec2::new(a - wall * k, 0.0),
        Vec2::new(bottom - wall * k, -depth + wall),
        Vec2::new(-bottom + wall * k, -depth + wall),
        Vec2::new(-a + wall * k, 0.0),
    ]
}

fn decks(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for bf in d.bridge_fatigue.iter().filter(|b| b.deck_thickness > 1e-6 && b.trough_spacing > 1e-6 && b.trough_width > 1e-6 && b.crossbeam_spacing > 1e-6) {
        if !out.own("bridgeFatigue", &bf.id) {
            continue;
        }
        let o = layout.origin(&bf.id);
        let (length, width) = (bf.crossbeam_spacing, bf.trough_spacing * TROUGHS as f64);
        let depth = TROUGH_DEPTH_RATIO * bf.trough_width;
        let wall = if bf.trough_thickness > 1e-6 { bf.trough_thickness } else { 0.006 };
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: format!("{}:deck", bf.id),
                collection: "bridgeFatigue",
                geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: v3(o.x + length / 2.0, o.y + width / 2.0, o.z), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, outline: Polygon2::solid(vec![Vec2::new(-length / 2.0, -width / 2.0), Vec2::new(length / 2.0, -width / 2.0), Vec2::new(length / 2.0, width / 2.0), Vec2::new(-length / 2.0, width / 2.0)]), thickness: bf.deck_thickness, offset: 0.0, stiffeners: Vec::new() }),
                role: MaterialRole::Steel,
                binding: binding(input.report, &[bf.id.as_str()]),
                name: loc(&format!("Deck plate of {}", bf.id), &format!("Fahrbahnblech von {}", bf.id)),
                detail: loc(&format!("t = {:.0} mm, cross beam spacing {:.2} m, {TROUGHS} troughs at {:.0} mm", bf.deck_thickness * 1e3, length, bf.trough_spacing * 1e3), &format!("t = {:.0} mm, Querträgerabstand {:.2} m, {TROUGHS} Tröge im Abstand {:.0} mm", bf.deck_thickness * 1e3, length, bf.trough_spacing * 1e3)),
                layer: LAYER_PLATES,
                handles: super::handles::ids(d, layout, &bf.id),
                deformation: None,
            },
            &[o.clone(), v3(o.x + length, o.y + width, o.z + bf.deck_thickness)],
        );
        for i in 0..TROUGHS {
            let y = o.y + bf.trough_spacing * (i as f64 + 0.5);
            out.push(
                Spec {
                    kind: EntityKind::Plate,
                    id: format!("{}:trough-{}", bf.id, i + 1),
                    collection: "bridgeFatigue",
                    geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: v3(o.x, y, o.z), x: Vec3::Y, y: Vec3::Z, z: Vec3::X }, outline: Polygon2::solid(trough_outline(bf.trough_width, depth, wall)), thickness: length, offset: 0.0, stiffeners: Vec::new() }),
                    role: MaterialRole::Steel,
                    binding: binding(input.report, &[bf.id.as_str()]),
                    name: loc(&format!("Trough {} of {}", i + 1, bf.id), &format!("Trog {} von {}", i + 1, bf.id)),
                    detail: loc(&format!("width {:.0} mm, wall {:.1} mm (depth illustrative)", bf.trough_width * 1e3, wall * 1e3), &format!("Breite {:.0} mm, Wand {:.1} mm (Tiefe veranschaulichend)", bf.trough_width * 1e3, wall * 1e3)),
                    layer: LAYER_PLATES,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[v3(o.x, y - bf.trough_width / 2.0, o.z - depth), v3(o.x + length, y + bf.trough_width / 2.0, o.z)],
            );
        }
    }
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    rings(input, layout, out);
    corrugated_webs(input, layout, out);
    decks(input, layout, out);
}
