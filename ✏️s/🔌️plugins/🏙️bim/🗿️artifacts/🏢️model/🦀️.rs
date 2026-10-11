//! 🏢️ BIM model artifact: building information as a parametric, event-sourced document.
//!
//! `ModelSnapshot` stores authored parameters only; every derived value (elevations, wall heights, quantities, solids) is a field of
//! `ModelInference`. Mutations are sparse typed diffs with concrete inverses (`ModelMutation`, `ModelDiff`).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

use {semio_framework_artifact_reference::Dialect, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId, semio_framework_plugin::ArtifactKindSpec, semio_framework_plugin::MediaClass, semio_framework_plugin::MediaForm, semio_framework_plugin::MediaType, semio_framework_plugin::OsMediaCapability};

pub use crate::standards::v1::subsets::any::schema::diff::patches::*;
pub use crate::standards::v1::subsets::any::schema::diff::{Assigned, ClassificationSetPatch, Entry, KeyedDelta, ModelDiff, Patch, PropertySetPatch};
pub use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
pub use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::*;

/// 🪪️ The document schema identity of the artifact.
pub const BIM_MODEL_DOCUMENT_SCHEMA: &str = "s.bim.model@1";

/// 🪪️ This artifact's dialect: the canonical surface ids are `s.bim.model@1/*#editor` and `s.bim.model@1/*#viewer`.
pub const BIM_MODEL_DIALECT: Dialect = Dialect { artifact_kind: "s.bim.model", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`, stitched into the app manifests.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.bim-model".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("BIM Model", "BIM-Modell"),
        source_format: "bim.model".into(),
        component_kind: "bim".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: BIM_MODEL_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's capability definition: standard, schema, inference, document codec and localization rows.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let codec_extension = format!("{}:{}:bim", BIM_MODEL_DOCUMENT_SCHEMA.chars().count(), BIM_MODEL_DOCUMENT_SCHEMA);
    let rows: Vec<(&str, &str, &str, Vec<(&str, &str)>, Option<(&str, &str)>)> = vec![
        ("s.bim.model.standard.v1", "standard", "1", vec![], None),
        ("s.bim.model.standard.v1.profile.any", "profile", "any", vec![], None),
        ("s.bim.model.schema.artifact", "schema", "s.bim.model", vec![("schema", "s.bim.model")], None),
        ("s.bim.model.inference.artifact", "inference", "s.bim.model.inference", vec![("schema", "s.bim.model.inference")], None),
        ("s.bim.model.codec.document-1", "codec", "bim.model:bim", vec![("codec", BIM_MODEL_DOCUMENT_SCHEMA), ("codec-extension", codec_extension.as_str())], None),
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
    ArtifactDeclaration { kind: semio_framework_artifact_reference::ArtifactKindId::parse(BIM_MODEL_DIALECT.artifact_kind).expect("canonical BIM kind"), localization: &[], standards: vec![standards::v1::standard()] }
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦴️analytical-members/🦀️.rs"] mod component;
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
                            pub mod wall_sweeps {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧊️element-solids/🧷️wall-sweeps/🦀️.rs"]
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

                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿️horizontal-rules/🦀️.rs"]
                        pub mod horizontal_rules;

                        //#region 🔖️Leaves
                        #[path = "."]
                        pub mod create_site {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️create-site/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️create-site/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️create-site/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️create-site/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️create-site/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️create-site/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_site {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-site/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-site/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-site/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-site/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-site/🧪️tests/🌊️cascades-its-buildings/🦀️.rs"]
                            mod tests_cascades_its_buildings;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-site/🧪️tests/🏗️cascades-the-whole-site/🦀️.rs"]
                            mod tests_cascades_the_whole_site;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-site/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_building {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️create-building/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️create-building/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️create-building/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️create-building/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️create-building/🧪️tests/🚫️site-missing/🦀️.rs"]
                            mod tests_site_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️create-building/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_building {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-building/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-building/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-building/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-building/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-building/🧪️tests/🌊️cascades-its-storeys/🦀️.rs"]
                            mod tests_cascades_its_storeys;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-building/🧪️tests/🏗️cascades-the-whole-building/🦀️.rs"]
                            mod tests_cascades_the_whole_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏚️delete-building/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_storey {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🧪️tests/✅️stacks-above/🦀️.rs"]
                            mod tests_stacks_above;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🧪️tests/🚫️level-taken/🦀️.rs"]
                            mod tests_level_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🧪️tests/⛔️building-missing/🦀️.rs"]
                            mod tests_building_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🧪️tests/🧮️with-a-plan-cut-height/🦀️.rs"]
                            mod tests_with_a_plan_cut_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜️create-storey/🧪️tests/🧯️non-positive-plan-cut-height/🦀️.rs"]
                            mod tests_non_positive_plan_cut_height;
                        }
                        #[path = "."]
                        pub mod rename_storey {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-storey/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-storey/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-storey/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-storey/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-storey/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_storey_height {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️set-storey-height/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️set-storey-height/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️set-storey-height/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️set-storey-height/🧪️tests/✅️raises-the-ground-storey/🦀️.rs"]
                            mod tests_raises_the_ground_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️set-storey-height/🧪️tests/🚫️non-positive/🦀️.rs"]
                            mod tests_non_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️set-storey-height/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_storey_level {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️set-storey-level/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️set-storey-level/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️set-storey-level/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️set-storey-level/🧪️tests/✅️relevels/🦀️.rs"]
                            mod tests_relevels;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️set-storey-level/🧪️tests/🚫️level-taken/🦀️.rs"]
                            mod tests_level_taken;
                        }
                        #[path = "."]
                        pub mod delete_storey {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/🧕️cascades-clash-sets-and-rules-scoped-to-it/🦀️.rs"]
                            mod tests_cascades_clash_sets_and_rules_scoped_to_it;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/✅️cascades-the-walls/🦀️.rs"]
                            mod tests_cascades_the_walls;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/🧲️empty-storey/🦀️.rs"]
                            mod tests_empty_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/🚫️constrains-another-wall/🦀️.rs"]
                            mod tests_constrains_another_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/🌊️cascades-the-opening/🦀️.rs"]
                            mod tests_cascades_the_opening;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/🏗️cascades-everything-on-it/🦀️.rs"]
                            mod tests_cascades_everything_on_it;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮️delete-storey/🧪️tests/📋️cascades-the-schedules/🦀️.rs"]
                            mod tests_cascades_the_schedules;
                        }
                        #[path = "."]
                        pub mod create_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/✅️adds-on-the-first-storey/🦀️.rs"]
                            mod tests_adds_on_the_first_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/🚫️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/⛔️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/🧮️adds-a-wall-under-the-roof/🦀️.rs"]
                            mod tests_adds_a_wall_under_the_roof;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/🧯️adds-a-wall-standing-on-a-slab/🦀️.rs"]
                            mod tests_adds_a_wall_standing_on_a_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/🧰️roof-missing/🦀️.rs"]
                            mod tests_roof_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/🧲️base-slab-missing/🦀️.rs"]
                            mod tests_base_slab_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️create-wall/🧪️tests/🧳️base-slab-in-another-building/🦀️.rs"]
                            mod tests_base_slab_in_another_building;
                        }
                        #[path = "."]
                        pub mod delete_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/🧪️tests/🌊️cascades-the-opening/🦀️.rs"]
                            mod tests_cascades_the_opening;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/🧪️tests/🏗️cascades-the-opening-and-data/🦀️.rs"]
                            mod tests_cascades_the_opening_and_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💥️delete-wall/🧪️tests/🧭️cascades-its-sweeps/🦀️.rs"]
                            mod tests_cascades_its_sweeps;
                        }
                        #[path = "."]
                        pub mod set_wall_top {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/✅️constrains-to-the-first-storey/🦀️.rs"]
                            mod tests_constrains_to_the_first_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🧲️frees-the-height/🦀️.rs"]
                            mod tests_frees_the_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🚫️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🧭️attaches-to-a-roof/🦀️.rs"]
                            mod tests_attaches_to_a_roof;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🧮️attaches-to-a-slab/🦀️.rs"]
                            mod tests_attaches_to_a_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🧯️roof-missing/🦀️.rs"]
                            mod tests_roof_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🧰️ceiling-missing/🦀️.rs"]
                            mod tests_ceiling_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🧳️frees-an-attached-top/🦀️.rs"]
                            mod tests_frees_an_attached_top;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔝️set-wall-top/🧪️tests/🧴️roof-in-another-building/🦀️.rs"]
                            mod tests_roof_in_another_building;
                        }
                        #[path = "."]
                        pub mod create_railing {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧱️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/📏️path-too-short/🦀️.rs"]
                            mod tests_path_too_short;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/💥️non-positive-height/🦀️.rs"]
                            mod tests_non_positive_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🚧️non-positive-post-spacing/🦀️.rs"]
                            mod tests_non_positive_post_spacing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧮️adds-balusters-and-a-glass-infill/🦀️.rs"]
                            mod tests_adds_balusters_and_a_glass_infill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧯️baluster-spacing-not-positive/🦀️.rs"]
                            mod tests_baluster_spacing_not_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧰️infill-thickness-not-positive/🦀️.rs"]
                            mod tests_infill_thickness_not_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧲️rail-profile-degenerate/🦀️.rs"]
                            mod tests_rail_profile_degenerate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧳️adds-a-railing-hosted-by-a-ramp/🦀️.rs"]
                            mod tests_adds_a_railing_hosted_by_a_ramp;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧴️adds-a-railing-hosted-by-a-stair/🦀️.rs"]
                            mod tests_adds_a_railing_hosted_by_a_stair;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧶️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧷️hosted-railing-with-a-path/🦀️.rs"]
                            mod tests_hosted_railing_with_a_path;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧸️negative-host-inset/🦀️.rs"]
                            mod tests_negative_host_inset;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️create-railing/🧪️tests/🧺️slab-edge-missing/🦀️.rs"]
                            mod tests_slab_edge_missing;
                        }
                        #[path = "."]
                        pub mod delete_railing {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-railing/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-railing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-railing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-railing/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-railing/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚️delete-railing/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_railing {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/✅️reshapes/🦀️.rs"]
                            mod tests_reshapes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🏷️renames-only/🦀️.rs"]
                            mod tests_renames_only;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/💤️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/📏️path-too-short/🦀️.rs"]
                            mod tests_path_too_short;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/💥️non-positive-height/🦀️.rs"]
                            mod tests_non_positive_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🚧️non-positive-post-spacing/🦀️.rs"]
                            mod tests_non_positive_post_spacing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧱️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧮️adds-balusters-and-glass/🦀️.rs"]
                            mod tests_adds_balusters_and_glass;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧯️removes-the-balusters/🦀️.rs"]
                            mod tests_removes_the_balusters;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧰️post-profile-degenerate/🦀️.rs"]
                            mod tests_post_profile_degenerate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧲️hosts-on-a-stair/🦀️.rs"]
                            mod tests_hosts_on_a_stair;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧳️hosts-on-a-ramp/🦀️.rs"]
                            mod tests_hosts_on_a_ramp;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧴️hosts-on-a-slab-edge/🦀️.rs"]
                            mod tests_hosts_on_a_slab_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧶️releases-the-host/🦀️.rs"]
                            mod tests_releases_the_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧷️moves-to-another-host/🦀️.rs"]
                            mod tests_moves_to_another_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧸️host-without-clearing-the-path/🦀️.rs"]
                            mod tests_host_without_clearing_the_path;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧺️releasing-without-a-path/🦀️.rs"]
                            mod tests_releasing_without_a_path;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧻️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🧼️host-is-no-stair-ramp-or-slab/🦀️.rs"]
                            mod tests_host_is_no_stair_ramp_or_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🪐️slab-edge-missing/🦀️.rs"]
                            mod tests_slab_edge_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🪒️slab-edge-curved/🦀️.rs"]
                            mod tests_slab_edge_curved;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🪓️stair-host-with-an-edge-index/🦀️.rs"]
                            mod tests_stair_host_with_an_edge_index;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️set-railing/🧪️tests/🪔️negative-host-inset/🦀️.rs"]
                            mod tests_negative_host_inset;
                        }
                        #[path = "."]
                        pub mod create_space {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/✅️adds-a-bounded-space/🦀️.rs"]
                            mod tests_adds_a_bounded_space;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/📐️adds-an-explicit-space/🦀️.rs"]
                            mod tests_adds_an_explicit_space;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/🔁️reuses-a-number-on-another-storey/🦀️.rs"]
                            mod tests_reuses_a_number_on_another_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/🔢️number-taken/🦀️.rs"]
                            mod tests_number_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/💥️outline-degenerate/🦀️.rs"]
                            mod tests_outline_degenerate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/🎨️adds-a-finished-space/🦀️.rs"]
                            mod tests_adds_a_finished_space;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/🏘️zone-missing/🦀️.rs"]
                            mod tests_zone_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️create-space/🧪️tests/🧱️finish-material-missing/🦀️.rs"]
                            mod tests_finish_material_missing;
                        }
                        #[path = "."]
                        pub mod delete_space {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-space/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-space/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-space/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-space/🧪️tests/🌡️removes-its-conditions/🦀️.rs"]
                            mod tests_removes_its_conditions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-space/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-space/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️delete-space/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_space {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/✅️renames-and-retypes/🦀️.rs"]
                            mod tests_renames_and_retypes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/📐️redraws-the-boundary/🦀️.rs"]
                            mod tests_redraws_the_boundary;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🔢️renumbers/🦀️.rs"]
                            mod tests_renumbers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/💤️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🚫️number-taken/🦀️.rs"]
                            mod tests_number_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/💥️outline-degenerate/🦀️.rs"]
                            mod tests_outline_degenerate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🏘️assigns-a-zone/🦀️.rs"]
                            mod tests_assigns_a_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🚶️moves-to-another-zone/🦀️.rs"]
                            mod tests_moves_to_another_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🚮️clears-the-zone/🦀️.rs"]
                            mod tests_clears_the_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🎨️finishes-the-room/🦀️.rs"]
                            mod tests_finishes_the_room;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🧼️clears-a-finish/🦀️.rs"]
                            mod tests_clears_a_finish;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/📌️zone-already-set/🦀️.rs"]
                            mod tests_zone_already_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🗺️zone-missing/🦀️.rs"]
                            mod tests_zone_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑️set-space/🧪️tests/🧱️finish-material-missing/🦀️.rs"]
                            mod tests_finish_material_missing;
                        }
                        #[path = "."]
                        pub mod create_column {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/✅️adds-to-the-storey-top/🦀️.rs"]
                            mod tests_adds_to_the_storey_top;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/✨️spans-into-the-storey-above/🦀️.rs"]
                            mod tests_spans_into_the_storey_above;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🧲️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🧭️top-storey-missing/🦀️.rs"]
                            mod tests_top_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🧩️top-storey-other-building/🦀️.rs"]
                            mod tests_top_storey_other_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🔻️top-below-base/🦀️.rs"]
                            mod tests_top_below_base;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🛑️zero-height/🦀️.rs"]
                            mod tests_zero_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/📌️authored-top-is-stored/🦀️.rs"]
                            mod tests_authored_top_is_stored;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🧮️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/💪️adds-a-leaning-column/🦀️.rs"]
                            mod tests_adds_a_leaning_column;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️create-column/🧪️tests/🧱️tilt-too-steep/🦀️.rs"]
                            mod tests_tilt_too_steep;
                        }
                        #[path = "."]
                        pub mod delete_column {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦️delete-column/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦️delete-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦️delete-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦️delete-column/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦️delete-column/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦️delete-column/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_column {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/✅️retypes-moves-and-renames/🦀️.rs"]
                            mod tests_retypes_moves_and_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/✨️constrains-the-top/🦀️.rs"]
                            mod tests_constrains_the_top;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/⚖️keeps-equal-fields-out-of-the-diff/🦀️.rs"]
                            mod tests_keeps_equal_fields_out_of_the_diff;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/🧲️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/🧭️top-storey-missing/🦀️.rs"]
                            mod tests_top_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/🧩️top-storey-other-building/🦀️.rs"]
                            mod tests_top_storey_other_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/🔻️top-below-base/🦀️.rs"]
                            mod tests_top_below_base;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/🛑️base-above-top/🦀️.rs"]
                            mod tests_base_above_top;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️set-column/🧪️tests/⚠️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                        }
                        #[path = "."]
                        pub mod create_beam {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/✅️adds-below-the-storey-top/🦀️.rs"]
                            mod tests_adds_below_the_storey_top;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/🧲️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/🛑️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/📌️authored-top-offset-is-stored/🦀️.rs"]
                            mod tests_authored_top_offset_is_stored;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/💪️adds-an-arc-beam/🦀️.rs"]
                            mod tests_adds_an_arc_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/➕️adds-an-inclined-beam/🦀️.rs"]
                            mod tests_adds_an_inclined_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️create-beam/🧪️tests/🧱️flat-arc/🦀️.rs"]
                            mod tests_flat_arc;
                        }
                        #[path = "."]
                        pub mod delete_beam {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-beam/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-beam/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-beam/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-beam/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-beam/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-beam/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod create_slab {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/✅️adds-a-slab-with-a-hole/🦀️.rs"]
                            mod tests_adds_a_slab_with_a_hole;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/➕️adds-a-sloped-curved-slab/🦀️.rs"]
                            mod tests_adds_a_sloped_curved_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/❌️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/🛑️too-few-vertices/🦀️.rs"]
                            mod tests_too_few_vertices;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/🚷️zero-area/🦀️.rs"]
                            mod tests_zero_area;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/🙅️self-intersecting/🦀️.rs"]
                            mod tests_self_intersecting;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/📛️clockwise/🦀️.rs"]
                            mod tests_clockwise;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/🚧️hole-outside/🦀️.rs"]
                            mod tests_hole_outside;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⬜️create-slab/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_slab {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🧪️tests/🧮️takes-its-hosted-railing-with-it/🦀️.rs"]
                            mod tests_takes_its_hosted_railing_with_it;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🧪️tests/🧯️attached-by-the-top-of-a-wall/🦀️.rs"]
                            mod tests_attached_by_the_top_of_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔻️delete-slab/🧪️tests/🧰️attached-by-the-base-of-a-wall/🦀️.rs"]
                            mod tests_attached_by_the_base_of_a_wall;
                        }
                        #[path = "."]
                        pub mod set_slab_boundary {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🧪️tests/✅️reshapes/🦀️.rs"]
                            mod tests_reshapes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🧪️tests/➕️drops-the-hole/🦀️.rs"]
                            mod tests_drops_the_hole;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🧪️tests/⛔️hole-outside/🦀️.rs"]
                            mod tests_hole_outside;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🧪️tests/❌️overlapping-holes/🦀️.rs"]
                            mod tests_overlapping_holes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🧪️tests/🛑️self-intersecting/🦀️.rs"]
                            mod tests_self_intersecting;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔷️set-slab-boundary/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_slab {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/✅️retypes-and-offsets/🦀️.rs"]
                            mod tests_retypes_and_offsets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/➕️slopes/🦀️.rs"]
                            mod tests_slopes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/✨️clears-the-slope/🦀️.rs"]
                            mod tests_clears_the_slope;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/👍️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/❌️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/🛑️slope-too-steep/🦀️.rs"]
                            mod tests_slope_too_steep;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔸️set-slab/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_roof {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/✅️adds-a-gable/🦀️.rs"]
                            mod tests_adds_a_gable;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/➕️adds-a-flat-roof/🦀️.rs"]
                            mod tests_adds_a_flat_roof;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/✨️adds-a-mansard/🦀️.rs"]
                            mod tests_adds_a_mansard;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/❌️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/🛑️self-intersecting/🦀️.rs"]
                            mod tests_self_intersecting;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/🚷️pitch-zero/🦀️.rs"]
                            mod tests_pitch_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/🙅️pitch-too-steep/🦀️.rs"]
                            mod tests_pitch_too_steep;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/📛️negative-overhang/🦀️.rs"]
                            mod tests_negative_overhang;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏠️create-roof/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_roof {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️delete-roof/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️delete-roof/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️delete-roof/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️delete-roof/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️delete-roof/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️delete-roof/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏘️delete-roof/🧪️tests/🧮️attached-by-a-wall/🦀️.rs"]
                            mod tests_attached_by_a_wall;
                        }
                        #[path = "."]
                        pub mod set_roof_footprint {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🧪️tests/✅️reshapes/🦀️.rs"]
                            mod tests_reshapes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🧪️tests/➕️curves-an-edge/🦀️.rs"]
                            mod tests_curves_an_edge;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🧪️tests/⛔️too-few-vertices/🦀️.rs"]
                            mod tests_too_few_vertices;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🧪️tests/❌️self-intersecting/🦀️.rs"]
                            mod tests_self_intersecting;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👣️set-roof-footprint/🧪️tests/🛑️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_roof_shape {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🧪️tests/✅️hips-the-roof/🦀️.rs"]
                            mod tests_hips_the_roof;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🧪️tests/➕️overhangs-and-lifts/🦀️.rs"]
                            mod tests_overhangs_and_lifts;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🧪️tests/❌️pitch-too-flat/🦀️.rs"]
                            mod tests_pitch_too_flat;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🧪️tests/🛑️negative-overhang/🦀️.rs"]
                            mod tests_negative_overhang;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️set-roof-shape/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_column_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🧪️tests/✅️adds-a-round-column/🦀️.rs"]
                            mod tests_adds_a_round_column;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🧪️tests/🧲️adds-an-l-shaped-outline/🦀️.rs"]
                            mod tests_adds_an_l_shaped_outline;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🧪️tests/🛑️non-positive-profile/🦀️.rs"]
                            mod tests_non_positive_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🧪️tests/❌️clockwise-outline/🦀️.rs"]
                            mod tests_clockwise_outline;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗼️create-column-type/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_column_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏺️delete-column-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏺️delete-column-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏺️delete-column-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏺️delete-column-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏺️delete-column-type/🧪️tests/🚫️in-use/🦀️.rs"]
                            mod tests_in_use;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏺️delete-column-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏺️delete-column-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_column_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/🧲️swaps-the-profile/🦀️.rs"]
                            mod tests_swaps_the_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/✨️changes-every-field/🦀️.rs"]
                            mod tests_changes_every_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/🛑️invalid-profile/🦀️.rs"]
                            mod tests_invalid_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/❌️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️set-column-type/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_beam_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🧪️tests/✅️adds-an-i-shape/🦀️.rs"]
                            mod tests_adds_an_i_shape;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🧪️tests/🧲️adds-a-bulged-outline/🦀️.rs"]
                            mod tests_adds_a_bulged_outline;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🧪️tests/🛑️oversized-flanges/🦀️.rs"]
                            mod tests_oversized_flanges;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🧪️tests/❌️self-crossing-outline/🦀️.rs"]
                            mod tests_self_crossing_outline;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌉️create-beam-type/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_beam_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️delete-beam-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️delete-beam-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️delete-beam-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️delete-beam-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️delete-beam-type/🧪️tests/🚫️in-use/🦀️.rs"]
                            mod tests_in_use;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️delete-beam-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢️delete-beam-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_beam_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/🧲️swaps-the-profile/🦀️.rs"]
                            mod tests_swaps_the_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/✨️changes-every-field/🦀️.rs"]
                            mod tests_changes_every_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/🛑️invalid-profile/🦀️.rs"]
                            mod tests_invalid_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/❌️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-beam-type/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_window_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/🧲️adds-a-floor-level-window/🦀️.rs"]
                            mod tests_adds_a_floor_level_window;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/🛑️non-positive-width/🦀️.rs"]
                            mod tests_non_positive_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/❌️negative-sill/🦀️.rs"]
                            mod tests_negative_sill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/📛️no-panes/🦀️.rs"]
                            mod tests_no_panes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/🚷️non-positive-frame/🦀️.rs"]
                            mod tests_non_positive_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️create-window-type/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_window_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️delete-window-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️delete-window-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️delete-window-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️delete-window-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️delete-window-type/🧪️tests/🚫️in-use/🦀️.rs"]
                            mod tests_in_use;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️delete-window-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊️delete-window-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_window_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/✅️widens/🦀️.rs"]
                            mod tests_widens;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/🧲️lowers-the-sill-and-adds-panes/🦀️.rs"]
                            mod tests_lowers_the_sill_and_adds_panes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/✨️renames-and-reframes/🦀️.rs"]
                            mod tests_renames_and_reframes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/🛑️non-positive-width/🦀️.rs"]
                            mod tests_non_positive_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/❌️negative-sill/🦀️.rs"]
                            mod tests_negative_sill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/📛️no-panes/🦀️.rs"]
                            mod tests_no_panes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/🚷️non-positive-frame-depth/🦀️.rs"]
                            mod tests_non_positive_frame_depth;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/🚳️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔅️set-window-type/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_door_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🧪️tests/✅️adds-a-double-door/🦀️.rs"]
                            mod tests_adds_a_double_door;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🧪️tests/🛑️non-positive-height/🦀️.rs"]
                            mod tests_non_positive_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🧪️tests/❌️non-positive-frame/🦀️.rs"]
                            mod tests_non_positive_frame;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛗️create-door-type/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_door_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️delete-door-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️delete-door-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️delete-door-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️delete-door-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️delete-door-type/🧪️tests/🚫️in-use/🦀️.rs"]
                            mod tests_in_use;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️delete-door-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️delete-door-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_door_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/✅️widens/🦀️.rs"]
                            mod tests_widens;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/🧲️swaps-leaves-and-swing/🦀️.rs"]
                            mod tests_swaps_leaves_and_swing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/🛑️non-positive-height/🦀️.rs"]
                            mod tests_non_positive_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/❌️non-positive-frame-width/🦀️.rs"]
                            mod tests_non_positive_frame_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/📛️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️set-door-type/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/🧪️tests/⛔️negative-density/🦀️.rs"]
                            mod tests_negative_density;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/🧪️tests/🌈️colour-out-of-range/🦀️.rs"]
                            mod tests_colour_out_of_range;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨️create-material/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚫️used-by-a-layer/🦀️.rs"]
                            mod tests_used_by_a_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🏛️used-by-a-column-type/🦀️.rs"]
                            mod tests_used_by_a_column_type;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🛤️used-by-a-railing/🦀️.rs"]
                            mod tests_used_by_a_railing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🛋️used-by-a-space-finish/🦀️.rs"]
                            mod tests_used_by_a_space_finish;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🧭️used-by-a-ramp/🦀️.rs"]
                            mod tests_used_by_a_ramp;
                        }
                        #[path = "."]
                        pub mod set_material {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/✅️recolours-and-renames/🦀️.rs"]
                            mod tests_recolours_and_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/🌡️retunes-the-physics/🦀️.rs"]
                            mod tests_retunes_the_physics;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/🧲️recategorises/🦀️.rs"]
                            mod tests_recategorises;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/📭️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/♻️same-values/🦀️.rs"]
                            mod tests_same_values;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/🚫️negative-conductivity/🦀️.rs"]
                            mod tests_negative_conductivity;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/🌈️colour-out-of-range/🦀️.rs"]
                            mod tests_colour_out_of_range;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-material/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_wall_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🧪️tests/🕳️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🧪️tests/📉️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵️create-wall-type/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_wall_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️delete-wall-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️delete-wall-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️delete-wall-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️delete-wall-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️delete-wall-type/🧪️tests/🚫️used-by-walls/🦀️.rs"]
                            mod tests_used_by_walls;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️delete-wall-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️delete-wall-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_wall_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/🧲️restacks/🦀️.rs"]
                            mod tests_restacks;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/🧩️both-fields/🦀️.rs"]
                            mod tests_both_fields;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/📭️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/🕳️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/📉️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/👻️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔨️set-wall-type/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_slab_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🧪️tests/🕳️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🧪️tests/📉️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟫️create-slab-type/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_slab_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️delete-slab-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️delete-slab-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️delete-slab-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️delete-slab-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️delete-slab-type/🧪️tests/🚫️used-by-slabs/🦀️.rs"]
                            mod tests_used_by_slabs;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️delete-slab-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟥️delete-slab-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_slab_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/🧲️restacks/🦀️.rs"]
                            mod tests_restacks;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/🧩️both-fields/🦀️.rs"]
                            mod tests_both_fields;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/📭️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/🕳️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/📉️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/👻️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟧️set-slab-type/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_roof_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🧪️tests/🕳️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🧪️tests/📉️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛖️create-roof-type/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_roof_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛺️delete-roof-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛺️delete-roof-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛺️delete-roof-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛺️delete-roof-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛺️delete-roof-type/🧪️tests/🚫️used-by-roofs/🦀️.rs"]
                            mod tests_used_by_roofs;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛺️delete-roof-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛺️delete-roof-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_roof_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/🧲️restacks/🦀️.rs"]
                            mod tests_restacks;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/🧩️both-fields/🦀️.rs"]
                            mod tests_both_fields;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/📭️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/🕳️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/📉️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/👻️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️set-roof-type/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_opening {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/✅️adds-a-window/🦀️.rs"]
                            mod tests_adds_a_window;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/➕️adds-a-door-to-a-curtain-wall/🦀️.rs"]
                            mod tests_adds_a_door_to_a_curtain_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🟢️adds-a-void/🦀️.rs"]
                            mod tests_adds_a_void;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/⛔️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🛑️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/❌️negative-sill/🦀️.rs"]
                            mod tests_negative_sill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/❗️non-positive-width/🦀️.rs"]
                            mod tests_non_positive_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/📛️beyond-the-host-end/🦀️.rs"]
                            mod tests_beyond_the_host_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🔴️overlaps-a-neighbour/🦀️.rs"]
                            mod tests_overlaps_a_neighbour;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🧮️adds-a-window-with-a-raised-sill/🦀️.rs"]
                            mod tests_adds_a_window_with_a_raised_sill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🧯️adds-a-window-with-a-reveal/🦀️.rs"]
                            mod tests_adds_a_window_with_a_reveal;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🧰️negative-reveal-depth/🦀️.rs"]
                            mod tests_negative_reveal_depth;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️create-opening/🧪️tests/🧲️reveal-material-missing/🦀️.rs"]
                            mod tests_reveal_material_missing;
                        }
                        #[path = "."]
                        pub mod delete_opening {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓️delete-opening/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓️delete-opening/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓️delete-opening/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓️delete-opening/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓️delete-opening/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓️delete-opening/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod move_opening {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🧪️tests/✅️slides-along-the-host/🦀️.rs"]
                            mod tests_slides_along_the_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🧪️tests/➕️rehosts-to-another-wall/🦀️.rs"]
                            mod tests_rehosts_to_another_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🧪️tests/⛔️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🧪️tests/🛑️beyond-the-host-end/🦀️.rs"]
                            mod tests_beyond_the_host_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🧪️tests/❌️overlaps-a-neighbour/🦀️.rs"]
                            mod tests_overlaps_a_neighbour;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛷️move-opening/🧪️tests/❗️already-there/🦀️.rs"]
                            mod tests_already_there;
                        }
                        #[path = "."]
                        pub mod set_opening {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/✅️resizes/🦀️.rs"]
                            mod tests_resizes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/➕️clears-the-width-override/🦀️.rs"]
                            mod tests_clears_the_width_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🟢️retypes-and-flips/🦀️.rs"]
                            mod tests_retypes_and_flips;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/⛔️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🛑️negative-sill/🦀️.rs"]
                            mod tests_negative_sill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/❌️non-positive-height/🦀️.rs"]
                            mod tests_non_positive_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/❗️too-wide-for-the-host/🦀️.rs"]
                            mod tests_too_wide_for_the_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/📛️overlaps-a-neighbour/🦀️.rs"]
                            mod tests_overlaps_a_neighbour;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🔴️already-set/🦀️.rs"]
                            mod tests_already_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🧭️sets-a-sill-override/🦀️.rs"]
                            mod tests_sets_a_sill_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🧮️restores-the-type-sill/🦀️.rs"]
                            mod tests_restores_the_type_sill;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🧯️sets-a-reveal/🦀️.rs"]
                            mod tests_sets_a_reveal;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🧰️clears-the-reveal/🦀️.rs"]
                            mod tests_clears_the_reveal;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🧲️negative-reveal-depth/🦀️.rs"]
                            mod tests_negative_reveal_depth;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕹️set-opening/🧪️tests/🧳️reveal-material-missing/🦀️.rs"]
                            mod tests_reveal_material_missing;
                        }
                        #[path = "."]
                        pub mod create_stair {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/✅️adds-a-straight-flight/🦀️.rs"]
                            mod tests_adds_a_straight_flight;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/➕️adds-a-u-run-to-the-first-storey/🦀️.rs"]
                            mod tests_adds_a_u_run_to_the_first_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🛑️non-positive-width/🦀️.rs"]
                            mod tests_non_positive_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/❌️invalid-riser/🦀️.rs"]
                            mod tests_invalid_riser;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/❗️invalid-tread/🦀️.rs"]
                            mod tests_invalid_tread;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/📛️broken-flight/🦀️.rs"]
                            mod tests_broken_flight;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🔴️top-storey-missing/🦀️.rs"]
                            mod tests_top_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🧮️adds-an-open-stair-with-cut-stringers/🦀️.rs"]
                            mod tests_adds_an_open_stair_with_cut_stringers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🧯️stringer-without-size/🦀️.rs"]
                            mod tests_stringer_without_size;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🧰️nosing-longer-than-the-tread/🦀️.rs"]
                            mod tests_nosing_longer_than_the_tread;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🧲️non-positive-tread-thickness/🦀️.rs"]
                            mod tests_non_positive_tread_thickness;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧗️create-stair/🧪️tests/🧳️non-positive-landing-depth/🦀️.rs"]
                            mod tests_non_positive_landing_depth;
                        }
                        #[path = "."]
                        pub mod delete_stair {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-stair/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-stair/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-stair/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-stair/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-stair/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-stair/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨️delete-stair/🧪️tests/🧮️takes-its-hosted-railing-with-it/🦀️.rs"]
                            mod tests_takes_its_hosted_railing_with_it;
                        }
                        #[path = "."]
                        pub mod set_stair {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/✅️moves-and-widens/🦀️.rs"]
                            mod tests_moves_and_widens;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/➕️turns-into-an-l-run/🦀️.rs"]
                            mod tests_turns_into_an_l_run;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/🟢️renames-and-flattens/🦀️.rs"]
                            mod tests_renames_and_flattens;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/⛔️non-positive-width/🦀️.rs"]
                            mod tests_non_positive_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/🛑️invalid-riser/🦀️.rs"]
                            mod tests_invalid_riser;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/❌️top-storey-missing/🦀️.rs"]
                            mod tests_top_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/❗️already-set/🦀️.rs"]
                            mod tests_already_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/🧭️builds-open-risers-and-a-mono-stringer/🦀️.rs"]
                            mod tests_builds_open_risers_and_a_mono_stringer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/🧮️nosing-longer-than-the-tread/🦀️.rs"]
                            mod tests_nosing_longer_than_the_tread;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮️set-stair/🧪️tests/🧯️stringer-without-size/🦀️.rs"]
                            mod tests_stringer_without_size;
                        }
                        #[path = "."]
                        pub mod set_project_info {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🧪️tests/✅️renames-the-project/🦀️.rs"]
                            mod tests_renames_the_project;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🧪️tests/🧲️sets-the-phases/🦀️.rs"]
                            mod tests_sets_the_phases;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🧪️tests/🚫️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🧪️tests/⛔️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🧪️tests/💥️blank-phase/🦀️.rs"]
                            mod tests_blank_phase;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📇️set-project-info/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod set_site {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/✅️moves-the-site/🦀️.rs"]
                            mod tests_moves_the_site;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/🧲️outlines-the-boundary/🦀️.rs"]
                            mod tests_outlines_the_boundary;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/🧵️clears-the-boundary/🦀️.rs"]
                            mod tests_clears_the_boundary;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/🚫️latitude-out-of-range/🦀️.rs"]
                            mod tests_latitude_out_of_range;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/💥️longitude-out-of-range/🦀️.rs"]
                            mod tests_longitude_out_of_range;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/🔒️open-boundary/🦀️.rs"]
                            mod tests_open_boundary;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/🔁️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/🪜️raises-the-absolute-levels/🦀️.rs"]
                            mod tests_raises_the_absolute_levels;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-site/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod set_building {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🧪️tests/✅️raises-the-datum/🦀️.rs"]
                            mod tests_raises_the_datum;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🧪️tests/🧲️places-the-building/🦀️.rs"]
                            mod tests_places_the_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🧪️tests/🔁️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🧪️tests/🪜️raises-the-absolute-levels/🦀️.rs"]
                            mod tests_raises_the_absolute_levels;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️set-building/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod create_grid_line {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/✅️adds-axis-a/🦀️.rs"]
                            mod tests_adds_axis_a;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/🧲️same-label-other-building/🦀️.rs"]
                            mod tests_same_label_other_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/🚫️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/⛔️building-missing/🦀️.rs"]
                            mod tests_building_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/💥️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/🔒️label-taken/🦀️.rs"]
                            mod tests_label_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/🔁️blank-label/🦀️.rs"]
                            mod tests_blank_label;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲️create-grid-line/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod delete_grid_line {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️delete-grid-line/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️delete-grid-line/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️delete-grid-line/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️delete-grid-line/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️delete-grid-line/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔳️delete-grid-line/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_grid_line {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🧪️tests/✅️relabels/🦀️.rs"]
                            mod tests_relabels;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🧪️tests/🧲️moves-the-end/🦀️.rs"]
                            mod tests_moves_the_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🧪️tests/🚫️label-taken/🦀️.rs"]
                            mod tests_label_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🧪️tests/💥️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🧪️tests/🔁️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-grid-line/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                        }
                        #[path = "."]
                        pub mod wall_geometry {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉️wall-geometry/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉️wall-geometry/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[path = "."]
                        pub mod set_wall_axis {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/✅️stretches-under-an-opening/🦀️.rs"]
                            mod tests_stretches_under_an_opening;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/🌀️curves-the-wall/🦀️.rs"]
                            mod tests_curves_the_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/🧵️straightens-the-arc/🦀️.rs"]
                            mod tests_straightens_the_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/🚫️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/⛔️flat-arc/🦀️.rs"]
                            mod tests_flat_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/〰️set-wall-axis/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod set_wall_base_offset {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/🧪️tests/✅️raises-the-base/🦀️.rs"]
                            mod tests_raises_the_base;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/🧪️tests/🌟️sinks-below-the-floor/🦀️.rs"]
                            mod tests_sinks_below_the_floor;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔽️set-wall-base-offset/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod set_wall_type_of {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/🧪️tests/✅️retypes-the-wall/🦀️.rs"]
                            mod tests_retypes_the_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/🧪️tests/🚫️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥞️set-wall-type-of/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod set_wall_location {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/🧪️tests/✅️moves-to-the-exterior-face/🦀️.rs"]
                            mod tests_moves_to_the_exterior_face;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/🧪️tests/🌟️moves-to-the-core-centre/🦀️.rs"]
                            mod tests_moves_to_the_core_centre;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦚️set-wall-location/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod flip_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/🧪️tests/✅️reverses-a-line/🦀️.rs"]
                            mod tests_reverses_a_line;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/🧪️tests/🌀️reverses-an-arc/🦀️.rs"]
                            mod tests_reverses_an_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/🧪️tests/🧲️pointlike-wall/🦀️.rs"]
                            mod tests_pointlike_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞️flip-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod split_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/✅️splits-a-straight-wall/🦀️.rs"]
                            mod tests_splits_a_straight_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🌀️splits-an-arc/🦀️.rs"]
                            mod tests_splits_an_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🪝️hands-openings-over/🦀️.rs"]
                            mod tests_hands_openings_over;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🚫️at-the-start/🦀️.rs"]
                            mod tests_at_the_start;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/⛔️beyond-the-end/🦀️.rs"]
                            mod tests_beyond_the_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🧩️new-id-taken/🦀️.rs"]
                            mod tests_new_id_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🧮️repeats-the-sweeps-on-the-new-wall/🦀️.rs"]
                            mod tests_repeats_the_sweeps_on_the_new_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈️split-wall/🧪️tests/🧯️sweep-copy-id-taken/🦀️.rs"]
                            mod tests_sweep_copy_id_taken;
                        }
                        #[path = "."]
                        pub mod delete_curtain_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🧪️tests/🧭️cascades-the-opening/🦀️.rs"]
                            mod tests_cascades_the_opening;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🧪️tests/🧮️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦖️delete-curtain-wall/🧪️tests/💪️cascades-its-overrides/🦀️.rs"]
                            mod tests_cascades_its_overrides;
                        }
                        #[path = "."]
                        pub mod move_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🧪️tests/✅️moves-a-wall-and-its-opening/🦀️.rs"]
                            mod tests_moves_a_wall_and_its_opening;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🧪️tests/🚛️moves-every-placed-kind/🦀️.rs"]
                            mod tests_moves_every_placed_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🧪️tests/⛔️empty-selection/🦀️.rs"]
                            mod tests_empty_selection;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🧪️tests/🛑️zero-vector/🦀️.rs"]
                            mod tests_zero_vector;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🧪️tests/🚧️storey-has-no-placement/🦀️.rs"]
                            mod tests_storey_has_no_placement;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-elements/🧪️tests/🧭️moves-a-ramp/🦀️.rs"]
                            mod tests_moves_a_ramp;
                        }
                        #[path = "."]
                        pub mod rotate_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/✅️turns-a-wall-and-a-column/🦀️.rs"]
                            mod tests_turns_a_wall_and_a_column;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/🚛️turns-every-placed-kind/🦀️.rs"]
                            mod tests_turns_every_placed_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/🌀️turns-by-thirty-degrees/🦀️.rs"]
                            mod tests_turns_by_thirty_degrees;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/⛔️empty-selection/🦀️.rs"]
                            mod tests_empty_selection;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/🛑️zero-angle/🦀️.rs"]
                            mod tests_zero_angle;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/🚧️storey-has-no-placement/🦀️.rs"]
                            mod tests_storey_has_no_placement;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎡️rotate-elements/🧪️tests/🧭️turns-a-ramp/🦀️.rs"]
                            mod tests_turns_a_ramp;
                        }
                        #[path = "."]
                        pub mod place_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/🧪️tests/✅️sets-absolute-placements/🦀️.rs"]
                            mod tests_sets_absolute_placements;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/🧪️tests/🚧️kind-mismatch/🦀️.rs"]
                            mod tests_kind_mismatch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/🧪️tests/🛑️same-placement/🦀️.rs"]
                            mod tests_same_placement;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧️place-elements/🧪️tests/⛔️empty-map/🦀️.rs"]
                            mod tests_empty_map;
                        }
                        #[path = "."]
                        pub mod delete_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🧪️tests/✅️cascades-openings-and-data/🦀️.rs"]
                            mod tests_cascades_openings_and_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🧪️tests/🚛️removes-furnishing-kinds/🦀️.rs"]
                            mod tests_removes_furnishing_kinds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🧪️tests/🌀️removes-a-storey-with-everything-on-it/🦀️.rs"]
                            mod tests_removes_a_storey_with_everything_on_it;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🧪️tests/⛔️empty-selection/🦀️.rs"]
                            mod tests_empty_selection;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🧪️tests/🚧️pinned-storey/🦀️.rs"]
                            mod tests_pinned_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💣️delete-elements/🧪️tests/🧭️removes-a-wall-with-its-sweeps/🦀️.rs"]
                            mod tests_removes_a_wall_with_its_sweeps;
                        }
                        #[path = "."]
                        pub mod rename_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-element/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-element/🧪️tests/✅️renames-a-wall/🦀️.rs"]
                            mod tests_renames_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-element/🧪️tests/🚛️relabels-a-grid-line/🦀️.rs"]
                            mod tests_relabels_a_grid_line;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-element/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️rename-element/🧪️tests/🛑️same-name/🦀️.rs"]
                            mod tests_same_name;
                        }
                        #[path = "."]
                        pub mod set_element_property {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/✅️adds-the-first-property/🦀️.rs"]
                            mod tests_adds_the_first_property;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/🚛️adds-to-an-existing-set/🦀️.rs"]
                            mod tests_adds_to_an_existing_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/🌀️replaces-a-value/🦀️.rs"]
                            mod tests_replaces_a_value;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/🚫️unknown-element/🦀️.rs"]
                            mod tests_unknown_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/🛑️same-value/🦀️.rs"]
                            mod tests_same_value;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/🧭️blank-property/🦀️.rs"]
                            mod tests_blank_property;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/🗂️sets-a-type-property/🦀️.rs"]
                            mod tests_sets_a_type_property;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾️set-element-property/🧪️tests/🏷️type-property-replaced/🦀️.rs"]
                            mod tests_type_property_replaced;
                        }
                        #[path = "."]
                        pub mod remove_element_property {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/🧪️tests/✅️removes-one-of-two/🦀️.rs"]
                            mod tests_removes_one_of_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/🧪️tests/🚛️removes-the-last-property/🦀️.rs"]
                            mod tests_removes_the_last_property;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/🧪️tests/🚫️missing-property/🦀️.rs"]
                            mod tests_missing_property;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/🧪️tests/⛔️unknown-element/🦀️.rs"]
                            mod tests_unknown_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫧️remove-element-property/🧪️tests/🗂️removes-a-type-property/🦀️.rs"]
                            mod tests_removes_a_type_property;
                        }
                        #[path = "."]
                        pub mod set_element_classification {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/✅️classifies-an-element/🦀️.rs"]
                            mod tests_classifies_an_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/🚛️reclassifies/🦀️.rs"]
                            mod tests_reclassifies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/🚫️unknown-element/🦀️.rs"]
                            mod tests_unknown_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/🚧️empty-code/🦀️.rs"]
                            mod tests_empty_code;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/🛑️same-classification/🦀️.rs"]
                            mod tests_same_classification;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/🧩️adds-another-system/🦀️.rs"]
                            mod tests_adds_another_system;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/🏷️classifies-a-type/🦀️.rs"]
                            mod tests_classifies_a_type;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/📝️code-outside-the-table/🦀️.rs"]
                            mod tests_code_outside_the_table;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️set-element-classification/🧪️tests/⛔️unknown-system/🦀️.rs"]
                            mod tests_unknown_system;
                        }
                        #[path = "."]
                        pub mod remove_element_classification {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️remove-element-classification/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️remove-element-classification/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️remove-element-classification/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️remove-element-classification/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️remove-element-classification/🧪️tests/🚫️not-classified/🦀️.rs"]
                            mod tests_not_classified;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️remove-element-classification/🧪️tests/⛔️unknown-element/🦀️.rs"]
                            mod tests_unknown_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️remove-element-classification/🧪️tests/🔢️removes-one-of-two/🦀️.rs"]
                            mod tests_removes_one_of_two;
                        }
                        #[path = "."]
                        pub mod set_storey_cut_height {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🧪️tests/✅️sets-the-cut/🦀️.rs"]
                            mod tests_sets_the_cut;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🧪️tests/➕️clears-the-cut/🦀️.rs"]
                            mod tests_clears_the_cut;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🧪️tests/🚫️non-positive/🦀️.rs"]
                            mod tests_non_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🧪️tests/🔴️already-set/🦀️.rs"]
                            mod tests_already_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️set-storey-cut-height/🧪️tests/🟢️already-default/🦀️.rs"]
                            mod tests_already_default;
                        }
                        #[path = "."]
                        pub mod create_ceiling_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/🧪️tests/❌️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎑️create-ceiling-type/🧪️tests/🛑️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                        }
                        #[path = "."]
                        pub mod delete_ceiling_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎏️delete-ceiling-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎏️delete-ceiling-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎏️delete-ceiling-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎏️delete-ceiling-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎏️delete-ceiling-type/🧪️tests/🚫️used-by-ceilings/🦀️.rs"]
                            mod tests_used_by_ceilings;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎏️delete-ceiling-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎏️delete-ceiling-type/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                        }
                        #[path = "."]
                        pub mod set_ceiling_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/➕️restacks/🦀️.rs"]
                            mod tests_restacks;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/✨️both-fields/🦀️.rs"]
                            mod tests_both_fields;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/👍️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/🚫️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/⛔️empty-layers/🦀️.rs"]
                            mod tests_empty_layers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/❌️thin-layer/🦀️.rs"]
                            mod tests_thin_layer;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/🛑️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎐️set-ceiling-type/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_ceiling {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/✅️adds-a-ceiling-with-a-hole/🦀️.rs"]
                            mod tests_adds_a_ceiling_with_a_hole;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/➕️adds-a-sloped-curved-ceiling/🦀️.rs"]
                            mod tests_adds_a_sloped_curved_ceiling;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/❌️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/🛑️too-few-vertices/🦀️.rs"]
                            mod tests_too_few_vertices;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/🚷️zero-area/🦀️.rs"]
                            mod tests_zero_area;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/🙅️self-intersecting/🦀️.rs"]
                            mod tests_self_intersecting;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/📛️clockwise/🦀️.rs"]
                            mod tests_clockwise;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️create-ceiling/🧪️tests/🚧️hole-outside/🦀️.rs"]
                            mod tests_hole_outside;
                        }
                        #[path = "."]
                        pub mod delete_ceiling {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-ceiling/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-ceiling/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-ceiling/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-ceiling/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-ceiling/🧪️tests/➕️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-ceiling/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌆️delete-ceiling/🧪️tests/🧭️attached-by-a-wall/🦀️.rs"]
                            mod tests_attached_by_a_wall;
                        }
                        #[path = "."]
                        pub mod set_ceiling_boundary {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🧪️tests/✅️reshapes/🦀️.rs"]
                            mod tests_reshapes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🧪️tests/➕️drops-the-hole/🦀️.rs"]
                            mod tests_drops_the_hole;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🧪️tests/⛔️hole-outside/🦀️.rs"]
                            mod tests_hole_outside;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🧪️tests/❌️overlapping-holes/🦀️.rs"]
                            mod tests_overlapping_holes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🧪️tests/🛑️self-intersecting/🦀️.rs"]
                            mod tests_self_intersecting;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌇️set-ceiling-boundary/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_ceiling {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/✅️retypes-and-drops/🦀️.rs"]
                            mod tests_retypes_and_drops;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/➕️slopes/🦀️.rs"]
                            mod tests_slopes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/✨️clears-the-slope/🦀️.rs"]
                            mod tests_clears_the_slope;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/👍️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/❌️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/🛑️slope-too-steep/🦀️.rs"]
                            mod tests_slope_too_steep;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌄️set-ceiling/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_ramp {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/✅️adds-a-straight-ramp/🦀️.rs"]
                            mod tests_adds_a_straight_ramp;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/📐️adds-a-bent-ramp-with-railings/🦀️.rs"]
                            mod tests_adds_a_bent_ramp_with_railings;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/🌀️adds-a-curved-ramp-to-the-first-storey/🦀️.rs"]
                            mod tests_adds_a_curved_ramp_to_the_first_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/🧱️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/🔴️top-storey-missing/🦀️.rs"]
                            mod tests_top_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/📏️path-too-short/🦀️.rs"]
                            mod tests_path_too_short;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/🛑️non-positive-width/🦀️.rs"]
                            mod tests_non_positive_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/💥️non-positive-thickness/🦀️.rs"]
                            mod tests_non_positive_thickness;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/📉️non-positive-slope-limit/🦀️.rs"]
                            mod tests_non_positive_slope_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛝️create-ramp/🧪️tests/🚧️negative-landing/🦀️.rs"]
                            mod tests_negative_landing;
                        }
                        #[path = "."]
                        pub mod set_ramp {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/✅️reshapes/🦀️.rs"]
                            mod tests_reshapes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/🏷️renames-only/🦀️.rs"]
                            mod tests_renames_only;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/🛤️carries-railings-on-both-sides/🦀️.rs"]
                            mod tests_carries_railings_on_both_sides;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/⛰️steepens-beyond-the-limit/🦀️.rs"]
                            mod tests_steepens_beyond_the_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/💤️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/📏️path-too-short/🦀️.rs"]
                            mod tests_path_too_short;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/🛑️non-positive-width/🦀️.rs"]
                            mod tests_non_positive_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/📉️non-positive-slope-limit/🦀️.rs"]
                            mod tests_non_positive_slope_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/🧱️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛹️set-ramp/🧪️tests/🔴️top-storey-missing/🦀️.rs"]
                            mod tests_top_storey_missing;
                        }
                        #[path = "."]
                        pub mod delete_ramp {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛼️delete-ramp/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛼️delete-ramp/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛼️delete-ramp/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛼️delete-ramp/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛼️delete-ramp/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛼️delete-ramp/🧪️tests/🧭️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛼️delete-ramp/🧪️tests/🛤️takes-its-hosted-railing-with-it/🦀️.rs"]
                            mod tests_takes_its_hosted_railing_with_it;
                        }
                        #[path = "."]
                        pub mod create_view {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/✅️adds-a-plan/🦀️.rs"]
                            mod tests_adds_a_plan;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧲️adds-a-ceiling-plan/🦀️.rs"]
                            mod tests_adds_a_ceiling_plan;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧵️adds-a-section/🦀️.rs"]
                            mod tests_adds_a_section;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧴️adds-an-elevation/🦀️.rs"]
                            mod tests_adds_an_elevation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧳️adds-a-camera/🦀️.rs"]
                            mod tests_adds_a_camera;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧱️adds-a-configured-plan/🦀️.rs"]
                            mod tests_adds_a_configured_plan;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧰️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧯️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧮️building-missing/🦀️.rs"]
                            mod tests_building_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧭️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧩️storey-of-another-building/🦀️.rs"]
                            mod tests_storey_of_another_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧨️plan-without-storey/🦀️.rs"]
                            mod tests_plan_without_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧧️section-without-plane/🦀️.rs"]
                            mod tests_section_without_plane;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧦️plane-without-length/🦀️.rs"]
                            mod tests_plane_without_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧥️camera-without-camera/🦀️.rs"]
                            mod tests_camera_without_camera;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧤️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧣️name-taken/🦀️.rs"]
                            mod tests_name_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧢️depth-zero/🦀️.rs"]
                            mod tests_depth_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧡️unordered-hidden/🦀️.rs"]
                            mod tests_unordered_hidden;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧠️scale-zero/🦀️.rs"]
                            mod tests_scale_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧟️empty-crop/🦀️.rs"]
                            mod tests_empty_crop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📽️create-view/🧪️tests/🧞️cut-height-on-a-section/🦀️.rs"]
                            mod tests_cut_height_on_a_section;
                        }
                        #[path = "."]
                        pub mod set_view {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧲️moves-the-plane/🦀️.rs"]
                            mod tests_moves_the_plane;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧵️retargets-the-plan/🦀️.rs"]
                            mod tests_retargets_the_plan;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧴️cuts-lower/🦀️.rs"]
                            mod tests_cuts_lower;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧳️clears-the-cut/🦀️.rs"]
                            mod tests_clears_the_cut;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧱️crops/🦀️.rs"]
                            mod tests_crops;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧰️clears-the-crop/🦀️.rs"]
                            mod tests_clears_the_crop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧯️hides-categories/🦀️.rs"]
                            mod tests_hides_categories;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧮️filters-by-phase/🦀️.rs"]
                            mod tests_filters_by_phase;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧭️clears-the-phase/🦀️.rs"]
                            mod tests_clears_the_phase;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧩️rescales/🦀️.rs"]
                            mod tests_rescales;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧨️orbits/🦀️.rs"]
                            mod tests_orbits;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧧️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧦️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧥️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧤️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧣️name-taken/🦀️.rs"]
                            mod tests_name_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧢️plane-on-a-plan/🦀️.rs"]
                            mod tests_plane_on_a_plan;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧡️camera-on-a-section/🦀️.rs"]
                            mod tests_camera_on_a_section;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧠️storey-on-a-section/🦀️.rs"]
                            mod tests_storey_on_a_section;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧟️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧞️storey-of-another-building/🦀️.rs"]
                            mod tests_storey_of_another_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧝️depth-zero/🦀️.rs"]
                            mod tests_depth_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧜️scale-too-large/🦀️.rs"]
                            mod tests_scale_too_large;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧛️duplicate-hidden/🦀️.rs"]
                            mod tests_duplicate_hidden;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭️set-view/🧪️tests/🧚️empty-crop/🦀️.rs"]
                            mod tests_empty_crop;
                        }
                        #[path = "."]
                        pub mod delete_view {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📺️delete-view/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📺️delete-view/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📺️delete-view/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📺️delete-view/🧪️tests/🧕️cascades-its-viewports/🦀️.rs"]
                            mod tests_cascades_its_viewports;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📺️delete-view/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📺️delete-view/🧪️tests/🧲️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📺️delete-view/🧪️tests/🧵️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_annotation_style {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🧪️tests/➕️adds-a-millimetre-style/🦀️.rs"]
                            mod tests_adds_a_millimetre_style;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🧪️tests/❌️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🧪️tests/🛑️text-height-not-positive/🦀️.rs"]
                            mod tests_text_height_not_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️create-annotation-style/🧪️tests/🚷️too-many-decimals/🦀️.rs"]
                            mod tests_too_many_decimals;
                        }
                        #[path = "."]
                        pub mod delete_annotation_style {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🧪️tests/🚫️used-by-dimensions/🦀️.rs"]
                            mod tests_used_by_dimensions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🧪️tests/⛔️used-by-tags/🦀️.rs"]
                            mod tests_used_by_tags;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🧪️tests/❌️used-by-text-notes/🦀️.rs"]
                            mod tests_used_by_text_notes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🧪️tests/🛑️used-by-leaders/🦀️.rs"]
                            mod tests_used_by_leaders;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-annotation-style/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_annotation_style {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/✅️scales-the-text/🦀️.rs"]
                            mod tests_scales_the_text;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/➕️prints-centimetres/🦀️.rs"]
                            mod tests_prints_centimetres;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/✨️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/👍️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/❌️text-height-not-positive/🦀️.rs"]
                            mod tests_text_height_not_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/🛑️too-many-decimals/🦀️.rs"]
                            mod tests_too_many_decimals;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖊️set-annotation-style/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_dimension {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/✅️dimensions-a-wall/🦀️.rs"]
                            mod tests_dimensions_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/➕️chains-an-opening-centre/🦀️.rs"]
                            mod tests_chains_an_opening_centre;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/✨️thickness-between-faces/🦀️.rs"]
                            mod tests_thickness_between_faces;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/👍️locks-a-free-span/🦀️.rs"]
                            mod tests_locks_a_free_span;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/❌️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/🛑️style-missing/🦀️.rs"]
                            mod tests_style_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/🚷️anchor-missing/🦀️.rs"]
                            mod tests_anchor_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/🙅️one-anchor/🦀️.rs"]
                            mod tests_one_anchor;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️create-dimension/🧪️tests/📛️lock-not-positive/🦀️.rs"]
                            mod tests_lock_not_positive;
                        }
                        #[path = "."]
                        pub mod delete_dimension {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️delete-dimension/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️delete-dimension/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️delete-dimension/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️delete-dimension/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎️delete-dimension/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_dimension {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/✅️moves-the-dimension-line/🦀️.rs"]
                            mod tests_moves_the_dimension_line;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/➕️re-anchors/🦀️.rs"]
                            mod tests_re_anchors;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/✨️locks-the-value/🦀️.rs"]
                            mod tests_locks_the_value;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/👍️removes-the-lock/🦀️.rs"]
                            mod tests_removes_the_lock;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/❌️style-missing/🦀️.rs"]
                            mod tests_style_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/🛑️anchor-missing/🦀️.rs"]
                            mod tests_anchor_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/🚷️one-anchor/🦀️.rs"]
                            mod tests_one_anchor;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/🙅️lock-not-positive/🦀️.rs"]
                            mod tests_lock_not_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-dimension/🧪️tests/📛️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_tag {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🧪️tests/✅️tags-a-wall/🦀️.rs"]
                            mod tests_tags_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🧪️tests/➕️tags-a-window-size/🦀️.rs"]
                            mod tests_tags_a_window_size;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🧪️tests/❌️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🧪️tests/🛑️element-missing/🦀️.rs"]
                            mod tests_element_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️create-tag/🧪️tests/🚷️style-missing/🦀️.rs"]
                            mod tests_style_missing;
                        }
                        #[path = "."]
                        pub mod delete_tag {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫️delete-tag/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫️delete-tag/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫️delete-tag/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫️delete-tag/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫️delete-tag/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_tag {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/✅️reads-the-type/🦀️.rs"]
                            mod tests_reads_the_type;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/➕️retargets-and-moves/🦀️.rs"]
                            mod tests_retargets_and_moves;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/✨️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/❌️element-missing/🦀️.rs"]
                            mod tests_element_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/🛑️style-missing/🦀️.rs"]
                            mod tests_style_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏴️set-tag/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_text_note {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🧪️tests/➕️adds-rotated/🦀️.rs"]
                            mod tests_adds_rotated;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🧪️tests/❌️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🧪️tests/🛑️style-missing/🦀️.rs"]
                            mod tests_style_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗒️create-text-note/🧪️tests/🚷️blank/🦀️.rs"]
                            mod tests_blank;
                        }
                        #[path = "."]
                        pub mod delete_text_note {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️delete-text-note/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️delete-text-note/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️delete-text-note/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️delete-text-note/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️delete-text-note/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_text_note {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/✅️rewrites/🦀️.rs"]
                            mod tests_rewrites;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/➕️moves-and-turns/🦀️.rs"]
                            mod tests_moves_and_turns;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/✨️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/❌️blank/🦀️.rs"]
                            mod tests_blank;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/🛑️style-missing/🦀️.rs"]
                            mod tests_style_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️set-text-note/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_leader {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/✅️points-at-a-wall-face/🦀️.rs"]
                            mod tests_points_at_a_wall_face;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/➕️points-at-a-free-point/🦀️.rs"]
                            mod tests_points_at_a_free_point;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/❌️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/🛑️anchor-missing/🦀️.rs"]
                            mod tests_anchor_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/🚷️style-missing/🦀️.rs"]
                            mod tests_style_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️create-leader/🧪️tests/🙅️blank/🦀️.rs"]
                            mod tests_blank;
                        }
                        #[path = "."]
                        pub mod delete_leader {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️delete-leader/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️delete-leader/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️delete-leader/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️delete-leader/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↪️delete-leader/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_leader {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/✅️rewrites/🦀️.rs"]
                            mod tests_rewrites;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/➕️re-anchors/🦀️.rs"]
                            mod tests_re_anchors;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/✨️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/❌️anchor-missing/🦀️.rs"]
                            mod tests_anchor_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/🛑️blank/🦀️.rs"]
                            mod tests_blank;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⤴️set-leader/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾️create-zone/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾️create-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾️create-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾️create-zone/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾️create-zone/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾️create-zone/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾️create-zone/🧪️tests/📉️negative-density/🦀️.rs"]
                            mod tests_negative_density;
                        }
                        #[path = "."]
                        pub mod set_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🧪️tests/✅️renames-and-recategorises/🦀️.rs"]
                            mod tests_renames_and_recategorises;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🧪️tests/👥️sets-the-density/🦀️.rs"]
                            mod tests_sets_the_density;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🧪️tests/💤️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪄️set-zone/🧪️tests/📉️negative-density/🦀️.rs"]
                            mod tests_negative_density;
                        }
                        #[path = "."]
                        pub mod delete_zone {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/🧪️tests/✅️clears-the-memberships/🦀️.rs"]
                            mod tests_clears_the_memberships;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/🧪️tests/🛖️empty-zone/🦀️.rs"]
                            mod tests_empty_zone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/🧪️tests/🗃️counted-by-an-area-scheme/🦀️.rs"]
                            mod tests_counted_by_an_area_scheme;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯️delete-zone/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_area_scheme {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🧪️tests/🎯️adds-a-restricted-scheme/🦀️.rs"]
                            mod tests_adds_a_restricted_scheme;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🧪️tests/🏘️zone-missing/🦀️.rs"]
                            mod tests_zone_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗃️create-area-scheme/🧪️tests/📝️blank-usage/🦀️.rs"]
                            mod tests_blank_usage;
                        }
                        #[path = "."]
                        pub mod set_area_scheme {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/✅️retargets-the-rule/🦀️.rs"]
                            mod tests_retargets_the_rule;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/📐️changes-the-measure/🦀️.rs"]
                            mod tests_changes_the_measure;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/♾️counts-everything/🦀️.rs"]
                            mod tests_counts_everything;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/💤️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/🏘️zone-missing/🦀️.rs"]
                            mod tests_zone_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️set-area-scheme/🧪️tests/📝️blank-usage/🦀️.rs"]
                            mod tests_blank_usage;
                        }
                        #[path = "."]
                        pub mod delete_area_scheme {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️delete-area-scheme/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️delete-area-scheme/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️delete-area-scheme/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️delete-area-scheme/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️delete-area-scheme/🧪️tests/🗂️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧻️delete-area-scheme/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🚪️adds-a-scoped-door-schedule/🦀️.rs"]
                            mod tests_adds_a_scoped_door_schedule;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🧱️adds-a-material-take-off/🦀️.rs"]
                            mod tests_adds_a_material_take_off;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/📛️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🕳️no-columns/🦀️.rs"]
                            mod tests_no_columns;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🧲️field-not-offered/🦀️.rs"]
                            mod tests_field_not_offered;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/♻️repeated-column/🦀️.rs"]
                            mod tests_repeated_column;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🔎️filter-without-a-value/🦀️.rs"]
                            mod tests_filter_without_a_value;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️create-schedule/🧪️tests/🏢️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                        }
                        #[path = "."]
                        pub mod set_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/📐️replaces-the-columns/🦀️.rs"]
                            mod tests_replaces_the_columns;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🔎️sorts-and-filters/🦀️.rs"]
                            mod tests_sorts_and_filters;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🗂️groups-and-collapses/🦀️.rs"]
                            mod tests_groups_and_collapses;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🏢️scopes-the-storeys-and-phases/🦀️.rs"]
                            mod tests_scopes_the_storeys_and_phases;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🚪️changes-the-category/🦀️.rs"]
                            mod tests_changes_the_category;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/💤️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🧲️category-drops-a-field/🦀️.rs"]
                            mod tests_category_drops_a_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🕳️no-columns/🦀️.rs"]
                            mod tests_no_columns;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/📛️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-schedule/🧪️tests/🏘️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                        }
                        #[path = "."]
                        pub mod delete_schedule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️delete-schedule/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️delete-schedule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️delete-schedule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️delete-schedule/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📉️delete-schedule/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_element_storey {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/✅️moves-a-wall-and-its-openings-follow/🦀️.rs"]
                            mod tests_moves_a_wall_and_its_openings_follow;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🏛️moves-a-column/🦀️.rs"]
                            mod tests_moves_a_column;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🛋️moves-a-room/🦀️.rs"]
                            mod tests_moves_a_room;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/⬜️moves-a-slab/🦀️.rs"]
                            mod tests_moves_a_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🚪️opening-would-break/🦀️.rs"]
                            mod tests_opening_would_break;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🔝️top-no-longer-above-the-base/🦀️.rs"]
                            mod tests_top_no_longer_above_the_base;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🚫️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🏘️other-building/🦀️.rs"]
                            mod tests_other_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🪟️opening-follows-its-host/🦀️.rs"]
                            mod tests_opening_follows_its_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/🌍️not-storey-placed/🦀️.rs"]
                            mod tests_not_storey_placed;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎢️set-element-storey/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }

                        #[path = "."]
                        pub mod set_element_phase {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/✅️demolishes-a-wall/🦀️.rs"]
                            mod tests_demolishes_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/🏚️keeps-a-demolished-wall-existing/🦀️.rs"]
                            mod tests_keeps_a_demolished_wall_existing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/🛋️phases-a-room/🦀️.rs"]
                            mod tests_phases_a_room;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/⬜️phases-a-slab/🦀️.rs"]
                            mod tests_phases_a_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/🪟️opening-takes-the-phase-of-its-host/🦀️.rs"]
                            mod tests_opening_takes_the_phase_of_its_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/🌍️carries-no-phase/🦀️.rs"]
                            mod tests_carries_no_phase;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕰️set-element-phase/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod modify {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧙️modify/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod set_wall_end_join {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🧪️tests/✅️butts-the-start/🦀️.rs"]
                            mod tests_butts_the_start;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🧪️tests/🌀️miters-the-end/🦀️.rs"]
                            mod tests_miters_the_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🧪️tests/🌟️frees-the-end/🦀️.rs"]
                            mod tests_frees_the_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🧪️tests/🚛️back-to-automatic/🦀️.rs"]
                            mod tests_back_to_automatic;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️set-wall-end-join/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod copy_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/✅️copies-a-wall-with-its-openings/🦀️.rs"]
                            mod tests_copies_a_wall_with_its_openings;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🚛️copies-every-placed-kind/🦀️.rs"]
                            mod tests_copies_every_placed_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🧲️copies-in-place/🦀️.rs"]
                            mod tests_copies_in_place;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/⛔️empty-selection/🦀️.rs"]
                            mod tests_empty_selection;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🚧️opening-alone/🦀️.rs"]
                            mod tests_opening_alone;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🪝️storey-has-no-placement/🦀️.rs"]
                            mod tests_storey_has_no_placement;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🛑️blank-prefix/🦀️.rs"]
                            mod tests_blank_prefix;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🧩️minted-id-taken/🦀️.rs"]
                            mod tests_minted_id_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️copy-elements/🧪️tests/🧭️copies-a-wall-with-its-sweeps/🦀️.rs"]
                            mod tests_copies_a_wall_with_its_sweeps;
                        }
                        #[path = "."]
                        pub mod mirror_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/✅️mirrors-walls-and-their-openings/🦀️.rs"]
                            mod tests_mirrors_walls_and_their_openings;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🌀️mirrors-an-arc-wall-and-a-curtain-wall/🦀️.rs"]
                            mod tests_mirrors_an_arc_wall_and_a_curtain_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🚛️mirrors-every-placed-kind/🦀️.rs"]
                            mod tests_mirrors_every_placed_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🌟️mirrors-a-turning-stair/🦀️.rs"]
                            mod tests_mirrors_a_turning_stair;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🔗️mirrors-as-copies/🦀️.rs"]
                            mod tests_mirrors_as_copies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🚧️fixed-hand-stair/🦀️.rs"]
                            mod tests_fixed_hand_stair;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🛑️line-without-length/🦀️.rs"]
                            mod tests_line_without_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/⛔️empty-selection/🦀️.rs"]
                            mod tests_empty_selection;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️mirror-elements/🧪️tests/🧲️already-symmetric/🦀️.rs"]
                            mod tests_already_symmetric;
                        }
                        #[path = "."]
                        pub mod array_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/✅️arrays-a-column-in-a-row/🦀️.rs"]
                            mod tests_arrays_a_column_in_a_row;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/🌀️arrays-a-column-around-a-centre/🦀️.rs"]
                            mod tests_arrays_a_column_around_a_centre;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/🔗️arrays-a-wall-with-its-openings/🦀️.rs"]
                            mod tests_arrays_a_wall_with_its_openings;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/🛑️no-copies/🦀️.rs"]
                            mod tests_no_copies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/⛔️spacing-without-length/🦀️.rs"]
                            mod tests_spacing_without_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/🚧️too-many-copies/🦀️.rs"]
                            mod tests_too_many_copies;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💠️array-elements/🧪️tests/🧩️minted-id-taken/🦀️.rs"]
                            mod tests_minted_id_taken;
                        }
                        #[path = "."]
                        pub mod align_elements {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🧪️tests/✅️aligns-walls-to-a-line/🦀️.rs"]
                            mod tests_aligns_walls_to_a_line;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🧪️tests/🌟️aligns-centres/🦀️.rs"]
                            mod tests_aligns_centres;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🧪️tests/🌀️aligns-an-arc-by-its-extremes/🦀️.rs"]
                            mod tests_aligns_an_arc_by_its_extremes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🧪️tests/🧲️already-aligned/🦀️.rs"]
                            mod tests_already_aligned;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🧪️tests/🪝️openings-follow-their-host/🦀️.rs"]
                            mod tests_openings_follow_their_host;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🧪️tests/⛔️empty-selection/🦀️.rs"]
                            mod tests_empty_selection;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️align-elements/🧪️tests/🚫️unknown-id/🦀️.rs"]
                            mod tests_unknown_id;
                        }
                        #[path = "."]
                        pub mod offset_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🧪️tests/✅️offsets-to-the-left/🦀️.rs"]
                            mod tests_offsets_to_the_left;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🧪️tests/🌟️offsets-to-the-right/🦀️.rs"]
                            mod tests_offsets_to_the_right;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🧪️tests/🌀️offsets-an-arc/🦀️.rs"]
                            mod tests_offsets_an_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🧪️tests/🚧️arc-collapses/🦀️.rs"]
                            mod tests_arc_collapses;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🧪️tests/🛑️zero-distance/🦀️.rs"]
                            mod tests_zero_distance;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🧪️tests/🧩️new-id-taken/🦀️.rs"]
                            mod tests_new_id_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶️offset-wall/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod trim_extend_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/✅️trims-to-the-target/🦀️.rs"]
                            mod tests_trims_to_the_target;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🌟️extends-to-the-target/🦀️.rs"]
                            mod tests_extends_to_the_target;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🪝️keeps-openings-in-place/🦀️.rs"]
                            mod tests_keeps_openings_in_place;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🌀️extends-an-arc/🦀️.rs"]
                            mod tests_extends_an_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🚧️never-meets/🦀️.rs"]
                            mod tests_never_meets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🛑️behind-the-other-end/🦀️.rs"]
                            mod tests_behind_the_other_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🧲️already-there/🦀️.rs"]
                            mod tests_already_there;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/⛔️opening-no-longer-fits/🦀️.rs"]
                            mod tests_opening_no_longer_fits;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🧩️its-own-target/🦀️.rs"]
                            mod tests_its_own_target;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🚫️target-missing/🦀️.rs"]
                            mod tests_target_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔪️trim-extend-wall/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod split_slab {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/✅️splits-a-rectangle/🦀️.rs"]
                            mod tests_splits_a_rectangle;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🧱️sends-a-hole-with-its-piece/🦀️.rs"]
                            mod tests_sends_a_hole_with_its_piece;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🌀️splits-an-arc-outline/🦀️.rs"]
                            mod tests_splits_an_arc_outline;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🌟️splits-through-two-corners/🦀️.rs"]
                            mod tests_splits_through_two_corners;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🚫️misses-the-slab/🦀️.rs"]
                            mod tests_misses_the_slab;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🚧️crosses-a-hole/🦀️.rs"]
                            mod tests_crosses_a_hole;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/⛔️cuts-more-than-twice/🦀️.rs"]
                            mod tests_cuts_more_than_twice;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🛑️line-without-length/🦀️.rs"]
                            mod tests_line_without_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🧩️new-id-taken/🦀️.rs"]
                            mod tests_new_id_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍰️split-slab/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod split_beam {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/✅️splits-a-beam/🦀️.rs"]
                            mod tests_splits_a_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/🌟️splits-a-slanted-beam/🦀️.rs"]
                            mod tests_splits_a_slanted_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/🚫️at-the-start/🦀️.rs"]
                            mod tests_at_the_start;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/⛔️beyond-the-end/🦀️.rs"]
                            mod tests_beyond_the_end;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/🧩️new-id-taken/🦀️.rs"]
                            mod tests_new_id_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/💪️splits-an-arc-beam/🦀️.rs"]
                            mod tests_splits_an_arc_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥖️split-beam/🧪️tests/➕️splits-an-inclined-beam/🦀️.rs"]
                            mod tests_splits_an_inclined_beam;
                        }
                        #[path = "."]
                        pub mod create_sheet {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/✅️adds-a-sheet/🦀️.rs"]
                            mod tests_adds_a_sheet;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧲️adds-a-portrait-a1/🦀️.rs"]
                            mod tests_adds_a_portrait_a1;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧵️adds-a-custom-paper/🦀️.rs"]
                            mod tests_adds_a_custom_paper;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧴️adds-a-filled-title-block/🦀️.rs"]
                            mod tests_adds_a_filled_title_block;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧳️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧱️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧰️number-taken/🦀️.rs"]
                            mod tests_number_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧯️blank-number/🦀️.rs"]
                            mod tests_blank_number;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧮️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧭️custom-paper-too-small/🦀️.rs"]
                            mod tests_custom_paper_too_small;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧩️custom-paper-too-large/🦀️.rs"]
                            mod tests_custom_paper_too_large;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️create-sheet/🧪️tests/🧨️date-unreadable/🦀️.rs"]
                            mod tests_date_unreadable;
                        }
                        #[path = "."]
                        pub mod set_sheet {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧲️renumbers/🦀️.rs"]
                            mod tests_renumbers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧵️resizes-the-paper/🦀️.rs"]
                            mod tests_resizes_the_paper;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧴️turns-the-sheet/🦀️.rs"]
                            mod tests_turns_the_sheet;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧳️cuts-a-custom-paper/🦀️.rs"]
                            mod tests_cuts_a_custom_paper;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧱️fills-the-title-block/🦀️.rs"]
                            mod tests_fills_the_title_block;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧰️marks-the-revision-and-scale/🦀️.rs"]
                            mod tests_marks_the_revision_and_scale;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧯️clears-the-title-block/🦀️.rs"]
                            mod tests_clears_the_title_block;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧮️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧭️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧩️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧨️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧧️number-taken/🦀️.rs"]
                            mod tests_number_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧦️blank-number/🦀️.rs"]
                            mod tests_blank_number;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧥️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧤️custom-paper-too-small/🦀️.rs"]
                            mod tests_custom_paper_too_small;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️set-sheet/🧪️tests/🧣️date-unreadable/🦀️.rs"]
                            mod tests_date_unreadable;
                        }
                        #[path = "."]
                        pub mod delete_sheet {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️delete-sheet/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️delete-sheet/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️delete-sheet/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️delete-sheet/🧪️tests/✅️cascades-viewports-and-revisions/🦀️.rs"]
                            mod tests_cascades_viewports_and_revisions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️delete-sheet/🧪️tests/🧲️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️delete-sheet/🧪️tests/🧵️removes-an-empty-sheet/🦀️.rs"]
                            mod tests_removes_an_empty_sheet;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️delete-sheet/🧪️tests/🧴️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_viewport {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/✅️places-a-plan/🦀️.rs"]
                            mod tests_places_a_plan;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧲️places-a-cropped-section/🦀️.rs"]
                            mod tests_places_a_cropped_section;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧵️places-the-same-view-twice/🦀️.rs"]
                            mod tests_places_the_same_view_twice;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧴️places-at-the-largest-scale/🦀️.rs"]
                            mod tests_places_at_the_largest_scale;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧳️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧱️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧰️sheet-missing/🦀️.rs"]
                            mod tests_sheet_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧯️view-missing/🦀️.rs"]
                            mod tests_view_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧮️camera-view/🦀️.rs"]
                            mod tests_camera_view;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧭️scale-zero/🦀️.rs"]
                            mod tests_scale_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧩️scale-too-large/🦀️.rs"]
                            mod tests_scale_too_large;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧨️empty-crop/🦀️.rs"]
                            mod tests_empty_crop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📓️create-viewport/🧪️tests/🧧️blank-label/🦀️.rs"]
                            mod tests_blank_label;
                        }
                        #[path = "."]
                        pub mod set_viewport {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧲️rescales/🦀️.rs"]
                            mod tests_rescales;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧵️shows-another-view/🦀️.rs"]
                            mod tests_shows_another_view;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧴️moves-to-another-sheet/🦀️.rs"]
                            mod tests_moves_to_another_sheet;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧳️crops/🦀️.rs"]
                            mod tests_crops;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧱️clears-the-crop/🦀️.rs"]
                            mod tests_clears_the_crop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧰️labels/🦀️.rs"]
                            mod tests_labels;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧯️clears-the-label/🦀️.rs"]
                            mod tests_clears_the_label;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧮️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧭️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧩️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧨️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧧️sheet-missing/🦀️.rs"]
                            mod tests_sheet_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧦️view-missing/🦀️.rs"]
                            mod tests_view_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧥️camera-view/🦀️.rs"]
                            mod tests_camera_view;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧤️scale-zero/🦀️.rs"]
                            mod tests_scale_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧣️scale-too-large/🦀️.rs"]
                            mod tests_scale_too_large;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧢️empty-crop/🦀️.rs"]
                            mod tests_empty_crop;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📔️set-viewport/🧪️tests/🧡️blank-label/🦀️.rs"]
                            mod tests_blank_label;
                        }
                        #[path = "."]
                        pub mod delete_viewport {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-viewport/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-viewport/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-viewport/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-viewport/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-viewport/🧪️tests/🧲️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📕️delete-viewport/🧪️tests/🧵️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_sheet_revision {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/✅️adds-a-revision/🦀️.rs"]
                            mod tests_adds_a_revision;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧲️adds-the-next-mark/🦀️.rs"]
                            mod tests_adds_the_next_mark;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧵️adds-an-undated-revision/🦀️.rs"]
                            mod tests_adds_an_undated_revision;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧴️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧳️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧱️sheet-missing/🦀️.rs"]
                            mod tests_sheet_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧰️mark-taken/🦀️.rs"]
                            mod tests_mark_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧯️blank-mark/🦀️.rs"]
                            mod tests_blank_mark;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧮️date-unreadable/🦀️.rs"]
                            mod tests_date_unreadable;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-sheet-revision/🧪️tests/🧭️blank-description/🦀️.rs"]
                            mod tests_blank_description;
                        }
                        #[path = "."]
                        pub mod set_sheet_revision {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/✅️redates/🦀️.rs"]
                            mod tests_redates;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧲️describes/🦀️.rs"]
                            mod tests_describes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧵️re-marks/🦀️.rs"]
                            mod tests_re_marks;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧴️signs/🦀️.rs"]
                            mod tests_signs;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧳️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧱️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧰️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧯️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧮️mark-taken/🦀️.rs"]
                            mod tests_mark_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧭️blank-mark/🦀️.rs"]
                            mod tests_blank_mark;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧩️date-unreadable/🦀️.rs"]
                            mod tests_date_unreadable;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📗️set-sheet-revision/🧪️tests/🧨️blank-description/🦀️.rs"]
                            mod tests_blank_description;
                        }
                        #[path = "."]
                        pub mod delete_sheet_revision {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📘️delete-sheet-revision/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📘️delete-sheet-revision/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📘️delete-sheet-revision/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📘️delete-sheet-revision/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📘️delete-sheet-revision/🧪️tests/🧲️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_beam_axis {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/✅️curves-the-beam/🦀️.rs"]
                            mod tests_curves_the_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/➕️straightens-the-arc/🦀️.rs"]
                            mod tests_straightens_the_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/✨️moves-the-beam/🦀️.rs"]
                            mod tests_moves_the_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/👍️keeps-the-inclination/🦀️.rs"]
                            mod tests_keeps_the_inclination;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/🚫️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/⛔️flat-arc/🦀️.rs"]
                            mod tests_flat_arc;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/❌️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝️set-beam-axis/🧪️tests/🛑️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_column_tilt {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/✅️leans-the-column/🦀️.rs"]
                            mod tests_leans_the_column;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/➕️re-aims-the-lean/🦀️.rs"]
                            mod tests_re_aims_the_lean;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/✨️straightens-the-column/🦀️.rs"]
                            mod tests_straightens_the_column;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/🚫️angle-zero/🦀️.rs"]
                            mod tests_angle_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/⛔️too-steep/🦀️.rs"]
                            mod tests_too_steep;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/❌️negative-angle/🦀️.rs"]
                            mod tests_negative_angle;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/🛑️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/🚷️already-plumb/🦀️.rs"]
                            mod tests_already_plumb;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗽️set-column-tilt/🧪️tests/🙅️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_curtain_wall_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/✅️adds-a-uniform-grid/🦀️.rs"]
                            mod tests_adds_a_uniform_grid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/➕️adds-explicit-lines-and-a-solid-panel/🦀️.rs"]
                            mod tests_adds_explicit_lines_and_a_solid_panel;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/✨️adds-a-door-default/🦀️.rs"]
                            mod tests_adds_a_door_default;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/❌️spacing-non-positive/🦀️.rs"]
                            mod tests_spacing_non_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/🛑️lines-not-ascending/🦀️.rs"]
                            mod tests_lines_not_ascending;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/🚷️mullion-flat/🦀️.rs"]
                            mod tests_mullion_flat;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/🙅️door-type-missing/🦀️.rs"]
                            mod tests_door_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️create-curtain-wall-type/🧪️tests/📛️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                        }
                        #[path = "."]
                        pub mod set_curtain_wall_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/➕️regrids/🦀️.rs"]
                            mod tests_regrids;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/✨️swaps-the-mullions/🦀️.rs"]
                            mod tests_swaps_the_mullions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/👍️changes-the-default-panel/🦀️.rs"]
                            mod tests_changes_the_default_panel;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/🧲️swaps-the-materials/🦀️.rs"]
                            mod tests_swaps_the_materials;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/🌟️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/🚫️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/⛔️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/❌️spacing-non-positive/🦀️.rs"]
                            mod tests_spacing_non_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/🛑️lines-not-ascending/🦀️.rs"]
                            mod tests_lines_not_ascending;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/🚷️window-type-missing/🦀️.rs"]
                            mod tests_window_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/🙅️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏤️set-curtain-wall-type/🧪️tests/📛️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod delete_curtain_wall_type {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏥️delete-curtain-wall-type/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏥️delete-curtain-wall-type/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏥️delete-curtain-wall-type/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏥️delete-curtain-wall-type/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏥️delete-curtain-wall-type/🧪️tests/🚫️used-by-curtain-walls/🦀️.rs"]
                            mod tests_used_by_curtain_walls;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏥️delete-curtain-wall-type/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_curtain_wall_type_of {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/🧪️tests/✅️retypes-the-facade/🦀️.rs"]
                            mod tests_retypes_the_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/🧪️tests/➕️keeps-the-overrides/🦀️.rs"]
                            mod tests_keeps_the_overrides;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/🧪️tests/⛔️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏦️set-curtain-wall-type-of/🧪️tests/❌️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_curtain_wall_grid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/✅️sets-explicit-lines/🦀️.rs"]
                            mod tests_sets_explicit_lines;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/➕️sets-a-uniform-spacing/🦀️.rs"]
                            mod tests_sets_a_uniform_spacing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/✨️sets-both-directions/🦀️.rs"]
                            mod tests_sets_both_directions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/👍️adds-a-line/🦀️.rs"]
                            mod tests_adds_a_line;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/🧲️removes-a-line/🦀️.rs"]
                            mod tests_removes_a_line;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/🌟️clears-the-override/🦀️.rs"]
                            mod tests_clears_the_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/💪️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/❌️already-following-the-type/🦀️.rs"]
                            mod tests_already_following_the_type;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/🛑️lines-not-ascending/🦀️.rs"]
                            mod tests_lines_not_ascending;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/🚷️line-at-the-start/🦀️.rs"]
                            mod tests_line_at_the_start;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/🙅️spacing-non-positive/🦀️.rs"]
                            mod tests_spacing_non_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏧️set-curtain-wall-grid/🧪️tests/📛️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_curtain_panel_override {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/✅️adds-a-door/🦀️.rs"]
                            mod tests_adds_a_door;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/➕️adds-a-window/🦀️.rs"]
                            mod tests_adds_a_window;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/✨️adds-a-solid-panel/🦀️.rs"]
                            mod tests_adds_a_solid_panel;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/👍️adds-an-empty-cell/🦀️.rs"]
                            mod tests_adds_an_empty_cell;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/❌️cell-taken/🦀️.rs"]
                            mod tests_cell_taken;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/🛑️curtain-wall-missing/🦀️.rs"]
                            mod tests_curtain_wall_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/🚷️door-type-missing/🦀️.rs"]
                            mod tests_door_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/🙅️window-type-missing/🦀️.rs"]
                            mod tests_window_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏨️create-curtain-panel-override/🧪️tests/📛️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                        }
                        #[path = "."]
                        pub mod set_curtain_panel_override {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🧪️tests/✅️swaps-the-panel/🦀️.rs"]
                            mod tests_swaps_the_panel;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🧪️tests/➕️empties-the-cell/🦀️.rs"]
                            mod tests_empties_the_cell;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🧪️tests/✨️another-door-type/🦀️.rs"]
                            mod tests_another_door_type;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🧪️tests/⛔️door-type-missing/🦀️.rs"]
                            mod tests_door_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🧪️tests/❌️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏩️set-curtain-panel-override/🧪️tests/🛑️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod delete_curtain_panel_override {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏪️delete-curtain-panel-override/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏪️delete-curtain-panel-override/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏪️delete-curtain-panel-override/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏪️delete-curtain-panel-override/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏪️delete-curtain-panel-override/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_beam {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/✅️retypes-and-lowers/🦀️.rs"]
                            mod tests_retypes_and_lowers;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/➕️inclines-the-beam/🦀️.rs"]
                            mod tests_inclines_the_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/✨️levels-the-beam/🦀️.rs"]
                            mod tests_levels_the_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/👍️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/🧲️keeps-equal-fields-out-of-the-diff/🦀️.rs"]
                            mod tests_keeps_equal_fields_out_of_the_diff;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/🚫️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/❌️already-level/🦀️.rs"]
                            mod tests_already_level;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/🛑️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_curtain_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/✅️moves-the-facade/🦀️.rs"]
                            mod tests_moves_the_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/➕️curves-the-facade/🦀️.rs"]
                            mod tests_curves_the_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/✨️constrains-the-top/🦀️.rs"]
                            mod tests_constrains_the_top;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/👍️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/❌️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🛑️top-storey-missing/🦀️.rs"]
                            mod tests_top_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_curtain_wall {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/✅️adds-a-facade/🦀️.rs"]
                            mod tests_adds_a_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/➕️adds-a-curved-facade/🦀️.rs"]
                            mod tests_adds_a_curved_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/✨️adds-a-facade-with-its-own-grid/🦀️.rs"]
                            mod tests_adds_a_facade_with_its_own_grid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/❌️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🛑️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🚷️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🙅️lines-not-ascending/🦀️.rs"]
                            mod tests_lines_not_ascending;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/📛️spacing-non-positive/🦀️.rs"]
                            mod tests_spacing_non_positive;
                        }
                        #[path = "."]
                        pub mod create_property_template {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/🚫️adds-the-first-template/🦀️.rs"]
                            mod tests_adds_the_first_template;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/⛔️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/🛑️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/🧭️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/💤️name-already-defined/🦀️.rs"]
                            mod tests_name_already_defined;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/📝️kind-listed-twice/🦀️.rs"]
                            mod tests_kind_listed_twice;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/🎯️property-defined-twice/🦀️.rs"]
                            mod tests_property_defined_twice;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/📉️default-breaks-the-range/🦀️.rs"]
                            mod tests_default_breaks_the_range;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/🗂️default-of-another-kind/🦀️.rs"]
                            mod tests_default_of_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰️create-property-template/🧪️tests/📌️range-on-text/🦀️.rs"]
                            mod tests_range_on_text;
                        }
                        #[path = "."]
                        pub mod set_property_template {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/✅️renames-and-retargets/🦀️.rs"]
                            mod tests_renames_and_retargets;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/🚫️replaces-the-definitions/🦀️.rs"]
                            mod tests_replaces_the_definitions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/⛔️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/🛑️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/🧭️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/💤️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/📝️name-already-defined/🦀️.rs"]
                            mod tests_name_already_defined;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/🎯️kind-listed-twice/🦀️.rs"]
                            mod tests_kind_listed_twice;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️set-property-template/🧪️tests/📉️broken-definition/🦀️.rs"]
                            mod tests_broken_definition;
                        }
                        #[path = "."]
                        pub mod delete_property_template {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️delete-property-template/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️delete-property-template/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️delete-property-template/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️delete-property-template/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️delete-property-template/🧪️tests/🚫️removes-the-last-template/🦀️.rs"]
                            mod tests_removes_the_last_template;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️delete-property-template/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_classification_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/✅️adds/🦀️.rs"]
                            mod tests_adds;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/🚫️adds-the-first-system/🦀️.rs"]
                            mod tests_adds_the_first_system;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/⛔️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/🛑️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/🧭️reserved-id/🦀️.rs"]
                            mod tests_reserved_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/💤️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/📝️blank-code/🦀️.rs"]
                            mod tests_blank_code;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/🎯️code-used-twice/🦀️.rs"]
                            mod tests_code_used_twice;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/📉️parent-missing/🦀️.rs"]
                            mod tests_parent_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️create-classification-system/🧪️tests/🗂️parents-form-a-cycle/🦀️.rs"]
                            mod tests_parents_form_a_cycle;
                        }
                        #[path = "."]
                        pub mod set_classification_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/✅️renames-and-reeditions/🦀️.rs"]
                            mod tests_renames_and_reeditions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/🚫️replaces-the-entries/🦀️.rs"]
                            mod tests_replaces_the_entries;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/⛔️sets-the-source/🦀️.rs"]
                            mod tests_sets_the_source;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/🛑️clears-the-source/🦀️.rs"]
                            mod tests_clears_the_source;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/💤️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/📝️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/🎯️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔏️set-classification-system/🧪️tests/📉️parent-missing/🦀️.rs"]
                            mod tests_parent_missing;
                        }
                        #[path = "."]
                        pub mod delete_classification_system {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐️delete-classification-system/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐️delete-classification-system/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐️delete-classification-system/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐️delete-classification-system/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐️delete-classification-system/🧪️tests/🚫️removes-its-classifications/🦀️.rs"]
                            mod tests_removes_its_classifications;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐️delete-classification-system/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_family {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/🧪️tests/✅️creates-a-table-family/🦀️.rs"]
                            mod tests_creates_a_table_family;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/🧪️tests/➕️creates-a-profile-family/🦀️.rs"]
                            mod tests_creates_a_profile_family;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-family/🧪️tests/❌️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                        }
                        #[path = "."]
                        pub mod delete_family {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/🧪️tests/✅️removes-with-its-parts/🦀️.rs"]
                            mod tests_removes_with_its_parts;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/🧪️tests/➕️removes-an-empty-family/🦀️.rs"]
                            mod tests_removes_an_empty_family;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/🧪️tests/🚫️used-by-a-column-type/🦀️.rs"]
                            mod tests_used_by_a_column_type;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/🧪️tests/⛔️used-by-a-beam-type/🦀️.rs"]
                            mod tests_used_by_a_beam_type;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪅️delete-family/🧪️tests/❌️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_family {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/➕️changes-the-category/🦀️.rs"]
                            mod tests_changes_the_category;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/✨️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/❌️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/🛑️leaves-profile-while-used/🦀️.rs"]
                            mod tests_leaves_profile_while_used;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪆️set-family/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_family_parameter {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/✅️adds-a-parameter/🦀️.rs"]
                            mod tests_adds_a_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/➕️adds-a-dependent-parameter/🦀️.rs"]
                            mod tests_adds_a_dependent_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/✨️changes-a-formula/🦀️.rs"]
                            mod tests_changes_a_formula;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/👍️changes-the-kind/🦀️.rs"]
                            mod tests_changes_the_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/❌️family-missing/🦀️.rs"]
                            mod tests_family_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/🛑️bad-name/🦀️.rs"]
                            mod tests_bad_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/🚷️formula-does-not-parse/🦀️.rs"]
                            mod tests_formula_does_not_parse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/🙅️unknown-parameter/🦀️.rs"]
                            mod tests_unknown_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/📛️closes-a-circle/🦀️.rs"]
                            mod tests_closes_a_circle;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/🚧️new-needs-a-kind/🦀️.rs"]
                            mod tests_new_needs_a_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/🧯️new-needs-a-formula/🦀️.rs"]
                            mod tests_new_needs_a_formula;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔡️set-family-parameter/🧪️tests/❗️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod remove_family_parameter {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/🧪️tests/✅️removes-an-unused-parameter/🦀️.rs"]
                            mod tests_removes_an_unused_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/🧪️tests/🚫️used-by-a-formula/🦀️.rs"]
                            mod tests_used_by_a_formula;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/🧪️tests/⛔️used-by-a-solid/🦀️.rs"]
                            mod tests_used_by_a_solid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/🧪️tests/❌️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔠️remove-family-parameter/🧪️tests/🛑️family-missing/🦀️.rs"]
                            mod tests_family_missing;
                        }
                        #[path = "."]
                        pub mod create_family_solid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/✅️extrudes-a-profile/🦀️.rs"]
                            mod tests_extrudes_a_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/➕️places-a-cuboid/🦀️.rs"]
                            mod tests_places_a_cuboid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/✨️sweeps-a-rail/🦀️.rs"]
                            mod tests_sweeps_a_rail;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/👍️revolves-a-foot/🦀️.rs"]
                            mod tests_revolves_a_foot;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/❌️family-missing/🦀️.rs"]
                            mod tests_family_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/🛑️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/🚷️formula-does-not-parse/🦀️.rs"]
                            mod tests_formula_does_not_parse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/🙅️unknown-parameter/🦀️.rs"]
                            mod tests_unknown_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔶️create-family-solid/🧪️tests/📛️polygon-too-small/🦀️.rs"]
                            mod tests_polygon_too_small;
                        }
                        #[path = "."]
                        pub mod delete_family_solid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔹️delete-family-solid/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔹️delete-family-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔹️delete-family-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔹️delete-family-solid/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔹️delete-family-solid/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_family_solid {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/✅️hides-the-solid/🦀️.rs"]
                            mod tests_hides_the_solid;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/➕️reshapes/🦀️.rs"]
                            mod tests_reshapes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/✨️moves/🦀️.rs"]
                            mod tests_moves;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/👍️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/❌️formula-does-not-parse/🦀️.rs"]
                            mod tests_formula_does_not_parse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/🛑️unknown-parameter/🦀️.rs"]
                            mod tests_unknown_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/🚷️polygon-too-small/🦀️.rs"]
                            mod tests_polygon_too_small;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/🙅️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-family-solid/🧪️tests/📛️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_wall_sweep {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/✅️adds-a-baseboard/🦀️.rs"]
                            mod tests_adds_a_baseboard;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/➕️adds-an-embedded-rail/🦀️.rs"]
                            mod tests_adds_an_embedded_rail;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/❌️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/🛑️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/🚷️profile-degenerate/🦀️.rs"]
                            mod tests_profile_degenerate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/🙅️negative-height/🦀️.rs"]
                            mod tests_negative_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛️create-wall-sweep/🧪️tests/📛️inset-swallows-the-profile/🦀️.rs"]
                            mod tests_inset_swallows_the_profile;
                        }
                        #[path = "."]
                        pub mod set_wall_sweep {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/✅️lifts-and-resizes/🦀️.rs"]
                            mod tests_lifts_and_resizes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/➕️moves-to-the-other-face/🦀️.rs"]
                            mod tests_moves_to_the_other_face;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/✨️rehosts/🦀️.rs"]
                            mod tests_rehosts;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/👍️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/❌️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/🛑️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/🚷️profile-degenerate/🦀️.rs"]
                            mod tests_profile_degenerate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/🙅️negative-height/🦀️.rs"]
                            mod tests_negative_height;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/📛️inset-swallows-the-profile/🦀️.rs"]
                            mod tests_inset_swallows_the_profile;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠️set-wall-sweep/🧪️tests/🚧️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod delete_wall_sweep {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡️delete-wall-sweep/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡️delete-wall-sweep/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡️delete-wall-sweep/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡️delete-wall-sweep/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡️delete-wall-sweep/🧪️tests/➕️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡️delete-wall-sweep/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_wall_base_slab {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🧪️tests/✅️attaches/🦀️.rs"]
                            mod tests_attaches;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🧪️tests/➕️frees-the-base/🦀️.rs"]
                            mod tests_frees_the_base;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🧪️tests/⛔️already-free/🦀️.rs"]
                            mod tests_already_free;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🧪️tests/❌️slab-missing/🦀️.rs"]
                            mod tests_slab_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🧪️tests/🛑️slab-in-another-building/🦀️.rs"]
                            mod tests_slab_in_another_building;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪔️set-wall-base-slab/🧪️tests/🚷️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_support {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-support/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-support/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-support/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_support {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-support/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-support/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-support/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_support {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-support/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-support/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-support/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_load_case {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-load-case/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-load-case/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-load-case/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_load_case {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-load-case/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-load-case/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-load-case/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_load_case {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-load-case/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-load-case/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-load-case/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_load {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-load/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-load/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️create-load/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_load {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-load/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-load/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️set-load/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_load {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-load/🦠️mutation/🦀️.rs"] mod mutation;
                            pub use mutation::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-load/🔺️diff/🦀️.rs"] pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦴️delete-load/↩️inverse/🦀️.rs"] pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_option_group {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-option-group/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-option-group/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-option-group/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-option-group/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod set_option_group {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-option-group/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-option-group/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-option-group/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-option-group/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod delete_option_group {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-option-group/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-option-group/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-option-group/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-option-group/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod create_design_option {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-design-option/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-design-option/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-design-option/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-design-option/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod set_design_option {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-design-option/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-design-option/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-design-option/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-design-option/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod delete_design_option {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-design-option/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-design-option/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-design-option/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-design-option/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod create_workset {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-workset/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-workset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-workset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️create-workset/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod set_workset {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-workset/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-workset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-workset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-workset/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod delete_workset {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-workset/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-workset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-workset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️delete-workset/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod set_element_option {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-option/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-option/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-option/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-option/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod set_element_workset {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-workset/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-workset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-workset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-element-workset/🧪️tests/✅️basic/🦀️.rs"]
                            mod tests_basic;
                        }
                        #[path = "."]
                        pub mod create_clash_set {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/✅️adds-beams-against-walls/🦀️.rs"]
                            mod tests_adds_beams_against_walls;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧲️adds-a-storey-restricted-set/🦀️.rs"]
                            mod tests_adds_a_storey_restricted_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧵️adds-a-clearance-set/🦀️.rs"]
                            mod tests_adds_a_clearance_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧴️adds-an-element-selected-set/🦀️.rs"]
                            mod tests_adds_an_element_selected_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧳️adds-a-phase-restricted-set/🦀️.rs"]
                            mod tests_adds_a_phase_restricted_set;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧱️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧰️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧯️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧮️negative-tolerance/🦀️.rs"]
                            mod tests_negative_tolerance;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧭️clearance-too-large/🦀️.rs"]
                            mod tests_clearance_too_large;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧩️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧨️element-missing/🦀️.rs"]
                            mod tests_element_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨️create-clash-set/🧪️tests/🧧️class-twice/🦀️.rs"]
                            mod tests_class_twice;
                        }
                        #[path = "."]
                        pub mod set_clash_set {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧲️retargets-side-a/🦀️.rs"]
                            mod tests_retargets_side_a;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧵️retargets-side-b/🦀️.rs"]
                            mod tests_retargets_side_b;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧴️loosens-the-tolerance/🦀️.rs"]
                            mod tests_loosens_the_tolerance;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧳️asks-for-a-clearance/🦀️.rs"]
                            mod tests_asks_for_a_clearance;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧱️drops-the-clearance/🦀️.rs"]
                            mod tests_drops_the_clearance;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧰️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧯️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧮️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧭️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧩️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧨️tolerance-too-large/🦀️.rs"]
                            mod tests_tolerance_too_large;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧧️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚥️set-clash-set/🧪️tests/🧦️element-missing/🦀️.rs"]
                            mod tests_element_missing;
                        }
                        #[path = "."]
                        pub mod delete_clash_set {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💢️delete-clash-set/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💢️delete-clash-set/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💢️delete-clash-set/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💢️delete-clash-set/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💢️delete-clash-set/🧪️tests/🧲️keeps-the-issues-raised-from-it/🦀️.rs"]
                            mod tests_keeps_the_issues_raised_from_it;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💢️delete-clash-set/🧪️tests/🧵️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/✅️adds-a-riser-limit/🦀️.rs"]
                            mod tests_adds_a_riser_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧲️adds-a-door-width/🦀️.rs"]
                            mod tests_adds_a_door_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧵️adds-a-ramp-slope/🦀️.rs"]
                            mod tests_adds_a_ramp_slope;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧴️adds-a-corridor-width/🦀️.rs"]
                            mod tests_adds_a_corridor_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧳️adds-a-compartment-area/🦀️.rs"]
                            mod tests_adds_a_compartment_area;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧱️adds-a-note-for-one-element/🦀️.rs"]
                            mod tests_adds_a_note_for_one_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧰️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧯️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧮️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧭️zero-limit/🦀️.rs"]
                            mod tests_zero_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧩️slope-too-steep/🦀️.rs"]
                            mod tests_slope_too_steep;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧨️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️create-rule/🧪️tests/🧧️element-missing/🦀️.rs"]
                            mod tests_element_missing;
                        }
                        #[path = "."]
                        pub mod set_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/✅️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧲️tightens-the-limit/🦀️.rs"]
                            mod tests_tightens_the_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧵️changes-the-check/🦀️.rs"]
                            mod tests_changes_the_check;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧴️softens-the-severity/🦀️.rs"]
                            mod tests_softens_the_severity;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧳️narrows-the-scope/🦀️.rs"]
                            mod tests_narrows_the_scope;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧱️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧰️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧯️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧮️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧭️blank-name/🦀️.rs"]
                            mod tests_blank_name;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧩️zero-limit/🦀️.rs"]
                            mod tests_zero_limit;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧨️slope-too-steep/🦀️.rs"]
                            mod tests_slope_too_steep;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️set-rule/🧪️tests/🧧️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❗️delete-rule/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❗️delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❗️delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❗️delete-rule/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❗️delete-rule/🧪️tests/🧲️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_issue {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/✅️raises-an-issue/🦀️.rs"]
                            mod tests_raises_an_issue;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧲️raises-one-from-a-clash/🦀️.rs"]
                            mod tests_raises_one_from_a_clash;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧵️raises-one-with-a-section-box/🦀️.rs"]
                            mod tests_raises_one_with_a_section_box;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧴️raises-a-bare-issue/🦀️.rs"]
                            mod tests_raises_a_bare_issue;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧳️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧱️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧰️blank-title/🦀️.rs"]
                            mod tests_blank_title;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧯️blank-author/🦀️.rs"]
                            mod tests_blank_author;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧮️unreadable-date/🦀️.rs"]
                            mod tests_unreadable_date;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧭️label-twice/🦀️.rs"]
                            mod tests_label_twice;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧩️element-missing/🦀️.rs"]
                            mod tests_element_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧨️clash-set-missing/🦀️.rs"]
                            mod tests_clash_set_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧧️clash-with-itself/🦀️.rs"]
                            mod tests_clash_with_itself;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧦️camera-without-distance/🦀️.rs"]
                            mod tests_camera_without_distance;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧥️inverted-section-box/🦀️.rs"]
                            mod tests_inverted_section_box;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚩️create-issue/🧪️tests/🧤️isolated-element-missing/🦀️.rs"]
                            mod tests_isolated_element_missing;
                        }
                        #[path = "."]
                        pub mod set_issue {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/✅️retitles/🦀️.rs"]
                            mod tests_retitles;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧲️assigns/🦀️.rs"]
                            mod tests_assigns;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧵️resolves/🦀️.rs"]
                            mod tests_resolves;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧴️raises-the-priority/🦀️.rs"]
                            mod tests_raises_the_priority;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧳️relabels/🦀️.rs"]
                            mod tests_relabels;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧱️adds-an-element/🦀️.rs"]
                            mod tests_adds_an_element;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧰️sets-the-viewpoint/🦀️.rs"]
                            mod tests_sets_the_viewpoint;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧯️clears-the-viewpoint/🦀️.rs"]
                            mod tests_clears_the_viewpoint;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧮️links-a-clash/🦀️.rs"]
                            mod tests_links_a_clash;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧭️clears-the-clash/🦀️.rs"]
                            mod tests_clears_the_clash;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧩️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧨️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧧️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧦️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧥️blank-title/🦀️.rs"]
                            mod tests_blank_title;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧤️unreadable-date/🦀️.rs"]
                            mod tests_unreadable_date;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧣️element-missing/🦀️.rs"]
                            mod tests_element_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️set-issue/🧪️tests/🧢️clash-set-missing/🦀️.rs"]
                            mod tests_clash_set_missing;
                        }
                        #[path = "."]
                        pub mod delete_issue {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📢️delete-issue/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📢️delete-issue/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📢️delete-issue/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📢️delete-issue/🧪️tests/✅️cascades-its-comments/🦀️.rs"]
                            mod tests_cascades_its_comments;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📢️delete-issue/🧪️tests/🧲️removes-an-issue-without-comments/🦀️.rs"]
                            mod tests_removes_an_issue_without_comments;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📢️delete-issue/🧪️tests/🧵️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_issue_comment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/✅️adds-a-comment/🦀️.rs"]
                            mod tests_adds_a_comment;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧲️adds-a-reply/🦀️.rs"]
                            mod tests_adds_a_reply;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧵️adds-a-dated-comment/🦀️.rs"]
                            mod tests_adds_a_dated_comment;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧴️duplicate-id/🦀️.rs"]
                            mod tests_duplicate_id;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧳️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧱️issue-missing/🦀️.rs"]
                            mod tests_issue_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧰️blank-author/🦀️.rs"]
                            mod tests_blank_author;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧯️blank-text/🦀️.rs"]
                            mod tests_blank_text;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬️create-issue-comment/🧪️tests/🧮️unreadable-date/🦀️.rs"]
                            mod tests_unreadable_date;
                        }
                        #[path = "."]
                        pub mod set_issue_comment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/✅️edits-the-text/🦀️.rs"]
                            mod tests_edits_the_text;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧲️re-dates/🦀️.rs"]
                            mod tests_re_dates;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧵️re-signs/🦀️.rs"]
                            mod tests_re_signs;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧴️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧱️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧰️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧯️blank-text/🦀️.rs"]
                            mod tests_blank_text;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧮️blank-author/🦀️.rs"]
                            mod tests_blank_author;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗨️set-issue-comment/🧪️tests/🧭️unreadable-date/🦀️.rs"]
                            mod tests_unreadable_date;
                        }
                        #[path = "."]
                        pub mod delete_issue_comment {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️delete-issue-comment/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️delete-issue-comment/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️delete-issue-comment/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️delete-issue-comment/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️delete-issue-comment/🧪️tests/🧲️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod create_component {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/✅️places-a-table/🦀️.rs"]
                            mod tests_places_a_table;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/➕️mounts-a-basin/🦀️.rs"]
                            mod tests_mounts_a_basin;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/✨️places-a-terminal/🦀️.rs"]
                            mod tests_places_a_terminal;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/👍️places-on-the-first-storey/🦀️.rs"]
                            mod tests_places_on_the_first_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/❌️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/🛑️family-missing/🦀️.rs"]
                            mod tests_family_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/🚷️profile-family/🦀️.rs"]
                            mod tests_profile_family;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/🙅️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/📛️host-is-no-wall/🦀️.rs"]
                            mod tests_host_is_no_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️create-component/🧪️tests/🚧️host-on-another-storey/🦀️.rs"]
                            mod tests_host_on_another_storey;
                        }
                        #[path = "."]
                        pub mod set_component {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/✅️moves/🦀️.rs"]
                            mod tests_moves;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/➕️raises/🦀️.rs"]
                            mod tests_raises;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/✨️turns/🦀️.rs"]
                            mod tests_turns;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/👍️mirrors/🦀️.rs"]
                            mod tests_mirrors;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🧲️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/💪️mounts-on-a-wall/🦀️.rs"]
                            mod tests_mounts_on_a_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🎉️unmounts/🦀️.rs"]
                            mod tests_unmounts;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🔥️becomes-a-terminal/🦀️.rs"]
                            mod tests_becomes_a_terminal;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🚀️clears-the-system/🦀️.rs"]
                            mod tests_clears_the_system;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🌟️swaps-the-family/🦀️.rs"]
                            mod tests_swaps_the_family;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🌈️moves-to-another-storey/🦀️.rs"]
                            mod tests_moves_to_another_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🍀️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/❌️mounted-keeps-its-storey/🦀️.rs"]
                            mod tests_mounted_keeps_its_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🛑️host-missing/🦀️.rs"]
                            mod tests_host_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🚷️host-is-no-wall/🦀️.rs"]
                            mod tests_host_is_no_wall;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🙅️host-on-another-storey/🦀️.rs"]
                            mod tests_host_on_another_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/📛️family-missing/🦀️.rs"]
                            mod tests_family_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🚧️profile-family/🦀️.rs"]
                            mod tests_profile_family;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/🧯️override-needs-its-parameter/🦀️.rs"]
                            mod tests_override_needs_its_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/❗️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁️set-component/🧪️tests/📣️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod delete_component {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️delete-component/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️delete-component/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️delete-component/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️delete-component/🧪️tests/✅️removes-with-its-overrides/🦀️.rs"]
                            mod tests_removes_with_its_overrides;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️delete-component/🧪️tests/➕️removes-a-plain-component/🦀️.rs"]
                            mod tests_removes_a_plain_component;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️delete-component/🧪️tests/✨️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚽️delete-component/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_component_override {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/✅️adds-an-override/🦀️.rs"]
                            mod tests_adds_an_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/➕️replaces-an-override/🦀️.rs"]
                            mod tests_replaces_an_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/✨️uses-another-parameter/🦀️.rs"]
                            mod tests_uses_another_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/👍️breaks-a-family-circle-by-override/🦀️.rs"]
                            mod tests_breaks_a_family_circle_by_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/⛔️component-missing/🦀️.rs"]
                            mod tests_component_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/❌️parameter-missing/🦀️.rs"]
                            mod tests_parameter_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/🛑️formula-does-not-parse/🦀️.rs"]
                            mod tests_formula_does_not_parse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/🚷️unknown-parameter/🦀️.rs"]
                            mod tests_unknown_parameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/🙅️closes-a-circle/🦀️.rs"]
                            mod tests_closes_a_circle;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/📛️closes-a-circle-by-override/🦀️.rs"]
                            mod tests_closes_a_circle_by_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿️set-component-override/🧪️tests/🚧️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                        }
                        #[path = "."]
                        pub mod remove_component_override {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️remove-component-override/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️remove-component-override/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️remove-component-override/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️remove-component-override/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️remove-component-override/🧪️tests/➕️removes-one-of-two/🦀️.rs"]
                            mod tests_removes_one_of_two;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️remove-component-override/🧪️tests/🚫️no-such-override/🦀️.rs"]
                            mod tests_no_such_override;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚰️remove-component-override/🧪️tests/⛔️component-missing/🦀️.rs"]
                            mod tests_component_missing;
                        }
                        #[path = "."]
                        pub mod create_mep_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/✅️routes-a-duct/🦀️.rs"]
                            mod tests_routes_a_duct;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/➕️routes-a-pipe-with-a-riser/🦀️.rs"]
                            mod tests_routes_a_pipe_with_a_riser;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/✨️routes-a-cable-tray/🦀️.rs"]
                            mod tests_routes_a_cable_tray;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/❌️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/🛑️duct-without-width/🦀️.rs"]
                            mod tests_duct_without_width;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/🚷️pipe-without-diameter/🦀️.rs"]
                            mod tests_pipe_without_diameter;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/🙅️path-too-short/🦀️.rs"]
                            mod tests_path_too_short;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💧️create-mep-element/🧪️tests/📛️repeated-point/🦀️.rs"]
                            mod tests_repeated_point;
                        }
                        #[path = "."]
                        pub mod set_mep_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/✅️resizes/🦀️.rs"]
                            mod tests_resizes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/➕️changes-the-section-kind/🦀️.rs"]
                            mod tests_changes_the_section_kind;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/✨️reroutes/🦀️.rs"]
                            mod tests_reroutes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/👍️changes-the-system/🦀️.rs"]
                            mod tests_changes_the_system;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/🧲️renames/🦀️.rs"]
                            mod tests_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/💪️moves-to-another-storey/🦀️.rs"]
                            mod tests_moves_to_another_storey;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/🎉️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/🚫️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/⛔️names-no-field/🦀️.rs"]
                            mod tests_names_no_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/❌️section-not-positive/🦀️.rs"]
                            mod tests_section_not_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/🛑️path-too-short/🦀️.rs"]
                            mod tests_path_too_short;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/🚷️repeated-point/🦀️.rs"]
                            mod tests_repeated_point;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/🙅️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💨️set-mep-element/🧪️tests/📛️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod delete_mep_element {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️delete-mep-element/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️delete-mep-element/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️delete-mep-element/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️delete-mep-element/🧪️tests/✅️removes/🦀️.rs"]
                            mod tests_removes;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️delete-mep-element/🧪️tests/➕️removes-its-data/🦀️.rs"]
                            mod tests_removes_its_data;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💦️delete-mep-element/🧪️tests/🚫️missing/🦀️.rs"]
                            mod tests_missing;
                        }
                        #[path = "."]
                        pub mod set_space_conditions {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/✅️creates-the-record/🦀️.rs"]
                            mod tests_creates_the_record;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧲️creates-a-full-record/🦀️.rs"]
                            mod tests_creates_a_full_record;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧵️creates-an-empty-record/🦀️.rs"]
                            mod tests_creates_an_empty_record;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧴️raises-the-set-point/🦀️.rs"]
                            mod tests_raises_the_set_point;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧳️adds-ventilation-and-loads/🦀️.rs"]
                            mod tests_adds_ventilation_and_loads;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧱️names-occupancy-and-schedule/🦀️.rs"]
                            mod tests_names_occupancy_and_schedule;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧰️clears-a-field/🦀️.rs"]
                            mod tests_clears_a_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧯️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧮️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧭️empty-patch-on-a-record/🦀️.rs"]
                            mod tests_empty_patch_on_a_record;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧩️space-missing/🦀️.rs"]
                            mod tests_space_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧨️cooling-below-heating/🦀️.rs"]
                            mod tests_cooling_below_heating;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧧️set-point-out-of-range/🦀️.rs"]
                            mod tests_set_point_out_of_range;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧦️negative-density/🦀️.rs"]
                            mod tests_negative_density;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧥️negative-ventilation/🦀️.rs"]
                            mod tests_negative_ventilation;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧤️negative-lighting/🦀️.rs"]
                            mod tests_negative_lighting;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌡️set-space-conditions/🧪️tests/🧣️blank-schedule/🦀️.rs"]
                            mod tests_blank_schedule;
                        }
                        #[path = "."]
                        pub mod remove_space_conditions {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥶️remove-space-conditions/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥶️remove-space-conditions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥶️remove-space-conditions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥶️remove-space-conditions/🧪️tests/✅️removes-the-record/🦀️.rs"]
                            mod tests_removes_the_record;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥶️remove-space-conditions/🧪️tests/🧲️removes-a-sparse-record/🦀️.rs"]
                            mod tests_removes_a_sparse_record;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥶️remove-space-conditions/🧪️tests/🧵️space-without-conditions/🦀️.rs"]
                            mod tests_space_without_conditions;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥶️remove-space-conditions/🧪️tests/🧴️space-missing/🦀️.rs"]
                            mod tests_space_missing;
                        }
                        #[path = "."]
                        pub mod set_type_thermal_data {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/✅️states-a-window/🦀️.rs"]
                            mod tests_states_a_window;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧲️states-a-door/🦀️.rs"]
                            mod tests_states_a_door;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧵️improves-the-glazing/🦀️.rs"]
                            mod tests_improves_the_glazing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧴️clears-the-g-value/🦀️.rs"]
                            mod tests_clears_the_g_value;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧳️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧱️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧰️empty-patch/🦀️.rs"]
                            mod tests_empty_patch;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧯️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧮️u-value-zero/🦀️.rs"]
                            mod tests_u_value_zero;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧭️g-value-above-one/🦀️.rs"]
                            mod tests_g_value_above_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧩️frame-fraction-one/🦀️.rs"]
                            mod tests_frame_fraction_one;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧨️door-has-no-g-value/🦀️.rs"]
                            mod tests_door_has_no_g_value;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♨️set-type-thermal-data/🧪️tests/🧧️door-has-no-frame-fraction/🦀️.rs"]
                            mod tests_door_has_no_frame_fraction;
                        }
                        //#endregion 🔖️Leaves
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

pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}

#[path = "."]
pub mod examples {
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧰️checks/🦀️.rs"]
    pub mod checks;
    #[path = "."]
    pub mod house {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏡️house/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏡️house/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod office {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️office/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️office/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
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
        pub mod commands {
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
        #[path = "."]
        pub mod modes {
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
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
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
                    pub mod world {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️world/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod plan {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️plan/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[path = "."]
pub mod render {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🖌️render/🦀️.rs"]
    mod component;
    pub use component::*;
}
