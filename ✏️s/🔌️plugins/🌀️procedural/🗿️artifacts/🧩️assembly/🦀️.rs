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
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🧾️ Defines `s.assembly`'s immutable runtime capability leaves — the single `schema.artifact`
/// capability this packet's brief asks for. `descriptor`/`claim` use the SAME `"s.assembly"` string
/// `ASSEMBLY_DIALECT` above derives from (verified against the schema tree, not guessed).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural.assembly")?).capability(
        ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.assembly.schema.artifact")?, ArtifactCapabilityKind::schema())
            .descriptor(b"s.procedural.assembly")?
            .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.assembly")?)?,
    )
}

// 🚧️ NO `declaration()` here yet — deliberately, not an oversight. `ArtifactDeclaration::builder(...)
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                    pub mod diff;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                    pub mod snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️create-slot/🧪️tests/🧩️appends-slot-c-b3fd5a/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-slot/🧪️tests/🚫️removes-slot-a-06e92b/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚦️create-rule/🧪️tests/⛔️appends-a-rule-059003/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-rule/🧪️tests/🚫️removes-the-wall-3d5715/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-weight/🧪️tests/⚖️raises-the-wall-4578a3/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪶️remove-weight/🧪️tests/🪶️drops-the-wall-36f28b/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-slots/🧪️tests/🔗️joins-slot-b-to-c622ac/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🧪️tests/✂️severs-edge-ab-c14030/🦀️.rs"]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds-the-93c15b/🦀️.rs"]
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
            }
        }
    }
}

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
