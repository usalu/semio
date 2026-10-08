//! ✂️ Writer play app commands command — `text-splice`: the typing verb of a host that declares splice typing.

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

/// ✂️ One typed run as its host saw it (`semio.ui.scene.text-splice.v1`), the host's run sequence `seq`, and the selection the
/// host shows after the run (`anchor`/`caret`, UTF-8 byte offsets of the editor session).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "text-splice")]
pub struct TextSplice {
    pub start: u32,
    pub deleted: String,
    pub insert: String,
    pub before: String,
    pub after: String,
    pub seq: u64,
    pub anchor: usize,
    pub caret: usize,
}

impl TextSplice {
    /// ✂️ The language-neutral splice of this run.
    pub fn splice(&self) -> semio_framework_plugin::TextSplice {
        semio_framework_plugin::TextSplice { start: self.start, deleted: self.deleted.clone(), insert: self.insert.clone(), before: self.before.clone(), after: self.after.clone() }
    }
}

/// ⌨️ One typed run as one `splice-text`. A live typing delivery (`typing` argument) composes into its window's typing run
/// (`WriterPlayApp::typing_fold`), which commits as ONE net splice (design §13.2); the retained command job also records `seq`
/// in the window's selection (see `WriterCommandJob::emit`).
pub fn handle(payload: &TextSplice, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::schema::mutations::splice_text(payload.splice())]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
