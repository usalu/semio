//! 🎥️ Shared persisted-local configuration of every concrete Norm Viewport window: camera preset, hidden layers, colour map and unit system.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::mutations::change_viewport_view::ChangeViewportView;
pub use crate::mutations::NormViewportWindowConfigMutation;
pub use crate::diff::NormViewportWindowConfigDiff;
pub use crate::schema::NormViewportWindowConfig;

//#region 🔖️Config

impl store::ConfigRecord for NormViewportWindowConfig {}

//#endregion 🔖️Config

/// 🪟️ Declares one family's concrete Viewport-window owner without duplicating its shared schema.
#[macro_export]
macro_rules! norm_viewport_window_config_owner {
    ($owner:ident, $window_kind_id:expr) => {
        pub struct $owner;

        impl semio_plugin::WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $window_kind_id;
            const SCHEMA: &'static str = "s.norm.viewport-window.config";
            const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
            type State = $crate::schema::NormViewportWindowConfig;
            type Mutation = $crate::mutations::NormViewportWindowConfigMutation;
            type Edit = semio_plugin::app::WindowConfigApplyEdit<Self::State, Self::Mutation>;
            const MAXIMUM_PREPARATION_DEPTH: usize = 64;

            fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
                std::sync::Arc::new(semio_plugin::app::WindowConfigApplyEdit::new())
            }

            fn preparation_source_birth() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_plugin::bounded_window_config_preparation_source_birth::<Self>() }
            fn build_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State, Self::Mutation>> { semio_plugin::bounded_window_config_store_owners::<Self>(grant) }

            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                semio_plugin::bounded_window_config_preparation_factory::<Self>()
            }

            fn build_store_disposer() -> Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                semio_plugin::bounded_window_config_store_disposer::<Self>()
            }
        }
    };
}

/// 👁️ Reads the view only from the exact Viewport-window snapshot captured for this call.
pub fn current<O>(view: &semio_plugin::ConfigView<'_, semio_plugin::NoConfig>) -> NormViewportWindowConfig
where
    O: semio_plugin::WindowConfigOwner<State = NormViewportWindowConfig, Mutation = NormViewportWindowConfigMutation>,
{
    view.window::<O>().cloned().unwrap_or_default()
}

/// 🎯️ Addresses a view change to a concrete Viewport instance: the addressed or focused window when it is one, else the document's first Viewport window.
pub fn addressed<O>(
    view: &semio_plugin::ViewModel,
    mutation: NormViewportWindowConfigMutation,
) -> Result<semio_plugin::WindowConfigMutation, semio_plugin::Fault>
where
    O: semio_plugin::WindowConfigOwner<State = NormViewportWindowConfig, Mutation = NormViewportWindowConfigMutation>,
{
    let owner = |window: &&semio_plugin::ViewWindowInstance| window.window_kind_id == O::WINDOW_KIND_ID;
    let named = view.window_id.as_deref().into_iter().chain(view.focused_window_id.as_deref());
    let id = named
        .filter_map(|id| view.window_instances.iter().find(|window| window.id == id))
        .find(owner)
        .or_else(|| view.window_instances.iter().find(owner))
        .map(|window| window.id.as_str())
        .ok_or_else(|| semio_plugin::Fault::from(if view.window_instances.is_empty() { "norm-viewport-window-required" } else { "norm-viewport-window-stale" }))?;
    Ok(semio_plugin::WindowConfigMutation::of::<O>(id, mutation))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🪢️Mounts
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
        #[path = "🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod text {
        #[path = "🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
        #[path = "🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
}
//#endregion 🪢️Mounts
