//! 🌉️ EN 1994 beam field mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../📏️change-beam-bar-spacing-m/🦀️.rs"]
pub mod change_beam_bar_spacing_m;
#[path = "../🧱️change-beam-concrete-e-cm-pa/🦀️.rs"]
pub mod change_beam_concrete_e_cm_pa;
#[path = "../🪨️change-beam-concrete-f-ck-pa/🦀️.rs"]
pub mod change_beam_concrete_f_ck_pa;
#[path = "../🧬️change-beam-crack-width-limit-m/🦀️.rs"]
pub mod change_beam_crack_width_limit_m;
#[path = "../🔁️change-beam-fatigue-cycles/🦀️.rs"]
pub mod change_beam_fatigue_cycles;
//#endregion 🪢️Mounts

pub use semio_s_artifact_norm_en1994_mut_beam_fields_more::{change_beam_hogging_rebar_m2_per_m, change_beam_ltb_length_m, change_beam_spacing_m, change_beam_support, change_beam_rebar_product};
