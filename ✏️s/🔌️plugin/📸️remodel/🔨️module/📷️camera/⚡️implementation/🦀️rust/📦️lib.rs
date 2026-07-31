//! 📷️ Camera models and calibration: pinhole, Brown-Conrady and fisheye distortion, rolling shutter, rigs, planar and self-calibration.

pub use mathematical_algebra::{MatD, VecD};
pub use mathematical_lie::Se3;
pub use mathematical_optimize::{levenberg_marquardt, LeastSquaresProblem, LmConfig, LmResult, RobustLoss};

use mathematical_algebra::{pseudo_inverse, svd_nullvector, vec3d_cross, vec3d_length, Mat3d};
use mathematical_lie::So3;

// #region 🔖️Intrinsics
/// 📷️ Pinhole camera intrinsic parameters: focal lengths, principal point, pixel skew, and a lens distortion model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intrinsics {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub skew: f64,
    pub distortion: Distortion,
}

/// 🌀️ Lens distortion model applied to normalized camera-plane coordinates before the linear intrinsic map.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Distortion {
    None,
    BrownConrady { k1: f64, k2: f64, k3: f64, p1: f64, p2: f64 },
    FisheyeEquidistant { k1: f64, k2: f64, k3: f64, k4: f64 },
}

impl Distortion {
    fn distort(&self, p: [f64; 2]) -> [f64; 2] {
        match *self {
            Self::None => p,
            Self::BrownConrady { k1, k2, k3, p1, p2 } => {
                let (x, y) = (p[0], p[1]);
                let r2 = x * x + y * y;
                let radial = 1.0 + k1 * r2 + k2 * r2 * r2 + k3 * r2 * r2 * r2;
                let dx = 2.0 * p1 * x * y + p2 * (r2 + 2.0 * x * x);
                let dy = p1 * (r2 + 2.0 * y * y) + 2.0 * p2 * x * y;
                [x * radial + dx, y * radial + dy]
            }
            Self::FisheyeEquidistant { k1, k2, k3, k4 } => {
                let (x, y) = (p[0], p[1]);
                let r = (x * x + y * y).sqrt();
                if r < 1e-12 {
                    return [x, y];
                }
                let theta = r.atan();
                let t2 = theta * theta;
                let theta_d = theta * (1.0 + k1 * t2 + k2 * t2 * t2 + k3 * t2 * t2 * t2 + k4 * t2 * t2 * t2 * t2);
                let scale = theta_d / r;
                [x * scale, y * scale]
            }
        }
    }
}

impl Distortion {
    /// 🔢️ Free distortion-parameter count feeding the `[fx, fy, cx, cy, skew, <distortion params>]`
    /// intrinsics-Jacobian ordering used by [`reprojection_jacobians`].
    fn param_count(&self) -> usize {
        match self {
            Self::None => 0,
            Self::BrownConrady { .. } => 5,
            Self::FisheyeEquidistant { .. } => 4,
        }
    }

    /// 🔬️ Distorts `p` and returns alongside it the 2x2 Jacobian `d(distorted)/d(p)` and the per-parameter
    /// derivatives `d(distorted)/d(param)` (empty for [`Distortion::None`], ordered `[k1,k2,k3,p1,p2]` for
    /// [`Distortion::BrownConrady`], `[k1,k2,k3,k4]` for [`Distortion::FisheyeEquidistant`]) — the shared
    /// chain-rule kernel [`reprojection_jacobians`] builds its intrinsics/point/pose blocks on top of.
    fn distort_with_jacobian(&self, p: [f64; 2]) -> ([f64; 2], [[f64; 2]; 2], Vec<[f64; 2]>) {
        match *self {
            Self::None => (p, [[1.0, 0.0], [0.0, 1.0]], Vec::new()),
            Self::BrownConrady { k1, k2, k3, p1, p2 } => {
                let (x, y) = (p[0], p[1]);
                let r2 = x * x + y * y;
                let r4 = r2 * r2;
                let r6 = r4 * r2;
                let radial = 1.0 + k1 * r2 + k2 * r4 + k3 * r6;
                let kr = k1 + 2.0 * k2 * r2 + 3.0 * k3 * r4;
                let xd = x * radial + 2.0 * p1 * x * y + p2 * (r2 + 2.0 * x * x);
                let yd = y * radial + p1 * (r2 + 2.0 * y * y) + 2.0 * p2 * x * y;
                let dxd_dx = radial + 2.0 * x * x * kr + 2.0 * p1 * y + 6.0 * p2 * x;
                let dxd_dy = 2.0 * x * y * kr + 2.0 * p1 * x + 2.0 * p2 * y;
                let dyd_dx = 2.0 * x * y * kr + 2.0 * p1 * x + 2.0 * p2 * y;
                let dyd_dy = radial + 2.0 * y * y * kr + 6.0 * p1 * y + 2.0 * p2 * x;
                let params = vec![[x * r2, y * r2], [x * r4, y * r4], [x * r6, y * r6], [2.0 * x * y, r2 + 2.0 * y * y], [r2 + 2.0 * x * x, 2.0 * x * y]];
                ([xd, yd], [[dxd_dx, dxd_dy], [dyd_dx, dyd_dy]], params)
            }
            Self::FisheyeEquidistant { k1, k2, k3, k4 } => {
                let (x, y) = (p[0], p[1]);
                let r2 = x * x + y * y;
                let r = r2.sqrt();
                if r < 1e-12 {
                    return ([x, y], [[1.0, 0.0], [0.0, 1.0]], vec![[0.0, 0.0]; 4]);
                }
                let theta = r.atan();
                let t2 = theta * theta;
                let t3 = t2 * theta;
                let t5 = t3 * t2;
                let t7 = t5 * t2;
                let t9 = t7 * t2;
                let poly = 1.0 + k1 * t2 + k2 * t2 * t2 + k3 * t2 * t2 * t2 + k4 * t2 * t2 * t2 * t2;
                let theta_d = theta * poly;
                let scale = theta_d / r;
                let dpoly_dtheta2 = k1 + 2.0 * k2 * t2 + 3.0 * k3 * t2 * t2 + 4.0 * k4 * t2 * t2 * t2;
                let dthetad_dtheta = poly + 2.0 * t2 * dpoly_dtheta2;
                let dtheta_dr = 1.0 / (1.0 + r2);
                let dthetad_dr = dthetad_dtheta * dtheta_dr;
                let dscale_dr = (dthetad_dr * r - theta_d) / r2;
                let dscale_dx = dscale_dr * x / r;
                let dscale_dy = dscale_dr * y / r;
                let dxd_dx = scale + x * dscale_dx;
                let dxd_dy = x * dscale_dy;
                let dyd_dx = y * dscale_dx;
                let dyd_dy = scale + y * dscale_dy;
                let dscale_dk = |theta_pow: f64| theta_pow / r;
                let params = vec![
                    [x * dscale_dk(t3), y * dscale_dk(t3)],
                    [x * dscale_dk(t5), y * dscale_dk(t5)],
                    [x * dscale_dk(t7), y * dscale_dk(t7)],
                    [x * dscale_dk(t9), y * dscale_dk(t9)],
                ];
                ([x * scale, y * scale], [[dxd_dx, dxd_dy], [dyd_dx, dyd_dy]], params)
            }
        }
    }
}

impl Intrinsics {
    /// 🎯️ Projects a camera-space point to pixel coordinates, applying distortion to the normalized coordinates first; `None` when the point is behind the camera (`z <= 0`).
    pub fn project(&self, p_cam: [f64; 3]) -> Option<[f64; 2]> {
        if p_cam[2] <= 0.0 {
            return None;
        }
        let x = p_cam[0] / p_cam[2];
        let y = p_cam[1] / p_cam[2];
        let [xd, yd] = self.distortion.distort([x, y]);
        Some([self.fx * xd + self.skew * yd + self.cx, self.fy * yd + self.cy])
    }

    /// 🔬️ Newton iteration (finite-difference Jacobian, 8 steps) undoing the forward distortion map: finds normalized coordinates `p` such that `distortion.distort(p) == distorted`.
    fn newton_undistort(&self, distorted: [f64; 2]) -> [f64; 2] {
        if matches!(self.distortion, Distortion::None) {
            return distorted;
        }
        let mut p = distorted;
        let eps = 1e-6;
        for _ in 0..8 {
            let fp = self.distortion.distort(p);
            let residual = [fp[0] - distorted[0], fp[1] - distorted[1]];
            if residual[0].abs() < 1e-14 && residual[1].abs() < 1e-14 {
                break;
            }
            let fx0 = self.distortion.distort([p[0] + eps, p[1]]);
            let fy0 = self.distortion.distort([p[0], p[1] + eps]);
            let j = [[(fx0[0] - fp[0]) / eps, (fy0[0] - fp[0]) / eps], [(fx0[1] - fp[1]) / eps, (fy0[1] - fp[1]) / eps]];
            let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];
            if det.abs() < 1e-300 {
                break;
            }
            let dx = (j[1][1] * residual[0] - j[0][1] * residual[1]) / det;
            let dy = (j[0][0] * residual[1] - j[1][0] * residual[0]) / det;
            p = [p[0] - dx, p[1] - dy];
        }
        p
    }

    /// ↩️ Inverse of the linear intrinsic map followed by Newton undistortion, returning a ray direction with `z = 1`.
    pub fn unproject_ray(&self, p_px: [f64; 2]) -> [f64; 3] {
        let yd = (p_px[1] - self.cy) / self.fy;
        let xd = (p_px[0] - self.cx - self.skew * yd) / self.fx;
        let [x, y] = self.newton_undistort([xd, yd]);
        [x, y, 1.0]
    }

    /// ↩️ Undistorts a normalized (post-linear-map) point via fixed-point/Newton iteration, undoing [`Distortion::distort`].
    pub fn undistort_point(&self, p_norm: [f64; 2]) -> [f64; 2] {
        self.newton_undistort(p_norm)
    }
}
// #endregion 🔖️Intrinsics

