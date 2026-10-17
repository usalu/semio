//! 🔗️ Direct Writer mutation — `ChangeUri` sets the document's addressable location.
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗️ Sets `WriterSnapshot::uri` to `new_uri`. Diff/inverse delegate to the sibling
/// `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-uri")]
pub struct ChangeUri {
    pub new_uri: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_uri<Op>(new_uri: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_uri::ChangeUri>,
{
    Op::from(ChangeUri { new_uri })
}

impl<Op> MutationKind<WriterSnapshot, Op> for ChangeUri
where
    Op: protocol::Mutation<WriterSnapshot, Diff = WriterDiff> + From<ChangeUri>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "uri", kind: "change-uri", record: "ChangedUri" };

    fn diff(&self, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &WriterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    /// 🎯️ The writer's source URI. Concurrent writers of the same field conflict; writers of different fields never do.
    fn target(&self) -> Vec<String> {
        vec!["uri".to_string()]
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change document URI to \"{}\"", self.new_uri), &format!("Dokument-URI auf \"{}\" ändern", self.new_uri))
    }
}
//#endregion 🔖️Mutation
