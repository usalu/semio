//! 📐️ Generation3d artifact — snapshot re-exports, widget id helper, and artifact kind.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate self as semio_s_artifact_procedural_generation3d;





//#region 🔖️Declaration
/// 🧾️ Defines s.generation3d's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural.generation3d")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.procedural.generation3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.generation3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.procedural.generation3d.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.generation3d.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.procedural.generation3d@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.procedural.generation3d@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.txt")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.txt@utf-8/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.txt@utf-8/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"generation.3d:generation3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "generation.3d")?)?
                .claim(ArtifactIdentityClaim::codec_extension("generation.3d", "generation3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"3D Generation")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "3D Generation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"3D Generierung")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "3D Generierung")?)?,
        )
}

/// 🔖️ Assembles s.generation3d's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(semio_s_artifact_procedural_3d_model::generation3d_artifact_schema_descriptor())
        .inferences([semio_s_artifact_procedural_3d_inferences::generation3d_artifact_inference_descriptor()])
        .inference_services([semio_s_artifact_procedural_3d_geometry_service::geometry_inference_service()])
        .composers(semio_s_artifact_procedural_3d_io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<editor::generation3d::Generation3dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/📈️guest-memory/🦀️.rs"]
mod guest_memory_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️app-fixture/🦀️.rs"]
pub(crate) mod app_fixture;

#[cfg(test)]
#[path = "🧪️tests/🔬️serial/🦀️.rs"]
pub(crate) mod test_serial;



#[cfg(test)]
#[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]
pub(crate) mod store_fixture;

#[cfg(test)]
#[path = "🧪️tests/🔬️retirement-driver/🦀️.rs"]
pub(crate) mod retirement_driver;


#[cfg(test)]
#[path = "🧪️tests/🔬️publication-authority/🦀️.rs"]
pub(crate) mod publication_authority;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧭️graph-editor/🦀️.rs"]
mod graph_editor_tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🗺️touched-regions/🦀️.rs"]
mod touched_regions_tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod mutation_unit_tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🦀️.rs"]
mod gesture_leaves_tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/📄️document-restoration/🦀️.rs"]
mod document_restoration_tests;

#[cfg(test)]
#[path = "🔨️modules/🏠️host/🧪️tests/🔬️retained-authority-laws/🦀️.rs"]
mod retained_authority_laws;

//#endregion 🧪️Tests








/// 🧵️ The surface-neutral `flowEvalTick` chain BOTH surfaces run. Mounted at the artifact level,
/// beside `editor`/`viewer` rather than inside either, so the viewer can reach it without ever
/// importing through the sibling `editor` module (`policyViewerPurityBreaches`) — the same reason
/// [`GENERATION3D_DIALECT`] lives here (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod generation3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs"]
        pub mod transient;

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }


        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-generation/🦀️.rs"]
            pub mod add_generation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🦀️.rs"]
            pub mod add_widget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❌️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs"]
            pub mod flow_eval_resolve;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs"]
            pub mod flow_eval_tick;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs"]
            pub mod flow_tessellate_resolve;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔓️flow-eval-release/🦀️.rs"]
            pub mod flow_eval_release;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧯️flow-tessellate-cancel-resolve/🦀️.rs"]
            pub mod flow_tessellate_cancel_resolve;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs"]
            pub mod generation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-document/🦀️.rs"]
            pub mod export_document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📂️import-document-request/🦀️.rs"]
            pub mod import_document_request;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-document/🦀️.rs"]
            pub mod import_document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🦀️.rs"]
            pub mod navigate_graph;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-flow-widgets/🦀️.rs"]
            pub mod patch_flow_widgets;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🦀️.rs"]
            pub mod set_widget_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️remove-generation/🦀️.rs"]
            pub mod remove_generation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-widget/🦀️.rs"]
            pub mod remove_widget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-generation/🦀️.rs"]
            pub mod rename_generation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🦀️.rs"]
            pub mod reorganize;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️select-generation/🦀️.rs"]
            pub mod select_generation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-lod-mode/🦀️.rs"]
            pub mod cycle_lod_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️cycle-show-mode/🦀️.rs"]
            pub mod cycle_show_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod-mode/🦀️.rs"]
            pub mod set_lod_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-show-mode/🦀️.rs"]
            pub mod set_show_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️set-sun-azimuth/🦀️.rs"]
            pub mod set_sun_azimuth;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌄️set-sun-elevation/🦀️.rs"]
            pub mod set_sun_elevation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔆️set-sun-intensity/🦀️.rs"]
            pub mod set_sun_intensity;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️toggle-sun/🦀️.rs"]
            pub mod toggle_sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️update-generation-values/🦀️.rs"]
            pub mod update_generation_values;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs"]
                    pub mod flow;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs"]
                    pub mod form;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs"]
                    pub mod generations;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod generation3d {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🫧️transient/🦀️.rs"]
        pub mod transient;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/✅️flow-eval-resolve/🦀️.rs"]
            pub mod flow_eval_resolve;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/⏱️flow-eval-tick/🦀️.rs"]
            pub mod flow_eval_tick;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs"]
            pub mod flow_tessellate_resolve;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔓️flow-eval-release/🦀️.rs"]
            pub mod flow_eval_release;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🧯️flow-tessellate-cancel-resolve/🦀️.rs"]
            pub mod flow_tessellate_cancel_resolve;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔬️set-lod-mode/🦀️.rs"]
            pub mod set_lod_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/👁️set-show-mode/🦀️.rs"]
            pub mod set_show_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🧭️set-sun-azimuth/🦀️.rs"]
            pub mod set_sun_azimuth;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🌄️set-sun-elevation/🦀️.rs"]
            pub mod set_sun_elevation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🔆️set-sun-intensity/🦀️.rs"]
            pub mod set_sun_intensity;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🌞️toggle-sun/🦀️.rs"]
            pub mod toggle_sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/📤️export-document/🦀️.rs"]
            pub mod export_document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod preview {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🫧️transient/🦀️.rs"]
                        pub mod transient;
                    }
                }
            }
        }
    }
}
