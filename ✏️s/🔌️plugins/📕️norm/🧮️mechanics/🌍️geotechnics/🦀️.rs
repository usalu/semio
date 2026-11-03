//! 🌍️ Domain-neutral geotechnical kernel in N, m and degrees: Mononobe–Okabe earth pressure with water, and pseudo-static slope stability by slices. Every annex-driven value (seismic coefficients, unit weights, permeability class) is an argument; EN 1997 and EN 1998 call the same functions. See [EN 1998-5 Annex E](https://eurocodes.jrc.ec.europa.eu/).

#[path = "🧱️earth-pressure/🦀️.rs"]
pub mod earth_pressure;

#[path = "⛰️slope/🦀️.rs"]
pub mod slope;
