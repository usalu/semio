//! 🪨️ Ground: one translucent soil body per layer with its parameter label and interface dimension, plus the groundwater table plane.

use super::kit::*;
use super::scene::*;
use super::Ctx;

const GROUND_OPACITY: f64 = 0.55;
const WATER_OPACITY: f64 = 0.35;

fn rectangle(w: f64, d: f64) -> Polygon2 {
    polygon2(&[[0.0, 0.0], [w, 0.0], [w, d], [0.0, d]])
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d) = (ctx.layout, ctx.input.document);
    let (w, depth) = (layout.max[0] - layout.min[0], layout.max[1] - layout.min[1]);
    let mut cursor = 0.0_f64;
    for layer in &d.layers {
        let top = cursor;
        let bottom = layer.depth_bottom.max(top + 1e-3);
        cursor = bottom;
        if !out.own("layers", &layer.id) {
            continue;
        }
        let origin = v3(layout.min[0], layout.min[1], -top);
        let binding = ctx.binding(&layer.id, &[]);
        let name = loc(&format!("Layer {} ({})", layer.id, layer.soil_type), &format!("Schicht {} ({})", layer.id, layer.soil_type));
        let (phi, gamma, c) = (layer.phi_prime_deg, layer.gamma / 1000.0, layer.cohesion_effective / 1000.0);
        out.push(
            Spec::new(
                EntityKind::Solid,
                layer.id.clone(),
                "layers",
                Geometry::Solid(SolidGeometry {
                    shape: SolidShape::StrataColumn {
                        frame: Frame { origin: origin.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z },
                        footprint: rectangle(w, depth),
                        strata: vec![Stratum { thickness: bottom - top, role: soil(layer.soil_type.as_str()), label: name.clone() }],
                    },
                }),
                soil(layer.soil_type.as_str()),
                name,
                LAYER_GROUND,
            )
            .bound(binding)
            .channel(ColourChannel::Role)
            .opacity(GROUND_OPACITY)
            .detail(format!("{:.2} to {:.2} m, phi' {phi:.1} deg, c' {c:.1} kPa, gamma {gamma:.1} kN/m3", top, bottom), format!("{:.2} bis {:.2} m, phi' {phi:.1} Grad, c' {c:.1} kPa, gamma {gamma:.1} kN/m3", top, bottom))
            .handles(ctx.handles_of(&layer.id)),
            &[origin.clone(), v3(layout.max[0], layout.max[1], -bottom)],
        );
        let corner = v3(layout.min[0], layout.min[1], -(top + bottom) / 2.0);
        out.annotate(
            Annotation::Label {
                id: format!("{}:label", layer.id),
                anchor: corner.clone(),
                text: loc(&format!("{} phi' {phi:.0} deg, c' {c:.0} kPa, gamma {gamma:.1} kN/m3", layer.soil_type), &format!("{} phi' {phi:.0} Grad, c' {c:.0} kPa, gamma {gamma:.1} kN/m3", layer.soil_type)),
                tone: Tone::Neutral,
            },
            &corner,
        );
        let (from, to) = (v3(layout.min[0] - 0.5, layout.min[1], -top), v3(layout.min[0] - 0.5, layout.min[1], -bottom));
        out.annotate(
            Annotation::Dimension {
                id: format!("{}:thickness", layer.id),
                from: from.clone(),
                to,
                offset: Vec3::ZERO,
                value: Quantity::new(QuantityKind::Length, bottom - top),
                binding: Some(FieldBinding::new(format!("layers[id={}].depthBottom", layer.id), Affine::IDENTITY, "m")),
            },
            &from,
        );
    }
}

pub fn water(ctx: &Ctx, out: &mut Out) {
    let (layout, d) = (ctx.layout, ctx.input.document);
    let z = -d.groundwater_level;
    let (x0, y0, x1, y1) = (layout.min[0], layout.min[1], layout.max[0], layout.max[1]);
    let plane = [v3(x0, y0, z), v3(x1, y0, z), v3(x1, y1, z), v3(x0, y1, z)];
    out.push(
        Spec::new(
            EntityKind::Surface,
            "groundwater",
            "@groundwaterLevel",
            Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&plane) } }),
            MaterialRole::Neutral,
            loc("Groundwater table", "Grundwasserspiegel"),
            LAYER_WATER,
        )
        .detail(format!("{:.2} m below ground", d.groundwater_level), format!("{:.2} m unter Gelände", d.groundwater_level))
        .opacity(WATER_OPACITY)
        .handles(ctx.handles_of("groundwater")),
        &plane,
    );
    let anchor = v3(x0, y0, z);
    out.annotate(
        Annotation::Label { id: "groundwater:label".into(), anchor: anchor.clone(), text: loc(&format!("GWL {:.2} m", d.groundwater_level), &format!("GW {:.2} m", d.groundwater_level)), tone: Tone::Info },
        &anchor,
    );
    for well in d.layer_waters.iter().filter(|w| w.well_radius > 1e-6 && w.well_water_depth > 0.0) {
        let Some(layer) = d.layers.iter().find(|l| l.id == well.layer_id) else { continue };
        let [x, y] = layout.well_at();
        let id = format!("{}:well", well.id);
        let top = -well.well_water_depth;
        let bottom = -layer.depth_bottom;
        out.push(
            Spec::new(
                EntityKind::Solid,
                id.clone(),
                "layerWaters",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: Frame { origin: v3(x, y, bottom), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, radius: well.well_radius, height: (top - bottom).max(1e-3) } }),
                MaterialRole::Neutral,
                loc(&format!("Pumped well of {}", well.id), &format!("Gepumpter Brunnen von {}", well.id)),
                LAYER_WATER,
            )
            .detail(
                format!("water level {:.2} m below ground in the well, radius {:.2} m", well.well_water_depth, well.well_radius),
                format!("Wasserspiegel {:.2} m unter Gelände im Brunnen, Radius {:.2} m", well.well_water_depth, well.well_radius),
            )
            .opacity(WATER_OPACITY)
            .handles(ctx.handles_of(&well.id)),
            &[v3(x, y, top), v3(x, y, bottom)],
        );
    }
}
