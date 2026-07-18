//! 🏛️ Headless architectural programming — program registers, adjacency, analysis, and exchange.

mod adjacency;
mod analyze;
mod exchange;
mod kernel;
mod ops;
mod program;
mod registers;
mod report;
mod search;
mod status_summary;
mod template;
mod trace;
mod validate;

pub use adjacency::*;
pub use analyze::*;
pub use exchange::*;
pub use kernel::*;
pub use ops::*;
pub use program::*;
pub use registers::*;
pub use report::*;
pub use search::*;
pub use status_summary::*;
pub use template::*;
pub use trace::*;
pub use validate::*;
