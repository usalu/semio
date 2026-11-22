//! 🧪️ Mounts the native wire guard against the production 2D mutation library.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

pub mod standards {
    pub mod v1 {
        pub mod subsets {
            pub mod any {
                pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema;
                pub mod io {
                    pub mod binary {
                        pub mod mutations {
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::io::binary::mutations::*;
                            pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
                        }
                    }
                }
            }
        }
    }
}

#[path = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️wire-format-guard/🦀️.rs"]
mod wire_format_guard;
