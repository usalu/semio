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
    pub mod gisterrain {
        #[path = "../../🗿️artifacts/🏔️gisterrain/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::gisterrain::schema::snapshot::GisTerrainSnapshot;
        pub use crate::artifacts::gisterrain::schema::mutations::GisTerrainMutation;
        pub use crate::artifacts::gisterrain::schema::diff::GisTerrainDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_exaggeration {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/🎛set-exaggeration/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/🎛set-exaggeration/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/🎛set-exaggeration/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_imported_features {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/🎛set-imported-features/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/🎛set-imported-features/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/🎛set-imported-features/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🏔️gisterrain/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::gisterrain::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::gisterrain::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::gisterrain::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::gisterrain::schema::diff::*; pub use crate::artifacts::gisterrain::schema::diff::text::*; pub mod schema { pub use crate::artifacts::gisterrain::schema::diff::*; } pub mod text { pub use crate::artifacts::gisterrain::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::gisterrain::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::gisterrain::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::gisterrain::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🏔️gisterrain/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🏔️gisterrain/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🏔️gisterrain/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
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
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::gltf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::gltf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod las {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::las::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::las::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod ply {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::ply::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::ply::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gisterrain::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gisterrain::io::import::deserializers::artifacts::stl::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🏔️gisterrain/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod gismap {
        #[path = "../../🗿️artifacts/🗺️gismap/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::gismap::schema::snapshot::GisMapSnapshot;
        pub use crate::artifacts::gismap::schema::mutations::GisMapMutation;
        pub use crate::artifacts::gismap::schema::diff::GisMapDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🗺️gismap/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::gismap::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::gismap::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::gismap::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::gismap::schema::diff::*; pub use crate::artifacts::gismap::schema::diff::text::*; pub mod schema { pub use crate::artifacts::gismap::schema::diff::*; } pub mod text { pub use crate::artifacts::gismap::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::gismap::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::gismap::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::gismap::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🗺️gismap/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🗺️gismap/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🗺️gismap/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
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
                    pub use crate::artifacts::gismap::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gismap::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gismap::io::export::serializers::artifacts::dxf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gismap::io::import::deserializers::artifacts::dxf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gismap::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gismap::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gismap::io::export::serializers::artifacts::pdf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gismap::io::import::deserializers::artifacts::pdf::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gismap::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gismap::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::gismap::io::export::serializers::artifacts::svg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::gismap::io::import::deserializers::artifacts::svg::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🗺️gismap/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
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
