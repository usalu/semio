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

    /// 🏷️ What the run typed, quoted (at most [`SPLICE_LABEL_EXCERPT_SCALARS`] scalars, a line break as `↵`): "Type “…”",
    /// "Delete “…”" or "Replace “…” with “…”" — a typing run's history row reads its own text.
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (deleted, insert) = (splice_label_excerpt(&self.deleted), splice_label_excerpt(&self.insert));
        match (deleted.is_empty(), insert.is_empty()) {
            (true, false) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Type “{insert}”"), &format!("„{insert}“ tippen")),
            (false, true) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete “{deleted}”"), &format!("„{deleted}“ löschen")),
            (false, false) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace “{deleted}” with “{insert}”"), &format!("„{deleted}“ durch „{insert}“ ersetzen")),
            (true, true) => semio_framework_ui_locale::LocalizedLabel::native("Edit a text range", "Textbereich bearbeiten"),
        }
    }
}

/// ✂️ The most scalars of a run a history label quotes.
pub const SPLICE_LABEL_EXCERPT_SCALARS: usize = 24;

/// ✂️ A run as a label quotes it: line breaks as `↵`, cut after [`SPLICE_LABEL_EXCERPT_SCALARS`] scalars with `…`.
fn splice_label_excerpt(run: &str) -> String {
    let mut excerpt: String = run.chars().take(SPLICE_LABEL_EXCERPT_SCALARS).map(|scalar| if scalar == '\n' { '↵' } else { scalar }).collect();
    if run.chars().nth(SPLICE_LABEL_EXCERPT_SCALARS).is_some() {
        excerpt.push('…');
    }
    excerpt
}
//#endregion 🔖️Mutation
