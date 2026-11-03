//! 🏗️ Gravity: the imposed area loads of the floors, coloured by use category (Okabe-Ito palette, colour-blind safe, with the category as text), and the self-weight build-up
//! of the structural layers as a stack coloured by unit weight (EN 1991-1-1).

use super::kit::*;
use super::scene::colour_maps::{ramp, RampId, Variant};
use super::scene::*;
use super::{checks, handles, Ctx};
use checks::part_1_1;

const SLAB_LIFT: f64 = 0.1;
pub const STACK_SIDE: f64 = 4.0;
const GAMMA_MAX_KN_M3: f64 = 80.0;

fn category_rgb(category: &str) -> Rgb {
    let (r, g, b) = match category.chars().next().map(|c| c.to_ascii_uppercase()) {
        Some('A') => (230, 159, 0),
        Some('B') => (86, 180, 233),
        Some('C') => (0, 158, 115),
        Some('D') => (240, 228, 66),
        Some('E') => (0, 114, 178),
        Some('F') => (213, 94, 0),
        Some('G') => (204, 121, 167),
        Some('H') => (153, 153, 153),
        _ => (120, 120, 120),
    };
    Rgb { r, g, b }
}

/// 🏢 The slab of a floor in building coordinates as `(x0, y0, size_x, size_y, z)`: the footprint scaled to the floor area, at its storey.
pub fn plate(d: &super::model::En1991Snapshot, index: usize, floor: &super::model::FloorArea) -> (f64, f64, f64, f64, f64) {
    let storeys = usize::from(d.storey_count.max(1));
    let scale = (floor.area / (d.depth * d.width).max(1e-6)).clamp(0.01, 1.0).sqrt();
    let (sx, sy) = (d.depth * scale, d.width * scale);
    ((d.depth - sx) / 2.0, (d.width - sy) / 2.0, sx, sy, d.height / storeys as f64 * (index % storeys) as f64 + SLAB_LIFT)
}

pub fn floors(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    let pose = &layout.building;
    for (index, floor) in d.floors.iter().enumerate() {
        if !out.own(&floor.id) {
            continue;
        }
        let (x0, y0, sx, sy, z) = plate(d, index, floor);
        let points: Vec<Vec3> = rect2(x0, y0, x0 + sx, y0 + sy).iter().map(|p| pose.at(p[0], p[1], z)).collect();
        let required = part_1_1::imposed_qk_reduced_pa(&floor.category, d.annex, floor.area, d.storey_count);
        let bound = ctx.binding(&floor.id, &[]);
        let grips = handles::ids(d, layout, &floor.id);
        let name = bi(|de| format!("{} {}", if de { "Geschoss" } else { "Floor" }, floor.id));
        let detail = bi(|de| format!("{} {}, A = {} m², q_k = {} kN/m²", if de { "Kategorie" } else { "category" }, floor.category, num(floor.area, 1, de), required.map_or("?".to_string(), |q| num(q / 1000.0, 2, de))));
        out.push(
            Spec::new(EntityKind::Surface, floor.id.clone(), "floors", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&points) } }), load(LoadKind::Variable), name.clone(), LAYER_IMPOSED)
                .bound(bound.clone())
                .channel(ColourChannel::Custom { rgb: category_rgb(&floor.category) })
                .opacity(0.55)
                .detail(detail.clone())
                .handles(grips.clone()),
            &points,
        );
        out.label(format!("{}:label", floor.id), points[2].clone(), bi(|de| format!("{} {}", if de { "Kategorie" } else { "Category" }, floor.category)), tone_of(&bound));
        if let Some(q) = required.filter(|q| *q > 1e-9) {
            out.push(
                Spec::new(
                    EntityKind::Load,
                    format!("{}:load", floor.id),
                    "floors",
                    Geometry::Load(LoadGlyph::Area { polygon: polygon3(&points), direction: mul(&Vec3::Z, -1.0), intensity: pressure(q), case: case(&floor.id, LoadKind::Variable) }),
                    load(LoadKind::Variable),
                    name,
                    LAYER_IMPOSED,
                )
                .bound(bound)
                .channel(ColourChannel::Role)
                .opacity(0.8)
                .detail(detail)
                .handles(grips),
                &points,
            );
        }
    }
}

