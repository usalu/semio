//! ✂️ Writer play app commands command — `text-splice`: the typing verb of a host that declares splice typing.

use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

/// ✂️ One typed run as its host saw it (`semio.ui.scene.text-splice.v1`), the host's run sequence `seq`, and the selection the
/// host shows after the run (`anchor`/`caret`, UTF-8 byte offsets of the editor session).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

/// ⌨️ Keystroke-granular splices coalesce under the typing key `EditText` used, so a typing burst stays one undo step; the
/// retained command job also records `seq` in the window's selection (see `WriterCommandJob::emit`).
pub fn handle(payload: &TextSplice, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Ok(Emit::amend(vec![crate::op::splice_text(payload.splice())], "writer-text-edit"))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
