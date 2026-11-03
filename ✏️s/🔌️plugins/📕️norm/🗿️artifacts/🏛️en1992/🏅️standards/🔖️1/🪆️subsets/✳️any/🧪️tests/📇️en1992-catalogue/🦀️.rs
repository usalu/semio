//! 🦀️ EN 1992 catalogue examples — the Rust SUBJECT adapter. Each hand-checked catalogue example is evaluated by the checks crate and must reproduce its stated value within the stated relative tolerance; the independent Python oracle answers the same rows through `🐍️.py`.

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1992_checks::{evaluate_outcomes, Case};
    use semio_s_plugin_norm_compliance::AnnexChoice;

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.example = Box::new(|_, entry, annex, numbers, texts| {
            let mut case = Case::new("feature", if annex == "De" { AnnexChoice::De } else { AnnexChoice::En });
            for (key, value) in numbers {
                case = case.with_n(key, *value);
            }
            for (key, text) in texts {
                case = case.with_s(key, text);
            }
            Ok(evaluate_outcomes(entry, &case)?.into_iter().map(|outcome| (outcome.key.to_string(), if outcome.key.starts_with("util") { outcome.utilization() } else { outcome.value() })).collect())
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
