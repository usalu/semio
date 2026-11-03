//! 📈️ Diagram kit: pure, locale-free geometry builders for the 2D diagrams of the Eurocode editors.

#[path = "📉️curves/🦀️.rs"]
pub mod curves;
#[path = "🧿️interaction-polygon/🦀️.rs"]
pub mod interaction_polygon;
#[path = "🌀️sn-curves/🦀️.rs"]
pub mod sn_curves;
#[path = "📜️spectra/🦀️.rs"]
pub mod spectra;
#[path = "🧱️stress-blocks/🦀️.rs"]
pub mod stress_blocks;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

/// 📍️ A point of a diagram in data coordinates.
pub type Xy = [f64; 2];
