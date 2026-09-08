//! 🏗️ FEM plugin — finite-element structural analysis, bundled as a hot-swappable WASM component.
//! Two independent artifacts (`fem2d`, `fem3d`) share one cross-artifact compute kernel (`core`).
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory — `📦️packages/🦀️rust/`, two levels below the
//! owner root the taxonomy tree hangs off, hence every LEAF path's `../../` prefix. The grouping
//! modules carry a bare `#[path = "."]` so their own names are not spliced into that base directory —
//! without it, Rust resolves an inline module's children under `<file dir>/<inline mod name>/…` and
//! every leaf path dangles. A `"."` reset composes against its parent's already-resolved base, never
//! against the raw file directory, so it must NOT carry the `../../` prefix. Do not inline any
//! component file back into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint
//! both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
#[cfg(test)]
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧪️tests/🦀️.rs"]
pub(crate) mod numerical_testkit;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FemXMutation, FemXConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing
// it here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl
// itself (only on the free functions the taxonomy split creates), so this is a pure artefact of
// decomposition.
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs"]
pub mod analyses;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs"]
pub mod elements2d;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs"]
pub mod elements3d;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/➗️formulation/🦀️.rs"]
pub mod formulation;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs"]
pub mod mesh;
#[allow(clippy::result_large_err)]
//#region 🏗️Kernel modules
// 🔄️ 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: these 7 mounts moved from the artifact
// tree's (now-deleted) `⚙️engine` into a plugin-level module, `✏️s/🔨️modules/🏗️fem/⚙️engine/` — pure FE
// algorithm code (element stiffness, assembly, sparse solve, mesh generation), legitimately D6 "pure
// algorithm" and NOT snapshot-derived inference, NOT app behaviour. An artifact is a schema + io
// system, never an engine; a MODULE may still have one (`taxonomyLeafParentDirs` already lists
// `⚙️engine` globally). Mount NAMEs are unchanged (`crate::model`, `crate::analyses`, …), only the
// `#[path]` TARGET moved, so every existing `crate::model::X`-style call site elsewhere in this crate
// is unaffected.
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs"]
pub mod model;
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs"]
pub mod sparse;
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d: FEM's own
// dense-basics duplicate of `🧮️math/➕️algebra` (Mat2/VecD/MatD/Mat3d/vec3d_*) — the only slice FEM
// ever called; `crate::model`/`crate::analyses`/`crate::elements2d`/`crate::elements3d`/
// `crate::formulation`/`crate::sparse` were repointed from `math::algebra::` to `crate::algebra::`
// in the same wave. `semio-framework-math` is no longer a dependency of this crate.
#[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/➕️algebra/🦀️.rs"]
pub mod algebra;
// 🔄️ W2 packet P7 (26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): moved from
// `../../🎛️apps/◻️2d/⚙️engine/🖥️app-surface/` to a plugin-root `⚙️engine/🖥️app-surface/` — this module's
// own doc comment says it is used by BOTH `fem2d_ui` and `fem3d_ui`, so it never belonged nested under
// one app. Mount NAME is unchanged (`crate::app_surface`), only the `#[path]` TARGET moved, so every
// existing `crate::app_surface::X`-style call site elsewhere in this crate is unaffected.
#[path = "../../⚙️engine/🖥️app-surface/🦀️.rs"]
pub mod app_surface;

// 🔄️ Same ticket: the fem2d/fem3d-SPECIFIC engine content (Errors + top-level solve entry points,
// plus the artifact-specific meshing/modal-buckling/mesh-preview bridges) — also pure FE algorithm,
// also moved out of the artifact tree, but NOT shared cross-artifact so each gets its own crate-root
// module rather than joining the 7 above.
#[path = "."]
pub mod fem2d_engine {
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🗺️mesh-preview/🦀️.rs"]
    pub mod mesh_preview;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🕸️meshing/🦀️.rs"]
    pub mod meshing;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🎵️modal-buckling/🦀️.rs"]
    pub mod modal_buckling;
}
#[path = "."]
pub mod fem3d_engine {
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🗺️mesh-preview/🦀️.rs"]
    pub mod mesh_preview;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🕸️meshing/🦀️.rs"]
    pub mod meshing;
    #[path = "../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🎵️modal-buckling/🦀️.rs"]
    pub mod modal_buckling;
}

