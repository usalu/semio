//! ⚙️ Playbook-play app — headless compute (constitutional: engine).

use playbook::{PlaybookBlock, PLAYBOOK_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: playbook's real `DocumentApp::Config` — absorbs the former app-struct `RefCell<Vec<String>>`
/// selection state, plus `locale`, the one `ViewState` field the playbook UI actually reads
/// (`resolve_labels`/`is_de_locale`) — mirrors `writer_engine::WriterConfig`/`shooting_engine::ShootingConfig`'s
/// B1 shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "playbookcfg")]
#[dsl(layout = "lines")]
pub struct PlaybookConfig {
    /// 👁️ Selected step/block ids — was `PlaybookPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for PlaybookConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), locale: "en-US".into() }
    }
}

impl store::ConfigRecord for PlaybookConfig {}

/// @emoji 🧮️ Whole-record diff for `playbook_op::PlaybookConfigOperation` — mirrors
/// `writer_engine::WriterConfig`'s own `OperationDiff` impl (`apply` ignores `base` entirely).
impl protocol::OperationDiff<PlaybookConfig> for PlaybookConfig {
    fn apply(&self, _base: &PlaybookConfig) -> PlaybookConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports plus one
/// extra input, `chapters:in` (Text×Document, kind `text.document`, `Many` — fans in from several
/// upstream `writer` nodes' `"text:out"`). Playbook's own document kind is `text.playbook` (new — this
/// app declared no `ArtifactKindSpec` before the port recipe).
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
/// apps are on opposite sides of the wire and this crate must not depend on `writer_engine`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookChapterPayload {
    pub id: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub language_id: String,
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
/// 🧱️ A blank block of the requested kind — every optional field defaulted, ready to be edited.
pub fn default_block(id: String, kind: &str) -> PlaybookBlock {
    PlaybookBlock {
        id,
        label: kind.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playbook_config_dsl_round_trips_default_and_populated() {
        store::test_support::assert_config_round_trip(&PlaybookConfig::default());
        let populated = PlaybookConfig { selected_ids: vec!["step-1".into(), "block-1".into()], locale: "de-DE".into() };
        store::test_support::assert_config_round_trip(&populated);
    }

    #[test]
    fn playbook_config_pack_round_trips() {
        let config = PlaybookConfig { selected_ids: vec!["block-1".into()], locale: "de-DE".into() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <PlaybookConfig as store::DocumentPack>::decode_pack(&bytes).expect("decode playbook config pack");
        assert_eq!(decoded, config);
    }

    #[test]
    fn playbook_io_declares_the_extra_chapters_in_port() {
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
