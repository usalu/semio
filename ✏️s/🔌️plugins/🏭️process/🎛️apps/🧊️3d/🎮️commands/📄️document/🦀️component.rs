//! 📄️ Process 3d play app commands — wholesale document swaps (load example / set document).

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::artifacts::process3d::engine::{default_document, plate_document};
use crate::artifacts::process3d::{op::Process3dMutation, Process3dDocument};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetDocument
pub mod set_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "document")]
    pub struct SetDocument {
        #[dsl(block)]
        pub document: Process3dDocument,
    }

    pub fn handle(payload: &SetDocument, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        Ok(Emit {
            document_mutations: vec![Process3dMutation::SetDocument { document: payload.document.clone() }],
            config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: None }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️SetDocument

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let document = match payload.example_id.as_str() {
            crate::apps::process3d::PROCESS3D_EXAMPLE_PLATE | "plate" => plate_document(),
            "" => Process3dDocument::default(),
            _ => default_document(),
        };
        Ok(Emit { document_mutations: vec![Process3dMutation::SetDocument { document }], config_mutations: vec![Process3dConfigMutation::SetSelectedId { value: None }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample
