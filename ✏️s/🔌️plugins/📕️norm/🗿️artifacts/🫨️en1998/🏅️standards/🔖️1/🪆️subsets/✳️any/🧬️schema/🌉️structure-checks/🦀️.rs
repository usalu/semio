//! 🌉️ Structure checks of EN 1998 parts 2, 4 and 6: the spectrum analysis of the bridge rows, the hydrodynamic pressure and sloshing of the tanks, and the modal response and the foundation interaction of the towers.

extern crate semio_s_artifact_norm_en1998_model as model;
extern crate semio_s_artifact_norm_en1998_schema as schema;
extern crate semio_s_plugin_norm_compliance as compliance;

use compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use model::En1998Snapshot;
use schema::{part_1, part_2, part_3};

fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }

fn q_len(m: f64) -> Quantity { Quantity::length_m(m) }

fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }

fn q_press(pa: f64) -> Quantity { Quantity::new(QuantityKind::Pressure, pa) }

fn q_mom(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }

/// 🛡️ A resistance of zero or less is floored to a tiny positive limit so a positive demand fails instead of passing at utilisation 0.
fn floor_limit(limit: Quantity) -> Quantity { Quantity::new(limit.kind, limit.value.max(1e-9)) }

fn lc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

fn id_sel(collection: &str, id: &str) -> String {
    format!("{collection}[id={id}]")
}

#[path = "🌉️bridge-spectrum/🦀️.rs"]
mod bridge_spectrum;
pub use bridge_spectrum::{analyse as bridge_analysis, check_rows as check_bridge_rows, Analysis, Ground};

#[path = "🌉️bridge-action/🦀️.rs"]
mod bridge_action;
pub use bridge_action::{check_action as check_bridge_action, check_piers as check_bridge_piers};

#[path = "🛢️tank-pressure/🦀️.rs"]
mod tank_pressure;
pub use tank_pressure::check_tank_hydrodynamics;

#[path = "🗼️tower-modal/🦀️.rs"]
mod tower_modal;
pub use tower_modal::check_tower_modal;

#[path = "🛞️isolation/🦀️.rs"]
mod isolation_checks;
pub use isolation_checks::{check_system as check_isolation_system, Supported as IsolationSupported};

#[path = "🪨️pier-base/🦀️.rs"]
mod pier_base;
pub use pier_base::check_pier_base;
