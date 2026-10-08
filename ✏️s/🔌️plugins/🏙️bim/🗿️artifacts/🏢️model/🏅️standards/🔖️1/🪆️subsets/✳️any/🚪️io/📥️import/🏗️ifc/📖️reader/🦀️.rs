//! 📖️ Typed reading of an IFC 2x3 Part-21 graph: id lookup, the relationship index (containment, aggregation, voids, fillings, typing, materials, property definitions,
//! classifications), placement chains, curves, profiles and the swept body of a product.

use super::frames::{bulge_of, Rigid};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};
use std::collections::BTreeMap;

pub type Args = Vec<Part21Value>;
pub type Loop = Vec<([f64; 2], f64)>;

/// ▭️ The cross-section of a swept solid in its own plane.
#[derive(Clone, Debug, PartialEq)]
pub enum Section {
    Rectangle { centre: [f64; 2], width: f64, depth: f64 },
    Circle { diameter: f64 },
    IShape { width: f64, depth: f64, web: f64, flange: f64 },
    Outline { outer: Loop, holes: Vec<Loop> },
}

/// 🧊️ An `IfcExtrudedAreaSolid`: its section, where the section's plane sits in the product, the extrusion direction and the depth.
#[derive(Clone, Debug, PartialEq)]
pub struct Extrusion {
    pub section: Section,
    pub position: Rigid,
    pub direction: [f64; 3],
    pub depth: f64,
}

/// 〰️ The first item of a representation: a two-point polyline or a trimmed circle, in the product's plane.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub bulge: f64,
}

/// 🔗️ The relationships of a document, indexed by the instances they connect.
#[derive(Default)]
pub struct Index {
    pub contained: BTreeMap<u64, u64>,
    pub parts: BTreeMap<u64, Vec<u64>>,
    pub whole: BTreeMap<u64, u64>,
    pub voids: BTreeMap<u64, Vec<u64>>,
    pub filling: BTreeMap<u64, u64>,
    pub types: BTreeMap<u64, u64>,
    pub materials: BTreeMap<u64, u64>,
    pub definitions: BTreeMap<u64, Vec<u64>>,
    pub classifications: BTreeMap<u64, Vec<u64>>,
}

/// 📖️ A document with its id lookup and relationship index.
pub struct Doc<'a> {
    pub document: &'a Part21Document,
    by_id: BTreeMap<u64, &'a Part21Instance>,
    pub index: Index,
}

/// 🔢️ A real argument, `None` when unset or not numeric.
pub fn real(args: &[Part21Value], at: usize) -> Option<f64> {
    args.get(at).and_then(Part21Value::as_real)
}

/// 🔤️ A string argument, empty when unset.
pub fn text(args: &[Part21Value], at: usize) -> String {
    args.get(at).and_then(Part21Value::as_str).unwrap_or_default().to_string()
}

/// 🔤️ A string argument, `None` when unset or empty.
pub fn opt_text(args: &[Part21Value], at: usize) -> Option<String> {
    Some(text(args, at)).filter(|value| !value.is_empty())
}

/// 🔢️ The reals of a list argument.
pub fn reals(args: &[Part21Value], at: usize) -> Vec<f64> {
    args.get(at).and_then(Part21Value::as_list).map(|items| items.iter().filter_map(Part21Value::as_real).collect()).unwrap_or_default()
}

/// 🔗️ The reference ids of a list argument.
pub fn refs(args: &[Part21Value], at: usize) -> Vec<u64> {
    args.get(at).and_then(Part21Value::as_list).map(|items| items.iter().filter_map(Part21Value::as_ref_id).collect()).unwrap_or_default()
}

fn pad3(values: &[f64]) -> [f64; 3] {
    [values.first().copied().unwrap_or(0.0), values.get(1).copied().unwrap_or(0.0), values.get(2).copied().unwrap_or(0.0)]
}

impl<'a> Doc<'a> {
    /// 🌱️ Indexes `document`.
    pub fn new(document: &'a Part21Document) -> Self {
        let by_id: BTreeMap<u64, &Part21Instance> = document.instances.iter().map(|instance| (instance.id, instance)).collect();
        let mut doc = Self { document, by_id, index: Index::default() };
        doc.build_index();
        doc
    }

