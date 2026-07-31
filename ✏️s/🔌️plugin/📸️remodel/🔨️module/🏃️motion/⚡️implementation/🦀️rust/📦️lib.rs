//! 🏃️ Videogrammetry: multi-frame tracking, 3D trajectories, kinematics, vibration and modal analysis, camera sync, stabilization and non-rigid capture.

pub use mathematical_lie::Se3;
pub use remodel_camera::{CameraPose, Intrinsics};
pub use remodel_image::{ImageGray, Pyramid};

use mathematical_algebra::{solve_llsq, MatD, VecD};
use mathematical_optimize::{levenberg_marquardt, numeric_jacobian, LeastSquaresProblem, LmConfig};
use mathematical_spatial::KdTree;
use remodel_feature::{forward_backward_prune, klt_track, shi_tomasi_grid};

// #region 🔖️Vec3Helpers
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = sub3(a, b);
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

fn mat3_mul(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}
// #endregion 🔖️Vec3Helpers

// #region 🔖️Track2d
/// 🧵️ One 2D feature's optical-flow trajectory across frames: track identity plus `(frame_idx, x, y)`
/// samples in the order they were observed. A track that fails forward-backward consistency is dropped
/// from further extension but keeps its recorded history.
#[derive(Clone, Debug, PartialEq)]
pub struct Track2d {
    pub id: u32,
    pub samples: Vec<(u32, f32, f32)>,
}

const TRACK2D_FB_MAX_ERROR: f32 = 1.0;

/// 🎥️ Stateful multi-frame 2D point tracker: advances every live [`Track2d`] one frame at a time via
/// pyramidal KLT ([`klt_track`] + [`forward_backward_prune`]), drops any track whose point becomes
/// invalid, and refills spatial coverage by re-detecting fresh corners ([`shi_tomasi_grid`]) in grid
/// cells that have too few live tracks. Dropped tracks stay in [`Tracker2d::tracks`]'s history (so
/// downstream trajectory/kinematics code keeps their full sample history) but are never extended again.
#[derive(Clone, Debug, Default)]
pub struct Tracker2d {
    tracks: Vec<Track2d>,
    live: Vec<usize>,
    next_id: u32,
}

impl Tracker2d {
    /// 🆕️ An empty tracker with no live or historical tracks.
    pub fn new() -> Self {
        Self { tracks: Vec::new(), live: Vec::new(), next_id: 0 }
    }

    /// 👣️ Advances every live track from `pyr_prev` into `pyr_curr`, drops points that fail forward-backward
    /// consistency or leave the image, then re-detects new corners (via [`shi_tomasi_grid`] over a
    /// `redetect_grid`-pixel cell grid on `pyr_curr`'s finest level) in any cell holding fewer than
    /// `redetect_per_cell` live tracks, spawning a fresh [`Track2d`] per new corner. Returns the full
    /// (live + historical) track list, same as [`Tracker2d::tracks`].
    #[allow(clippy::too_many_arguments, reason = "one argument per physically distinct KLT/redetect tuning knob; a config struct would just move the same 7 fields one level down for this single call site")]
    pub fn step(&mut self, pyr_prev: &Pyramid, pyr_curr: &Pyramid, frame_idx: u32, window_radius: i32, max_iters: usize, redetect_grid: u32, redetect_per_cell: usize) -> &[Track2d] {
        if pyr_prev.levels.is_empty() || pyr_curr.levels.is_empty() {
            return &self.tracks;
        }
        let points: Vec<(f32, f32)> = self
            .live
            .iter()
            .map(|&i| {
                let &(_, x, y) = self.tracks[i].samples.last().expect("live index always has >=1 sample");
                (x, y)
            })
            .collect();
        let mut tracked = klt_track(pyr_prev, pyr_curr, &points, window_radius, max_iters);
        forward_backward_prune(pyr_prev, pyr_curr, &points, &mut tracked, window_radius, max_iters, TRACK2D_FB_MAX_ERROR);

        let mut still_live = Vec::with_capacity(self.live.len());
        for (&idx, tp) in self.live.iter().zip(tracked.iter()) {
            if tp.valid {
                self.tracks[idx].samples.push((frame_idx, tp.x, tp.y));
                still_live.push(idx);
            }
        }
        self.live = still_live;

        let finest = &pyr_curr.levels[0];
        let cell = redetect_grid.max(1);
        let cells_x = finest.width.div_ceil(cell).max(1);
        let cells_y = finest.height.div_ceil(cell).max(1);
        let mut counts = vec![0usize; (cells_x * cells_y) as usize];
        for &idx in &self.live {
            let &(_, x, y) = self.tracks[idx].samples.last().expect("live index always has >=1 sample");
            counts[bucket_index(x, y, cell, cells_x, cells_y)] += 1;
        }
        for (x, y, _score) in shi_tomasi_grid(finest, cell, redetect_per_cell) {
            let bucket = bucket_index(x as f32, y as f32, cell, cells_x, cells_y);
            if counts[bucket] >= redetect_per_cell {
                continue;
            }
            counts[bucket] += 1;
            let id = self.next_id;
            self.next_id += 1;
            self.tracks.push(Track2d { id, samples: vec![(frame_idx, x as f32, y as f32)] });
            self.live.push(self.tracks.len() - 1);
        }
        &self.tracks
    }

    /// 📜️ Every track this tracker has ever produced (live or dropped), in creation order.
    pub fn tracks(&self) -> &[Track2d] {
        &self.tracks
    }
}

fn bucket_index(x: f32, y: f32, cell: u32, cells_x: u32, cells_y: u32) -> usize {
    let bx = ((x.max(0.0) as u32) / cell).min(cells_x.saturating_sub(1));
    let by = ((y.max(0.0) as u32) / cell).min(cells_y.saturating_sub(1));
    (by * cells_x + bx) as usize
}
// #endregion 🔖️Track2d

// #region 🔖️Mot
/// 📍️ One raw 2D detection to associate against live tracks, with an optional external id hint (unused by
/// the gating/association logic itself, carried through purely as caller metadata).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Detection {
    pub x: f32,
    pub y: f32,
    pub id_hint: Option<u32>,
}

/// 🚦️ One multi-object track's constant-velocity state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackState {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub age: u32,
    pub missed: u32,
}

const MOT_ALPHA: f32 = 0.6;
const MOT_MAX_MISSED: u32 = 5;

/// 🕵️ Multi-object α–β tracker over 2D detections: predicts every track forward at constant velocity,
/// associates detections by gated nearest-cost greedy matching, and spawns/prunes tracks for
/// unmatched detections/tracks. `mathematical_graph_matching` (this crate's originally-planned
/// max-weight-matching dependency) is an unimplemented one-line stub with no public API — confirmed by
/// direct inspection and reflected in this crate's `Cargo.toml`, which no longer depends on it — so
/// [`MultiObjectTracker::update`] solves the gated assignment with a real, working, greedy
/// ascending-cost matcher instead: a documented, simpler substitute for true max-weight bipartite
/// matching, adequate for well-separated tracks/detections.
#[derive(Clone, Debug, Default)]
pub struct MultiObjectTracker {
    tracks: Vec<TrackState>,
    next_id: u32,
}

impl MultiObjectTracker {
    /// 🎬️ An empty tracker with no live tracks.
    pub fn new() -> Self {
        Self { tracks: Vec::new(), next_id: 0 }
    }

    /// 🔮️ Constant-velocity prediction step: advances every live track's `(x, y)` by `(vx, vy) * dt`.
    pub fn predict(&mut self, dt: f32) {
        for t in &mut self.tracks {
            t.x += t.vx * dt;
            t.y += t.vy * dt;
        }
    }