// #region 🔖️Extrinsics
/// 🎥️ World-to-camera rigid transform: `p_cam = pose.0.act(p_world)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraPose(pub Se3);

fn se3_at(x: &VecD, offset: usize) -> Se3 {
    Se3::exp(std::array::from_fn(|k| x.get(offset + k)))
}

fn retract_se3_block(x: &VecD, dx: &VecD, offset: usize) -> [f64; 6] {
    let pose = se3_at(x, offset);
    let dxi: [f64; 6] = std::array::from_fn(|k| dx.get(offset + k));
    Se3::exp(dxi).compose(&pose).log()
}

/// 🎯️ Transforms a world point by a camera pose and projects it through the camera's intrinsics.
pub fn reproject(intr: &Intrinsics, pose: &CameraPose, point_world: [f64; 3]) -> Option<[f64; 2]> {
    intr.project(pose.0.act(point_world))
}

/// 🎯️ Reprojection residual `predicted_pixel - observed_pixel` — the shared kernel behind bundle
/// adjustment, PnP and calibration in this crate and downstream (`remodel_sfm`). Points behind the camera
/// (`z <= 0`) fall back to a large constant residual so callers never see `NaN` or a discontinuous jump.
pub fn reprojection_residual(intrinsics: &Intrinsics, pose: &CameraPose, point_world: [f64; 3], observed_pixel: [f64; 2]) -> [f64; 2] {
    match reproject(intrinsics, pose, point_world) {
        Some(pred) => [pred[0] - observed_pixel[0], pred[1] - observed_pixel[1]],
        None => [1.0e3, 1.0e3],
    }
}

/// 🔬️ Analytic Jacobians of [`reprojection_residual`], returned as `(d/d(pose tangent) 2x6, d/d(point) 2x3,
/// d/d(intrinsics) 2xK)`. The pose block is w.r.t. the camera's SE(3) *left*-perturbation tangent
/// `(rho, phi)` (matching how [`CameraPose`]-carrying least-squares problems in this crate retract via
/// `Se3::exp(dxi).compose(pose)`): since `p_cam ↦ p_cam + rho + phi × p_cam` to first order, `d(p_cam)/d(xi)
/// = [I₃ | -hat(p_cam)]`. The intrinsics block is ordered `[fx, fy, cx, cy, skew, <distortion params>]`
/// (`K = 5 + distortion.param_count()`), chaining the linear intrinsic map through
/// [`Distortion::distort_with_jacobian`]. Points behind the camera return all-zero Jacobians, matching
/// [`reprojection_residual`]'s constant-residual fallback so a damped LM step simply ignores the observation.
pub fn reprojection_jacobians(intrinsics: &Intrinsics, pose: &CameraPose, point_world: [f64; 3], _observed_pixel: [f64; 2]) -> (MatD, MatD, MatD) {
    let k = 5 + intrinsics.distortion.param_count();
    let mut j_pose = MatD::zeros(2, 6);
    let mut j_point = MatD::zeros(2, 3);
    let mut j_intr = MatD::zeros(2, k);

    let p_cam = pose.0.act(point_world);
    let pz = p_cam[2];
    if pz <= 1e-9 {
        return (j_pose, j_point, j_intr);
    }
    let (px, py) = (p_cam[0], p_cam[1]);
    let (x, y) = (px / pz, py / pz);

    let j_proj: [[f64; 3]; 2] = [[1.0 / pz, 0.0, -px / (pz * pz)], [0.0, 1.0 / pz, -py / (pz * pz)]];
    let (distorted, j_distort, param_derivs) = intrinsics.distortion.distort_with_jacobian([x, y]);
    let (xd, yd) = (distorted[0], distorted[1]);
    let j_lin: [[f64; 2]; 2] = [[intrinsics.fx, intrinsics.skew], [0.0, intrinsics.fy]];

    let j_uv_xy = mat2_mul(&j_lin, &j_distort);
    let j_uv_pcam = mat2x2_mul_2x3(&j_uv_xy, &j_proj);

    let neg_hat_pcam = skew3(scale3(p_cam, -1.0));
    let j_uv_phi = mat2x3_mul_3x3(&j_uv_pcam, &neg_hat_pcam);
    for row in 0..2 {
        for col in 0..3 {
            j_pose.set(row, col, j_uv_pcam[row][col]);
            j_pose.set(row, col + 3, j_uv_phi[row][col]);
        }
    }

    let j_uv_point = mat2x3_mul_3x3(&j_uv_pcam, &mat3d_rowmajor(&pose.0.r.0));
    for (row, cols) in j_uv_point.iter().enumerate() {
        for (col, &value) in cols.iter().enumerate() {
            j_point.set(row, col, value);
        }
    }

    j_intr.set(0, 0, xd);
    j_intr.set(0, 2, 1.0);
    j_intr.set(0, 4, yd);
    j_intr.set(1, 1, yd);
    j_intr.set(1, 3, 1.0);
    for (i, dparam) in param_derivs.iter().enumerate() {
        let (dxd, dyd) = (dparam[0], dparam[1]);
        j_intr.set(0, 5 + i, intrinsics.fx * dxd + intrinsics.skew * dyd);
        j_intr.set(1, 5 + i, intrinsics.fy * dyd);
    }

    (j_pose, j_point, j_intr)
}

/// 🧩️ Multi-camera multi-point reprojection least-squares problem: the shared residual kernel reused by
/// bundle adjustment, PnP and calibration solvers built on `remodel_camera`. The parameter vector packs an
/// SE(3) log-tangent per camera (6 values, interpreted as `Se3::exp(tangent)`) followed by an XYZ position
/// per point (3 values); `intrinsics` are fixed and shared across every camera and observation.
#[derive(Clone, Debug)]
pub struct ReprojectionProblem {
    pub observations: Vec<(usize, usize, [f64; 2])>,
    pub num_cameras: usize,
    pub num_points: usize,
    pub intrinsics: Intrinsics,
}

impl ReprojectionProblem {
    fn point_offset(&self, point_idx: usize) -> usize {
        self.num_cameras * 6 + point_idx * 3
    }

    fn point_at(&self, x: &VecD, point_idx: usize) -> [f64; 3] {
        let base = self.point_offset(point_idx);
        [x.get(base), x.get(base + 1), x.get(base + 2)]
    }
}

impl LeastSquaresProblem for ReprojectionProblem {
    fn residual_count(&self) -> usize {
        self.observations.len() * 2
    }

    fn parameter_count(&self) -> usize {
        self.num_cameras * 6 + self.num_points * 3
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        for (row, &(cam_idx, point_idx, obs)) in self.observations.iter().enumerate() {
            let pose = CameraPose(se3_at(x, cam_idx * 6));
            let point = self.point_at(x, point_idx);
            let pred = reproject(&self.intrinsics, &pose, point).unwrap_or([obs[0] + 1.0e3, obs[1] + 1.0e3]);
            out.set(2 * row, pred[0] - obs[0]);
            out.set(2 * row + 1, pred[1] - obs[1]);
        }
    }

    /// 🔬️ Analytic Jacobians assembled per-observation from [`reprojection_jacobians`]'s pose/point blocks
    /// (intrinsics are fixed here, so its intrinsics block is discarded), scattered into this problem's
    /// per-camera/per-point column ranges.
    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        for (row, &(cam_idx, point_idx, obs)) in self.observations.iter().enumerate() {
            let pose = CameraPose(se3_at(x, cam_idx * 6));
            let point = self.point_at(x, point_idx);
            let (j_pose, j_point, _j_intr) = reprojection_jacobians(&self.intrinsics, &pose, point, obs);
            let point_base = self.point_offset(point_idx);
            for r in 0..2 {
                for c in 0..6 {
                    out.set(2 * row + r, cam_idx * 6 + c, j_pose.get(r, c));
                }
                for c in 0..3 {
                    out.set(2 * row + r, point_base + c, j_point.get(r, c));
                }
            }
        }
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for cam_idx in 0..self.num_cameras {
            let updated = retract_se3_block(x, dx, cam_idx * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(cam_idx * 6 + k, v);
            }
        }
        let base = self.num_cameras * 6;
        for i in base..x.len() {
            out.set(i, x.get(i) + dx.get(i));
        }
        out
    }
}
// #endregion 🔖️Extrinsics

// #region 🔖️RollingShutter
/// 📐️↕️ Rolling-shutter readout model: per-row time delay and scan direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RollingShutterModel {
    pub line_delay_s: f64,
    pub readout: ReadoutDirection,
}

/// ↕️ Which way the sensor is scanned during rolling-shutter readout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadoutDirection {
    TopToBottom,
    BottomToTop,
}

fn scale6(xi: [f64; 6], s: f64) -> [f64; 6] {
    std::array::from_fn(|k| xi[k] * s)
}

/// 🎞️ Camera pose at a given sensor row under a linearized constant-velocity rolling-shutter model: the
/// row's readout time `t = effective_row * line_delay_s` (top-to-bottom counts rows directly, bottom-to-top
/// counts from the last row) integrates `velocity_se3` on top of `pose0` via the SE(3) exponential.
pub fn pose_at_row(model: &RollingShutterModel, row: u32, image_height: u32, pose0: &CameraPose, velocity_se3: [f64; 6]) -> CameraPose {
    let last_row = image_height.saturating_sub(1);
    let clamped_row = row.min(last_row);
    let effective_row = match model.readout {
        ReadoutDirection::TopToBottom => clamped_row,
        ReadoutDirection::BottomToTop => last_row - clamped_row,
    };
    let t = effective_row as f64 * model.line_delay_s;
    CameraPose(Se3::exp(scale6(velocity_se3, t)).compose(&pose0.0))
}

