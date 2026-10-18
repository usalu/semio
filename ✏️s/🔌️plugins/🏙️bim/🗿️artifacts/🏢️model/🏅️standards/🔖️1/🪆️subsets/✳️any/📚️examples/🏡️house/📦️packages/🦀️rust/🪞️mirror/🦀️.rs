//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../../../🚪️io/📥️import/🏗️ifc/🧭️frames/🦀️.rs"]
                            pub mod frames;
                            #[path = "../../../../../🚪️io/📥️import/🏗️ifc/📖️reader/🦀️.rs"]
                            pub mod reader;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../../../../🚪️io/📤️export/🧊️gltf/🧱️codec/🦀️.rs"]
                            pub mod codec;
                            #[path = "../../../../../🚪️io/📤️export/🧊️gltf/🧬️document/🦀️.rs"]
                            pub mod document;
                        }
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../../../../🚪️io/📤️export/📊️csv/🧱️codec/🦀️.rs"]
                            pub mod codec;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../../../../🚪️io/📤️export/🧾️json/🧱️codec/🦀️.rs"]
                            pub mod codec;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../../../../🚪️io/📤️export/🎨️svg/🧱️codec/🦀️.rs"]
                            pub mod codec;
                        }
                        #[path = "."]
                        pub mod energy {
                            #[path = "../../../../../🚪️io/📤️export/🔋️energy/🧬️target/🦀️.rs"]
                            pub mod target;
                            #[path = "../../../../../🚪️io/📤️export/🔋️energy/🔗️pairs/🦀️.rs"]
                            pub mod pairs;
                            #[path = "../../../../../🚪️io/📤️export/🔋️energy/📖️document/🦀️.rs"]
                            pub mod document;
                        }
                        #[path = "."]
                        pub mod sheets {
                            #[path = "."]
                            pub mod pdf {
                                #[path = "../../../../../🚪️io/📤️export/📄️sheets/📖️pdf/🌀️arcs/🦀️.rs"]
                                pub mod arcs;
                            }
                        }
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../../../🚪️io/📤️export/🏗️ifc/🧰️writer/🦀️.rs"]
                            pub mod writer;
                        }
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "../../../../../🚪️io/📤️export/🌿️gbxml/🧱️codec/🦀️.rs"]
                            pub mod codec;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod house {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
