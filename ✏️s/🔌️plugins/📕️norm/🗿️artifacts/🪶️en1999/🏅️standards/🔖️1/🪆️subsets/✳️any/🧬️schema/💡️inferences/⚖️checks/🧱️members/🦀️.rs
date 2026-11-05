//! 🧱️ EN 1999 member checks: members, connections, fire and fatigue, thin-wall shells and sheets, weld runs, outlines and free-outline stability and torsion. See [`super`].

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1999_model::{AluminiumConnection, AluminiumMaterial, AluminiumMember, AluminiumSection, AluminiumShell, ColdFormedSheet, En1999Snapshot, FatigueDetail, FireScenario, SectionVertex, SupportCondition, WeldRun};
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

//#region 🔖️ComplianceHelpers

pub fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
pub fn q_moment(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }
pub fn q_stress(pa: f64) -> Quantity { Quantity::new(QuantityKind::Stress, pa) }
pub fn q_length(m: f64) -> Quantity { Quantity::new(QuantityKind::Length, m) }
pub fn q_temp(c: f64) -> Quantity { Quantity::new(QuantityKind::Temperature, c) }
pub fn q_time(s: f64) -> Quantity { Quantity::new(QuantityKind::Time, s) }
pub fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }
pub fn loc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

pub use semio_s_artifact_norm_en1999_check_parts::*;

#[path = "../🧱️member-checks/🦀️.rs"]
mod member_checks;
pub use member_checks::*;

#[path = "../🔗️connection-checks/🦀️.rs"]
mod connection_checks;
pub use connection_checks::*;

#[path = "../🔥️fire-fatigue-checks/🦀️.rs"]
mod fire_fatigue_checks;
pub use fire_fatigue_checks::*;

#[path = "../🥫️thin-wall-checks/🦀️.rs"]
mod thin_wall_checks;
pub use thin_wall_checks::*;

#[path = "../🧵️weld-run-checks/🦀️.rs"]
mod weld_run_checks;
pub use weld_run_checks::*;

#[path = "../🔷️outline-checks/🦀️.rs"]
mod outline_checks;
pub use outline_checks::*;

#[path = "../🌀️free-outline-stability/🦀️.rs"]
mod free_outline_stability;
pub use free_outline_stability::*;

#[path = "../🪢️free-outline-torsion/🦀️.rs"]
mod free_outline_torsion;
pub use free_outline_torsion::*;
