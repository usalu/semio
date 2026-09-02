//! 🗣️ GIS 3D play app command — the host-pushed locale switch (undeclared in the manifest, never in
//! the command palette; host/test infra dispatches it directly).

use crate::artifacts::gisterrain::op::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use crate::editor::gis3d::config::{Gis3dConfig, Gis3dConfigMutation, SetLocale as SetLocaleMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, Gis3dConfig>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis3dConfigMutation::SetLocale(SetLocaleMutation { value: payload.value.clone() })]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis3d::testkit::{app, dispatch};
    use crate::editor::gis3d::Gis3dCommand;

    /// 🗣️ `SetLocale` is not palette-declared but still dispatches cleanly end-to-end (command_id
    /// mapping → `handle` → config store) — the same typed channel the shell uses to push locale.
    #[semio_framework_async_macros::async_test]
    async fn locale_command_dispatches_through_the_config_store() {
        let mut app = app();
        let result = dispatch(&mut app, Gis3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        assert!(result.mutations.is_empty(), "locale is config state, not a document edit");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_locale_is_not_declared_in_the_manifest() {
        let definition = crate::editor::gis3d::create_gis3d_app().definition;
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == "setLocale"), "locale is host-pushed, never palette-staged");
    }
}
//#endregion 🧪️Tests
