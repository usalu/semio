//! 🧠️ Reasoning plugin — declarative WIRES mindmap play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`), hence the `../../` prefix
//! back out to the plugin (owner) root. The grouping modules carry `#[path = "."]` so their own names are
//! not spliced into that base directory — without it, Rust resolves an inline module's children under
//! `<file dir>/<inline mod name>/…` and every leaf path dangles. Do not inline any component file back
//! into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see
//! master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard
//! ruling, and its Shape V2 addendum for the `📦️packages`-relocated entry file).

extern crate infinite_canvas as infinite_board_normal_undirected;
extern crate infinite_canvas as infinite_board_port_directed;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<WiresMutation, WiresConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod wires {
        #[path = "../../🗿️artifacts/🔌️wires/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🔌️wires/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🔌️wires/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🔌️wires/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/🔌️wires/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod add_node {
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➕add-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➕add-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➕add-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_node {
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➖remove-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➖remove-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➖remove-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod patch_node {
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/🩹patch-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/🩹patch-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/🩹patch-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod add_relationship {
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➕add-relationship/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➕add-relationship/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/➕add-relationship/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_edge {
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/✂️remove-edge/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/✂️remove-edge/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/✂️remove-edge/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🔌️wires/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🔌️wires/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🔌️wires/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🔌️wires/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
        #[path = "../../🗿️artifacts/🔌️wires/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🔌️wires/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod wires {
        #[path = "../../🎛️apps/🔌️wires/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🔌️wires/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🔌️wires/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🔌️wires/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🔌️wires/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🔌️wires/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🧬️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🔵️node/🦀️component.rs"]
            pub mod node;
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🔗️relationship/🦀️component.rs"]
            pub mod relationship;
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🗑️delete/🦀️component.rs"]
            pub mod delete;
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🔄️layout/🦀️component.rs"]
            pub mod layout;
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🖱️pointer/🦀️component.rs"]
            pub mod pointer;
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🔌️wires/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🔌️wires/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🔌️wires/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🦀️component.rs"]
                    pub mod canvas;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🔌️wires/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🔌️wires/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🔌️wires/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
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
    #[path = "../../🗿️artifacts/🔌️wires/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_wires_demo;
    #[path = "../../🎛️apps/🔌️wires/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_wires_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
