//! 📍️ `set-vertex-positions` — writes absolute positions onto the named vertices of an object's managed mesh: the exact, accumulation-free
//! undo of every selection motion (`move-selection`, `rotate-selection`, `scale-selection`) and the concrete kind of a kernel
//! mesh edit that only repositions vertices; `channels` carries the absolute content of the Normal attribute channels the same edit rewrote. The normals are recomputed and the mesh handle re-derived by the central applier.

use crate::diff::LowpolyVertexPosition;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetVertexPositions {
    pub object_id: String,
    pub positions: Vec<LowpolyVertexPosition>,
    pub channels: Vec<crate::LowpolyMeshAttribute>,
}

impl SetVertexPositions {
    /// 🧪️ The payload's own invariant, independent of any document: every vertex named once and every position finite.
    pub fn invariant_violation(&self) -> Option<String> {
        let ids: Vec<u32> = self.positions.iter().map(|row| row.vertex).collect();
        let mut names = std::collections::BTreeSet::new();
        crate::mutations::lowpoly_selection_vertex_violation(&ids)
            .or_else(|| self.positions.iter().any(|row| row.position.iter().any(|value| !value.is_finite())).then(|| "Every position must be finite.".into()))
            .or_else(|| self.channels.iter().find(|channel| channel.semantic != crate::LowpolyMeshAttributeSemantic::Normal || !names.insert(channel.name.as_str())).map(|channel| format!("Channel \"{}\" must be a Normal channel named once.", channel.name)))
    }
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for SetVertexPositions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "vertex-positions", kind: "set-vertex-positions", record: "SetVertexPositionsDone" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let count = self.positions.len();
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Position {count} vertices of \"{}\"", self.object_id), &format!("{count} Eckpunkte von \"{}\" positionieren", self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone()]
    }
}
//#endregion 🔖️Payload

//#region 🧪️Laws
#[cfg(test)]
pub use crate::mutations::laws;
//#endregion 🧪️Laws
