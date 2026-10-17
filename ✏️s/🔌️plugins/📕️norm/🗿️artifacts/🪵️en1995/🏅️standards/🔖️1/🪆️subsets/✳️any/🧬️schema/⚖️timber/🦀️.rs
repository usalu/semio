//! 🪵 EN 1995-1-1 / DIN NA / 1-2 / 2 timber compliance formulas and check builders.

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckBuilder, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1995_model::{CharacteristicAction, ConnectionAction, MemberRole, SupportType, TimberConnection, TimberMember, TimberProduct, TimberProperties};

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}
fn member_subject(m: &TimberMember, leaf: &str) -> SubjectRef {
    SubjectRef::new(&m.id, format!("members[id={}].{leaf}", m.id), loc(&m.label_en, &m.label_de))
}
fn conn_subject(c: &TimberConnection, leaf: &str) -> SubjectRef {
    SubjectRef::new(&c.id, format!("connections[id={}].{leaf}", c.id), loc(&c.label_en, &c.label_de))
}

#[path = "🌲️material-and-annex-factors/🦀️.rs"]
mod material_and_annex_factors;
pub use material_and_annex_factors::*;

#[path = "🔩️fasteners-and-johansen/🦀️.rs"]
mod fasteners_and_johansen;
pub use fasteners_and_johansen::*;

#[path = "🧮️load-combinations/🦀️.rs"]
mod load_combinations;
pub use load_combinations::*;

#[path = "🧰️assessment-vocabulary/🦀️.rs"]
mod assessment_vocabulary;
pub use assessment_vocabulary::*;

#[path = "🏗️structure-and-members/🦀️.rs"]
mod structure_and_members;

#[path = "🌉️bridge-and-connections/🦀️.rs"]
mod bridge_and_connections;
use bridge_and_connections::*;
