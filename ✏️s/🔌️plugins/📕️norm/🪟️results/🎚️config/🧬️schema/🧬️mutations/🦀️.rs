//! 🎚️ The closed semantic mutation vocabulary for Norm Results-window configuration.

use crate::mutations::change_selected_check_index::ChangeSelectedCheckIndex;
use crate::diff::NormResultsWindowConfigDiff;
use crate::schema::NormResultsWindowConfig;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = NormResultsWindowConfig, diff = NormResultsWindowConfigDiff, schema = "s.norm.results-window.config")]
pub enum NormResultsWindowConfigMutation {
    ChangeSelectedCheckIndex(ChangeSelectedCheckIndex),
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<NormResultsWindowConfig> for NormResultsWindowConfigMutation {
    fn exchange(self, post: &mut NormResultsWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::ChangeSelectedCheckIndex(ChangeSelectedCheckIndex { index }) => Self::ChangeSelectedCheckIndex(ChangeSelectedCheckIndex { index: std::mem::replace(&mut post.selected_check_index, index) }),
        })
    }
}

//#region 🪢️Mounts
#[path = "☑️change-selected-check-index/🦀️.rs"]
pub mod change_selected_check_index;
//#endregion 🪢️Mounts
