//! 🦴️ Authored structural boundary conditions and SI loading; geometry and connectivity are inferred.
use super::{ModelSnapshot, Point3};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar)]
pub enum StructuralLocation { Point { station: f64 }, Line { start: f64, end: f64 }, Area }
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct Restraints { pub x: bool, pub y: bool, pub z: bool, pub rx: bool, pub ry: bool, pub rz: bool }
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct StructuralSupport { pub name: String, pub member: String, pub location: StructuralLocation, pub offset: Point3, pub restraints: Restraints }
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct LoadCase { pub name: String, pub category: String, pub factor: f64 }
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct StructuralLoad { pub name: String, pub load_case: String, pub member: String, pub location: StructuralLocation, pub force: Point3, pub moment: Point3 }
/// 🎯️ Whether an authored element participates in structural analysis.
pub fn member_exists(model: &ModelSnapshot, id: &str) -> bool { model.beams.contains_key(id) || model.columns.contains_key(id) || model.walls.contains_key(id) || model.slabs.contains_key(id) }
fn finite(p: &Point3) -> bool { p.x.is_finite() && p.y.is_finite() && p.z.is_finite() }
/// 📍️ A location must address a member of matching dimensionality and stay inside its normalized extent.
pub fn location_problem(model: &ModelSnapshot, member: &str, location: &StructuralLocation) -> Option<&'static str> {
 if !member_exists(model, member) { return Some("Structural member does not exist."); }
 match location { StructuralLocation::Point { station } if !station.is_finite() || !(0.0..=1.0).contains(station) => Some("Point station must lie between zero and one."), StructuralLocation::Line { start, end } if !start.is_finite() || !end.is_finite() || *start < 0.0 || *end > 1.0 || start >= end => Some("Line range must be increasing inside zero and one."), StructuralLocation::Area if !model.slabs.contains_key(member) && !model.walls.contains_key(member) => Some("Area loading requires a wall or slab."), _ => None }
}
/// 🧱️ Validates only authored support parameters.
pub fn support_problem(model: &ModelSnapshot, record: &StructuralSupport) -> Option<&'static str> {
 if record.name.trim().is_empty() { return Some("Support name must not be blank."); }
 if !finite(&record.offset) { return Some("Support offset must be finite."); }
 if !(record.restraints.x || record.restraints.y || record.restraints.z || record.restraints.rx || record.restraints.ry || record.restraints.rz) { return Some("Support must restrain at least one degree of freedom."); }
 location_problem(model, &record.member, &record.location)
}
/// ⚖️ Validates only authored case parameters.
pub fn load_case_problem(_: &ModelSnapshot, record: &LoadCase) -> Option<&'static str> { if record.name.trim().is_empty() || record.category.trim().is_empty() { Some("Load case name and category must not be blank.") } else if !record.factor.is_finite() || record.factor == 0.0 { Some("Load factor must be finite and nonzero.") } else { None } }
/// ⬇️ Validates only authored loading parameters.
pub fn load_problem(model: &ModelSnapshot, record: &StructuralLoad) -> Option<&'static str> { if !model.load_cases.contains_key(&record.load_case) { return Some("Load case does not exist."); } if record.name.trim().is_empty() || !finite(&record.force) || !finite(&record.moment) { return Some("Load name and vectors must be valid."); } location_problem(model, &record.member, &record.location) }
