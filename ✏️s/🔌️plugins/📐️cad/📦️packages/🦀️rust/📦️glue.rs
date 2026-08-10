//! 📐️ CAD plugin — the spatial-model play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, Shape V2 — see ticket
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`), prefixed with `../../` to reach back up to the plugin
//! root. The grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust resolves an inline module's children under
//! `<file dir>/<inline mod name>/…` and every leaf path dangles. Do not inline any component file back
//! into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see
//! master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard
//! ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<CadMutation, CadConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing
// it here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl
// itself (only on the free functions the taxonomy split creates), so this is a pure artefact of
// decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod cad {
        #[path = "../../🗿️artifacts/📐️cad/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs"]
        mod interaction_spec;
        pub use interaction_spec::*;
        #[path = "../../🗿️artifacts/📐️cad/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod add_object {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➕️add-object/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➕️add-object/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➕️add-object/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_object {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➖️remove-object/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➖️remove-object/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➖️remove-object/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod patch_object {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🩹patch-object/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🩹patch-object/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🩹patch-object/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod translate_objects {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/↕️translate-objects/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/↕️translate-objects/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/↕️translate-objects/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod rotate_objects {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🔄rotate-objects/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🔄rotate-objects/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🔄rotate-objects/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod scale_objects {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/↔️scale-objects/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/↔️scale-objects/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/↔️scale-objects/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_pane_objects {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🖼️set-pane-objects/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🖼️set-pane-objects/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🖼️set-pane-objects/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod add_node {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➕️add-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➕️add-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➕️add-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_node {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➖️remove-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➖️remove-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/➖️remove-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod rename_node {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🏷️rename-node/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🏷️rename-node/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🏷️rename-node/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod patch_reference {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🩹patch-reference/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🩹patch-reference/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🩹patch-reference/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_references {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/📎set-references/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/📎set-references/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/📎set-references/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_active_model_definition {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🎯set-active-model-definition/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🎯set-active-model-definition/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🎯set-active-model-definition/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📐️cad/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

        }

        #[path = "../../🗿️artifacts/📐️cad/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📐️cad/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📐️cad/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "../../🗿️artifacts/📐️cad/🗣️dsl/🦀️component.rs"]
        pub mod dsl;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📐️cad/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/📐️cad/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/📐️cad/📡️spr/🦀️component.rs"]
        pub mod spr;


        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📐️cad/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🖊️dwg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🖊️dwg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🧊️glb/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🧊️glb/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🧊️gltf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🧊️gltf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod ifc {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🏗️ifc/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🏗️ifc/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🧊️obj/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🧊️obj/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/📷️png/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/📷️png/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod step {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/📐️step/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/📐️step/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🟪️stl/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📐️cad/🚪️io/🟪️stl/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/📥️geometry-import/🦀️component.rs"]
            pub mod geometry_import;
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🔄️transformation/🦀️component.rs"]
            pub mod transformation;
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs"]
            pub mod interaction;
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🔍️construct/🦀️component.rs"]
            pub mod construct;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod cad {
        #[path = "../../🎛️apps/📐️cad/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/📐️cad/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📐️cad/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/📐️cad/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📐️cad/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "../../🎛️apps/📐️cad/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/📐️cad/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🧱️object/🦀️component.rs"]
            pub mod object;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🕸️node/🦀️component.rs"]
            pub mod node;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🔄️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/📥️io/🦀️component.rs"]
            pub mod io;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🖼️reference/🦀️component.rs"]
            pub mod reference;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🗺️model-definition/🦀️component.rs"]
            pub mod model_definition;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🧩️contribution/🦀️component.rs"]
            pub mod contribution;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📐️cad/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📐️cad/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📐️cad/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🎚️options/🎥️projection/🦀️component.rs"]
                    pub mod projection;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🎚️options/🌞️sun/🦀️component.rs"]
                    pub mod sun;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🎚️options/🕹️dislocate/🦀️component.rs"]
                    pub mod dislocate;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/📐️shape/🦀️component.rs"]
                    pub mod shape;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/🏢️building/🦀️component.rs"]
                    pub mod building;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/🔥️energy/🦀️component.rs"]
                    pub mod energy;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/🏛️structure-classic/🦀️component.rs"]
                    pub mod structure_classic;
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
    #[path = "../../🗿️artifacts/📐️cad/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_cad_demo;
    #[path = "../../🎛️apps/📐️cad/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_cad_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
