//! 🏛️ EN 1993 annex clause functions in mm, N, MPa (schema-neutral, numbers in and out): external pressure on ring-stiffened shells (1-6), corrugated webs (1-5 Annex D), orthotropic deck strips (2), purlin free flanges (1-3) and stiffened plates (1-5 Annex A). See [EN 1993](https://eurocodes.jrc.ec.europa.eu/).

#[path = "🛢️shell-pressure/🦀️.rs"]
pub mod shell_pressure;
#[path = "🧱️corrugated-web/🦀️.rs"]
pub mod corrugated_web;
#[path = "🌉️orthotropic-deck/🦀️.rs"]
pub mod orthotropic_deck;
#[path = "🏠️purlin-free-flange/🦀️.rs"]
pub mod purlin_free_flange;
#[path = "📚️stiffened-plate/🦀️.rs"]
pub mod stiffened_plate;
