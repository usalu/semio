//! 🧮️ Structural mechanics kernel: 3D frame statics, linear buckling, LTB, modal analysis, fastenings and geotechnics (earth pressure, slope stability) with no runtime dependencies.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

#[path = "🎛️control/🦀️.rs"]
mod control;
pub use control::Control;

#[path = "🧮️linalg/🦀️.rs"]
pub mod linalg;
use linalg::Skyline;

#[path = "🧱️elements/🦀️.rs"]
pub mod elements;

#[path = "📏️statics/🦀️.rs"]
mod statics;
pub use statics::{Analysis, MemberLoads, Raw, System};

#[path = "🌀️buckling/🦀️.rs"]
mod buckling;

#[path = "🎵️modal/🦀️.rs"]
mod modal;

#[path = "🦋️ltb/🦀️.rs"]
pub mod ltb;

#[path = "🌉️continuous-beam/🦀️.rs"]
pub mod continuous_beam;
pub use continuous_beam::{continuous_beam_envelope, permanent_arrangements, solve_continuous_beam, span_subsets, BeamActions, ContinuousBeam, SpanBeam, SpanEnvelope, SpanFactors};

#[path = "📈️moment-line/🦀️.rs"]
pub mod moment_line;
pub use moment_line::{BeamSupports, MomentLine, PointLoad, SpanLoad};

#[path = "🏛️member-modes/🦀️.rs"]
pub mod member_modes;

#[path = "🔲️section/🦀️.rs"]
pub mod section;
pub use section::{analyse_section, analyse_section_with, analyse_thin_walled};

pub use semio_s_plugin_norm_mechanics_applied::{fastening, geotechnics};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔲️section/🦀️.rs"]
mod section_tests;

#[cfg(test)]
#[path = "🧪️tests/🔩️fastening/🦀️.rs"]
mod fastening_tests;

#[cfg(test)]
#[path = "🧪️tests/🌍️geotechnics/🦀️.rs"]
mod geotechnics_tests;

#[cfg(test)]
#[path = "🧪️tests/🧵️fibre/🦀️.rs"]
mod fibre_tests;

#[cfg(test)]
#[path = "🧪️tests/⏱️latency/🦀️.rs"]
mod latency_tests;

#[cfg(test)]
#[path = "🧪️tests/📈️moment-line/🦀️.rs"]
mod moment_line_tests;
