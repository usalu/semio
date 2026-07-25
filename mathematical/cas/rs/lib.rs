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
#[path = "src/pattern.rs"]
pub mod pattern;
#[path = "src/polybridge.rs"]
pub mod polybridge;
#[path = "src/simplify.rs"]
pub mod simplify;
#[path = "src/trig.rs"]
pub mod trig;
#[path = "src/diff.rs"]
pub mod diff;
#[path = "src/series.rs"]
pub mod series;
#[path = "src/limits.rs"]
pub mod limits;
#[path = "src/rootof.rs"]
pub mod rootof;
#[path = "src/solve.rs"]
pub mod solve;
#[path = "src/matrix.rs"]
pub mod matrix;
#[path = "src/integrate.rs"]
pub mod integrate;
#[path = "src/sums.rs"]
pub mod sums;
#[path = "src/ode.rs"]
pub mod ode;
#[path = "src/transforms.rs"]
pub mod transforms;

pub use assume::{is_even, is_integer, is_nonzero, is_positive, is_real, AssumeSet, Assumptions, RelationalOperator as AssumeRelationalOperator};
pub use diff::{diff, gradient, idiff};
pub use expr::{Constant, Expr, Kind, RelationalOperator, Symbol, WildKind};
pub use fmt::to_latex;
pub use fnkind::FnKind;
pub use integrate::{integrate, integrate_definite, residue};
pub use limits::{limit, Direction};
pub use matrix::SymMatrix;
pub use ode::{solve_linear_constant_coeff_homogeneous, solve_ode_first_order, OdeSolution};
pub use pattern::{wild, wild_free, wild_nonzero, wild_num, wild_seq, Binding, Bindings, Rule, RuleSet, Strategy};
pub use polybridge::{as_poly, as_poly_auto, as_poly_uni, from_poly, PolyMap};
pub use rootof::{real_roots_of, root_of_expr, root_of_refine, root_of_sign, root_of_to_f64};
pub use series::{leading_term, series_to_expr, taylor_series, Series};
pub use simplify::{apart, cancel, collect, denest_sqrt, expand, factor, simplify, together};
pub use solve::{solve_inequality, solve_linear_system, solve_univariate, Bound, SolutionSet};
pub use sums::{fourier_coefficients, sum_closed};
pub use transforms::{inverse_laplace_transform, laplace_transform};
pub use trig::{expand_log, expand_trig, logcombine, powsimp, trig_canon};
pub use visit::{contains_symbol, free_symbols, map_children, node_count, replace_bottom_up, subs, subs_many};
