//! 📏️ Norm plugin — fifteen compliance-standard document apps (DIN 4108, DIN EN 16798, DIN V 18599,
//! EN 1990–1999, ISO 16757, VDI 3805) in one hot-swappable WASM plugin, each backed by a headless
//! `NormHost` that recomputes its `CheckReport` from the document on every read.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! 🫀️ `core` is unusually large for a plugin kernel here, and deliberately so: the fifteen standards are
//! structurally identical apps over fifteen genuinely different document schemas, so the *domain* kernel
//! (quantities, clause identity, check results, national annexes, the `NormFamily`/`NormHost` contract,
//! the generic whole-document mutation and its text/binary codecs) and the *app-surface* kernel (the one
//! shared config, the media ports, the render primitives, the manifest constructors) each exist exactly
//! once, while every per-standard fact — schema, ids, labels, compute — lives in that standard's own
//! artifact and app nodes.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit< Mutation, NormConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it here
// would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 📄️Document kernel
// 🐛️ W5a fix: dir is `📄️artifact` on disk (renamed from `📄️document` by the framework-wide
// document->artifact terminology migration, ticket 26/08/05); this `#[path]` still pointed at the
// old name and broke every downstream build. Module name stays `document` — `crate::document::…`
// is used pervasively across this crate's engines/apps.
#[path = "../../📄️artifact/🦀️component.rs"]
pub mod document;
#[path = "."]
pub mod config {
    #[path = "../../🎚️config/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🎚️config/🧬️schema/🦀️component.rs"]
    pub mod schema;
}
#[path = "."]
pub mod presence {
    #[path = "../../👥️presence/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../👥️presence/🧬️schema/🦀️component.rs"]
    pub mod schema;
}
#[path = "../../🖥️app-surface/🦀️component.rs"]
pub mod app_surface;
//#endregion 📄️Document kernel

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod iso16757 {
        #[path = "../../🗿️artifacts/📓️iso16757/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            
                                #[path = "."]
                                pub mod remove_selection_constraint {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod rename_product {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️rename-product/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️rename-product/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️rename-product/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod rename_product_group {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿rename-product-group/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿rename-product-group/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿rename-product-group/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod add_selection_constraint {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁add-selection-constraint/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁add-selection-constraint/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁add-selection-constraint/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod remove_part_number_input {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿remove-part-number-input/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿remove-part-number-input/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿remove-part-number-input/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_product_group {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀create-product-group/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀create-product-group/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀create-product-group/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_property_definition {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾create-property-definition/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾create-property-definition/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾create-property-definition/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_subject {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵create-subject/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵create-subject/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵create-subject/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_selection_class {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-selection-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-selection-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-selection-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod rename_manufacturer {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳rename-manufacturer/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳rename-manufacturer/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳rename-manufacturer/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod rename_catalogue {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲rename-catalogue/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲rename-catalogue/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲rename-catalogue/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_product {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁create-product/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁create-product/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁create-product/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod replace_part_number_rule {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂replace-part-number-rule/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂replace-part-number-rule/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂replace-part-number-rule/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_exchange_process {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-exchange-process/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-exchange-process/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-exchange-process/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_part_number_input {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-part-number-input/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-part-number-input/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-part-number-input/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_script_limits {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷update-script-limits/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷update-script-limits/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷update-script-limits/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_product {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸delete-product/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸delete-product/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸delete-product/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_product_group {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹delete-product-group/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹delete-product-group/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹delete-product-group/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_property_definition {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺delete-property-definition/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺delete-property-definition/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺delete-property-definition/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_subject {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻delete-subject/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻delete-subject/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻delete-subject/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_selection_series {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-selection-series/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-selection-series/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-selection-series/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::Iso16757Snapshot;
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::Iso16757Mutation;
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::Iso16757Diff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📓️iso16757/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod vdi3805 {
        #[path = "../../🗿️artifacts/📔️vdi3805/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::Vdi3805Snapshot;
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation;
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::Vdi3805Diff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📔️vdi3805/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod din4108 {
        #[path = "../../🗿️artifacts/📕️din4108/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            
                                #[path = "."]
                                pub mod change_solar_absorptance {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-solar-absorptance/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-solar-absorptance/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-solar-absorptance/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bb2_details_conform {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-bb2-details-conform/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-bb2-details-conform/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-bb2-details-conform/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod insert_layer {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢insert-layer/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢insert-layer/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢insert-layer/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod remove_layer {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️remove-layer/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️remove-layer/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️remove-layer/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_psi_times_l_sum {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-psi-times-l-sum/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-psi-times-l-sum/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-psi-times-l-sum/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_t_int_c {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-t-int-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-t-int-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-t-int-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_climate {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-climate/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-climate/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-climate/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_moisture_mu_exterior {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-moisture-mu-exterior/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-moisture-mu-exterior/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-moisture-mu-exterior/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_airtightness_class {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-airtightness-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-airtightness-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-airtightness-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_category {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-category/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-category/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-category/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_moisture_mu_interior {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-moisture-mu-interior/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-moisture-mu-interior/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-moisture-mu-interior/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_rh_int {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-rh-int/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-rh-int/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-rh-int/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_airtightness_n50 {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-airtightness-n50/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-airtightness-n50/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-airtightness-n50/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_catalog_id {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-catalog-id/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-catalog-id/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-catalog-id/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_declared_application_class {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-declared-application-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-declared-application-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-declared-application-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_layer_thickness {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-layer-thickness/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-layer-thickness/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-layer-thickness/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_envelope_area_m2 {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-envelope-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-envelope-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-envelope-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod reorder_layers {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷reorder-layers/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷reorder-layers/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷reorder-layers/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_irradiance_w_m2 {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-irradiance-wm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-irradiance-wm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-irradiance-wm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_application_type {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-application-type/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-application-type/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-application-type/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_layer_lambda {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-layer-lambda/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-layer-lambda/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-layer-lambda/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_material_id {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-material-id/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-material-id/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-material-id/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot;
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::Din4108Mutation;
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::Din4108Diff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📕️din4108/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod din16798 {
        #[path = "../../🗿️artifacts/📗️din16798/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_occupancy {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂change-occupancy/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂change-occupancy/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂change-occupancy/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_comfort_category {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-comfort-category/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-comfort-category/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-comfort-category/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_t_op_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-t-op-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-t-op-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-t-op-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_rh_percent {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹change-rh-percent/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹change-rh-percent/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹change-rh-percent/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_air_speed_m_s {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-air-speed-ms/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-air-speed-ms/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-air-speed-ms/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_theta_rm_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-theta-rm-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-theta-rm-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-theta-rm-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_co2_ppm {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-co2-ppm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-co2-ppm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-co2-ppm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_df_percent {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-df-percent/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-df-percent/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-df-percent/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_l_aeq_db {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-l-aeq-db/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-l-aeq-db/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-l-aeq-db/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_persons {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-persons/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-persons/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-persons/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_ida_class {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-ida-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-ida-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-ida-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_ventilation_m3_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-ventilation-m3-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-ventilation-m3-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_floor_area_m2 {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-floor-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-floor-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-floor-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bedrooms {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-bedrooms/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-bedrooms/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-bedrooms/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_dwelling_ventilation_m3_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-dwelling-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-dwelling-ventilation-m3-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-dwelling-ventilation-m3-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_occupants {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-occupants/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-occupants/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-occupants/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_residential_ventilation_m3_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸change-residential-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸change-residential-ventilation-m3-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸change-residential-ventilation-m3-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sfp_w_m3_s {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻change-sfp-wm3-s/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻change-sfp-wm3-s/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻change-sfp-wm3-s/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sfp_required_class {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺change-sfp-required-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺change-sfp-required-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺change-sfp-required-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_heat_recovery_eta {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-heat-recovery-eta/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-heat-recovery-eta/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-heat-recovery-eta/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_heat_recovery_eta_min {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-heat-recovery-eta-min/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-heat-recovery-eta-min/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-heat-recovery-eta-min/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_system_type {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-system-type/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-system-type/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-system-type/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_years_since_inspection {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-years-since-inspection/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-years-since-inspection/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-years-since-inspection/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_humidification_required_kg_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-humidification-required-kg-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-humidification-required-kg-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-humidification-required-kg-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_humidification_provided_kg_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-humidification-provided-kg-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-humidification-provided-kg-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-humidification-provided-kg-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fan_q_v_m3_s {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-fan-qvm3-s/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-fan-qvm3-s/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-fan-qvm3-s/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fan_t_run_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-fan-t-run-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-fan-t-run-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-fan-t-run-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fan_energy_reference_kwh {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-fan-energy-reference-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-fan-energy-reference-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-fan-energy-reference-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_night_setback_k {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-night-setback-k/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-night-setback-k/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-night-setback-k/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hr_m_dot_kg_s {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-hr-m-dot-kg-s/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-hr-m-dot-kg-s/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-hr-m-dot-kg-s/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hr_cp_j_kgk {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-hr-cp-j-kgk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-hr-cp-j-kgk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-hr-cp-j-kgk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hr_delta_t_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-hr-delta-tc/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-hr-delta-tc/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-hr-delta-tc/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hr_t_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-hr-th/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-hr-th/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-hr-th/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hr_savings_reference_kwh {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-hr-savings-reference-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-hr-savings-reference-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-hr-savings-reference-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n50_h_inv {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-n50-h-inv/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-n50-h-inv/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-n50-h-inv/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_volume_m3 {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-volume-m3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-volume-m3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-volume-m3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_infiltration_allowance_m3_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-infiltration-allowance-m3-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-infiltration-allowance-m3-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-infiltration-allowance-m3-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cellar_area_m2 {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️change-cellar-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️change-cellar-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️change-cellar-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cellar_ventilation_m3_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-cellar-ventilation-m3-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-cellar-ventilation-m3-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-cellar-ventilation-m3-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_tr_w_k {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-h-tr-wk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-h-tr-wk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-h-tr-wk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_ve_w_k {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-h-ve-wk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-h-ve-wk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-h-ve-wk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_theta_e_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-theta-ec/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-theta-ec/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-theta-ec/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_theta_set_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-theta-set-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-theta-set-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-theta-set-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cooling_delta_t_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-cooling-delta-th/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-cooling-delta-th/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-cooling-delta-th/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cooling_gains_kwh {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-cooling-gains-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-cooling-gains-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-cooling-gains-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cooling_utilization_factor {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-cooling-utilization-factor/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-cooling-utilization-factor/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-cooling-utilization-factor/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cooling_reference_kwh {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-cooling-reference-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-cooling-reference-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-cooling-reference-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_chiller_type {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-chiller-type/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-chiller-type/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-chiller-type/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_eer_actual {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-eer-actual/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-eer-actual/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-eer-actual/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_q_c_kwh {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷change-qc-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷change-qc-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷change-qc-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_generation_reference_kwh {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-generation-reference-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-generation-reference-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-generation-reference-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_data_center_supply_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-data-center-supply-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-data-center-supply-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-data-center-supply-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_st_w_k {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-h-st-wk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-h-st-wk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-h-st-wk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_theta_st_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-theta-st-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-theta-st-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-theta-st-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_theta_amb_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-theta-amb-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-theta-amb-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-theta-amb-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_storage_t_h {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-storage-th/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-storage-th/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-storage-th/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_storage_allowance_kwh {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-storage-allowance-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-storage-allowance-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-storage-allowance-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_dhw_delivery_c {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-dhw-delivery-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-dhw-delivery-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-dhw-delivery-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_duct_class {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-duct-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-duct-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-duct-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_duct_test_pressure_pa {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-duct-test-pressure-pa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-duct-test-pressure-pa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-duct-test-pressure-pa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_duct_leakage_m3_s_m2 {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-duct-leakage-m3-sm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-duct-leakage-m3-sm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-duct-leakage-m3-sm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::Din16798Snapshot;
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::Din16798Mutation;
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::Din16798Diff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📗️din16798/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod en1990 {
        #[path = "../../🗿️artifacts/📘️en1990/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            
                                #[path = "."]
                                pub mod insert_variable_action {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴insert-variable-action/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴insert-variable-action/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴insert-variable-action/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod remove_variable_action {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎remove-variable-action/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎remove-variable-action/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎remove-variable-action/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_seismic_action {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-seismic-action/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-seismic-action/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-seismic-action/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_consequence_class {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-consequence-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-consequence-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-consequence-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_permanent_action {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-permanent-action/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-permanent-action/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-permanent-action/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_variable_action_category {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-variable-action-category/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-variable-action-category/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-variable-action-category/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod reorder_variable_actions {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗reorder-variable-actions/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗reorder-variable-actions/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗reorder-variable-actions/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_variable_action_value {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-variable-action-value/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-variable-action-value/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-variable-action-value/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_resistance {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-resistance/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-resistance/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-resistance/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::En1990Snapshot;
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::En1990Mutation;
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::En1990Diff;


    }
    #[path = "."]
    pub mod en1991 {
        #[path = "../../🗿️artifacts/📘️en1991/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod change_bridge_lane_width_m {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-bridge-lane-width-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fire_resistance_min {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-fire-resistance-min/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_en_sk_kn_m2 {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-en-sk-kn-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_assumed_gk_kn_m2 {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-assumed-gk-kn-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hoist_class {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-hoist-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_snow_altitude_m {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-snow-altitude-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_snow_zone {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-snow-zone/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hoisting_speed_ms {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-hoisting-speed-ms/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cs {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-cs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bridge_lane {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-bridge-lane/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_delta_tk {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-delta-tk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_cd {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-cd/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_wind_zone {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞change-wind-zone/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_bulk_density_kn_m3 {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗change-silo-bulk-density-kn-m3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_construction_activity {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟change-construction-activity/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_category {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭change-category/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_accidental_mass_t {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-accidental-mass-t/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_hydraulic_radius_m {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊change-silo-hydraulic-radius-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_en_vbms {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈change-en-vbms/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_mu {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉change-silo-mu/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_self_weight_thickness_m {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮change-self-weight-thickness-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_area_m2 {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪change-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧫change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_k {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬change-silo-k/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fire_curve {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭change-fire-curve/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bridge_moment_resistance_knm {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡change-bridge-moment-resistance-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_height_m {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-silo-height-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_accidental_speed_km_h {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️change-accidental-speed-km-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bridge_span_m {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-bridge-span-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fire_member_capacity_c {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️change-fire-member-capacity-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_self_weight_material {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-self-weight-material/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_crane_class {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-crane-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::En1991Snapshot;
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::En1991Mutation;
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::En1991Diff;


    }
    #[path = "."]
    pub mod en1992 {
        #[path = "../../🗿️artifacts/📘️en1992/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_a_c_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-ac-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-ac-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-ac-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_a_s_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_c1_mm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_cracked {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-anchor-cracked/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-anchor-cracked/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-anchor-cracked/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_d_mm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-anchor-d-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-anchor-d-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-anchor-d-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_f_uk_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_f_yk_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-anchor-f-yk-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-anchor-f-yk-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-anchor-f-yk-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_h_ef_mm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_n_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_anchor_v_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_a_s_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-as-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-as-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-as-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_b_mm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-b-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-b-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-b-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bridge_delta_sigma_s_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bridge_sigma_c_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_d_mm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-d-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-d-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-d-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_ck {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-f-ck/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-f-ck/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-f-ck/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_yk {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-f-yk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-f-yk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-f-yk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fire_rating {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-fire-rating/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-fire-rating/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-fire-rating/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_hd_over_h {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-hd-over-h/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-hd-over-h/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-hd-over-h/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_liquid_e_s_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_liquid_f_ct_eff_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_liquid_rho_p_eff {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_liquid_sigma_s_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_liquid_s_r_max_mm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_m_ed_knm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-m-ed-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-m-ed-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-m-ed-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-n-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-n-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-n-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_p_kn {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-p-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-p-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-p-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_provided_axis_distance_mm {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_rho_l {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-rho-l/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-rho-l/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-rho-l/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_span_m {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-span-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-span-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-span-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tightness_class {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-tightness-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-tightness-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-tightness-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_udl_kn_m {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-udl-kn-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-udl-kn-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-udl-kn-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_use_fem {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-use-fem/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-use-fem/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-use-fem/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-v-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-v-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-v-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::En1992Snapshot;
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::En1992Mutation;
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::En1992Diff;


    }
    #[path = "."]
    pub mod en1993 {
        #[path = "../../🗿️artifacts/📘️en1993/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            
                                #[path = "."]
                                pub mod update_pile_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️update-pile-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️update-pile-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️update-pile-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_weld_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱update-weld-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱update-weld-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱update-weld-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_bridge_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️update-bridge-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️update-bridge-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️update-bridge-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_member_properties {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊update-member-properties/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊update-member-properties/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊update-member-properties/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_crane_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-crane-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-crane-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️update-crane-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_hss_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧update-hss-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧update-hss-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧update-hss-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_through_thickness_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️update-through-thickness-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️update-through-thickness-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️update-through-thickness-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_cold_formed_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥update-cold-formed-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥update-cold-formed-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥update-cold-formed-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_fatigue_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️update-fatigue-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️update-fatigue-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️update-fatigue-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_tension_component_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡update-tension-component-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡update-tension-component-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡update-tension-component-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_fire_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆update-fire-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆update-fire-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆update-fire-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_bolt_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞update-bolt-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞update-bolt-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌞update-bolt-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_tower_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗update-tower-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗update-tower-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗update-tower-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_silo_shell_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟update-silo-shell-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟update-silo-shell-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪟update-silo-shell-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_plated_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭update-plated-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭update-plated-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭update-plated-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_stainless_inputs {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️update-stainless-inputs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️update-stainless-inputs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️update-stainless-inputs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::En1993Snapshot;
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::En1993Mutation;
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::En1993Diff;


    }
    #[path = "."]
    pub mod en1994 {
        #[path = "../../🗿️artifacts/📘️en1994/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            
                                #[path = "."]
                                pub mod change_f_y_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-fy-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-fy-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-fy-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-v-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-v-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-v-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_d_mm {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-d-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-d-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-d-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_span_m {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-span-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-span-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-span-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_m_pl_rd {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-m-pl-rd/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-m-pl-rd/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-m-pl-rd/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_u_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-fu-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-fu-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-fu-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_e_cm_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-e-cm-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-e-cm-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-e-cm-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_delta_tau_stud_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-delta-tau-stud-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-delta-tau-stud-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-delta-tau-stud-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_delta_sigma_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-delta-sigma-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-delta-sigma-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-delta-sigma-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_sc_mm {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-h-sc-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-h-sc-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-h-sc-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_m_ed_knm {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-m-ed-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-m-ed-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-m-ed-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n_cycles_stud {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-n-cycles-stud/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-n-cycles-stud/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-n-cycles-stud/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fatigue_detail {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-fatigue-detail/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-fatigue-detail/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-fatigue-detail/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_eta {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-eta/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-eta/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-eta/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_insulation_thickness_mm {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-insulation-thickness-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-insulation-thickness-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-insulation-thickness-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_ed_per_stud_kn {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-v-ed-per-stud-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-v-ed-per-stud-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-v-ed-per-stud-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_m_pla {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-m-pla/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-m-pla/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-m-pla/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fire_rating {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-fire-rating/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-fire-rating/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-fire-rating/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_ck_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-f-ck-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-f-ck-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-f-ck-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_deck_type {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-deck-type/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-deck-type/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-deck-type/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_l_rd {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-vl-rd/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-vl-rd/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-vl-rd/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::En1994Snapshot;
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::En1994Mutation;
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::En1994Diff;


    }
    #[path = "."]
    pub mod en1995 {
        #[path = "../../🗿️artifacts/📘️en1995/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_a_ef_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-a-ef-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-a-ef-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-a-ef-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_a_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-a-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-a-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-a-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_a_vert_m_s2 {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-a-vert-ms2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-a-vert-ms2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-a-vert-ms2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_b_mm {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-b-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-b-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-b-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-f-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-f-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-f-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_c_0_k {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-fc0-k/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-fc0-k/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-fc0-k/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fire_duration_min {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-fire-duration-min/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-fire-duration-min/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-fire-duration-min/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_m_k {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-fmk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-fmk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-fmk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_v_k {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-fvk/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-fvk/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-fvk/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_mm {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-h-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-h-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-h-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_load_duration {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-load-duration/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-load-duration/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-load-duration/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_m_crit_knm {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-m-crit-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-m-crit-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-m-crit-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_m_ed_knm {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-m-ed-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-m-ed-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-m-ed-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n_cycles_bridge {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-n-cycles-bridge/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-n-cycles-bridge/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-n-cycles-bridge/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-n-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-n-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-n-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_section_depth_mm {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-section-depth-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-section-depth-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-section-depth-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_service_class {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-service-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-service-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-service-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-v-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-v-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-v-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_w_mm3 {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-w-mm3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-w-mm3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-w-mm3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
}
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::En1995Snapshot;
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::En1995Mutation;
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::En1995Diff;


    }
    #[path = "."]
    pub mod en1996 {
        #[path = "../../🗿️artifacts/📘️en1996/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod change_m_ed_knm {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-m-ed-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-m-ed-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-m-ed-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-n-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-n-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-n-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼change-v-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼change-v-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔼change-v-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-h-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-h-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️change-h-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_z_mm3 {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️change-z-mm3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️change-z-mm3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➡️change-z-mm3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_area_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬅️change-area-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬅️change-area-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬅️change-area-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_shear_area_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-shear-area-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-shear-area-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏change-shear-area-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_k_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩change-fk-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩change-fk-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟩change-fk-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_f_vk_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-f-vk-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-f-vk-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️change-f-vk-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_masonry_class {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-masonry-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-masonry-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️change-masonry-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_design_situation {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-design-situation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-design-situation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-design-situation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_mu {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-mu/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-mu/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️change-mu/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_wall_thickness_mm {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-wall-thickness-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-wall-thickness-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢change-wall-thickness-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fire_resistance_min {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-fire-resistance-min/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-fire-resistance-min/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊change-fire-resistance-min/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_unit {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-unit/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-unit/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️change-unit/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_exposure {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-exposure/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-exposure/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧change-exposure/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_mortar {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-mortar/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-mortar/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌬️change-mortar/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bed_joint_thickness_mm {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-bed-joint-thickness-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-bed-joint-thickness-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔥change-bed-joint-thickness-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_storeys {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-storeys/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-storeys/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❄️change-storeys/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_ef_mm {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-h-ef-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-h-ef-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚡change-h-ef-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_t_ef_mm {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-t-ef-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-t-ef-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆change-t-ef-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::En1996Snapshot;
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::En1996Mutation;
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::En1996Diff;


    }
    #[path = "."]
    pub mod en1997 {
        #[path = "../../🗿️artifacts/📘️en1997/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod change_v_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-v-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-v-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-v-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-h-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-h-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-h-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_footing_area_m2 {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-footing-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-footing-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-footing-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_phi_deg {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-phi-deg/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-phi-deg/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-phi-deg/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_c_kpa {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-c-kpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-c-kpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-c-kpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_gamma_kn_m3 {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-gamma-kn-m3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-gamma-kn-m3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-gamma-kn-m3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_b_m {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-bm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-bm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-bm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_d_f_m {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-dfm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-dfm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-dfm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_e_s_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-es-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-es-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-es-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_nu {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-nu/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-nu/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-nu/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_design_approach {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-design-approach/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-design-approach/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-design-approach/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_settlement_limit_mm {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-settlement-limit-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-settlement-limit-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-settlement-limit-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n_pile_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-n-pile-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-n-pile-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-n-pile-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_alpha_s {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-alpha-s/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-alpha-s/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-alpha-s/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_pile_d_m {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-pile-dm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-pile-dm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-pile-dm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_q_s_kpa {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-qs-kpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-qs-kpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-qs-kpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_pile_l_m {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-pile-lm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-pile-lm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-pile-lm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_q_b_kpa {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-qb-kpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-qb-kpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-qb-kpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_pile_base_area_m2 {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-pile-base-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-pile-base-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-pile-base-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_pile_n_profiles {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-pile-n-profiles/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-pile-n-profiles/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-pile-n-profiles/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_z_investigated_m {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-z-investigated-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-z-investigated-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-z-investigated-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::En1997Snapshot;
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::En1997Mutation;
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::En1997Diff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📘️en1997/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod en1998 {
        #[path = "../../🗿️artifacts/📘️en1998/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod change_seismic_zone {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-seismic-zone/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-seismic-zone/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-seismic-zone/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_ground_type {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-ground-type/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-ground-type/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-ground-type/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_importance_class {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-importance-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-importance-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-importance-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_structural_system {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-structural-system/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-structural-system/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-structural-system/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_t1_s {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-t1-s/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-t1-s/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-t1-s/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_mass_t {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-mass-t/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-mass-t/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-mass-t/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_rd_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-v-rd-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-v-rd-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-v-rd-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_drift_mm {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-drift-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-drift-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-drift-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_height_m {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-height-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-height-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-height-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_multiple_resisting_systems {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-multiple-resisting-systems/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-multiple-resisting-systems/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-multiple-resisting-systems/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_en_a_gr {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-en-a-gr/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-en-a-gr/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-en-a-gr/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_en_ground_type {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-en-ground-type/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-en-ground-type/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-en-ground-type/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_en_spectrum_type {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-en-spectrum-type/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-en-spectrum-type/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-en-spectrum-type/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_period_ratio {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-period-ratio/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-period-ratio/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-period-ratio/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bridge_v_rd_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-bridge-v-rd-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-bridge-v-rd-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-bridge-v-rd-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bearing_d_ed_mm {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-bearing-d-ed-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-bearing-d-ed-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-bearing-d-ed-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_bearing_d_rd_mm {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-bearing-d-rd-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-bearing-d-rd-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-bearing-d-rd-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_retrofit_knowledge_level {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-retrofit-knowledge-level/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-retrofit-knowledge-level/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-retrofit-knowledge-level/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_retrofit_limit_state {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-retrofit-limit-state/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-retrofit-limit-state/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-retrofit-limit-state/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_retrofit_e_d_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-retrofit-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-retrofit-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-retrofit-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_retrofit_r_k_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-retrofit-rk-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-retrofit-rk-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-retrofit-rk-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_retrofit_gamma_el {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-retrofit-gamma-el/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-retrofit-gamma-el/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-retrofit-gamma-el/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_height_m {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-silo-height-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-silo-height-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-silo-height-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_radius_m {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-silo-radius-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-silo-radius-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-silo-radius-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_n_rd_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-silo-n-rd-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-silo-n-rd-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-silo-n-rd-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_v_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-silo-v-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-silo-v-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-silo-v-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_v_rd_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-silo-v-rd-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-silo-v-rd-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-silo-v-rd-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_silo_q_nominal {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-silo-q-nominal/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-silo-q-nominal/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-silo-q-nominal/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tank_height_m {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-tank-height-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-tank-height-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-tank-height-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tank_radius_m {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-tank-radius-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-tank-radius-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-tank-radius-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tank_mass_t {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-tank-mass-t/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-tank-mass-t/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-tank-mass-t/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tank_v_rd_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-tank-v-rd-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-tank-v-rd-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-tank-v-rd-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tower_m_ed_knm {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-tower-m-ed-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-tower-m-ed-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-tower-m-ed-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tower_m_rd_knm {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-tower-m-rd-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-tower-m-rd-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-tower-m-rd-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tower_is_chimney {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-tower-is-chimney/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-tower-is-chimney/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-tower-is-chimney/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tower_q_nominal {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-tower-q-nominal/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-tower-q-nominal/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-tower-q-nominal/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_tower_mass_t {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-tower-mass-t/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-tower-mass-t/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-tower-mass-t/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_foundation_area_m2 {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-foundation-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-foundation-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-foundation-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_foundation_p_rd_kpa {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-foundation-p-rd-kpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-foundation-p-rd-kpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-foundation-p-rd-kpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_foundation_h_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-foundation-h-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-foundation-h-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-foundation-h-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_foundation_h_rd_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-foundation-h-rd-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-foundation-h-rd-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-foundation-h-rd-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_k_foundation {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-k-foundation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-k-foundation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-k-foundation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_k_soil {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-k-soil/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-k-soil/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-k-soil/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_wall_height_m {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-wall-height-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-wall-height-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-wall-height-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_wall_phi_deg {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-wall-phi-deg/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-wall-phi-deg/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-wall-phi-deg/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_wall_soil_gamma_kn_m3 {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-wall-soil-gamma-kn-m3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-wall-soil-gamma-kn-m3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-wall-soil-gamma-kn-m3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_wall_r {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-wall-r/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-wall-r/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-wall-r/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_wall_h_rd_kn {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-wall-h-rd-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-wall-h-rd-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-wall-h-rd-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::En1998Snapshot;
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::En1998Mutation;
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::En1998Diff;


    }
    #[path = "."]
    pub mod en1999 {
        #[path = "../../🗿️artifacts/📘️en1999/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod change_n_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-n-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-n-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-n-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_m_ed_knm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-m-ed-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-m-ed-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-m-ed-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_a_mm2 {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-a-mm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-a-mm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-a-mm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_w_el_mm3 {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-w-el-mm3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-w-el-mm3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-w-el-mm3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_alloy {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-alloy/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-alloy/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-alloy/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_chi {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-chi/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-chi/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-chi/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_i_t_mm4 {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-it-mm4/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-it-mm4/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-it-mm4/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_l_cr_mm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-l-cr-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-l-cr-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-l-cr-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_theta_c {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-theta-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-theta-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-theta-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_delta_sigma_ed {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-delta-sigma-ed/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-delta-sigma-ed/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-delta-sigma-ed/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_delta_sigma_c {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-delta-sigma-c/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-delta-sigma-c/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-delta-sigma-c/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_fatigue_m {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-fatigue-m/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-fatigue-m/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-fatigue-m/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_n_cycles {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-n-cycles/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-n-cycles/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-n-cycles/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_v_weld_ed_kn {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-v-weld-ed-kn/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-v-weld-ed-kn/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-v-weld-ed-kn/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_weld_throat_mm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-weld-throat-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-weld-throat-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-weld-throat-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_weld_length_mm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-weld-length-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-weld-length-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-weld-length-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_beta_w {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-beta-w/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-beta-w/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-beta-w/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sheet_b_mm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-sheet-b-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-sheet-b-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-sheet-b-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sheet_t_mm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-sheet-t-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-sheet-t-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-sheet-t-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sheet_k_sigma {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-sheet-k-sigma/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-sheet-k-sigma/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-sheet-k-sigma/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sheet_w_el_mm3 {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-sheet-w-el-mm3/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-sheet-w-el-mm3/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-sheet-w-el-mm3/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sheet_m_ed_knm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-sheet-m-ed-knm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-sheet-m-ed-knm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-sheet-m-ed-knm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_shell_t_mm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-shell-t-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-shell-t-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-shell-t-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_shell_r_mm {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-shell-r-mm/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-shell-r-mm/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-shell-r-mm/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_sigma_ed_shell_mpa {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-sigma-ed-shell-mpa/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-sigma-ed-shell-mpa/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-sigma-ed-shell-mpa/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annex {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-annex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-annex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-annex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::En1999Snapshot;
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::En1999Mutation;
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::En1999Diff;


    }
    #[path = "."]
    pub mod din18599 {
        #[path = "../../🗿️artifacts/📙️din18599/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod change_use_class {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-use-class/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-use-class/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-use-class/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_heated_area_m2 {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-heated-area-m2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-heated-area-m2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-heated-area-m2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_occupants {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-occupants/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-occupants/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-occupants/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_t {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-ht/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-ht/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-ht/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_h_v {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-hv/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-hv/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-hv/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_internal_gains_w_m2 {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-internal-gains-wm2/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-internal-gains-wm2/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-internal-gains-wm2/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_solar_gains_kwh {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦥change-solar-gains-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦥change-solar-gains-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦥change-solar-gains-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_system_losses_kwh {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦦change-system-losses-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦦change-system-losses-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦦change-system-losses-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_renewable_kwh {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦨change-renewable-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦨change-renewable-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦨change-renewable-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_annual_limit_kwh {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦡change-annual-limit-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦡change-annual-limit-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦡change-annual-limit-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_energy_carrier {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-energy-carrier/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-energy-carrier/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-energy-carrier/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_reference_q_p_kwh {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-reference-qp-kwh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-reference-qp-kwh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽change-reference-qp-kwh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_climate {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘update-climate/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘update-climate/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘update-climate/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::Din18599Snapshot;
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::Din18599Mutation;
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::Din18599Diff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📙️din18599/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod din4108 {
        #[path = "../../🎛️apps/📕️din4108/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📕️din4108/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📕️din4108/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📕️din4108/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📕️din4108/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📕️din4108/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📕️din4108/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📕️din4108/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📕️din4108/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📕️din4108/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod din16798 {
        #[path = "../../🎛️apps/📗️din16798/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📗️din16798/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📗️din16798/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📗️din16798/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📗️din16798/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📗️din16798/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📗️din16798/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📗️din16798/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📗️din16798/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📗️din16798/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod din18599 {
        #[path = "../../🎛️apps/📙️din18599/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📙️din18599/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📙️din18599/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📙️din18599/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📙️din18599/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📙️din18599/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📙️din18599/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📙️din18599/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📙️din18599/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📙️din18599/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1990 {
        #[path = "../../🎛️apps/📘️en1990/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1990/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1990/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1990/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1990/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1990/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1990/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1990/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1990/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1991 {
        #[path = "../../🎛️apps/📘️en1991/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1991/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1991/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1991/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1991/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1991/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1991/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1991/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1991/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1991/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1992 {
        #[path = "../../🎛️apps/📘️en1992/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1992/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1992/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1992/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1992/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1992/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1992/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1992/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1992/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1992/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1993 {
        #[path = "../../🎛️apps/📘️en1993/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1993/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1993/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1993/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1993/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1993/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1993/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1993/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1993/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1993/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1994 {
        #[path = "../../🎛️apps/📘️en1994/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1994/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1994/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1994/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1994/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1994/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1994/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1994/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1994/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1994/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1995 {
        #[path = "../../🎛️apps/📘️en1995/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1995/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1995/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1995/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1995/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1995/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1995/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1995/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1995/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1995/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1996 {
        #[path = "../../🎛️apps/📘️en1996/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1996/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1996/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1996/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1996/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1996/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1996/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1996/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1996/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1996/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1997 {
        #[path = "../../🎛️apps/📘️en1997/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1997/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1997/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1997/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1997/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1997/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1997/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1997/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1997/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1997/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1998 {
        #[path = "../../🎛️apps/📘️en1998/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1998/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1998/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1998/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1998/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1998/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1998/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1998/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1998/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1998/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1999 {
        #[path = "../../🎛️apps/📘️en1999/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1999/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📘️en1999/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1999/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1999/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1999/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1999/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1999/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1999/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1999/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod iso16757 {
        #[path = "../../🎛️apps/📓️iso16757/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📓️iso16757/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📓️iso16757/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📓️iso16757/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📓️iso16757/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📓️iso16757/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📓️iso16757/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📓️iso16757/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📓️iso16757/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📓️iso16757/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod vdi3805 {
        #[path = "../../🎛️apps/📔️vdi3805/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📔️vdi3805/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
            #[path = "../../🎛️apps/📔️vdi3805/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📔️vdi3805/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📔️vdi3805/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📔️vdi3805/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📔️vdi3805/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📔️vdi3805/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📔️vdi3805/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📔️vdi3805/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔧️setup/🦀️component.rs"]
mod setup;
pub use setup::register_norm_exports;

#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📓️iso16757/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_iso16757_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📓️iso16757/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_iso16757_demo_tests;
    #[path = "../../🗿️artifacts/📔️vdi3805/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_vdi3805_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📔️vdi3805/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_vdi3805_demo_tests;
    #[path = "../../🗿️artifacts/📕️din4108/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_din4108_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📕️din4108/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_din4108_demo_tests;
    #[path = "../../🗿️artifacts/📗️din16798/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_din16798_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📗️din16798/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_din16798_demo_tests;
    #[path = "../../🗿️artifacts/📘️en1990/📚️examples/📕️high-consequence-office/🦀️component.rs"]
    pub mod art_en1990_high_consequence_office;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1990/📚️examples/📕️high-consequence-office/🧪️tests/🦀️test.rs"]
    mod art_en1990_high_consequence_office_tests;
    #[path = "../../🗿️artifacts/📘️en1991/📚️examples/📕️retail-hydrocarbon-fire/🦀️component.rs"]
    pub mod art_en1991_retail_hydrocarbon_fire;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1991/📚️examples/📕️retail-hydrocarbon-fire/🧪️tests/🦀️test.rs"]
    mod art_en1991_retail_hydrocarbon_fire_tests;
    #[path = "../../🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor/🦀️component.rs"]
    pub mod art_en1992_liquid_retaining_fem_anchor;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor/🧪️tests/🦀️test.rs"]
    mod art_en1992_liquid_retaining_fem_anchor_tests;
    #[path = "../../🗿️artifacts/📘️en1993/📚️examples/📕️high-strength-connection/🦀️component.rs"]
    pub mod art_en1993_high_strength_connection;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1993/📚️examples/📕️high-strength-connection/🧪️tests/🦀️test.rs"]
    mod art_en1993_high_strength_connection_tests;
    #[path = "../../🗿️artifacts/📘️en1994/📚️examples/📕️composite-bridge-girder/🦀️component.rs"]
    pub mod art_en1994_composite_bridge_girder;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1994/📚️examples/📕️composite-bridge-girder/🧪️tests/🦀️test.rs"]
    mod art_en1994_composite_bridge_girder_tests;
    #[path = "../../🗿️artifacts/📘️en1995/📚️examples/📕️glulam-footbridge/🦀️component.rs"]
    pub mod art_en1995_glulam_footbridge;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1995/📚️examples/📕️glulam-footbridge/🧪️tests/🦀️test.rs"]
    mod art_en1995_glulam_footbridge_tests;
    #[path = "../../🗿️artifacts/📘️en1996/📚️examples/📕️loadbearing-wall/🦀️component.rs"]
    pub mod art_en1996_loadbearing_wall;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1996/📚️examples/📕️loadbearing-wall/🧪️tests/🦀️test.rs"]
    mod art_en1996_loadbearing_wall_tests;
    #[path = "../../🗿️artifacts/📘️en1997/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_en1997_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1997/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_en1997_demo_tests;
    #[path = "../../🗿️artifacts/📘️en1998/📚️examples/📕️seismic-rc-frame/🦀️component.rs"]
    pub mod art_en1998_seismic_rc_frame;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1998/📚️examples/📕️seismic-rc-frame/🧪️tests/🦀️test.rs"]
    mod art_en1998_seismic_rc_frame_tests;
    #[path = "../../🗿️artifacts/📘️en1999/📚️examples/📕️aluminium-roof-purlin/🦀️component.rs"]
    pub mod art_en1999_aluminium_roof_purlin;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📘️en1999/📚️examples/📕️aluminium-roof-purlin/🧪️tests/🦀️test.rs"]
    mod art_en1999_aluminium_roof_purlin_tests;
    #[path = "../../🗿️artifacts/📙️din18599/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_din18599_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📙️din18599/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_din18599_demo_tests;
    #[path = "../../🎛️apps/📓️iso16757/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_iso16757_demo_session;
    #[path = "../../🎛️apps/📔️vdi3805/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_vdi3805_demo_session;
    #[path = "../../🎛️apps/📕️din4108/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_din4108_demo_session;
    #[path = "../../🎛️apps/📗️din16798/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_din16798_demo_session;
    #[path = "../../🎛️apps/📘️en1990/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1990_demo_session;
    #[path = "../../🎛️apps/📘️en1991/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1991_demo_session;
    #[path = "../../🎛️apps/📘️en1992/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1992_demo_session;
    #[path = "../../🎛️apps/📘️en1993/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1993_demo_session;
    #[path = "../../🎛️apps/📘️en1994/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1994_demo_session;
    #[path = "../../🎛️apps/📘️en1995/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1995_demo_session;
    #[path = "../../🎛️apps/📘️en1996/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1996_demo_session;
    #[path = "../../🎛️apps/📘️en1997/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1997_demo_session;
    #[path = "../../🎛️apps/📘️en1998/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1998_demo_session;
    #[path = "../../🎛️apps/📘️en1999/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1999_demo_session;
    #[path = "../../🎛️apps/📙️din18599/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_din18599_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
