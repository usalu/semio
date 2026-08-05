//! 📄️ Mathematical play app commands — replacing the whole document (graph + geometry) in one go.

use crate::apps::mathematical::config::{MathConfig, MathConfigOperation};
use crate::artifacts::mathematical::dsl::MathGraphDsl;
use crate::artifacts::mathematical::op::MathOperation;
use crate::artifacts::mathematical::{MathGeometry, MathProjection};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetDocument
pub mod set_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-document")]
    pub struct SetDocument {
        #[dsl(block)]
        pub graph: MathGraphDsl,
        #[dsl(block)]
        pub geometry: MathGeometry,
    }

    pub fn handle(payload: &SetDocument, doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathOperation, MathConfigOperation>, Fault> {
        let projection = doc.projection;
        let Ok(graph) = crate::artifacts::mathematical::dsl::math_graph_from_dsl(payload.graph.clone()) else {
            return Ok(Emit::default());
        };
        let mut operations = Vec::new();
        if graph != projection.graph {
            operations.push(MathOperation::SetGraph { graph });
        }
        if payload.geometry != projection.geometry {
            operations.push(MathOperation::SetGeometry { geometry: payload.geometry.clone() });
        }
        Ok(Emit::operations(operations))
    }
}
//#endregion 🔖️SetDocument

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_document;
    use crate::apps::mathematical::testkit::{dispatch, math_app};
    use crate::apps::mathematical::MathCommand;
    use crate::artifacts::mathematical::MathGeometry;

    #[test]
    fn set_document_replaces_graph_and_geometry() {
        let mut app = math_app();
        let geometry = MathGeometry { points: vec![crate::artifacts::mathematical::MathPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, MathCommand::SetDocument(set_document::SetDocument { graph: crate::artifacts::mathematical::dsl::math_graph_to_dsl(&crate::artifacts::mathematical::MathGraph { algorithm: "components".into(), ..Default::default() }), geometry: geometry.clone() }));
        let projection = app.projection().expect("projection");
        assert_eq!(projection.graph.algorithm, "components");
        assert_eq!(projection.geometry, geometry);
    }
}
//#endregion 🧪️Tests
