//! 🦎 `change-connections` mutation leaf.

use crate::diff::En1999Diff;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;
use crate::snapshot::AluminiumConnection;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeConnections {
    pub connections: Vec<AluminiumConnection>,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeConnections {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "connections",
        kind: "change-connections",
        record: "ChangedConnections",
    };

    fn diff(&self, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change connections", "Verbindungen ändern")
    }
}