/// 🎨️ The legend of the unit weight γ that colours the self-weight stack.
pub fn weight_legend() -> Legend {
    Legend { id: "unit-weight".into(), title: loc("Unit weight γ", "Wichte γ"), scale: LegendScale::Scalar { field_id: "unit-weight".into(), map: ColourMapRef { id: RampId::Cool.wire().into(), min: 0.0, max: GAMMA_MAX_KN_M3 }, unit: QuantityKind::Dimensionless } }
}

pub fn stack(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    let pose = &layout.self_weight;
    let map = ramp(RampId::Cool, Variant::Light);
    let mut z = 0.0;
    for element in &d.self_weight_elements {
        let thickness = element.thickness.max(0.01);
        let bottom = z;
        z += thickness;
        if !out.own(&element.id) {
            continue;
        }
        let z = bottom;
        let gamma = part_1_1::density_kn_m3(&element.material, d.annex).map_or(0.0, |g| g.2);
        let rgb = map.sample((gamma / GAMMA_MAX_KN_M3).clamp(0.0, 1.0));
        let bound = ctx.binding(&element.id, &[]);
        let required = part_1_1::self_weight_pa(&element.material, thickness, d.annex);
        let grips = handles::ids(d, layout, &element.id);
        let name = bi(|de| format!("{} {}", if de { "Eigengewicht" } else { "Self-weight" }, element.id));
        let detail = bi(|de| format!("{}, {} {} m, γ = {} kN/m³, g_k = {} kN/m²", element.material, if de { "Dicke" } else { "thickness" }, num(thickness, 3, de), num(gamma, 1, de), required.map_or("?".to_string(), |g| num(g / 1000.0, 2, de))));
        let corners = [pose.at(0.0, 0.0, z), pose.at(STACK_SIDE, STACK_SIDE, z + thickness)];
        out.push(
            Spec::new(
                EntityKind::Solid,
                element.id.clone(),
                "selfWeightElements",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([0.0, 0.0, z]), base: polygon2(&rect2(0.0, 0.0, STACK_SIDE, STACK_SIDE)), height: thickness } }),
                MaterialRole::Concrete,
                name.clone(),
                LAYER_SELF_WEIGHT,
            )
            .bound(bound.clone())
            .channel(ColourChannel::Custom { rgb: Rgb { r: rgb[0], g: rgb[1], b: rgb[2] } })
            .detail(detail.clone())
            .handles(grips.clone()),
            &corners,
        );
        let from = pose.at(STACK_SIDE + 0.3, 0.0, z);
        let to = pose.at(STACK_SIDE + 0.3, 0.0, z + thickness);
        out.annotate(Annotation::Dimension { id: format!("{}:thickness", element.id), from, to: to.clone(), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, thickness), binding: Some(FieldBinding::new(format!("selfWeightElements[id={}].thickness", element.id), Affine::IDENTITY, "m")) }, &to);
        if let Some(g) = required.filter(|g| *g > 1e-9) {
            let top = pose.at(0.0, 0.0, z + thickness);
            let patch: Vec<Vec3> = rect2(0.0, 0.0, STACK_SIDE, STACK_SIDE).iter().map(|p| pose.at(p[0], p[1], z + thickness)).collect();
            out.push(
                Spec::new(
                    EntityKind::Load,
                    format!("{}:load", element.id),
                    "selfWeightElements",
                    Geometry::Load(LoadGlyph::Area { polygon: polygon3(&patch), direction: mul(&Vec3::Z, -1.0), intensity: pressure(g), case: case(&element.id, LoadKind::Permanent) }),
                    load(LoadKind::Permanent),
                    name,
                    LAYER_SELF_WEIGHT,
                )
                .bound(bound)
                .channel(ColourChannel::Role)
                .opacity(0.8)
                .detail(detail)
                .handles(grips),
                &[top],
            );
        }
    }
    if !d.self_weight_elements.is_empty() && out.own(global::SELF_WEIGHT) {
        out.label("selfWeight:label".into(), pose.at(0.0, 0.0, z + 0.5), loc("Self-weight build-up", "Eigengewichtsaufbau"), Tone::Neutral);
    }
}
