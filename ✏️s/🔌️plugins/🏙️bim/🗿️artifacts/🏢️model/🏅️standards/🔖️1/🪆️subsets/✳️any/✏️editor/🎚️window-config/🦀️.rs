//! 🎚️ Authored BIM editor world-window configuration and shared window record declarations.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "../🎭️modes/✏️edit/🪟️windows/🧊️world/🎚️config/🦀️.rs"]
pub mod world;

/// 🧭️ An option chosen for one group in a local window.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct OptionSelection {
    pub group: String,
    pub option: String,
}

/// 👁️ One local visibility override of an authored workset.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct WorksetVisibility {
    pub workset: String,
    pub visible: bool,
}

//#region 🔖️WindowConfig
/// 🎚️ Declares the persisted-local state of one window kind from its field table: the record with its default, then the shared `semio_s_artifact_bim_model_tests::bim_window_config!` for the codecs, the
/// sparse diff, the `Replace` mutation and the `WindowConfigOwner` with `current`, `from_snapshot` and `addressed`, and the owner's `register`.
#[macro_export]
macro_rules! window_config {
    (
        window: $window:expr,
        schema: $schema:literal,
        envelope: $envelope:literal,
        extension: $extension:literal,
        owner_path: $owner_path:literal,
        display: $display:literal,
        type $Config:ident, $Diff:ident, $Mutation:ident, $Owner:ident;
        $( $(#[$attr:meta])* $field:ident : $ty:ty = $default:expr; )+
    ) => {
        #[derive(semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_os_kernel::DslArtifact)]
        #[canonical_json(owner = semio_framework_pack_json)]
        #[value(rename_all = "camelCase", deny_unknown_fields)]
        #[dsl(layout = "lines")]
        #[artifact(id = $envelope, extension = $extension)]
        pub struct $Config {
            $( $(#[$attr])* pub $field: $ty, )+
        }

        impl Default for $Config {
            fn default() -> Self {
                Self { $( $field: $default, )+ }
            }
        }

        semio_s_artifact_bim_model_tests::bim_window_config! { config: $Config, diff: $Diff, mutation: $Mutation, owner: $Owner, window: $window, schema: $schema, owner_path: $owner_path, display: $display, bytes: 4_096, fields: { $($field: $ty),+ } }

        pub fn register(registry: &mut semio_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_plugin::Fault> {
            registry.register::<$Owner>()
        }
    };
}

//#endregion 🔖️WindowConfig
