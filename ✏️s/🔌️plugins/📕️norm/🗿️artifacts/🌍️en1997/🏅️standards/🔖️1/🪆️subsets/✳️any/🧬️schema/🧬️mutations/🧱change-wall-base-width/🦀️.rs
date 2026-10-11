//! 🧹 `change-wall-base-width` payload.

use crate::diff::En1997Diff;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeWallBaseWidth {
    pub id: String,
    pub new_base_width: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeWallBaseWidth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change", entity: "wall-base-width", kind: "change-wall-base-width", record: "ChangedWallBaseWidth",
    };
    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1997Snapshot) -> Result<Vec<En1997Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change retaining wall base width", "Sohlbreite der Stützwand ändern")
    }
}
