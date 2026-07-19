//! 🧭 Shared element-formulation toolkit: Gauss quadrature rules, isoparametric shape functions and
//! their parametric derivatives, Jacobians, plane/solid B-matrices, and constitutive D-matrices —
//! consumed by the continuum/plate/shell elements in `elements2d`/`elements3d`.

use mathematical_algebra::MatD;

// #region 🔖Quadrature
/// 🎯 1D Gauss-Legendre points/weights on `[-1,1]`, `n = 1..=4`.
pub fn gauss_1d(n: usize) -> Vec<(f64, f64)> {
    match n {
        1 => vec![(0.0, 2.0)],
        2 => {
            let g = 1.0 / 3.0f64.sqrt();
            vec![(-g, 1.0), (g, 1.0)]
        }
        3 => {
            let g = (3.0 / 5.0f64).sqrt();
            vec![(-g, 5.0 / 9.0), (0.0, 8.0 / 9.0), (g, 5.0 / 9.0)]
        }
        4 => {
            let x1 = 0.3399810435848563;
            let x2 = 0.8611363115940526;
            let w1 = 0.6521451548625461;
            let w2 = 0.3478548451374538;
            vec![(-x2, w2), (-x1, w1), (x1, w1), (x2, w2)]
        }
        _ => panic!("gauss_1d: unsupported order {n}, only 1..=4 are implemented"),
    }
}

/// 🎯 Triangle (area-coordinate) Gauss rules on the UNIT triangle (vertices `(0,0),(1,0),(0,1)`,
/// area 0.5). `n=1`: centroid, weight 0.5. `n=3`: standard 3-point rule (weights sum to 0.5).
/// `n=7`: 7-point Gauss-Hammer rule (weights sum to 0.5). Points/weights are in the triangle's
/// PARAMETRIC `(xi, eta)` coords, ready to use directly as `∫f dA ≈ Σ f(xi_i,eta_i) * weight_i`.
pub fn gauss_tri(n: usize) -> Vec<(f64, f64, f64)> {
    match n {
        1 => vec![(1.0 / 3.0, 1.0 / 3.0, 0.5)],
        3 => vec![(1.0 / 6.0, 1.0 / 6.0, 1.0 / 6.0), (2.0 / 3.0, 1.0 / 6.0, 1.0 / 6.0), (1.0 / 6.0, 2.0 / 3.0, 1.0 / 6.0)],
        7 => {
            let a = (6.0 - 15.0f64.sqrt()) / 21.0;
            let b = (6.0 + 15.0f64.sqrt()) / 21.0;
            let w1 = (155.0 - 15.0f64.sqrt()) / 2400.0;
            let w2 = (155.0 + 15.0f64.sqrt()) / 2400.0;
            vec![
                (1.0 / 3.0, 1.0 / 3.0, 9.0 / 80.0),
                (a, a, w1),
                (1.0 - 2.0 * a, a, w1),
                (a, 1.0 - 2.0 * a, w1),
                (b, b, w2),
                (1.0 - 2.0 * b, b, w2),
                (b, 1.0 - 2.0 * b, w2),
            ]
        }
        _ => panic!("gauss_tri: unsupported order {n}, only 1, 3, 7 are implemented"),
    }
}

/// 🎯 Tensor-product Gauss rule on the reference square `[-1,1]x[-1,1]`, `n x n` points.
pub fn gauss_quad(n: usize) -> Vec<(f64, f64, f64)> {
    let rule = gauss_1d(n);
    let mut out = Vec::with_capacity(n * n);
    for &(xi, w_xi) in &rule {
        for &(eta, w_eta) in &rule {
            out.push((xi, eta, w_xi * w_eta));
        }
    }
    out
}
// #endregion 🔖Quadrature

// #region 🔖ShapeFunctions
/// 📐 Tri3 (linear) shape functions and PARAMETRIC derivatives at `(xi, eta)`:
/// `N = [1-xi-eta, xi, eta]`.
pub fn shape_tri3(xi: f64, eta: f64) -> ([f64; 3], [[f64; 2]; 3]) {
    let n = [1.0 - xi - eta, xi, eta];
    let dn = [[-1.0, -1.0], [1.0, 0.0], [0.0, 1.0]];
    (n, dn)
}