/// 🗺️ Per-pixel rolling-shutter rectification field: for every output pixel `(col, row)`, interpreted
/// under the reference pose `pose0`'s geometry, finds the source pixel in the *actual* rolling-shutter
/// image a consumer (e.g. `remodel_image::remap`) should sample to undo the skew. Direction-only: the
/// world-frame bearing of the output pixel is held fixed and re-projected through each candidate sensor
/// row's own pose (bounded 3-step fixed-point search, since "which row" and "what pose that row had" are
/// mutually dependent), ignoring translational parallax — the standard rolling-shutter-without-depth
/// approximation, exact for rotation-dominated motion. Pixels that never reproject in front of the camera
/// fall back to their own (unchanged) coordinate.
pub fn rectify_remap_field(intrinsics: &Intrinsics, model: &RollingShutterModel, pose0: &CameraPose, velocity_se3: [f64; 6], width: u32, height: u32) -> Vec<[f32; 2]> {
    let mut field = vec![[0.0f32; 2]; width as usize * height as usize];
    for row in 0..height {
        for col in 0..width {
            let idx = row as usize * width as usize + col as usize;
            let px = [col as f64 + 0.5, row as f64 + 0.5];
            let ray_cam0 = intrinsics.unproject_ray(px);
            let ray_world = pose0.0.r.inverse().act(ray_cam0);
            let mut src_row = row;
            let mut source_px = px;
            for _ in 0..3 {
                let pose_row = pose_at_row(model, src_row, height, pose0, velocity_se3);
                let ray_local = pose_row.0.r.act(ray_world);
                if ray_local[2] <= 1e-9 {
                    source_px = px;
                    break;
                }
                source_px = intrinsics.project(ray_local).unwrap_or(px);
                let next_row = (source_px[1].round().clamp(0.0, height.saturating_sub(1) as f64)) as u32;
                if next_row == src_row {
                    break;
                }
                src_row = next_row;
            }
            field[idx] = [source_px[0] as f32, source_px[1] as f32];
        }
    }
    field
}
// #endregion 🔖️RollingShutter

// #region 🔖️Rig
/// 🧷️ One camera's fixed extrinsic pose within a multi-camera rig, relative to the rig's own reference
/// frame: `p_camera = pose_in_rig.act(p_rig)`.
#[derive(Clone, Debug, PartialEq)]
pub struct RigExtrinsic {
    pub camera_id: String,
    pub pose_in_rig: Se3,
}

/// 🧷️ Multi-camera rig: one fixed extrinsic per camera, relative to the rig's own reference frame.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraRig {
    pub cameras: Vec<RigExtrinsic>,
}

/// 🔗️ Composes a camera's fixed rig-relative pose with the rig's world pose (`rig_pose`: world-to-rig)
/// into that camera's world-to-camera transform.
pub fn rig_pose_of_camera(rig_pose: &Se3, camera_in_rig: &Se3) -> Se3 {
    camera_in_rig.compose(rig_pose)
}

/// 📡️ Projects a world point through one rig camera via [`rig_pose_of_camera`], then reprojects. `None`
/// if `camera_id` isn't in the rig or the point is behind the camera.
pub fn rig_project(rig: &CameraRig, intrinsics: &[Intrinsics], camera_id: &str, rig_pose: &CameraPose, point_world: [f64; 3]) -> Option<[f64; 2]> {
    let idx = rig.cameras.iter().position(|c| c.camera_id == camera_id)?;
    let world_to_camera = rig_pose_of_camera(&rig_pose.0, &rig.cameras[idx].pose_in_rig);
    reproject(intrinsics.get(idx)?, &CameraPose(world_to_camera), point_world)
}

/// 📡️ One 2D observation feeding [`refine_rig`]: a known world point seen by one rig camera at one
/// shared-timestamp rig instance.
#[derive(Clone, Copy, Debug)]
pub struct RigObservation {
    pub instance_idx: usize,
    pub camera_idx: usize,
    pub point_world: [f64; 3],
    pub pixel: [f64; 2],
}

/// 📦️ Outcome of [`refine_rig`]: refined per-camera rig extrinsics (camera 0 fixed at its initial
/// `pose_in_rig`, the rig's frame anchor) and refined per-instance world-to-rig poses.
#[derive(Clone, Debug)]
pub struct RigRefinementResult {
    pub cameras: Vec<RigExtrinsic>,
    pub rig_poses: Vec<Se3>,
    pub rms_px: f64,
    pub converged: bool,
}

/// 🧩️ Multi-camera rig bundle-style refinement problem: `(num_cameras - 1)` free camera-in-rig SE(3)
/// tangents (camera 0 is the fixed rig-frame anchor) followed by `num_instances` free world-to-rig SE(3)
/// tangents. Since `world_to_camera = camera_in_rig ∘ rig_pose`, a left-perturbation `da` of `camera_in_rig`
/// acts directly on `world_to_camera`'s tangent, while a left-perturbation `db` of `rig_pose` acts through
/// `camera_in_rig`'s adjoint (`exp(da) ∘ A ∘ exp(db) ∘ B = exp(da + Adj_A(db)) ∘ A ∘ B` to first order) — so
/// [`Se3::adjoint`] turns [`reprojection_jacobians`]'s single pose block into both halves without needing a
/// second hand-derived chain rule.
struct RigRefinementProblem<'a> {
    intrinsics: &'a [Intrinsics],
    observations: &'a [RigObservation],
    num_cameras: usize,
    num_instances: usize,
    camera0_pose_in_rig: Se3,
}

impl RigRefinementProblem<'_> {
    fn camera_in_rig(&self, x: &VecD, camera_idx: usize) -> Se3 {
        if camera_idx == 0 {
            self.camera0_pose_in_rig
        } else {
            se3_at(x, (camera_idx - 1) * 6)
        }
    }

    fn rig_instance_base(&self) -> usize {
        (self.num_cameras - 1) * 6
    }
}

impl LeastSquaresProblem for RigRefinementProblem<'_> {
    fn residual_count(&self) -> usize {
        self.observations.len() * 2
    }

    fn parameter_count(&self) -> usize {
        (self.num_cameras - 1) * 6 + self.num_instances * 6
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let base = self.rig_instance_base();
        for (row, obs) in self.observations.iter().enumerate() {
            let camera_in_rig = self.camera_in_rig(x, obs.camera_idx);
            let rig_pose = se3_at(x, base + obs.instance_idx * 6);
            let world_to_camera = rig_pose_of_camera(&rig_pose, &camera_in_rig);
            let r = reprojection_residual(&self.intrinsics[obs.camera_idx], &CameraPose(world_to_camera), obs.point_world, obs.pixel);
            out.set(2 * row, r[0]);
            out.set(2 * row + 1, r[1]);
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        let base = self.rig_instance_base();
        for (row, obs) in self.observations.iter().enumerate() {
            let camera_in_rig = self.camera_in_rig(x, obs.camera_idx);
            let rig_pose = se3_at(x, base + obs.instance_idx * 6);
            let world_to_camera = rig_pose_of_camera(&rig_pose, &camera_in_rig);
            let (j_pose, _j_point, _j_intr) = reprojection_jacobians(&self.intrinsics[obs.camera_idx], &CameraPose(world_to_camera), obs.point_world, obs.pixel);
            if obs.camera_idx != 0 {
                let camera_base = (obs.camera_idx - 1) * 6;
                for r in 0..2 {
                    for c in 0..6 {
                        out.set(2 * row + r, camera_base + c, j_pose.get(r, c));
                    }
                }
            }
            let adjoint = camera_in_rig.adjoint();
            let j_db = j_pose.matmul(&adjoint);
            let instance_base = base + obs.instance_idx * 6;
            for r in 0..2 {
                for c in 0..6 {
                    out.set(2 * row + r, instance_base + c, j_db.get(r, c));
                }
            }
        }
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for i in 0..(self.num_cameras - 1) {
            let updated = retract_se3_block(x, dx, i * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(i * 6 + k, v);
            }
        }
        let base = self.rig_instance_base();
        for i in 0..self.num_instances {
            let updated = retract_se3_block(x, dx, base + i * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(base + i * 6 + k, v);
            }
        }
        out
    }
}

/// 🧷️ Jointly refines a multi-camera rig's per-camera extrinsics and per-instance world-to-rig poses from
/// shared-timestamp observations of known world points, via Levenberg-Marquardt (Huber-robustified) over
/// the shared [`reprojection_residual`]. Camera 0's `pose_in_rig` is held fixed at its initial value — it
/// defines the rig's own reference frame, so refining it too would leave the whole rig gauge-free and the
/// normal equations singular. Requires at least one camera, one rig instance and one observation.
pub fn refine_rig(intrinsics: &[Intrinsics], initial: &CameraRig, initial_rig_poses: &[Se3], observations: &[RigObservation]) -> Result<RigRefinementResult, CameraError> {
    let num_cameras = initial.cameras.len();
    let num_instances = initial_rig_poses.len();
    if num_cameras == 0 || num_instances == 0 || observations.is_empty() {
        return Err(CameraError::TooFewViews);
    }
    let problem = RigRefinementProblem { intrinsics, observations, num_cameras, num_instances, camera0_pose_in_rig: initial.cameras[0].pose_in_rig };
    let base = problem.rig_instance_base();
    let mut x0 = VecD::zeros(base + num_instances * 6);
    for i in 1..num_cameras {
        for (k, v) in initial.cameras[i].pose_in_rig.log().into_iter().enumerate() {
            x0.set((i - 1) * 6 + k, v);
        }
    }
    for (i, pose) in initial_rig_poses.iter().enumerate() {
        for (k, v) in pose.log().into_iter().enumerate() {
            x0.set(base + i * 6 + k, v);
        }
    }
    let cfg = LmConfig { max_iters: 100, loss: RobustLoss::Huber(2.0), ..LmConfig::default() };
    let residual_count = problem.residual_count();
    let result = levenberg_marquardt(&problem, x0, &cfg);
    if !result.converged {
        return Err(CameraError::Convergence);
    }
    let mut cameras = vec![initial.cameras[0].clone()];
    for i in 1..num_cameras {
        cameras.push(RigExtrinsic { camera_id: initial.cameras[i].camera_id.clone(), pose_in_rig: se3_at(&result.x, (i - 1) * 6) });
    }
    let rig_poses: Vec<Se3> = (0..num_instances).map(|i| se3_at(&result.x, base + i * 6)).collect();
    let rms_px = (4.0 * result.cost / residual_count as f64).sqrt();
    Ok(RigRefinementResult { cameras, rig_poses, rms_px, converged: result.converged })
}
// #endregion 🔖️Rig

