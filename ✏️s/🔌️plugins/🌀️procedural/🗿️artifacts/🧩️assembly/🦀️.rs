//! 🧩️ Assembly artifact — a WaveFunctionCollapse-style rule/slot composition engine. Authored fresh
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, packet W2-P5): unlike `generation2d`/
//! `generation3d`, this artifact never had a `🎛️apps` tree to migrate — the schema/mutations/
//! inferences tree under `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/` predates this ticket and is
//! reused as-is (never guessed); only this file plus the `✏️editor`/`👁️viewer` surfaces are new.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::ASSEMBLY_DOCUMENT_SCHEMA;

#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🦀️.rs"]
pub(crate) mod wfc_engine;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this artifact's ONE subset (`✳️any`) — `s.assembly@1/*`. Lives at the ARTIFACT root
/// (not under `✏️editor`/`👁️viewer`) so a viewer file can read it without ever importing through the
/// sibling editor module. `artifact_kind` matches `AssemblySnapshot`/`AssemblyDiff`'s own real
/// `#[artifact_schema(id = "s.assembly")]` attribute and `ASSEMBLY_DOCUMENT_SCHEMA`'s literal value —
/// grepped against the schema tree before writing this, NOT the `"s.procedural.assembly"` naming this
/// ticket's brief guessed by analogy with `generation2d`/`generation3d` (those two nest under the
/// `procedural` plugin id; assembly's own schema tree was authored with a bare `"s.assembly"` id by
/// the wave that built it, predating this ticket — followed as found, not overridden). `standard`/
/// `subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location on disk.
pub const ASSEMBLY_DIALECT: Dialect = Dialect { artifact_kind: ASSEMBLY_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — a headless rule/slot specification (the WFC SOLVE is an
/// inference, never persisted media), so `dimension`/`media_class`/`media_form` follow `energy.model`'s
/// "data" precedent (also a schema-first, app-free artifact authored under this same ticket) rather
/// than `generation2d`/`generation3d`'s `Flow` shape — assembly has no flow-graph fixture to render.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "data.assembly".into(),
        name: "Assembly".into(),
        source_format: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        component_kind: "assembly".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.txt".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧩️ModuleChild
/// 🧩️ Addresses one placeable MODULE as this document's child in the `s.stdio.semio@v1/kit` store.
/// A module is never embedded inline — `AssemblySnapshot::modules` holds handles — so every author
/// (examples, editor affordances, importers) mints the handle here rather than spelling the dialect
/// out again. `child_id` IS the module id the weight table and the rule set name.
pub fn module_child_handle(module_id: &str) -> store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot> {
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    store::ArtifactChild::new(module_id.to_string(), store::os_io::ArtifactRef { artifact_id: module_id.to_string(), dialect })
}
//#endregion 🧩️ModuleChild

//#region 🔖️Declaration
/// 🧾️ Defines `s.assembly`'s immutable runtime capability leaves — the single `schema.artifact`
/// capability this packet's brief asks for. `descriptor`/`claim` use the SAME `"s.assembly"` string
/// `ASSEMBLY_DIALECT` above derives from (verified against the schema tree, not guessed).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural.assembly")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.assembly")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.assembly")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.assembly.solve")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.assembly.solve")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.assembly@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.assembly@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"s.assembly:assembly")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "s.assembly")?)?
                .claim(ArtifactIdentityClaim::codec_extension("s.assembly", "assembly")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Assembly")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Assembly")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Montage")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Montage")?)?,
        )
}

/// 🔖️ Assembles `s.procedural.assembly`'s typed runtime declaration.
#[cfg(feature = "component-app-assembly")]
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(standards::v1::subsets::any::schema::assembly_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::assembly_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec_bare::<AssemblySnapshot, AssemblyMutation>(ASSEMBLY_DOCUMENT_SCHEMA)
        .try_build()
}