/// 📐 Tri6 (quadratic) shape functions. Node order: 3 corners `[n0,n1,n2]` at `(0,0),(1,0),(0,1)`,
/// then 3 mid-edge nodes `[n01,n12,n20]` where mid-edge `ij` sits at the midpoint between corner
/// `i` and corner `j` — i.e. the full node order is `[n0,n1,n2,n01,n12,n20]`. `mesh.rs`'s
/// quadratic-promotion code must number Tri6 nodes to match this exact convention.
pub fn shape_tri6(xi: f64, eta: f64) -> ([f64; 6], [[f64; 2]; 6]) {
    let l1 = 1.0 - xi - eta;
    let l2 = xi;
    let l3 = eta;
    let n = [l1 * (2.0 * l1 - 1.0), l2 * (2.0 * l2 - 1.0), l3 * (2.0 * l3 - 1.0), 4.0 * l1 * l2, 4.0 * l2 * l3, 4.0 * l3 * l1];
    let dn = [
        [1.0 - 4.0 * l1, 1.0 - 4.0 * l1],
        [4.0 * l2 - 1.0, 0.0],
        [0.0, 4.0 * l3 - 1.0],
        [4.0 * (l1 - l2), -4.0 * l2],
        [4.0 * l3, 4.0 * l2],
        [-4.0 * l3, 4.0 * (l1 - l3)],
    ];
    (n, dn)
}

/// 📐 Quad4 (bilinear) on reference square `[-1,1]^2`, node order counterclockwise from `(-1,-1)`.
pub fn shape_quad4(xi: f64, eta: f64) -> ([f64; 4], [[f64; 2]; 4]) {
    let n = [0.25 * (1.0 - xi) * (1.0 - eta), 0.25 * (1.0 + xi) * (1.0 - eta), 0.25 * (1.0 + xi) * (1.0 + eta), 0.25 * (1.0 - xi) * (1.0 + eta)];
    let dn = [
        [-0.25 * (1.0 - eta), -0.25 * (1.0 - xi)],
        [0.25 * (1.0 - eta), -0.25 * (1.0 + xi)],
        [0.25 * (1.0 + eta), 0.25 * (1.0 + xi)],
        [-0.25 * (1.0 + eta), 0.25 * (1.0 - xi)],
    ];
    (n, dn)
}

/// 📐 Quad8 (serendipity quadratic), node order: 4 corners CCW from `(-1,-1)`, then 4 mid-edges in
/// order (bottom `(0,-1)`, right `(1,0)`, top `(0,1)`, left `(-1,0)`).
pub fn shape_quad8(xi: f64, eta: f64) -> ([f64; 8], [[f64; 2]; 8]) {
    let n = [
        -0.25 * (1.0 - xi) * (1.0 - eta) * (1.0 + xi + eta),
        0.25 * (1.0 + xi) * (1.0 - eta) * (xi - eta - 1.0),
        0.25 * (1.0 + xi) * (1.0 + eta) * (xi + eta - 1.0),
        0.25 * (1.0 - xi) * (1.0 + eta) * (eta - xi - 1.0),
        0.5 * (1.0 - xi * xi) * (1.0 - eta),
        0.5 * (1.0 + xi) * (1.0 - eta * eta),
        0.5 * (1.0 - xi * xi) * (1.0 + eta),
        0.5 * (1.0 - xi) * (1.0 - eta * eta),
    ];
    let dn = [
        [0.25 * (1.0 - eta) * (2.0 * xi + eta), 0.25 * (1.0 - xi) * (xi + 2.0 * eta)],
        [0.25 * (1.0 - eta) * (2.0 * xi - eta), -0.25 * (1.0 + xi) * (xi - 2.0 * eta)],
        [0.25 * (1.0 + eta) * (2.0 * xi + eta), 0.25 * (1.0 + xi) * (xi + 2.0 * eta)],
        [0.25 * (1.0 + eta) * (2.0 * xi - eta), 0.25 * (1.0 - xi) * (2.0 * eta - xi)],
        [-xi * (1.0 - eta), -0.5 * (1.0 - xi * xi)],
        [0.5 * (1.0 - eta * eta), -eta * (1.0 + xi)],
        [-xi * (1.0 + eta), 0.5 * (1.0 - xi * xi)],
        [-0.5 * (1.0 - eta * eta), -eta * (1.0 - xi)],
    ];
    (n, dn)
}
// #endregion 🔖ShapeFunctions

