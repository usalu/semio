//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::standards::v1::subsets::any::schema::snapshot::{decode_en1997_dsl, compliant_demo, noncompliant_demo};
use crate::{En1997Mutation, En1997Snapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

fn from_primary(text: &str, fallback: En1997Snapshot) -> En1997Snapshot {
    if text.trim().is_empty() {
        return fallback;
    }
    decode_en1997_dsl(text).unwrap_or(fallback)
}

/// 🎨️ Replaces the live document with a named geotechnical example subject (DSL assets as PRIMARY_TEXT).
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1997Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1997Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" | "demo" => from_primary(crate::examples::demo::PRIMARY_TEXT, En1997Snapshot::default()),
        "compliant" => from_primary(crate::examples::compliant::PRIMARY_TEXT, compliant_demo()),
        "noncompliant" | "failing" => from_primary(crate::examples::noncompliant::PRIMARY_TEXT, noncompliant_demo()),
        id if id == crate::examples::demo::ID => from_primary(crate::examples::demo::PRIMARY_TEXT, En1997Snapshot::default()),
        id if id == crate::examples::compliant::ID => from_primary(crate::examples::compliant::PRIMARY_TEXT, compliant_demo()),
        id if id == crate::examples::noncompliant::ID => from_primary(crate::examples::noncompliant::PRIMARY_TEXT, noncompliant_demo()),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
