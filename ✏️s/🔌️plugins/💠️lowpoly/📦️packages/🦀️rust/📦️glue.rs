//! 💠️ Lowpoly plugin — mesh + paint editor bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod lowpoly {
        #[path = "../../🗿️artifacts/💠️lowpoly/🦀️component.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod objects_move {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/↔️objects-move/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/↔️objects-move/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/↔️objects-move/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod add_paint_layer {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➕️add-paint-layer/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➕️add-paint-layer/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➕️add-paint-layer/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod objects_add {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➕️objects-add/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➕️objects-add/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➕️objects-add/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod objects_remove {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➖️objects-remove/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➖️objects-remove/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➖️objects-remove/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_paint_layer {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➖️remove-paint-layer/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➖️remove-paint-layer/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/➖️remove-paint-layer/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod paint_stroke {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🖌️paint-stroke/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🖌️paint-stroke/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🖌️paint-stroke/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod objects_patch {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🩹objects-patch/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🩹objects-patch/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🩹objects-patch/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod patch_paint_layer {
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🩹patch-paint-layer/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🩹patch-paint-layer/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/💠️lowpoly/🧬️schema/🧬️mutations/🩹patch-paint-layer/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::lowpoly::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::lowpoly::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::lowpoly::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::lowpoly::schema::diff::*; pub use crate::artifacts::lowpoly::schema::diff::text::*; pub mod schema { pub use crate::artifacts::lowpoly::schema::diff::*; } pub mod text { pub use crate::artifacts::lowpoly::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::lowpoly::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::lowpoly::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::lowpoly::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/💠️lowpoly/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/💠️lowpoly/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
                            mod component;
                            pub use component::*;
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
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/💠️lowpoly/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::gltf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::gltf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod las {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::las::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::las::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod ply {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::ply::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::ply::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::lowpoly::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::lowpoly::io::import::deserializers::artifacts::stl::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🎨️paint/🦀️component.rs"]
            pub mod paint;
            #[path = "../../🗿️artifacts/💠️lowpoly/⚙️engine/🧵️media/🦀️component.rs"]
            pub mod media;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod lowpoly {
        #[path = "../../🎛️apps/💠️lowpoly/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/💠️lowpoly/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/💠️lowpoly/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/💠️lowpoly/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/💠️lowpoly/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/💠️lowpoly/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/💠️lowpoly/🧭️view/🦀️component.rs"]
        pub mod view;
        #[path = "../../🎛️apps/💠️lowpoly/🖌️session/🦀️component.rs"]
        pub mod session;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/➕️add-primitive/🦀️component.rs"]
            pub mod add_primitive;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/✏️patch-object/🦀️component.rs"]
            pub mod patch_object;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🔷️mesh-edit/🦀️component.rs"]
            pub mod mesh_edit;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🧵️uv/🦀️component.rs"]
            pub mod uv;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🧲️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🖌️paint/🦀️component.rs"]
            pub mod paint;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🌍️world/🦀️component.rs"]
            pub mod world;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/💬️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/📄️fixture/🦀️component.rs"]
            pub mod fixture;
            #[path = "../../🎛️apps/💠️lowpoly/🎮️commands/👁️chrome/🦀️component.rs"]
            pub mod chrome;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/💠️lowpoly/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/💠️lowpoly/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️component.rs"]
                    pub mod model;
                }
            }

            #[path = "."]
            pub mod paint {
                #[path = "../../🎛️apps/💠️lowpoly/🎭️modes/🎨️paint/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/💠️lowpoly/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️component.rs"]
                    pub mod uv;
                }
            }
        }

        #[path = "."]
        pub mod options {
            #[path = "../../🎛️apps/💠️lowpoly/🎚️options/👁️show-edges/🦀️component.rs"]
            pub mod show_edges;
            #[path = "../../🎛️apps/💠️lowpoly/🎚️options/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/💠️lowpoly/🎚️options/🧲️snap/🦀️component.rs"]
            pub mod snap;
            #[path = "../../🎛️apps/💠️lowpoly/🎚️options/🗂️select/🦀️component.rs"]
            pub mod select;
            #[path = "../../🎛️apps/💠️lowpoly/🎚️options/🖌️paint-params-brush/🦀️component.rs"]
            pub mod paint_params_brush;
            #[path = "../../🎛️apps/💠️lowpoly/🎚️options/🧽️paint-params-eraser/🦀️component.rs"]
            pub mod paint_params_eraser;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/💠️lowpoly/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/💠️lowpoly/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/💠️lowpoly/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "../../🎛️apps/💠️lowpoly/📌️panels/🗂️layers/🦀️component.rs"]
            pub mod layers;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🔧️setup/🦀️component.rs"]
mod setup;
pub use setup::register_lowpoly_exports;

#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/💠️lowpoly/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_lowpoly_demo;
    #[path = "../../🎛️apps/💠️lowpoly/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_lowpoly_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