    /// 🔄️ Associates `detections` against live tracks: builds a Euclidean-distance cost matrix, gates out
    /// pairs farther apart than `gate_radius`, then greedily accepts the globally-cheapest remaining pair
    /// repeatedly (a documented simplified substitute for max-weight bipartite matching — see this type's
    /// docs). Matched tracks blend the measured `(dx, dy)` displacement into `(vx, vy)` with a fixed gain
    /// ([`MOT_ALPHA`]); unmatched tracks accumulate a miss and are pruned once `missed` exceeds
    /// [`MOT_MAX_MISSED`]; unmatched detections spawn fresh tracks. Returns `(track_id, detection_index)`
    /// pairs for every detection associated with a track this call — both re-matched existing tracks and
    /// freshly spawned ones, so callers always learn the id backing every input detection.
    pub fn update(&mut self, detections: &[Detection], gate_radius: f32) -> Vec<(u32, usize)> {
        let mut candidates: Vec<(f32, usize, usize)> = Vec::new();
        for (ti, t) in self.tracks.iter().enumerate() {
            for (di, d) in detections.iter().enumerate() {
                let dist = (t.x - d.x).hypot(t.y - d.y);
                if dist <= gate_radius {
                    candidates.push((dist, ti, di));
                }
            }
        }
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut track_used = vec![false; self.tracks.len()];
        let mut det_used = vec![false; detections.len()];
        let mut assignments = Vec::new();
        for (_, ti, di) in candidates {
            if track_used[ti] || det_used[di] {
                continue;
            }
            track_used[ti] = true;
            det_used[di] = true;
            assignments.push((ti, di));
        }

        let mut result = Vec::with_capacity(assignments.len());
        for (ti, di) in assignments {
            let det = detections[di];
            let t = &mut self.tracks[ti];
            let (mvx, mvy) = (det.x - t.x, det.y - t.y);
            t.vx = t.vx * (1.0 - MOT_ALPHA) + mvx * MOT_ALPHA;
            t.vy = t.vy * (1.0 - MOT_ALPHA) + mvy * MOT_ALPHA;
            t.x = det.x;
            t.y = det.y;
            t.age += 1;
            t.missed = 0;
            result.push((t.id, di));
        }
        for (ti, t) in self.tracks.iter_mut().enumerate() {
            if !track_used[ti] {
                t.missed += 1;
            }
        }
        self.tracks.retain(|t| t.missed <= MOT_MAX_MISSED);
        for (di, det) in detections.iter().enumerate() {
            if !det_used[di] {
                let id = self.next_id;
                self.next_id += 1;
                self.tracks.push(TrackState { id, x: det.x, y: det.y, vx: 0.0, vy: 0.0, age: 0, missed: 0 });
                result.push((id, di));
            }
        }
        result
    }
}
// #endregion 🔖️Mot

// #region 🔖️Trajectory3d
/// 🛤️ A recovered 3D position time-series: `(timestamp, [x, y, z])` samples, one per successfully
/// triangulated frame of a [`Track2d`].
#[derive(Clone, Debug, PartialEq)]
pub struct Trajectory3d {
    pub samples: Vec<(f64, [f64; 3])>,
}

const TRAJECTORY_WINDOW_RADIUS: usize = 3;

/// 📐️ Triangulates every track into a moving 3D trajectory from a **single moving camera** whose per-frame
/// poses are already known (e.g. from prior SfM) — `cams_per_frame` is expected to hold at most one
/// `(CameraPose, Intrinsics)` per frame index, not a synced multi-camera rig. A lone camera cannot
/// triangulate an instantaneous position from a single observation, so each output sample instead reuses a
/// short sliding window of up to `2 * TRAJECTORY_WINDOW_RADIUS + 1` neighbouring frames' poses and this
/// track's pixel observation at each of them as a multi-view [`remodel_sfm::triangulate_dlt`] problem,
/// under the documented quasi-static approximation that the tracked point moves little relative to the
/// camera's baseline change across that short window. This is a deliberate simplification of the more
/// general synced-rig formulation (multiple simultaneous observations at one frame index), which the flat
/// [`Track2d`] type cannot represent without a per-observation camera id.
pub fn triangulate_tracks(cams_per_frame: &[(u32, CameraPose, Intrinsics)], tracks: &[Track2d], frame_timestamps: &std::collections::HashMap<u32, f64>) -> Vec<Trajectory3d> {
    let mut pose_by_frame: std::collections::HashMap<u32, (CameraPose, Intrinsics)> = std::collections::HashMap::new();
    for &(frame, pose, intr) in cams_per_frame {
        pose_by_frame.entry(frame).or_insert((pose, intr));
    }
    tracks
        .iter()
        .map(|track| {
            let mut samples = Vec::new();
            for (i, &(frame, _, _)) in track.samples.iter().enumerate() {
                let Some(&ts) = frame_timestamps.get(&frame) else { continue };
                let lo = i.saturating_sub(TRAJECTORY_WINDOW_RADIUS);
                let hi = (i + TRAJECTORY_WINDOW_RADIUS).min(track.samples.len() - 1);
                let mut poses = Vec::new();
                let mut obs = Vec::new();
                for &(wframe, wx, wy) in &track.samples[lo..=hi] {
                    if let Some(&(pose, intr)) = pose_by_frame.get(&wframe) {
                        poses.push((pose, intr));
                        obs.push([f64::from(wx), f64::from(wy)]);
                    }
                }
                if poses.len() < 2 {
                    continue;
                }
                if let Some(p) = remodel_sfm::triangulate_dlt(&poses, &obs) {
                    samples.push((ts, p));
                }
            }
            Trajectory3d { samples }
        })
        .collect()
}
// #endregion 🔖️Trajectory3d

// #region 🔖️Kinematics
const KINEMATICS_SG_WINDOW: usize = 5;
const KINEMATICS_SG_ORDER: usize = 2;

fn odd_window(desired: usize, max_len: usize) -> usize {
    let cap = desired.min(max_len);
    let odd = if cap.is_multiple_of(2) { cap.saturating_sub(1) } else { cap };
    odd.max(3)
}

fn mean_dt(traj: &Trajectory3d) -> f64 {
    let n = traj.samples.len();
    if n < 2 {
        return 1.0;
    }
    let span = traj.samples[n - 1].0 - traj.samples[0].0;
    if span <= 0.0 {
        1.0
    } else {
        span / (n - 1) as f64
    }
}

/// 📈️ Shared Savitzky-Golay derivative kernel for [`velocity`]/[`acceleration`]: assumes roughly-uniform
/// sampling (`dt` = mean spacing between consecutive timestamps) and applies
/// [`mathematical_signal::savitzky_golay`] independently to each of the x/y/z channels. Falls back to all
/// zeros when there are too few samples for even the smallest valid odd window (`< 3` points).
fn sg_derivative(traj: &Trajectory3d, deriv: usize) -> Vec<[f64; 3]> {
    let n = traj.samples.len();
    if n < 3 {
        return vec![[0.0; 3]; n];
    }
    let dt = mean_dt(traj);
    let window = odd_window(KINEMATICS_SG_WINDOW, n);
    let order = KINEMATICS_SG_ORDER.min(window - 1);
    let deriv = deriv.min(order).min(2);
    let xs: Vec<f64> = traj.samples.iter().map(|s| s.1[0]).collect();
    let ys: Vec<f64> = traj.samples.iter().map(|s| s.1[1]).collect();
    let zs: Vec<f64> = traj.samples.iter().map(|s| s.1[2]).collect();
    let vx = mathematical_signal::savitzky_golay(&xs, window, order, deriv, dt);
    let vy = mathematical_signal::savitzky_golay(&ys, window, order, deriv, dt);
    let vz = mathematical_signal::savitzky_golay(&zs, window, order, deriv, dt);
    (0..n).map(|i| [vx[i], vy[i], vz[i]]).collect()
}

/// ⚡️ Per-sample velocity of a [`Trajectory3d`] via first-order Savitzky-Golay differentiation, assuming
/// roughly-uniform timestamp spacing (see [`sg_derivative`]).
pub fn velocity(traj: &Trajectory3d) -> Vec<[f64; 3]> {
    sg_derivative(traj, 1)
}

/// 🚀️ Per-sample acceleration of a [`Trajectory3d`] via second-order Savitzky-Golay differentiation,
/// assuming roughly-uniform timestamp spacing (see [`sg_derivative`]).
pub fn acceleration(traj: &Trajectory3d) -> Vec<[f64; 3]> {
    sg_derivative(traj, 2)
}

/// 📏️ Polyline arc length: the sum of Euclidean distances between consecutive samples.
pub fn arc_length(traj: &Trajectory3d) -> f64 {
    traj.samples.windows(2).map(|w| dist3(w[0].1, w[1].1)).sum()
}

/// 🪢️ One neighbourhood edge's Green-Lagrange strain magnitude from [`neighborhood_affine_strain`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrainPair {
    pub point_a: usize,
    pub point_b: usize,
    pub strain: f64,
}

const STRAIN_MIN_NEIGHBORS: usize = 3;

/// 🧩️ Solves the best-fit local linear map `F` (`d1_k ≈ F d0_k`) for a neighbourhood's offset vectors via
/// independent per-row least squares ([`solve_llsq`]) — this crate depends on `mathematical_algebra`
/// (already required transitively via `remodel_camera`/`remodel_sfm`), so a hand-rolled 3x3 normal-equation
/// solve would just duplicate its QR-based `solve_llsq`.
fn fit_affine_3x3(d0: &[[f64; 3]], d1: &[[f64; 3]]) -> Option<[[f64; 3]; 3]> {
    let mut a = MatD::zeros(d0.len(), 3);
    for (row, p) in d0.iter().enumerate() {
        a.set(row, 0, p[0]);
        a.set(row, 1, p[1]);
        a.set(row, 2, p[2]);
    }
    let mut f = [[0.0; 3]; 3];
    for (r, row) in f.iter_mut().enumerate() {
        let b = VecD::from_vec(d1.iter().map(|p| p[r]).collect());
        let x = solve_llsq(&a, &b).ok()?;
        *row = [x.get(0), x.get(1), x.get(2)];
    }
    Some(f)
}

