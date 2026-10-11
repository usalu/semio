//! 🎯️ `move-points` — one point-cloud drag as intent (the twin of `move-nodes`, design §13.3): the canvas offset every
//! addressed point moves by from its BASE position, so editing the drag in history replays it on any base.

use crate::standards::v1::subsets::geometry::schema::mutations::set_point_positions::equation_point_targets;
use crate::{EquationMutation, EquationSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MovePoints {
    pub indices: Vec<usize>,
    pub dx: f64,
    pub dy: f64,
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for MovePoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "points", kind: "move-points", record: "MovedPoints" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de)] = [self.dx, self.dy].map(crate::standards::v1::subsets::graph::schema::mutations::set_node_positions::equation_label_number);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {} point(s) by ({x_en}, {y_en})", self.indices.len()), &format!("{} Punkt(e) um ({x_de}; {y_de}) verschieben", self.indices.len()))
    }
    fn target(&self) -> Vec<String> {
        equation_point_targets(&self.indices)
    }
}
//#endregion 🔖️Payload
