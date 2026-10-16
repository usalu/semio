//! ♾️ Headless computer algebra system: symbolic expressions, calculus, equation solving, symbolic
//! linear algebra, ODEs, transforms, and more, over a pure Rust API — no context handle, no thread-local
//! state; `Expr` is an ordinary `Clone`-able value built through operator overloads and free functions.
//!
//! 🚚 Migrated verbatim from `🧰️framework/🔨️modules/🧮️math/🧮️cas/🦀️.rs` (ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave M3a): `🧮️math` is being
//! dissolved into event-sourced artifacts and `🧮️cas` has no independent artifact home of its own —
//! this crate's `➗️equation` plugin is where equation content actually lives. This module is the
//! Rust-only compute internals a `💡️inferences/<slug>/` facet's `compute()` delegates into (mirroring
//! stdio's `📐️step` io facet's `🪜️ladder`/`📐️part21`/`🧱️brep` precedent for deep Rust-only helper
//! dirs under a facet); `crate::algebra`'s two call sites (`MatG`/`VecG` for exact linear-system
//! solving in `apart`'s partial-fraction decomposition, and in `matrix::SymMatrix`'s numeric paths)
//! became `math::algebra::MatG`/`math::algebra::VecG` against the `extern crate semio_framework_math
//! as math;` dependency — briefly a genuine pre-existing breakage (`E0433: cannot find algebra in
//! math`), since wave M3d had moved `algebra` out of `semio_framework_math` into `📸️remodel` on the
//! belief that `📸️remodel` was its sole consumer, not knowing this wave had just created a second one.
//! Wave FIXALG (same ticket) resolved it by relocating `VecG`/`MatG` out of `📸️remodel` into
//! `semio_framework_number`'s own `algebra` module — these two sites now read `number::MatG`/
//! `number::VecG`, and the now-unused `semio_framework_math` dependency is gone from this crate's
//! `Cargo.toml`. `crate::number` became `number::` (wave MATHEND) — `number` was relocated out of
//! `🧮️math` into its own framework module.
//! Every `crate::cas`/`crate::polynomial` self-reference is preserved unedited via a crate-root
//! `pub use … as cas;` / `pub use … as polynomial;` alias in `🦀️.rs` — the physical file moved,
//! the module's internal wiring did not.
// #region 🔖️Fnkind
#[path = "🏷️fnkind/🦀️.rs"]
pub mod fnkind;
// #endregion 🔖️Fnkind

// #region 🔖️Expr
#[path = "🧮️expr/🦀️.rs"]
pub mod expr;
// #endregion 🔖️Expr

// #region 🔖️Canon
#[path = "📏️canon/🦀️.rs"]
mod canon;
// #endregion 🔖️Canon

// #region 🔖️Assume
#[path = "🧷️assume/🦀️.rs"]
pub mod assume;
// #endregion 🔖️Assume

// #region 🔖️Visit
#[path = "🧭️visit/🦀️.rs"]
pub mod visit;
// #endregion 🔖️Visit

// #region 🔖️Fmt
#[path = "🖨️fmt/🦀️.rs"]
pub mod fmt;
// #endregion 🔖️Fmt

// #region 🔖️Pattern
#[path = "🧩️pattern/🦀️.rs"]
pub mod pattern;
// #endregion 🔖️Pattern

// #region 🔖️Polybridge
#[path = "🌉️polybridge/🦀️.rs"]
pub mod polybridge;
// #endregion 🔖️Polybridge

// #region 🔖️Simplify
#[path = "🧹️simplify/🦀️.rs"]
pub mod simplify;
// #endregion 🔖️Simplify

// #region 🔖️Trig
#[path = "📐️trig/🦀️.rs"]
pub mod trig;
// #endregion 🔖️Trig

// #region 🔖️Diff
#[path = "📈️diff/🦀️.rs"]
pub mod diff;
// #endregion 🔖️Diff

// #region 🔖️Series
#[path = "🎞️series/🦀️.rs"]
pub mod series;
// #endregion 🔖️Series

// #region 🔖️Limits
#[path = "🎯️limits/🦀️.rs"]
pub mod limits;
// #endregion 🔖️Limits

// #region 🔖️Rootof
#[path = "🌱️rootof/🦀️.rs"]
pub mod rootof;
// #endregion 🔖️Rootof

// #region 🔖️Solve
#[path = "🧠️solve/🦀️.rs"]
pub mod solve;
// #endregion 🔖️Solve

// #region 🔖️Matrix
#[path = "🔢️matrix/🦀️.rs"]
pub mod matrix;
// #endregion 🔖️Matrix

// #region 🔖️Integrate
#[path = "➗️integrate/🦀️.rs"]
pub mod integrate;
// #endregion 🔖️Integrate

// #region 🔖️Sums
#[path = "➕️sums/🦀️.rs"]
pub mod sums;
// #endregion 🔖️Sums

// #region 🔖️Ode
#[path = "🌊️ode/🦀️.rs"]
pub mod ode;
// #endregion 🔖️Ode

// #region 🔖️Transforms
#[path = "🔄️transforms/🦀️.rs"]
pub mod transforms;
// #endregion 🔖️Transforms

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
