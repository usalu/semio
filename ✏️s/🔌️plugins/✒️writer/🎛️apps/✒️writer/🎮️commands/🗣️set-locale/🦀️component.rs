//! 🗣️ 🗣️ Writer play app commands command — `set-locale`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation};
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(Emit::config(vec![WriterConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
        use crate::apps::writer::testkit::{dispatch, new_app, render};
    use crate::apps::writer::{WriterCommand, WRITER_PLAY_BODY_INSPECTION};

    #[test]
    fn writer_labels_resolve_native_english_and_german() {
        let mut app = new_app();
        let english = render(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(english.contains("\"Document\"") && english.contains("\"Camera\""), "english labels: {english}");
        dispatch(&mut app, WriterCommand::SetLocale(SetLocale { value: "de-DE".into() }));
        let german = render(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(german.contains("Dokument") && german.contains("Kamera"), "german labels: {german}");
    }
}
//#endregion 🧪️Tests
