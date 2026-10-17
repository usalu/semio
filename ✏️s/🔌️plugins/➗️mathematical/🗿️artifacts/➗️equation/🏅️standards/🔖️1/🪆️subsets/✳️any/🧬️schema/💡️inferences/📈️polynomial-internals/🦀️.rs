//! 📈️ Generic univariate and multivariate polynomials over the `number::` algebraic trait
//! hierarchy: arithmetic, GCD, resultants, factorization over `Z`/`Q`/`GF(p)`, Groebner bases, real
//! root isolation via Sturm sequences, and real algebraic numbers.
//!
//! 🚚 Migrated verbatim from `🧰️framework/🔨️modules/🧮️math/📈️polynomial/🦀️.rs` (ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave M3a) — see
//! `🌿️cas-internals/🦀️.rs`'s doc header (its sibling in this move) for the full rationale.
//! `crate::polynomial` self-references are preserved unedited via a crate-root `pub use … as
//! polynomial;` alias in `🦀️.rs`; `crate::number` became `number::` (wave MATHEND). Unlike
//! `🌿️cas-internals`, this module has no `algebra` dependency at all — nothing here references `cas`
//! either (verified one-directional in the wave M3a coupling map: `cas → polynomial`, never the
//! reverse).
// #region 🔖️Univariate
pub use semio_s_artifact_mathematical_equation_examples::polynomial::univariate;
// #endregion 🔖️Univariate

// #region 🔖️Multivariate
pub use semio_s_artifact_mathematical_equation_examples::polynomial::multivariate;
// #endregion 🔖️Multivariate

// #region 🔖️Finite
pub use semio_s_artifact_mathematical_equation_examples::polynomial::finite;
// #endregion 🔖️Finite

// #region 🔖️Factor
pub use semio_s_artifact_mathematical_equation_examples::polynomial::factor;
// #endregion 🔖️Factor

// #region 🔖️Roots
pub use semio_s_artifact_mathematical_equation_examples::polynomial::roots;
// #endregion 🔖️Roots

// #region 🔖️Algebraic
pub use semio_s_artifact_mathematical_equation_examples::polynomial::algebraic;
// #endregion 🔖️Algebraic

pub use algebraic::AlgebraicReal;
pub use factor::{factor_integer_poly, rational_roots};
pub use finite::{distinct_degree_factor, equal_degree_factor, factor_mod_p, is_irreducible, poly_mod_pow};
pub use multivariate::{Monomial, MonomialOrder, PolyM};
pub use roots::{cauchy_root_bound, count_roots_in, isolate_real_roots, refine_root, sturm_sequence};
pub use univariate::PolyU;
