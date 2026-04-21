//! Geometric helpers for design flatten (mirrors `semio/py/main.py` plane/connector math).

use nalgebra::{Matrix4, Vector3};

use crate::geom::{Coord, Plane};

pub(crate) const FLATTEN_TOLERANCE: f64 = 1e-5;

fn v3(c: Coord) -> Vector3<f64> {
    Vector3::new(c.x, c.y, c.z)
}

#[allow(dead_code)]
fn coord(v: Vector3<f64>) -> Coord {
    Coord::new(v.x, v.y, v.z)
}

fn normalize(v: Vector3<f64>) -> Vector3<f64> {
    let n = v.norm();
    if n < 1e-10 {
        Vector3::new(0.0, 0.0, 1.0)
    } else {
        v / n
    }
}

pub(crate) fn plane_to_matrix(p: &Plane) -> Matrix4<f64> {
    let o = v3(p.origin);
    let x = normalize(v3(p.x_axis));
    let y = normalize(v3(p.y_axis));
    let z = normalize(x.cross(&y));
    let mut m = Matrix4::identity();
    m[(0, 0)] = x.x;
    m[(1, 0)] = x.y;
    m[(2, 0)] = x.z;
    m[(0, 1)] = y.x;
    m[(1, 1)] = y.y;
    m[(2, 1)] = y.z;
    m[(0, 2)] = z.x;
    m[(1, 2)] = z.y;
    m[(2, 2)] = z.z;
    m[(0, 3)] = o.x;
    m[(1, 3)] = o.y;
    m[(2, 3)] = o.z;
    m
}

pub(crate) fn matrix_to_plane(m: &Matrix4<f64>) -> Plane {
    let ox = m[(0, 3)];
    let oy = m[(1, 3)];
    let oz = m[(2, 3)];
    Plane {
        origin: Coord::new(ox, oy, oz),
        x_axis: Coord::new(m[(0, 0)], m[(1, 0)], m[(2, 0)]),
        y_axis: Coord::new(m[(0, 1)], m[(1, 1)], m[(2, 1)]),
    }
}

