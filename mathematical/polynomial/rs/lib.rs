//! 📈 Generic univariate and multivariate polynomials over the `mathematical_number` algebraic trait
//! hierarchy: arithmetic, GCD, resultants, factorization over `Z`/`Q`/`GF(p)`, Groebner bases, real
//! root isolation via Sturm sequences, and real algebraic numbers.

#[path = "src/univariate.rs"]
pub mod univariate;
#[path = "src/multivariate.rs"]
pub mod multivariate;
#[path = "src/finite.rs"]
pub mod finite;
#[path = "src/factor.rs"]
pub mod factor;
#[path = "src/roots.rs"]
pub mod roots;
#[path = "src/algebraic.rs"]
pub mod algebraic;

pub use algebraic::AlgebraicReal;
pub use factor::{factor_integer_poly, rational_roots};
pub use finite::{distinct_degree_factor, equal_degree_factor, factor_mod_p, is_irreducible, poly_mod_pow};
pub use multivariate::{Monomial, MonomialOrder, PolyM};
pub use roots::{cauchy_root_bound, count_roots_in, isolate_real_roots, refine_root, sturm_sequence};
pub use univariate::PolyU;
