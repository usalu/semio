//! 👁️ Fem2d play app commands — which case/mode the results window shows. Config-only: never touches
//! the document.

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigOperation};
use crate::artifacts::fem2d::op::Fem2dOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dDocument = crate::artifacts::fem2d::Fem2dDocument;

//#region 🔖️SetResultDisplay
pub mod set_result_display {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "result-display")]
    pub struct SetResultDisplay {
        pub source_id: Option<String>,
        pub mode: String,
        pub mode_index: u32,
    }

    pub fn handle(payload: &SetResultDisplay, _doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Fem2dConfigOperation::SetResultDisplay { source_id: payload.source_id.clone(), mode: payload.mode.clone(), mode_index: payload.mode_index }]))
    }
}
//#endregion 🔖️SetResultDisplay

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn set_result_display_is_config_only() {
        let mut app = fem2d_app();
        let before = app.projection().expect("projection");
        let result = dispatch(&mut app, Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }));
        assert!(result.operations.is_empty(), "setResultDisplay must not emit document operations (it's config-only)");
        assert_eq!(app.projection().expect("projection"), before);
    }
}
//#endregion 🧪️Tests
