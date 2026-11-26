//! 📐️ The geometry of a gbXML surface: the polygon in a frame whose +Y points at true north (so `Azimuth` means the same whatever `CADModelAzimuth` a reader applies), the frame of the surface seen from outside (right, up, outward normal), and its
//! rectangular geometry (the bounding rectangle in that frame with its lower-left corner), all rounded to nine decimals so the document and the tables the oracles compare are made of the same numbers.
//! 📎 https://www.gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html

use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::Vec3;

/// 📍️ A point or vector in metres.
pub type P3 = [f64; 3];

/// 🔢️ A number rounded to nine decimals, never `-0`.
pub fn round9(value: f64) -> f64 {
    let rounded = (value * 1e9).round() / 1e9;
    if rounded == 0.0 {
        0.0
    } else {
        rounded
    }
}

/// 🧭️ A point of the building turned clockwise by `bearing` radians (true north minus the rotation of the building) so +Y is true north.
pub fn turned(point: &Vec3, bearing: f64) -> P3 {
    let (sine, cosine) = bearing.sin_cos();
    [round9(point.x * cosine + point.y * sine), round9(-point.x * sine + point.y * cosine), round9(point.z)]
}

fn sub(a: P3, b: P3) -> P3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: P3, b: P3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: P3, b: P3) -> P3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn unit(a: P3) -> Option<P3> {
    let length = dot(a, a).sqrt();
    (length > 1e-12).then(|| [a[0] / length, a[1] / length, a[2] / length])
}

/// 🧮️ Twice the area vector of a polygon (Newell): its direction is the normal of the counter-clockwise side, its length twice the area.
pub fn area_vector(polygon: &[P3]) -> P3 {
    let mut sum = [0.0; 3];
    for (index, a) in polygon.iter().enumerate() {
        add_edge(&mut sum, *a, polygon[(index + 1) % polygon.len()]);
    }
    sum
}

fn add_edge(sum: &mut P3, a: P3, b: P3) {
    sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
    sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
    sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
}

/// 📏️ The area of a planar polygon.
pub fn area(polygon: &[P3]) -> f64 {
    vector_area(area_vector(polygon))
}

fn vector_area(vector: P3) -> f64 { dot(vector, vector).sqrt() / 2.0 }

/// 🪟️ The frame of a surface seen from outside: `right` and `up` span its plane, `normal` points out. Up is the vertical projected into the plane (for a horizontal surface north), right is `up x normal`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub right: P3,
    pub up: P3,
    pub normal: P3,
}

impl Frame {
    /// 🪟️ The frame of a polygon, `None` for a degenerate one.
    pub fn of(polygon: &[P3]) -> Option<Self> {
        Self::from_area_vector(area_vector(polygon))
    }

    fn from_area_vector(vector: P3) -> Option<Self> {
        let normal = unit(vector)?;
        let normal = if normal[0].hypot(normal[1]) < 1e-8 { [0.0, 0.0, normal[2].signum()] } else { normal };
        let vertical = [0.0, 0.0, 1.0];
        let up = unit(sub(vertical, scaled(normal, dot(vertical, normal)))).or_else(|| unit(sub([0.0, 1.0, 0.0], scaled(normal, normal[1]))))?;
        Some(Self { right: cross(up, normal), up, normal })
    }

    /// 🧭️ The azimuth of the outward normal in degrees clockwise from +Y, zero for a horizontal surface.
    pub fn azimuth(&self) -> f64 {
        if self.normal[0].hypot(self.normal[1]) < 1e-9 {
            0.0
        } else {
            round9(self.normal[0].atan2(self.normal[1]).to_degrees().rem_euclid(360.0))
        }
    }

    /// 📐️ The tilt in degrees: 0 faces up, 90 a wall, 180 faces down.
    pub fn tilt(&self) -> f64 {
        round9(self.normal[2].clamp(-1.0, 1.0).acos().to_degrees())
    }
}

fn scaled(a: P3, factor: f64) -> P3 {
    [a[0] * factor, a[1] * factor, a[2] * factor]
}

/// 📦️ The rectangular geometry of a polygon: the lower-left corner of its bounding rectangle in the frame, and the width and height of that rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub corner: P3,
    pub width: f64,
    pub height: f64,
}

/// 📦️ The bounding rectangle of `polygon` in the axes of `frame`, measured from the first vertex.
pub fn rect_in(frame: &Frame, polygon: &[P3]) -> Rect {
    let mut bounds = Bounds::new(polygon[0]);
    for point in polygon {
        bounds.add(frame, *point);
    }
    bounds.rect(frame)
}

