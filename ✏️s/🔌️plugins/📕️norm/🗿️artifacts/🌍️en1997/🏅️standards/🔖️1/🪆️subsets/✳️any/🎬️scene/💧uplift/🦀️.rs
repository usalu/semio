//! 💧uplift: the slab of an uplift case with stabilising and destabilising actions and the pore-pressure area under it.

use super::kit::*;
use super::layout::UPLIFT_SIDE;
use super::scene::*;
use super::Ctx;

pub const SLAB_THICKNESS: f64 = 0.5;
pub const SLAB_DEPTH: f64 = 2.0;

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for u in &d.uplift_cases {
        if !out.own("upliftCases", &u.id) {
            continue;
        }
        let pose = layout.pose(&u.id);
        let h = UPLIFT_SIDE / 2.0;
        let overall = ctx.binding(&u.id, &[]);
        let z = -SLAB_DEPTH;
        out.push(
            Spec::new(
                EntityKind::Plate,
                u.id.clone(),
                "upliftCases",
                Geometry::Plate(PlateGeometry { frame: pose.upright([0.0, 0.0, z]), outline: polygon2(&[[-h, -h], [h, -h], [h, h], [-h, h]]), thickness: SLAB_THICKNESS, offset: 0.0, stiffeners: Vec::new() }),
                MaterialRole::Concrete,
                loc(&format!("Uplift case {}", u.id), &format!("Auftriebsfall {}", u.id)),
                LAYER_UPLIFT,
            )
            .bound(overall.clone())
            .handles(ctx.handles_of(&u.id))
            .detail(format!("pore pressure {:.1} kPa", u.pore_pressure / 1000.0), format!("Porenwasserdruck {:.1} kPa", u.pore_pressure / 1000.0)),
            &[pose.at(-h, -h, z), pose.at(h, h, z + SLAB_THICKNESS)],
        );
        if u.pore_pressure > 1.0 {
            let ring = [pose.at(-h, -h, z), pose.at(h, -h, z), pose.at(h, h, z), pose.at(-h, h, z)];
            out.push(
                Spec::new(EntityKind::Load, format!("{}:pore-pressure", u.id), "upliftCases", Geometry::Load(LoadGlyph::Area { polygon: polygon3(&ring), direction: Vec3::Z, intensity: Quantity::new(QuantityKind::Pressure, u.pore_pressure), case: load_case("pore-pressure", LoadKind::Variable) }), MaterialRole::Load { kind: LoadKind::Variable }, loc(&format!("Pore pressure under {}", u.id), &format!("Porenwasserdruck unter {}", u.id)), LAYER_LOADS),
                &ring,
            );
        }
        let top = pose.at(0.0, 0.0, z + SLAB_THICKNESS);
        let below = pose.at(0.0, 0.0, z);
        for (tag, value, up, kind, en, de) in [
            ("stabilising", u.permanent_stabilizing, false, LoadKind::Permanent, "Stabilising permanent action", "Stabilisierende ständige Einwirkung"),
            ("destabilising-g", u.permanent_destabilizing, true, LoadKind::Permanent, "Destabilising permanent action", "Destabilisierende ständige Einwirkung"),
            ("destabilising-q", u.variable_destabilizing, true, LoadKind::Variable, "Destabilising variable action", "Destabilisierende veränderliche Einwirkung"),
        ] {
            if value > 1.0 {
                let at = if up { below.clone() } else { top.clone() };
                out.push(
                    Spec::new(EntityKind::Load, format!("{}:{tag}", u.id), "upliftCases", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: Vec3::new(0.0, 0.0, if up { 1.0 } else { -1.0 }), magnitude: force(value), case: load_case(tag, kind) }), MaterialRole::Load { kind }, loc(&format!("{en} on {}", u.id), &format!("{de} auf {}", u.id)), LAYER_LOADS),
                    &[at],
                );
            }
        }
    }
}
