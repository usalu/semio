//! 🌐 3D scene math, orbit camera, mesh instances, and screen picking.

//#region Math
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_array(v: [f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len < 1e-8 {
            return Self::ZERO;
        }
        self.scale(1.0 / len)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            cols: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        Self {
            cols: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (far + near) / (near - far), -1.0],
                [0.0, 0.0, (2.0 * far * near) / (near - far), 0.0],
            ],
        }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = target.sub(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        Self {
            cols: [
                [s.x, u.x, -f.x, 0.0],
                [s.y, u.y, -f.y, 0.0],
                [s.z, u.z, -f.z, 0.0],
                [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0],
            ],
        }
    }

    pub fn mul(self, other: Self) -> Self {
        let mut out = Self::identity();
        for col in 0..4 {
            for row in 0..4 {
                out.cols[col][row] = self.cols[0][row] * other.cols[col][0]
                    + self.cols[1][row] * other.cols[col][1]
                    + self.cols[2][row] * other.cols[col][2]
                    + self.cols[3][row] * other.cols[col][3];
            }
        }
        out
    }

    pub fn transform_point(self, p: Vec3) -> Vec3 {
        let x = p.x * self.cols[0][0] + p.y * self.cols[1][0] + p.z * self.cols[2][0] + self.cols[3][0];
        let y = p.x * self.cols[0][1] + p.y * self.cols[1][1] + p.z * self.cols[2][1] + self.cols[3][1];
        let z = p.x * self.cols[0][2] + p.y * self.cols[1][2] + p.z * self.cols[2][2] + self.cols[3][2];
        let w = p.x * self.cols[0][3] + p.y * self.cols[1][3] + p.z * self.cols[2][3] + self.cols[3][3];
        if w.abs() < 1e-8 {
            return Vec3::new(x, y, z);
        }
        Vec3::new(x / w, y / w, z / w)
    }

    pub fn transform_direction(self, dir: Vec3) -> Vec3 {
        let x = dir.x * self.cols[0][0] + dir.y * self.cols[1][0] + dir.z * self.cols[2][0];
        let y = dir.x * self.cols[0][1] + dir.y * self.cols[1][1] + dir.z * self.cols[2][1];
        let z = dir.x * self.cols[0][2] + dir.y * self.cols[1][2] + dir.z * self.cols[2][2];
        Vec3::new(x, y, z).normalize()
    }

    pub fn inverse(self) -> Self {
        let m = self.cols;
        let mut inv = [[0.0f32; 4]; 4];
        inv[0][0] = m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3]
            + m[2][1] * m[1][3] * m[3][2] + m[3][1] * m[1][2] * m[2][3] - m[3][1] * m[1][3] * m[2][2];
        let det = m[0][0] * inv[0][0] + m[0][1] * (m[1][0] * m[2][2] * m[3][3] - m[1][0] * m[2][3] * m[3][2] - m[2][0] * m[1][2] * m[3][3]
            + m[2][0] * m[1][3] * m[3][2] + m[3][0] * m[1][2] * m[2][3] - m[3][0] * m[1][3] * m[2][2])
            + m[0][2] * (m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3]
                + m[2][0] * m[1][3] * m[3][1] + m[3][0] * m[1][1] * m[2][3] - m[3][0] * m[1][3] * m[2][1])
            + m[0][3] * (m[1][0] * m[2][1] * m[3][2] - m[1][0] * m[2][2] * m[3][1] - m[2][0] * m[1][1] * m[3][2]
                + m[2][0] * m[1][2] * m[3][1] + m[3][0] * m[1][1] * m[2][2] - m[3][0] * m[1][2] * m[2][1]);
        if det.abs() < 1e-8 {
            return Self::identity();
        }
        let inv_det = 1.0 / det;
        for col in 0..4 {
            for row in 0..4 {
                inv[col][row] *= inv_det;
            }
        }
        Self { cols: inv }
    }

    pub fn translation(v: Vec3) -> Self {
        let mut m = Self::identity();
        m.cols[3] = [v.x, v.y, v.z, 1.0];
        m
    }

    pub fn scale_vec(v: Vec3) -> Self {
        Self {
            cols: [
                [v.x, 0.0, 0.0, 0.0],
                [0.0, v.y, 0.0, 0.0],
                [0.0, 0.0, v.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn from_quat(x: f32, y: f32, z: f32, w: f32) -> Self {
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        Self {
            cols: [
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0],
                [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0],
                [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn to_cols_array(self) -> [f32; 16] {
        let mut out = [0.0; 16];
        for col in 0..4 {
            for row in 0..4 {
                out[col * 4 + row] = self.cols[col][row];
            }
        }
        out
    }
}
//#endregion Math

//#region Camera
#[derive(Clone, Debug)]
pub struct Camera3d {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera3d {
    fn default() -> Self {
        Self {
            position: Vec3::new(4.0, -4.0, 3.0),
            target: Vec3::ZERO,
            up: Vec3::new(0.0, 0.0, 1.0),
            fov_y: 45.0_f32.to_radians(),
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl Camera3d {
    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        Mat4::perspective(self.fov_y, aspect, self.near, self.far)
            .mul(Mat4::look_at(self.position, self.target, self.up))
    }

    pub fn ray_from_screen(&self, aspect: f32, x: f32, y: f32, width: f32, height: f32) -> (Vec3, Vec3) {
        let ndc_x = (x / width) * 2.0 - 1.0;
        let ndc_y = 1.0 - (y / height) * 2.0;
        let view = Mat4::look_at(self.position, self.target, self.up);
        let proj = Mat4::perspective(self.fov_y, aspect, self.near, self.far);
        let inv = proj.mul(view).inverse();
        let near = inv.transform_point(Vec3::new(ndc_x, ndc_y, -1.0));
        let far = inv.transform_point(Vec3::new(ndc_x, ndc_y, 1.0));
        let dir = far.sub(near).normalize();
        (self.position, dir)
    }
}

#[derive(Clone, Debug)]
pub struct OrbitController {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
}

impl Default for OrbitController {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 8.0,
            yaw: 0.8,
            pitch: 0.5,
            fov_y: 45.0_f32.to_radians(),
        }
    }
}

impl OrbitController {
    pub fn from_camera(camera: &Camera3d) -> Self {
        let offset = camera.position.sub(camera.target);
        let distance = offset.length().max(0.5);
        Self {
            target: camera.target,
            distance,
            yaw: offset.y.atan2(offset.x),
            pitch: (offset.z / distance).asin(),
            fov_y: camera.fov_y,
        }
    }

    pub fn to_camera(&self) -> Camera3d {
        let cp = self.pitch.cos();
        let position = Vec3::new(
            self.target.x + self.distance * cp * self.yaw.cos(),
            self.target.y + self.distance * cp * self.yaw.sin(),
            self.target.z + self.distance * self.pitch.sin(),
        );
        Camera3d {
            position,
            target: self.target,
            up: Vec3::new(0.0, 0.0, 1.0),
            fov_y: self.fov_y,
            near: 0.1,
            far: 1000.0,
        }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw -= dx * 0.01;
        self.pitch = (self.pitch + dy * 0.01).clamp(-1.5, 1.5);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let camera = self.to_camera();
        let right = camera.position.sub(camera.target).cross(camera.up).normalize();
        let up = right.cross(camera.position.sub(camera.target)).normalize();
        let scale = self.distance * 0.001;
        self.target = self
            .target
            .add(right.scale(-dx * scale))
            .add(up.scale(dy * scale));
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance * (1.0 - delta * 0.001)).clamp(0.5, 500.0);
    }
}
//#endregion Camera

//#region Mesh
#[derive(Clone, Debug)]
pub struct Mesh3d {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub aabb_min: [f32; 3],
    pub aabb_max: [f32; 3],
}

impl Mesh3d {
    pub fn from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> Self {
        let mut aabb_min = [f32::INFINITY; 3];
        let mut aabb_max = [f32::NEG_INFINITY; 3];
        for chunk in positions.chunks_exact(3) {
            for axis in 0..3 {
                aabb_min[axis] = aabb_min[axis].min(chunk[axis]);
                aabb_max[axis] = aabb_max[axis].max(chunk[axis]);
            }
        }
        Self {
            positions,
            normals,
            indices,
            aabb_min,
            aabb_max,
        }
    }
}

impl From<(&[f32], &[f32], &[u32])> for Mesh3d {
    fn from((positions, normals, indices): (&[f32], &[f32], &[u32])) -> Self {
        Self::from_buffers(positions.to_vec(), normals.to_vec(), indices.to_vec())
    }
}

#[derive(Clone, Debug)]
pub struct Instance3d {
    pub id: String,
    pub model: Mat4,
    pub color: [f32; 4],
    pub selected: bool,
    pub hovered: bool,
}

impl Instance3d {
    pub fn model_from_trs(position: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Mat4 {
        Mat4::translation(Vec3::from_array(position))
            .mul(Mat4::from_quat(rotation[0], rotation[1], rotation[2], rotation[3]))
            .mul(Mat4::scale_vec(Vec3::from_array(scale)))
    }
}
//#endregion Mesh

//#region ScenePass
#[derive(Clone, Debug)]
pub struct SceneDraw3d {
    pub mesh_key: String,
    pub mesh_version: u64,
    pub instances: Vec<Instance3d>,
}

#[derive(Clone, Debug)]
pub struct ScenePass3d {
    pub viewport: [f32; 4],
    pub view_proj: [f32; 16],
    pub light_dir: [f32; 3],
    pub draws: Vec<SceneDraw3d>,
}
//#endregion ScenePass

//#region Culling
#[derive(Clone, Copy, Debug)]
pub struct FrustumPlane {
    pub normal: Vec3,
    pub distance: f32,
}

pub fn frustum_planes(view_proj: Mat4) -> [FrustumPlane; 6] {
    let m = view_proj.cols;
    let rows = [
        [
            m[0][0] + m[3][0],
            m[0][1] + m[3][1],
            m[0][2] + m[3][2],
            m[0][3] + m[3][3],
        ],
        [
            m[0][0] - m[3][0],
            m[0][1] - m[3][1],
            m[0][2] - m[3][2],
            m[0][3] - m[3][3],
        ],
        [
            m[0][0] + m[1][0],
            m[0][1] + m[1][1],
            m[0][2] + m[1][2],
            m[0][3] + m[1][3],
        ],
        [
            m[0][0] - m[1][0],
            m[0][1] - m[1][1],
            m[0][2] - m[1][2],
            m[0][3] - m[1][3],
        ],
        [
            m[0][0] + m[2][0],
            m[0][1] + m[2][1],
            m[0][2] + m[2][2],
            m[0][3] + m[2][3],
        ],
        [
            m[0][0] - m[2][0],
            m[0][1] - m[2][1],
            m[0][2] - m[2][2],
            m[0][3] - m[2][3],
        ],
    ];
    let mut planes = [FrustumPlane {
        normal: Vec3::ZERO,
        distance: 0.0,
    }; 6];
    for (index, row) in rows.into_iter().enumerate() {
        let normal = Vec3::new(row[0], row[1], row[2]);
        let length = normal.length();
        if length > 1e-8 {
            planes[index] = FrustumPlane {
                normal: normal.scale(1.0 / length),
                distance: row[3] / length,
            };
        }
    }
    planes
}

pub fn transform_aabb(model: Mat4, min: [f32; 3], max: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    let corners = [
        [min[0], min[1], min[2]],
        [max[0], min[1], min[2]],
        [min[0], max[1], min[2]],
        [max[0], max[1], min[2]],
        [min[0], min[1], max[2]],
        [max[0], min[1], max[2]],
        [min[0], max[1], max[2]],
        [max[0], max[1], max[2]],
    ];
    let mut out_min = [f32::INFINITY; 3];
    let mut out_max = [f32::NEG_INFINITY; 3];
    for corner in corners {
        let world = model.transform_point(Vec3::from_array(corner)).to_array();
        for axis in 0..3 {
            out_min[axis] = out_min[axis].min(world[axis]);
            out_max[axis] = out_max[axis].max(world[axis]);
        }
    }
    (out_min, out_max)
}

pub fn aabb_intersects_frustum(planes: &[FrustumPlane; 6], min: [f32; 3], max: [f32; 3]) -> bool {
    for plane in planes {
        let positive = Vec3::new(
            if plane.normal.x >= 0.0 { max[0] } else { min[0] },
            if plane.normal.y >= 0.0 { max[1] } else { min[1] },
            if plane.normal.z >= 0.0 { max[2] } else { min[2] },
        );
        if plane.normal.dot(positive) + plane.distance < 0.0 {
            return false;
        }
    }
    true
}
//#endregion Culling

//#region Picking
pub fn ray_aabb_slab(origin: Vec3, dir: Vec3, min: [f32; 3], max: [f32; 3]) -> Option<f32> {
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;
    let axes = [origin.x, origin.y, origin.z];
    let dirs = [dir.x, dir.y, dir.z];
    let bounds = [(min[0], max[0]), (min[1], max[1]), (min[2], max[2])];
    for axis in 0..3 {
        if dirs[axis].abs() < 1e-8 {
            if axes[axis] < bounds[axis].0 || axes[axis] > bounds[axis].1 {
                return None;
            }
            continue;
        }
        let inv = 1.0 / dirs[axis];
        let mut t0 = (bounds[axis].0 - axes[axis]) * inv;
        let mut t1 = (bounds[axis].1 - axes[axis]) * inv;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max < t_min {
            return None;
        }
    }
    if t_max < 0.0 {
        return None;
    }
    Some(if t_min >= 0.0 { t_min } else { t_max })
}

pub fn ray_pick_instance(
    origin: Vec3,
    dir: Vec3,
    mesh: &Mesh3d,
    instance: &Instance3d,
) -> Option<f32> {
    let (world_min, world_max) = transform_aabb(instance.model, mesh.aabb_min, mesh.aabb_max);
    if ray_aabb_slab(origin, dir, world_min, world_max).is_none() {
        return None;
    }
    let mut best = None;
    for tri in mesh.indices.chunks_exact(3) {
        let a = instance.model.transform_point(vertex(mesh, tri[0]));
        let b = instance.model.transform_point(vertex(mesh, tri[1]));
        let c = instance.model.transform_point(vertex(mesh, tri[2]));
        if let Some(t) = ray_triangle(origin, dir, a, b, c) {
            best = Some(best.map_or(t, |prev: f32| prev.min(t)));
        }
    }
    best
}

fn vertex(mesh: &Mesh3d, index: u32) -> Vec3 {
    let i = index as usize * 3;
    Vec3::new(mesh.positions[i], mesh.positions[i + 1], mesh.positions[i + 2])
}

fn ray_triangle(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<f32> {
    let edge1 = b.sub(a);
    let edge2 = c.sub(a);
    let h = dir.cross(edge2);
    let det = edge1.dot(h);
    if det.abs() < 1e-8 {
        return None;
    }
    let f = 1.0 / det;
    let s = origin.sub(a);
    let u = f * s.dot(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross(edge1);
    let v = f * dir.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * edge2.dot(q);
    if t > 1e-4 { Some(t) } else { None }
}

pub fn project_point(view_proj: Mat4, point: Vec3, width: f32, height: f32) -> Option<[f32; 2]> {
    let clip = view_proj.transform_point(point);
    if clip.z < -1.0 || clip.z > 1.0 {
        return None;
    }
    Some([(clip.x * 0.5 + 0.5) * width, (1.0 - (clip.y * 0.5 + 0.5)) * height])
}

pub fn point_in_polygon(point: [f32; 2], polygon: &[[f32; 2]]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut winding = 0;
    for index in 0..polygon.len() {
        let a = polygon[index];
        let b = polygon[(index + 1) % polygon.len()];
        if a[1] <= point[1] {
            if b[1] > point[1] && cross2(a, b, point) > 0.0 {
                winding += 1;
            }
        } else if b[1] <= point[1] && cross2(a, b, point) < 0.0 {
            winding -= 1;
        }
    }
    winding != 0
}

fn cross2(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

pub fn rect_contains(rect: [f32; 4], point: [f32; 2]) -> bool {
    let min_x = rect[0].min(rect[2]);
    let max_x = rect[0].max(rect[2]);
    let min_y = rect[1].min(rect[3]);
    let max_y = rect[1].max(rect[3]);
    point[0] >= min_x && point[0] <= max_x && point[1] >= min_y && point[1] <= max_y
}

pub fn projected_aabb_bounds(
    view_proj: Mat4,
    model: Mat4,
    min: [f32; 3],
    max: [f32; 3],
    width: f32,
    height: f32,
) -> Option<[f32; 4]> {
    let corners = [
        [min[0], min[1], min[2]],
        [max[0], min[1], min[2]],
        [min[0], max[1], min[2]],
        [max[0], max[1], min[2]],
        [min[0], min[1], max[2]],
        [max[0], min[1], max[2]],
        [min[0], max[1], max[2]],
        [max[0], max[1], max[2]],
    ];
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut visible = false;
    for corner in corners {
        let world = model.transform_point(Vec3::from_array(corner));
        if let Some(screen) = project_point(view_proj, world, width, height) {
            visible = true;
            min_x = min_x.min(screen[0]);
            min_y = min_y.min(screen[1]);
            max_x = max_x.max(screen[0]);
            max_y = max_y.max(screen[1]);
        }
    }
    visible.then_some([min_x, min_y, max_x, max_y])
}

fn aabb_overlaps_marquee(
    projected: [f32; 4],
    polygon: &[[f32; 2]],
    rectangle: bool,
) -> bool {
    if rectangle {
        let marquee = [polygon[0][0], polygon[0][1], polygon[1][0], polygon[1][1]];
        let marquee_min_x = marquee[0].min(marquee[2]);
        let marquee_max_x = marquee[0].max(marquee[2]);
        let marquee_min_y = marquee[1].min(marquee[3]);
        let marquee_max_y = marquee[1].max(marquee[3]);
        return projected[2] >= marquee_min_x
            && projected[0] <= marquee_max_x
            && projected[3] >= marquee_min_y
            && projected[1] <= marquee_max_y;
    }
    let corners = [
        [projected[0], projected[1]],
        [projected[2], projected[1]],
        [projected[2], projected[3]],
        [projected[0], projected[3]],
    ];
    corners.iter().any(|corner| point_in_polygon(*corner, polygon))
        || polygon.iter().any(|point| {
            point[0] >= projected[0]
                && point[0] <= projected[2]
                && point[1] >= projected[1]
                && point[1] <= projected[3]
        })
}

pub fn screen_select_instances(
    mesh_lookup: &std::collections::HashMap<String, Mesh3d>,
    draws: &[SceneDraw3d],
    view_proj: Mat4,
    width: f32,
    height: f32,
    polygon: &[[f32; 2]],
    rectangle: bool,
) -> Vec<String> {
    let mut selected = Vec::new();
    for draw in draws {
        let Some(mesh) = mesh_lookup.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            let Some(projected) = projected_aabb_bounds(
                view_proj,
                instance.model,
                mesh.aabb_min,
                mesh.aabb_max,
                width,
                height,
            ) else {
                continue;
            };
            if !aabb_overlaps_marquee(projected, polygon, rectangle) {
                continue;
            }
            let mut covered = false;
            for tri in mesh.indices.chunks_exact(3) {
                let mut projected = Vec::new();
                for &index in tri {
                    let world = instance.model.transform_point(vertex(mesh, index));
                    if let Some(screen) = project_point(view_proj, world, width, height) {
                        projected.push(screen);
                    }
                }
                if projected.len() < 3 {
                    continue;
                }
                let centroid = [
                    (projected[0][0] + projected[1][0] + projected[2][0]) / 3.0,
                    (projected[0][1] + projected[1][1] + projected[2][1]) / 3.0,
                ];
                covered = if rectangle {
                    rect_contains(
                        [polygon[0][0], polygon[0][1], polygon[1][0], polygon[1][1]],
                        centroid,
                    )
                } else {
                    point_in_polygon(centroid, polygon)
                };
                if covered {
                    break;
                }
            }
            if covered {
                selected.push(instance.id.clone());
            }
        }
    }
    selected
}
//#endregion Picking

#[cfg(test)]
mod tests {
    use super::*;

    fn test_box_mesh() -> Mesh3d {
        Mesh3d::from_buffers(
            vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
            vec![0, 1, 2],
        )
    }

    #[test]
    fn orbit_round_trip() {
        let camera = Camera3d::default();
        let orbit = OrbitController::from_camera(&camera);
        let next = orbit.to_camera();
        assert!((next.position.x - camera.position.x).abs() < 0.5);
    }

    #[test]
    fn point_in_square() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        assert!(point_in_polygon([5.0, 5.0], &square));
        assert!(!point_in_polygon([20.0, 5.0], &square));
    }

    #[test]
    fn ray_hits_triangle_direct() {
        let hit = ray_triangle(
            Vec3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert!(hit.is_some());
    }

    #[test]
    fn ray_hits_box() {
        let mesh = test_box_mesh();
        let instance = Instance3d {
            id: "box".into(),
            model: Mat4::identity(),
            color: [1.0, 1.0, 1.0, 1.0],
            selected: false,
            hovered: false,
        };
        let hit = ray_pick_instance(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0), &mesh, &instance);
        assert!(hit.is_some());
    }

    #[test]
    fn ray_aabb_misses_offset_box() {
        let mesh = test_box_mesh();
        let instance = Instance3d {
            id: "box".into(),
            model: Mat4::translation(Vec3::new(100.0, 0.0, 0.0)),
            color: [1.0, 1.0, 1.0, 1.0],
            selected: false,
            hovered: false,
        };
        let hit = ray_pick_instance(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0), &mesh, &instance);
        assert!(hit.is_none());
    }

    #[test]
    fn frustum_contains_origin_box() {
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let planes = frustum_planes(view_proj);
        assert!(aabb_intersects_frustum(&planes, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]));
    }

    #[test]
    fn projected_aabb_skips_far_instance() {
        let mesh = test_box_mesh();
        let draws = vec![SceneDraw3d {
            mesh_key: "box".into(),
            mesh_version: 0,
            instances: vec![Instance3d {
                id: "far".into(),
                model: Mat4::translation(Vec3::new(0.0, 0.0, -500.0)),
                color: [1.0, 1.0, 1.0, 1.0],
                selected: false,
                hovered: false,
            }],
        }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let ids = screen_select_instances(
            &lookup,
            &draws,
            view_proj,
            200.0,
            200.0,
            &[[0.0, 0.0], [200.0, 0.0], [200.0, 200.0], [0.0, 200.0]],
            true,
        );
        assert!(ids.is_empty());
    }

    #[test]
    fn world_globals_slot_alignment() {
        use crate::draw::WORLD_GLOBALS_SLOT_SIZE;
        assert!(WORLD_GLOBALS_SLOT_SIZE >= 80);
        assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
    }
}