#[derive(Clone, Copy, Debug)]
struct Bounds {
    origin: P3,
    low: [f64; 2],
    high: [f64; 2],
}

impl Bounds {
    fn new(origin: P3) -> Self {
        Self { origin, low: [f64::INFINITY; 2], high: [f64::NEG_INFINITY; 2] }
    }

    fn add(&mut self, frame: &Frame, point: P3) {
        let offset = sub(point, self.origin);
        let local = [dot(offset, frame.right), dot(offset, frame.up)];
        for axis in 0..2 {
            self.low[axis] = self.low[axis].min(local[axis]);
            self.high[axis] = self.high[axis].max(local[axis]);
        }
    }

    fn rect(&self, frame: &Frame) -> Rect {
        let [x, y] = self.low;
        let corner = std::array::from_fn(|axis| self.origin[axis] + frame.right[axis] * x + frame.up[axis] * y);
        Rect { corner: corner.map(round9), width: round9(self.high[0] - x), height: round9(self.high[1] - y) }
    }
}

/// 📐️ Shared geometry of a canonical surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectedPolygon {
    pub frame: Frame,
    pub rect: Rect,
    pub area: f64,
}

/// ⏱️ Incrementally calculates a surface frame and its bounds using only scalar state.
#[derive(Clone, Copy, Debug)]
pub struct PolygonProjection {
    len: usize,
    next: usize,
    vector: P3,
    frame: Option<(Frame, Bounds)>,
    result: Option<Option<ProjectedPolygon>>,
}

impl PolygonProjection {
    /// 🧭️ Starts one polygon with the vertex count of the borrowed committed surface.
    pub fn new(len: usize) -> Self {
        Self { len, next: 0, vector: [0.0; 3], frame: None, result: (len < 3).then_some(None) }
    }

    /// 🔢️ Advances up to `steps` edges or vertices. The point source must remain the same for the cursor's lifetime.
    pub fn advance(&mut self, steps: usize, mut point: impl FnMut(usize) -> P3) -> Option<Option<ProjectedPolygon>> {
        for _ in 0..steps {
            if self.result.is_some() { break; }
            if let Some((frame, bounds)) = &mut self.frame {
                bounds.add(frame, point(self.next));
                self.next += 1;
                if self.next == self.len {
                    self.result = Some(Some(ProjectedPolygon { frame: *frame, rect: bounds.rect(frame), area: vector_area(self.vector) }));
                }
            } else {
                add_edge(&mut self.vector, point(self.next), point((self.next + 1) % self.len));
                self.next += 1;
                if self.next == self.len {
                    if let Some(frame) = Frame::from_area_vector(self.vector) {
                        self.frame = Some((frame, Bounds::new(point(0))));
                        self.next = 0;
                    } else {
                        self.result = Some(None);
                    }
                }
            }
        }
        self.result
    }
}

/// 📍️ The position of `polygon`'s bounding rectangle inside the rectangle of its host surface: the offset of its corner along the host's right and up axes, from the host's corner.
pub fn offset_in(host: &Frame, host_corner: P3, polygon: &[P3]) -> ([f64; 2], Rect) {
    let rect = rect_in(host, polygon);
    (offset_of(host, host_corner, rect), rect)
}

fn offset_of(host: &Frame, host_corner: P3, rect: Rect) -> [f64; 2] {
    let offset = sub(rect.corner, host_corner);
    [round9(dot(offset, host.right)), round9(dot(offset, host.up))]
}

/// 🪟️ Accumulates an opening's bounding rectangle in a known host frame.
#[derive(Clone, Copy, Debug)]
pub struct OpeningProjection {
    host: Frame,
    corner: P3,
    bounds: Bounds,
    next: usize,
    extent: usize,
}

impl OpeningProjection {
    /// 🪟️ Starts the rectangle at the first original opening vertex.
    pub fn new(host: Frame, corner: P3, first: P3, extent: usize) -> Self {
        Self { host, corner, bounds: Bounds::new(first), next: 0, extent }
    }

    /// 📐️ Reads a bounded slice of opening vertices in canonical order.
    pub fn advance(&mut self, vertices: usize, mut point: impl FnMut(usize) -> P3) -> Option<([f64; 2], Rect)> {
        let end = self.next.saturating_add(vertices).min(self.extent);
        for index in self.next..end {
            self.bounds.add(&self.host, point(index));
        }
        self.next = end;
        (end == self.extent).then(|| {
            let rect = self.bounds.rect(&self.host);
            (offset_of(&self.host, self.corner, rect), rect)
        })
    }
}


