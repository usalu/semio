//! 📜️ Response spectra: EN 1998-1 3.2.2 elastic and design spectra sampled with exact corner periods.

use crate::diagram_kit::{curves::sample_with_breaks, Xy};

/// 🌍️ Spectrum parameters: reference acceleration, soil factor, corner periods, damping correction and behaviour factor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpectrumParameters {
    pub ag: f64,
    pub s: f64,
    pub tb: f64,
    pub tc: f64,
    pub td: f64,
    pub eta: f64,
    pub q: f64,
    pub beta: f64,
}

/// 🌍️ Horizontal elastic response spectrum Se(T) of EN 1998-1 (3.2).
pub fn elastic(p: &SpectrumParameters, t: f64) -> f64 {
    let plateau = p.ag * p.s * p.eta * 2.5;
    if t <= p.tb {
        p.ag * p.s * (1.0 + t / p.tb * (p.eta * 2.5 - 1.0))
    } else if t <= p.tc {
        plateau
    } else if t <= p.td {
        plateau * p.tc / t
    } else {
        plateau * p.tc * p.td / (t * t)
    }
}

/// 🛡️ Design spectrum Sd(T) of EN 1998-1 (3.2.2.5) with the lower bound β·ag.
pub fn design(p: &SpectrumParameters, t: f64) -> f64 {
    let plateau = p.ag * p.s * 2.5 / p.q;
    let floor = p.beta * p.ag;
    if t <= p.tb {
        p.ag * p.s * (2.0 / 3.0 + t / p.tb * (2.5 / p.q - 2.0 / 3.0))
    } else if t <= p.tc {
        plateau
    } else if t <= p.td {
        (plateau * p.tc / t).max(floor)
    } else {
        (plateau * p.tc * p.td / (t * t)).max(floor)
    }
}

/// 📜️ Which spectrum a sampling produces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpectrumKind {
    Elastic,
    Design,
}

/// 📜️ A sampled spectrum over `[0, t_max]` that contains the corner periods exactly.
pub fn spectrum_curve(p: &SpectrumParameters, kind: SpectrumKind, t_max: f64, n: usize) -> Vec<Xy> {
    let f = |t: f64| match kind {
        SpectrumKind::Elastic => elastic(p, t),
        SpectrumKind::Design => design(p, t),
    };
    sample_with_breaks(f, 0.0, t_max, n, &[p.tb, p.tc, p.td])
}

/// 📍️ Corner period markers `(name, T, value)` for TB, TC and TD.
pub fn period_markers(p: &SpectrumParameters, kind: SpectrumKind) -> Vec<(&'static str, Xy)> {
    let value = |t: f64| match kind {
        SpectrumKind::Elastic => elastic(p, t),
        SpectrumKind::Design => design(p, t),
    };
    vec![("TB", [p.tb, value(p.tb)]), ("TC", [p.tc, value(p.tc)]), ("TD", [p.td, value(p.td)])]
}
