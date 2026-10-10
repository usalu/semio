//! 🧬️ `families`: the evaluated parametric families of the model, keyed by family id. A family is authored data (`families`, `family_parameters`, `family_solids`: formulas as canonical text); everything
//! evaluated is inferred here and never stored: the resolved parameter values, the solids as meshes, the profile outline of a profile family and the issues (parse, kind, cycle, unknown name, negative
//! dimension, division by zero).
//!
//! The pure function is [`family_of`] (the snapshot, a family id and the override formulas of a placed instance): the expression crate resolves the parameters in dependency order
//! ([`parameters`]), the framework geometry builds the solids ([`solids`]). In the model graph a family is one node without parents; the solids of the columns, beams, mullions and railings whose
//! profile is `Profile::Family` have that node as a parent and read its outline through [`FamilyProfiles`]. See `r11-decision-families.md` and `r12-exec-w2-f1-expression.md` in the BIM-PLUGIN ticket.

use super::super::element_solids::{dep_object, dep_value, SolidBounds};
use crate::{FamilyCategory, ModelSnapshot, ParameterKind, Profile, Railing, Vertex};
use semio_framework_value::DslValue;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[path = "⚠️issues/🦀️.rs"]
pub mod issues;
#[path = "🧮️parameters/🦀️.rs"]
pub mod parameters;
#[path = "🩺️findings/🦀️.rs"]
pub mod findings;
#[path = "📊️metrics/🦀️.rs"]
pub mod metrics;
#[path = "🧊️solids/🦀️.rs"]
pub mod solids;

pub use issues::{FamilyIssue, FamilyIssueCode, IssueOwner};

/// 🗺️ The snapshot collections a family reads.
pub const READS: &[&str] = &["families", "family_parameters", "family_solids", "materials"];

//#region 🔖️Values
/// 🔢️ An evaluated parameter value in SI base units: metres, radians.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum ParameterValue {
    Number { value: f64 },
    Length { value: f64 },
    Angle { value: f64 },
    Boolean { value: bool },
    Text { value: String },
}

/// 🔢️ One parameter after evaluation: its kind, the effective formula (an override replaces the authored one) and the value; absent when the formula failed (see the issues).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ResolvedParameter {
    pub kind: ParameterKind,
    pub formula: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ParameterValue>,
}

/// 🧊️ One evaluated solid: flat counter-clockwise outward triangles in the family frame (metres), the measures, the evaluated material id and whether the visibility formula holds.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FamilySolidMesh {
    pub name: String,
    pub material: String,
    pub visible: bool,
    pub positions: Vec<f64>,
    pub normals: Vec<f64>,
    pub indices: Vec<u32>,
    pub bounds: SolidBounds,
    pub volume: f64,
    pub area: f64,
}

/// 🧬️ Everything inferred about one family (or one placed instance of it).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FamilyValue {
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<FamilyCategory>,
    pub order: Vec<String>,
    pub parameters: BTreeMap<String, ResolvedParameter>,
    pub solids: BTreeMap<String, FamilySolidMesh>,
    pub outline: Vec<Vertex>,
    pub issues: Vec<FamilyIssue>,
}

impl FamilyValue {
    /// 🔢️ The evaluated parameter `name`, if it resolved.
    pub fn value(&self, name: &str) -> Option<&ParameterValue> {
        self.parameters.get(name).and_then(|parameter| parameter.value.as_ref())
    }

    /// 🧊️ The visible solids.
    pub fn visible(&self) -> impl Iterator<Item = (&String, &FamilySolidMesh)> {
        self.solids.iter().filter(|(_, solid)| solid.visible)
    }

    /// 📦️ The summed volume of the visible solids.
    pub fn volume(&self) -> f64 {
        self.visible().map(|(_, solid)| solid.volume).sum()
    }
}
//#endregion 🔖️Values

//#region 🔖️Evaluation
/// 🔷️ The snapshot form of a geometry loop.
fn marked(outline: &[semio_framework_geometry::loops::Vertex]) -> Vec<Vertex> {
    outline.iter().map(|vertex| Vertex { point: crate::Point2 { x: vertex.point.x, y: vertex.point.y }, bulge: vertex.bulge }).collect()
}

