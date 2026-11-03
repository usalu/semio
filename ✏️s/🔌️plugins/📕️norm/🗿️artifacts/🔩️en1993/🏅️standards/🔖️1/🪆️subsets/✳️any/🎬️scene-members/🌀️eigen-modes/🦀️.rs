//! 🌀️ Eigen modes and the static deflection of one member from the shared mechanics kernel ([`mechanics`]): the exact buckling load factor and mode shape for the drawn supports, the lateral-torsional mode of the declared moment diagram and restraints, and the Hermite deflection line under the serviceability moment. Nothing is sampled from a sine.

use super::kit::{add, mul};
use super::layout::is_upright;
use super::mechanics as mech;
use super::model::{SteelMaterial, SteelMember, SteelSection};
use super::scene::{DisplacementSample, Frame, Vec3};

const LTB_STRIDE: usize = 4;
pub const MODE_AMPLITUDE: f64 = 0.04;
pub const DEFLECTION_TARGET: f64 = 0.05;

/// 🌀️ One computed shape: the kernel load factor (`Ncr` in N for the unit axial load, `Mcr` in Nm for the LTB moment, the display magnification for a deflection) and the displacement samples in world directions.
pub struct Eigen {
    pub factor: f64,
    pub samples: Vec<DisplacementSample>,
}

fn world(frame: &Frame, u: [f64; 3]) -> Vec3 {
    add(&add(&mul(&frame.x, u[0]), &mul(&frame.y, u[1])), &mul(&frame.z, u[2]))
}

fn props(member: &SteelMember, section: &SteelSection, material: &SteelMaterial) -> mech::member_modes::Props {
    mech::member_modes::Props { length: member.length, e: material.e_modulus, g: material.g_modulus, a: section.area, iy: section.iy, iz: section.iz, j: section.it }
}

fn ends(member: &SteelMember) -> mech::member_modes::Ends {
    if is_upright(member) { mech::member_modes::Ends::Cantilever } else { mech::member_modes::Ends::Pinned }
}

fn mapped(frame: &Frame, curve: mech::member_modes::Curve, scale: f64) -> Vec<DisplacementSample> {
    curve.into_iter().map(|(at, u)| DisplacementSample { at, delta: world(frame, u.map(|v| v * scale)) }).collect()
}

/// 🌀️ Flexural buckling under a unit compression at the free end: `factor` is `Ncr`, the shape (largest translation 1) the first mode of the drawn supports (fixed base for a column, pinned and roller ends otherwise).
pub fn flexural(member: &SteelMember, section: &SteelSection, material: &SteelMaterial, frame: &Frame) -> Option<Eigen> {
    let (ncr, curve) = mech::member_modes::buckling(&props(member, section, material), ends(member)).ok()?;
    Some(Eigen { factor: ncr, samples: mapped(frame, curve, 1.0) })
}

/// 🦋️ Moment table of the declared diagram scaled to the peak moment `m_ed`: constant, parabolic (with its uniform load `8 M / L²`) or the straight line between `M` and `ψ M`.
fn moments(member: &SteelMember, m_ed: f64) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let l = member.length;
    match member.moment_diagram {
        semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform => (mech::ltb::uniform_moment(m_ed, l), Vec::new()),
        semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic => ((0..=8).map(|i| i as f64 / 8.0).map(|s| (s * l, m_ed * 4.0 * s * (1.0 - s))).collect(), vec![(0.0, 8.0 * m_ed / (l * l)), (l, 8.0 * m_ed / (l * l))]),
        _ => {
            let psi = member.end_moment_ratio_psi;
            let top = psi.abs().max(1.0);
            (mech::ltb::end_moments(m_ed / top, m_ed * psi / top, l), Vec::new())
        }
    }
}

/// 🦋️ Lateral-torsional buckling of the declared moment diagram with fork ends, the load height of the declared load application and the lateral and twist restraints every `ltbRestraintSpacing`: `factor` is `Mcr` for the peak `m_ed`, the shape the displacement of the top flange (shear-centre sway plus half-depth twist).
pub fn lateral_torsional(member: &SteelMember, section: &SteelSection, material: &SteelMaterial, frame: &Frame, m_ed: f64) -> Option<Eigen> {
    let l = member.length;
    let spacing = member.ltb_restraint_spacing;
    let (moment, line_load) = moments(member, m_ed);
    let height = match member.load_application {
        semio_s_artifact_norm_en1993_model::LoadApplication::TopFlange => section.h / 2.0,
        semio_s_artifact_norm_en1993_model::LoadApplication::BottomFlange => -section.h / 2.0,
        _ => 0.0,
    };
    let restraints = if spacing > 1e-6 && spacing < l - 1e-6 { (1..).map(|k| k as f64 * spacing).take_while(|x| *x < l - 1e-6).map(|x| mech::LtbRestraint { x, lateral: true, twist: true }).collect() } else { Vec::new() };
    let ltb = mech::LtbMember {
        length: l,
        e: material.e_modulus,
        g: material.g_modulus,
        iz: section.iz,
        iw: section.iw,
        j: section.it,
        polar_radius_sq: (section.iy + section.iz) / section.area,
        end_i: mech::ltb::fork_end(),
        end_j: mech::ltb::fork_end(),
        restraints,
        moment,
        line_load,
        load_height: height,
        elements: 32,
        ..Default::default()
    };
    let result = mech::ltb::ltb_analyze(&ltb, &mech::Control::none()).ok()?;
    let last = result.shape.len().saturating_sub(1);
    let samples = result
        .shape
        .iter()
        .enumerate()
        .filter(|(i, _)| i % LTB_STRIDE == 0 || *i == last)
        .map(|(_, p)| DisplacementSample { at: p.x / l, delta: mul(&frame.y, p.lateral + p.twist * section.h / 2.0) })
        .collect();
    Some(Eigen { factor: result.mcr, samples })
}

/// 📐️ Deflection line of a simply supported member under the serviceability moment `m_sls` (parabolic: its uniform load; otherwise the end couples `M` and `ψ M`): `factor` is the display magnification that brings the largest deflection to a twentieth of the length, the samples are the true displacements.
pub fn deflection(member: &SteelMember, section: &SteelSection, material: &SteelMaterial, frame: &Frame, m_sls: f64) -> Option<Eigen> {
    if is_upright(member) || m_sls <= 0.0 {
        return None;
    }
    let l = member.length;
    let loads = if member.moment_diagram == semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic {
        mech::member_modes::Loads { udl: 8.0 * m_sls / (l * l), ..Default::default() }
    } else {
        let psi = if member.moment_diagram == semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform { 1.0 } else { member.end_moment_ratio_psi };
        mech::member_modes::Loads { couples: (m_sls, m_sls * psi), ..Default::default() }
    };
    let curve = mech::member_modes::deflection(&props(member, section, material), mech::member_modes::Ends::Pinned, &loads).ok()?;
    let top = curve.iter().flat_map(|(_, u)| u.iter()).fold(0.0_f64, |m, v| m.max(v.abs()));
    (top > 0.0).then(|| Eigen { factor: (DEFLECTION_TARGET * l / top).max(1.0), samples: mapped(frame, curve, 1.0) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
