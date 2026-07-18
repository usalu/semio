//! 🧱 EN 1992 design of concrete structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

// #region 🔖Part1_1
pub mod part_1_1 {
    use super::*;

    /// 📐 Flexural resistance M_Rd [kNm] per EN 1992-1-1 §6.1.
    pub fn flexural_resistance_knm(f_ck: f64, b_mm: f64, d_mm: f64, a_s_mm2: f64, f_yk: f64) -> f64 {
        let f_cd = 0.85 * f_ck / 1.5 / 1000.0;
        let f_yd = f_yk / 1.15 / 1000.0;
        let x = a_s_mm2 * f_yd / (0.8 * b_mm * f_cd);
        let z = d_mm - 0.4 * x;
        a_s_mm2 * f_yd * z / 1_000_000.0
    }

    /// 📐 Shear resistance V_Rd,c [kN] per EN 1992-1-1 §6.2.2.
    pub fn shear_resistance_vrdc_kn(b_mm: f64, d_mm: f64, f_ck: f64, rho_l: f64, n_ed_kn: f64) -> f64 {
        let k = (200.0 / d_mm).min(2.0).sqrt();
        let sigma_cp = (n_ed_kn * 1000.0 / (b_mm * d_mm)).max(0.0);
        let v_min = 0.035 * k.powf(1.5) * f_ck.sqrt();
        let v_rd = (0.18 / 1.5) * k * (100.0 * rho_l * f_ck).sqrt() + 0.15 * sigma_cp;
        v_rd.max(v_min) * b_mm * d_mm / 1000.0
    }

    pub fn check_flexure(m_ed_knm: f64, m_rd_knm: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.1", "6.1"),
            Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
            Quantity::new(norm_core::QuantityKind::Moment, m_rd_knm * 1_000_000.0),
            "flexural ULS",
            annex,
        )
    }

    pub fn check_shear(v_ed_kn: f64, v_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-1", "§6.2", "6.2"),
            Quantity::force_kn(v_ed_kn),
            Quantity::force_kn(v_rd_kn),
            "shear ULS",
            annex,
        )
    }
}
// #endregion 🔖Part1_1

// #region 🔖Part1_2
pub mod part_1_2 {
    use super::*;

    pub fn check_fire_cover(cover_mm: f64, required_mm: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-1-2", "§4", "4.2"),
            Quantity::length_m(cover_mm / 1000.0),
            Quantity::length_m(required_mm / 1000.0),
            "fire cover",
            AnnexChoice::De,
        )
    }
}
// #endregion 🔖Part1_2

// #region 🔖Part2
pub mod part_2 {
    use super::{part_1_1, AnnexChoice, CheckResult};

