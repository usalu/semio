//! ♾️ Headless computer algebra system: symbolic expressions, calculus, equation solving, symbolic
//! linear algebra, ODEs, transforms, and more, over a pure Rust API — no context handle, no thread-local
//! state; `Expr` is an ordinary `Clone`-able value built through operator overloads and free functions.

#[path = "src/fnkind.rs"]
pub mod fnkind;
#[path = "src/expr.rs"]
pub mod expr;
#[path = "src/canon.rs"]
mod canon;
#[path = "src/assume.rs"]
pub mod assume;
#[path = "src/visit.rs"]
pub mod visit;
#[path = "src/fmt.rs"]
pub mod fmt;

pub use assume::{is_even, is_integer, is_nonzero, is_positive, is_real, AssumeSet, Assumptions, RelOp as AssumeRelOp};
pub use expr::{Constant, Expr, Kind, RelOp, Symbol, WildKind};
pub use fmt::to_latex;
pub use fnkind::FnKind;
pub use visit::{contains_symbol, free_symbols, map_children, node_count, replace_bottom_up, subs, subs_many};
