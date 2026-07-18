//! 🧊 3D structural elements: axial `Bar3` truss and Euler-Bernoulli `Frame3` frame member (with
//! torsion). Solid (Tet4/Hex8) and facet-shell elements land here in follow-up workstreams.

use crate::{BeamStation, Dof, Element, ElementContext, ElementResult, MemberUdl};
use mathematical_algebra::{vec3d_cross, vec3d_length, vec3d_normalize, vec3d_sub, Mat3d, MatD, VecD};

// #region 🔖Bar3
/// 🪵 Two-node 3D axial truss element — carries only translational DOFs, stiffness `k = EA/L`
/// projected onto the member's unit direction.
pub struct Bar3 {
    pub id: String,
    pub node_a: String,
    pub node_b: String,
    pub e: f64,
    pub a: f64,
}

impl Element for Bar3 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.node_a.clone(), self.node_b.clone()]
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 3] = [Dof::Tx, Dof::Ty, Dof::Tz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let c = vec3d_normalize(d);
        let k = self.e * self.a / l;
        let mut ke = MatD::zeros(6, 6);
        for i in 0..3 {
            for j in 0..3 {
                let v = k * c[i] * c[j];
                ke.set(i, j, v);
                ke.set(i, j + 3, -v);
                ke.set(i + 3, j, -v);
                ke.set(i + 3, j + 3, v);
            }
        }
        ke
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let c = vec3d_normalize(d);
        let k = self.e * self.a / l;
        let du = [u_elem.get(3) - u_elem.get(0), u_elem.get(4) - u_elem.get(1), u_elem.get(5) - u_elem.get(2)];
        let n = k * (c[0] * du[0] + c[1] * du[1] + c[2] * du[2]);
        ElementResult::Bar { n }
    }
}
// #endregion 🔖Bar3

// #region 🔖Frame3
/// 🧮 Places a 4x4 bending block into `k` at the given DOF indices (used for both the y- and z-bending
/// planes, which are decoupled from each other and from axial/torsion).
fn set_bend_block(k: &mut MatD, idx: [usize; 4], block: [[f64; 4]; 4]) {
    for (bi, &gi) in idx.iter().enumerate() {
        for (bj, &gj) in idx.iter().enumerate() {
            k.set(gi, gj, block[bi][bj]);
        }
    }
}

/// 🏗️ Two-node 3D Euler-Bernoulli frame element with torsion — full 6-DOF-per-node member. Local x
/// runs node-a to node-b; local y/z are built from a reference "up" vector and rotated by `roll`
/// (radians) about local x. `stiffness_global`/`recover` rotate the decoupled axial/torsion/biaxial
/// bending local stiffness into global coordinates via the block-diagonal transform `T`.
pub struct Frame3 {
    pub id: String,
    pub node_a: String,
    pub node_b: String,
    pub e: f64,
    pub g: f64,
    pub a: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
    pub roll: f64,
}

