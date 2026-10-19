//! 🔬️ tests of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

#[path = "."]
pub mod render {
    #[path = "../../../../../../../🖌️render/🪟️window-config/🦀️.rs"]
    pub mod window_config;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "."]
                            pub mod codec {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/🌿️gbxml/🧱️codec/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod energy {
                            #[path = "."]
                            pub mod target {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/🔋️energy/🧬️target/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod pairs {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/🔋️energy/🔗️pairs/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "."]
                            pub mod frames {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🧭️frames/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "."]
                        pub mod phases {
                            #[cfg(test)]
                            #[path = "../../🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod terminology {
            #[path = "../../../../../../../✏️editor/🗣️terminology/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "../../../../../../../✏️editor/🎭️modes/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../../../../../✏️editor/📌️panels/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../✏️editor/🎮️commands/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        #[path = "../../../../../../../👁️viewer/🗣️terminology/🦀️.rs"]
        pub mod terminology;
    }
}
