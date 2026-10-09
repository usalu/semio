//! 🧊️ `solids`: the evaluated solids of a family. Each formula slot of a [`FamilySolid`] is parsed and evaluated against the resolved parameters; a slot that fails becomes an issue and leaves the solid
//! empty. The geometry is the framework's: [`extrude_loops`] for extrusions and cuboids, [`sweep_profile`] for sweeps along a plan path, [`revolve_profile`] for revolutions, all within the chord
//! tolerance of the element solids. A solid lives in the family frame (metres, `z` up); its offset moves it.

use super::issues::{FamilyIssue, FamilyIssueCode, IssueOwner};
use super::FamilySolidMesh;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{SolidBounds, SolidPoint, CHORD_TOLERANCE};
use crate::{ExprPoint, FamilySolid, ModelSnapshot, ParametricProfile, SolidAxis, SolidShape};
use semio_framework_expression::{evaluate, parse, Kind, Value};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::loops::{self, Vertex};
use semio_framework_geometry::mesh::{extrude_loops, revolve_profile, sweep_profile, TriMesh};
use semio_framework_geometry::placement::{Affine3, ZPlane};
use semio_framework_geometry::Point;
use std::collections::{BTreeMap, BTreeSet};

/// 🎁️ A solid with the cross-section it was built from (an extrusion only): the profile of a profile family is the section of its first visible extrusion.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Evaluated {
    pub mesh: FamilySolidMesh,
    pub section: Option<Vec<Vertex>>,
    pub offset: [f64; 3],
}

struct Cx<'a> {
    id: &'a str,
    env: &'a BTreeMap<String, Value>,
    names: &'a BTreeSet<String>,
    issues: Vec<FamilyIssue>,
}

impl Cx<'_> {
    fn value(&mut self, field: &str, text: &str, want: Kind) -> Option<Value> {
        let expr = match parse(text) {
            Ok(expr) => expr,
            Err(error) => {
                self.issues.push(FamilyIssue::of_parse(IssueOwner::Solid, self.id, field, &error));
                return None;
            }
        };
        match evaluate(&expr, self.env) {
            Ok(value) if value.kind() == want => Some(value),
            Ok(value) => {
                self.issues.push(FamilyIssue::new(FamilyIssueCode::Kind, IssueOwner::Solid, self.id, field, format!("expected {} but the formula is {}", want.name(), value.kind().name()), Vec::new()));
                None
            }
            Err(error) => {
                let names = self.names;
                self.issues.push(FamilyIssue::of_error(IssueOwner::Solid, self.id, field, &error, &|name| names.contains(name)));
                None
            }
        }
    }

    fn length(&mut self, field: &str, text: &str) -> Option<f64> {
        self.value(field, text, Kind::Length).and_then(|value| value.magnitude())
    }

    fn positive(&mut self, field: &str, text: &str) -> Option<f64> {
        let value = self.length(field, text)?;
        if value > 1e-9 {
            return Some(value);
        }
        self.issues.push(FamilyIssue::new(FamilyIssueCode::Negative, IssueOwner::Solid, self.id, field, format!("{field} is {value} m"), Vec::new()));
        None
    }

    fn outline(&mut self, field: &str, detail: impl Into<String>) {
        self.issues.push(FamilyIssue::new(FamilyIssueCode::Outline, IssueOwner::Solid, self.id, field, detail, Vec::new()));
    }

    fn point(&mut self, field: &str, point: &ExprPoint) -> Option<Point> {
        let x = self.length(&format!("{field}.x"), &point.x);
        let y = self.length(&format!("{field}.y"), &point.y);
        Some(Point::new(x?, y?))
    }
}

