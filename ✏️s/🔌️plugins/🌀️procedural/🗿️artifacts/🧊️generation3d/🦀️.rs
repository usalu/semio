//! 📐️ Generation3d artifact — snapshot re-exports, widget id helper, and artifact kind.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

#[cfg(feature = "component-app-assembly")]
#[path = "../../🫀️core/🖼️semantic-ui/🦀️.rs"]
mod semantic_ui;
#[cfg(feature = "component-app-assembly")]
pub(crate) use semantic_ui::*;

pub const GENERATION_3D_SCHEMA: &str = "generation.3d";

/// 🎯️ This subset's canonical dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §2.1/§7.4) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind` matches this artifact's own `s.generation3d.schema.artifact` capability descriptor
/// above (`b"s.procedural.generation3d"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
/// `s.procedural.generation3d@1/*#editor` / `s.procedural.generation3d@1/*#viewer`.
pub const GENERATION3D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️Helpers
/// 🌡️ A flow widget's stable id, across every widget variant (mirrors flow's private accessor).
pub fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}
//#endregion 🔖️Helpers

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::generation3d::create_generation3d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.generation".into(),
        name: "3D Generation".into(),
        source_format: "generation.3d".into(),
        component_kind: "generation3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        schema: "generation.3d".into(),
        export_formats: vec![],
        import_formats: vec![],
        // 🖼️ "stdio.json"/"stdio.png" stay out of exports (generation2d owns those EXPORT claims, D3)
        // but stay in imports below — see `🚪️io/🦀️.rs`'s `🚪️IoRegistry` region.
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

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
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.las")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.las@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.las@1.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.ply")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.ply@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.ply@1.0/*")?)?,
        )?
        // 🖼️ No `composer.png`/`composer.json` here: both are generic bridge dialects with no
        // real per-artifact fidelity difference (both generation2d's and generation3d's export stubs
        // are equally `print_dsl` placeholders), so generation2d — the plugin's first-declared,
        // primary 2D artifact — keeps the EXPORT claim (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME
        // D3, a documented tie-break, not evidence-backed like the DWG↔mesh-bridge split below). Import
        // still works: `reads()` on this artifact's own native composer is unaffected by this removal.
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.dwg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.dwg@ac1018/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.dwg@ac1018/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.stl")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.stl@ascii/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.stl@ascii/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.gltf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.gltf@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.gltf@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.obj")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.obj@3.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.obj@3.0/*")?)?,
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
#[cfg(feature = "component-app-assembly")]
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(standards::v1::subsets::any::schema::generation3d_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::generation3d_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<editor::generation3d::Generation3dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️flow-operators/🦀️.rs"]
pub(crate) mod flow_operators;

#[cfg(test)]
#[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]
pub(crate) mod store_fixture;

#[cfg(test)]
#[path = "🧪️tests/🔬️brep-extension/🦀️.rs"]
pub(crate) mod brep_extension;

#[cfg(test)]
#[path = "🧪️tests/🔬️publication-authority/🦀️.rs"]
pub(crate) mod publication_authority;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
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
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod delete_widget_position {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-widget-position/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-widget-position/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-widget-position/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-widget-position/🧪️tests/🧹️unpins-the-node-a-position/🦀️.rs"]
                            mod tests_unpins_the_node_a_position;
                        }
                        #[path = "."]
                        pub mod disconnect_synapse {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🧪️tests/✂️cuts-wire-ab-leaving-both-nodes/🦀️.rs"]
                            mod tests_cuts_wire_ab_leaving_both_nodes;
                        }
                        #[path = "."]
                        pub mod delete_widget {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🧪️tests/🚫️removes-node-a-and-leaves-wire-ab-dangling/🦀️.rs"]
                            mod tests_removes_node_a_and_leaves_wire_ab_dangling;
                        }
                        #[path = "."]
                        pub mod update_camera {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️update-camera/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️update-camera/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️update-camera/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️update-camera/🧪️tests/🔍️frames-the-graph-at-double-zoom/🦀️.rs"]
                            mod tests_frames_the_graph_at_double_zoom;
                        }
                        #[path = "."]
                        pub mod move_widget {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widget/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widget/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widget/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widget/🧪️tests/📍️repositions-node-a-in-the-graph/🦀️.rs"]
                            mod tests_repositions_node_a_in_the_graph;
                        }
                        #[path = "."]
                        pub mod change_schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️change-schema/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️change-schema/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️change-schema/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️change-schema/🧪️tests/🏷️restamps-the-fixture-schema-id/🦀️.rs"]
                            mod tests_restamps_the_fixture_schema_id;
                        }
                        #[path = "."]
                        pub mod update_synapse {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse/🧪️tests/📡️repoints-wire-ab-onto-the-cap-port/🦀️.rs"]
                            mod tests_repoints_wire_ab_onto_the_cap_port;
                        }
                        #[path = "."]
                        pub mod update_widget {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🧪️tests/🎚️retunes-the-knob-slider-value/🦀️.rs"]
                            mod tests_retunes_the_knob_slider_value;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod las {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod ply {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod stl {
                                    #[path = "."]
                                    pub mod v_ascii {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod gltf {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod las {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod ply {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod stl {
                                    #[path = "."]
                                    pub mod v_ascii {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod gltf {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod obj {
                                    #[path = "."]
                                    pub mod v3_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot;

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🦀️.rs"]
    pub mod art_generation3d_box_fillet_preview;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🦀️.rs"]
    pub mod art_generation3d_box_shell_preview;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🦀️.rs"]
    pub mod art_generation3d_face_sweep_extrude;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🦀️.rs"]
    pub mod art_generation3d_hexagonal_mushroom_column;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🦀️.rs"]
    pub mod art_generation3d_rectangle_extrude_volume;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🦀️.rs"]
    pub mod art_generation3d_rectangle_wire_preview;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🦀️.rs"]
    pub mod art_generation3d_sphere_box_fuse;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🦀️.rs"]
    pub mod art_generation3d_sphere_cut_with_torus;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📐️box-fillet-preview/🧪️tests/🧩️example/🦀️.rs"]
    mod box_fillet_preview_tests;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🐚️box-shell-preview/🧪️tests/🧩️example/🦀️.rs"]
    mod box_shell_preview_tests;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧹️face-sweep-extrude/🧪️tests/🧩️example/🦀️.rs"]
    mod face_sweep_extrude_tests;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍄️hexagonal-mushroom-column/🧪️tests/🧩️example/🦀️.rs"]
    mod hexagonal_mushroom_column_tests;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️rectangle-extrude-volume/🧪️tests/🧩️example/🦀️.rs"]
    mod rectangle_extrude_volume_tests;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🪢️rectangle-wire-preview/🧪️tests/🧩️example/🦀️.rs"]
    mod rectangle_wire_preview_tests;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧲️sphere-box-fuse/🧪️tests/🧩️example/🦀️.rs"]
    mod sphere_box_fuse_tests;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🧩️example/🦀️.rs"]
    mod sphere_cut_with_torus_tests;
}

/// 🧵️ The surface-neutral `flowEvalTick` chain BOTH surfaces run. Mounted at the artifact level,
/// beside `editor`/`viewer` rather than inside either, so the viewer can reach it without ever
/// importing through the sibling `editor` module (`policyViewerPurityBreaches`) — the same reason
/// [`GENERATION3D_DIALECT`] lives here (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(feature = "component-app-assembly")]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs"]
pub mod preview_eval;

#[cfg(feature = "component-app-assembly")]
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

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
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
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️cancel-preview-eval/🦀️.rs"]
            pub mod cancel_preview_eval;
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
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-flow-widgets/🦀️.rs"]
            pub mod patch_flow_widgets;
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
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[cfg(feature = "component-app-assembly")]
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
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🛑️cancel-preview-eval/🦀️.rs"]
            pub mod cancel_preview_eval;
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