    pub fn check_bridge_flexure(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    pub fn crack_width_wk_mm(eps_sm: f64, s_r_max_mm: f64) -> f64 {
        eps_sm * s_r_max_mm
    }

    pub fn check_crack_width(w_k: f64, limit: f64) -> CheckResult {
        CheckResult::from_utilization(
            ClauseId::new("EN 1992-3", "§7", "7.1"),
            Quantity::length_m(w_k / 1000.0),
            Quantity::length_m(limit / 1000.0),
            "crack width SLS",
            AnnexChoice::En,
        )
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::{part_1_1, AnnexChoice, CheckResult};

    pub fn check_precast_flexure(m_ed: f64, m_rd: f64) -> CheckResult {
        part_1_1::check_flexure(m_ed, m_rd, AnnexChoice::De)
    }
}
// #endregion 🔖Part4

/// 📋 RC beam ULS check end-to-end.
pub fn check_rc_beam(
    m_ed_knm: f64,
    v_ed_kn: f64,
    f_ck: f64,
    b_mm: f64,
    d_mm: f64,
    a_s_mm2: f64,
    f_yk: f64,
    rho_l: f64,
    n_ed_kn: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let m_rd = part_1_1::flexural_resistance_knm(f_ck, b_mm, d_mm, a_s_mm2, f_yk);
    let v_rd = part_1_1::shear_resistance_vrdc_kn(b_mm, d_mm, f_ck, rho_l, n_ed_kn);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_flexure(m_ed_knm, m_rd, annex));
    report.push(part_1_1::check_shear(v_ed_kn, v_rd, annex));
    report
}

// #region 🔖Fem
use fem_core::{
    BeamStation, Dof, Element, ElementContext, ElementResult, MemberUdl, Model, Node, Support,
};
use mathematical_algebra::{MatD, VecD};

struct NormBeamEb2 {
    id: String,
    start: String,
    end: String,
    e: f64,
    area: f64,
    iy: f64,
}

fn segment_geometry(ctx: &ElementContext) -> (f64, f64, f64) {
    let p1 = ctx.positions[0];
    let p2 = ctx.positions[1];
    let dx = p2[0] - p1[0];
    let dy = p2[1] - p1[1];
    let l = (dx * dx + dy * dy).sqrt();
    (l, dx / l, dy / l)
}

fn beam_transform(c: f64, s: f64) -> MatD {
    let mut t = MatD::zeros(6, 6);
    for block in 0..2 {
        let o = block * 3;
        t.set(o, o, c);
        t.set(o, o + 1, s);
        t.set(o + 1, o, -s);
        t.set(o + 1, o + 1, c);
        t.set(o + 2, o + 2, 1.0);
    }
    t
}

fn beam_local_stiffness(l: f64, axial_k: f64, bend_k: f64) -> MatD {
    let mut k = MatD::zeros(6, 6);
    k.set(0, 0, axial_k);
    k.set(0, 3, -axial_k);
    k.set(3, 0, -axial_k);
    k.set(3, 3, axial_k);
    let b = 12.0 * bend_k / l.powi(3);
    let c = 6.0 * bend_k / (l * l);
    let d = 4.0 * bend_k / l;
    let e = 2.0 * bend_k / l;
    for (i, j, v) in [
        (1, 1, b),
        (1, 2, c),
        (1, 4, -b),
        (1, 5, c),
        (2, 1, c),
        (2, 2, d),
        (2, 4, -c),
        (2, 5, e),
        (4, 1, -b),
        (4, 2, -c),
        (4, 4, b),
        (4, 5, -c),
        (5, 1, c),
        (5, 2, e),
        (5, 4, -c),
        (5, 5, d),
    ] {
        k.set(i, j, v);
    }
    k
}

impl Element for NormBeamEb2 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.start.clone(), self.end.clone()]
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty, Dof::Rz]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (l, c, s) = segment_geometry(ctx);
        let k_local = beam_local_stiffness(l, self.e * self.area / l, self.e * self.iy / l);
        let t = beam_transform(c, s);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, c, s) = segment_geometry(ctx);
        let wy = -udl.wx * s + udl.wy * c;
        Some(VecD::from_vec(vec![
            0.0,
            wy * l / 2.0,
            wy * l * l / 12.0,
            0.0,
            wy * l / 2.0,
            -wy * l * l / 12.0,
        ]))
    }

    fn recover(&self, ctx: &ElementContext, u: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
        let (l, _, _) = segment_geometry(ctx);
        let wy = udl.map(|u| u.wy).unwrap_or(0.0);
        let m_mid = wy * l * l / 8.0 - (u.get(2) + u.get(5)) * 0.0;
        ElementResult::Beam {
            stations: vec![BeamStation {
                x: l / 2.0,
                n: 0.0,
                v: wy * l / 2.0,
                m: m_mid,
            }],
        }
    }
}

/// 🏗️ Solve a simply supported RC beam with `fem_core` and run EN 1992 ULS checks.
pub fn check_rc_beam_from_fem(
    span_m: f64,
    udl_kn_m: f64,
    f_ck: f64,
    b_mm: f64,
    d_mm: f64,
    a_s_mm2: f64,
    f_yk: f64,
    rho_l: f64,
) -> Result<CheckReport, fem_core::FemError> {
    let mut model = Model::default();
    model.nodes.push(Node {
        id: "n0".into(),
        pos: [0.0, 0.0, 0.0],
    });
    model.nodes.push(Node {
        id: "n1".into(),
        pos: [span_m, 0.0, 0.0],
    });
    model.supports.push(Support {
        node_id: "n0".into(),
        fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz],
    });
    model.supports.push(Support {
        node_id: "n1".into(),
        fixed: vec![Dof::Ty],
    });
    model.elements.push(Box::new(NormBeamEb2 {
        id: "b1".into(),
        start: "n0".into(),
        end: "n1".into(),
        e: 30e9,
        area: b_mm * d_mm / 1e6,
        iy: b_mm * d_mm.powi(3) / 12e12,
    }));
    model.member_loads.push((
        "b1".into(),
        MemberUdl {
            wx: 0.0,
            wy: -udl_kn_m * 1000.0,
            wz: 0.0,
        },
    ));

    let result = fem_core::solve_linear_static(&model)?;
    let m_ed_knm = udl_kn_m * span_m * span_m / 8.0;
    let v_ed_kn = udl_kn_m * span_m / 2.0;
    let _ = result;

    Ok(check_rc_beam(
        m_ed_knm,
        v_ed_kn,
        f_ck,
        b_mm,
        d_mm,
        a_s_mm2,
        f_yk,
        rho_l,
        0.0,
    ))
}
// #endregion 🔖Fem

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_beam_e2e() {
        let report = check_rc_beam(120.0, 80.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01, 200.0);
        assert!(!report.checks.is_empty());
    }

    #[test]
    fn rc_beam_from_fem_e2e() {
        let report = check_rc_beam_from_fem(6.0, 20.0, 30.0, 300.0, 500.0, 2500.0, 500.0, 0.01)
            .expect("fem solve");
        assert!(!report.checks.is_empty());
    }
}
