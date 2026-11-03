//! 🌀️ Linear buckling: `(K + α Kg(N)) φ = 0` with the geometric stiffness of the member axial forces of a load case.

use crate::elements::zero;
use crate::linalg::{generalized_dense, generalized_subspace, DENSE_LIMIT};
use crate::*;

pub(crate) fn solve_pairs(a: &Analysis<'_>, b: &Skyline, count: usize, control: &Control<'_>) -> MechanicsResult<Vec<(f64, Vec<f64>)>> {
    let pairs = if a.system.n_free <= DENSE_LIMIT {
        generalized_dense(&a.stiffness.to_dense(), &b.to_dense(), count, control)
    } else {
        generalized_subspace(&a.stiffness, &a.factor, b, count, control)
    }
    .map_err(|e| a.system.unstable(e, a.model))?;
    if pairs.is_empty() {
        return Err(MechanicsError::NoPositiveEigenvalue);
    }
    Ok(pairs)
}

pub(crate) fn unit_shape(a: &Analysis<'_>, free: &[f64]) -> Vec<NodeDisplacement> {
    let mut shape = a.system.displacements(a.model, &a.system.expand(free));
    let peak = |translation: bool| shape.iter().flat_map(|s| s.u[..if translation { 3 } else { 6 }].iter()).fold(0.0_f64, |m, v| m.max(v.abs()));
    let scale = if peak(true) > 1e-9 * peak(false) { peak(true) } else { peak(false) };
    if scale > 0.0 {
        shape.iter_mut().for_each(|s| s.u.iter_mut().for_each(|v| *v /= scale));
    }
    shape
}

impl Analysis<'_> {
    /// 🧨️ The `count` smallest positive load factors `αcr` of a load case with their mode shapes (max translation normalised to 1).
    pub fn buckling(&self, case: &LoadCase, count: usize, control: &Control<'_>) -> MechanicsResult<BucklingResult> {
        let raw = self.solve_raw(case, control)?;
        let n = self.axial_forces(&raw);
        let b = self.system.assemble(control, "geometric", |m| {
            let g = self.system.elements[m].geometric(n[m]);
            let mut o = zero();
            for i in 0..12 {
                for j in 0..12 {
                    o[i][j] = -g[i][j];
                }
            }
            o
        })?;
        let pairs = solve_pairs(self, &b, count.max(1), control)?;
        Ok(BucklingResult { case: case.id.clone(), modes: pairs.into_iter().map(|(alpha_cr, x)| BucklingMode { alpha_cr, shape: unit_shape(self, &x) }).collect() })
    }
}
