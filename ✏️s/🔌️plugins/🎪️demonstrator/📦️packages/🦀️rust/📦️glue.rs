//! 🎪️ Entwerfen mit Bestand demonstrator — the six demonstrator panes (generator, koordinator,
//! aggregator, aussuchen, bearbeiten, verfolgen) bundled as ONE hot-swappable WASM plugin instead of
//! six separate ones, so they share one framework/kernel linkage and one plugin worker/module (see
//! `acquirePluginModule`'s lease pool in framework core `📦️index.ts`) instead of statically
//! duplicating the SDK six times over.
//!
//! This crate also owns the minimal `🎪️playground` artifact (schema/snapshot/diff/dsl/op/spr/engine)
//! so the demonstrator taxonomy slot is complete. Pane apps still come from the six source plugins.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust`, hence the `../../` climb
//! back out to the owner root's tree). The grouping module carries `#[path = "."]` so its own name is
//! not spliced into that base directory. Do not inline any component file back into this one: the
//! taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod playground {
        #[path = "../../🗿️artifacts/🎪️playground/🦀️component.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod no_mutation {
                    #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/🫙no-mutation/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/🫙no-mutation/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🎪️playground/🧬️schema/🧬️mutations/🫙no-mutation/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::playground::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::playground::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::playground::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::playground::schema::diff::*; pub use crate::artifacts::playground::schema::diff::text::*; pub mod schema { pub use crate::artifacts::playground::schema::diff::*; } pub mod text { pub use crate::artifacts::playground::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::playground::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::playground::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::playground::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🎪️playground/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🎪️playground/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
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
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🎪️playground/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::playground::io::export::serializers::artifacts::csv::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::playground::io::import::deserializers::artifacts::csv::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::playground::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::playground::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::playground::io::export::serializers::artifacts::xlsx::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::playground::io::import::deserializers::artifacts::xlsx::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::playground::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::playground::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🎪️playground/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎪️Panes
#[path = "."]
pub mod panes {
    #[path = "../../🎪️panes/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🎪️panes/🌱️generator/🦀️component.rs"]
    pub mod generator;
    #[path = "../../🎪️panes/📐️koordinator/🦀️component.rs"]
    pub mod koordinator;
    #[path = "../../🎪️panes/🧩️aggregator/🦀️component.rs"]
    pub mod aggregator;
    #[path = "../../🎪️panes/🗂️aussuchen/🦀️component.rs"]
    pub mod aussuchen;
    #[path = "../../🎪️panes/🏭️bearbeiten/🦀️component.rs"]
    pub mod bearbeiten;
    #[path = "../../🎪️panes/🗺️verfolgen/🦀️component.rs"]
    pub mod verfolgen;
}
//#endregion 🎪️Panes

//#region 🔖️Manifest
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);
//#endregion 🔖️Manifest

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🎪️playground/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_playground_demo;
}
//#endregion 📚️Examples
