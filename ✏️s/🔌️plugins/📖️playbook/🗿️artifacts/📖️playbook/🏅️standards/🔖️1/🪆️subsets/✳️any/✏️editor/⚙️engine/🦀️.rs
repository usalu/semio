//! ⚙️ Playbook app engine — the app's own media-io surface and wire payload shapes. Relocated from
//! the deleted artifact-tree `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
//! an artifact is a schema + io, never an engine. `playbook_io()` returns `AppIo` and
//! `PlaybookChapterPayload` is this app's own wire-decode shape for the `chapters:in` port — both
//! app-owned per the region → destination map's rule 4. `default_block` (pure, no app type) stayed at
//! `🧬️schema`; `empty_playbook_snapshot`/`flatten_playbook_blocks` were re-export-only in the old
//! engine and now resolve straight to their real home, the artifact root (`crate::artifacts::playbook`).

use crate::artifacts::playbook::PLAYBOOK_DOCUMENT_SCHEMA;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports plus one
/// extra input, `chapters:in` (Text×Document, kind `text.document`, `Many` — fans in from several
/// upstream `writer` nodes' `"text:out"`).
pub fn playbook_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Text, form: semio_framework_plugin::MediaForm::Document },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "chapters:in".into(),
            label: "Chapters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Text, form: semio_framework_plugin::MediaForm::Document },
            kind_id: Some("text.document".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "text.playbook".into(), name: "Playbook".into(), dimension: "text".into(), component_kind: "playbook".into() },
    }
}

/// 📥️ Mirror of `writer_engine::WriterChapterPayload` — the JSON shape `"chapters:in"` decodes (a
/// writer document's text as one "chapter"). Kept structurally identical rather than shared: the two
/// apps are on opposite sides of the wire and this crate must not depend on the writer plugin.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct PlaybookChapterPayload {
    pub id: String,
    pub title: String,
    pub text: String,
    #[value(default)]
    pub language_id: String,
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn playbook_io_declares_the_extra_chapters_in_port() {
        let io = playbook_io();
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let chapters_in = ports.iter().find(|port| port.id == "chapters:in").expect("chapters:in port declared");
        assert_eq!(chapters_in.kind_id.as_deref(), Some("text.document"));
        assert_eq!(chapters_in.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
        assert_eq!(chapters_in.direction, semio_framework_plugin::MediaPortDirection::In);
    }
}
//#endregion 🧪️Tests