/// ▭️ The counter-clockwise loop of a profile in `(u, v)`: rectangle and I-shape centred on the origin, a circle as two half arcs, a polygon as given (reoriented).
fn profile_loop(cx: &mut Cx<'_>, field: &str, profile: &ParametricProfile) -> Option<Vec<Vertex>> {
    match profile {
        ParametricProfile::Rectangle { width, depth } => {
            let (w, d) = (cx.positive(&format!("{field}.width"), width), cx.positive(&format!("{field}.depth"), depth));
            let (w, d) = (w? / 2.0, d? / 2.0);
            Some([(-w, -d), (w, -d), (w, d), (-w, d)].iter().map(|&(x, y)| Vertex::corner(x, y)).collect())
        }
        ParametricProfile::Circle { diameter } => {
            let radius = cx.positive(&format!("{field}.diameter"), diameter)? / 2.0;
            Some(vec![Vertex::new(Point::new(radius, 0.0), 1.0), Vertex::new(Point::new(-radius, 0.0), 1.0)])
        }
        ParametricProfile::IShape { width, depth, web, flange } => {
            let measures = [("width", width), ("depth", depth), ("web", web), ("flange", flange)].map(|(name, text)| cx.positive(&format!("{field}.{name}"), text));
            let [w, d, t, f] = measures;
            let (w, d, t, f) = (w?, d?, t?, f?);
            if t >= w || 2.0 * f >= d {
                cx.outline(field, "the web must be thinner than the width and the flanges thinner than half the depth");
                return None;
            }
            let (x, y, h) = (w / 2.0, d / 2.0, t / 2.0);
            Some([(-x, -y), (x, -y), (x, -y + f), (h, -y + f), (h, y - f), (x, y - f), (x, y), (-x, y), (-x, y - f), (-h, y - f), (-h, -y + f), (-x, -y + f)].iter().map(|&(px, py)| Vertex::corner(px, py)).collect())
        }
        ParametricProfile::Polygon { points } => {
            let evaluated: Vec<Option<Point>> = points.iter().enumerate().map(|(index, point)| cx.point(&format!("{field}.points[{index}]"), point)).collect();
            let all: Option<Vec<Point>> = evaluated.into_iter().collect();
            let all = all?;
            if all.len() < 3 {
                cx.outline(field, "a polygon needs at least three points");
                return None;
            }
            let outline = loops::ccw(&all.iter().map(|point| Vertex::new(*point, 0.0)).collect::<Vec<_>>());
            if loops::area(&outline) < 1e-12 || !loops::self_intersections(&outline).is_empty() {
                cx.outline(field, "the polygon is degenerate or crosses itself");
                return None;
            }
            Some(outline)
        }
    }
}

