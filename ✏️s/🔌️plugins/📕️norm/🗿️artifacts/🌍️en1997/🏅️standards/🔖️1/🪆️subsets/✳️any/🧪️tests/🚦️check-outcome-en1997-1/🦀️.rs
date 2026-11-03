//! 🦀️ EN 1997 check outcome — the Rust SUBJECT adapter. The committed before-snapshot is decoded with the io crate, evaluated by the production `evaluate` and the status and utilisation of the named check are compared by the feature's relative tolerance with the independent Python oracle in the oracle role. Step plumbing is the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::{pick_outcome, Binding};
    use semio_s_artifact_norm_en1997_inferences::evaluate;
    use semio_s_artifact_norm_en1997_io::text::snapshot::decode_en1997_snapshot_json;
    use semio_s_plugin_norm_compliance::CheckStatus;

    fn status_name(status: &CheckStatus) -> &'static str {
        match status {
            CheckStatus::Pass => "pass",
            CheckStatus::Warning => "warning",
            CheckStatus::Fail => "fail",
            CheckStatus::NotApplicable => "notApplicable",
            CheckStatus::UnverifiedBasis => "unverifiedBasis",
            CheckStatus::Info => "info",
        }
    }

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.check = Box::new(|context, fixture, check| {
            let text = String::from_utf8(context.input_bytes(fixture)?).map_err(|error| format!("{fixture}: {error}"))?;
            let document = decode_en1997_snapshot_json(&text).map_err(|error| format!("{fixture}: {error}"))?;
            let rows: Vec<(String, &'static str, f64)> = evaluate(&document).checks.iter().map(|row| (row.id.clone(), status_name(&row.status), row.utilization)).collect();
            pick_outcome(&rows, check)
        });
        bound
    }
}

/// 🧭️ Registration is by scenario family; the oracle role is the Python adapter beside this file.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = steps::norm_adapter(subject::binding());
    }
    built
}
