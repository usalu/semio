//! ✂️ Direct Writer mutation — `SpliceText` replaces one range of the authored text body as its author saw it.
use crate::schema::mutations::WriterMutation;
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_plugin::TextSplice;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ One range-text operation on `WriterSnapshot::text` (`semio.ui.scene.text-splice.v1`): at scalar `start` of its author's
/// text, `deleted` became `insert`, with `before`/`after` the context the author saw. Applying it to any other text relocates it
/// by that context, so two humans typing at once keep both runs; replaying it after the store rewinds to a fork point is the
/// rebase. Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf, dsl::ToValue, dsl::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "splice-text")]
pub struct SpliceText {
    pub start: u32,
    pub deleted: String,
    pub insert: String,
    pub before: String,
    pub after: String,
}

impl SpliceText {
    /// ✂️ The language-neutral splice this payload carries.
    pub fn splice(&self) -> TextSplice {
        TextSplice { start: self.start, deleted: self.deleted.clone(), insert: self.insert.clone(), before: self.before.clone(), after: self.after.clone() }
    }
}

/// 🏗️ Builder — wraps a splice in its dispatch variant.
pub fn splice_text(splice: TextSplice) -> WriterMutation {
    WriterMutation::SpliceText(SpliceText { start: splice.start, deleted: splice.deleted, insert: splice.insert, before: splice.before, after: splice.after })
}

impl MutationKind<WriterSnapshot, WriterMutation> for SpliceText {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "text-range", kind: "splice-text", record: "SplicedText" };

    fn diff(&self, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &WriterSnapshot) -> Vec<WriterMutation> {
        super::inverse::inverse(self, base)
    }

    /// 🎯️ The writer's authored body, like `EditText`: its composed `document` child derives from it.
    fn target(&self) -> Vec<String> {
        vec!["text".to_string()]
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Edit a text range", "Textbereich bearbeiten")
    }
}
//#endregion 🔖️Mutation
