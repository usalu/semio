//! 🔗️ Joint-aware frame analysis: the frame solutions of `frame_eigen` with the semi-rigid bar ends (`releaseStart`/`releaseEnd` = `semiRigid`) sprung by the secant stiffness S_j of the end plate joint of their member (EN 1993-1-8 5.1.2, EN 1993-1-1 5.2.1), under the governing ultimate combination of the frame unless a load vector is given. See [EN 1993-1-8](https://eurocodes.jrc.ec.europa.eu/).

use super::frame_envelope;
use semio_s_artifact_norm_en1993_frame_eigen::{FrameEigen, FrameFault, FrameLoads, Imperfections, SecondOrder};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

/// 🌀️ The buckling modes of the frame with semi-rigid joints under the governing ultimate combination, without progress or cancellation.
pub fn solve(document: &En1993Snapshot) -> Result<FrameEigen, FrameFault> {
    frame_envelope::eigen(document, &frame_envelope::governing_loads(document)?.loads)
}

/// 🌀️ The sway load factor α_cr of the frame with semi-rigid joints (EN 1993-1-1 5.2.1) under the governing ultimate combination.
pub fn alpha_cr(document: &En1993Snapshot) -> Result<f64, FrameFault> {
    solve(document).map(|e| e.governing())
}

/// 🔁️ The second-order solution of the frame with semi-rigid joints under the load vector `loads` and the given imperfection loads.
pub fn solve_with_imperfections(document: &En1993Snapshot, loads: &FrameLoads, imperfections: &Imperfections) -> Result<SecondOrder, FrameFault> {
    frame_envelope::second_order(document, loads, imperfections)
}

/// 🔁️ The second-order solution of the frame with semi-rigid joints under the governing ultimate combination without imperfections.
pub fn solve_second_order(document: &En1993Snapshot) -> Result<SecondOrder, FrameFault> {
    solve_with_imperfections(document, &frame_envelope::governing_loads(document)?.loads, &Imperfections::default())
}
