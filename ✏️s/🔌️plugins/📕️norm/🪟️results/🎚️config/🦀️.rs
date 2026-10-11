//! 📊️ Shared persisted-local configuration for every concrete Norm Results window.

pub use super::mutations::{change_selected_check_index::ChangeSelectedCheckIndex, NormResultsWindowConfigMutation};
pub use super::diff::NormResultsWindowConfigDiff;
pub use super::schema::NormResultsWindowConfig;

//#region 🔖️Config


impl store::ConfigRecord for NormResultsWindowConfig {}

//#endregion 🔖️Config

/// 🪟️ Declares one family's concrete Results-window owner without duplicating its shared schema.
#[macro_export]
macro_rules! norm_results_window_config_owner {
    ($owner:ident, $window_kind_id:expr) => {
        pub struct $owner;

        impl semio_framework_plugin::WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $window_kind_id;
            const SCHEMA: &'static str = "s.norm.results-window.config";
            const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
            type State = $crate::results_window_config::NormResultsWindowConfig;
            type Mutation = $crate::results_window_config::NormResultsWindowConfigMutation;
            type Edit = semio_framework_plugin::app::WindowConfigApplyEdit<Self::State, Self::Mutation>;
            const MAXIMUM_PREPARATION_DEPTH: usize = 64;

            fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
                std::sync::Arc::new(semio_framework_plugin::app::WindowConfigApplyEdit::new())
            }

            fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
                semio_framework_plugin::bounded_window_config_store_owners::<Self>()
            }

            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
            }

            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
            }
        }
    };
}

/// 👁️ Reads selection only from the exact Results-window snapshot captured for this call.
pub fn current<O>(view: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> NormResultsWindowConfig
where
    O: semio_framework_plugin::WindowConfigOwner<State = NormResultsWindowConfig, Mutation = NormResultsWindowConfigMutation>,
{
    view.window::<O>().cloned().unwrap_or_default()
}

/// 🎯️ Addresses selection to a concrete Results instance, including a panel's focused window.
pub fn addressed<O>(
    view: &semio_framework_plugin::ViewModel,
    mutation: NormResultsWindowConfigMutation,
) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault>
where
    O: semio_framework_plugin::WindowConfigOwner<State = NormResultsWindowConfig, Mutation = NormResultsWindowConfigMutation>,
{
    let id = view
        .window_id
        .as_deref()
        .or(view.focused_window_id.as_deref())
        .ok_or_else(|| semio_framework_plugin::Fault::from("norm-results-window-required"))?;
    let kind = view
        .window_instances
        .iter()
        .find(|window| window.id == id)
        .map(|window| window.window_kind_id.as_str())
        .ok_or_else(|| semio_framework_plugin::Fault::from("norm-results-window-stale"))?;
    if kind != O::WINDOW_KIND_ID {
        return Err(semio_framework_plugin::Fault::from("norm-results-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<O>(id, mutation))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