// 🚧️ declaration gap closed — deliberately, not an oversight. `ArtifactDeclaration::builder(...)
// .schema(descriptor)` (`🧰️framework/…/🔌️plugin/🦀️.rs:2883`) is typestate-MANDATORY: the
// builder cannot reach `.try_build()` without it, and `ArtifactSchemaDescriptor` needs FOUR facets
// (artifact/snapshot/diff/mutations) each carrying FIVE handcrafted `&'static str` leaves (rust/
// typescript/graphql/json_schema/proto) via `include_str!`. Verified on disk: `🧬️schema/📸️snapshot/`,
// `🔺️diff/`, `🧬️mutations/` each carry ONLY `🦀️.rs`+`🟦️.ts` — no `🔣️.json`/
// `🔗️.graphql`/`🛰️.proto` anywhere in this artifact's schema tree, and there is no
// `🧬️schema/🦀️component.rs` artifact-facet file at all (contrast `energy.model`, whose equivalent
// facet + a `energy_model_artifact_schema_descriptor()` fn were built by an EARLIER, separate wave
// before this ticket ever touched it — this ticket's own `📓️w2-cad-report.md` recipe never asks a W2
// surface packet to author schema-descriptor leaves, and this packet's brief scoped "fully real" to
// snapshot/diff/mutations/inferences specifically, not the descriptor facet). Authoring 14 new
// handcrafted GraphQL/JSON-Schema/Protobuf files for a domain this packet did not design is out of
// this packet's named scope; see `📓️w2-p5-assembly-notes.md` for the exact gap and who should close
// it. `artifact_kind()`/`definition()` above are complete and independent of this gap.
//#endregion 🔖️Declaration

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
                    #[path =  "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                    pub mod diff;
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
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🧪️tests/🧩️appends-slot-c-at-index-2/🦀️.rs"]
                            mod tests_appends_slot_c_at_index_2;
                        }
                        #[path = "."]
                        pub mod delete_slot {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🧪️tests/🚫️removes-slot-a-and-cascades-edge-ab/🦀️.rs"]
                            mod tests_removes_slot_a_and_cascades_edge_ab;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🧪️tests/⛔️appends-a-rule-forbidding-roof-over-wall/🦀️.rs"]
                            mod tests_appends_a_rule_forbidding_roof_over_wall;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/🧪️tests/🚫️removes-the-wall-roof-rule/🦀️.rs"]
                            mod tests_removes_the_wall_roof_rule;
                        }
                        #[path = "."]
                        pub mod change_weight {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-weight/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-weight/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-weight/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-weight/🧪️tests/⚖️raises-the-wall-module-selection-bias/🦀️.rs"]
                            mod tests_raises_the_wall_module_selection_bias;
                        }
                        #[path = "."]
                        pub mod remove_weight {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️remove-weight/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️remove-weight/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️remove-weight/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️remove-weight/🧪️tests/🪶️drops-the-wall-module-weight-override/🦀️.rs"]
                            mod tests_drops_the_wall_module_weight_override;
                        }
                        #[path = "."]
                        pub mod connect_slots {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🧪️tests/🔗️joins-slot-b-to-slot-c-at-index-1/🦀️.rs"]
                            mod tests_joins_slot_b_to_slot_c_at_index_1;
                        }
                        #[path = "."]
                        pub mod disconnect_slots {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🧪️tests/✂️severs-edge-ab-leaving-both-slots/🦀️.rs"]
                            mod tests_severs_edge_ab_leaving_both_slots;
                        }
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds-the-solve-from-7-to-99/🦀️.rs"]
                            mod tests_reseeds_the_solve_from_7_to_99;
                        }
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                            }
                        }
                    }
                }
            }
        }
    }
}


#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod assembly {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs"]
                    pub mod structure;
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod assembly {
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️.rs"]
                    pub mod structure;
                }
            }
        }
    }
}

//#region 📚️Examples
/// 📚️ The bundled WFC problem specs this subset ships — one forced path, one cyclic lattice. Each
/// slug owns its own directory under `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`, and the Rust
/// builder there is the authority its `🗣️.dsl.semio` asset is printed from.
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🚪️two-room-corridor/🦀️.rs"]
    pub mod two_room_corridor;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧱️wall-roof-facade-strip/🦀️.rs"]
    pub mod wall_roof_facade_strip;

    /// 📇️ Every bundled example, in the order the editor's example picker offers them.
    pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![two_room_corridor::source(), wall_roof_facade_strip::source()]
    }

    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    mod tests;
}
//#endregion 📚️Examples

// ---- Shims: flat access from the artifact root, mirroring generation2d/generation3d ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
}
pub use crate::standards::v1::subsets::any::schema::diff::AssemblyDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::AssemblyMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::AssemblySnapshot;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;