    fn build_index(&mut self) {
        let mut index = Index::default();
        for instance in &self.document.instances {
            for (name, args) in &instance.entities {
                let single = |at: usize| args.get(at).and_then(Part21Value::as_ref_id);
                match name.to_ascii_uppercase().as_str() {
                    "IFCRELCONTAINEDINSPATIALSTRUCTURE" => {
                        if let Some(structure) = single(5) {
                            refs(args, 4).into_iter().for_each(|element| {
                                index.contained.insert(element, structure);
                            });
                        }
                    }
                    "IFCRELAGGREGATES" => {
                        if let Some(whole) = single(4) {
                            let parts = refs(args, 5);
                            parts.iter().for_each(|part| {
                                index.whole.insert(*part, whole);
                            });
                            index.parts.entry(whole).or_default().extend(parts);
                        }
                    }
                    "IFCRELVOIDSELEMENT" => {
                        if let (Some(element), Some(opening)) = (single(4), single(5)) {
                            index.voids.entry(element).or_default().push(opening);
                        }
                    }
                    "IFCRELFILLSELEMENT" => {
                        if let (Some(opening), Some(filler)) = (single(4), single(5)) {
                            index.filling.insert(opening, filler);
                        }
                    }
                    "IFCRELDEFINESBYTYPE" => {
                        if let Some(kind) = single(5) {
                            refs(args, 4).into_iter().for_each(|object| {
                                index.types.insert(object, kind);
                            });
                        }
                    }
                    "IFCRELASSOCIATESMATERIAL" => {
                        if let Some(material) = single(5) {
                            refs(args, 4).into_iter().for_each(|object| {
                                index.materials.insert(object, material);
                            });
                        }
                    }
                    "IFCRELDEFINESBYPROPERTIES" => {
                        if let Some(definition) = single(5) {
                            refs(args, 4).into_iter().for_each(|object| index.definitions.entry(object).or_default().push(definition));
                        }
                    }
                    "IFCRELASSOCIATESCLASSIFICATION" => {
                        if let Some(reference) = single(5) {
                            refs(args, 4).into_iter().for_each(|object| index.classifications.entry(object).or_default().push(reference));
                        }
                    }
                    _ => {}
                }
            }
        }
        self.index = index;
    }