/// ▶️ The inferred value of family `id` with the override formulas of an instance (empty for the family itself). A family that does not exist is the default (empty) value.
pub fn family_of(snapshot: &ModelSnapshot, id: &str, overrides: &BTreeMap<String, String>) -> FamilyValue {
    let Some(family) = snapshot.families.get(id) else { return FamilyValue::default() };
    let resolution = parameters::resolve(snapshot, id, overrides);
    let mut value = FamilyValue { name: family.name.clone(), category: Some(family.category), order: resolution.order.clone(), parameters: resolution.parameters.clone(), issues: resolution.issues.clone(), ..FamilyValue::default() };
    let mut sections: Vec<(Vec<semio_framework_geometry::loops::Vertex>, [f64; 3])> = Vec::new();
    for (solid_id, solid) in snapshot.family_solids.iter().filter(|(_, solid)| solid.family == id) {
        let (evaluated, found) = solids::evaluate_solid(solid_id, solid, &resolution.env, &resolution.names, snapshot);
        value.issues.extend(found);
        if evaluated.mesh.visible {
            if let Some(section) = evaluated.section {
                sections.push((section, evaluated.offset));
            }
        }
        value.solids.insert(solid_id.clone(), evaluated.mesh);
    }
    if family.category == FamilyCategory::Profile {
        match sections.into_iter().next() {
            Some((section, offset)) => {
                let moved: Vec<semio_framework_geometry::loops::Vertex> = section.iter().map(|vertex| semio_framework_geometry::loops::Vertex::new(semio_framework_geometry::Point::new(vertex.point.x + offset[0], vertex.point.y + offset[1]), vertex.bulge)).collect();
                value.outline = marked(&moved);
            }
            None => value.issues.push(FamilyIssue::new(FamilyIssueCode::Outline, IssueOwner::Family, "", "", "a profile family needs a visible extrusion whose section is its outline", Vec::new())),
        }
    }
    value
}

/// 🔑️ Everything `family_of` reads of the snapshot for family `id`: the family record, its parameters and solids, and the project materials a material formula may name.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let materials: Vec<String> = snapshot.materials.keys().cloned().collect();
    dep_object([
        ("family", dep_value(&snapshot.families.get(id).cloned())),
        ("parameters", DslValue::object(snapshot.family_parameters.iter().filter(|(_, row)| row.family == id).map(|(key, row)| (key.clone(), dep_value(row))))),
        ("solids", DslValue::object(snapshot.family_solids.iter().filter(|(_, row)| row.family == id).map(|(key, row)| (key.clone(), dep_value(row))))),
        ("materials", dep_value(&materials)),
    ])
}
//#endregion 🔖️Evaluation

//#region 🔖️Profiles
/// ▭️ The outlines of the profile families, by family id: what a `Profile::Family` stands for in a solid, a quantity or a plan.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FamilyProfiles<'a> {
    outlines: BTreeMap<&'a str, &'a [Vertex]>,
}

impl<'a> FamilyProfiles<'a> {
    /// 🆕️ No family known: every `Profile::Family` resolves to an empty outline.
    pub fn new() -> Self {
        Self::default()
    }

    /// ➕️ The outline of family `id`.
    pub fn with(mut self, id: &'a str, outline: &'a [Vertex]) -> Self {
        self.outlines.insert(id, outline);
        self
    }

    /// 🔎️ The outline of family `id`, empty when the family is unknown or has no outline.
    pub fn outline(&self, id: &str) -> &'a [Vertex] {
        self.outlines.get(id).copied().unwrap_or(&[])
    }

    /// 🛤️ The railing with every family profile (rail, post, baluster) replaced by its outline; the railing itself when it names no family.
    pub fn resolve_railing<'r>(&self, railing: &'r Railing) -> Cow<'r, Railing> {
        let names = |profile: &Profile| family_of_profile(profile).is_some();
        if !(names(&railing.profile) || names(&railing.post_profile) || railing.baluster.as_ref().is_some_and(|baluster| names(&baluster.profile))) {
            return Cow::Borrowed(railing);
        }
        let mut resolved = railing.clone();
        resolved.profile = self.resolve(&railing.profile).into_owned();
        resolved.post_profile = self.resolve(&railing.post_profile).into_owned();
        if let Some(baluster) = resolved.baluster.as_mut() {
            baluster.profile = self.resolve(&baluster.profile).into_owned();
        }
        Cow::Owned(resolved)
    }

    /// 🔁️ The profile itself, or for `Profile::Family` the custom profile of the outline of that family.
    pub fn resolve<'p>(&self, profile: &'p Profile) -> Cow<'p, Profile> {
        match profile {
            Profile::Family { family } => Cow::Owned(Profile::Custom { outline: self.outline(family).to_vec() }),
            other => Cow::Borrowed(other),
        }
    }
}

/// 🔗️ The family a profile stands for, if it is a family profile.
pub fn family_of_profile(profile: &Profile) -> Option<&str> {
    match profile {
        Profile::Family { family } => Some(family),
        _ => None,
    }
}
//#endregion 🔖️Profiles

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
