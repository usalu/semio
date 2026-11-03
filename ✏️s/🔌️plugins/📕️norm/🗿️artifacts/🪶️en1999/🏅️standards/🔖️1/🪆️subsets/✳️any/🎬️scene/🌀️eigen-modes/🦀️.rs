//! 🌀️ Eigen modes and the deflection line of one aluminium member from the shared mechanics kernel ([`mechanics`]): the flexural buckling mode of the declared support, the lateral-torsional mode of the load-induced moment line with the unrestrained length as restraint spacing, and the elastic deflection line. Nothing is sampled from a sine.

use super::checks::na_de::{E_PA, G_PA};
use super::checks::part_1_1::{section_geometry, shear_centre_offset};
use super::kit::{add, mul};
use super::mechanics as mech;
use super::model::{AluminiumMember, AluminiumSection, SectionKind, SupportCondition};
use super::scene::{DisplacementSample, Frame, SurfaceSample, Vec2, Vec3};

pub const MODE_AMPLITUDE: f64 = 0.04;
pub const DEFLECTION_TARGET: f64 = 0.05;
const LTB_STRIDE: usize = 4;
const MOMENT_SAMPLES: usize = 40;

/// 🌀️ One computed shape: `Ncr` in N for the unit axial load, `Mcr` in Nm for the given peak moment, or the display magnification of a deflection; plus the displacement samples in world directions.
pub struct Eigen {
    pub factor: f64,
    pub samples: Vec<DisplacementSample>,
}

fn world(frame: &Frame, u: [f64; 3]) -> Vec3 {
    add(&add(&mul(&frame.x, u[0]), &mul(&frame.y, u[1])), &mul(&frame.z, u[2]))
}

fn props(member: &AluminiumMember, section: &AluminiumSection) -> mech::member_modes::Props {
    let (a, iy, iz, _, _, it, _) = section_geometry(section);
    mech::member_modes::Props { length: member.length, e: E_PA, g: G_PA, a, iy, iz, j: it }
}

fn ends(member: &AluminiumMember) -> mech::member_modes::Ends {
    match member.support {
        SupportCondition::SimplySupported => mech::member_modes::Ends::Pinned,
        SupportCondition::Cantilever => mech::member_modes::Ends::Cantilever,
        SupportCondition::Continuous => mech::member_modes::Ends::Fixed,
    }
}

fn mapped(frame: &Frame, curve: mech::member_modes::Curve, scale: f64) -> Vec<DisplacementSample> {
    curve.into_iter().map(|(at, u)| DisplacementSample { at, delta: world(frame, u.map(|v| v * scale)) }).collect()
}

/// 🌀️ Flexural buckling under a unit compression: `factor` is `Ncr`, the shape (largest translation 1) the first mode of the support condition (pinned, fixed at the root, fixed at both ends).
pub fn flexural(member: &AluminiumMember, section: &AluminiumSection, frame: &Frame) -> Option<Eigen> {
    let (ncr, curve) = mech::member_modes::buckling(&props(member, section), ends(member)).ok()?;
    Some(Eigen { factor: ncr, samples: mapped(frame, curve, 1.0) })
}

/// 📈️ Signed moment line `M(s)/max|M|` of a uniform load (`udl`) or of a mid-span point load (tip load of a cantilever): positive sagging.
pub fn moment_line(support: SupportCondition, udl: bool, s: f64) -> f64 {
    let m = s.min(1.0 - s);
    match (support, udl) {
        (SupportCondition::SimplySupported, true) => 4.0 * s * (1.0 - s),
        (SupportCondition::SimplySupported, false) => 2.0 * m,
        (SupportCondition::Cantilever, true) => -(1.0 - s).powi(2),
        (SupportCondition::Cantilever, false) => -(1.0 - s),
        (SupportCondition::Continuous, true) => 6.0 * s * (1.0 - s) - 1.0,
        (SupportCondition::Continuous, false) => 4.0 * m - 1.0,
    }
}

