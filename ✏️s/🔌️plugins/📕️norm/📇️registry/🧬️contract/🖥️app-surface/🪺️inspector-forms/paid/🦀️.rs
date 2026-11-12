//! 🎟️ Original descriptor, request and semantic ownership prepares bounded immutable inspector pages.
#[macro_use]
mod funding;

mod pagination;
pub use pagination::*;

mod original;
pub use original::*;

#[path="static-source.rs"]
mod static_source;
pub use static_source::*;

mod walk;

mod page;
pub use page::*;

mod choices;
pub use choices::*;