// Quaternion (qx,qy,qz,qw) from two unit vectors
fn quat_from_unit_vectors_full(v_from: Vector3<f64>, v_to: Vector3<f64>) -> (f64, f64, f64, f64) {
    let vf = normalize(v_from);
    let vt = normalize(v_to);
    let r = vf.dot(&vt) + 1.0;
    let (qx, qy, qz, qw) = if r < 0.000001 {
        if vf.x.abs() > vf.z.abs() {
            (-vf.y, vf.x, 0.0, 0.0)
        } else {
            (0.0, -vf.z, vf.y, 0.0)
        }
    } else {
        let c = vf.cross(&vt);
        (c.x, c.y, c.z, r)
    };
    let n = (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
    (qx / n, qy / n, qz / n, qw / n)
}

fn quat_from_axis_angle(axis: Vector3<f64>, angle: f64) -> (f64, f64, f64, f64) {
    let a = normalize(axis);
    let half = angle / 2.0;
    let s = half.sin();
    (a.x * s, a.y * s, a.z * s, half.cos())
}

fn quat_to_mat4(q: (f64, f64, f64, f64)) -> Matrix4<f64> {
    let (x, y, z, w) = q;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;
    let mut m = Matrix4::identity();
    m[(0, 0)] = 1.0 - (yy + zz);
    m[(0, 1)] = xy - wz;
    m[(0, 2)] = xz + wy;
    m[(1, 0)] = xy + wz;
    m[(1, 1)] = 1.0 - (xx + zz);
    m[(1, 2)] = yz - wx;
    m[(2, 0)] = xz - wy;
    m[(2, 1)] = yz + wx;
    m[(2, 2)] = 1.0 - (xx + yy);
    m
}

fn make_rotation_axis(axis: Vector3<f64>, angle: f64) -> Matrix4<f64> {
    quat_to_mat4(quat_from_axis_angle(axis, angle))
}

fn make_translation(x: f64, y: f64, z: f64) -> Matrix4<f64> {
    let mut m = Matrix4::identity();
    m[(0, 3)] = x;
    m[(1, 3)] = y;
    m[(2, 3)] = z;
    m
}

fn apply_mat3_upper(m: &Matrix4<f64>, v: Vector3<f64>) -> Vector3<f64> {
    Vector3::new(
        m[(0, 0)] * v.x + m[(0, 1)] * v.y + m[(0, 2)] * v.z,
        m[(1, 0)] * v.x + m[(1, 1)] * v.y + m[(1, 2)] * v.z,
        m[(2, 0)] * v.x + m[(2, 1)] * v.y + m[(2, 2)] * v.z,
    )
}

fn round_tol(x: f64) -> f64 {
    (x / FLATTEN_TOLERANCE).round() * FLATTEN_TOLERANCE
}

/// `parent_connector` / `child_connector`: local anchor point + direction in type space.
pub(crate) fn compute_child_plane(
    parent_plane: &Plane,
    parent_point: Coord,
    parent_direction: Coord,
    child_point: Coord,
    child_direction: Coord,
    gap: f64,
    shift: f64,
    rise: f64,
    rotation_deg: f64,
    turn_deg: f64,
    tilt_deg: f64,
) -> Plane {
    let parent_matrix = plane_to_matrix(parent_plane);
    let parent_pt = v3(parent_point);
    let parent_dir = normalize(v3(parent_direction));
    let child_pt = v3(child_point);
    let child_dir = normalize(v3(child_direction));

    let rotation_rad = rotation_deg.to_radians();
    let turn_rad = turn_deg.to_radians();
    let tilt_rad = tilt_deg.to_radians();

    let reverse_child_dir = -child_dir;
    let cross_vec = parent_dir.cross(&reverse_child_dir);
    let cross_len = cross_vec.norm();
    let align_quat = if cross_len < 0.01 {
        if parent_dir.z.abs() < FLATTEN_TOLERANCE {
            quat_from_axis_angle(Vector3::new(0.0, 0.0, 1.0), std::f64::consts::PI)
        } else {
            let axis = normalize(Vector3::new(0.0, 0.0, 1.0).cross(&parent_dir));
            quat_from_axis_angle(axis, std::f64::consts::PI)
        }
    } else {
        quat_from_unit_vectors_full(reverse_child_dir, parent_dir)
    };

    let direction_t = quat_to_mat4(align_quat);
    let y_axis = Vector3::new(0.0, 1.0, 0.0);
    let parent_connector_quat = quat_from_unit_vectors_full(y_axis, parent_dir);
    let parent_rotation_t = quat_to_mat4(parent_connector_quat);

    let gap_direction = apply_mat3_upper(&parent_rotation_t, Vector3::new(0.0, 1.0, 0.0));
    let shift_direction = apply_mat3_upper(&parent_rotation_t, Vector3::new(1.0, 0.0, 0.0));
    let raise_direction = apply_mat3_upper(&parent_rotation_t, Vector3::new(0.0, 0.0, 1.0));
    let mut turn_axis = apply_mat3_upper(&parent_rotation_t, Vector3::new(0.0, 0.0, 1.0));
    let mut tilt_axis = apply_mat3_upper(&parent_rotation_t, Vector3::new(1.0, 0.0, 0.0));

    let mut orientation_t = direction_t;
    let rotate_t = make_rotation_axis(parent_dir, -rotation_rad);
    orientation_t = rotate_t * orientation_t;
    turn_axis = apply_mat3_upper(&rotate_t, turn_axis);
    tilt_axis = apply_mat3_upper(&rotate_t, tilt_axis);
    let turn_t = make_rotation_axis(turn_axis, turn_rad);
    orientation_t = turn_t * orientation_t;
    let tilt_t = make_rotation_axis(tilt_axis, tilt_rad);
    orientation_t = tilt_t * orientation_t;

    let center_child_t = make_translation(-child_pt.x, -child_pt.y, -child_pt.z);
    let mut transform = orientation_t * center_child_t;

    let gap_transform = make_translation(
        gap_direction.x * gap,
        gap_direction.y * gap,
        gap_direction.z * gap,
    );
    let shift_transform = make_translation(
        shift_direction.x * shift,
        shift_direction.y * shift,
        shift_direction.z * shift,
    );
    let raise_transform = make_translation(
        raise_direction.x * rise,
        raise_direction.y * rise,
        raise_direction.z * rise,
    );
    let translation_t = raise_transform * shift_transform * gap_transform;
    transform = translation_t * transform;
    let move_to_parent = make_translation(parent_pt.x, parent_pt.y, parent_pt.z);
    transform = move_to_parent * transform;

    let final_matrix = parent_matrix * transform;
    let mut pl = matrix_to_plane(&final_matrix);
    pl.origin.x = round_tol(pl.origin.x);
    pl.origin.y = round_tol(pl.origin.y);
    pl.origin.z = round_tol(pl.origin.z);
    pl.x_axis.x = round_tol(pl.x_axis.x);
    pl.x_axis.y = round_tol(pl.x_axis.y);
    pl.x_axis.z = round_tol(pl.x_axis.z);
    pl.y_axis.x = round_tol(pl.y_axis.x);
    pl.y_axis.y = round_tol(pl.y_axis.y);
    pl.y_axis.z = round_tol(pl.y_axis.z);
    pl
}

pub(crate) const FLATTEN_RADIUS: f64 = 2.697;
pub(crate) const FLATTEN_VERTICAL_V_EXTRA: f64 = 1.0;
pub(crate) const FLATTEN_HORIZONTAL_SCALE: f64 = 3.0633;

/// UV center for child from parent center and connection u/v (matches Python BFS).
pub(crate) fn compute_child_center_uv(
    parent_center: Coord,
    connection_u: f64,
    connection_v: f64,
    parent_connector_dir_z: f64,
    parent_t: f64,
) -> Coord {
    let pu = parent_center.x;
    let pv = parent_center.y;
    if pu.abs() < FLATTEN_TOLERANCE && pv.abs() < FLATTEN_TOLERANCE {
        let angle = 2.0 * std::f64::consts::PI * parent_t;
        Coord::new(
            round_tol(FLATTEN_RADIUS * angle.sin()),
            round_tol(FLATTEN_RADIUS * angle.cos()),
            0.0,
        )
    } else {
        let is_vertical = parent_connector_dir_z.abs() > 0.5;
        let (cu, cv) = if is_vertical {
            (
                pu + connection_u,
                pv + connection_v + FLATTEN_VERTICAL_V_EXTRA,
            )
        } else {
            (
                pu + connection_u * FLATTEN_HORIZONTAL_SCALE,
                pv + connection_v * FLATTEN_HORIZONTAL_SCALE,
            )
        };
        Coord::new(round_tol(cu), round_tol(cv), 0.0)
    }
}
