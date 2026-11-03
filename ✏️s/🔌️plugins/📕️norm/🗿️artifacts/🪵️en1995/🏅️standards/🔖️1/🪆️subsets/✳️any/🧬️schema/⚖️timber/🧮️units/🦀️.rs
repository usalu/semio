//! 🧮️ EN 1995 check units: one global unit (integrity and verdicts) and one unit per member, connection, catalogue case and typed entity (taper, cutout, fastener pattern, CLT panel, floor, diaphragm, truss, bracing), in exactly the order of `evaluate_structure`. An entity row reads its own row, the member and connection rows it references and the id columns that decide whether a reference resolves, not whole collections.

use super::{entity_ids, evaluate_scoped, Scope, ENTITY_TABLES};
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, EntityEdge, EntityRef, Rows, UnitSource};

/// 🔗️ The member and connection ids the entity row `index` of `table` refers to.
fn references_of(d: &En1995Snapshot, table: &str, index: usize) -> (Option<String>, Option<String>) {
    match table {
        "tapers" => (d.tapers.get(index).map(|r| r.member_id.clone()), None),
        "cutouts" => (d.cutouts.get(index).map(|r| r.member_id.clone()), None),
        "fasteners" => (d.fasteners.get(index).map(|r| r.member_id.clone()), d.fasteners.get(index).map(|r| r.connection_id.clone())),
        "floors" => (d.floors.get(index).map(|r| r.member_id.clone()), None),
        "bracings" => (d.bracings.get(index).map(|r| r.member_id.clone()), None),
        _ => (None, None),
    }
}

/// 📖️ The paths an entity row reads besides the shared ones and its own row: the referenced member and connection rows by position and the id column of the collection each reference resolves in (a missing target is a dangling-reference verdict listing every id).
fn reads_of(d: &En1995Snapshot, table: &str, index: usize) -> Vec<String> {
    let (member, connection) = references_of(d, table, index);
    let mut reads = Vec::new();
    if let Some(id) = member {
        reads.extend(d.members.iter().position(|m| m.id == id).map(|at| format!("members.{at}")));
        reads.push("members.*.id".to_string());
    }
    if let Some(id) = connection {
        reads.extend(d.connections.iter().position(|c| c.id == id).map(|at| format!("connections.{at}")));
        reads.push("connections.*.id".to_string());
    }
    reads
}

/// 📖️ What the global unit reads: the annex and the id column of every collection it checks for duplicates (members and connections with their action ids, the typed entities, the cases with their number and text ids); an edit of any other field leaves it clean.
const GLOBAL_READS: [&str; 18] = [
    "annex",
    "members.*.id",
    "members.*.actions.*.id",
    "connections.*.id",
    "connections.*.actions.*.id",
    "placements.*.id",
    "tapers.*.id",
    "cutouts.*.id",
    "fasteners.*.id",
    "panels.*.id",
    "floors.*.id",
    "diaphragms.*.id",
    "trusses.*.id",
    "bracings.*.id",
    "decks.*.id",
    "cases.*.id",
    "cases.*.numbers.*.id",
    "cases.*.texts.*.id",
];

/// 🧮️ The unit source of the EN 1995 timber subject: `SharedEvaluation<En1995Units>` is the one evaluation path of editor and viewer.
pub struct En1995Units;

impl UnitSource for En1995Units {
    type Document = En1995Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["members", "connections", "cases", "tapers", "cutouts", "fasteners", "panels", "floors", "diaphragms", "trusses", "bracings", "decks"], &["en1995.subject.empty", "en1995.integrity."]);

    fn units(d: &En1995Snapshot) -> Vec<CheckUnit<En1995Snapshot>> {
        let mut rows = Rows::new(["annex", "ulsEquation", "reliabilityClass"]).global_reading(&GLOBAL_READS, |d: &En1995Snapshot| evaluate_scoped(d, Scope::Globals).checks);
        for (index, member) in d.members.iter().enumerate() {
            rows = rows.row("members", "members", index, &member.id, CostTier::T1, &[], move |d: &En1995Snapshot| evaluate_scoped(d, Scope::Row("members", index)).checks);
        }
        for (index, connection) in d.connections.iter().enumerate() {
            rows = rows.row("connections", "connections", index, &connection.id, CostTier::T0, &[], move |d: &En1995Snapshot| evaluate_scoped(d, Scope::Row("connections", index)).checks);
        }
        for (index, case) in d.cases.iter().enumerate() {
            rows = rows.row("cases", "cases", index, &case.id, CostTier::T0, &[], move |d: &En1995Snapshot| evaluate_scoped(d, Scope::Row("cases", index)).checks);
        }
        for table in ENTITY_TABLES {
            for (index, id) in entity_ids(d, table).into_iter().enumerate() {
                let reads = reads_of(d, table, index);
                let reads: Vec<&str> = reads.iter().map(String::as_str).collect();
                rows = rows.row(table, table, index, &id, CostTier::T0, &reads, move |d: &En1995Snapshot| evaluate_scoped(d, Scope::Row(table, index)).checks);
            }
        }
        rows.finish()
    }

    fn references(d: &En1995Snapshot) -> Vec<EntityEdge> {
        let mut edges = Vec::new();
        for table in ENTITY_TABLES {
            for (index, id) in entity_ids(d, table).into_iter().enumerate() {
                let (member, connection) = references_of(d, table, index);
                edges.extend(member.filter(|m| d.members.iter().any(|row| row.id == *m)).map(|m| EntityEdge { from: EntityRef::new(table, &id), to: EntityRef::new("members", &m) }));
                edges.extend(connection.filter(|c| d.connections.iter().any(|row| row.id == *c)).map(|c| EntityEdge { from: EntityRef::new(table, &id), to: EntityRef::new("connections", &c) }));
            }
        }
        edges
    }
}
