//! 🎥️ Shooting plugin — icon-studio play app bundled as a hot-swappable WASM component.
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
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod shooting {
        #[path = "../../🗿️artifacts/🎥️shooting/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🎥️shooting/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🎥️shooting/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🎥️shooting/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "../../🗿️artifacts/🎥️shooting/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod assets {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📦assets/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📦assets/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📦assets/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod shots {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📸shots/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📸shots/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📸shots/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod saved_cameras {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🎥saved-cameras/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🎥saved-cameras/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🎥saved-cameras/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_active_shot {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🎯set-active-shot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🎯set-active-shot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🎯set-active-shot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_active_asset {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📌set-active-asset/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📌set-active-asset/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📌set-active-asset/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_shot_camera {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📷set-shot-camera/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📷set-shot-camera/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📷set-shot-camera/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod patch_scene {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/☀️patch-scene/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/☀️patch-scene/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/☀️patch-scene/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod translate_assets {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/↔️translate-assets/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/↔️translate-assets/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/↔️translate-assets/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod rotate_assets {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🔄rotate-assets/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🔄rotate-assets/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/🔄rotate-assets/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod scale_assets {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/↕️scale-assets/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/↕️scale-assets/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/↕️scale-assets/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🎥️shooting/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/🎥️shooting/🗣️dsl/🦀️component.rs"]
        pub mod dsl;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🎥️shooting/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🎥️shooting/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
        pub use snapshot::pack;
        #[path = "../../🗿️artifacts/🎥️shooting/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod bmp {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🖼️bmp/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🖼️bmp/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🖊️dwg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🖊️dwg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod gif {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🎞️gif/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🎞️gif/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod jpg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/📷️jpg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/📷️jpg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/📄️pdf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/📄️pdf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/📷️png/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/📷️png/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🎨️svg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🎨️svg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod tiff {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🖼️tiff/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🎥️shooting/🚪️io/🖼️tiff/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/🎥️shooting/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod shooting {
        #[path = "../../🎛️apps/🎥️shooting/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🎥️shooting/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🎥️shooting/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🎥️shooting/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🎥️shooting/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🎥️shooting/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🎥️shooting/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🗃️fixture/🦀️component.rs"]
            pub mod fixture;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/📷️shot/🦀️component.rs"]
            pub mod shot;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/📦️asset/🦀️component.rs"]
            pub mod asset;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/☀️scene/🦀️component.rs"]
            pub mod scene;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🧭️gumball/🦀️component.rs"]
            pub mod gumball;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🖨️export/🦀️component.rs"]
            pub mod export;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🎯️center-model/🦀️component.rs"]
                            pub mod center_model;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/☀️sun-enabled/🦀️component.rs"]
                            pub mod sun_enabled;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🧭️sun-azimuth/🦀️component.rs"]
                            pub mod sun_azimuth;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/📐️sun-elevation/🦀️component.rs"]
                            pub mod sun_elevation;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/💡️sun-intensity/🦀️component.rs"]
                            pub mod sun_intensity;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🌫️ambient/🦀️component.rs"]
                            pub mod ambient;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🌑️shadow/🦀️component.rs"]
                            pub mod shadow;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/✨️roughness/🦀️component.rs"]
                            pub mod roughness;
                        }
                    }

                    #[path = "."]
                    pub mod icon {
                        #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/📷️shot/🦀️component.rs"]
                            pub mod shot;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/🗂️format/🦀️component.rs"]
                            pub mod format;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/🔷️shape/🦀️component.rs"]
                            pub mod shape;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🎥️shooting/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🎥️shooting/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🎥️shooting/📌️panels/🔍️inspection/🦀️component.rs"]
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
    #[path = "../../🗿️artifacts/🎥️shooting/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_shooting_demo;
    #[path = "../../🎛️apps/🎥️shooting/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_shooting_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
