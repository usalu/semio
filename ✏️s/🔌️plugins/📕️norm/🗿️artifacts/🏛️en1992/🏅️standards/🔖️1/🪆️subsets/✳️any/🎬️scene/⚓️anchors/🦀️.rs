//! ⚓️ Anchors: the anchor shaft with head and base plate in a translucent concrete block, the concrete cone as a pyramid of half-angle 35 degrees (`scr,N = 3 hef`) cut at a close edge, tension and shear glyphs, failure glyph, and bound `hef`, `c1` and `d` dimensions and grips.

use super::handle_set::*;
use super::kit::*;
use super::layout::Layout;
use super::model::Anchor;
use super::scene::*;
use super::SceneInput;

const CONE_FACTOR: f64 = 1.5;
const MARGIN: f64 = 2.0;
const BASE_PLATE_THICKNESS: f64 = 0.02;

fn pyramid(hef: f64, left: f64) -> Geometry {
    let r = CONE_FACTOR * hef;
    let (xl, xr) = (-left.min(r), r);
    let vertices = vec![v3(0.0, 0.0, -hef), v3(xl, -r, 0.0), v3(xr, -r, 0.0), v3(xr, r, 0.0), v3(xl, r, 0.0)];
    let tri = |a, b, c| Triangle { a, b, c };
    let triangles = vec![tri(1, 2, 3), tri(1, 3, 4), tri(2, 1, 0), tri(3, 2, 0), tri(4, 3, 0), tri(1, 4, 0)];
    Geometry::Solid(SolidGeometry { shape: SolidShape::Mesh { vertices, triangles } })
}

fn shift(origin: &Vec3, p: Vec3) -> Vec3 {
    add(origin, &p)
}