/// 🦋️ Lateral-torsional buckling of the moment line scaled to the peak `m_ed`, with the end conditions of the support (fork, fixed root and free tip, fixed ends) and lateral and twist restraints every unrestrained length `L_LT`: `factor` is `Mcr`, the shape the displacement of the section top (shear-centre sway plus half-depth twist).
pub fn lateral_torsional(member: &AluminiumMember, section: &AluminiumSection, frame: &Frame, udl: bool, m_ed: f64, depth: f64) -> Option<Eigen> {
    let l = member.length;
    let (a, iy, iz, _, _, it, iw) = section_geometry(section);
    let spacing = if member.ltb_length > 1e-6 { member.ltb_length } else { l };
    let restraints = (1..).map(|k| k as f64 * spacing).take_while(|x| *x < l - 1e-6).map(|x| mech::LtbRestraint { x, lateral: true, twist: true }).collect();
    let (end_i, end_j) = match member.support {
        SupportCondition::SimplySupported => (mech::ltb::fork_end(), mech::ltb::fork_end()),
        SupportCondition::Cantilever => (mech::ltb::fixed_end(), mech::LtbEnd::default()),
        SupportCondition::Continuous => (mech::ltb::fixed_end(), mech::ltb::fixed_end()),
    };
    let ltb = mech::LtbMember {
        length: l,
        e: E_PA,
        g: G_PA,
        iz,
        iw,
        j: it,
        polar_radius_sq: (iy + iz) / a.max(1e-12),
        end_i,
        end_j,
        restraints,
        moment: (0..=MOMENT_SAMPLES).map(|i| i as f64 / MOMENT_SAMPLES as f64).map(|s| (s * l, m_ed * moment_line(member.support, udl, s))).collect(),
        elements: 32,
        ..Default::default()
    };
    let result = mech::ltb::ltb_analyze(&ltb, &mech::Control::none()).ok()?;
    let last = result.shape.len().saturating_sub(1);
    let samples = result.shape.iter().enumerate().filter(|(i, _)| i % LTB_STRIDE == 0 || *i == last).map(|(_, p)| DisplacementSample { at: p.x / l, delta: mul(&frame.y, p.lateral + p.twist * depth / 2.0) }).collect();
    Some(Eigen { factor: result.mcr, samples })
}

/// 📐️ Elastic deflection line under a unit uniform load (`udl`) or a unit mid-span point load (tip load of a cantilever) scaled to the computed deflection `delta`: the largest displacement is `delta`, `factor` the display magnification bringing it to a twentieth of the length.
pub fn deflection(member: &AluminiumMember, section: &AluminiumSection, frame: &Frame, udl: bool, delta: f64) -> Option<Eigen> {
    let l = member.length;
    let at = if member.support == SupportCondition::Cantilever { l } else { l / 2.0 };
    let loads = if udl { mech::member_modes::Loads { udl: 1.0, ..Default::default() } } else { mech::member_modes::Loads { point: (at, 1.0), ..Default::default() } };
    let curve = mech::member_modes::deflection(&props(member, section), ends(member), &loads).ok()?;
    let top = curve.iter().flat_map(|(_, u)| u.iter()).fold(0.0_f64, |m, v| m.max(v.abs()));
    (top > 0.0 && delta > 0.0).then(|| Eigen { factor: (DEFLECTION_TARGET * l / delta).max(1.0), samples: mapped(frame, curve, delta / top) })
}

const TF_SAMPLES: usize = 20;
const TF_SCAN: usize = 4000;
const TF_PARTICIPATION: f64 = 0.05;

/// 🧭️ Coupled torsional-flexural system of a fork-supported member with sine modes: `K(N)·(u, v, φ) = 0` for the shear-centre translations `u` (along y) and `v` (along z) and the twist `φ`, with `Nu = π²E·I_z/L²`, `Nv = π²E·I_y/L²`, `Nt = (G·I_t + π²E·I_w/L²)/i0²`, `i0²` the polar radius of gyration about the shear centre and `(y0, z0)` the shear centre measured from the centroid (EN 1999-1-1 6.3.1.4, Annex I). Its determinant is the cubic of `section_solver::stability::n_cr_tf`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TorsionalFlexural {
    pub nu: f64,
    pub nv: f64,
    pub nt: f64,
    pub i0_sq: f64,
    pub y0: f64,
    pub z0: f64,
}

impl TorsionalFlexural {
    /// 🧭️ The matrix `K(N)` of the system at the load `n`; its null vector is the buckling mode at a critical load.
    pub fn matrix(&self, n: f64) -> [[f64; 3]; 3] {
        [[self.nu - n, 0.0, -n * self.z0], [0.0, self.nv - n, n * self.y0], [-n * self.z0, n * self.y0, self.i0_sq * (self.nt - n)]]
    }

