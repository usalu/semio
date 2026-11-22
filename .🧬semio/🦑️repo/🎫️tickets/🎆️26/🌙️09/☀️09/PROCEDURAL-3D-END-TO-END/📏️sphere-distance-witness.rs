use semio_framework_3d_brep_vector::{Pnt3, Vec3};
use semio_framework_value::{ValueError, ValueRefusalKind};

extern crate self as semio_framework_3d_brep_construction;
extern crate self as semio_framework_3d_brep_curve;
extern crate self as semio_framework_3d_brep_solid_transaction;

pub mod skew_box_clip {
    use super::*;
    pub fn boxes_separated(_: (Pnt3, [Vec3; 3], [f64; 3]), _: (Pnt3, [Vec3; 3], [f64; 3]), _: f64) -> Option<bool> {
        unreachable!()
    }
}
pub enum Curve3 { Line { origin: Pnt3, dir: Vec3 } }
mod linear_distance {
    use super::*;
    type Span<'a> = (&'a Curve3, Option<(f64, f64)>);
    pub fn original_linear_distance(_: Span, _: Span) -> Result<f64, ValueError> { unreachable!() }
}

use semio_framework_3d_brep_vector::matrix::Frame3 as Frame;
#[derive(Clone, Copy)]
enum SolidPrimitive {
    Box(OrientedBox),
    Sphere { frame: Frame, radius: f64 },
    Cylinder { frame: Frame, radius: f64, height: f64 },
    Cone { frame: Frame, radius: f64, height: f64 },
    Torus { frame: Frame, major: f64, minor: f64 },
}
impl SolidPrimitive {
    __PRIMITIVE_DISTANCE__
}
#[derive(Clone, Copy)]
pub struct OrientedBox(Pnt3, [Vec3; 3], [f64; 3]);
impl OrientedBox {
    fn from_primitive(primitive: SolidPrimitive) -> Option<Self> {
        if let SolidPrimitive::Box(value) = primitive { Some(value) } else { None }
    }
    fn placement(self) -> (Pnt3, [Vec3; 3], [f64; 3]) { (self.0, self.1, self.2) }
}
struct Rectangular;
impl Rectangular {
    fn rectangular(&self) -> Option<OrientedBox> { unreachable!() }
}
struct PrimitiveSource { primitive: SolidPrimitive }
#[derive(Clone, Copy)]
struct SphericalShell(SolidPrimitive, SolidPrimitive);
impl SphericalShell {
    fn boundary(self, inner: bool) -> SolidPrimitive { if inner { self.1 } else { self.0 } }
}
fn exterior(shape: SphericalShell) -> SolidPrimitive { shape.0 }
__SHELL_DISTANCE__
enum SolidProvenance { Primitive(PrimitiveSource), PlanarBoundary(Option<Rectangular>), SphericalShell(SphericalShell, ()), Orthogonal(()), PlacedOrthogonal((), ()) }
impl SolidProvenance {
    fn certified_signed_distance(&self, point: Pnt3) -> Result<f64, ValueError> {
        match self { Self::SphericalShell(shape, _) => Ok(spherical_shell_distance(*shape, point)), _ => unreachable!() }
    }
}
impl From<SolidPrimitive> for SolidProvenance {
    fn from(primitive: SolidPrimitive) -> Self { Self::Primitive(PrimitiveSource { primitive }) }
}
enum Entity { Solid(usize, Option<SolidProvenance>), Other }
struct SolidArena(Vec<usize>);
impl SolidArena { fn contains(&self, id: usize) -> bool { self.0.contains(&id) } }
struct Body { solids: SolidArena }
struct BrepRegistry { body: Body, entities: [Entity; 2] }
impl BrepRegistry {
    fn original_query_entity(&self, key: &str) -> Result<&Entity, ValueError> {
        Ok(&self.entities[usize::from(key == "right")])
    }
}

