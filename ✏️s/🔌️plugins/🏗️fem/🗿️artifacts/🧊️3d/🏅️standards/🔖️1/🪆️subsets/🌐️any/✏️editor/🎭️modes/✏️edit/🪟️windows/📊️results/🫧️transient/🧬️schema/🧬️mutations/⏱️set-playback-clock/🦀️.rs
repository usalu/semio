//! ⏱️ Publishes — or clears — the running playback clock of one concrete FEM 3D results window.

use super::{Fem3dResultsWindowTransient, Fem3dResultsWindowTransientMutation};
use crate::editor::fem3d::modes::edit::windows::results::transient::Fem3dPlaybackClock;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-playback-clock")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPlaybackClock {
    #[dsl(block)]
    pub clock: Option<Fem3dPlaybackClock>,
}

impl protocol::MutationKind<Fem3dResultsWindowTransient, Fem3dResultsWindowTransientMutation> for SetPlaybackClock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "results-window-playback-clock", kind: "set-playback-clock", record: "SetPlaybackClock" };

    fn diff(&self, base: &Fem3dResultsWindowTransient) -> protocol::MutationOutcome<Fem3dResultsWindowTransient> {
        let mut next = base.clone();
        next.clock = self.clock;
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Fem3dResultsWindowTransient) -> Vec<Fem3dResultsWindowTransientMutation> {
        vec![Self { clock: base.clock }.into()]
    }

    fn label(&self) -> String {
        "Set Results Window Playback Clock".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["clock".into()]
    }
}