// #region 🔖Jacobian
/// 🧮 Jacobian matrix (2x2 as `[[dx/dxi, dx/deta],[dy/dxi, dy/deta]]`), its determinant, and physical
/// `(x,y)` shape-function derivatives (`dN/dx, dN/dy`) computed from parametric derivatives via
/// `J^-1`, given nodal `(x,y)` coordinates and the parametric derivatives from a `shape_*` function.
pub fn jacobian_2d(coords: &[[f64; 2]], d_n_param: &[[f64; 2]]) -> ([[f64; 2]; 2], f64, Vec<[f64; 2]>) {
    let mut dx_dxi = 0.0;
    let mut dx_deta = 0.0;
    let mut dy_dxi = 0.0;
    let mut dy_deta = 0.0;
    for (p, dn) in coords.iter().zip(d_n_param.iter()) {
        dx_dxi += dn[0] * p[0];
        dx_deta += dn[1] * p[0];
        dy_dxi += dn[0] * p[1];
        dy_deta += dn[1] * p[1];
    }
    let j = [[dx_dxi, dx_deta], [dy_dxi, dy_deta]];
    let det_j = dx_dxi * dy_deta - dx_deta * dy_dxi;
    let d_n_xy = d_n_param
        .iter()
        .map(|dn| [(dy_deta * dn[0] - dy_dxi * dn[1]) / det_j, (-dx_deta * dn[0] + dx_dxi * dn[1]) / det_j])
        .collect();
    (j, det_j, d_n_xy)
}
// #endregion 🔖Jacobian

// #region 🔖BMatrix
/// 🧮 Plane-stress/strain B-matrix (3 x 2n) from physical shape derivatives, standard ordering
/// `[du/dx, dv/dy, du/dy+dv/dx]` rows, node-major `[u_i, v_i]` column order.
pub fn b_matrix_plane(d_n_xy: &[[f64; 2]]) -> MatD {
    let mut b = MatD::zeros(3, d_n_xy.len() * 2);
    for (i, dn) in d_n_xy.iter().enumerate() {
        let (dx, dy) = (dn[0], dn[1]);
        b.set(0, 2 * i, dx);
        b.set(1, 2 * i + 1, dy);
        b.set(2, 2 * i, dy);
        b.set(2, 2 * i + 1, dx);
    }
    b
}
// #endregion 🔖BMatrix

// #region 🔖DMatrix
/// 🧱 Plane-stress constitutive matrix (3x3) from `E`, `nu`:
/// `E/(1-nu^2) * [[1,nu,0],[nu,1,0],[0,0,(1-nu)/2]]`.
pub fn d_matrix_plane_stress(e: f64, nu: f64) -> MatD {
    let factor = e / (1.0 - nu * nu);
    let mut d = MatD::zeros(3, 3);
    d.set(0, 0, factor);
    d.set(0, 1, factor * nu);
    d.set(1, 0, factor * nu);
    d.set(1, 1, factor);
    d.set(2, 2, factor * (1.0 - nu) / 2.0);
    d
}

