//! 📉️ Effect profile: the relative moment, shear and elastic deflection of a member along its length, from the span, the support and the line and point loads of its actions; the stations of the ultimate and serviceability checks follow it. See [`super`].

use super::*;

/// 📉️ Load shares of one member: moment and shear weights of the uniform line load and of the mid-span point load.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Profile {
    support: SupportCondition,
    span: f64,
    line: f64,
    point: f64,
}

impl Profile {
    /// 🧭️ Profile of a flexural member whose effects are derived (not external or FEM); `None` for upright members and unloaded spans.
    pub(super) fn of(member: &RcMember) -> Option<Self> {
        Self::weighted(member, |_| 1.0)
    }

    /// 🧭️ Profile of the quasi-permanent loads: permanent actions in full, variable actions with their `ψ2`.
    pub(super) fn quasi_permanent(member: &RcMember, factors: &semio_s_plugin_norm_compliance::ActionFactors) -> Option<Self> {
        Self::weighted(member, |a| if matches!(a.kind, semio_s_artifact_norm_en1992_model::ActionKind::Permanent | semio_s_artifact_norm_en1992_model::ActionKind::Prestress) { 1.0 } else { psi_factors(factors, a.category).2 })
    }

    fn weighted(member: &RcMember, factor: impl Fn(&LoadCaseActions) -> f64) -> Option<Self> {
        let flexural = matches!(member.kind, MemberKind::Beam | MemberKind::Slab | MemberKind::FlatSlab | MemberKind::RibbedSlab | MemberKind::Bridge | MemberKind::LiquidRetaining);
        let derived = !member.use_fem && member.actions.iter().all(|a| a.source != semio_s_artifact_norm_en1992_model::ActionSource::External);
        let (line, point) = member.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1992_model::ActionKind::Accidental).fold((member.udl, 0.0), |(q, p), a| (q + factor(a) * (a.g_k_line + a.q_k_line), p + factor(a) * a.point_force.abs()));
        (flexural && derived && member.span > 0.0 && line + point > 0.0).then_some(Self { support: member.support, span: member.span, line, point })
    }

    fn weights(&self) -> (f64, f64) {
        let l = self.span;
        match self.support {
            SupportCondition::SimplySupported => (self.line * l * l / 8.0, self.point * l / 4.0),
            SupportCondition::Cantilever => (self.line * l * l / 2.0, self.point * l / 4.0),
            SupportCondition::Continuous | SupportCondition::Fixed => (self.line * l * l / 12.0, self.point * l / 4.0),
        }
    }

    /// 📉️ `|M(s)| / |M|max` at `s = x / L`, signed shapes of both loads superposed.
    pub(super) fn moment(&self, s: f64) -> f64 {
        let (wu, wp) = self.weights();
        let half = s.min(1.0 - s);
        let (u, p) = match self.support {
            SupportCondition::SimplySupported => (4.0 * s * (1.0 - s), 2.0 * half),
            SupportCondition::Cantilever => ((1.0 - s).powi(2), (1.0 - 2.0 * s).max(0.0)),
            SupportCondition::Continuous | SupportCondition::Fixed => (6.0 * s * (1.0 - s) - 1.0, 4.0 * half - 1.0),
        };
        ((wu * u + wp * p) / (wu + wp)).abs()
    }

    /// ✂️ `|V(s)| / |V|max` at `s = x / L`.
    pub(super) fn shear(&self, s: f64) -> f64 {
        let (qu, qp) = match self.support {
            SupportCondition::Cantilever => (self.line * self.span, self.point / 2.0),
            _ => (self.line * self.span / 2.0, self.point / 2.0),
        };
        let (u, p) = match self.support {
            SupportCondition::Cantilever => (1.0 - s, if s < 0.5 { 1.0 } else { 0.0 }),
            _ => ((1.0 - 2.0 * s).abs(), 1.0),
        };
        (qu * u + qp * p) / (qu + qp)
    }

    /// 🌊️ Elastic deflection times the bending stiffness `w(s) EI` (N m³) of the line load and the mid-span point load.
    pub(super) fn deflection_ei(&self, s: f64) -> f64 {
        let (l, q, p) = (self.span, self.line, self.point);
        let x = s * l;
        let (u, pt) = match self.support {
            SupportCondition::SimplySupported => (q * x * (l.powi(3) - 2.0 * l * x * x + x.powi(3)) / 24.0, p * x.min(l - x) * (3.0 * l * l - 4.0 * x.min(l - x).powi(2)) / 48.0),
            SupportCondition::Cantilever => {
                let a = 0.5 * l;
                (q * x * x * (6.0 * l * l - 4.0 * l * x + x * x) / 24.0, if x <= a { p * x * x * (3.0 * a - x) / 6.0 } else { p * a * a * (3.0 * x - a) / 6.0 })
            }
            SupportCondition::Continuous | SupportCondition::Fixed => (q * (x * (l - x)).powi(2) / 24.0, p * x.min(l - x).powi(2) * (3.0 * l - 4.0 * x.min(l - x)) / 48.0),
        };
        u + pt
    }
}
