//! ⏱️ Publishes — or clears — the running playback clock of one concrete FEM results window.

use super::{FemPlaybackClockChange, FemResultsWindowTransient, FemResultsWindowTransientDiff, FemResultsWindowTransientMutation};
use crate::editor::fem2d::modes::edit::windows::results::transient::FemPlaybackClock;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-playback-clock")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPlaybackClock {
    #[dsl(block)]
    pub clock: Option<FemPlaybackClock>,
}

impl protocol::MutationKind<FemResultsWindowTransient, FemResultsWindowTransientMutation> for SetPlaybackClock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "results-window-playback-clock", kind: "set-playback-clock", record: "SetPlaybackClock" };

    fn diff(&self, _base: &FemResultsWindowTransient) -> protocol::MutationOutcome<FemResultsWindowTransientDiff> {
        protocol::MutationOutcome::new(FemResultsWindowTransientDiff { clock: Some(FemPlaybackClockChange { clock: self.clock }) })
    }

    fn inverse(&self, base: &FemResultsWindowTransient) -> Result<Vec<FemResultsWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { clock: base.clock }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Results Window Playback Clock", "Wiedergabeuhr des Ergebnisfensters setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["clock".into()]
    }
}