// #region 🔖️Error
/// ⚠️ Error type for fallible camera-geometry operations: homography estimation, planar calibration, and self-calibration solves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraError {
    TooFewViews,
    DegenerateHomography,
    SingularSystem,
    Convergence,
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewViews => write!(f, "fewer than the minimum required number of calibration views"),
            Self::DegenerateHomography => write!(f, "homography estimation is degenerate"),
            Self::SingularSystem => write!(f, "linear system is singular or has no valid solution"),
            Self::Convergence => write!(f, "nonlinear refinement failed to converge"),
        }
    }
}

impl std::error::Error for CameraError {}
// #endregion 🔖️Error

// #region 🔖️Mat3Helpers
fn scale3(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn skew3(v: [f64; 3]) -> [[f64; 3]; 3] {
    [[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]]
}

fn mat3_vec(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|r| m[r][0] * v[0] + m[r][1] * v[1] + m[r][2] * v[2])
}

fn mat3_mul(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}

fn transpose3(m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m[c][r]))
}

fn mat3d_rowmajor(m: &Mat3d) -> [[f64; 3]; 3] {
    std::array::from_fn(|r| std::array::from_fn(|c| m.cols[c][r]))
}

fn mat2_mul(a: &[[f64; 2]; 2], b: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    std::array::from_fn(|r| std::array::from_fn(|c| a[r][0] * b[0][c] + a[r][1] * b[1][c]))
}

fn mat2x2_mul_2x3(a: &[[f64; 2]; 2], b: &[[f64; 3]; 2]) -> [[f64; 3]; 2] {
    std::array::from_fn(|r| std::array::from_fn(|c| a[r][0] * b[0][c] + a[r][1] * b[1][c]))
}

fn mat2x3_mul_3x3(a: &[[f64; 3]; 2], b: &[[f64; 3]; 3]) -> [[f64; 3]; 2] {
    std::array::from_fn(|r| std::array::from_fn(|c| (0..3).map(|k| a[r][k] * b[k][c]).sum()))
}
// #endregion 🔖️Mat3Helpers

// #region 🔖️PlanarCalibration
/// 📐️ Result of [`calibrate_planar`]: recovered intrinsics, one pose per input view, the final RMS
/// reprojection error in pixels, and the marginal covariance of the full parameter vector (intrinsics
/// followed by per-view poses) from the last accepted Levenberg-Marquardt iteration.
#[derive(Clone, Debug)]
pub struct CalibrationResult {
    pub intrinsics: Intrinsics,
    pub poses: Vec<CameraPose>,
    pub rms_px: f64,
    pub covariance: MatD,
}

fn normalize_2d(pts: &[[f64; 2]]) -> (Vec<[f64; 2]>, [[f64; 3]; 3]) {
    let n = pts.len() as f64;
    let mean = pts.iter().fold([0.0, 0.0], |acc, p| [acc[0] + p[0], acc[1] + p[1]]);
    let mean = [mean[0] / n, mean[1] / n];
    let avg_dist = pts.iter().map(|p| ((p[0] - mean[0]).powi(2) + (p[1] - mean[1]).powi(2)).sqrt()).sum::<f64>() / n;
    let scale = if avg_dist > 1e-12 { 2f64.sqrt() / avg_dist } else { 1.0 };
    let normalized = pts.iter().map(|p| [(p[0] - mean[0]) * scale, (p[1] - mean[1]) * scale]).collect();
    let t = [[scale, 0.0, -scale * mean[0]], [0.0, scale, -scale * mean[1]], [0.0, 0.0, 1.0]];
    (normalized, t)
}

fn invert_similarity(t: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let s = t[0][0];
    let mx = -t[0][2] / s;
    let my = -t[1][2] / s;
    [[1.0 / s, 0.0, mx], [0.0, 1.0 / s, my], [0.0, 0.0, 1.0]]
}

/// 📐️ Estimates a planar homography `image_px ~ H * [board_xy, 1]` via normalized DLT (Hartley
/// normalization on both point sets, nullspace of the 2n x 9 design matrix via [`svd_nullvector`]).
fn estimate_homography(board_xy: &[[f64; 2]], image_px: &[[f64; 2]]) -> Result<[[f64; 3]; 3], CameraError> {
    let n = board_xy.len();
    if n < 4 {
        return Err(CameraError::DegenerateHomography);
    }
    let (nb, tb) = normalize_2d(board_xy);
    let (ni, ti) = normalize_2d(image_px);
    let mut a = MatD::zeros(2 * n, 9);
    for (row, (b, im)) in nb.iter().zip(ni.iter()).enumerate() {
        let (x, y) = (b[0], b[1]);
        let (u, v) = (im[0], im[1]);
        a.set(2 * row, 0, -x);
        a.set(2 * row, 1, -y);
        a.set(2 * row, 2, -1.0);
        a.set(2 * row, 6, u * x);
        a.set(2 * row, 7, u * y);
        a.set(2 * row, 8, u);
        a.set(2 * row + 1, 3, -x);
        a.set(2 * row + 1, 4, -y);
        a.set(2 * row + 1, 5, -1.0);
        a.set(2 * row + 1, 6, v * x);
        a.set(2 * row + 1, 7, v * y);
        a.set(2 * row + 1, 8, v);
    }
    let h = svd_nullvector(&a).map_err(|_| CameraError::DegenerateHomography)?;
    let h_tilde = [[h.get(0), h.get(1), h.get(2)], [h.get(3), h.get(4), h.get(5)], [h.get(6), h.get(7), h.get(8)]];
    let ti_inv = invert_similarity(&ti);
    Ok(mat3_mul(&mat3_mul(&ti_inv, &h_tilde), &tb))
}

fn v_pq(h: &[[f64; 3]; 3], p: usize, q: usize) -> [f64; 6] {
    let hp = |r: usize| h[r][p];
    let hq = |r: usize| h[r][q];
    [hp(0) * hq(0), hp(0) * hq(1) + hp(1) * hq(0), hp(1) * hq(1), hp(2) * hq(0) + hp(0) * hq(2), hp(2) * hq(1) + hp(1) * hq(2), hp(2) * hq(2)]
}

/// 📐️ Recovers `(fx, fy, cx, cy, skew)` from the symmetric image-of-the-absolute-conic vector
/// `b = [B11, B12, B22, B13, B23, B33]` via Zhang's closed-form.
fn recover_intrinsics(b: &VecD) -> Result<(f64, f64, f64, f64, f64), CameraError> {
    let mut vals: [f64; 6] = std::array::from_fn(|i| b.get(i));
    if vals[0] < 0.0 {
        for v in vals.iter_mut() {
            *v = -*v;
        }
    }
    let (b11, b12, b22, b13, b23, b33) = (vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]);
    let denom = b11 * b22 - b12 * b12;
    if denom.abs() < 1e-300 || b11.abs() < 1e-300 {
        return Err(CameraError::SingularSystem);
    }
    let cy = (b12 * b13 - b11 * b23) / denom;
    let lambda = b33 - (b13 * b13 + cy * (b12 * b13 - b11 * b23)) / b11;
    let fx_sq = lambda / b11;
    let fy_sq = lambda * b11 / denom;
    if fx_sq <= 0.0 || fy_sq <= 0.0 || !fx_sq.is_finite() || !fy_sq.is_finite() {
        return Err(CameraError::SingularSystem);
    }
    let fx = fx_sq.sqrt();
    let fy = fy_sq.sqrt();
    let skew = -b12 * fx * fx * fy / lambda;
    let cx = skew * cy / fy - b13 * fx * fx / lambda;
    if !(fx.is_finite() && fy.is_finite() && cx.is_finite() && cy.is_finite() && skew.is_finite()) {
        return Err(CameraError::SingularSystem);
    }
    Ok((fx, fy, cx, cy, skew))
}

/// 📐️ Recovers a view's extrinsic pose from its homography and the shared intrinsics: `r1 = K⁻¹h1/‖·‖`,
/// `r2 = K⁻¹h2/‖·‖`, `r3 = r1 × r2`, `t = K⁻¹h3/‖·‖`, orthonormalized via [`So3::project_to_so3`].
fn recover_pose(h: &[[f64; 3]; 3], fx: f64, fy: f64, cx: f64, cy: f64, skew: f64) -> Result<CameraPose, CameraError> {
    let k_inv = [[1.0 / fx, -skew / (fx * fy), (skew * cy - cx * fy) / (fx * fy)], [0.0, 1.0 / fy, -cy / fy], [0.0, 0.0, 1.0]];
    let h1 = [h[0][0], h[1][0], h[2][0]];
    let h2 = [h[0][1], h[1][1], h[2][1]];
    let h3 = [h[0][2], h[1][2], h[2][2]];
    let r1_raw = mat3_vec(&k_inv, h1);
    let r2_raw = mat3_vec(&k_inv, h2);
    let t_raw = mat3_vec(&k_inv, h3);
    let n1 = vec3d_length(r1_raw);
    let n2 = vec3d_length(r2_raw);
    if n1 < 1e-12 || n2 < 1e-12 {
        return Err(CameraError::DegenerateHomography);
    }
    let mut lambda_scale = 2.0 / (n1 + n2);
    if lambda_scale * t_raw[2] < 0.0 {
        lambda_scale = -lambda_scale;
    }
    let r1 = scale3(r1_raw, lambda_scale);
    let r2 = scale3(r2_raw, lambda_scale);
    let r3 = vec3d_cross(r1, r2);
    let t = scale3(t_raw, lambda_scale);
    let rot = Mat3d::from_axes(r1, r2, r3);
    Ok(CameraPose(Se3 { r: So3::project_to_so3(&rot), t }))
}

