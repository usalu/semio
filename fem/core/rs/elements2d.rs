//! 📐 2D structural elements: axial `Bar2` truss and Euler-Bernoulli `BeamEb2` frame member.
//! Continuum (Tri3/Tri6/Quad4/Quad8) and plate (DKT) elements land here in follow-up workstreams.

use crate::{Dof, Element, ElementContext, ElementResult, MemberUdl};
use mathematical_algebra::{MatD, VecD};

// #region 🔖Geometry
fn segment_geometry(ctx: &ElementContext) -> (f64, f64, f64) {
    let p1 = ctx.positions[0];
    let p2 = ctx.positions[1];
    let dx = p2[0] - p1[0];
    let dy = p2[1] - p1[1];
    let l = (dx * dx + dy * dy).sqrt();
    (l, dx / l, dy / l)
}
// #endregion 🔖Geometry

// #region 🔖Bar2
/// 🪢 2-node axial truss element — DOFs `[Tx, Ty]` per node.
pub struct Bar2 {
    pub id: String,
    pub start: String,
    pub end: String,
    pub e: f64,
    pub area: f64,
}

impl Element for Bar2 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.start.clone(), self.end.clone()]
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tx, Dof::Ty]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let mut m = MatD::zeros(4, 4);
        let terms = [[cx * cx, cx * cy, -cx * cx, -cx * cy], [cx * cy, cy * cy, -cx * cy, -cy * cy]];
        for row in 0..2 {
            for col in 0..4 {
                m.set(row, col, k * terms[row][col]);
                m.set(row + 2, col, if col < 2 { -k * terms[row][col] } else { k * terms[row][col - 2] });
            }
        }
        m
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let (l, cx, cy) = segment_geometry(ctx);
        let k = self.e * self.area / l;
        let n = k * ((u_local.get(2) - u_local.get(0)) * cx + (u_local.get(3) - u_local.get(1)) * cy);
        ElementResult::Bar { n }
    }
}
// #endregion 🔖Bar2

// #region 🔖BeamEb2
/// 🧭 2D frame transformation matrix — block-diagonal 3 copies of the planar rotation, mapping
/// GLOBAL `[u1,v1,θ1,u2,v2,θ2]` to LOCAL coordinates.
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

/// 🧮 Local 6x6 Euler-Bernoulli beam stiffness, dof order `[u1,v1,θ1,u2,v2,θ2]`.
fn beam_local_stiffness(l: f64, axial_k: f64, bend_k: f64) -> MatD {
    let mut k = MatD::zeros(6, 6);
    k.set(0, 0, axial_k);
    k.set(0, 3, -axial_k);
    k.set(3, 0, -axial_k);
    k.set(3, 3, axial_k);

    let l2 = l * l;
    let bending = [
        (1, 1, 12.0 * bend_k / l2),
        (1, 2, 6.0 * bend_k / l),
        (1, 4, -12.0 * bend_k / l2),
        (1, 5, 6.0 * bend_k / l),
        (2, 1, 6.0 * bend_k / l),
        (2, 2, 4.0 * bend_k),
        (2, 4, -6.0 * bend_k / l),
        (2, 5, 2.0 * bend_k),
        (4, 1, -12.0 * bend_k / l2),
        (4, 2, -6.0 * bend_k / l),
        (4, 4, 12.0 * bend_k / l2),
        (4, 5, -6.0 * bend_k / l),
        (5, 1, 6.0 * bend_k / l),
        (5, 2, 2.0 * bend_k),
        (5, 4, -6.0 * bend_k / l),
        (5, 5, 4.0 * bend_k),
    ];
    for (row, col, value) in bending {
        k.set(row, col, value);
    }
    k
}

/// 🌬️ Local fixed-end load vector `[u1,v1,θ1,u2,v2,θ2]` for a local-frame UDL `(wx_local, wy_local)`.
fn beam_local_udl(l: f64, wx_local: f64, wy_local: f64) -> VecD {
    VecD::from_vec(vec![
        wx_local * l / 2.0,
        wy_local * l / 2.0,
        wy_local * l * l / 12.0,
        wx_local * l / 2.0,
        wy_local * l / 2.0,
        -wy_local * l * l / 12.0,
    ])
}

/// 🏗️ 2-node Euler-Bernoulli frame element — DOFs `[Tx, Ty, Rz]` per node.
pub struct BeamEb2 {
    pub id: String,
    pub start: String,
    pub end: String,
    pub e: f64,
    pub area: f64,
    pub iy: f64,
}

