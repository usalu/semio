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
pub use crate::standards::v1::subsets::any::schema::diff::{Assigned, Entry, KeyedDelta, ModelDiff, Patch, PropertySetPatch};
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
                        pub mod set_beam {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🦠️mutation/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/✅️retypes-and-stretches/🦀️.rs"]
                            mod tests_retypes_and_stretches;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/✨️lowers-the-beam/🦀️.rs"]
                            mod tests_lowers_the_beam;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/⚖️keeps-equal-fields-out-of-the-diff/🦀️.rs"]
                            mod tests_keeps_equal_fields_out_of_the_diff;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/⛔️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/🧲️type-missing/🦀️.rs"]
                            mod tests_type_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/🛑️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-beam/🧪️tests/⚠️nothing-to-change/🦀️.rs"]
                            mod tests_nothing_to_change;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🌀️adds-a-curved-facade/🦀️.rs"]
                            mod tests_adds_a_curved_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🚫️duplicate/🦀️.rs"]
                            mod tests_duplicate;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/⛔️storey-missing/🦀️.rs"]
                            mod tests_storey_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/💥️zero-length/🦀️.rs"]
                            mod tests_zero_length;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/📏️spacing-non-positive/🦀️.rs"]
                            mod tests_spacing_non_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🧪️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏬️create-curtain-wall/🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
                            mod tests_id_taken_by_another_kind;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/✅️re-grids-the-facade/🦀️.rs"]
                            mod tests_re_grids_the_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🌀️curves-the-facade/🦀️.rs"]
                            mod tests_curves_the_facade;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🎨️swaps-materials-and-renames/🦀️.rs"]
                            mod tests_swaps_materials_and_renames;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🧲️unchanged/🦀️.rs"]
                            mod tests_unchanged;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🚫️spacing-non-positive/🦀️.rs"]
                            mod tests_spacing_non_positive;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/⛔️material-missing/🦀️.rs"]
                            mod tests_material_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🕳️missing/🦀️.rs"]
                            mod tests_missing;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            mod tests_follows_by_inference;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔆️set-curtain-wall/🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
                            mod tests_restates_an_unchanged_field;
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
        pub mod inference {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔮️inference/🦀️.rs"]
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
                }
            }
        }
        #[path = "."]
        pub mod panels {
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
