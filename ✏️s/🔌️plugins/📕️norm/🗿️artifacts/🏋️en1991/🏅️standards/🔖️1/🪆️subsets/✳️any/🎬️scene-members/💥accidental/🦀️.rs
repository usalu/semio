//! 💥 Accidental: vehicle impact forces on the windward facade and explosion pressure waves at their stand-off distance (EN 1991-1-7).

use super::kit::*;
use super::scene::*;
use super::{checks, handles, Ctx};
use checks::part_1_7;

pub(crate) const IMPACT_HEIGHT: f64 = 1.2;
pub(crate) const BLAST_HEIGHT: f64 = 1.0;
pub(crate) const IMPACT_SPACING: f64 = 0.8;

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    let pose = &layout.building;
    let count = d.accidental_cases.len();
    for (index, accidental) in d.accidental_cases.iter().enumerate() {
        if !out.own(&accidental.id) {
            continue;
        }
        let y = d.width * (index + 1) as f64 / (count + 1) as f64;
        let bound = ctx.binding(&accidental.id, &[]);
        let grips = handles::ids(d, layout, &accidental.id);
        for (j, impact) in accidental.impact.iter().enumerate() {
            let force_n = part_1_7::vehicle_impact_force_n(impact.vehicle_mass, impact.vehicle_speed);
            if !force_n.is_finite() || force_n <= 0.0 {
                continue;
            }
            let at = pose.at(0.0, y + IMPACT_SPACING * j as f64, IMPACT_HEIGHT);
            out.push(
                Spec::new(
                    EntityKind::Load,
                    format!("{}:impact{j}", accidental.id),
                    "accidentalCases",
                    Geometry::Load(LoadGlyph::Impact { at: at.clone(), direction: pose.dir(&Vec3::X), energy_or_force: force(force_n), case: case(&accidental.id, LoadKind::Accidental) }),
                    load(LoadKind::Accidental),
                    bi(|de| format!("{} {}", if de { "Anprall" } else { "Impact" }, accidental.id)),
                    LAYER_ACCIDENTAL,
                )
                .bound(bound.clone())
                .detail(bi(|de| format!("m = {} kg, v = {} m/s, F = {} kN", num(impact.vehicle_mass, 0, de), num(impact.vehicle_speed, 1, de), num(force_n / 1000.0, 0, de))))
                .handles(grips.clone()),
                &[at.clone()],
            );
            out.label(format!("{}:impact{j}:label", accidental.id), at, bi(|de| format!("F = {} kN", num(force_n / 1000.0, 0, de))), tone_of(&bound));
        }
        for (j, blast) in accidental.explosion.iter().enumerate() {
            let p = part_1_7::explosion_pressure_pa(blast.explosion_mass, blast.standoff);
            if !p.is_finite() || p <= 0.0 {
                continue;
            }
            let centre = pose.at(-blast.standoff.max(0.0), y + IMPACT_SPACING * j as f64, BLAST_HEIGHT);
            out.push(
                Spec::new(
                    EntityKind::Load,
                    format!("{}:explosion{j}", accidental.id),
                    "accidentalCases",
                    Geometry::Load(LoadGlyph::Explosion { centre: centre.clone(), pressure: pressure(p), shape: PressureWave::Spherical, case: case(&accidental.id, LoadKind::Accidental) }),
                    load(LoadKind::Accidental),
                    loc(&format!("Explosion {}", accidental.id), &format!("Explosion {}", accidental.id)),
                    LAYER_ACCIDENTAL,
                )
                .bound(bound.clone())
                .detail(bi(|de| format!("m = {} kg, {} {} m, p = {} kN/m²", num(blast.explosion_mass, 1, de), if de { "Abstand" } else { "stand-off" }, num(blast.standoff, 1, de), num(p / 1000.0, 1, de))))
                .handles(grips.clone()),
                &[centre.clone()],
            );
            let wall = pose.at(0.0, y + IMPACT_SPACING * j as f64, BLAST_HEIGHT);
            if blast.standoff > 0.01 {
                out.annotate(Annotation::Dimension { id: format!("{}:explosion{j}:standoff", accidental.id), from: centre.clone(), to: wall, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, blast.standoff), binding: None }, &centre);
            }
        }
    }
}