impl Element for BeamEb2 {
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
        let axial_k = self.e * self.area / l;
        let bend_k = self.e * self.iy / l;
        let k_local = beam_local_stiffness(l, axial_k, bend_k);
        let t = beam_transform(c, s);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    fn equivalent_nodal_loads(&self, ctx: &ElementContext, udl: &MemberUdl) -> Option<VecD> {
        let (l, c, s) = segment_geometry(ctx);
        let wx_local = udl.wx * c + udl.wy * s;
        let wy_local = -udl.wx * s + udl.wy * c;
        let f_local = beam_local_udl(l, wx_local, wy_local);
        let t = beam_transform(c, s);
        Some(t.transpose().mul_vec(&f_local))
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, udl: Option<&MemberUdl>) -> ElementResult {
        let (l, c, s) = segment_geometry(ctx);
        let axial_k = self.e * self.area / l;
        let bend_k = self.e * self.iy / l;
        let t = beam_transform(c, s);
        let u_loc = t.mul_vec(u_local);
        let k_local = beam_local_stiffness(l, axial_k, bend_k);

        let (wx_local, wy_local) = match udl {
            Some(u) => (u.wx * c + u.wy * s, -u.wx * s + u.wy * c),
            None => (0.0, 0.0),
        };
        let f_udl_local = beam_local_udl(l, wx_local, wy_local);
        let f_end = k_local.mul_vec(&u_loc).sub(&f_udl_local);

        let n1 = f_end.get(0);
        let v1 = f_end.get(1);
        let m1 = f_end.get(2);

        let stations = (0..11)
            .map(|i| {
                let x = l * (i as f64) / 10.0;
                crate::BeamStation { x, n: -n1, v: v1 + wy_local * x, m: m1 + v1 * x + wy_local * x * x / 2.0 }
            })
            .collect();
        ElementResult::Beam { stations }
    }
}
// #endregion 🔖BeamEb2

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve_linear_static, Model, Node, NodalLoad, Support};

    /// 🪢 Headless (no document layer) axial elongation check: δ = FL/EA, N = F.
    #[test]
    fn bar2_axial_matches_hand_calc() {
        let (e, area, l, p) = (200e9, 0.001, 2.0, 5000.0);
        let model = Model {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area })],
            // A single bar only resists motion along its own axis, so `b`'s transverse (Ty) DOF must
            // also be restrained here — otherwise it's a mechanism (zero stiffness, singular system).
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
            nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("solves");
        let expected = p * l / (e * area);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        assert!((b.values[Dof::Tx.index()] - expected).abs() / expected < 1e-9);
        let ElementResult::Bar { n } = result.elements[0].1 else { panic!("expected bar") };
        assert!((n - p).abs() < 1e-6);
    }

    /// 🏗️ Headless cantilever tip-load check: δ = PL³/3EI, θ = PL²/2EI — the classic beam-theory
    /// benchmark, exercised here directly against `fem_core::Model` (no document layer involved).
    #[test]
    fn beam_eb2_cantilever_matches_hand_calc() {
        let (e, iy, area, l, p) = (200e9, 1e-5, 0.01, 2.0, 1000.0);
        let model = Model {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Box::new(BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
            nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -p }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("solves");
        let expected_deflection = p * l.powi(3) / (3.0 * e * iy);
        let expected_rotation = p * l.powi(2) / (2.0 * e * iy);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        assert!((b.values[Dof::Ty.index()].abs() - expected_deflection).abs() / expected_deflection < 1e-6);
        assert!((b.values[Dof::Rz.index()].abs() - expected_rotation).abs() / expected_rotation < 1e-6);
    }

    /// 🌀 Rigid-body test: a pure translation (no relative deformation) must produce zero internal
    /// force — `Ke * rigid_translation ≈ 0`. Catches sign/assembly bugs that a single load case might not.
    #[test]
    fn beam_eb2_rigid_translation_gives_zero_force() {
        let (e, iy, area, l) = (200e9, 1e-5, 0.01, 2.0);
        let beam = BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [l, 0.0, 0.0]] };
        let ke = beam.stiffness_global(&ctx);
        let rigid = VecD::from_vec(vec![3.0, 4.0, 0.0, 3.0, 4.0, 0.0]);
        let f = ke.mul_vec(&rigid);
        for i in 0..6 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
        }
    }
}
// #endregion 🔖Tests