fn intrinsics_from_params(x: &VecD) -> Intrinsics {
    Intrinsics {
        fx: x.get(0),
        fy: x.get(1),
        cx: x.get(2),
        cy: x.get(3),
        skew: x.get(4),
        distortion: Distortion::BrownConrady { k1: x.get(5), k2: x.get(6), k3: 0.0, p1: x.get(7), p2: x.get(8) },
    }
}

/// 🧩️ Joint planar-calibration least-squares problem: one shared `Intrinsics` (9 params: fx, fy, cx, cy,
/// skew, k1, k2, p1, p2) plus one SE(3) log-tangent pose per view. A local wrapper distinct from
/// [`ReprojectionProblem`] because planar calibration has a single shared intrinsics block and known
/// (not optimized) board points, rather than per-observation shared points across many cameras.
struct PlanarCalibrationProblem<'a> {
    views: &'a [Vec<([f64; 2], [f64; 2])>],
}

impl LeastSquaresProblem for PlanarCalibrationProblem<'_> {
    fn residual_count(&self) -> usize {
        self.views.iter().map(|v| v.len() * 2).sum()
    }

    fn parameter_count(&self) -> usize {
        9 + 6 * self.views.len()
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let intr = intrinsics_from_params(x);
        let mut row = 0usize;
        for (i, view) in self.views.iter().enumerate() {
            let pose = CameraPose(se3_at(x, 9 + i * 6));
            for &(board_xy, obs) in view {
                let point_world = [board_xy[0], board_xy[1], 0.0];
                let pred = reproject(&intr, &pose, point_world).unwrap_or([obs[0] + 1.0e3, obs[1] + 1.0e3]);
                out.set(row, pred[0] - obs[0]);
                out.set(row + 1, pred[1] - obs[1]);
                row += 2;
            }
        }
    }

    /// 🔬️ Analytic Jacobians via [`reprojection_jacobians`]: the pose block scatters into this view's 6
    /// pose columns, and the intrinsics block (ordered `[fx,fy,cx,cy,skew,k1,k2,k3,p1,p2]`) drops its `k3`
    /// column (index 7, fixed at zero and not part of `x`) when scattering into this problem's 9 shared
    /// intrinsics columns `[fx,fy,cx,cy,skew,k1,k2,p1,p2]`.
    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        const INTRINSICS_SRC_COLS: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 8, 9];
        let intr = intrinsics_from_params(x);
        let mut row = 0usize;
        for (i, view) in self.views.iter().enumerate() {
            let pose = CameraPose(se3_at(x, 9 + i * 6));
            for &(board_xy, obs) in view {
                let point_world = [board_xy[0], board_xy[1], 0.0];
                let (j_pose, _j_point, j_intr) = reprojection_jacobians(&intr, &pose, point_world, obs);
                for r in 0..2 {
                    for (dest, &src) in INTRINSICS_SRC_COLS.iter().enumerate() {
                        out.set(row + r, dest, j_intr.get(r, src));
                    }
                    for c in 0..6 {
                        out.set(row + r, 9 + i * 6 + c, j_pose.get(r, c));
                    }
                }
                row += 2;
            }
        }
    }

    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        let mut out = VecD::zeros(x.len());
        for k in 0..9 {
            out.set(k, x.get(k) + dx.get(k));
        }
        for i in 0..self.views.len() {
            let updated = retract_se3_block(x, dx, 9 + i * 6);
            for (k, &v) in updated.iter().enumerate() {
                out.set(9 + i * 6 + k, v);
            }
        }
        out
    }
}

/// 📐️ Zhang's method planar calibration: per-view homographies via normalized DLT, closed-form intrinsics
/// from the image-of-the-absolute-conic linear system, per-view extrinsics from each homography, then a
/// full nonlinear joint refinement (shared intrinsics + BrownConrady distortion + all view poses) via
/// [`levenberg_marquardt`]. Requires at least 3 views; returns [`CameraError`] for degenerate inputs.
pub fn calibrate_planar(views: &[Vec<([f64; 2], [f64; 2])>], image_width: u32, image_height: u32) -> Result<CalibrationResult, CameraError> {
    if views.len() < 3 {
        return Err(CameraError::TooFewViews);
    }
    let homographies: Vec<[[f64; 3]; 3]> = views
        .iter()
        .map(|v| {
            let board: Vec<[f64; 2]> = v.iter().map(|&(b, _)| b).collect();
            let img: Vec<[f64; 2]> = v.iter().map(|&(_, p)| p).collect();
            estimate_homography(&board, &img)
        })
        .collect::<Result<_, _>>()?;

    let mut v_mat = MatD::zeros(2 * homographies.len(), 6);
    for (i, h) in homographies.iter().enumerate() {
        let v12 = v_pq(h, 0, 1);
        let v11 = v_pq(h, 0, 0);
        let v22 = v_pq(h, 1, 1);
        for (k, ((&v12k, &v11k), &v22k)) in v12.iter().zip(v11.iter()).zip(v22.iter()).enumerate() {
            v_mat.set(2 * i, k, v12k);
            v_mat.set(2 * i + 1, k, v11k - v22k);
        }
    }
    let b = svd_nullvector(&v_mat).map_err(|_| CameraError::SingularSystem)?;
    let (fx, fy, cx, cy, skew) = recover_intrinsics(&b)?;
    let (w, h) = (image_width as f64, image_height as f64);
    if cx < -w || cx > 2.0 * w || cy < -h || cy > 2.0 * h {
        return Err(CameraError::SingularSystem);
    }

    let poses: Vec<CameraPose> = homographies.iter().map(|hg| recover_pose(hg, fx, fy, cx, cy, skew)).collect::<Result<_, _>>()?;

    let mut x0 = VecD::zeros(9 + 6 * views.len());
    x0.set(0, fx);
    x0.set(1, fy);
    x0.set(2, cx);
    x0.set(3, cy);
    x0.set(4, skew);
    for (i, pose) in poses.iter().enumerate() {
        let log = pose.0.log();
        for (k, v) in log.into_iter().enumerate() {
            x0.set(9 + i * 6 + k, v);
        }
    }

    let problem = PlanarCalibrationProblem { views };
    let cfg = LmConfig { max_iters: 200, ..LmConfig::default() };
    let result = levenberg_marquardt(&problem, x0, &cfg);
    if !result.converged {
        return Err(CameraError::Convergence);
    }

    let final_intr = intrinsics_from_params(&result.x);
    let final_poses: Vec<CameraPose> = (0..views.len()).map(|i| CameraPose(se3_at(&result.x, 9 + i * 6))).collect();
    let residual_count = problem.residual_count();
    let rms_px = (4.0 * result.cost / residual_count as f64).sqrt();
    let covariance = pseudo_inverse(&result.jtj, 1e-9).map_err(|_| CameraError::SingularSystem)?;
    Ok(CalibrationResult { intrinsics: final_intr, poses: final_poses, rms_px, covariance })
}
// #endregion 🔖️PlanarCalibration

// #region 🔖️SelfCalibration
fn nullvector_3x3(m: &[[f64; 3]; 3]) -> Option<[f64; 3]> {
    let mut a = MatD::zeros(3, 3);
    for (r, row) in m.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            a.set(r, c, v);
        }
    }
    let n = svd_nullvector(&a).ok()?;
    Some([n.get(0), n.get(1), n.get(2)])
}

/// 📍️ `(e1, e2)` epipoles of a fundamental matrix: `e1` satisfies `F·e1 = 0` (epipole in image 1), `e2`
/// satisfies `Fᵀ·e2 = 0` (epipole in image 2), both as raw (not `z`-normalized) SVD nullvectors.
fn epipoles_from_fundamental_matrix(f: &[[f64; 3]; 3]) -> Option<([f64; 3], [f64; 3])> {
    let e1 = nullvector_3x3(f)?;
    let e2 = nullvector_3x3(&transpose3(f))?;
    Some((e1, e2))
}