fn anchor(a: &Anchor, origin: &Vec3, input: &SceneInput, out: &mut Out) {
    let path = format!("anchors[id={}]", a.id);
    let name = |en: &str, de: &str| loc(&format!("Anchor {} {en}", a.id), &format!("Dübel {} {de}", a.id));
    let hef = a.h_ef.max(0.02);
    let d = a.d.max(0.006);
    let left = if a.c1 > 0.0 { a.c1 } else { CONE_FACTOR * hef + MARGIN * hef * 0.5 };
    let right = CONE_FACTOR * hef + 0.5 * hef;
    let half = MARGIN * hef;
    let depth = 1.6 * hef;
    let all = binding(input.report, &a.id, &[]);
    let steel = binding(input.report, &a.id, &["en1992-4.steel"]);
    let cone = binding(input.report, &a.id, &["en1992-4.cone."]);
    let shaft_ref = entity_ref(EntityKind::Fastener, &format!("{}:shaft", a.id), &path);
    let grips = [
        anchor_grip(&a.id, &shaft_ref, "embedment", HandleKind::ScalarAlong { axis: v3(0.0, 0.0, -1.0) }, shift(origin, v3(0.0, 0.0, -hef)), "hEf", hef, 1.0, (0.03, 0.5), ("embedment", "Verankerungstiefe")),
        anchor_grip(&a.id, &shaft_ref, "diameter", HandleKind::Radial { centre: shift(origin, Vec3::ZERO) }, shift(origin, v3(d / 2.0, 0.0, 0.0)), "d", d, 2.0, (0.006, 0.04), ("diameter", "Durchmesser")),
        anchor_grip(&a.id, &shaft_ref, "edge-distance", HandleKind::FaceOffset { normal: v3(-1.0, 0.0, 0.0) }, shift(origin, v3(-left, 0.0, 0.0)), "c1", a.c1, 1.0, (0.03, 1.5), ("edge distance", "Randabstand")),
    ];
    let with_edge = if a.c1 > 0.0 { 3 } else { 2 };
    let mut ids: Vec<String> = grips.iter().take(with_edge).map(|g| g.id.clone()).collect();
    out.handles.extend(grips.into_iter().take(with_edge).map(Grip::build));
    let moves = super::projection_kit::handle_kit::move_handles(&shaft_ref, &a.id, origin, input.placements, 0.0, false);
    ids.extend(moves.iter().map(|h| h.id.clone()));
    out.handles.extend(moves);
    let dh = if a.head_diameter > d { a.head_diameter } else { 2.0 * d };
    let head_ref = entity_ref(EntityKind::Solid, &format!("{}:head", a.id), &path);
    let head_grip = anchor_grip(&a.id, &head_ref, "head-diameter", HandleKind::Radial { centre: shift(origin, v3(0.0, 0.0, -hef)) }, shift(origin, v3(dh / 2.0, 0.0, -hef)), "headDiameter", dh, 2.0, (1.2 * d, 0.1), ("head diameter", "Kopfdurchmesser"));
    let head_ring: Vec<Vec2> = (0..16).map(|i| {
        let t = std::f64::consts::TAU * i as f64 / 16.0;
        Vec2::new(dh / 2.0 * t.cos(), dh / 2.0 * t.sin())
    }).collect();
    let head = Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: Frame { origin: shift(origin, v3(0.0, 0.0, -hef - 0.5 * d)), ..Frame::WORLD }, base: Polygon2::solid(head_ring), height: 0.5 * d } });
    out.push(Spec::new(EntityKind::Solid, format!("{}:head", a.id), path.clone(), head, MaterialRole::Steel, LAYER_ANCHORS, name("head", "Kopf"), name(&format!("head Ø{} mm", mm(dh)), &format!("Kopf Ø{} mm", mm(dh)))).bound(steel.clone()).with_handles(vec![head_grip.id.clone()]), &[shift(origin, v3(-dh / 2.0, -dh / 2.0, -hef - 0.5 * d)), shift(origin, v3(dh / 2.0, dh / 2.0, -hef))]);
    out.handles.push(head_grip.build());

    let block = Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: Frame { origin: shift(origin, v3(0.0, 0.0, -depth)), ..Frame::WORLD }, base: Polygon2::solid(vec![Vec2::new(-left, -half), Vec2::new(right, -half), Vec2::new(right, half), Vec2::new(-left, half)]), height: depth } });
    out.push(Spec::new(EntityKind::Solid, format!("{}:block", a.id), path.clone(), block, MaterialRole::Concrete, LAYER_ANCHORS, name("concrete", "Beton"), name(&format!("concrete block, hef = {} mm", mm(hef)), &format!("Betonkörper, hef = {} mm", mm(hef)))).translucent(0.15), &[shift(origin, v3(-left, -half, -depth)), shift(origin, v3(right, half, 0.0))]);

    let shaft = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Anchor, axis: Segment3 { from: shift(origin, Vec3::ZERO), to: shift(origin, v3(0.0, 0.0, -hef)) }, diameter: d, head: HeadShape::Hex, pattern: None, hole_clearance: None });
    out.push(Spec::new(EntityKind::Fastener, format!("{}:shaft", a.id), path.clone(), shaft, MaterialRole::Steel, LAYER_ANCHORS, name("shaft", "Schaft"), name(&format!("{} mm anchor, {}", mm(d), if a.cracked { "cracked concrete" } else { "uncracked concrete" }), &format!("Dübel {} mm, {}", mm(d), if a.cracked { "gerissener Beton" } else { "ungerissener Beton" }))).bound(steel).with_handles(ids), &[shift(origin, Vec3::ZERO), shift(origin, v3(0.0, 0.0, -hef))]);

    let side = (8.0 * d).max(0.15);
    let plate = Geometry::Plate(PlateGeometry { frame: Frame { origin: shift(origin, v3(0.0, 0.0, BASE_PLATE_THICKNESS / 2.0 + d)), ..Frame::WORLD }, outline: Polygon2::solid(vec![Vec2::new(-side / 2.0, -side / 2.0), Vec2::new(side / 2.0, -side / 2.0), Vec2::new(side / 2.0, side / 2.0), Vec2::new(-side / 2.0, side / 2.0)]), thickness: BASE_PLATE_THICKNESS, offset: 0.0, stiffeners: Vec::new() });
    out.push(Spec::new(EntityKind::Plate, format!("{}:base-plate", a.id), path.clone(), plate, MaterialRole::Steel, LAYER_ANCHORS, name("base plate", "Ankerplatte"), name("base plate", "Ankerplatte")), &[shift(origin, v3(-side / 2.0, -side / 2.0, d)), shift(origin, v3(side / 2.0, side / 2.0, d))]);

    let cone_shape = shift_geometry(pyramid(hef, left), origin);
    let cone_ref = entity_ref(EntityKind::Solid, &format!("{}:cone", a.id), &format!("{path}.hEf"));
    out.push(Spec::new(EntityKind::Solid, format!("{}:cone", a.id), format!("{path}.hEf"), cone_shape, MaterialRole::Concrete, LAYER_ANCHORS, name("concrete cone", "Betonausbruchkegel"), name(&format!("concrete cone, scr,N = {} mm{}", mm(2.0 * CONE_FACTOR * hef), if a.c1 > 0.0 && a.c1 < CONE_FACTOR * hef { ", cut at the edge" } else { "" }), &format!("Betonausbruchkegel, scr,N = {} mm{}", mm(2.0 * CONE_FACTOR * hef), if a.c1 > 0.0 && a.c1 < CONE_FACTOR * hef { ", am Rand abgeschnitten" } else { "" }))).bound(cone.clone()).translucent(0.3).ghosted(), &[shift(origin, v3(-left.min(CONE_FACTOR * hef), -CONE_FACTOR * hef, -hef)), shift(origin, v3(CONE_FACTOR * hef, CONE_FACTOR * hef, 0.0))]);
    if cone.as_ref().is_some_and(|b| !super::scene::norm::within_limit(b.ratio)) || all.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) {
        out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", a.id), at: cone_ref, mode: FailureMode::ShearCone, extent: None }, &shift(origin, Vec3::ZERO));
    }

    for (i, action) in a.actions.iter().enumerate() {
        let case = LoadCaseRef { id: action.id.clone(), kind: load_kind(action.kind), psi: None };
        let lift = 0.1 * (i as f64 + 1.0);
        for (key, direction, magnitude) in [("tension", v3(0.0, 0.0, 1.0), action.n_k), ("shear", v3(1.0, 0.0, 0.0), action.v_k)] {
            if magnitude.abs() <= 0.0 {
                continue;
            }
            let at_point = shift(origin, v3(0.0, 0.0, d + lift));
            let glyph = LoadGlyph::Point { at: at_point.clone(), direction: if magnitude > 0.0 { direction } else { mul(&direction, -1.0) }, magnitude: Quantity::new(QuantityKind::Force, magnitude), case: case.clone() };
            out.push(Spec::new(EntityKind::Load, format!("{}:load-{}-{key}", a.id, action.id), format!("{path}.actions[id={}]", action.id), Geometry::Load(glyph), MaterialRole::Load { kind: load_kind(action.kind) }, LAYER_LOADS, name(&format!("{key} action {}", action.id), &format!("{key} Einwirkung {}", action.id)), name(&format!("{key} action {} = {} kN", action.id, kn(magnitude)), &format!("{key} Einwirkung {} = {} kN", action.id, kn(magnitude)))), &[at_point]);
        }
    }

    let top = shift(origin, v3(0.0, 0.0, 0.0));
    out.annotate(Annotation::Dimension { id: format!("{}:dim-hef", a.id), from: top.clone(), to: shift(origin, v3(0.0, 0.0, -hef)), offset: v3(0.0, -0.25 * hef - d, 0.0), value: Quantity::new(QuantityKind::Length, hef), binding: Some(FieldBinding::new(format!("{path}.hEf"), Affine::IDENTITY, "m")) }, &top);
    if a.c1 > 0.0 {
        out.annotate(Annotation::Dimension { id: format!("{}:dim-c1", a.id), from: shift(origin, v3(-a.c1, 0.0, 0.0)), to: top.clone(), offset: v3(0.0, 0.2 * hef + d, 0.0), value: Quantity::new(QuantityKind::Length, a.c1), binding: Some(FieldBinding::new(format!("{path}.c1"), Affine::IDENTITY, "m")) }, &top);
    }
}

fn shift_geometry(g: Geometry, origin: &Vec3) -> Geometry {
    match g {
        Geometry::Solid(SolidGeometry { shape: SolidShape::Mesh { vertices, triangles } }) => Geometry::Solid(SolidGeometry { shape: SolidShape::Mesh { vertices: vertices.into_iter().map(|v| add(origin, &v)).collect(), triangles } }),
        other => other,
    }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for a in &input.document.anchors {
        if !out.wants(&a.id) {
            continue;
        }
        if let Some(origin) = layout.anchor(&a.id) {
            anchor(a, origin, input, out);
        }
    }
}
