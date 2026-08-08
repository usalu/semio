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
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FemXMutation, FemXConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing
// it here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl
// itself (only on the free functions the taxonomy split creates), so this is a pure artefact of
// decomposition.
#[allow(clippy::result_large_err)]

//#region 🏗️Kernel modules
#[path = "../../🏗️model/🦀️component.rs"]
pub mod model;
#[path = "../../🧮️analyses/🦀️component.rs"]
pub mod analyses;
#[path = "../../📏️elements2d/🦀️component.rs"]
pub mod elements2d;
#[path = "../../🧊️elements3d/🦀️component.rs"]
pub mod elements3d;
#[path = "../../➗️formulation/🦀️component.rs"]
pub mod formulation;
#[path = "../../🕸️mesh/🦀️component.rs"]
pub mod mesh;
#[path = "../../🔢️sparse/🦀️component.rs"]
pub mod sparse;
#[path = "../../🖥️app-surface/🦀️component.rs"]
pub mod app_surface;

/// 🗂️ Registers both artifacts' engines with the host.
pub fn register_all_engines() {
    crate::artifacts::fem2d::engine::register();
    crate::artifacts::fem3d::engine::register();
}
//#endregion 🏗️Kernel modules

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🗿️artifacts/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/◻2d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/◻2d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_node {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/📍set-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/📍set-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/📍set-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_node {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_element {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-element/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-element/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-element/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_element {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-element/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-element/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-element/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_material {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-material/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-material/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-material/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_material {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-material/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-material/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-material/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_section {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-section/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-section/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-section/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_section {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-section/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-section/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-section/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_support {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-support/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-support/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-support/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_support {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-support/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-support/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-support/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_load_case {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-load-case/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-load-case/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-load-case/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_load_case {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-load-case/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-load-case/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-load-case/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_region {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-region/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-region/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-region/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_region {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-region/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-region/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-region/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_combination {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-combination/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-combination/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-combination/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_combination {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-combination/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-combination/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/➖remove-combination/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_analysis_settings {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-analysis-settings/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-analysis-settings/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/🎛set-analysis-settings/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_document {
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/📄set-document/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/📄set-document/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/◻2d/🧬️mutations/📄set-document/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/◻2d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/◻2d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/◻2d/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🕸️meshing/🦀️component.rs"]
            pub mod meshing;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🎵️modal-buckling/🦀️component.rs"]
            pub mod modal_buckling;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🗺️mesh-preview/🦀️component.rs"]
            pub mod mesh_preview;
        }
    }

    #[path = "."]
    pub mod fem3d {
        #[path = "../../🗿️artifacts/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🧊️3d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🧊️3d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_node {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/📍set-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/📍set-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/📍set-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_node {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_element {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-element/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-element/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-element/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_element {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-element/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-element/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-element/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_material {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-material/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-material/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-material/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_material {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-material/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-material/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-material/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_section {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-section/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-section/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-section/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_section {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-section/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-section/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-section/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_solid {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-solid/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-solid/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-solid/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_solid {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-solid/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-solid/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-solid/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_support {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-support/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-support/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-support/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_support {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-support/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-support/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-support/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_load_case {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-load-case/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-load-case/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-load-case/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_load_case {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-load-case/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-load-case/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-load-case/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_combination {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-combination/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-combination/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-combination/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_combination {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-combination/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-combination/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/➖remove-combination/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_analysis_settings {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-analysis-settings/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-analysis-settings/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/🎛set-analysis-settings/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_document {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/📄set-document/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/📄set-document/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️mutations/📄set-document/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🧊️3d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🧊️3d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🧊️3d/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🕸️meshing/🦀️component.rs"]
            pub mod meshing;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🎵️modal-buckling/🦀️component.rs"]
            pub mod modal_buckling;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🗺️mesh-preview/🦀️component.rs"]
            pub mod mesh_preview;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/◻2d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧱️model/🦀️component.rs"]
            pub mod model;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🏋️loads/🦀️component.rs"]
            pub mod loads;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧮️analysis/🦀️component.rs"]
            pub mod analysis;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/📚️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/◻2d/🎮️commands/👁️results/🦀️component.rs"]
            pub mod results;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs"]
                    pub mod model;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }
    }

    #[path = "."]
    pub mod fem3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧱️model/🦀️component.rs"]
            pub mod model;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🏋️loads/🦀️component.rs"]
            pub mod loads;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧮️analysis/🦀️component.rs"]
            pub mod analysis;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/📚️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👁️results/🦀️component.rs"]
            pub mod results;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs"]
                    pub mod model;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/◻2d/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_2d_demo;
    #[path = "../../🗿️artifacts/🧊️3d/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_3d_demo;
    #[path = "../../🎛️apps/◻2d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_2d_demo_session;
    #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_3d_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