fn green_lagrange_frobenius(f: &[[f64; 3]; 3]) -> f64 {
    let mut sum_sq = 0.0;
    for i in 0..3 {
        for j in 0..3 {
            let ftf_ij: f64 = (0..3).map(|k| f[k][i] * f[k][j]).sum();
            let e = 0.5 * (ftf_ij - if i == j { 1.0 } else { 0.0 });
            sum_sq += e * e;
        }
    }
    sum_sq.sqrt()
}

/// 🧵️ Local (neighbourhood-level) affine deformation-gradient strain: for every point `i` in `cloud_t0`
/// with at least [`STRAIN_MIN_NEIGHBORS`] index-corresponding neighbours within `radius` (matched to
/// `cloud_t1` by shared index — the same point cloud at two instants), fits the best-fit `3x3` linear map
/// `F` ([`fit_affine_3x3`]) taking each neighbour's `t0`-relative offset to its `t1`-relative offset, then
/// reports the Green-Lagrange strain tensor's Frobenius norm `‖(FᵀF - I) / 2‖_F` (zero for a pure rotation,
/// since `FᵀF = I` there) once per `(point_a: i, point_b: neighbour)` edge that supported the fit.
/// <https://en.wikipedia.org/wiki/Finite_strain_theory>
pub fn neighborhood_affine_strain(cloud_t0: &[[f64; 3]], cloud_t1: &[[f64; 3]], radius: f64) -> Vec<StrainPair> {
    if cloud_t0.len() != cloud_t1.len() || cloud_t0.is_empty() {
        return Vec::new();
    }
    let tree = KdTree::<3>::build(cloud_t0);
    let mut out = Vec::new();
    for i in 0..cloud_t0.len() {
        let neighbor_idxs: Vec<usize> = tree.radius(&cloud_t0[i], radius).into_iter().filter_map(|(id, _)| (id as usize != i).then_some(id as usize)).collect();
        if neighbor_idxs.len() < STRAIN_MIN_NEIGHBORS {
            continue;
        }
        let d0: Vec<[f64; 3]> = neighbor_idxs.iter().map(|&j| sub3(cloud_t0[j], cloud_t0[i])).collect();
        let d1: Vec<[f64; 3]> = neighbor_idxs.iter().map(|&j| sub3(cloud_t1[j], cloud_t1[i])).collect();
        let Some(f) = fit_affine_3x3(&d0, &d1) else { continue };
        let strain = green_lagrange_frobenius(&f);
        for &j in &neighbor_idxs {
            out.push(StrainPair { point_a: i, point_b: j, strain });
        }
    }
    out
}
// #endregion 🔖️Kinematics

// #region 🔖️Modal
/// 🎼️ Extracted vibration modes from [`modal_analysis`]: parallel arrays indexed by mode, plus mode shapes
/// laid out `mode_shapes[mode][track] = cross-spectral magnitude between that track and the reference
/// track at the mode's frequency, signed by `cos(phase)` so in-phase vs. anti-phase motion (the coarse part
/// of the phase information a real-valued mode shape can carry) survives the fold into a single scalar.
#[derive(Clone, Debug, PartialEq)]
pub struct ModalResult {
    pub frequencies_hz: Vec<f64>,
    pub damping_ratios: Vec<f64>,
    pub mode_shapes: Vec<Vec<f64>>,
}

const MODAL_WELCH_SEG_LEN: usize = 512;
const MODAL_WELCH_OVERLAP: f64 = 0.5;
const MODAL_PROMINENCE_FRACTION: f64 = 0.05;

fn half_power_bandwidth_damping(psd: &[f64], peak_idx: usize, bin_hz: f64, freq_hz: f64) -> f64 {
    if freq_hz <= 0.0 {
        return 0.0;
    }
    let half_power = psd[peak_idx] * 0.5;
    let mut lo = peak_idx;
    while lo > 0 && psd[lo] > half_power {
        lo -= 1;
    }
    let mut hi = peak_idx;
    while hi + 1 < psd.len() && psd[hi] > half_power {
        hi += 1;
    }
    let bandwidth_hz = (hi - lo) as f64 * bin_hz;
    (bandwidth_hz / (2.0 * freq_hz)).max(0.0)
}

/// 🎵️ Per-track Welch-PSD modal analysis: extracts each track's y-displacement signal, computes
/// `reference_track`'s power spectral density ([`mathematical_signal::welch_psd`]), picks the strongest
/// `max_modes` peaks ([`mathematical_signal::find_peaks`]) above a PSD-relative prominence floor, estimates
/// each mode's half-power-bandwidth damping ratio, and derives every track's mode shape from cross-spectral
/// analysis ([`mathematical_signal::cross_spectrum`]) against the reference track at each modal frequency.
/// <https://en.wikipedia.org/wiki/Q_factor#Bandwidth_definition>
pub fn modal_analysis(tracks: &[Track2d], fps: f64, reference_track: usize, max_modes: usize) -> ModalResult {
    if tracks.is_empty() || fps <= 0.0 || max_modes == 0 || reference_track >= tracks.len() {
        return ModalResult { frequencies_hz: Vec::new(), damping_ratios: Vec::new(), mode_shapes: Vec::new() };
    }
    let signals: Vec<Vec<f64>> = tracks.iter().map(|t| t.samples.iter().map(|&(_, _, y)| f64::from(y)).collect()).collect();
    if signals[reference_track].len() < 4 {
        return ModalResult { frequencies_hz: Vec::new(), damping_ratios: Vec::new(), mode_shapes: Vec::new() };
    }
    let seg_len = MODAL_WELCH_SEG_LEN.min(signals[reference_track].len());
    let psd = mathematical_signal::welch_psd(&signals[reference_track], seg_len, MODAL_WELCH_OVERLAP);
    let nfft = mathematical_signal::next_pow2(seg_len);
    let bin_hz = fps / nfft as f64;
    let max_psd = psd.iter().copied().fold(0.0f64, f64::max);

    let mut peaks = mathematical_signal::find_peaks(&psd, max_psd * MODAL_PROMINENCE_FRACTION);
    peaks.sort_by(|a, b| b.value.total_cmp(&a.value));
    peaks.truncate(max_modes);
    peaks.sort_by_key(|p| p.index);

    let cross: Vec<(Vec<f64>, Vec<f64>)> = signals.iter().map(|sig| mathematical_signal::cross_spectrum(&signals[reference_track], sig, seg_len, MODAL_WELCH_OVERLAP)).collect();

    let mut frequencies_hz = Vec::with_capacity(peaks.len());
    let mut damping_ratios = Vec::with_capacity(peaks.len());
    let mut mode_shapes = Vec::with_capacity(peaks.len());
    for peak in &peaks {
        let freq = peak.index as f64 * bin_hz;
        frequencies_hz.push(freq);
        damping_ratios.push(half_power_bandwidth_damping(&psd, peak.index, bin_hz, freq));
        let shape: Vec<f64> = cross
            .iter()
            .map(|(mag, phase)| {
                let m = mag.get(peak.index).copied().unwrap_or(0.0);
                let p = phase.get(peak.index).copied().unwrap_or(0.0);
                m * p.cos()
            })
            .collect();
        mode_shapes.push(shape);
    }
    ModalResult { frequencies_hz, damping_ratios, mode_shapes }
}
// #endregion 🔖️Modal

// #region 🔖️Sync
/// ⏱️ Coarse-to-fine camera-sync result: fractional-frame offset of `motion_signal_b` relative to
/// `motion_signal_a`, and the peak normalized-correlation confidence backing it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SyncResult {
    pub offset_frames: f64,
    pub confidence: f64,
}

/// 🔗️ Coarse sync-offset estimate between two per-frame motion-activity signals (e.g. mean optical-flow
/// magnitude or track-position-change per frame): normalized cross-correlation
/// ([`mathematical_signal::xcorr_normalized`]) locates the lag, sub-sample-refined
/// ([`mathematical_signal::subsample_peak`]) to fractional-frame precision. `fps` is accepted for interface
/// symmetry with time-domain callers (and to mirror [`refine_subframe`]'s frame-domain contract) — the
/// correlation itself already operates in frame-index units, so it does not change the computation.
pub fn estimate_offset(motion_signal_a: &[f64], motion_signal_b: &[f64], fps: f64) -> SyncResult {
    let _ = fps;
    if motion_signal_a.is_empty() || motion_signal_b.is_empty() {
        return SyncResult { offset_frames: 0.0, confidence: 0.0 };
    }
    let max_lag = (motion_signal_a.len().min(motion_signal_b.len()) / 4).max(4);
    let xc = mathematical_signal::xcorr_normalized(motion_signal_a, motion_signal_b, max_lag);
    let confidence = xc.iter().copied().fold(f64::MIN, f64::max).max(0.0);
    let Some(peak_idx_f) = mathematical_signal::subsample_peak(&xc) else {
        return SyncResult { offset_frames: 0.0, confidence };
    };
    SyncResult { offset_frames: peak_idx_f - max_lag as f64, confidence }
}

