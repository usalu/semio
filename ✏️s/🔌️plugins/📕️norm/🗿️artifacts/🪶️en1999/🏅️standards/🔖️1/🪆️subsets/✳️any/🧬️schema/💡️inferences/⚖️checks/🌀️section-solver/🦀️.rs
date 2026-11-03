//! 🌀️ Stability helpers of a free outline: principal-axis frames, Wagner coefficient, lateral-torsional and torsional-flexural elastic critical loads. The St Venant torsion constant, warping constant, shear centre and shear areas come from the finite-element section solver of the shared mechanics kernel (`semio_s_plugin_norm_mechanics::analyse_section`, validated against `sectionproperties`).

pub type Pt = (f64, f64);

#[path = "🧭️stability/🦀️.rs"]
pub mod stability;

#[cfg(test)]
#[path = "🧪️tests/🧭️stability/🦀️.rs"]
mod stability_tests;