/// 🧱 Plane-strain constitutive matrix (3x3):
/// `E/((1+nu)(1-2nu)) * [[1-nu,nu,0],[nu,1-nu,0],[0,0,(1-2nu)/2]]`.
pub fn d_matrix_plane_strain(e: f64, nu: f64) -> MatD {
    let factor = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
    let mut d = MatD::zeros(3, 3);
    d.set(0, 0, factor * (1.0 - nu));
    d.set(0, 1, factor * nu);
    d.set(1, 0, factor * nu);
    d.set(1, 1, factor * (1.0 - nu));
    d.set(2, 2, factor * (1.0 - 2.0 * nu) / 2.0);
    d
}
// #endregion 🔖DMatrix

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauss_1d_weights_sum_to_two() {
        for n in 1..=4 {
            let sum: f64 = gauss_1d(n).iter().map(|(_, w)| w).sum();
            assert!((sum - 2.0).abs() < 1e-12, "n={n} sum={sum}");
        }
    }

    #[test]
    fn gauss_tri_weights_sum_to_half() {
        for n in [1, 3, 7] {
            let sum: f64 = gauss_tri(n).iter().map(|(_, _, w)| w).sum();
            assert!((sum - 0.5).abs() < 1e-12, "n={n} sum={sum}");
        }
    }

    #[test]
    fn gauss_quad_weights_sum_to_four() {
        for n in 1..=4 {
            let sum: f64 = gauss_quad(n).iter().map(|(_, _, w)| w).sum();
            assert!((sum - 4.0).abs() < 1e-9, "n={n} sum={sum}");
        }
    }

    #[test]
    fn shape_tri3_partition_of_unity_and_node_values() {
        let (n, _) = shape_tri3(0.2, 0.3);
        assert!((n[0] + n[1] + n[2] - 1.0).abs() < 1e-12);
        let nodes = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
        for (i, &(xi, eta)) in nodes.iter().enumerate() {
            let (n, _) = shape_tri3(xi, eta);
            for (j, &nj) in n.iter().enumerate() {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((nj - expected).abs() < 1e-12, "tri3 node {i} shape {j} = {nj}");
            }
        }
    }

    #[test]
    fn shape_tri6_partition_of_unity_and_node_values() {
        let (n, _) = shape_tri6(0.15, 0.35);
        let sum: f64 = n.iter().sum();
        assert!((sum - 1.0).abs() < 1e-12);
        let nodes = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.5, 0.0), (0.5, 0.5), (0.0, 0.5)];
        for (i, &(xi, eta)) in nodes.iter().enumerate() {
            let (n, _) = shape_tri6(xi, eta);
            for (j, &nj) in n.iter().enumerate() {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((nj - expected).abs() < 1e-10, "tri6 node {i} shape {j} = {nj}");
            }
        }
    }

    #[test]
    fn shape_quad4_partition_of_unity_and_node_values() {
        let (n, _) = shape_quad4(0.3, -0.4);
        let sum: f64 = n.iter().sum();
        assert!((sum - 1.0).abs() < 1e-12);
        let nodes = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
        for (i, &(xi, eta)) in nodes.iter().enumerate() {
            let (n, _) = shape_quad4(xi, eta);
            for (j, &nj) in n.iter().enumerate() {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((nj - expected).abs() < 1e-12, "quad4 node {i} shape {j} = {nj}");
            }
        }
    }

    #[test]
    fn shape_quad8_partition_of_unity_and_node_values() {
        let (n, _) = shape_quad8(0.25, -0.6);
        let sum: f64 = n.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
        let nodes = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)];
        for (i, &(xi, eta)) in nodes.iter().enumerate() {
            let (n, _) = shape_quad8(xi, eta);
            for (j, &nj) in n.iter().enumerate() {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((nj - expected).abs() < 1e-10, "quad8 node {i} shape {j} = {nj}");
            }
        }
    }

    #[test]
    fn jacobian_of_axis_aligned_rectangle_is_diagonal() {
        let (lx, ly) = (4.0, 6.0);
        let coords = [[0.0, 0.0], [lx, 0.0], [lx, ly], [0.0, ly]];
        let (_, d_n) = shape_quad4(0.3, -0.2);
        let (j, det_j, _) = jacobian_2d(&coords, &d_n);
        assert!(j[0][1].abs() < 1e-12);
        assert!(j[1][0].abs() < 1e-12);
        assert!((j[0][0] - lx / 2.0).abs() < 1e-12);
        assert!((j[1][1] - ly / 2.0).abs() < 1e-12);
        assert!((det_j - lx * ly / 4.0).abs() < 1e-9);
    }
}
// #endregion 🔖Tests