fn correlation_at_offset(a: &[f64], b: &[f64], offset: f64) -> f64 {
    let mut sum_ab = 0.0;
    let mut sum_a2 = 0.0;
    let mut sum_b2 = 0.0;
    let mut count = 0usize;
    for (i, &av) in a.iter().enumerate() {
        let bi = i as f64 + offset;
        if bi < 0.0 || bi > (b.len() - 1) as f64 {
            continue;
        }
        let lo = bi.floor() as usize;
        let hi = (lo + 1).min(b.len() - 1);
        let frac = bi - lo as f64;
        let bv = b[lo] * (1.0 - frac) + b[hi] * frac;
        sum_ab += av * bv;
        sum_a2 += av * av;
        sum_b2 += bv * bv;
        count += 1;
    }
    if count == 0 || sum_a2 <= 0.0 || sum_b2 <= 0.0 {
        return -1.0;
    }
    sum_ab / (sum_a2.sqrt() * sum_b2.sqrt())
}

/// 🔬️ Sub-frame polish of a coarse sync offset: a local golden-section search
/// ([`mathematical_optimize::golden_section`]) over `[coarse_offset - 1, coarse_offset + 1]` maximizing the
/// linearly-interpolated normalized correlation between `motion_signal_a` and a continuously-shifted
/// `motion_signal_b`.
pub fn refine_subframe(motion_signal_a: &[f64], motion_signal_b: &[f64], coarse_offset: f64) -> f64 {
    if motion_signal_a.is_empty() || motion_signal_b.is_empty() {
        return coarse_offset;
    }
    let (best_offset, _best_value) = mathematical_optimize::golden_section(|o| -correlation_at_offset(motion_signal_a, motion_signal_b, o), coarse_offset - 1.0, coarse_offset + 1.0, 1e-4);
    best_offset
}
// #endregion 🔖️Sync

// #region 🔖️RollingShutterComp
fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        sxy += (xi - mean_x) * (yi - mean_y);
        sxx += (xi - mean_x) * (xi - mean_x);
    }
    if sxx.abs() < 1e-9 {
        return (mean_y, 0.0);
    }
    let slope = sxy / sxx;
    (mean_y - slope * mean_x, slope)
}

/// 🌀️ Simplified 2D-only rolling-shutter velocity estimate: a real 3D SE(3) tangent is not observable from
/// bare 2D tracks (no depth/pose), so this instead fits a per-row-affine 2D displacement field — linear
/// regression of each track's frame-to-frame `(dx, dy)` against its source row — and packs it into the
/// requested `[f64; 6]` slot as `[vx0, vy0, kx, ky, 0.0, 0.0]`: a baseline per-frame velocity `(vx0, vy0)`
/// plus its row-dependent gradient `(kx, ky)` (px/frame per pixel-row), the standard rolling-shutter
/// approximation when only image-plane correspondences are available.
pub fn estimate_rs_velocity(tracks: &[Track2d], image_height: u32) -> [f64; 6] {
    let mut rows = Vec::new();
    let mut dxs = Vec::new();
    let mut dys = Vec::new();
    for track in tracks {
        for w in track.samples.windows(2) {
            let (_, x0, y0) = w[0];
            let (_, x1, y1) = w[1];
            rows.push(f64::from(y0));
            dxs.push(f64::from(x1 - x0));
            dys.push(f64::from(y1 - y0));
        }
    }
    if rows.is_empty() || image_height == 0 {
        return [0.0; 6];
    }
    let (vx0, kx) = linear_regression(&rows, &dxs);
    let (vy0, ky) = linear_regression(&rows, &dys);
    [vx0, vy0, kx, ky, 0.0, 0.0]
}

/// 🗺️ Per-pixel `(map_x, map_y)` rolling-shutter rectification field from [`estimate_rs_velocity`]'s
/// simplified per-row-affine `model`, consumable by `remodel_image::remap`: shifts each output pixel by
/// the row-dependent gradient `(kx, ky)` relative to the frame's vertical center. `line_delay_s <= 0` (no
/// rolling-shutter skew) short-circuits to the identity map; otherwise, since the fitted `model` is already
/// expressed directly in px-per-row units, `line_delay_s`'s absolute seconds value does not further scale
/// the correction under this simplified model — an explicit, honest limitation of using only 2D track data.
pub fn build_rs_rectify_maps(model: [f64; 6], image_width: u32, image_height: u32, line_delay_s: f64) -> (Vec<f32>, Vec<f32>) {
    let n = image_width as usize * image_height as usize;
    let mut map_x = vec![0.0f32; n];
    let mut map_y = vec![0.0f32; n];
    if line_delay_s <= 0.0 || image_width == 0 || image_height == 0 {
        for row in 0..image_height {
            for col in 0..image_width {
                let idx = (row * image_width + col) as usize;
                map_x[idx] = col as f32;
                map_y[idx] = row as f32;
            }
        }
        return (map_x, map_y);
    }
    let [_, _, kx, ky, ..] = model;
    let ref_row = f64::from(image_height) * 0.5;
    for row in 0..image_height {
        let shift_x = kx * (f64::from(row) - ref_row);
        let shift_y = ky * (f64::from(row) - ref_row);
        for col in 0..image_width {
            let idx = (row * image_width + col) as usize;
            map_x[idx] = (f64::from(col) + shift_x) as f32;
            map_y[idx] = (f64::from(row) + shift_y) as f32;
        }
    }
    (map_x, map_y)
}
// #endregion 🔖️RollingShutterComp

// #region 🔖️Stabilize
const STABILIZE_SG_ORDER: usize = 2;

/// 🧘️ Savitzky-Golay smoothing of a camera pose sequence in the SE(3) Lie algebra: consecutive relative
/// motions are logged to twists, each of the 6 twist channels is smoothed independently (`deriv = 0`), and
/// the smoothed twists are reintegrated via [`Se3::exp`]/[`Se3::semio_compose_rs`] — so the result stays exactly on
/// the SE(3) manifold instead of naively averaging matrix or quaternion components. `window` is clamped to
/// the largest valid odd value `>= 3` for however many pose-to-pose twists are available.
pub fn smooth_camera_path(poses: &[Se3], window: usize) -> Vec<Se3> {
    let n = poses.len();
    if n < 2 {
        return poses.to_vec();
    }
    let twists: Vec<[f64; 6]> = poses.windows(2).map(|w| w[1].semio_compose_rs(&w[0].inverse()).log()).collect();
    let smoothed_twists = if twists.len() >= 3 {
        let win = odd_window(window, twists.len());
        let order = STABILIZE_SG_ORDER.min(win - 1);
        let channels: Vec<Vec<f64>> = (0..6).map(|c| mathematical_signal::savitzky_golay(&twists.iter().map(|t| t[c]).collect::<Vec<f64>>(), win, order, 0, 1.0)).collect();
        (0..twists.len()).map(|i| std::array::from_fn(|c| channels[c][i])).collect()
    } else {
        twists
    };
    let mut out = Vec::with_capacity(n);
    out.push(poses[0]);
    for xi in &smoothed_twists {
        let prev = *out.last().expect("out seeded with poses[0]");
        out.push(Se3::exp(*xi).semio_compose_rs(&prev));
    }
    out
}

fn intrinsics_matrix(intr: &Intrinsics) -> [[f64; 3]; 3] {
    [[intr.fx, intr.skew, intr.cx], [0.0, intr.fy, intr.cy], [0.0, 0.0, 1.0]]
}

fn intrinsics_matrix_inverse(intr: &Intrinsics) -> [[f64; 3]; 3] {
    let (fx, fy, cx, cy, skew) = (intr.fx, intr.fy, intr.cx, intr.cy, intr.skew);
    [[1.0 / fx, -skew / (fx * fy), (skew * cy - cx * fy) / (fx * fy)], [0.0, 1.0 / fy, -cy / fy], [0.0, 0.0, 1.0]]
}

/// 🪟️ Per-frame pure-rotation stabilization homography `H = K · ΔR · K⁻¹` warping the original jittery
/// viewpoint into the smoothed one, appropriate for a distant scene where translational parallax is
/// negligible; `ΔR` is the rotation from `original[i]`'s orientation to `smoothed[i]`'s.
pub fn stabilization_warps(original: &[Se3], smoothed: &[Se3], intr: &Intrinsics) -> Vec<[[f64; 3]; 3]> {
    let k = intrinsics_matrix(intr);
    let k_inv = intrinsics_matrix_inverse(intr);
    original
        .iter()
        .zip(smoothed.iter())
        .map(|(orig, smooth)| {
            let r_orig: [[f64; 3]; 3] = std::array::from_fn(|row| std::array::from_fn(|col| orig.r.0.cols[col][row]));
            let r_smooth: [[f64; 3]; 3] = std::array::from_fn(|row| std::array::from_fn(|col| smooth.r.0.cols[col][row]));
            let r_orig_t: [[f64; 3]; 3] = std::array::from_fn(|row| std::array::from_fn(|col| r_orig[col][row]));
            let r_rel = mat3_mul(&r_smooth, &r_orig_t);
            mat3_mul(&mat3_mul(&k, &r_rel), &k_inv)
        })
        .collect()
}
// #endregion 🔖️Stabilize

