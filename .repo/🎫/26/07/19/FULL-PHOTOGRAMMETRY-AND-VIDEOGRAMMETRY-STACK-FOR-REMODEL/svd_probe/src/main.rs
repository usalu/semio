use mathematical_algebra::{svd_nullvector, MatD, VecD};

fn main() {
    // Simulate the EXACT shape fit_fundamental_dlt builds for a minimal 8-sample RANSAC draw: 8x9.
    let f_true = VecD::from_vec(vec![0.1, 0.3, -0.2, 0.05, 0.4, -0.1, 0.7, -0.3, 0.2]);
    let n = f_true.norm2();
    let f_true = f_true.scale(1.0 / n);

    let mut state = 999u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((state >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    };
    let mut a = MatD::zeros(8, 9);
    for row in 0..8 {
        let r = VecD::from_vec((0..9).map(|_| next()).collect());
        let proj = r.dot(&f_true);
        let ortho = r.sub(&f_true.scale(proj));
        for c in 0..9 {
            a.set(row, c, ortho.get(c));
        }
    }
    println!("residual A*f_true (should be ~0): {:?}", a.mul_vec(&f_true).0);
    match svd_nullvector(&a) {
        Ok(v) => {
            println!("A * nullvector(v) = {:?}", a.mul_vec(&v).0);
            println!("dot(v, f_true) = {}", v.dot(&f_true));
        }
        Err(e) => println!("err: {e}"),
    }
}