    fn determinant(&self, n: f64) -> f64 {
        let m = self.matrix(n);
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    fn roots(&self) -> Vec<f64> {
        let low = self.nu.min(self.nv).min(self.nt) * 1e-3;
        let high = self.nu.max(self.nv).max(self.nt) * 1e3;
        let at = |i: usize| low * (high / low).powf(i as f64 / TF_SCAN as f64);
        let mut out = Vec::new();
        for i in 0..TF_SCAN {
            let (mut a, mut b) = (at(i), at(i + 1));
            let (fa, fb) = (self.determinant(a), self.determinant(b));
            if fa == 0.0 {
                out.push(a);
            } else if fa * fb < 0.0 {
                for _ in 0..200 {
                    let mid = 0.5 * (a + b);
                    if self.determinant(mid) * fa < 0.0 { b = mid } else { a = mid }
                }
                out.push(0.5 * (a + b));
            }
        }
        out
    }

    fn mode(&self, n: f64) -> [f64; 3] {
        let m = self.matrix(n);
        let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
        let norm = |v: [f64; 3]| v.iter().map(|x| x * x).sum::<f64>().sqrt();
        let best = [cross(m[0], m[1]), cross(m[0], m[2]), cross(m[1], m[2])].into_iter().max_by(|a, b| norm(*a).total_cmp(&norm(*b))).unwrap_or([0.0; 3]);
        let length = norm(best);
        if length > 0.0 { best.map(|x| x / length) } else { best }
    }

    /// 🧭️ The lowest critical load whose mode carries at least 5 % of its norm in the twist (a numerically noisy shear centre of a doubly symmetric section must not turn the weak-axis mode into a twisting one), with the mode `(u, v, φ)` normalised to unit length; `None` for a degenerate system.
    pub fn lowest_twisting(&self) -> Option<(f64, [f64; 3])> {
        self.roots().into_iter().find_map(|n| {
            let x = self.mode(n);
            let weight = self.i0_sq.sqrt() * x[2].abs();
            (weight > TF_PARTICIPATION * (x[0] * x[0] + x[1] * x[1] + weight * weight).sqrt()).then_some((n, x))
        })
    }
}

/// 🧭️ The system of a fork-supported member: `None` for other supports, angles and free outlines (their principal axes are not the section axes) and degenerate properties.
pub fn torsional_flexural_system(member: &AluminiumMember, section: &AluminiumSection) -> Option<TorsionalFlexural> {
    if member.support != SupportCondition::SimplySupported || matches!(section.kind, SectionKind::Angle | SectionKind::Free) || member.length <= 0.0 {
        return None;
    }
    let (a, iy, iz, _, _, it, iw) = section_geometry(section);
    let (y0, z0) = shear_centre_offset(section);
    let i0_sq = (iy + iz) / a + y0 * y0 + z0 * z0;
    let pi_l = std::f64::consts::PI / member.length;
    let system = TorsionalFlexural { nu: pi_l * pi_l * E_PA * iz, nv: pi_l * pi_l * E_PA * iy, nt: (G_PA * it + pi_l * pi_l * E_PA * iw) / i0_sq, i0_sq, y0, z0 };
    [a, iy, iz, it, i0_sq, system.nt].iter().all(|v| v.is_finite() && *v > 0.0).then_some(system)
}

/// 🌀️ Torsional-flexural buckling of a fork-supported member: `factor` is `N_cr,TF` in N (the pure torsional load of a doubly symmetric section), the shape the sine mode of the coupled system with the displacement of the section top (shear-centre translations plus twist about the shear centre), largest displacement 1.
pub fn torsional_flexural(member: &AluminiumMember, section: &AluminiumSection, frame: &Frame, depth: f64) -> Option<Eigen> {
    let system = torsional_flexural_system(member, section)?;
    let (n, [u, v, phi]) = system.lowest_twisting()?;
    let at = |t: f64| {
        let s = (std::f64::consts::PI * t).sin();
        [s * (u - phi * (depth / 2.0 - system.z0)), s * (v - phi * system.y0)]
    };
    let top = (0..=TF_SAMPLES).map(|i| at(i as f64 / TF_SAMPLES as f64)).flatten().fold(0.0_f64, |m, d| m.max(d.abs()));
    (top > 0.0).then(|| Eigen {
        factor: n,
        samples: (0..=TF_SAMPLES).map(|i| i as f64 / TF_SAMPLES as f64).map(|t| DisplacementSample { at: t, delta: world(frame, [0.0, at(t)[0] / top, at(t)[1] / top]) }).collect(),
    })
}

const SHELL_COLUMNS: usize = 48;
const SHELL_WAVES_SHOWN: u32 = 4;
const SHELL_ROWS_PER_WAVE: usize = 6;

/// 🥫️ Axisymmetric (Lorenz) buckling pattern of an axially compressed cylinder: `half_wave` is `π/k` with `k⁴ = 12(1 − ν²)/(r·t)²`, the minimum of `N(k) = D·k² + E·t/(r²·k²)` that gives the classical stress `σ = E·t/(r·√(3(1 − ν²)))` the EN 1999-1-5 critical stress `σ_x,Rcr` of a medium cylinder is based on (ν = E/(2G) − 1); the pattern repeats over the governing bay in `waves` half-waves, at most [`SHELL_WAVES_SHOWN`] of them from the bay start of a long bay.
#[derive(Clone, Debug, PartialEq)]
pub struct ShellMode {
    pub half_wave: f64,
    pub waves: u32,
    pub window: (f64, f64),
    pub generatrix: Vec<f64>,
    pub samples: Vec<SurfaceSample>,
}

/// 🥫️ The half-wave length `1.728·√(r·t)` (for ν = 0.3) of the classical axisymmetric pattern of a cylinder of radius `r` and wall `t`.
pub fn shell_half_wave(radius: f64, thickness: f64) -> f64 {
    let nu = E_PA / (2.0 * G_PA) - 1.0;
    std::f64::consts::PI * (radius * thickness).sqrt() / (12.0 * (1.0 - nu * nu)).powf(0.25)
}

/// 🥫️ Radial displacement (unit amplitude, outward positive) of the pattern at axial position `x`.
pub fn shell_profile(x: f64, window: (f64, f64), waves: u32) -> f64 {
    if x < window.0 - 1e-12 || x > window.1 + 1e-12 || window.1 <= window.0 {
        return 0.0;
    }
    (f64::from(waves) * std::f64::consts::PI * (x - window.0) / (window.1 - window.0)).sin()
}

/// 🥫️ The pattern of a shell of `length` whose governing bay (between ring stiffeners, or the whole shell) is `bay`: `None` for degenerate sizes.
pub fn shell_axial(radius: f64, thickness: f64, length: f64, bay: (f64, f64)) -> Option<ShellMode> {
    let bay_length = bay.1 - bay.0;
    if !(radius > 0.0 && thickness > 0.0 && length > 0.0 && bay_length > 0.0) {
        return None;
    }
    let half_wave = shell_half_wave(radius, thickness);
    let fitting = (bay_length / half_wave).round().max(1.0);
    let waves = fitting.min(f64::from(SHELL_WAVES_SHOWN)) as u32;
    let window = if fitting <= f64::from(SHELL_WAVES_SHOWN) { bay } else { (bay.0, bay.0 + f64::from(SHELL_WAVES_SHOWN) * half_wave) };
    let steps = SHELL_ROWS_PER_WAVE * waves as usize;
    let mut generatrix: Vec<f64> = std::iter::once(0.0).chain((0..=steps).map(|k| window.0 + (window.1 - window.0) * k as f64 / steps as f64)).chain(std::iter::once(length)).collect();
    generatrix.sort_by(f64::total_cmp);
    generatrix.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    let chord = 2.0 * radius * (std::f64::consts::PI / SHELL_COLUMNS as f64).sin();
    let samples = generatrix
        .iter()
        .flat_map(|x| (0..=SHELL_COLUMNS).map(move |c| (*x, c)))
        .map(|(x, c)| {
            let angle = std::f64::consts::TAU * c as f64 / SHELL_COLUMNS as f64;
            let w = shell_profile(x, window, waves);
            SurfaceSample { at: Vec2::new(c as f64 * chord, x - generatrix[0]), delta: Vec3::new(w * angle.cos(), w * angle.sin(), 0.0) }
        })
        .collect();
    Some(ShellMode { half_wave, waves, window, generatrix, samples })
}
