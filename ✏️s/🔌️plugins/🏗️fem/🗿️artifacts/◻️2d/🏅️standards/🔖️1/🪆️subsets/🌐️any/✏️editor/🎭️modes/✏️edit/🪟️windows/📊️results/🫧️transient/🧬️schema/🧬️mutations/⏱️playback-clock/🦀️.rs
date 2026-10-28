//! ⏱️ Publishes — or clears — the running playback clock of one concrete FEM results window.

use super::FemPlaybackClockChange;

use super::FemResultsWindowTransient;

use super::FemResultsWindowTransientDiff;

use super::FemResultsWindowTransientMutation;
use crate::transient::FemPlaybackClock;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-playback-clock")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPlaybackClock {
    #[dsl(block)]
    pub clock: Option<FemPlaybackClock>,
}

impl protocol::MutationKind<FemResultsWindowTransient, FemResultsWindowTransientMutation> for SetPlaybackClock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "results-window-playback-clock", kind: "set-playback-clock", record: "SetPlaybackClock" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Results Window Playback Clock", "Wiedergabeuhr des Ergebnisfensters setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["clock".into()]
    }
}
impl protocol::ColdMutationKind<FemResultsWindowTransient, FemResultsWindowTransientMutation> for SetPlaybackClock {

    fn diff(&self, base: &FemResultsWindowTransient) -> protocol::MutationOutcome<FemResultsWindowTransientDiff> {
        if base.clock == self.clock {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The playback clock already holds that value.");
        }
        protocol::MutationOutcome::new(FemResultsWindowTransientDiff { clock: Some(FemPlaybackClockChange { clock: self.clock }) })
    }


    fn inverse(&self, base: &FemResultsWindowTransient) -> Result<Vec<FemResultsWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { clock: base.clock }.into()]
    
    })())
}
}

#[cfg(test)]
#[path = "🧪️tests/⚖️law/🦀️.rs"]
mod law_tests;