// #region 🔖️Deblur
/// 🌫️ A small square motion-blur point-spread function, normalized to sum to 1.
#[derive(Clone, Debug, PartialEq)]
pub struct Psf {
    pub kernel: Vec<f32>,
    pub width: u32,
    pub height: u32,
}

const DEBLUR_KERNEL_MARGIN: f32 = 2.0;
const DEBLUR_WIENER_EPS: f32 = 1e-6;

fn splat_bilinear(kernel: &mut [f32], size: u32, x: f32, y: f32, weight: f32) {
    let x0 = x.floor();
    let y0 = y.floor();
    let (fx, fy) = (x - x0, y - y0);
    for (dx, dy, w) in [(0.0, 0.0, (1.0 - fx) * (1.0 - fy)), (1.0, 0.0, fx * (1.0 - fy)), (0.0, 1.0, (1.0 - fx) * fy), (1.0, 1.0, fx * fy)] {
        let px = x0 + dx;
        let py = y0 + dy;
        if px >= 0.0 && py >= 0.0 && (px as u32) < size && (py as u32) < size {
            kernel[(py as u32 * size + px as u32) as usize] += weight * w;
        }
    }
}

/// 💨️ Simple linear motion-blur kernel: a line segment of length `speed * exposure_fraction` (clamped to at
/// least 1px) along the velocity direction, rasterized into a small square kernel via bilinear splatting
/// and normalized to sum to 1.
pub fn estimate_motion_psf(track_velocity_px_per_frame: (f32, f32), exposure_fraction: f32) -> Psf {
    let (vx, vy) = track_velocity_px_per_frame;
    let speed = vx.hypot(vy);
    let length = (speed * exposure_fraction.clamp(0.0, 1.0)).max(1.0);
    let size = (((length + DEBLUR_KERNEL_MARGIN * 2.0).ceil() as u32).max(3)) | 1;
    let mut kernel = vec![0.0f32; (size * size) as usize];
    let center = size as f32 / 2.0;
    let (dir_x, dir_y) = if speed > 1e-6 { (vx / speed, vy / speed) } else { (1.0, 0.0) };
    let steps = ((length.ceil() as usize) * 4).max(2);
    for s in 0..=steps {
        let t = (s as f32 / steps as f32 - 0.5) * length;
        splat_bilinear(&mut kernel, size, center + dir_x * t, center + dir_y * t, 1.0);
    }
    let sum: f32 = kernel.iter().sum();
    if sum > 0.0 {
        for v in &mut kernel {
            *v /= sum;
        }
    }
    Psf { kernel, width: size, height: size }
}

/// 🔍️ Frequency-domain Wiener deconvolution against a known [`Psf`]: `G = conj(H) / (|H|² + 1/snr)` applied
/// to the DFT of the (zero-padded, power-of-two) image via [`mathematical_signal::fft2`]/[`mathematical_signal::ifft2`],
/// with the PSF's center wrapped to the origin for circular convolution, then cropped back to the input size.
/// <https://en.wikipedia.org/wiki/Wiener_deconvolution>
pub fn wiener_deconvolve(img: &ImageGray, psf: &Psf, snr: f32) -> ImageGray {
    if img.width == 0 || img.height == 0 || psf.width == 0 || psf.height == 0 {
        return img.clone();
    }
    let pad_w = mathematical_signal::next_pow2((img.width + psf.width) as usize);
    let pad_h = mathematical_signal::next_pow2((img.height + psf.height) as usize);
    let mut img_re = vec![0.0f64; pad_w * pad_h];
    let mut img_im = vec![0.0f64; pad_w * pad_h];
    for y in 0..img.height {
        for x in 0..img.width {
            img_re[y as usize * pad_w + x as usize] = f64::from(img.get(x, y));
        }
    }
    let mut psf_re = vec![0.0f64; pad_w * pad_h];
    let mut psf_im = vec![0.0f64; pad_w * pad_h];
    let cx = i64::from(psf.width / 2);
    let cy = i64::from(psf.height / 2);
    for y in 0..psf.height {
        for x in 0..psf.width {
            let value = psf.kernel[(y * psf.width + x) as usize];
            let ox = (i64::from(x) - cx).rem_euclid(pad_w as i64) as usize;
            let oy = (i64::from(y) - cy).rem_euclid(pad_h as i64) as usize;
            psf_re[oy * pad_w + ox] += f64::from(value);
        }
    }
    mathematical_signal::fft2(&mut img_re, &mut img_im, pad_w, pad_h);
    mathematical_signal::fft2(&mut psf_re, &mut psf_im, pad_w, pad_h);
    let inv_snr = 1.0f64 / f64::from(snr.max(DEBLUR_WIENER_EPS));
    let mut out_re = vec![0.0f64; pad_w * pad_h];
    let mut out_im = vec![0.0f64; pad_w * pad_h];
    for i in 0..pad_w * pad_h {
        let (hr, hi) = (psf_re[i], psf_im[i]);
        let denom = hr * hr + hi * hi + inv_snr;
        let (gr, gi) = if denom > 1e-300 { (hr / denom, -hi / denom) } else { (0.0, 0.0) };
        out_re[i] = gr * img_re[i] - gi * img_im[i];
        out_im[i] = gr * img_im[i] + gi * img_re[i];
    }
    mathematical_signal::ifft2(&mut out_re, &mut out_im, pad_w, pad_h);
    let mut result = ImageGray::new(img.width, img.height);
    for y in 0..img.height {
        for x in 0..img.width {
            result.set(x, y, out_re[y as usize * pad_w + x as usize].clamp(0.0, 1.0) as f32);
        }
    }
    result
}
// #endregion 🔖️Deblur

// #region 🔖️NonRigid
/// 🕸️ One embedded-deformation graph node: rest position plus its currently-fitted rigid transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeformationNode {
    pub position: [f64; 3],
    pub transform: Se3,
}

/// 🕷️ A sparse embedded-deformation graph (Sumner et al.): control nodes plus undirected neighbour edges.
#[derive(Clone, Debug, PartialEq)]
pub struct DeformationGraph {
    pub nodes: Vec<DeformationNode>,
    pub edges: Vec<(usize, usize)>,
}

/// 🏗️ Builds a deformation graph with one identity-transform node per `seed_points` entry, connected by an
/// undirected edge whenever two nodes are within `edge_radius` of each other.
pub fn build_deformation_graph(seed_points: &[[f64; 3]], edge_radius: f64) -> DeformationGraph {
    let nodes: Vec<DeformationNode> = seed_points.iter().map(|&p| DeformationNode { position: p, transform: Se3::identity() }).collect();
    let mut edges = Vec::new();
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if dist3(nodes[i].position, nodes[j].position) <= edge_radius {
                edges.push((i, j));
            }
        }
    }
    DeformationGraph { nodes, edges }
}

struct DeformationNodeFitProblem<'a> {
    offsets: &'a [[f64; 3]],
    targets: &'a [[f64; 3]],
}

impl LeastSquaresProblem for DeformationNodeFitProblem<'_> {
    fn residual_count(&self) -> usize {
        self.offsets.len() * 3
    }

    fn parameter_count(&self) -> usize {
        6
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let xi: [f64; 6] = std::array::from_fn(|k| x.get(k));
        let se3 = Se3::exp(xi);
        for (row, (off, tgt)) in self.offsets.iter().zip(self.targets.iter()).enumerate() {
            let pred = se3.act(*off);
            out.set(row * 3, pred[0] - tgt[0]);
            out.set(row * 3 + 1, pred[1] - tgt[1]);
            out.set(row * 3 + 2, pred[2] - tgt[2]);
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        numeric_jacobian(self, x, 1e-6, out);
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let cur: [f64; 6] = std::array::from_fn(|k| x.get(k));
        let d: [f64; 6] = std::array::from_fn(|k| dx.get(k));
        VecD::from_vec(Se3::exp(d).semio_compose_rs(&Se3::exp(cur)).log().to_vec())
    }
}

const NONRIGID_MIN_CORRESPONDENCES: usize = 3;