type Spec = serde_json::Value;
fn xyz(spec: &Spec) -> [f64; 3] { serde_json::from_value(spec.clone()).unwrap() }
fn number(spec: &Spec) -> f64 { spec.as_f64().unwrap() }
fn sphere(spec: &Spec) -> SolidPrimitive {
    SolidPrimitive::Sphere { frame: Frame { origin: Pnt3::from_array(xyz(&spec["center"])), ..Frame::WORLD }, radius: number(&spec["radius"]) }
}
fn box_shape(spec: &Spec) -> SolidPrimitive {
    let (sin, cos) = number(&spec["rotation"]).sin_cos();
    let axes = [[cos, sin, 0.0], [-sin, cos, 0.0], [0.0, 0.0, 1.0]].map(Vec3::from_array);
    SolidPrimitive::Box(OrientedBox(Pnt3::from_array(xyz(&spec["origin"])), axes, xyz(&spec["size"])))
}
fn compare(a: impl Into<SolidProvenance>, b: impl Into<SolidProvenance>, row: &Spec) {
    let entities = [a.into(),b.into()].map(|proof| Entity::Solid(0, Some(proof)));
    let registry = BrepRegistry { body: Body { solids: SolidArena(vec![0]) }, entities };
    let mut results = Vec::new();
    for [first, second] in [["left", "right"], ["right", "left"]] {
        let actual = registry.__DISTANCE_METHOD__(first, second).unwrap_or_else(|_| panic!("{} unexpectedly refused", row["name"]));
        let expected = number(&row["expected"]);
        assert!((actual - expected).abs() < 1e-12, "{}: {actual} != {expected}", row["name"]);
        results.push(actual);
    }
    assert_eq!(results[0], results[1]);
}
#[test]
fn sphere_pairs() {
    let fixture: Spec = serde_json::from_str(include_str!(__FIXTURE_PATH__)).unwrap();
    for row in fixture["spherePairs"].as_array().unwrap() {
        compare(sphere(&row["left"]), sphere(&row["right"]), row);
    }
}
#[test]
fn sphere_curved_solids() {
    let fixture: Spec = serde_json::from_str(include_str!(__FIXTURE_PATH__)).unwrap();
    for row in fixture["curvedSolids"].as_array().unwrap() {
        let frame = Frame { origin: Pnt3::from_array(xyz(&row["origin"])), ..Frame::WORLD };
        let param = |name| number(&row["parameters"][name]);
        let solid = match row["kind"].as_str().unwrap() {
            "cylinder" => SolidPrimitive::Cylinder { frame, radius: param("radius"), height: param("height") },
            "cone" => SolidPrimitive::Cone { frame, radius: param("radius"), height: param("height") },
            "torus" => SolidPrimitive::Torus { frame, major: param("major"), minor: param("minor") },
            _ => unreachable!(),
        };
        compare(solid, sphere(&row["sphere"]), row);
    }
}
fn cylinder(spec: &Spec) -> SolidPrimitive {
    let normal = if spec["reverse"] == true { -Vec3::Z } else { Vec3::Z };
    let frame = Frame::from_normal(Pnt3::from_array(xyz(&spec["center"])), normal).unwrap();
    SolidPrimitive::Cylinder { frame, radius: number(&spec["radius"]), height: number(&spec["height"]) }
}
#[test]
fn parallel_cylinders() {
    let fixture: Spec = serde_json::from_str(include_str!(__CYLINDER_FIXTURE_PATH__)).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        compare(cylinder(&fixture["stock"]), cylinder(row), row);
    }
}
#[test]
fn cylinder_boxes() {
    let fixture: Spec = serde_json::from_str(include_str!(__CYLINDER_FIXTURE_PATH__)).unwrap();
    for row in fixture["boxes"].as_array().unwrap() {
        compare(box_shape(&row["box"]), cylinder(&row["cylinder"]), row);
    }
}
#[test]
fn sphere_cavities() {
    let fixture: Spec = serde_json::from_str(include_str!(__CAVITY_FIXTURE_PATH__)).unwrap();
    for shell in fixture["shells"].as_array().unwrap() {
        let shape = if shell["kind"] == "sphere" {
            let origin = Pnt3::from_array(xyz(&shell["origin"]));
            SolidPrimitive::Sphere { frame: Frame { origin, ..Frame::WORLD }, radius: number(&shell["radius"]) }
        } else {
            let origin = Pnt3::from_array(xyz(&shell["origin"]));
            SolidPrimitive::Box(OrientedBox(origin, [Vec3::X, Vec3::Y, Vec3::Z], xyz(&shell["size"])))
        };
        for row in shell["cases"].as_array().unwrap() {
            compare(SolidProvenance::SphericalShell(SphericalShell(shape, sphere(&shell["void"])), ()), sphere(row), row);
        }
    }
}
#[test]
fn sphere_boxes() {
    let fixture: Spec = serde_json::from_str(include_str!(__FIXTURE_PATH__)).unwrap();
    for row in fixture["sphereBoxes"].as_array().unwrap() {
        compare(box_shape(row), sphere(&row["sphere"]), row);
    }
}
