//! 📤️ Generation3d export serializers: one leaf per foreign format (dwg, gltf, json, las, obj, ply, stl, txt) turning a document into bytes.


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
                    #[path = "🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
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
                    #[path = "🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