impl Frame3 {
    /// 🧭 Builds the member length, local 12x12 stiffness, and the 12x12 global<->local block-diagonal
    /// rotation `T` (four `R^T` 3x3 blocks) shared by `stiffness_global` and `recover`.
    fn local_system(&self, ctx: &ElementContext) -> (f64, MatD, MatD) {
        let d = vec3d_sub(ctx.positions[1], ctx.positions[0]);
        let l = vec3d_length(d);
        let cx = vec3d_normalize(d);
        let reference = if cx[2].abs() > 0.99 { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] };
        let y_unrot = vec3d_normalize(vec3d_cross(reference, cx));
        let z_unrot = vec3d_cross(cx, y_unrot);
        let (sin_r, cos_r) = self.roll.sin_cos();
        let local_y = [
            y_unrot[0] * cos_r + z_unrot[0] * sin_r,
            y_unrot[1] * cos_r + z_unrot[1] * sin_r,
            y_unrot[2] * cos_r + z_unrot[2] * sin_r,
        ];
        let local_z = [
            z_unrot[0] * cos_r - y_unrot[0] * sin_r,
            z_unrot[1] * cos_r - y_unrot[1] * sin_r,
            z_unrot[2] * cos_r - y_unrot[2] * sin_r,
        ];
        let rt = Mat3d::from_axes(cx, local_y, local_z).transpose();
        let mut t = MatD::zeros(12, 12);
        for offset in [0usize, 3, 6, 9] {
            for row in 0..3 {
                for col in 0..3 {
                    t.set(offset + row, offset + col, rt.cols[col][row]);
                }
            }
        }
        (l, self.local_stiffness(l), t)
    }

    /// 🧮 Decoupled local 12x12 stiffness: axial, torsion, and biaxial (y/z) Euler-Bernoulli bending.
    fn local_stiffness(&self, l: f64) -> MatD {
        let mut k = MatD::zeros(12, 12);
        let l2 = l * l;
        let ax = self.e * self.a / l;
        k.set(0, 0, ax);
        k.set(0, 6, -ax);
        k.set(6, 0, -ax);
        k.set(6, 6, ax);
        let tor = self.g * self.j / l;
        k.set(3, 3, tor);
        k.set(3, 9, -tor);
        k.set(9, 3, -tor);
        k.set(9, 9, tor);
        let bz = self.e * self.iz / l;
        set_bend_block(
            &mut k,
            [1, 5, 7, 11],
            [
                [12.0 * bz / l2, 6.0 * bz / l, -12.0 * bz / l2, 6.0 * bz / l],
                [6.0 * bz / l, 4.0 * bz, -6.0 * bz / l, 2.0 * bz],
                [-12.0 * bz / l2, -6.0 * bz / l, 12.0 * bz / l2, -6.0 * bz / l],
                [6.0 * bz / l, 2.0 * bz, -6.0 * bz / l, 4.0 * bz],
            ],
        );
        let by = self.e * self.iy / l;
        set_bend_block(
            &mut k,
            [2, 4, 8, 10],
            [
                [12.0 * by / l2, -6.0 * by / l, -12.0 * by / l2, -6.0 * by / l],
                [-6.0 * by / l, 4.0 * by, 6.0 * by / l, 2.0 * by],
                [-12.0 * by / l2, 6.0 * by / l, 12.0 * by / l2, 6.0 * by / l],
                [-6.0 * by / l, 2.0 * by, 6.0 * by / l, 4.0 * by],
            ],
        );
        k
    }
}

impl Element for Frame3 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        vec![self.node_a.clone(), self.node_b.clone()]
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (_l, k_local, t) = self.local_system(ctx);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let (l, k_local, t) = self.local_system(ctx);
        let u_loc = t.mul_vec(u_elem);
        let f = k_local.mul_vec(&u_loc);
        let n = -f.get(0);
        let v1 = f.get(2);
        let m1 = f.get(4);
        let stations = (0..11)
            .map(|i| {
                let x = l * (i as f64) / 10.0;
                BeamStation { x, n, v: v1, m: m1 + v1 * x }
            })
            .collect();
        ElementResult::Beam { stations }
    }
}
// #endregion 🔖Frame3

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{solve_linear_static, Model, Node, NodalLoad, Support};

    /// 🪵 Headless axial elongation check along an arbitrary (non-axis-aligned) 3D direction.
    #[test]
    fn bar3_axial_matches_hand_calc_on_skew_member() {
        let (e, a) = (200e9, 0.001);
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [3.0, 4.0, 0.0]; // length 5, direction (0.6, 0.8, 0.0)
        let l = 5.0;
        let p = 2000.0;
        let model = Model {
            nodes: vec![Node { id: "a".into(), pos: p1 }, Node { id: "b".into(), pos: p2 }],
            elements: vec![Box::new(Bar3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e, a })],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Tz] }],
            nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p * 0.6 }, NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: p * 0.8 }],
            member_loads: vec![],
        };
        let result = solve_linear_static(&model).expect("solves");
        let ElementResult::Bar { n } = result.elements[0].1 else { panic!("expected bar") };
        assert!((n - p).abs() / p < 1e-6, "axial force {n} vs expected {p}");
        let expected_elongation = p * l / (e * a);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        let actual_elongation = (b.values[Dof::Tx.index()] * 0.6 + b.values[Dof::Ty.index()] * 0.8).abs();
        assert!((actual_elongation - expected_elongation).abs() / expected_elongation < 1e-6);
    }

    /// 🌀 Rigid-body test: a pure 3D translation must produce zero internal force on a `Frame3`.
    #[test]
    fn frame3_rigid_translation_gives_zero_force() {
        let frame = Frame3 { id: "e1".into(), node_a: "a".into(), node_b: "b".into(), e: 210e9, g: 80.77e9, a: 0.005, iy: 1e-5, iz: 1e-5, j: 1e-6, roll: 0.0 };
        let ctx = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 1.0, 0.5]] };
        let ke = frame.stiffness_global(&ctx);
        let rigid = VecD::from_vec(vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0]);
        let f = ke.mul_vec(&rigid);
        for i in 0..12 {
            assert!(f.get(i).abs() < 1e-6, "rigid-body force[{i}] = {}", f.get(i));
        }
    }
}
// #endregion 🔖Tests
