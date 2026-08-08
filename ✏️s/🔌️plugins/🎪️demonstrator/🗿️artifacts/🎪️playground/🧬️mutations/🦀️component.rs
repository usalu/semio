//! 🧬️ Playground artifact — minimal mutation dispatch.
use serde::{Deserialize, Serialize};
use protocol::Mutation;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PlaygroundMutation {
    #[default]
    NoMutation,
    SetDocument {
        #[dsl(block)]
        document: PlaygroundDocument,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlaygroundDocument {
    pub schema: String,
}

pub fn apply_playground_mutation(projection: &mut PlaygroundDocument, mutation: &PlaygroundMutation) {
    match mutation {
        PlaygroundMutation::NoMutation => {}
        PlaygroundMutation::SetDocument { document } => *projection = document.clone(),
    }
}

impl Mutation<PlaygroundDocument> for PlaygroundMutation {
    type Diff = PlaygroundMutation;
    fn diff(&self, _p: &PlaygroundDocument) -> Self::Diff { self.clone() }
    fn inverse(&self, p: &PlaygroundDocument) -> Vec<Self> {
        vec![PlaygroundMutation::SetDocument { document: p.clone() }]
    }
}
