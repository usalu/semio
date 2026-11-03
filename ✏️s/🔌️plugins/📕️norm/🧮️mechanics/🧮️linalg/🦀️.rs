//! 🔢️ Dense and skyline symmetric solvers plus the symmetric and generalised eigen solvers.

#[path = "🟫️dense/🦀️.rs"]
mod dense;
pub use dense::*;

#[path = "🏔️skyline/🦀️.rs"]
mod skyline;
pub use skyline::*;

#[path = "🌌️eigen/🦀️.rs"]
mod eigen;
pub use eigen::*;
