//! 🌀️ Procedural plugin — 2D and 3D flow apps bundled as one hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
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
    pub mod procedural2d {
        #[path = "../../🗿️artifacts/🌀️procedural2d/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::procedural2d::schema::snapshot::Procedural2dSnapshot;
        pub use crate::artifacts::procedural2d::schema::mutations::Procedural2dMutation;
        pub use crate::artifacts::procedural2d::schema::diff::Procedural2dDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod remove_layout {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-layout/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-layout/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-layout/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_synapse {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-synapse/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-synapse/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-synapse/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_widget {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-widget/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-widget/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/➖remove-widget/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_camera {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-camera/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-camera/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-camera/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_layout {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-layout/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-layout/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-layout/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_schema {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-schema/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-schema/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-schema/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_synapse {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-synapse/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-synapse/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-synapse/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_widget {
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-widget/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-widget/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌀️procedural2d/🧬️schema/🧬️mutations/🎛set-widget/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::procedural2d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::procedural2d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::procedural2d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::procedural2d::schema::diff::*; pub use crate::artifacts::procedural2d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::procedural2d::schema::diff::*; } pub mod text { pub use crate::artifacts::procedural2d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::procedural2d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::procedural2d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::procedural2d::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🌀️procedural2d/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🌀️procedural2d/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
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
                    pub use crate::artifacts::procedural2d::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural2d::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural2d::io::export::serializers::artifacts::dxf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural2d::io::import::deserializers::artifacts::dxf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural2d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural2d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural2d::io::export::serializers::artifacts::pdf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural2d::io::import::deserializers::artifacts::pdf::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural2d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural2d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural2d::io::export::serializers::artifacts::svg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural2d::io::import::deserializers::artifacts::svg::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🌀️procedural2d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod procedural3d {
        #[path = "../../🗿️artifacts/🧊️procedural3d/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::procedural3d::schema::snapshot::Procedural3dSnapshot;
        pub use crate::artifacts::procedural3d::schema::mutations::Procedural3dMutation;
        pub use crate::artifacts::procedural3d::schema::diff::Procedural3dDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod remove_layout {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-layout/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-layout/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-layout/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_synapse {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-synapse/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-synapse/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-synapse/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_widget {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-widget/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-widget/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/➖remove-widget/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_camera {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-camera/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-camera/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-camera/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_layout {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-layout/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-layout/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-layout/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_schema {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-schema/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-schema/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-schema/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_synapse {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-synapse/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-synapse/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-synapse/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_widget {
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-widget/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-widget/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️procedural3d/🧬️schema/🧬️mutations/🎛set-widget/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::procedural3d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::procedural3d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::procedural3d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::procedural3d::schema::diff::*; pub use crate::artifacts::procedural3d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::procedural3d::schema::diff::*; } pub mod text { pub use crate::artifacts::procedural3d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::procedural3d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::procedural3d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::procedural3d::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🧊️procedural3d/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🧊️procedural3d/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
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
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::gltf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::gltf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod las {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::las::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::las::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod ply {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::ply::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::ply::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::procedural3d::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::procedural3d::io::import::deserializers::artifacts::stl::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs"]
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
    pub mod procedural2d {
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

        #[path = "../../🎛️apps/◻2d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/◻2d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧩️widget/🦀️component.rs"]
            pub mod widget;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs"]
            pub mod generation;
            #[path = "../../🎛️apps/◻2d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧮️eval/🦀️component.rs"]
            pub mod eval;
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
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs"]
                    pub mod flow;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️component.rs"]
                    pub mod generations;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️component.rs"]
                    pub mod form;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
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
    pub mod procedural3d {
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

        #[path = "../../🎛️apps/🧊️3d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧩️widget/🦀️component.rs"]
            pub mod widget;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧭️gumball/🦀️component.rs"]
            pub mod gumball;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧬️generation/🦀️component.rs"]
            pub mod generation;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧮️eval/🦀️component.rs"]
            pub mod eval;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
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
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs"]
                    pub mod flow;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️component.rs"]
                    pub mod generations;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️component.rs"]
                    pub mod form;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs"]
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
    #[path = "../../🗿️artifacts/🌀️procedural2d/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_procedural2d_demo;
    #[path = "../../🗿️artifacts/🧊️procedural3d/📚️examples/🎬️box-fillet-preview/🦀️component.rs"]
    pub mod art_procedural3d_box_fillet_preview;
    #[path = "../../🗿️artifacts/🧊️procedural3d/📚️examples/🎬️box-shell-preview/🦀️component.rs"]
    pub mod art_procedural3d_box_shell_preview;
    #[path = "../../🗿️artifacts/🧊️procedural3d/📚️examples/🎬️face-sweep-extrude/🦀️component.rs"]
    pub mod art_procedural3d_face_sweep_extrude;
    #[path = "../../🗿️artifacts/🧊️procedural3d/📚️examples/🎬️rectangle-extrude-volume/🦀️component.rs"]
    pub mod art_procedural3d_rectangle_extrude_volume;
    #[path = "../../🗿️artifacts/🧊️procedural3d/📚️examples/🎬️rectangle-wire-preview/🦀️component.rs"]
    pub mod art_procedural3d_rectangle_wire_preview;
    #[path = "../../🗿️artifacts/🧊️procedural3d/📚️examples/🎬️sphere-box-fuse/🦀️component.rs"]
    pub mod art_procedural3d_sphere_box_fuse;
    #[path = "../../🗿️artifacts/🧊️procedural3d/📚️examples/🎬️sphere-cut-with-torus/🦀️component.rs"]
    pub mod art_procedural3d_sphere_cut_with_torus;
    #[path = "../../🎛️apps/◻2d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_2d_demo_session;
    #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_3d_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
