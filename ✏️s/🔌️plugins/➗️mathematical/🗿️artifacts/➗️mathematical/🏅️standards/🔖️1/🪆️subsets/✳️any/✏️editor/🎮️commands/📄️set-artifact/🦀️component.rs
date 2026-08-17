//! 📄️ 📄️ Mathematical play app commands command — `set-artifact`.

use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::dsl::MathematicalGraphDsl;
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::mathematical::schema::mutations::replace_points::mutation::ReplacePoints;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-artifact")]
pub struct SetArtifact {
    #[dsl(block)]
    pub graph: MathematicalGraphDsl,
    #[dsl(block)]
    pub geometry: MathematicalGeometry,
}

pub fn handle(payload: &SetArtifact, doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    let Ok(graph) = crate::artifacts::mathematical::dsl::math_graph_from_dsl(payload.graph.clone()) else {
        return Ok(Emit::default());
    };
    let mut operations = Vec::new();
    if graph != crate::artifacts::mathematical::mathematical_graph(doc.snapshot) {
        operations.push(MathematicalMutation::ReplaceGraph(ReplaceGraph { graph }));
    }
    if payload.geometry != crate::artifacts::mathematical::mathematical_geometry(doc.snapshot) {
        operations.push(MathematicalMutation::ReplacePoints(ReplacePoints { points: payload.geometry.points.clone() }));
    }
    Ok(Emit::mutations(operations))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::mathematical::testkit::{dispatch, math_app};
    use crate::editor::mathematical::MathematicalCommand;
    use crate::artifacts::mathematical::MathematicalGeometry;

    #[test]
    fn set_artifact_replaces_graph_and_geometry() {
        let mut app = math_app();
        let geometry = MathematicalGeometry { points: vec![crate::artifacts::mathematical::MathematicalPoint { x: 1.0, y: 2.0 }] };
        dispatch(&mut app, MathematicalCommand::SetArtifact(SetArtifact { graph: crate::artifacts::mathematical::dsl::math_graph_to_dsl(&crate::artifacts::mathematical::MathematicalGraph { algorithm: "components".into(), ..Default::default() }), geometry: geometry.clone() }));
        let projection = app.snapshot().expect("projection");
        assert_eq!(crate::artifacts::mathematical::mathematical_graph(&projection).algorithm, "components");
        assert_eq!(crate::artifacts::mathematical::mathematical_geometry(&projection), geometry);
    }
}
//#endregion 🧪️Tests