    /// 🔎️ The instance with `id`.
    pub fn get(&self, id: u64) -> Option<&'a Part21Instance> {
        self.by_id.get(&id).copied()
    }

    /// 🔎️ The arguments of the instance with `id` when it is of `entity`.
    pub fn args(&self, id: u64, entity: &str) -> Option<&'a Args> {
        self.get(id).and_then(|instance| instance.entity(entity))
    }

    /// 🔗️ The instance a reference argument points at.
    pub fn follow(&self, value: &Part21Value) -> Option<&'a Part21Instance> {
        value.as_ref_id().and_then(|id| self.get(id))
    }

    /// 🔗️ The arguments of the `entity` instance a reference argument points at.
    pub fn follow_args(&self, value: &Part21Value, entity: &str) -> Option<&'a Args> {
        self.follow(value).and_then(|instance| instance.entity(entity))
    }

    /// 🧩️ Every instance of an entity type with its arguments, in file order.
    pub fn rows(&self, entity: &'static str) -> Vec<(&'a Part21Instance, &'a Args)> {
        self.document.instances.iter().filter_map(|instance| instance.entity(entity).map(|args| (instance, args))).collect()
    }

    /// 📍️ An `IfcCartesianPoint` as a 3D point (z is 0 for planar points).
    pub fn point(&self, value: &Part21Value) -> [f64; 3] {
        pad3(&self.follow_args(value, "IFCCARTESIANPOINT").map(|args| reals(args, 0)).unwrap_or_default())
    }

    /// 🧭️ An `IfcDirection` as a 3D vector.
    pub fn direction(&self, value: &Part21Value) -> Option<[f64; 3]> {
        self.follow_args(value, "IFCDIRECTION").map(|args| pad3(&reals(args, 0)))
    }

    /// 📐️ The rigid placement of an `IfcAxis2Placement3D` or `IfcAxis2Placement2D`.
    pub fn axis_placement(&self, value: &Part21Value) -> Rigid {
        if let Some(args) = self.follow_args(value, "IFCAXIS2PLACEMENT3D") {
            return Rigid::from_axes(self.point(&args[0]), args.get(1).and_then(|axis| self.direction(axis)), args.get(2).and_then(|reference| self.direction(reference)));
        }
        if let Some(args) = self.follow_args(value, "IFCAXIS2PLACEMENT2D") {
            return Rigid::from_axes(self.point(&args[0]), None, args.get(1).and_then(|reference| self.direction(reference)));
        }
        Rigid::IDENTITY
    }

    /// 🌍️ The world placement of an `IfcLocalPlacement` chain.
    pub fn world(&self, value: &Part21Value) -> Rigid {
        match self.follow_args(value, "IFCLOCALPLACEMENT") {
            Some(args) => {
                let relative = self.axis_placement(&args[1]);
                if args[0].is_unset() {
                    relative
                } else {
                    relative.transformed_by(&self.world(&args[0]))
                }
            }
            None => Rigid::IDENTITY,
        }
    }

    fn plane_points(&self, value: &Part21Value) -> Vec<[f64; 2]> {
        value.as_list().map(|items| items.iter().map(|item| self.point(item)).map(|point| [point[0], point[1]]).collect()).unwrap_or_default()
    }

    /// 〰️ The straight or circular edge a curve is made of: a two-point polyline or a trimmed circle (cartesian trims).
    pub fn edge(&self, value: &Part21Value) -> Option<Edge> {
        if let Some(args) = self.follow_args(value, "IFCPOLYLINE") {
            let points = self.plane_points(&args[0]);
            return (points.len() == 2).then(|| Edge { start: points[0], end: points[1], bulge: 0.0 });
        }
        let args = self.follow_args(value, "IFCTRIMMEDCURVE")?;
        let circle = self.follow_args(&args[0], "IFCCIRCLE")?;
        let centre = self.axis_placement(&circle[0]).origin;
        let trim = |at: usize| args[at].as_list().and_then(|items| items.first()).map(|item| self.point(item)).map(|point| [point[0], point[1]]);
        let (start, end) = (trim(1)?, trim(2)?);
        Some(Edge { start, end, bulge: bulge_of([centre[0], centre[1]], start, end, args[3].as_enum() == Some("T")) })
    }

    /// 🔷️ The closed bulged loop of a polyline or composite curve; the closing point of a polyline is dropped.
    pub fn closed_loop(&self, value: &Part21Value) -> Option<Loop> {
        if let Some(args) = self.follow_args(value, "IFCPOLYLINE") {
            let mut points = self.plane_points(&args[0]);
            if points.len() > 1 && points.first() == points.last() {
                points.pop();
            }
            return (points.len() >= 3).then(|| points.into_iter().map(|point| (point, 0.0)).collect());
        }
        let composite = self.follow_args(value, "IFCCOMPOSITECURVE")?;
        let edges: Vec<Edge> = composite[0].as_list()?.iter().filter_map(|segment| self.follow_args(segment, "IFCCOMPOSITECURVESEGMENT")).filter_map(|segment| self.edge(&segment[2])).collect();
        (edges.len() >= 3 || edges.iter().any(|edge| edge.bulge != 0.0)).then(|| edges.into_iter().map(|edge| (edge.start, edge.bulge)).collect())
    }

    /// ▭️ The section of a profile definition.
    pub fn section(&self, value: &Part21Value) -> Option<Section> {
        let instance = self.follow(value)?;
        if let Some(args) = instance.entity("IFCRECTANGLEPROFILEDEF") {
            let centre = self.axis_placement(&args[2]).origin;
            return Some(Section::Rectangle { centre: [centre[0], centre[1]], width: real(args, 3)?, depth: real(args, 4)? });
        }
        if let Some(args) = instance.entity("IFCCIRCLEPROFILEDEF") {
            return Some(Section::Circle { diameter: 2.0 * real(args, 3)? });
        }
        if let Some(args) = instance.entity("IFCISHAPEPROFILEDEF") {
            return Some(Section::IShape { width: real(args, 3)?, depth: real(args, 4)?, web: real(args, 5)?, flange: real(args, 6)? });
        }
        if let Some(args) = instance.entity("IFCARBITRARYCLOSEDPROFILEDEF") {
            return Some(Section::Outline { outer: self.closed_loop(&args[2])?, holes: Vec::new() });
        }
        let args = instance.entity("IFCARBITRARYPROFILEDEFWITHVOIDS")?;
        let holes = args[3].as_list()?.iter().filter_map(|curve| self.closed_loop(curve)).collect();
        Some(Section::Outline { outer: self.closed_loop(&args[2])?, holes })
    }

    fn representation_items(&self, product: &[Part21Value], identifier: &str) -> Vec<&'a Part21Value> {
        let Some(shape) = product.get(6).and_then(|value| self.follow_args(value, "IFCPRODUCTDEFINITIONSHAPE")) else { return Vec::new() };
        let representations = shape[2].as_list().unwrap_or_default();
        representations
            .iter()
            .filter_map(|value| self.follow_args(value, "IFCSHAPEREPRESENTATION"))
            .filter(|args| text(args, 1) == identifier)
            .flat_map(|args| args.get(3).and_then(Part21Value::as_list).unwrap_or_default().iter())
            .collect()
    }

    /// 〰️ The `Axis` representation of a product as one edge.
    pub fn axis_edge(&self, product: &[Part21Value]) -> Option<Edge> {
        self.representation_items(product, "Axis").into_iter().find_map(|item| self.edge(item))
    }

    /// 👣️ The `FootPrint` representation of a product as a closed loop.
    pub fn footprint(&self, product: &[Part21Value]) -> Option<Loop> {
        self.representation_items(product, "FootPrint").into_iter().find_map(|item| self.closed_loop(item))
    }

    /// 🧊️ The first swept `Body` solid of a product.
    pub fn body(&self, product: &[Part21Value]) -> Option<Extrusion> {
        self.representation_items(product, "Body").into_iter().find_map(|item| {
            let args = self.follow_args(item, "IFCEXTRUDEDAREASOLID")?;
            Some(Extrusion { section: self.section(&args[0])?, position: self.axis_placement(&args[1]), direction: self.direction(&args[2])?, depth: real(args, 3)? })
        })
    }

    /// 🔖️ The `Tag` (elements) or `ObjectType` (spatial elements) that carries the semio id.
    pub fn identity(args: &[Part21Value], entity: &str) -> Option<String> {
        let spatial = matches!(entity, "IFCPROJECT" | "IFCSITE" | "IFCBUILDING" | "IFCBUILDINGSTOREY" | "IFCSPACE" | "IFCGRID");
        opt_text(args, if spatial { 4 } else { 7 })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