/// 🧩️ Fits each node's local rigid [`Se3`] transform independently via Levenberg-Marquardt over its nearby
/// correspondences (any `dst_correspondences` entry whose `src_points` position lies within
/// `correspondence_radius` of that node): a documented simplification of full embedded deformation, which
/// jointly solves all nodes with an ARAP-style smoothness coupling between neighbours — here each node's
/// fit uses only its own local data, ignoring the `edges` regularization term, trading joint-consistency for
/// simplicity while remaining a real, working, per-node rigid fit. Nodes with fewer than
/// [`NONRIGID_MIN_CORRESPONDENCES`] nearby correspondences keep their prior transform unchanged.
pub fn fit_deformation(graph: &DeformationGraph, src_points: &[[f64; 3]], dst_correspondences: &[(usize, [f64; 3])], correspondence_radius: f64) -> DeformationGraph {
    let mut nodes = graph.nodes.clone();
    for node in &mut nodes {
        let mut offsets = Vec::new();
        let mut targets = Vec::new();
        for &(src_idx, dst_pos) in dst_correspondences {
            let Some(&src_pt) = src_points.get(src_idx) else { continue };
            if dist3(src_pt, node.position) <= correspondence_radius {
                offsets.push(sub3(src_pt, node.position));
                targets.push(sub3(dst_pos, node.position));
            }
        }
        if offsets.len() < NONRIGID_MIN_CORRESPONDENCES {
            continue;
        }
        let problem = DeformationNodeFitProblem { offsets: &offsets, targets: &targets };
        let result = levenberg_marquardt(&problem, VecD::zeros(6), &LmConfig::default());
        let xi: [f64; 6] = std::array::from_fn(|k| result.x.get(k));
        node.transform = Se3::exp(xi);
    }
    DeformationGraph { nodes, edges: graph.edges.clone() }
}

const NONRIGID_WEIGHT_EPS: f64 = 1e-6;

/// 🔮️ Embedded-deformation query: blends every graph node within `radius` of `point` by inverse distance,
/// each contributing `node.transform.act(point - node.position) + node.position`. Falls back to `point`
/// unchanged when no node is within range. The natural complement to [`fit_deformation`] — without this,
/// a fitted [`DeformationGraph`]'s per-node transforms have no way to be evaluated at an arbitrary point.
pub fn deform_point(graph: &DeformationGraph, point: [f64; 3], radius: f64) -> [f64; 3] {
    let mut weighted = [0.0; 3];
    let mut weight_sum = 0.0;
    for node in &graph.nodes {
        let d = dist3(point, node.position);
        if d > radius {
            continue;
        }
        let w = 1.0 / (d + NONRIGID_WEIGHT_EPS);
        let pred = add3(node.transform.act(sub3(point, node.position)), node.position);
        weighted = add3(weighted, scale3(pred, w));
        weight_sum += w;
    }
    if weight_sum <= 0.0 {
        return point;
    }
    scale3(weighted, 1.0 / weight_sum)
}
// #endregion 🔖️NonRigid

// #region 🔖️Pose6d
const POSE6D_SMOOTH_WINDOW: usize = 5;
const POSE6D_MIN_VISIBLE_POINTS: usize = 6;