/// 🔄️ 2D rotation (embedded in the top-left 3x3 block) that eliminates the `y`-component of `v`'s
/// direction: the [`bougnoux_focal_from_fundamental`] epipole-alignment step.
fn rotation_eliminate_y(v: [f64; 3]) -> [[f64; 3]; 3] {
    let r = (v[0] * v[0] + v[1] * v[1]).sqrt();
    if r < 1e-300 {
        return [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    }
    let (c, s) = (v[0] / r, -v[1] / r);
    [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
}

/// 📐️ Fundamental matrix corresponding to both principal points shifted to the coordinate origin:
/// `F_shifted = T2⁻ᵀ · F · T1⁻¹` for the pure-translation `T1, T2` moving `pp1, pp2` to `(0,0)`.
fn shift_principal_points_to_origin(f: &[[f64; 3]; 3], pp1: [f64; 2], pp2: [f64; 2]) -> [[f64; 3]; 3] {
    let t1_inv = [[1.0, 0.0, pp1[0]], [0.0, 1.0, pp1[1]], [0.0, 0.0, 1.0]];
    let t2_inv_transpose = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [pp2[0], pp2[1], 1.0]];
    mat3_mul(&mat3_mul(&t2_inv_transpose, f), &t1_inv)
}

/// 🔭️ Bougnoux/Hartley closed-form recovery of two cameras' distinct focal lengths `(f1, f2)` from their
/// shared fundamental matrix and known principal points: shift both principal points to the origin, rotate
/// each image so its epipole's `y`-component vanishes, and read the closed-form focal pair off the
/// resulting 2x2 block (Hartley, "Extraction of focal lengths from the fundamental matrix",
/// axiom.anu.edu.au/~hartley/Papers/focal-lengths/focal.pdf — the same reduction implemented in libmv's
/// `FocalFromFundamental`). Scale-invariant in `f_matrix` (numerator and denominator are both linear in
/// it, so any nonzero multiple gives the same answer). `None` for degenerate epipolar geometry (near-zero
/// epipole components, singular denominators) or a non-positive focal-length estimate.
pub fn bougnoux_focal_from_fundamental(f_matrix: &MatD, pp1: [f64; 2], pp2: [f64; 2]) -> Option<(f64, f64)> {
    if f_matrix.rows != 3 || f_matrix.cols != 3 {
        return None;
    }
    let f: [[f64; 3]; 3] = std::array::from_fn(|r| std::array::from_fn(|c| f_matrix.get(r, c)));
    let f_shifted = shift_principal_points_to_origin(&f, pp1, pp2);
    let (e1, e2) = epipoles_from_fundamental_matrix(&f_shifted)?;
    let t1 = rotation_eliminate_y(e1);
    let t2 = rotation_eliminate_y(e2);
    let f_rotated = mat3_mul(&mat3_mul(&t2, &f_shifted), &transpose3(&t1));
    let (e1r, e2r) = epipoles_from_fundamental_matrix(&f_rotated)?;
    if e1r[0].abs() < 1e-9 || e1r[2].abs() < 1e-9 || e2r[0].abs() < 1e-9 || e2r[2].abs() < 1e-9 {
        return None;
    }
    let t1b0 = 1.0 / e1r[2];
    let t2b0 = 1.0 / e2r[2];
    let a = t2b0 * f_rotated[0][0] * t1b0;
    let b = t2b0 * f_rotated[0][1];
    let c = f_rotated[1][0] * t1b0;
    let d = f_rotated[1][1];
    let f1_denom = a * c * e1r[2] * e1r[2] + b * d;
    let f2_denom = a * b * e2r[2] * e2r[2] + c * d;
    if f1_denom.abs() < 1e-300 || f2_denom.abs() < 1e-300 {
        return None;
    }
    let f1_sq = -(a * c * e1r[0] * e1r[0]) / f1_denom;
    let f2_sq = -(a * b * e2r[0] * e2r[0]) / f2_denom;
    if !(f1_sq.is_finite() && f2_sq.is_finite()) || f1_sq <= 0.0 || f2_sq <= 0.0 {
        return None;
    }
    Some((f1_sq.sqrt(), f2_sq.sqrt()))
}

/// 📊️ Componentwise median aggregation of noisy per-pair `(f1, f2)` candidates (e.g. many
/// [`bougnoux_focal_from_fundamental`] estimates across image pairs of the same two cameras): each
/// closed-form estimate is sensitive to noise in `F` and to near-degenerate epipolar geometry, so the
/// median across many pairs is far more stable than trusting any single one. `(0.0, 0.0)` when empty.
pub fn aggregate_self_calibration(candidates: &[(f64, f64)]) -> (f64, f64) {
    fn median_of(values: &mut [f64]) -> f64 {
        values.sort_by(f64::total_cmp);
        let n = values.len();
        if n == 0 {
            return 0.0;
        }
        if n % 2 == 1 {
            values[n / 2]
        } else {
            0.5 * (values[n / 2 - 1] + values[n / 2])
        }
    }
    let mut f1s: Vec<f64> = candidates.iter().map(|&(f1, _)| f1).collect();
    let mut f2s: Vec<f64> = candidates.iter().map(|&(_, f2)| f2).collect();
    (median_of(&mut f1s), median_of(&mut f2s))
}
// #endregion 🔖️SelfCalibration

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn lcg(state: &mut u64) -> f64 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((*state >> 11) as f64 / (1_u64 << 53) as f64) * 2.0 - 1.0
    }

    fn gaussian(state: &mut u64, sigma: f64) -> f64 {
        let u1 = (0.5 * (lcg(state) + 1.0)).max(1e-12);
        let u2 = 0.5 * (lcg(state) + 1.0);
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos() * sigma
    }

    // #region 🔖️DistortionTests
    #[test]
    fn project_unproject_round_trips_for_none_distortion() {
        let intr = Intrinsics { fx: 600.0, fy: 610.0, cx: 320.0, cy: 240.0, skew: 0.5, distortion: Distortion::None };
        for ix in -3..=3 {
            for iy in -3..=3 {
                let p_cam = [ix as f64 * 0.1, iy as f64 * 0.1, 1.0];
                let px = intr.project(p_cam).expect("in front of camera");
                let ray = intr.unproject_ray(px);
                assert!((ray[0] - p_cam[0]).abs() < 1e-9, "x mismatch at {ix},{iy}: {} vs {}", ray[0], p_cam[0]);
                assert!((ray[1] - p_cam[1]).abs() < 1e-9, "y mismatch at {ix},{iy}: {} vs {}", ray[1], p_cam[1]);
            }
        }
    }

    #[test]
    fn undistort_distort_round_trips_for_brown_conrady_and_fisheye() {
        let models = [
            Distortion::BrownConrady { k1: -0.15, k2: 0.03, k3: -0.002, p1: 0.001, p2: -0.0015 },
            Distortion::FisheyeEquidistant { k1: -0.05, k2: 0.01, k3: -0.002, k4: 0.0005 },
        ];
        for distortion in models {
            let intr = Intrinsics { fx: 700.0, fy: 690.0, cx: 330.0, cy: 250.0, skew: 0.0, distortion };
            for ix in -4..=4 {
                for iy in -4..=4 {
                    let p_norm = [ix as f64 * 0.06, iy as f64 * 0.06];
                    let distorted = distortion.distort(p_norm);
                    let recovered = intr.undistort_point(distorted);
                    assert!((recovered[0] - p_norm[0]).abs() < 1e-6, "x mismatch at {ix},{iy}: {} vs {}", recovered[0], p_norm[0]);
                    assert!((recovered[1] - p_norm[1]).abs() < 1e-6, "y mismatch at {ix},{iy}: {} vs {}", recovered[1], p_norm[1]);
                }
            }
        }
    }

    #[test]
    fn project_unproject_round_trips_through_moderate_distortion() {
        let intr = Intrinsics { fx: 650.0, fy: 655.0, cx: 315.0, cy: 245.0, skew: 0.0, distortion: Distortion::BrownConrady { k1: -0.1, k2: 0.02, k3: 0.0, p1: 0.0005, p2: -0.0004 } };
        for ix in -3..=3 {
            for iy in -3..=3 {
                let p_cam = [ix as f64 * 0.08, iy as f64 * 0.08, 1.0];
                let px = intr.project(p_cam).expect("in front of camera");
                let ray = intr.unproject_ray(px);
                assert!((ray[0] - p_cam[0]).abs() < 1e-6, "x mismatch at {ix},{iy}");
                assert!((ray[1] - p_cam[1]).abs() < 1e-6, "y mismatch at {ix},{iy}");
            }
        }
    }
    // #endregion 🔖️DistortionTests

    // #region 🔖️ReprojectionTests
    #[test]
    fn reprojection_problem_residuals_vanish_at_ground_truth_and_lm_recovers_from_perturbation() {
        let intr = Intrinsics { fx: 500.0, fy: 500.0, cx: 250.0, cy: 200.0, skew: 0.0, distortion: Distortion::None };
        let num_cameras = 3;
        let num_points = 8;
        let mut state = 42_u64;
        let true_poses: Vec<Se3> = (0..num_cameras)
            .map(|i| Se3 { r: So3::exp([0.1 * i as f64, -0.05 * i as f64, 0.05 * i as f64]), t: [0.2 * i as f64, -0.1 * i as f64, 0.0] })
            .collect();
        let true_points: Vec<[f64; 3]> = (0..num_points).map(|_| [lcg(&mut state) * 0.5, lcg(&mut state) * 0.5, 2.0 + lcg(&mut state) * 0.3]).collect();

        let mut observations = Vec::new();
        for (cam_idx, pose) in true_poses.iter().enumerate() {
            for (point_idx, point) in true_points.iter().enumerate() {
                if let Some(px) = reproject(&intr, &CameraPose(*pose), *point) {
                    observations.push((cam_idx, point_idx, px));
                }
            }
        }
        let problem = ReprojectionProblem { observations, num_cameras, num_points, intrinsics: intr };

        let mut x_truth = VecD::zeros(problem.parameter_count());
        for (i, pose) in true_poses.iter().enumerate() {
            let log = pose.log();
            for (k, v) in log.into_iter().enumerate() {
                x_truth.set(i * 6 + k, v);
            }
        }
        for (j, point) in true_points.iter().enumerate() {
            let base = num_cameras * 6 + j * 3;
            x_truth.set(base, point[0]);
            x_truth.set(base + 1, point[1]);
            x_truth.set(base + 2, point[2]);
        }
        let mut residuals_at_truth = VecD::zeros(problem.residual_count());
        problem.residuals(&x_truth, &mut residuals_at_truth);
        assert!(residuals_at_truth.norm_inf() < 1e-9, "residuals at ground truth: {}", residuals_at_truth.norm_inf());

        let mut x0 = VecD::zeros(problem.parameter_count());
        for i in 0..x0.len() {
            x0.set(i, x_truth.get(i) + 0.02 * lcg(&mut state));
        }
        let cfg = LmConfig { max_iters: 100, ..LmConfig::default() };
        let result = levenberg_marquardt(&problem, x0, &cfg);
        let mut final_residuals = VecD::zeros(problem.residual_count());
        problem.residuals(&result.x, &mut final_residuals);
        let rmse = (final_residuals.dot(&final_residuals) / final_residuals.len() as f64).sqrt();
        assert!(rmse < 1e-4, "reprojection RMSE after LM: {rmse}");
    }
    // #endregion 🔖️ReprojectionTests

    // #region 🔖️PlanarCalibrationTests
    #[test]
    fn calibrate_planar_recovers_intrinsics_from_synthetic_checkerboard() {
        let true_intr = Intrinsics { fx: 800.0, fy: 810.0, cx: 322.0, cy: 238.0, skew: 0.0, distortion: Distortion::BrownConrady { k1: -0.12, k2: 0.02, k3: 0.0, p1: 0.0005, p2: -0.0003 } };
        let (nx, ny, square) = (7, 5, 0.03);
        let board_points: Vec<[f64; 2]> = (0..ny).flat_map(|iy| (0..nx).map(move |ix| [(ix as f64 - (nx - 1) as f64 / 2.0) * square, (iy as f64 - (ny - 1) as f64 / 2.0) * square])).collect();

        let mut state = 20260719_u64;
        let num_views = 15;
        let mut views: Vec<Vec<([f64; 2], [f64; 2])>> = Vec::with_capacity(num_views);
        for i in 0..num_views {
            let j1 = 0.2 * lcg(&mut state);
            let j2 = 0.2 * lcg(&mut state);
            let j3 = 0.15 * lcg(&mut state);
            let rot = match i % 3 {
                0 => [0.55 + j1, j2, j3],
                1 => [j1, 0.55 + j2, j3],
                _ => [0.4 + j1, 0.4 + j2, j3],
            };
            let t = [0.25 * lcg(&mut state), 0.25 * lcg(&mut state), 0.7 + 0.15 * lcg(&mut state)];
            let pose = Se3 { r: So3::exp(rot), t };
            let mut view = Vec::with_capacity(board_points.len());
            for &board_xy in &board_points {
                let point_world = [board_xy[0], board_xy[1], 0.0];
                let px = reproject(&true_intr, &CameraPose(pose), point_world).expect("board point in front of camera");
                let noisy = [px[0] + gaussian(&mut state, 0.2), px[1] + gaussian(&mut state, 0.2)];
                view.push((board_xy, noisy));
            }
            views.push(view);
        }

        let result = calibrate_planar(&views, 640, 480).expect("well-posed synthetic calibration");
        let fx_err = (result.intrinsics.fx - true_intr.fx).abs() / true_intr.fx;
        let fy_err = (result.intrinsics.fy - true_intr.fy).abs() / true_intr.fy;
        assert!(fx_err < 0.005, "fx relative error {fx_err}, recovered {}", result.intrinsics.fx);
        assert!(fy_err < 0.005, "fy relative error {fy_err}, recovered {}", result.intrinsics.fy);
        if let Distortion::BrownConrady { k1, .. } = result.intrinsics.distortion {
            assert!((k1 - (-0.12)).abs() < 0.03, "k1 = {k1}");
        } else {
            panic!("expected BrownConrady distortion");
        }
        assert!(result.rms_px < 1.0, "rms_px = {}", result.rms_px);
        assert_eq!(result.poses.len(), num_views);
    }

    #[test]
    fn calibrate_planar_rejects_too_few_views() {
        let view = vec![([0.0, 0.0], [100.0, 100.0]), ([0.1, 0.0], [150.0, 100.0]), ([0.0, 0.1], [100.0, 150.0]), ([0.1, 0.1], [150.0, 150.0])];
        let views = vec![view.clone(), view];
        assert!(matches!(calibrate_planar(&views, 640, 480), Err(CameraError::TooFewViews)));
    }
    // #endregion 🔖️PlanarCalibrationTests

    // #region 🔖️RollingShutterTests
    #[test]
    fn pose_at_row_matches_pose0_at_first_row_and_grows_monotonically() {
        let model = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::TopToBottom };
        let pose0 = CameraPose(Se3::exp([0.1, -0.05, 0.2, 0.05, 0.02, -0.03]));
        let velocity = [0.5, 0.2, -0.1, 0.05, -0.02, 0.03];
        let at_first = pose_at_row(&model, 0, 480, &pose0, velocity);
        assert!((at_first.0.log().iter().zip(pose0.0.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max)) < 1e-9);

        let mut last_translation_norm = 0.0;
        for row in [0_u32, 100, 250, 400, 479] {
            let pose = pose_at_row(&model, row, 480, &pose0, velocity);
            let delta = pose.0.compose(&pose0.0.inverse());
            let norm = vec3d_length(delta.t);
            assert!(norm >= last_translation_norm - 1e-12, "translation magnitude should grow monotonically with row");
            last_translation_norm = norm;
        }
    }

    #[test]
    fn pose_at_row_bottom_to_top_reverses_direction() {
        let model_top = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::TopToBottom };
        let model_bottom = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::BottomToTop };
        let pose0 = CameraPose(Se3::identity());
        let velocity = [0.3, 0.1, 0.0, 0.02, 0.0, 0.0];
        let top_last = pose_at_row(&model_top, 479, 480, &pose0, velocity);
        let bottom_first = pose_at_row(&model_bottom, 0, 480, &pose0, velocity);
        let diff = top_last.0.log().iter().zip(bottom_first.0.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max);
        assert!(diff < 1e-9, "top-to-bottom last row should match bottom-to-top first row");
    }

    #[test]
    fn rectify_remap_field_is_near_identity_at_zero_velocity() {
        let intr = Intrinsics { fx: 400.0, fy: 400.0, cx: 100.0, cy: 75.0, skew: 0.0, distortion: Distortion::None };
        let model = RollingShutterModel { line_delay_s: 1e-5, readout: ReadoutDirection::TopToBottom };
        let pose0 = CameraPose(Se3::exp([0.1, 0.0, 0.0, 0.0, 0.05, 0.0]));
        let (width, height) = (20, 15);
        let field = rectify_remap_field(&intr, &model, &pose0, [0.0; 6], width, height);
        assert_eq!(field.len(), (width * height) as usize);
        for row in 0..height {
            for col in 0..width {
                let [sx, sy] = field[(row * width + col) as usize];
                assert!((sx - (col as f32 + 0.5)).abs() < 1e-3, "col {col} row {row}: sx = {sx}");
                assert!((sy - (row as f32 + 0.5)).abs() < 1e-3, "col {col} row {row}: sy = {sy}");
            }
        }
    }
    // #endregion 🔖️RollingShutterTests

    // #region 🔖️RigTests
    #[test]
    fn rig_project_matches_manual_composition() {
        let intr = Intrinsics { fx: 400.0, fy: 400.0, cx: 200.0, cy: 150.0, skew: 0.0, distortion: Distortion::None };
        let camera_from_rig = Se3::exp([0.1, 0.0, 0.0, 0.0, 0.2, 0.0]);
        let rig = CameraRig { cameras: vec![RigExtrinsic { camera_id: "cam7".to_string(), pose_in_rig: camera_from_rig }] };
        let rig_pose = CameraPose(Se3::exp([0.0, 0.3, 0.0, 0.1, 0.0, 0.0]));
        let point_world = [0.3, -0.1, 3.0];
        let expected = reproject(&intr, &CameraPose(camera_from_rig.compose(&rig_pose.0)), point_world);
        let actual = rig_project(&rig, &[intr], "cam7", &rig_pose, point_world);
        assert_eq!(actual, expected);
        assert!(rig_project(&rig, &[intr], "unknown", &rig_pose, point_world).is_none());
    }

    #[test]
    fn refine_rig_recovers_planted_extrinsics_and_instance_poses_from_perturbation() {
        let intr = Intrinsics { fx: 500.0, fy: 505.0, cx: 250.0, cy: 200.0, skew: 0.0, distortion: Distortion::None };
        let intrinsics = [intr, intr, intr];
        let true_cameras = [
            RigExtrinsic { camera_id: "cam0".to_string(), pose_in_rig: Se3::identity() },
            RigExtrinsic { camera_id: "cam1".to_string(), pose_in_rig: Se3::exp([0.3, 0.0, 0.0, 0.0, 0.4, 0.0]) },
            RigExtrinsic { camera_id: "cam2".to_string(), pose_in_rig: Se3::exp([-0.3, 0.0, 0.0, 0.0, -0.4, 0.0]) },
        ];
        let mut state = 555_u64;
        let num_instances = 5;
        let true_rig_poses: Vec<Se3> = (0..num_instances).map(|i| Se3 { r: So3::exp([0.1 * i as f64, -0.05 * i as f64, 0.05 * i as f64]), t: [0.15 * i as f64, -0.1 * i as f64, 0.05 * i as f64] }).collect();
        let true_points: Vec<[f64; 3]> = (0..25).map(|_| [0.6 * lcg(&mut state), 0.6 * lcg(&mut state), 3.0 + 0.4 * lcg(&mut state)]).collect();

        let mut observations = Vec::new();
        for (instance_idx, rig_pose) in true_rig_poses.iter().enumerate() {
            for (camera_idx, cam) in true_cameras.iter().enumerate() {
                let world_to_camera = rig_pose_of_camera(rig_pose, &cam.pose_in_rig);
                for &point_world in &true_points {
                    if let Some(pixel) = reproject(&intrinsics[camera_idx], &CameraPose(world_to_camera), point_world) {
                        observations.push(RigObservation { instance_idx, camera_idx, point_world, pixel });
                    }
                }
            }
        }

        let initial_cameras = CameraRig {
            cameras: true_cameras
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        // camera 0 is the rig-frame anchor: refine_rig never touches it, so it must start
                        // at its true value (as it would in practice — the rig frame is *defined* by it).
                        return c.clone();
                    }
                    let mut log = c.pose_in_rig.log();
                    for v in log.iter_mut() {
                        *v += 0.01 * lcg(&mut state);
                    }
                    RigExtrinsic { camera_id: c.camera_id.clone(), pose_in_rig: Se3::exp(log) }
                })
                .collect(),
        };
        let initial_rig_poses: Vec<Se3> = true_rig_poses
            .iter()
            .map(|p| {
                let mut log = p.log();
                for v in log.iter_mut() {
                    *v += 0.01 * lcg(&mut state);
                }
                Se3::exp(log)
            })
            .collect();

        let result = refine_rig(&intrinsics, &initial_cameras, &initial_rig_poses, &observations).expect("well-posed synthetic rig refinement");
        assert!(result.rms_px < 1e-3, "rms_px = {}", result.rms_px);
        for (recovered, truth) in result.cameras.iter().zip(true_cameras.iter()) {
            let diff = recovered.pose_in_rig.log().iter().zip(truth.pose_in_rig.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max);
            assert!(diff < 1e-3, "camera {} pose_in_rig diff {diff}", recovered.camera_id);
        }
        for (recovered, truth) in result.rig_poses.iter().zip(true_rig_poses.iter()) {
            let diff = recovered.log().iter().zip(truth.log().iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max);
            assert!(diff < 1e-3, "rig pose diff {diff}");
        }
    }
    // #endregion 🔖️RigTests

    // #region 🔖️SelfCalibrationTests
    fn k_inverse(f: f64, px: f64, py: f64) -> [[f64; 3]; 3] {
        [[1.0 / f, 0.0, -px / f], [0.0, 1.0 / f, -py / f], [0.0, 0.0, 1.0]]
    }

    fn matd_from_3x3(m: &[[f64; 3]; 3]) -> MatD {
        let mut a = MatD::zeros(3, 3);
        for (r, row) in m.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                a.set(r, c, v);
            }
        }
        a
    }

    #[test]
    fn bougnoux_focal_from_fundamental_recovers_distinct_ground_truth_focals() {
        let f1_true = 850.0;
        let f2_true = 1200.0;
        let pp1 = [310.0, 245.0];
        let pp2 = [300.0, 260.0];
        let r = mat3d_rowmajor(&So3::exp([0.05, -0.12, 0.08]).0);
        let t = [1.0, 0.2, -0.3];
        let e = mat3_mul(&skew3(t), &r);
        let f = mat3_mul(&mat3_mul(&transpose3(&k_inverse(f2_true, pp2[0], pp2[1])), &e), &k_inverse(f1_true, pp1[0], pp1[1]));
        let f_matd = matd_from_3x3(&f);

        let (rf1, rf2) = bougnoux_focal_from_fundamental(&f_matd, pp1, pp2).expect("well-posed synthetic stereo pair");
        assert!((rf1 - f1_true).abs() / f1_true < 0.01, "f1 = {rf1}, expected {f1_true}");
        assert!((rf2 - f2_true).abs() / f2_true < 0.01, "f2 = {rf2}, expected {f2_true}");

        // scale invariance: an arbitrary nonzero rescale of F must give the same answer.
        let mut f_scaled = MatD::zeros(3, 3);
        for r_ in 0..3 {
            for c_ in 0..3 {
                f_scaled.set(r_, c_, f_matd.get(r_, c_) * 17.0);
            }
        }
        let (rf1_scaled, rf2_scaled) = bougnoux_focal_from_fundamental(&f_scaled, pp1, pp2).expect("scaled F still well-posed");
        assert!((rf1_scaled - rf1).abs() < 1e-6, "f1 should be scale-invariant: {rf1_scaled} vs {rf1}");
        assert!((rf2_scaled - rf2).abs() < 1e-6, "f2 should be scale-invariant: {rf2_scaled} vs {rf2}");
    }

    #[test]
    fn aggregate_self_calibration_recovers_median_and_rejects_outliers() {
        let candidates = [(800.0, 1000.0), (820.0, 990.0), (5000.0, 40.0), (810.0, 1010.0), (790.0, 1005.0)];
        let (f1, f2) = aggregate_self_calibration(&candidates);
        assert!((f1 - 810.0).abs() < 1e-9, "f1 median = {f1}");
        assert!((f2 - 1000.0).abs() < 1e-9, "f2 median = {f2}");
        assert_eq!(aggregate_self_calibration(&[]), (0.0, 0.0));
    }

    #[test]
    fn bougnoux_focal_from_fundamental_rejects_wrong_shaped_matrix() {
        let mut f_wrong = MatD::zeros(2, 3);
        f_wrong.set(0, 0, 1.0);
        assert!(bougnoux_focal_from_fundamental(&f_wrong, [0.0, 0.0], [0.0, 0.0]).is_none());
    }
    // #endregion 🔖️SelfCalibrationTests

    // #region 🔖️ReprojectionJacobianTests
    fn perturb_intrinsics(intr: &Intrinsics, k: usize, delta: f64) -> Intrinsics {
        let mut out = *intr;
        match k {
            0 => out.fx += delta,
            1 => out.fy += delta,
            2 => out.cx += delta,
            3 => out.cy += delta,
            4 => out.skew += delta,
            _ => {
                let di = k - 5;
                out.distortion = match out.distortion {
                    Distortion::None => Distortion::None,
                    Distortion::BrownConrady { mut k1, mut k2, mut k3, mut p1, mut p2 } => {
                        match di {
                            0 => k1 += delta,
                            1 => k2 += delta,
                            2 => k3 += delta,
                            3 => p1 += delta,
                            4 => p2 += delta,
                            _ => {}
                        }
                        Distortion::BrownConrady { k1, k2, k3, p1, p2 }
                    }
                    Distortion::FisheyeEquidistant { mut k1, mut k2, mut k3, mut k4 } => {
                        match di {
                            0 => k1 += delta,
                            1 => k2 += delta,
                            2 => k3 += delta,
                            3 => k4 += delta,
                            _ => {}
                        }
                        Distortion::FisheyeEquidistant { k1, k2, k3, k4 }
                    }
                };
            }
        }
        out
    }

    #[test]
    fn reprojection_jacobians_match_central_difference_at_random_configurations() {
        let mut state = 909_u64;
        let models = [
            Distortion::None,
            Distortion::BrownConrady { k1: -0.12, k2: 0.02, k3: 0.001, p1: 0.0008, p2: -0.0005 },
            Distortion::FisheyeEquidistant { k1: -0.04, k2: 0.008, k3: -0.001, k4: 0.0002 },
        ];
        let eps = 1e-6;
        for distortion in models {
            for _ in 0..5 {
                let intr = Intrinsics { fx: 600.0 + 20.0 * lcg(&mut state), fy: 605.0 + 20.0 * lcg(&mut state), cx: 320.0 + 5.0 * lcg(&mut state), cy: 240.0 + 5.0 * lcg(&mut state), skew: 0.3 * lcg(&mut state), distortion };
                let pose = CameraPose(Se3::exp([0.3 * lcg(&mut state), 0.2 * lcg(&mut state), 0.1 * lcg(&mut state), 0.4 * lcg(&mut state), -0.3 * lcg(&mut state), 0.2 * lcg(&mut state)]));
                let point = [0.3 * lcg(&mut state), 0.2 * lcg(&mut state), 2.0 + 0.3 * lcg(&mut state)];
                let obs = reproject(&intr, &pose, point).expect("synthetic point stays in front of camera");

                let (j_pose, j_point, j_intr) = reprojection_jacobians(&intr, &pose, point, obs);

                for k in 0..6 {
                    let mut dxi = [0.0; 6];
                    dxi[k] = eps;
                    let pose_plus = CameraPose(Se3::exp(dxi).compose(&pose.0));
                    dxi[k] = -eps;
                    let pose_minus = CameraPose(Se3::exp(dxi).compose(&pose.0));
                    let r_plus = reprojection_residual(&intr, &pose_plus, point, obs);
                    let r_minus = reprojection_residual(&intr, &pose_minus, point, obs);
                    for row in 0..2 {
                        let numeric = (r_plus[row] - r_minus[row]) / (2.0 * eps);
                        assert!((numeric - j_pose.get(row, k)).abs() < 1e-4, "pose col {k} row {row}: numeric {numeric} vs analytic {}", j_pose.get(row, k));
                    }
                }

                for k in 0..3 {
                    let mut p_plus = point;
                    p_plus[k] += eps;
                    let mut p_minus = point;
                    p_minus[k] -= eps;
                    let r_plus = reprojection_residual(&intr, &pose, p_plus, obs);
                    let r_minus = reprojection_residual(&intr, &pose, p_minus, obs);
                    for row in 0..2 {
                        let numeric = (r_plus[row] - r_minus[row]) / (2.0 * eps);
                        assert!((numeric - j_point.get(row, k)).abs() < 1e-4, "point col {k} row {row}: numeric {numeric} vs analytic {}", j_point.get(row, k));
                    }
                }

                let k_count = 5 + distortion.param_count();
                for k in 0..k_count {
                    let intr_plus = perturb_intrinsics(&intr, k, eps);
                    let intr_minus = perturb_intrinsics(&intr, k, -eps);
                    let r_plus = reprojection_residual(&intr_plus, &pose, point, obs);
                    let r_minus = reprojection_residual(&intr_minus, &pose, point, obs);
                    for row in 0..2 {
                        let numeric = (r_plus[row] - r_minus[row]) / (2.0 * eps);
                        assert!((numeric - j_intr.get(row, k)).abs() < 1e-4, "intrinsics col {k} row {row}: numeric {numeric} vs analytic {}", j_intr.get(row, k));
                    }
                }
            }
        }
    }
    // #endregion 🔖️ReprojectionJacobianTests
}
// #endregion 🔖️Tests
