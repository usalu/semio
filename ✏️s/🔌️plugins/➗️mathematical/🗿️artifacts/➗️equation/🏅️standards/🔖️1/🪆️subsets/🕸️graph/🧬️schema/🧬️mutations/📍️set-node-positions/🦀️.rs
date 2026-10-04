//! 📍️ `set-node-positions` — absolute canvas positions of a set of graph nodes in ONE row: the exact undo of a
//! `move-nodes` drag (every moved node back at its base position) and of itself.

use crate::{EquationMutation, EquationSnapshot};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Payload
/// 📌️ One graph node's absolute canvas position.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive)]
#[value(rename_all = "camelCase")]
pub struct EquationNodePosition {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetNodePositions {
    pub positions: Vec<EquationNodePosition>,
}

impl SetNodePositions {
    /// 🆔️ The positioned node ids, in payload order.
    pub fn ids(&self) -> Vec<String> {
        self.positions.iter().map(|position| position.id.clone()).collect()
    }
}

/// 🔢️ A label number in both languages: at most three decimals, a decimal comma in German.
pub fn equation_label_number(value: f64) -> (String, String) {
    let english = format!("{}", (value * 1_000.0).round() / 1_000.0);
    let german = english.replace('.', ",");
    (english, german)
}

/// 🧱️ The payload-intrinsic target law every multi-node leaf states in its schema: at least one node, each once.
pub fn equation_targets_invariant(targets: &[String]) -> Result<(), &'static str> {
    if targets.is_empty() || targets.iter().any(String::is_empty) {
        return Err("a multi-node leaf names at least one non-empty node");
    }
    if targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return Err("a multi-node leaf names each node once");
    }
    Ok(())
}

impl protocol::MutationKind<EquationSnapshot, EquationMutation> for SetNodePositions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node-positions", kind: "set-node-positions", record: "SetNodePositions" };

    fn diff(&self, base: &EquationSnapshot) -> protocol::MutationOutcome<<EquationMutation as protocol::Mutation<EquationSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the positions of {} node(s)", self.positions.len()), &format!("Positionen von {} Knoten setzen", self.positions.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids()
    }
}
//#endregion 🔖️Payload