/// 📦️ Per-frame rigid-body 6DoF pose from partially-visible model-point observations: for each frame, gathers
/// the model points with a visible observation, solves an initial pose via `remodel_sfm::epnp` (skipping
/// frames with fewer than [`POSE6D_MIN_VISIBLE_POINTS`] visible points, `epnp`'s own minimum), polishes it
/// with `remodel_sfm::refine_pose_lm`, then temporally smooths the recovered sequence via
/// [`smooth_camera_path`]'s SE(3) log-tangent Savitzky-Golay filter.
pub fn track_rigid_body(model_points: &[[f64; 3]], per_frame_obs: &[(u32, Vec<Option<[f64; 2]>>)], intr: &Intrinsics) -> Vec<(u32, Se3)> {
    let mut raw: Vec<(u32, Se3)> = Vec::new();
    for (frame, obs) in per_frame_obs {
        let mut world_pts = Vec::new();
        let mut obs_px = Vec::new();
        for (i, o) in obs.iter().enumerate() {
            if let Some(px) = o {
                if let Some(&mp) = model_points.get(i) {
                    world_pts.push(mp);
                    obs_px.push(*px);
                }
            }
        }
        if world_pts.len() < POSE6D_MIN_VISIBLE_POINTS {
            continue;
        }
        let Some(initial) = remodel_sfm::epnp(intr, &world_pts, &obs_px) else { continue };
        let refined = remodel_sfm::refine_pose_lm(intr, &world_pts, &obs_px, initial);
        raw.push((*frame, refined));
    }
    if raw.len() < 3 {
        return raw;
    }
    let poses: Vec<Se3> = raw.iter().map(|&(_, p)| p).collect();
    let smoothed = smooth_camera_path(&poses, POSE6D_SMOOTH_WINDOW);
    raw.iter().zip(smoothed.iter()).map(|(&(frame, _), &p)| (frame, p)).collect()
}
// #endregion 🔖️Pose6d

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical_algebra::{vec3d_cross, vec3d_normalize, vec3d_sub, Mat3d};
    use mathematical_lie::So3;
    use remodel_camera::Distortion;
    use remodel_image::{build_pyramid, scharr_gradients, warp_affine};

    // #region 🔖️Fixtures
    fn lcg_next(state: &mut u64) -> f64 {
        *state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (*state >> 11) as f64 / (1u64 << 53) as f64
    }

    fn grid_texture(size: u32) -> ImageGray {
        let mut img = ImageGray::new(size, size);
        for v in img.data.iter_mut() {
            *v = 0.1;
        }
        let (step, square) = (10u32, 5u32);
        let mut y = 3;
        while y + square <= size {
            let mut x = 3;
            while x + square <= size {
                for dy in 0..square {
                    for dx in 0..square {
                        img.set(x + dx, y + dy, 0.9);
                    }
                }
                x += step;
            }
            y += step;
        }
        img
    }

    fn look_at_pose(eye: [f64; 3], target: [f64; 3], up: [f64; 3]) -> Se3 {
        let forward = vec3d_normalize(vec3d_sub(target, eye));
        let right = vec3d_normalize(vec3d_cross(up, forward));
        let true_up = vec3d_cross(forward, right);
        let r_cw = Mat3d::from_axes(right, true_up, forward);
        let r_wc = So3(r_cw).inverse();
        let t = scale3(r_wc.act(eye), -1.0);
        Se3 { r: r_wc, t }
    }

    fn pinhole(fx: f64, fy: f64, cx: f64, cy: f64) -> Intrinsics {
        Intrinsics { fx, fy, cx, cy, skew: 0.0, distortion: Distortion::None }
    }

    fn se3_error_norm(a: &Se3, b: &Se3) -> f64 {
        let xi = a.inverse().semio_compose_rs(b).log();
        xi.iter().map(|v| v * v).sum::<f64>().sqrt()
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️Track2dTests
    #[test]
    fn tracker2d_maintains_tracks_and_redetects_lost_coverage() {
        let base = grid_texture(80);
        let mut pyrs = vec![build_pyramid(&base, 3)];
        for i in 1..6u32 {
            let shifted = warp_affine(&base, &[[1.0, 0.0, (i * 4) as f32], [0.0, 1.0, 0.0]], 80, 80);
            pyrs.push(build_pyramid(&shifted, 3));
        }
        let mut tracker = Tracker2d::new();
        tracker.step(&pyrs[0], &pyrs[0], 0, 4, 20, 16, 2);
        assert!(!tracker.tracks().is_empty(), "seeding step should redetect at least one track");
        for i in 1..pyrs.len() {
            tracker.step(&pyrs[i - 1], &pyrs[i], i as u32, 4, 20, 16, 2);
        }
        let tracks = tracker.tracks();
        assert!(tracks.iter().any(|t| t.samples.len() > 1), "at least one track should survive multiple frames");
        assert!(tracks.iter().any(|t| t.samples[0].0 > 0), "redetection should spawn new tracks at a later frame as content shifts out of view");
    }
    // #endregion 🔖️Track2dTests

    // #region 🔖️MotTests
    #[test]
    fn multi_object_tracker_associates_spawns_and_prunes() {
        let mut tracker = MultiObjectTracker::new();
        let det_a = |t: f32| Detection { x: 10.0 + t, y: 10.0, id_hint: None };
        let det_b = |t: f32| Detection { x: 60.0 - t, y: 30.0, id_hint: None };

        let r0 = tracker.update(&[det_a(0.0), det_b(0.0)], 20.0);
        assert_eq!(r0.len(), 2);
        let id_a = r0.iter().find(|&&(_, di)| di == 0).unwrap().0;
        let id_b = r0.iter().find(|&&(_, di)| di == 1).unwrap().0;
        assert_ne!(id_a, id_b);

        for step in 1..8 {
            tracker.predict(1.0);
            let t = step as f32;
            let r = tracker.update(&[det_a(t), det_b(t)], 20.0);
            let matched_a = r.iter().find(|&&(id, _)| id == id_a);
            let matched_b = r.iter().find(|&&(id, _)| id == id_b);
            assert_eq!(matched_a.map(|&(_, di)| di), Some(0), "object A identity must persist through the crossing");
            assert_eq!(matched_b.map(|&(_, di)| di), Some(1), "object B identity must persist through the crossing");
        }

        let r_new = tracker.update(&[det_a(8.0), det_b(8.0), Detection { x: 200.0, y: 200.0, id_hint: None }], 20.0);
        let new_id = r_new.iter().find(|&&(_, di)| di == 2).unwrap().0;
        assert_ne!(new_id, id_a);
        assert_ne!(new_id, id_b);

        for _ in 0..(MOT_MAX_MISSED as usize + 2) {
            tracker.predict(1.0);
            tracker.update(&[], 20.0);
        }
        let r_after_gap = tracker.update(&[Detection { x: 200.0, y: 200.0, id_hint: None }], 5.0);
        let (revived_id, _) = r_after_gap[0];
        assert_ne!(revived_id, new_id, "a track missing for longer than MOT_MAX_MISSED must be pruned, so this detection spawns a fresh id");
    }
    // #endregion 🔖️MotTests

    // #region 🔖️Trajectory3dTests
    #[test]
    fn triangulate_tracks_recovers_a_slow_moving_point_from_a_single_orbiting_camera() {
        let intr = pinhole(700.0, 700.0, 320.0, 240.0);
        let n_frames = 24u32;
        let p0 = [0.2, 0.1, 0.0];
        let v = [0.01, 0.005, 0.0];

        let mut cams_per_frame = Vec::new();
        let mut frame_timestamps = std::collections::HashMap::new();
        let mut samples = Vec::new();
        for f in 0..n_frames {
            let ts = f64::from(f);
            let angle = f64::from(f) * 0.08;
            let eye = [6.0 * angle.cos(), 6.0 * angle.sin(), 1.2];
            let pose = CameraPose(look_at_pose(eye, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]));
            let point = add3(p0, scale3(v, ts));
            let Some(px) = remodel_camera::reproject(&intr, &pose, point) else { continue };
            cams_per_frame.push((f, pose, intr));
            frame_timestamps.insert(f, ts);
            samples.push((f, px[0] as f32, px[1] as f32));
        }
        let track = Track2d { id: 0, samples };
        let trajectories = triangulate_tracks(&cams_per_frame, std::slice::from_ref(&track), &frame_timestamps);
        assert_eq!(trajectories.len(), 1);
        let traj = &trajectories[0];
        assert!(traj.samples.len() > 10, "expected most frames to triangulate successfully");
        for &(ts, pos) in &traj.samples {
            let truth = add3(p0, scale3(v, ts));
            assert!(dist3(pos, truth) < 0.3, "triangulated position too far from ground truth: got {pos:?} want {truth:?}");
        }
    }
    // #endregion 🔖️Trajectory3dTests

    // #region 🔖️KinematicsTests
    #[test]
    fn velocity_and_acceleration_recover_planted_constant_motion() {
        let p0 = [1.0, -2.0, 0.5];
        let v = [0.3, -0.1, 0.2];
        let samples: Vec<(f64, [f64; 3])> = (0..20).map(|i| (f64::from(i), add3(p0, scale3(v, f64::from(i))))).collect();
        let traj = Trajectory3d { samples };
        let vel = velocity(&traj);
        for sample in &vel[4..16] {
            assert!(dist3(*sample, v) < 1e-3, "velocity mismatch: got {sample:?} want {v:?}");
        }

        let a = [0.05, -0.02, 0.01];
        let samples: Vec<(f64, [f64; 3])> = (0..20)
            .map(|i| {
                let t = f64::from(i);
                (t, add3(add3(p0, scale3(v, t)), scale3(a, 0.5 * t * t)))
            })
            .collect();
        let traj = Trajectory3d { samples };
        let acc = acceleration(&traj);
        for sample in &acc[4..16] {
            assert!(dist3(*sample, a) < 1e-2, "acceleration mismatch: got {sample:?} want {a:?}");
        }
    }

    #[test]
    fn neighborhood_affine_strain_distinguishes_rotation_from_stretch() {
        let mut state = 42u64;
        let cloud_t0: Vec<[f64; 3]> = (0..40).map(|_| [lcg_next(&mut state) * 4.0 - 2.0, lcg_next(&mut state) * 4.0 - 2.0, lcg_next(&mut state) * 4.0 - 2.0]).collect();

        let angle = 0.3f64;
        let (c, s) = (angle.cos(), angle.sin());
        let rotated: Vec<[f64; 3]> = cloud_t0.iter().map(|&p| [c * p[0] - s * p[1], s * p[0] + c * p[1], p[2]]).collect();
        let rot_pairs = neighborhood_affine_strain(&cloud_t0, &rotated, 2.5);
        assert!(!rot_pairs.is_empty(), "expected at least one neighbourhood with enough points");
        for pair in &rot_pairs {
            assert!(pair.strain < 1e-6, "pure rotation should report near-zero strain, got {}", pair.strain);
        }

        let (sx, sy, sz) = (1.3, 1.0, 0.9);
        let stretched: Vec<[f64; 3]> = cloud_t0.iter().map(|&p| [p[0] * sx, p[1] * sy, p[2] * sz]).collect();
        let expected = ((0.5f64 * (sx * sx - 1.0)).powi(2) + (0.5f64 * (sy * sy - 1.0)).powi(2) + (0.5f64 * (sz * sz - 1.0)).powi(2)).sqrt();
        let stretch_pairs = neighborhood_affine_strain(&cloud_t0, &stretched, 2.5);
        assert!(!stretch_pairs.is_empty());
        for pair in &stretch_pairs {
            assert!((pair.strain - expected).abs() < 1e-6, "got {} want {}", pair.strain, expected);
        }
    }
    // #endregion 🔖️KinematicsTests

    // #region 🔖️ModalTests
    #[test]
    fn modal_analysis_recovers_a_planted_frequency() {
        let fps = 50.0;
        let n = 2048usize;
        let target_hz = 4.0;
        let mut tracks = Vec::new();
        for track_idx in 0..3 {
            let phase = f64::from(track_idx) * 0.4;
            let samples: Vec<(u32, f32, f32)> = (0..n)
                .map(|i| {
                    let t = i as f64 / fps;
                    let y = 5.0 * (2.0 * std::f64::consts::PI * target_hz * t + phase).sin();
                    (i as u32, 0.0, y as f32)
                })
                .collect();
            tracks.push(Track2d { id: track_idx as u32, samples });
        }
        let result = modal_analysis(&tracks, fps, 0, 2);
        assert!(!result.frequencies_hz.is_empty(), "expected at least one detected mode");
        let closest = result.frequencies_hz.iter().min_by(|a, b| (**a - target_hz).abs().total_cmp(&(**b - target_hz).abs())).unwrap();
        assert!((closest - target_hz).abs() < 0.2, "closest detected frequency {closest} too far from planted {target_hz}");
        assert_eq!(result.mode_shapes[0].len(), tracks.len());
    }
    // #endregion 🔖️ModalTests

    // #region 🔖️SyncTests
    #[test]
    fn estimate_offset_and_refine_subframe_recover_a_fractional_shift() {
        let f = |t: f64| (2.0 * std::f64::consts::PI * t / 37.0).sin() + 0.5 * (2.0 * std::f64::consts::PI * t / 11.0).sin();
        let n = 220usize;
        let signal_a: Vec<f64> = (0..n).map(|i| f(i as f64)).collect();
        let true_offset = 3.6;
        let signal_b: Vec<f64> = (0..n).map(|i| f(i as f64 - true_offset)).collect();

        let coarse = estimate_offset(&signal_a, &signal_b, 30.0);
        assert!(coarse.confidence > 0.5, "expected a strong correlation peak, got confidence {}", coarse.confidence);
        let refined = refine_subframe(&signal_a, &signal_b, coarse.offset_frames);
        assert!((refined - true_offset).abs() < 0.1, "refined offset {refined} too far from planted {true_offset}");
    }
    // #endregion 🔖️SyncTests

    // #region 🔖️RollingShutterCompTests
    #[test]
    fn rolling_shutter_velocity_and_rectify_maps_are_sane() {
        let mut tracks = Vec::new();
        for (idx, &row) in [10.0f32, 30.0, 50.0, 70.0].iter().enumerate() {
            let base_vx = 2.0f32;
            let k = 0.05f32;
            let dx = base_vx + k * row;
            tracks.push(Track2d { id: idx as u32, samples: vec![(0, 100.0, row), (1, 100.0 + dx, row)] });
        }
        let model = estimate_rs_velocity(&tracks, 100);
        assert!((model[2] - 0.05).abs() < 0.01, "expected kx close to planted 0.05, got {}", model[2]);
        assert!(model[3].abs() < 0.01, "expected ky close to 0");

        let (map_x, _map_y) = build_rs_rectify_maps(model, 20, 100, 1e-5);
        assert_eq!(map_x.len(), 20 * 100);
        let idx = 90usize * 20 + 5;
        assert!((map_x[idx] - 5.0).abs() > 0.5, "expected a nonzero rectification shift far from the reference row");

        let mut flat_tracks = Vec::new();
        for (idx, &row) in [10.0f32, 30.0, 50.0, 70.0].iter().enumerate() {
            flat_tracks.push(Track2d { id: idx as u32, samples: vec![(0, 100.0, row), (1, 103.0, row)] });
        }
        let flat_model = estimate_rs_velocity(&flat_tracks, 100);
        let (flat_map_x, flat_map_y) = build_rs_rectify_maps(flat_model, 20, 100, 1e-5);
        for row in 0..100usize {
            for col in 0..20usize {
                let i = row * 20 + col;
                assert!((flat_map_x[i] - col as f32).abs() < 0.05, "pure translation should yield near-zero x rectification");
                assert!((flat_map_y[i] - row as f32).abs() < 0.05, "pure translation should yield near-zero y rectification");
            }
        }
    }
    // #endregion 🔖️RollingShutterCompTests

    // #region 🔖️StabilizeTests
    #[test]
    fn smooth_camera_path_reduces_planted_jitter() {
        let true_xi = [0.02, 0.0, 0.0, 0.0, 0.0, 0.03];
        let n = 81;
        let mut true_poses = Vec::with_capacity(n);
        true_poses.push(Se3::identity());
        for i in 1..n {
            true_poses.push(Se3::exp(true_xi).semio_compose_rs(&true_poses[i - 1]));
        }
        let mut state = 7u64;
        let jittered: Vec<Se3> = true_poses
            .iter()
            .map(|p| {
                let jitter: [f64; 6] = std::array::from_fn(|_| (lcg_next(&mut state) - 0.5) * 0.01);
                Se3::exp(jitter).semio_compose_rs(p)
            })
            .collect();
        let smoothed = smooth_camera_path(&jittered, 15);

        let orig_err: f64 = jittered.iter().zip(true_poses.iter()).map(|(a, b)| se3_error_norm(a, b)).sum();
        let smooth_err: f64 = smoothed.iter().zip(true_poses.iter()).map(|(a, b)| se3_error_norm(a, b)).sum();
        assert!(smooth_err < orig_err * 0.85, "smoothing should reduce total pose error: orig {orig_err} smoothed {smooth_err}");
    }

    #[test]
    fn stabilization_warps_are_near_identity_for_matching_poses() {
        let intr = pinhole(500.0, 500.0, 160.0, 120.0);
        let pose = Se3::exp([0.0, 0.0, 0.0, 0.1, 0.0, 0.0]);
        let warps = stabilization_warps(&[pose], &[pose], &intr);
        let h = warps[0];
        for (r, row) in h.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                let expect = if r == c { 1.0 } else { 0.0 };
                assert!((v - expect).abs() < 1e-6, "identity delta rotation should yield an identity homography, got {h:?}");
            }
        }
    }
    // #endregion 🔖️StabilizeTests

    // #region 🔖️DeblurTests
    #[test]
    fn wiener_deconvolve_measurably_sharpens_a_blurred_image() {
        let size = 48u32;
        let mut sharp = ImageGray::new(size, size);
        let mut state = 99u64;
        for y in 0..size {
            for x in 0..size {
                let block = (x / 6 + y / 6) % 2;
                let noise = (lcg_next(&mut state) - 0.5) * 0.02;
                sharp.set(x, y, (block as f32 * 0.8 + 0.1 + noise as f32).clamp(0.0, 1.0));
            }
        }
        let psf = estimate_motion_psf((6.0, 0.0), 1.0);
        let mut blurred = ImageGray::new(size, size);
        let half = i64::from(psf.width / 2);
        for y in 0..size {
            for x in 0..size {
                let mut acc = 0.0f32;
                for ky in 0..psf.height {
                    for kx in 0..psf.width {
                        let sx = i64::from(x) + i64::from(kx) - half;
                        let sy = i64::from(y) + i64::from(ky) - half;
                        if sx >= 0 && sy >= 0 && (sx as u32) < size && (sy as u32) < size {
                            acc += sharp.get(sx as u32, sy as u32) * psf.kernel[(ky * psf.width + kx) as usize];
                        }
                    }
                }
                blurred.set(x, y, acc);
            }
        }
        let gradient_energy = |img: &ImageGray| -> f32 {
            let g = scharr_gradients(img);
            g.gx.iter().zip(g.gy.iter()).map(|(&gx, &gy)| gx * gx + gy * gy).sum()
        };
        let energy_sharp = gradient_energy(&sharp);
        let energy_blurred = gradient_energy(&blurred);
        assert!(energy_blurred < energy_sharp, "blur should reduce gradient energy");

        let deconvolved = wiener_deconvolve(&blurred, &psf, 200.0);
        let energy_deconv = gradient_energy(&deconvolved);
        assert!(energy_deconv > energy_blurred, "deconvolution should measurably sharpen the blurred image: blurred {energy_blurred} deconv {energy_deconv}");
    }
    // #endregion 🔖️DeblurTests

    // #region 🔖️NonRigidTests
    #[test]
    fn build_and_fit_deformation_graph_recovers_a_planted_bend_for_held_out_points() {
        let radius = 3.0;
        let mut seed_points = Vec::new();
        for j in 0..4 {
            for i in 0..4 {
                seed_points.push([f64::from(i) * 2.0 - 3.0, f64::from(j) * 2.0 - 3.0, 0.0]);
            }
        }
        let graph = build_deformation_graph(&seed_points, radius);

        let mut src_points = Vec::new();
        for j in 0..9 {
            for i in 0..9 {
                src_points.push([f64::from(i) - 4.0, f64::from(j) - 4.0, 0.0]);
            }
        }
        let bend_radius = 8.0;
        let bend = |p: [f64; 3]| -> [f64; 3] {
            let theta = p[0] / bend_radius;
            [bend_radius * theta.sin(), p[1], bend_radius * (1.0 - theta.cos())]
        };

        let mut dst_correspondences = Vec::new();
        let mut held_out = Vec::new();
        for (idx, &p) in src_points.iter().enumerate() {
            if idx % 5 == 0 {
                held_out.push(idx);
            } else {
                dst_correspondences.push((idx, bend(p)));
            }
        }

        let fitted = fit_deformation(&graph, &src_points, &dst_correspondences, radius);
        let mut max_err = 0.0f64;
        for &idx in &held_out {
            let predicted = deform_point(&fitted, src_points[idx], radius);
            let truth = bend(src_points[idx]);
            max_err = max_err.max(dist3(predicted, truth));
        }
        assert!(max_err < 0.6, "held-out bend recovery error too large: {max_err}");
    }
    // #endregion 🔖️NonRigidTests

    // #region 🔖️Pose6dTests
    #[test]
    fn track_rigid_body_recovers_a_known_rotating_translating_cube_with_partial_visibility() {
        let intr = pinhole(700.0, 700.0, 320.0, 240.0);
        let model_points: Vec<[f64; 3]> = (0..8).map(|i| [if i & 1 == 0 { -0.3 } else { 0.3 }, if i & 2 == 0 { -0.3 } else { 0.3 }, if i & 4 == 0 { -0.3 } else { 0.3 }]).collect();

        let n_frames = 16u32;
        let mut per_frame_obs = Vec::new();
        let mut truth = Vec::new();
        for f in 0..n_frames {
            let t = f64::from(f);
            let angle = t * 0.1;
            let rot = So3::exp([0.0, angle, 0.0]);
            let translation = [0.0, 0.0, 5.0 + 0.05 * t];
            let pose = Se3 { r: rot, t: translation };
            let hidden = f as usize % 8;
            let obs: Vec<Option<[f64; 2]>> = model_points
                .iter()
                .enumerate()
                .map(|(i, &mp)| {
                    if i == hidden {
                        return None;
                    }
                    let cam_pt = pose.act(mp);
                    intr.project(cam_pt)
                })
                .collect();
            per_frame_obs.push((f, obs));
            truth.push((f, pose));
        }

        let recovered = track_rigid_body(&model_points, &per_frame_obs, &intr);
        assert!(recovered.len() > n_frames as usize / 2, "expected most frames to recover a pose");
        for &(frame, pose) in &recovered {
            let (_, true_pose) = truth.iter().find(|&&(f, _)| f == frame).unwrap();
            let err = se3_error_norm(&pose, true_pose);
            assert!(err < 0.1, "frame {frame}: pose error {err} too large");
        }
    }
    // #endregion 🔖️Pose6dTests
}
// #endregion 🔖️Tests
