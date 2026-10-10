//! 🧮️ `set-stair` payload. Sparsely changes a stair: start, direction, width, flight, top constraint, maximum riser, minimum tread, stringer, nosing, tread thickness, riser boards, landing depth, name.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2, RiserKind, StairFlight, StairStringer, TopConstraint};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStair {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flight: Option<StairFlight>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_riser: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_tread: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stringer: Option<StairStringer>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nosing: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tread_thickness: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub riser: Option<RiserKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub landing_depth: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetStair {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "stair", kind: "set-stair", record: "SetStair" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change stair \"{}\"", self.id), &format!("Treppe \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
