//! 📄️ Mathematical play app commands — replacing the whole document (graph + geometry) in one go.

use crate::apps::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::dsl::MathematicalGraphDsl;
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetDocument
pub mod set_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-document")]
    pub struct SetDocument {
        #[dsl(block)]
        pub graph: MathematicalGraphDsl,
        #[dsl(block)]
        pub geometry: MathematicalGeometry,
    }

    pub fn handle(payload: &SetDocument, doc: &DocumentView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
        let projection = doc.snapshot;
        let Ok(graph) = crate::artifacts::mathematical::dsl::math_graph_from_dsl(payload.graph.clone()) else {
            return Ok(Emit::default());
        };
        let mut operations = Vec::new();
        if graph != projection.graph {
            operations.push(MathematicalMutation::SetGraph { graph });
        }
        if payload.geometry != projection.geometry {
            operations.push(MathematicalMutation::SetGeometry { geometry: payload.geometry.clone() });
        }
        Ok(Emit::mutations(operations))
    }
}
//#endregion 🔖️SetDocument

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_document;
    use crate::apps::mathematical::testkit::{dispatch, math_app};
    use crate::apps::mathematical::MathematicalCommand;
    use crate::artifacts::mathematical::MathematicalGeometry;

    #[test]
    fn set_document_replaces_graph_and_geometry() {
        let mut app = math_app();
        let geometry = MathematicalGeometry { points: vec![crate::artifacts::mathematical::MathematicalPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, MathematicalCommand::SetDocument(set_document::SetDocument { graph: crate::artifacts::mathematical::dsl::math_graph_to_dsl(&crate::artifacts::mathematical::MathematicalGraph { algorithm: "components".into(), ..Default::default() }), geometry: geometry.clone() }));
        let projection = app.snapshot().expect("projection");
        assert_eq!(projection.graph.algorithm, "components");
        assert_eq!(projection.geometry, geometry);
    }
}
//#endregion 🧪️Tests
