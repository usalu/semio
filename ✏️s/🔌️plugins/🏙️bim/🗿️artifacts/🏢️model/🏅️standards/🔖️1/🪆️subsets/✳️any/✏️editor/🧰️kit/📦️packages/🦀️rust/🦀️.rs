//! 🧰️ kit of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;



#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod kit {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod utilities {
            #[path = "../../../🪛️utilities/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod plan {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🗺️plan/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod world {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧊️world/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod section {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/📐️section/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod schedule {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧮️schedule/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod sheet {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/📄️sheet/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
