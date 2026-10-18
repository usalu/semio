//! 🏢️ BIM model artifact: building information as a parametric, event-sourced document.
//!
//! `ModelSnapshot` stores authored parameters only; every derived value (elevations, wall heights, quantities, solids) is a field of
//! `ModelInference`. Mutations are sparse typed diffs with concrete inverses (`ModelMutation`, `ModelDiff`).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;


//#region 🔖️Declaration
/// 🔖️ This artifact's capability definition: standard, schema, inference, document codec and localization rows.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let codec_extension = format!("{}:{}:bim", standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA.chars().count(), standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA);
    let rows: Vec<(&str, &str, &str, Vec<(&str, &str)>, Option<(&str, &str)>)> = vec![
        ("s.bim.model.standard.v1", "standard", "1", vec![], None),
        ("s.bim.model.standard.v1.profile.any", "profile", "any", vec![], None),
        ("s.bim.model.schema.artifact", "schema", "s.bim.model", vec![("schema", "s.bim.model")], None),
        ("s.bim.model.inference.artifact", "inference", "s.bim.model.inference", vec![("schema", "s.bim.model.inference")], None),
        ("s.bim.model.codec.document-1", "codec", "bim.model:bim", vec![("codec", standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA), ("codec-extension", codec_extension.as_str())], None),
        ("s.bim.model.localization.en", "localization", "BIM Model", vec![], Some(("en", "BIM Model"))),
        ("s.bim.model.localization.de", "localization", "BIM-Modell", vec![], Some(("de", "BIM-Modell"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.bim.model")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(identity)?, ArtifactCapabilityKind::parse(kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(namespace)?, value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(locale)?, text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🌳️ This artifact's declaration tree root, consumed by the hub composition through `declare_artifact`: schema, io (native codec and the IFC, glTF and SVG hops), viewer, editor and examples of every subset.
pub fn artifact<A: BimApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    ArtifactDeclaration { kind: semio_framework_artifact_reference::ArtifactKindId::parse(standards::v1::subsets::any::schema::BIM_MODEL_DIALECT.artifact_kind).expect("canonical BIM kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait BimApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::bim::BimModelApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::bim::BimModelViewer>>>
{
}

impl<A> BimApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::bim::BimModelApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::bim::BimModelViewer>>>
{
}
//#endregion 🔖️Declaration

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod v1_component;
        pub use v1_component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod any_component;
                pub use any_component::*;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod authored {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📏️authored/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod storey_levels {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🏢️storey-levels/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod model_graph {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🕸️model-graph/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod wall_layout {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧱️wall-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod curtain_layout {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪞️curtain-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod opening_frames {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪟️opening-frames/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stair_runs {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪜️stair-runs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ramp_runs {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛝️ramp-runs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod spaces {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🏠️spaces/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod quantities {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧮️quantities/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod option_scope {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭️option-scope/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod phase_visibility {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎭️phase-visibility/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod finishes {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎨️finishes/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zones {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🏘️zones/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod effective_properties {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🏷️effective-properties/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod schedules {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📋️schedules/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod families {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧬️families/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod components {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪑️components/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod mep {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌀️mep/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod bodies {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️bodies/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod plan_linework {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗺️plan-linework/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod view_linework {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🖼️view-linework/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod sheet_layout {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📄️sheet-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod analytical_members {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦴️analytical-members/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod energy_envelope {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌡️energy-envelope/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod clash_sets {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧨️clash-sets/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rule_results {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️rule-results/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod annotation_layout {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪧️annotation-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod diagnostics {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚠️diagnostics/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod walls {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🧱️walls/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod curtain_walls {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🪟️curtain-walls/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod fillers {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🚪️fillers/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod wall_sweeps {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🧷️wall-sweeps/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod plan_kit {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/📐️plan-kit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod columns {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🏛️columns/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod beams {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/➖️beams/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod slabs {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/⬜️slabs/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🔲️ceilings/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod roofs {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🏠️roofs/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod stairs {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🪜️stairs/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod railings {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🛤️railings/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod ramps {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🛝️ramps/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod components {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🪑️components/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mep {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🌀️mep/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod rail_hosts {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🪝️rail-hosts/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod wall_geometry {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉️wall-geometry/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod modify {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧙️modify/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🦀️.rs"]
pub mod examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod dispatch_context {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📮️dispatch-context/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod marks {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖍️marks/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod kit {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️kit/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod terminology {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod entities {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod interaction {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod utilities {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod gestures {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️gestures/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod chrome {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎛️chrome/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod transient {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod plan {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️plan/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod world {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️world/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod section {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️section/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod schedule {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod family {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod sheet {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📄️sheet/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod options {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🧭️options/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod outliner {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🌳️outliner/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod properties {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod library {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️library/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod families {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🪑️families/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod classification {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod diagnostics {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🚨️diagnostics/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod coordination {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🤝️coordination/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod create_entity {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-entity/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod create_view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️create-view/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod delete_selection {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod rename_entity {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-entity/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_field {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️set-field/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪟️set-view/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_family {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️edit-family/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_schedule {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️edit-schedule/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod export_schedule_csv {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📊️export-schedule-csv/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod analyse_model {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️analyse-model/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod export_model {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-model/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod export_sheets {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️export-sheets/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_camera {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_down {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_move {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_pointer_up {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬆️canvas-pointer-up/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_double_click {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👆️canvas-double-click/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_commit_draft {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️canvas-commit-draft/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod canvas_escape {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚪️canvas-escape/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod world_pointer_down {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌍️world-pointer-down/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod world_pointer_move {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️world-pointer-move/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod arm_utility {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛠️arm-utility/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod flip_walls {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔃️flip-walls/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod attach_walls {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️attach-walls/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod move_storey {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️move-storey/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod split_wall {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️split-wall/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_property {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧾️set-property/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod remove_property {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️remove-property/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod select_findings {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️select-findings/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod coordinate {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️coordinate/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_classification {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️set-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod remove_classification {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗄️remove-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod apply_template {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️apply-template/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod conditions {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌡️conditions/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_template {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️edit-template/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod edit_classification {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📚️edit-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod search_classification {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️search-classification/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod cursor_keys {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️cursor-keys/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod place_elements {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📍️place-elements/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod place_grid_columns {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏛️place-grid-columns/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod engagement_input {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌨️engagement-input/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod browse_families {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪑️browse-families/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_override {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-override/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod gesture_keys {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔑️gesture-keys/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod engagement_submit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📨️engagement-submit/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "."]
pub mod render {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🖌️render/🦀️.rs"]
    mod component;
    pub use component::*;
}
