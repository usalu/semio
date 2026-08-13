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
// 🧩️ `wfc_engine`'s copied-verbatim `🗺️topology`/`🎲️sample`/`🔮️oracle`/… leaves reference these two
// by the exact same aliases `semio-framework-math` used before this wave — the legal plugin→framework
// dependency direction, not a new coupling.
extern crate semio_framework_geometry as geometry;
extern crate semio_framework_graph as graph_core;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<...Mutation, ...ConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
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
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod clear_widget_layout {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod disconnect_synapse {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_widget {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_camera {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod move_widget {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_schema {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod connect_synapse {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-synapse/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-synapse/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-synapse/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod replace_widget {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-widget/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-widget/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-widget/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod replace_synapse {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-synapse/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-synapse/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-synapse/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_generation {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_generation {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod rename_generation {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_generation_value {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_widget {
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️component.rs"]
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
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️component.rs"]
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
        pub mod op { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::Procedural2dSnapshot;
        pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::Procedural2dMutation;
        pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::Procedural2dDiff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod procedural3d {
        #[path = "../../🗿️artifacts/🧊️procedural3d/🦀️component.rs"]
        mod component;
        pub use component::*;

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
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod delete_widget_position {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-widget-position/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-widget-position/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-widget-position/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod disconnect_synapse {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_widget {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_camera {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷update-camera/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷update-camera/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷update-camera/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod move_widget {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_schema {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_synapse {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄update-synapse/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄update-synapse/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄update-synapse/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod update_widget {
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
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
        pub mod op { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::Procedural3dSnapshot;
        pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::Procedural3dMutation;
        pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::Procedural3dDiff;


    }

    #[path = "."]
    pub mod assembly {
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
                            #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                            pub mod snapshot;
                            #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                            pub mod diff;
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod create_slot {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-slot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-slot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-slot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_slot {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-slot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-slot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-slot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_rule {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-rule/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-rule/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-rule/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_rule {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-rule/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-rule/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-rule/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_weight {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-weight/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-weight/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-weight/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod remove_weight {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-weight/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-weight/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-weight/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod connect_slots {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod disconnect_slots {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_seed {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲change-seed/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲change-seed/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲change-seed/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: flat access from the artifact root, mirroring procedural2d/procedural3d ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod diff {
            pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::diff::*;
        }
        pub mod mutations {
            pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod inferences {
            pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::inferences::*;
        }
        pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::snapshot::AssemblySnapshot;
        pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::mutations::AssemblyMutation;
        pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::diff::AssemblyDiff;
    }
}
//#endregion 🗿️Artifacts

//#region 🧩️WfcEngine
#[path = "."]
pub(crate) mod wfc_engine {
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔦️beam/🦀️component.rs"]
    pub(crate) mod beam;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎛️bitset/🦀️component.rs"]
    pub mod bitset;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🍰️chunk/🦀️component.rs"]
    pub(crate) mod chunk;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️component.rs"]
    pub mod constraint;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔢️constraints-card/🦀️component.rs"]
    pub mod constraints_card;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔗️constraints-conn/🦀️component.rs"]
    pub mod constraints_conn;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🩺️diag/🦀️component.rs"]
    pub mod diag;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🌐️domain/🦀️component.rs"]
    pub mod domain;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚠️error/🦀️component.rs"]
    pub mod error;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧬️evolve/🦀️component.rs"]
    pub mod evolve;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛏️extract/🦀️component.rs"]
    pub mod extract;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🌊️flow/🦀️component.rs"]
    pub mod flow;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔲️grid-2d/🦀️component.rs"]
    pub mod grid2d;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧊️grid-3d/🦀️component.rs"]
    pub mod grid3d;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧭️heuristics/🦀️component.rs"]
    pub mod heuristics;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪜️hierarchy/🦀️component.rs"]
    pub(crate) mod hierarchy;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🆔️ids/🦀️component.rs"]
    pub mod ids;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🏗️model/🦀️component.rs"]
    pub mod model;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎼️motif/🦀️component.rs"]
    pub(crate) mod motif;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🚫️nogood/🦀️component.rs"]
    pub(crate) mod nogood;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔮️oracle/🦀️component.rs"]
    pub mod oracle;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🏁️outcome/🦀️component.rs"]
    pub mod outcome;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️parallel/🦀️component.rs"]
    pub(crate) mod parallel;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔁️prop-ac3/🦀️component.rs"]
    pub(crate) mod prop_ac3;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔄️prop-ac4/🦀️component.rs"]
    pub(crate) mod prop_ac4;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/📣️propagate/🦀️component.rs"]
    pub(crate) mod propagate;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔧️repair/🦀️component.rs"]
    pub(crate) mod repair;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎲️sample/🦀️component.rs"]
    pub mod sample;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔍️search/🦀️component.rs"]
    pub mod search;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💾️serial/🦀️component.rs"]
    pub mod serial;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🦀️component.rs"]
    pub mod soft;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕸️solver-graph/🦀️component.rs"]
    pub mod solver_graph;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔳️solver-grid-2d/🦀️component.rs"]
    pub mod solver_grid2d;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧱️solver-grid-3d/🦀️component.rs"]
    pub mod solver_grid3d;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕳️sparse-3d/🦀️component.rs"]
    pub mod sparse3d;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪞️symmetry/🦀️component.rs"]
    pub mod symmetry;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🀄️tiled/🦀️component.rs"]
    pub mod tiled;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🗺️topology/🦀️component.rs"]
    pub(crate) mod topology;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🐾️trail/🦀️component.rs"]
    pub(crate) mod trail;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚖️weights/🦀️component.rs"]
    pub mod weights;

    pub use beam::BeamConfig;
    pub use bitset::PatternSet;
    pub use constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
    pub use constraints_card::{CardinalityConstraint, Scope};
    pub use constraints_conn::{ConnectivityConstraint, ReachabilityConstraint};
    pub use diag::{DiagLevel, Event, EventSink, Metrics, TraceReplay};
    pub use domain::{Domain, DomainStore, RestrictResult};
    pub use error::{ConstraintError, ModelError, SolveError, TopologyError};
    pub use evolve::{evolve, EvolveConfig, EvolveResult};
    pub use extract::{extract_2d, Extract2dConfig, ExtractedModel2d, PatternDecoder2d, Sample2d};
    pub use flow::FlowConstraint;
    pub use grid2d::{declare_stencil_relations, declare_stencil_relations_tiled, Boundary, Grid2dTopology, Stencil2d};
    pub use grid3d::{declare_stencil_relations_3d, declare_stencil_relations_3d_tiled, Grid3dTopology, Stencil3d};
    pub use heuristics::ObserveHeuristic;
    pub use ids::{ConstraintId, DecisionId, NodeId, PatternId, PortId, RegionId, RelationId, TileId};
    pub use model::{CompiledModel, LintFinding, ModelBuilder, ModelStats, PatternInfo, RelationInfo};
    pub use nogood::NogoodConfig;
    pub use oracle::{check_assignment, enumerate, ArcSpec, OracleResult, Violation};
    pub use outcome::{ContradictionReport, PartialState, RunReport, Solution, SolveOutcome, UnsatReport};
    pub use sample::ValueSampler;
    pub use search::{Budget, CancelToken, RestartSchedule, SearchConfig, SearchMode};
    pub use serial::{CheckpointDoc, PairDoc, PatternDoc, RelationDoc, SourceModelDoc, CHECKPOINT_VERSION, SOURCE_MODEL_VERSION};
    pub use soft::{best_of_n, Attempt, BestOfNKeep, ScoreFn, SoftConstraint, WeightField};
    pub use solver_graph::{GraphSolver, GraphSolverBuilder};
    pub use solver_grid2d::{Grid2dSolver, Grid2dSolverBuilder};
    pub use solver_grid3d::{Grid3dSolver, Grid3dSolverBuilder};
    pub use sparse3d::{SparseVolume, VoxelCoord};
    pub use symmetry::{cube_rotations_24, cube_symmetries_48, SymmetryGroup2d, SymmetryGroup3d, Transform2d, Transform3d};
    pub use tiled::TiledModelBuilder;
    pub use topology::{from_graph_view, GraphTopology, GraphTopologyBuilder};
    pub use trail::Checkpoint;
    pub use weights::{WeightMode, WeightTable, ZeroWeightPolicy};
}
//#endregion 🧩️WfcEngine

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
            #[path = "../../🎛️apps/◻2d/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️artifact/🦀️component.rs"]
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
#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_procedural2d_demo;
    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️box-fillet-preview/🦀️component.rs"]
    pub mod art_procedural3d_box_fillet_preview;
    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️box-shell-preview/🦀️component.rs"]
    pub mod art_procedural3d_box_shell_preview;
    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️face-sweep-extrude/🦀️component.rs"]
    pub mod art_procedural3d_face_sweep_extrude;
    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️rectangle-extrude-volume/🦀️component.rs"]
    pub mod art_procedural3d_rectangle_extrude_volume;
    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️rectangle-wire-preview/🦀️component.rs"]
    pub mod art_procedural3d_rectangle_wire_preview;
    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️sphere-box-fuse/🦀️component.rs"]
    pub mod art_procedural3d_sphere_box_fuse;
    #[path = "../../🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️sphere-cut-with-torus/🦀️component.rs"]
    pub mod art_procedural3d_sphere_cut_with_torus;
    #[path = "../../🎛️apps/◻2d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_2d_demo_session;
    #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_3d_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
