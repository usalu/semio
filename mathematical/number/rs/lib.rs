//! 🔢 Arbitrary-precision integers and rationals, modular arithmetic, primality/factorization,
//! certified interval arithmetic, and the abstract-algebra trait hierarchy (`Ring` through `Field`)
//! that `mathematical_algebra`, `mathematical_polynomial`, and `mathematical_cas` are generic over.

#[path = "src/traits.rs"]
pub mod traits;
#[path = "src/natural.rs"]
pub mod natural;
#[path = "src/integer.rs"]
pub mod integer;
#[path = "src/rational.rs"]
pub mod rational;
#[path = "src/modular.rs"]
pub mod modular;
#[path = "src/primes.rs"]
pub mod primes;
#[path = "src/interval.rs"]
pub mod interval;

pub use integer::{Integer, Sign};
pub use interval::Interval;
pub use modular::ModInt;
pub use natural::Natural;
pub use rational::Rational;
pub use traits::{field_div_rem, field_gcd, CommutativeRing, EuclideanDomain, Field, GcdDomain, IntegralDomain, Ring};
