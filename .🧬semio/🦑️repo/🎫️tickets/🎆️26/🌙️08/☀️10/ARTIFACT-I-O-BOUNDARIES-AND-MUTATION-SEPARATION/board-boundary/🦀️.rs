//! 🧪️ Actual public Board schema and IO native witnesses, isolated from the OS kernel.
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🦀️.rs"]
pub mod board_schema;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/🧬️schema/🦀️.rs"]
pub mod port_schema;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🦀️.rs"]
pub mod directed_schema;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎨️palette/🦀️.rs"]
pub mod palette_io;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/📸️snapshot/🦀️.rs"]
pub mod snapshot_io;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/👁️visibility/🦀️.rs"]
pub mod visibility_io;
pub mod infinite{pub mod board{pub use crate::board_schema::*;pub mod ports{pub use crate::port_schema::*;pub mod directed{pub use crate::directed_schema::*;}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
