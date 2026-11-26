//! 🧪️ Relinks the current delete inverse and its authored native law against the completed preparation graph.
extern crate semio_os_kernel as protocol;

pub use semio_s_artifact_puzzle_2d_snapshot::Puzzle2dSnapshot;
pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::delete_node_preparation::{
    Puzzle2dDeleteNodePreparationCursor, Puzzle2dDeleteNodePreparationStep,
};

#[path = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/↩️inverse/🦀️.rs"]
pub mod current_inverse;
#[path = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/🗑️delete-node/🎮️prepare/↩️inverse/🧪️tests/🦀️.rs"]
mod tests;

pub mod standards {
    pub mod v1 {
        pub mod subsets {
            pub mod any {
                pub mod schema {
                    pub use semio_s_artifact_puzzle_2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::{records, testkit};
                    pub use semio_s_artifact_puzzle_2d_snapshot::standards::v1::subsets::any::schema::{lookup, snapshot};
                    pub mod preparation {
                        pub use semio_s_artifact_puzzle_2d_mutations::standards::v1::subsets::any::schema::preparation::native_preparation_child;
                        pub mod delete_node_preparation {
                            pub use crate::{Puzzle2dDeleteNodePreparationCursor, Puzzle2dDeleteNodePreparationStep};
                            pub use crate::current_inverse as inverse;
                        }
                    }
                }
            }
        }
    }
}
