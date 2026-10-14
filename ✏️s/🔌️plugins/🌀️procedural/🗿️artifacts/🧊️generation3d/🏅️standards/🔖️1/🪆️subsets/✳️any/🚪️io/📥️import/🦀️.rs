//! 📥️ Generation3d import deserializers: one leaf per foreign format (dwg, gltf, json, obj, ply, stl, txt) turning bytes into a document.


#[path = "."]
pub mod deserializers {
    #[path = "."]
    pub mod artifacts {
        #[path = "."]
        pub mod ply {
            #[path = "."]
            pub mod v1_0 {
                #[path = "."]
                pub mod any {
                    #[path = "🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                    #[path = "🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                    #[path = "🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                    #[path = "🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                    #[path = "🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                    #[path = "🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
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
                    #[path = "🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
