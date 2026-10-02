//! ⏱️ Publishes — or clears — the running playback clock of one concrete FEM 2D results window.

use super::{Fem2dResultsWindowTransient, Fem2dResultsWindowTransientMutation};
use crate::editor::fem2d::modes::edit::windows::results::transient::Fem2dPlaybackClock;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-playback-clock")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPlaybackClock {
    #[dsl(block)]
    pub clock: Option<Fem2dPlaybackClock>,
}

impl protocol::MutationKind<Fem2dResultsWindowTransient, Fem2dResultsWindowTransientMutation> for SetPlaybackClock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "results-window-playback-clock", kind: "set-playback-clock", record: "SetPlaybackClock" };

    fn diff(&self, base: &Fem2dResultsWindowTransient) -> protocol::MutationOutcome<Fem2dResultsWindowTransient> {
        let mut next = base.clone();
        next.clock = self.clock;
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Fem2dResultsWindowTransient) -> Vec<Fem2dResultsWindowTransientMutation> {
        vec![Self { clock: base.clock }.into()]
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Results Window Playback Clock", "Wiedergabeuhr des Ergebnisfensters setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["clock".into()]
    }
}
