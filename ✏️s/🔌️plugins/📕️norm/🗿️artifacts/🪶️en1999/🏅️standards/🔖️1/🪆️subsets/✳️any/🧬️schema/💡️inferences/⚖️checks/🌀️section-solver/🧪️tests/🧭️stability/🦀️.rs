//! 🧪️ Stability constants against the `sectionproperties` Wagner coefficient, the shear centre and the 3x3 eigenproblem of `🧰️scripts/en1999-wave4/wagner_ref.py` (the numbers below are its output; E = 70 000 MPa, G = 27 000 MPa).

use super::stability::*;

const E: f64 = 70_000.0e6;
const G: f64 = 27_000.0e6;
const IT: f64 = 6.650468399633888e-08;
const IW: f64 = 3.787856412407533e-11;
const Y_SC: f64 = -1.225768314176551e-07;
const Z_SC: f64 = 0.02658624991493054;

fn tee() -> Vec<(f64, f64)> {
    vec![(-0.005, 0.0), (0.005, 0.0), (0.005, 0.1), (0.05, 0.1), (0.05, 0.11), (-0.05, 0.11), (-0.05, 0.1), (-0.005, 0.1)]
}

fn angle() -> Vec<(f64, f64)> {
    vec![(0.0, 0.0), (0.1, 0.0), (0.1, 0.008), (0.008, 0.008), (0.008, 0.06), (0.0, 0.06)]
}

fn close(a: f64, b: f64, tol: f64) { assert!((a / b - 1.0).abs() < tol, "{a} against {b}"); }

#[test]
fn the_wagner_moment_of_a_tee_equals_the_sectionproperties_coefficient() {
    let f = frames(&tee(), &[], Y_SC, Z_SC).expect("frames");
    assert!(f.near.theta.abs() < 1e-9);
    close(-2.0 * z_j(&f.strong, true), -0.072447, 2e-5);
    close(-2.0 * z_j(&f.strong, false), 0.072447, 2e-5);
}

#[test]
fn the_tee_is_stronger_with_the_flange_in_compression() {
    let f = frames(&tee(), &[], Y_SC, Z_SC).expect("frames");
    close(m_cr_general(1.0, E, G, &f.strong, IT, IW, 3.0, true), 13371.213486552744, 1e-6);
    close(m_cr_general(1.0, E, G, &f.strong, IT, IW, 3.0, false), 8690.470152568918, 1e-6);
}

#[test]
fn a_doubly_symmetric_section_has_no_wagner_effect() {
    let i = vec![(-0.05, -0.06), (0.05, -0.06), (0.05, -0.052), (0.003, -0.052), (0.003, 0.052), (0.05, 0.052), (0.05, 0.06), (-0.05, 0.06), (-0.05, 0.052), (-0.003, 0.052), (-0.003, -0.052), (-0.05, -0.052)];
    let f = frames(&i, &[], 0.0, 0.0).expect("frames");
    assert!(z_j(&f.strong, true).abs() < 1e-12);
    let (it, iw, l, c1) = (4.1e-8, 4.2e-9, 4.0, 1.3);
    let pi2 = std::f64::consts::PI.powi(2);
    let closed = c1 * pi2 * E * f.strong.i_z / (l * l) * (iw / f.strong.i_z + l * l * G * it / (pi2 * E * f.strong.i_z)).sqrt();
    close(m_cr_general(c1, E, G, &f.strong, it, iw, l, true), closed, 1e-9);
}

#[test]
fn the_torsional_flexural_load_of_a_tee_is_the_smallest_root_of_the_coupled_cubic() {
    let f = frames(&tee(), &[], Y_SC, Z_SC).expect("frames");
    let (n_t, n_tf) = n_cr_tf(E, G, 0.002, &f.near, IT, IW, 3.0, 3.0, 3.0);
    close(n_t, 780361.3364515827, 1e-6);
    close(n_tf, 62917.19325769038, 1e-6);
    assert!(n_tf < n_t);
}

#[test]
fn an_angle_is_rotated_to_its_principal_axes() {
    let f = frames(&angle(), &[], 0.0, 0.0).expect("frames");
    assert!((f.near.theta.to_degrees() + 19.946).abs() < 0.01, "{}", f.near.theta.to_degrees());
    close(f.strong.i_y, 1.385e-6, 2e-3);
    close(f.strong.i_z, 2.07e-7, 2e-3);
}

#[test]
fn a_zero_length_is_an_infinite_lateral_buckling_moment() {
    let f = frames(&tee(), &[], Y_SC, Z_SC).expect("frames");
    assert!(m_cr_general(1.0, E, G, &f.strong, IT, IW, 0.0, true).is_infinite());
    assert!(frames(&[(0.0, 0.0), (1.0, 0.0)], &[], 0.0, 0.0).is_none());
}
