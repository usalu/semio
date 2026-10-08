//! 📊️ Native JSON inference table projection.
use std::collections::BTreeMap;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, ADJUDICATED, measure_of};

/// ⚖️ The canonical JSON table `"<slug>|<element>+<element>" → { "measure": x }` of the adjudicated findings.
pub fn table_json(found: &[Diagnostic]) -> String {
    let table: BTreeMap<String, BTreeMap<String, f64>> = found.iter().filter(|row| ADJUDICATED.contains(&row.code)).map(|row| (format!("{}|{}", row.code.slug(), row.elements.join("+")), BTreeMap::from([("measure".to_string(), measure_of(row))]))).collect();
    semio_framework_pack_json::to_json_string(&table)
}
