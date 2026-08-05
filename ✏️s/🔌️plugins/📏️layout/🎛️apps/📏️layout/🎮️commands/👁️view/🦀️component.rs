//! 👁️ Layout play app commands — pure view interactions: selection, active page, hover, preflight
//! focus, engagement draft text and locale. All config-only.

use crate::apps::layout::config::{LayoutConfig, LayoutConfigOperation};
use crate::artifacts::layout::{op::LayoutOperation, LayoutDocument};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        Ok(Emit::config(vec![LayoutConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetActivePage
pub mod set_active_page {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-page")]
    pub struct SetActivePage {
        pub page_id: String,
    }

    pub fn handle(payload: &SetActivePage, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        Ok(Emit::config(vec![LayoutConfigOperation::SetActivePage { page_id: payload.page_id.clone() }]))
    }
}
//#endregion 🔖️SetActivePage

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "hover")]
    pub struct SetHover {
        pub id: Option<String>,
    }

    pub fn handle(payload: &SetHover, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        Ok(Emit::config(vec![LayoutConfigOperation::SetHover { id: payload.id.clone() }]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️FocusPreflightIssue
pub mod focus_preflight_issue {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "focus-preflight-issue")]
    pub struct FocusPreflightIssue {
        pub object_id: Option<String>,
        pub page_id: Option<String>,
    }

    pub fn handle(payload: &FocusPreflightIssue, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        let mut config_operations = Vec::new();
        if let Some(object_id) = &payload.object_id {
            config_operations.push(LayoutConfigOperation::SetSelection { ids: vec![object_id.clone()] });
        }
        if let Some(page_id) = &payload.page_id {
            config_operations.push(LayoutConfigOperation::SetActivePage { page_id: page_id.clone() });
        }
        Ok(Emit::config(config_operations))
    }
}
//#endregion 🔖️FocusPreflightIssue

//#region 🔖️EngagementInput
pub mod engagement_input {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "engagement-input")]
    pub struct EngagementInput {
        pub value: String,
    }

    pub fn handle(payload: &EngagementInput, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        Ok(Emit::config(vec![LayoutConfigOperation::SetEngagementInput { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️EngagementInput

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    /// 🗣️ Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing
    /// a palette entry), which is why its wire keyword stays the bare `"locale"` rather than the
    /// kebab-cased `"set-locale"` its command id would suggest — see the `as` literal in
    /// `crate::apps::layout`'s `app_commands!` invocation.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, LayoutDocument>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault> {
        Ok(Emit::config(vec![LayoutConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::testkit::{dispatch, layout_app};
    use crate::apps::layout::LayoutCommand;

    #[test]
    fn set_selection_reflects_in_config() {
        let mut app = layout_app();
        dispatch(&mut app, LayoutCommand::SetSelection(set_selection::SetSelection { ids: vec!["frame-text-1".into()] }));
        assert_eq!(app.projection().expect("projection").schema, crate::artifacts::layout::LAYOUT_FIXTURE_SCHEMA, "selection is config-only, document is untouched");
    }

    #[test]
    fn focus_preflight_issue_sets_selection_and_active_page() {
        let mut app = layout_app();
        let result = dispatch(&mut app, LayoutCommand::FocusPreflightIssue(focus_preflight_issue::FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-2".into()) }));
        assert!(result.operations.is_empty(), "preflight focus is config-only");
    }

    #[test]
    fn set_locale_is_host_pushed_with_bare_wire_keyword() {
        let command = LayoutCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() });
        assert!(protocol::OpText::print_op(&command).starts_with("locale "), "wire keyword must stay bare 'locale'");
    }
}
//#endregion 🧪️Tests
