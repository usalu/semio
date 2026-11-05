//! 🎚️ Shared Forms host contribution projection and its config protocol.
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

#[path = "."]
pub mod forms_config {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../🧬️schema/🦀️.rs"]
    pub mod schema;
    #[path = "."]
    pub mod io {
        #[path = "."]
        pub mod text {
            #[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
            pub mod mutations;
        }
        #[path = "."]
        pub mod binary {
            #[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
            pub mod mutations;
        }
    }
}
