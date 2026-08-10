//! 🌐️ GIS plugin — 2D map + 3D terrain apps bundled as one hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_schema as schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<...Mutation, ...ConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod gismap {
        #[path = "../../🗿️artifacts/🗺️gismap/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🗺️gismap/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🗺️gismap/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/🗺️gismap/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🗺️gismap/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🗺️gismap/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🗺️gismap/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🗺️gismap/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
        #[path = "../../🗿️artifacts/🗺️gismap/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🖊️dwg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🖊️dwg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🖊️dxf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🖊️dxf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📄️pdf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📄️pdf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📷️png/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📷️png/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🎨️svg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🎨️svg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/🗺️gismap/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod gisterrain {
        #[path = "../../🗿️artifacts/🏔️gisterrain/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🏔️gisterrain/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🏔️gisterrain/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/🏔️gisterrain/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_exaggeration {
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/🎛set-exaggeration/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/🎛set-exaggeration/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/🎛set-exaggeration/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_imported_features {
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/🎛set-imported-features/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/🎛set-imported-features/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/🎛set-imported-features/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🏔️gisterrain/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🏔️gisterrain/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🏔️gisterrain/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
        #[path = "../../🗿️artifacts/🏔️gisterrain/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🖊️dwg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🖊️dwg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🧊️glb/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🧊️glb/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🧊️gltf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🧊️gltf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod las {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/☁️las/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/☁️las/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🧊️obj/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🧊️obj/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod ply {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/☁️ply/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/☁️ply/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📷️png/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📷️png/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🟪️stl/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🟪️stl/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/🏔️gisterrain/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod gis2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/◻2d/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/◻2d/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "../../🎛️apps/◻2d/🗺️maphost/🦀️component.rs"]
        pub mod maphost;
        #[path = "../../🎛️apps/◻2d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/◻2d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗺️features/🦀️component.rs"]
            pub mod features;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🌐️shell/🦀️component.rs"]
            pub mod shell;
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
                    #[path = "."]
                    pub mod map {
                        #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🖼️render-mode/🦀️component.rs"]
                            pub mod render_mode;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🎨️vector-style/🦀️component.rs"]
                            pub mod vector_style;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🔽️lod-mode/🦀️component.rs"]
                            pub mod lod_mode;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🖱️selection-method/🦀️component.rs"]
                            pub mod selection_method;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/👁️layers/🦀️component.rs"]
                            pub mod layers;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/📏️layer-weights/🦀️component.rs"]
                            pub mod layer_weights;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/◻2d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/◻2d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod gis3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🧊️3d/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🧊️3d/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🏔️exaggeration/🦀️component.rs"]
            pub mod exaggeration;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs"]
                    pub mod terrain;
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🔧️setup/🦀️component.rs"]
mod setup;
pub use setup::register_gis_exports;

#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🏔️gisterrain/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_gisterrain_demo;
    #[path = "../../🗿️artifacts/🗺️gismap/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_gismap_demo;
    #[path = "../../🎛️apps/◻2d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_2d_demo_session;
    #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_3d_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
