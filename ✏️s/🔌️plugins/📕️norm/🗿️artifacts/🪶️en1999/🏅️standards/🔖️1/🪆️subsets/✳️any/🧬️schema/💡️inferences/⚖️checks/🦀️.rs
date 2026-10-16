//! ⚖️ EN 1999 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1999_model::{AluminiumConnection, AluminiumMaterial, AluminiumMember, AluminiumSection, AluminiumShell, ColdFormedSheet, En1999Snapshot, FatigueDetail, FireScenario, SupportCondition};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

//#region 🔖️ComplianceHelpers

fn q_force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
fn q_moment(nm: f64) -> Quantity { Quantity::new(QuantityKind::Moment, nm) }
fn q_stress(pa: f64) -> Quantity { Quantity::new(QuantityKind::Stress, pa) }
fn q_length(m: f64) -> Quantity { Quantity::new(QuantityKind::Length, m) }
fn q_temp(c: f64) -> Quantity { Quantity::new(QuantityKind::Temperature, c) }
fn q_time(s: f64) -> Quantity { Quantity::new(QuantityKind::Time, s) }
fn q_dim(v: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, v) }
fn loc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

/// 🧮 Scale a characteristic leaf so the linear design effect drops to `limit` (0.99 margin).
fn scale_char_for_limit(current: f64, design: f64, limit: f64) -> f64 {
    if design.abs() < 1e-18 {
        return 0.0;
    }
    current * (limit / design).abs() * 0.99
}

/// 🩹 Emit at-most remedies for every action leaf that still carries the demand component.
fn scale_action_leaf_remedies(
    owner_kind: &str,
    owner_id: &str,
    actions: &[semio_s_artifact_norm_en1999_model::MemberAction],
    leaf: &str,
    read: impl Fn(&semio_s_artifact_norm_en1999_model::MemberAction) -> f64,
    design: f64,
    limit: f64,
    to_q: impl Fn(f64) -> Quantity,
    label: LocalizedCopy,
    action_en: &str,
    action_de: &str,
) -> Vec<Remedy> {
    let mut out = Vec::new();
    if design.abs() <= limit.abs() + 1e-18 {
        return out;
    }
    for a in actions.iter().filter(|a| a.kind != "fire") {
        let cur = read(a);
        if cur.abs() < 1e-18 {
            continue;
        }
        let req = scale_char_for_limit(cur, design, limit);
        out.push(Remedy::at_most(
            SubjectRef::new(
                owner_id,
                format!("{owner_kind}[id={owner_id}].actions[id={}].{leaf}", a.id),
                label.clone(),
            ),
            to_q(cur),
            to_q(req),
            loc(action_en, action_de),
        ));
    }
    out
}

#[path = "📏️parts/🦀️.rs"]
mod parts;
pub use parts::*;

#[path = "🧱️member-checks/🦀️.rs"]
mod member_checks;
pub use member_checks::*;

#[path = "🔗️connection-checks/🦀️.rs"]
mod connection_checks;
pub use connection_checks::*;

#[path = "🔥️fire-fatigue-checks/🦀️.rs"]
mod fire_fatigue_checks;
pub use fire_fatigue_checks::*;

#[path = "🥫️thin-wall-checks/🦀️.rs"]
mod thin_wall_checks;
use thin_wall_checks::*;

#[path = "🧾️structure-evaluation/🦀️.rs"]
mod structure_evaluation;
pub use structure_evaluation::*;

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests

//#endregion ️ComplianceTests