fn frame_of(axis: SolidAxis) -> Affine3 {
    match axis {
        SolidAxis::Z => Affine3::IDENTITY,
        SolidAxis::X => Affine3::from_frame([0.0; 3], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        SolidAxis::Y => Affine3::from_frame([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    }
}

fn flat(rows: &[[f64; 3]]) -> Vec<f64> {
    rows.iter().flatten().copied().collect()
}

fn mesh_of(name: &str, material: String, visible: bool, mesh: &TriMesh) -> FamilySolidMesh {
    let bounds = mesh.bounds().map_or_else(SolidBounds::default, |(lo, hi)| SolidBounds { min: SolidPoint { x: lo[0], y: lo[1], z: lo[2] }, max: SolidPoint { x: hi[0], y: hi[1], z: hi[2] } });
    FamilySolidMesh { name: name.to_string(), material, visible, positions: flat(&mesh.positions), normals: flat(&mesh.normals), indices: mesh.indices.iter().flatten().copied().collect(), bounds, volume: mesh.volume(), area: mesh.surface_area() }
}

/// ▶️ Evaluates one solid against the resolved parameters; `materials` are the material ids of the project.
pub fn evaluate_solid(id: &str, solid: &FamilySolid, env: &BTreeMap<String, Value>, names: &BTreeSet<String>, snapshot: &ModelSnapshot) -> (Evaluated, Vec<FamilyIssue>) {
    let mut cx = Cx { id, env, names, issues: Vec::new() };
    let visible = cx.value("visible", &solid.visible, Kind::Bool).and_then(|value| if let Value::Bool(flag) = value { Some(flag) } else { None }).unwrap_or(true);
    let material = match cx.value("material", &solid.material, Kind::Text) {
        Some(Value::Text(material)) => {
            if !material.is_empty() && !snapshot.materials.contains_key(&material) {
                cx.issues.push(FamilyIssue::new(FamilyIssueCode::Unknown, IssueOwner::Solid, id, "material", format!("material `{material}` does not exist"), vec![material.clone()]));
            }
            material
        }
        _ => String::new(),
    };
    let offset = [cx.length("offset.x", &solid.offset.x), cx.length("offset.y", &solid.offset.y), cx.length("offset.z", &solid.offset.z)];
    let mut section: Option<Vec<Vertex>> = None;
    let mesh = match &solid.shape {
        SolidShape::Extrusion { profile, base, height } => {
            let outline = profile_loop(&mut cx, "profile", profile);
            let (base, height) = (cx.length("base", base), cx.positive("height", height));
            match (outline, base, height) {
                (Some(outline), Some(base), Some(height)) => {
                    let mesh = extrude_loops(&outline, &[], CHORD_TOLERANCE, ZPlane::flat(base), ZPlane::flat(base + height));
                    section = Some(outline);
                    Some(mesh)
                }
                _ => None,
            }
        }
        SolidShape::Cuboid { x, y, z, width, depth, height } => {
            let corner = [cx.length("x", x), cx.length("y", y), cx.length("z", z)];
            let size = [cx.positive("width", width), cx.positive("depth", depth), cx.positive("height", height)];
            match (corner, size) {
                ([Some(x), Some(y), Some(z)], [Some(w), Some(d), Some(h)]) => {
                    let ring: Vec<Vertex> = [(x, y), (x + w, y), (x + w, y + d), (x, y + d)].iter().map(|&(px, py)| Vertex::corner(px, py)).collect();
                    Some(extrude_loops(&ring, &[], CHORD_TOLERANCE, ZPlane::flat(z), ZPlane::flat(z + h)))
                }
                _ => None,
            }
        }
        SolidShape::Sweep { profile, path } => {
            let outline = profile_loop(&mut cx, "profile", profile);
            let points: Vec<Option<Point>> = path.iter().enumerate().map(|(index, point)| cx.point(&format!("path[{index}]"), point)).collect();
            let points: Option<Vec<Point>> = points.into_iter().collect();
            match (outline, points) {
                (Some(outline), Some(points)) => {
                    let mut distinct: Vec<Point> = Vec::new();
                    for point in points {
                        if distinct.last().is_none_or(|last| (point - *last).hypot() > 1e-9) {
                            distinct.push(point);
                        }
                    }
                    if distinct.len() < 2 {
                        cx.outline("path", "a sweep path needs at least two distinct points");
                        None
                    } else {
                        let segments: Vec<BulgeSeg> = distinct.windows(2).map(|pair| BulgeSeg::line(pair[0], pair[1])).collect();
                        Some(sweep_profile(&loops::flatten(&outline, CHORD_TOLERANCE), &[], &segments, 0.0, CHORD_TOLERANCE))
                    }
                }
                _ => None,
            }
        }
        SolidShape::Revolution { profile, axis, angle } => {
            let outline = profile_loop(&mut cx, "profile", profile);
            let turn = cx.value("angle", angle, Kind::Angle).and_then(|value| value.magnitude());
            match (outline, turn) {
                (Some(outline), Some(turn)) => {
                    let ring = loops::flatten(&outline, CHORD_TOLERANCE);
                    if turn <= 1e-9 || turn > std::f64::consts::TAU + 1e-9 {
                        cx.issues.push(FamilyIssue::new(FamilyIssueCode::Negative, IssueOwner::Solid, id, "angle", format!("the angle is {turn} rad, it must lie in (0, 2π]"), Vec::new()));
                        None
                    } else if ring.iter().any(|point| point.x < -1e-9) {
                        cx.outline("profile", "the profile of a revolution must not cross the axis");
                        None
                    } else {
                        Some(revolve_profile(&ring, turn, CHORD_TOLERANCE).transformed(&frame_of(*axis)))
                    }
                }
                _ => None,
            }
        }
    };
    let placed = match (mesh, offset) {
        (Some(mesh), [Some(x), Some(y), Some(z)]) => mesh.translated([x, y, z]),
        _ => TriMesh::new(),
    };
    let moved = [offset[0].unwrap_or(0.0), offset[1].unwrap_or(0.0), offset[2].unwrap_or(0.0)];
    (Evaluated { mesh: mesh_of(&solid.name, material, visible, &placed), section, offset: moved }, cx.issues)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
