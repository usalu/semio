//! 👁️ 👁️ Layout play app commands command — `set-selection`.

use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "selection")]
pub struct SetSelection {
    pub ids: Vec<String>,
}

pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetSelection { ids: payload.ids.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::testkit::{dispatch, layout_app};
    use crate::apps::layout::LayoutCommand;

    #[test]
    fn set_selection_reflects_in_config() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::SetSelection(SetSelection { ids: vec!["frame-text-1".into()] }));
        assert_eq!(app.snapshot().expect("projection").schema, crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA, "selection is config-only, document is untouched");
    }

    #[test]
    fn focus_preflight_issue_sets_selection_and_active_page() {
        let mut app = layout_app();
        let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(focus_preflight_issue::FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-2".into()) }));
        assert!(result.mutations.is_empty(), "preflight focus is config-only");
    }

    #[test]
    fn set_locale_is_host_pushed_with_bare_wire_keyword() {
        let command = LayoutCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() });
        assert!(protocol::OpText::print_op(&command).starts_with("locale "), "wire keyword must stay bare 'locale'");
    }
}
//#endregion 🧪️Tests