//#endregion 🏗️Kernel modules

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🗿️artifacts/◻️2d/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/🦀️.rs"]
                                    mod tests_appends_node_n3;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/🦀️.rs"]
                                    mod tests_appends_the_canopy_strut_head_node_to_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚫️rejects-a-duplicate-eb0df0/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_node_id_on_the_steel_frame;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/🦀️.rs"]
                                    mod tests_removes_node_n3_without_cascading_to_its_support;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/⛔️rejects-a-missing-429801/🦀️.rs"]
                                    mod tests_rejects_deleting_a_node_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/🦀️.rs"]
                                    mod tests_drops_the_spare_canopy_node_from_the_steel_frame;
                                }
                                #[path = "."]
                                pub mod create_element {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-bar-e2-between-fc1c09/🦀️.rs"]
                                    mod tests_appends_bar_e2_between_n2_and_n3;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/🦀️.rs"]
                                    mod tests_braces_the_upper_storey_with_a_chs_diagonal;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚫️rejects-a-dangling-b9e64c/🦀️.rs"]
                                    mod tests_rejects_an_element_whose_start_node_is_missing;
                                }
                                #[path = "."]
                                pub mod delete_element {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-bar-e2-and-3c0260/🦀️.rs"]
                                    mod tests_removes_bar_e2_and_keeps_its_end_nodes;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛔️rejects-a-missing-611215/🦀️.rs"]
                                    mod tests_rejects_deleting_an_element_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/🦀️.rs"]
                                    mod tests_cuts_the_lower_storey_bracing_diagonal;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🔗️blocks-udl-a1df8e/🦀️.rs"]
                                    mod tests_refuses_to_delete_the_floor_beam_two_member_udls_still_load;
                                }
                                #[path = "."]
                                pub mod replace_element {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts-beam-e1-into-a-5d21f5/🦀️.rs"]
                                    mod tests_converts_beam_e1_into_a_bar_in_place;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⛔️rejects-a-missing-bd448c/🦀️.rs"]
                                    mod tests_rejects_replacing_an_element_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades-the-roof-fb20eb/🦀️.rs"]
                                    mod tests_regrades_the_roof_beam_onto_the_ipe270_profile;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️denies-rename-0d46d8/🦀️.rs"]
                                    mod tests_refuses_to_rename_the_roof_beam_through_a_replace_element;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚫️dangling-start-cda887/🦀️.rs"]
                                    mod tests_refuses_to_replace_the_brace_onto_a_start_node_that_does_not_exist;
                                }
                                #[path = "."]
                                pub mod create_material {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🧱️appends-concrete-c30/🦀️.rs"]
                                    mod tests_appends_concrete_c30;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🏗️adds-the-c25-slab-11d8df/🦀️.rs"]
                                    mod tests_adds_the_c25_30_material_for_the_ground_slab;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🚫️rejects-a-duplicate-f3220b/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_material_id_on_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/⚗️denies-poisson-329e35/🦀️.rs"]
                                    mod tests_refuses_an_elastomeric_bearing_at_the_incompressible_poisson_limit;
                                }
                                #[path = "."]
                                pub mod delete_material {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚫️removes-the-30f7a2/🦀️.rs"]
                                    mod tests_removes_the_unreferenced_timber_material;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/⛔️rejects-a-missing-d5b18f/🦀️.rs"]
                                    mod tests_rejects_deleting_a_material_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🗑️drops-the-spare-00e964/🦀️.rs"]
                                    mod tests_drops_the_unreferenced_s235_material_from_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🔗️blocks-in-use-e99619/🦀️.rs"]
                                    mod tests_refuses_to_delete_the_s355_grade_seven_members_are_made_of;
                                }
                                #[path = "."]
                                pub mod replace_material {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🏗️restates-steel-7c22bc/🦀️.rs"]
                                    mod tests_restates_steel_as_s355_in_its_original_slot;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⛔️rejects-a-missing-b3adee/🦀️.rs"]
                                    mod tests_rejects_replacing_a_material_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/📉️cracks-the-c30-b2b220/🦀️.rs"]
                                    mod tests_cracks_the_c30_37_stiffness_in_half_for_the_infill_panel;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🪪️denies-rename-a0d7aa/🦀️.rs"]
                                    mod tests_refuses_to_regrade_the_concrete_by_renaming_it_through_a_replace;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⚗️denies-zero-modulus-71e69a/🦀️.rs"]
                                    mod tests_refuses_a_concrete_row_whose_modulus_was_left_at_zero;
                                }
                                #[path = "."]
                                pub mod create_section {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends-the-ipe300-profile/🦀️.rs"]
                                    mod tests_appends_the_ipe300_profile;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/🦀️.rs"]
                                    mod tests_adds_the_hea220_profile_to_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚫️rejects-a-duplicate-e91bc7/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_section_id_on_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/⚗️denies-zero-area-58b5ca/🦀️.rs"]
                                    mod tests_refuses_an_ipe_100_profile_whose_area_was_left_at_zero;
                                }
                                #[path = "."]
                                pub mod delete_section {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/🦀️.rs"]
                                    mod tests_removes_the_spare_hollow_section;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛔️rejects-a-missing-dbd0a4/🦀️.rs"]
                                    mod tests_rejects_deleting_a_section_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/🦀️.rs"]
                                    mod tests_drops_the_unreferenced_ipe200_section_from_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🔗️blocks-in-use-0a6a3c/🦀️.rs"]
                                    mod tests_refuses_to_delete_the_heb_200_profile_four_columns_still_carry;
                                }
                                #[path = "."]
                                pub mod replace_section {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens-ipe200-with-5e9c08/🦀️.rs"]
                                    mod tests_stiffens_ipe200_with_a_reinforced_profile;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⛔️rejects-a-missing-b468f4/🦀️.rs"]
                                    mod tests_rejects_replacing_a_section_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens-the-chs-e235a5/🦀️.rs"]
                                    mod tests_thickens_the_chs_brace_wall_to_five_millimetres;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️denies-rename-1d02dd/🦀️.rs"]
                                    mod tests_refuses_to_rename_the_roof_beam_profile_through_a_replace_section;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⚗️denies-zero-iy-404e31/🦀️.rs"]
                                    mod tests_refuses_a_roof_beam_profile_whose_second_moment_was_left_at_zero;
                                }
                                #[path = "."]
                                pub mod create_support {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🛞️adds-a-vertical-6161a1/🦀️.rs"]
                                    mod tests_adds_a_vertical_roller_at_node_n2;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🔻️props-the-canopy-b9d719/🦀️.rs"]
                                    mod tests_props_the_canopy_tip_on_a_vertical_roller;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🚫️rejects-a-dangling-b0d60b/🦀️.rs"]
                                    mod tests_rejects_a_support_on_a_node_the_steel_frame_never_had;
                                }
                                #[path = "."]
                                pub mod delete_support {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🔓️releases-the-82b34f/🦀️.rs"]
                                    mod tests_releases_the_roller_at_node_n2;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/⛔️rejects-a-missing-23f3c3/🦀️.rs"]
                                    mod tests_rejects_deleting_a_support_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🕊️frees-the-roof-tie-44562b/🦀️.rs"]
                                    mod tests_frees_the_roof_level_lateral_tie_of_the_steel_frame;
                                }
                                #[path = "."]
                                pub mod replace_support {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔒️upgrades-the-834e4a/🦀️.rs"]
                                    mod tests_upgrades_the_roller_at_n2_to_a_full_fixity;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/⛔️rejects-a-missing-afbf6d/🦀️.rs"]
                                    mod tests_rejects_replacing_a_support_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔩️pins-the-left-base-7891ec/🦀️.rs"]
                                    mod tests_pins_the_left_column_base_by_releasing_its_rotation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🪪️denies-rename-63ec90/🦀️.rs"]
                                    mod tests_refuses_to_rename_the_roof_tie_support_through_a_replace_support;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/👻️dangling-node-98d979/🦀️.rs"]
                                    mod tests_refuses_to_move_the_roof_tie_onto_a_node_that_does_not_exist;
                                }
                                #[path = "."]
                                pub mod create_region {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/🦀️.rs"]
                                    mod tests_appends_a_solid_rectangular_slab;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/🦀️.rs"]
                                    mod tests_infills_the_upper_storey_bay_with_a_concrete_panel;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🚫️rejects-a-duplicate-11ca0d/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_region_id_on_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/📐️denies-two-point-99954a/🦀️.rs"]
                                    mod tests_refuses_a_floor_slab_region_outlined_by_only_two_points;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🕳️denies-loose-hole-d9efa1/🦀️.rs"]
                                    mod tests_refuses_an_upper_bay_panel_whose_door_opening_lies_outside_it;
                                }
                                #[path = "."]
                                pub mod delete_region {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes-the-slab-and-5b301a/🦀️.rs"]
                                    mod tests_removes_the_slab_and_keeps_its_material;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/⛔️rejects-a-missing-a83a6d/🦀️.rs"]
                                    mod tests_rejects_deleting_a_region_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/🦀️.rs"]
                                    mod tests_drops_the_spare_side_panel_region_from_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🔗️blocks-in-use-7c7862/🦀️.rs"]
                                    mod tests_refuses_to_delete_the_infill_panel_the_wind_case_still_presses_on;
                                }
                                #[path = "."]
                                pub mod replace_region {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/🦀️.rs"]
                                    mod tests_punches_a_stair_opening_through_the_slab;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/⛔️rejects-a-missing-6e0d70/🦀️.rs"]
                                    mod tests_rejects_replacing_a_region_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens-the-window-09a8ec/🦀️.rs"]
                                    mod tests_widens_the_window_opening_in_the_infill_wall_panel;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/🦀️.rs"]
                                    mod tests_refuses_to_rename_the_infill_panel_through_a_replace_region;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/👻️dangling-mat-7ef81b/🦀️.rs"]
                                    mod tests_refuses_to_repour_the_infill_panel_in_a_grade_the_model_lacks;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/📐️denies-zero-thick-7d805e/🦀️.rs"]
                                    mod tests_refuses_an_infill_panel_whose_thickness_was_set_to_zero;
                                }
                                #[path = "."]
                                pub mod create_load_case {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/📍️appends-a-live-case-59118a/🦀️.rs"]
                                    mod tests_appends_a_live_case_carrying_one_nodal_load;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/❄️appends-the-snow-4c007c/🦀️.rs"]
                                    mod tests_appends_the_snow_case_over_the_roof_beam;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🚫️rejects-a-dangling-4904f4/🦀️.rs"]
                                    mod tests_rejects_a_load_case_whose_udl_names_a_missing_element;
                                }
                                #[path = "."]
                                pub mod delete_load_case {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚫️removes-the-live-06415d/🦀️.rs"]
                                    mod tests_removes_the_live_case_together_with_its_loads;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/⛔️rejects-a-missing-79ed15/🦀️.rs"]
                                    mod tests_rejects_deleting_a_load_case_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🗑️drops-the-spare-49435f/🦀️.rs"]
                                    mod tests_drops_the_spare_snow_case_with_its_single_load;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🔗️blocks-in-use-7cdc5f/🦀️.rs"]
                                    mod tests_refuses_to_delete_the_dead_case_three_combinations_still_weight;
                                }
                                #[path = "."]
                                pub mod add_load {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/📏️appends-a-member-udl-to-the-dead-case/🦀️.rs"]
                                    mod tests_appends_a_member_udl_to_the_dead_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/💨️pushes-a-wind-load-5c3f1e/🦀️.rs"]
                                    mod tests_pushes_a_wind_point_load_onto_the_first_floor_of_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🚫️rejects-a-missing-4271bc/🦀️.rs"]
                                    mod tests_rejects_adding_a_load_to_a_load_case_that_does_not_exist;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/👻️dangling-node-8d113b/🦀️.rs"]
                                    mod tests_refuses_to_push_a_wind_load_at_a_node_the_frame_does_not_have;
                                }
                                #[path = "."]
                                pub mod remove_load {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/➖️strips-the-trailing-member-133914/🦀️.rs"]
                                    mod tests_strips_the_trailing_member_udl_from_the_dead_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/⛔️rejects-a-missing-1a8a80/🦀️.rs"]
                                    mod tests_rejects_removing_a_load_the_dead_case_never_carried;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/✂️strips-the-roof-udl-0c1b3c/🦀️.rs"]
                                    mod tests_strips_the_trailing_roof_udl_from_the_dead_case;
                                }
                                #[path = "."]
                                pub mod change_load_case_self_weight {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/⚖️switches-self-abbff2/🦀️.rs"]
                                    mod tests_switches_self_weight_on_for_the_dead_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🏋️switches-self-5977a5/🦀️.rs"]
                                    mod tests_switches_self_weight_on_for_the_imposed_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🔁️keeps-self-weight-ff696b/🦀️.rs"]
                                    mod tests_keeps_self_weight_on_for_the_dead_case;
                                }
                                #[path = "."]
                                pub mod create_combination {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🔗️appends-an-uls-0c18bb/🦀️.rs"]
                                    mod tests_appends_an_uls_combination_over_both_cases;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/➕️appends-the-6-10a-eefe01/🦀️.rs"]
                                    mod tests_appends_the_six_ten_a_combination_over_dead_and_snow;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🚫️rejects-a-dangling-2aa3ea/🦀️.rs"]
                                    mod tests_rejects_a_combination_term_naming_a_load_case_that_is_absent;
                                }
                                #[path = "."]
                                pub mod delete_combination {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/✂️removes-the-uls-438c0c/🦀️.rs"]
                                    mod tests_removes_the_uls_combination_and_keeps_both_cases;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/⛔️rejects-a-missing-d4bc03/🦀️.rs"]
                                    mod tests_rejects_deleting_a_combination_the_steel_frame_never_had;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🗑️drops-the-spare-60fda7/🦀️.rs"]
                                    mod tests_drops_the_spare_uls_combination_from_the_steel_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🔗️blocks-in-use-0b898b/🦀️.rs"]
                                    mod tests_refuses_to_delete_an_uls_combination_a_design_envelope_nests;
                                }
                                #[path = "."]
                                pub mod update_analysis_settings {
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-modal-3fbb1a/🦀️.rs"]
                                    mod tests_doubles_the_modal_count_and_halves_the_deformation_scale;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🎚️raises-the-mode-908c2b/🦀️.rs"]
                                    mod tests_raises_the_mode_counts_and_tightens_the_deformation_scale;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔁️keeps-the-analysis-196e4a/🦀️.rs"]
                                    mod tests_keeps_the_analysis_settings_exactly_as_they_are;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🚫️denies-zero-modes-babc1d/🦀️.rs"]
                                    mod tests_refuses_an_analysis_configured_to_extract_zero_modal_modes;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::Fem2dDiff;
        pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
        pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod fem3d {
        #[path = "../../🗿️artifacts/🧊️3d/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs"]
        pub mod live_visual;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-the-column-head-node-n3/🦀️.rs"]
                                    mod tests_appends_the_column_head_node_n3;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚨️dup-node-id-86f2e1/🦀️.rs"]
                                    mod tests_edge_dup_node_id;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏗️hall-new-node-b26700/🦀️.rs"]
                                    mod tests_hall_hall_new_node;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-the-column-head-056295/🦀️.rs"]
                                    mod tests_removes_the_column_head_node_under_a_live_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚨️no-such-node-4027a8/🦀️.rs"]
                                    mod tests_edge_no_such_node;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🏗️hall-cut-node-8350fd/🦀️.rs"]
                                    mod tests_hall_hall_cut_node;
                                }
                                #[path = "."]
                                pub mod create_element {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends-a-diagonal-bracing-bar/🦀️.rs"]
                                    mod tests_appends_a_diagonal_bracing_bar;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚨️dangling-start-ab4132/🦀️.rs"]
                                    mod tests_edge_dangling_start;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🏗️hall-new-tie-074a69/🦀️.rs"]
                                    mod tests_hall_hall_new_tie;
                                }
                                #[path = "."]
                                pub mod delete_element {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛓️rafter-under-udl-e0342d/🦀️.rs"]
                                    mod tests_edge_rafter_under_udl;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes-the-bracing-be89d2/🦀️.rs"]
                                    mod tests_removes_the_bracing_bar_and_leaves_the_frame;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚨️no-such-element-eb788c/🦀️.rs"]
                                    mod tests_edge_no_such_element;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🏗️hall-cut-tie-c268d4/🦀️.rs"]
                                    mod tests_hall_hall_cut_tie;
                                }
                                #[path = "."]
                                pub mod replace_element {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚨️dangling-sec-70b168/🦀️.rs"]
                                    mod tests_edge_dangling_sec;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️renames-brace-219be2/🦀️.rs"]
                                    mod tests_edge_renames_brace;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔄️rolls-the-column-50f732/🦀️.rs"]
                                    mod tests_rolls_the_column_about_its_own_axis;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⏸️same-element-61adb2/🦀️.rs"]
                                    mod tests_edge_same_element;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🏗️hall-strut-d0e4b7/🦀️.rs"]
                                    mod tests_hall_hall_strut;
                                }
                                #[path = "."]
                                pub mod create_material {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🧨️nu-at-a-half-8253d2/🦀️.rs"]
                                    mod tests_edge_nu_at_a_half;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🪙️appends-an-9fdced/🦀️.rs"]
                                    mod tests_appends_an_aluminium_alloy;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🚨️dup-material-id-1c0787/🦀️.rs"]
                                    mod tests_edge_dup_material_id;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🏗️hall-new-steel-0d2572/🦀️.rs"]
                                    mod tests_hall_hall_new_steel;
                                }
                                #[path = "."]
                                pub mod delete_material {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/⛓️glulam-in-use-1208e1/🦀️.rs"]
                                    mod tests_edge_glulam_in_use;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚫️removes-the-b7b56a/🦀️.rs"]
                                    mod tests_removes_the_unreferenced_aluminium_alloy;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚨️no-such-material-494b10/🦀️.rs"]
                                    mod tests_edge_no_such_material;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🏗️hall-cut-gl32c-264bca/🦀️.rs"]
                                    mod tests_hall_hall_cut_gl32c;
                                }
                                #[path = "."]
                                pub mod replace_material {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🧨️negative-e-84dad7/🦀️.rs"]
                                    mod tests_edge_negative_e;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🪪️renames-c24-b60696/🦀️.rs"]
                                    mod tests_edge_renames_c24;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/📉️softens-the-2cd183/🦀️.rs"]
                                    mod tests_softens_the_steel_shear_modulus_in_place;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⏸️same-material-950f90/🦀️.rs"]
                                    mod tests_edge_same_material;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🏗️hall-regrades-8dbc23/🦀️.rs"]
                                    mod tests_hall_hall_regrades;
                                }
                                #[path = "."]
                                pub mod create_section {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🧨️zero-area-475a19/🦀️.rs"]
                                    mod tests_edge_zero_area;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🔳️appends-a-square-bd0e4e/🦀️.rs"]
                                    mod tests_appends_a_square_hollow_profile;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚨️dup-section-id-a76686/🦀️.rs"]
                                    mod tests_edge_dup_section_id;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🏗️hall-new-beam-251a92/🦀️.rs"]
                                    mod tests_hall_hall_new_beam;
                                }
                                #[path = "."]
                                pub mod delete_section {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛓️purlin-in-use-99eb01/🦀️.rs"]
                                    mod tests_edge_purlin_in_use;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-30ecfb/🦀️.rs"]
                                    mod tests_removes_the_spare_square_hollow_profile;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚨️no-such-section-50d29b/🦀️.rs"]
                                    mod tests_edge_no_such_section;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🏗️hall-cut-shs-d44b9e/🦀️.rs"]
                                    mod tests_hall_hall_cut_shs;
                                }
                                #[path = "."]
                                pub mod replace_section {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🧨️negative-iy-d4e0a8/🦀️.rs"]
                                    mod tests_edge_negative_iy;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️renames-purlin-dfe160/🦀️.rs"]
                                    mod tests_edge_renames_purlin;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🌀️raises-the-torsion-296ef0/🦀️.rs"]
                                    mod tests_raises_the_torsion_constant_of_hea200;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⏸️same-section-d1d013/🦀️.rs"]
                                    mod tests_edge_same_section;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🏗️hall-deep-purlin-176fd0/🦀️.rs"]
                                    mod tests_hall_hall_deep_purlin;
                                }
                                #[path = "."]
                                pub mod create_support {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🔒️clamps-the-column-f801c9/🦀️.rs"]
                                    mod tests_clamps_the_column_base_in_all_six_dofs;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🚨️dangling-node-af37e2/🦀️.rs"]
                                    mod tests_edge_dangling_node;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🏗️hall-new-pin-c033f2/🦀️.rs"]
                                    mod tests_hall_hall_new_pin;
                                }
                                #[path = "."]
                                pub mod delete_support {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🔓️releases-the-b3ebb0/🦀️.rs"]
                                    mod tests_releases_the_pinned_node_n2;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🚨️no-such-support-edd22a/🦀️.rs"]
                                    mod tests_edge_no_such_support;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🏗️hall-cut-pin-66d795/🦀️.rs"]
                                    mod tests_hall_hall_cut_pin;
                                }
                                #[path = "."]
                                pub mod replace_support {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🚨️dangling-node-d44469/🦀️.rs"]
                                    mod tests_edge_dangling_node;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🪪️renames-pin-29f41a/🦀️.rs"]
                                    mod tests_edge_renames_pin;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔄️frees-the-three-7783c9/🦀️.rs"]
                                    mod tests_frees_the_three_rotations_at_the_column_base;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/⏸️same-support-bff8b3/🦀️.rs"]
                                    mod tests_edge_same_support;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🏗️hall-fixes-base-b5aa1b/🦀️.rs"]
                                    mod tests_hall_hall_fixes_base;
                                }
                                #[path = "."]
                                pub mod create_solid {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/📐️sliver-outline-316a7c/🦀️.rs"]
                                    mod tests_edge_sliver_outline;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🏠️appends-an-extruded-roof-slab/🦀️.rs"]
                                    mod tests_appends_an_extruded_roof_slab;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🚨️dangling-mat-1ebd78/🦀️.rs"]
                                    mod tests_edge_dangling_mat;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🏗️hall-new-slab-d79da4/🦀️.rs"]
                                    mod tests_hall_hall_new_slab;
                                }
                                #[path = "."]
                                pub mod delete_solid {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/⛓️raft-under-load-e4ea39/🦀️.rs"]
                                    mod tests_edge_raft_under_load;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🚫️removes-the-roof-slab-f0fb64/🦀️.rs"]
                                    mod tests_removes_the_roof_slab_and_keeps_its_material;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🚨️no-such-solid-f08d23/🦀️.rs"]
                                    mod tests_edge_no_such_solid;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🏗️hall-cut-apron-6c79d3/🦀️.rs"]
                                    mod tests_hall_hall_cut_apron;
                                }
                                #[path = "."]
                                pub mod replace_solid {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🚨️dangling-mat-9c89da/🦀️.rs"]
                                    mod tests_edge_dangling_mat;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/📐️zero-height-2b131a/🦀️.rs"]
                                    mod tests_edge_zero_height;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🪪️renames-apron-7bfadd/🦀️.rs"]
                                    mod tests_edge_renames_apron;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/📚️thickens-the-slab-and-b51ef0/🦀️.rs"]
                                    mod tests_thickens_the_slab_and_adds_a_mesh_layer;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/⏸️same-solid-8ad12c/🦀️.rs"]
                                    mod tests_edge_same_solid;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🏗️hall-thick-raft-cddc0f/🦀️.rs"]
                                    mod tests_hall_hall_thick_raft;
                                }
                                #[path = "."]
                                pub mod create_load_case {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🌬️appends-a-wind-case-a6c267/🦀️.rs"]
                                    mod tests_appends_a_wind_case_pushing_on_the_column_head;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🚨️dangling-solid-5e04d9/🦀️.rs"]
                                    mod tests_edge_dangling_solid;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🏗️hall-snow-drift-068d9b/🦀️.rs"]
                                    mod tests_hall_hall_snow_drift;
                                }
                                #[path = "."]
                                pub mod delete_load_case {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/⛓️dead-in-combos-e73167/🦀️.rs"]
                                    mod tests_edge_dead_in_combos;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚫️removes-the-wind-caeb06/🦀️.rs"]
                                    mod tests_removes_the_wind_case_together_with_its_load;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚨️no-such-case-ef1fde/🦀️.rs"]
                                    mod tests_edge_no_such_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🏗️hall-cut-crane-52270d/🦀️.rs"]
                                    mod tests_hall_hall_cut_crane;
                                }
                                #[path = "."]
                                pub mod add_load {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🚨️no-such-member-3fe6e9/🦀️.rs"]
                                    mod tests_edge_no_such_member;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🏠️lays-an-area-pressure-over-769710/🦀️.rs"]
                                    mod tests_lays_an_area_pressure_over_the_roof_slab;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/⏸️dup-load-id-4f4a0a/🦀️.rs"]
                                    mod tests_edge_dup_load_id;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🏗️hall-adds-udl-e345cb/🦀️.rs"]
                                    mod tests_hall_hall_adds_udl;
                                }
                                #[path = "."]
                                pub mod remove_load {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/➖️drops-the-trailing-member-b73b25/🦀️.rs"]
                                    mod tests_drops_the_trailing_member_udl_from_the_dead_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/🚨️no-such-load-5bab2d/🦀️.rs"]
                                    mod tests_edge_no_such_load;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/🏗️hall-cut-wind-6cf528/🦀️.rs"]
                                    mod tests_hall_hall_cut_wind;
                                }
                                #[path = "."]
                                pub mod change_load_case_self_weight {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/⏸️switches-self-7e0cda/🦀️.rs"]
                                    mod tests_switches_self_weight_off_for_the_dead_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🚨️sw-no-such-case-bfe5bc/🦀️.rs"]
                                    mod tests_edge_sw_no_such_case;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🏗️hall-crane-sw-978370/🦀️.rs"]
                                    mod tests_hall_hall_crane_sw;
                                }
                                #[path = "."]
                                pub mod create_combination {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🔗️appends-a-8ede20/🦀️.rs"]
                                    mod tests_appends_a_serviceability_combination_keyed_by_case_id;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🚨️dangling-term-b9d144/🦀️.rs"]
                                    mod tests_edge_dangling_term;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🏗️hall-new-acc-4099b2/🦀️.rs"]
                                    mod tests_hall_hall_new_acc;
                                }
                                #[path = "."]
                                pub mod delete_combination {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/✂️removes-the-182f7b/🦀️.rs"]
                                    mod tests_removes_the_serviceability_combination_and_keeps_both_cases;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🚨️no-such-combo-f42cd6/🦀️.rs"]
                                    mod tests_edge_no_such_combo;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🏗️hall-cut-qp-ebd806/🦀️.rs"]
                                    mod tests_hall_hall_cut_qp;
                                }
                                #[path = "."]
                                pub mod update_analysis_settings {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🧨️zero-modes-a27c74/🦀️.rs"]
                                    mod tests_edge_zero_modes;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles-the-7b5381/🦀️.rs"]
                                    mod tests_doubles_the_buckling_mode_count;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/⏸️same-settings-fdb832/🦀️.rs"]
                                    mod tests_edge_same_settings;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🏗️hall-more-modes-ecbb5c/🦀️.rs"]
                                    mod tests_hall_hall_more_modes;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::Fem3dDiff;
        pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
        pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs"]
        pub mod session;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🏋️add-area-load/🦀️.rs"]
            pub mod add_area_load;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/➖️add-bar/🦀️.rs"]
            pub mod add_bar;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🌉️add-beam/🦀️.rs"]
            pub mod add_beam;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🔗️add-combination/🦀️.rs"]
            pub mod add_combination;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📋️add-load-case/🦀️.rs"]
            pub mod add_load_case;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧱️add-material/🦀️.rs"]
            pub mod add_material;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📏️add-member-udl/🦀️.rs"]
            pub mod add_member_udl;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs"]
            pub mod add_nodal_load;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚪️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗺️add-region/🦀️.rs"]
            pub mod add_region;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📐️add-section/🦀️.rs"]
            pub mod add_section;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🛡️add-support/🦀️.rs"]
            pub mod add_support;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs"]
            pub mod remove_selection;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs"]
            pub mod set_analysis_settings;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs"]
            pub mod set_result_display;
            #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚖️set-self-weight/🦀️.rs"]
            pub mod set_self_weight;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs"]
                    pub mod model;
                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                    pub mod results;
                }
            }
        }
    }

    #[path = "."]
    pub mod fem3d {
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🏋️add-area-load/🦀️.rs"]
            pub mod add_area_load;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/➖️add-bar/🦀️.rs"]
            pub mod add_bar;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🔗️add-combination/🦀️.rs"]
            pub mod add_combination;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🖼️add-frame/🦀️.rs"]
            pub mod add_frame;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📋️add-load-case/🦀️.rs"]
            pub mod add_load_case;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧱️add-material/🦀️.rs"]
            pub mod add_material;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📏️add-member-udl/🦀️.rs"]
            pub mod add_member_udl;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📍️add-nodal-load/🦀️.rs"]
            pub mod add_nodal_load;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚪️add-node/🦀️.rs"]
            pub mod add_node;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📐️add-section/🦀️.rs"]
            pub mod add_section;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧊️add-solid/🦀️.rs"]
            pub mod add_solid;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🛡️add-support/🦀️.rs"]
            pub mod add_support;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs"]
            pub mod remove_selection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧮️set-analysis-settings/🦀️.rs"]
            pub mod set_analysis_settings;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs"]
            pub mod set_result_display;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/⚖️set-self-weight/🦀️.rs"]
            pub mod set_self_weight;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs"]
                    pub mod model;
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                    pub mod results;
                }
            }
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs"]
                    pub mod model;
                }
            }
        }
    }

    #[path = "."]
    pub mod fem3d {
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs"]
                    pub mod model;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::FemApps;
#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin::plugin, FemApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_2d_demo_session;
    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_3d_demo_session;
    #[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_2d_demo;
    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_3d_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
