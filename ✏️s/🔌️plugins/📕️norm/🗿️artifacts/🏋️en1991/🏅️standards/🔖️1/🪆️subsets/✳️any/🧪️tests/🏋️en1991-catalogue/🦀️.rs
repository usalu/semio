//! 🦀️ EN 1991 catalogue examples — the Rust SUBJECT adapter. Each hand-checked catalogue example is evaluated by the kernel and must reproduce its stated value within the stated relative tolerance; the independent Python oracle answers the same rows in the oracle role. Step parsing, tolerances and plumbing are the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1991_checks::catalogue::{evaluate_outcomes, Case};
    use semio_s_plugin_norm_compliance::AnnexChoice;

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.example = Box::new(|_, entry, annex, numbers, texts| {
            let mut case = Case::new("feature", if annex == "de" { AnnexChoice::De } else { AnnexChoice::En });
            let mut vectors: std::collections::BTreeMap<String, Vec<(usize, f64)>> = std::collections::BTreeMap::new();
            for (key, value) in numbers {
                match key.rsplit_once('.').and_then(|(name, index)| index.parse::<usize>().ok().map(|index| (name, index))) {
                    Some((name, index)) => vectors.entry(name.into()).or_default().push((index, *value)),
                    None => case = case.with_n(key, *value),
                }
            }
            for (name, mut entries) in vectors {
                entries.sort_by_key(|(index, _)| *index);
                case = case.with_v(&name, &entries.iter().map(|(_, value)| *value).collect::<Vec<_>>());
            }
            for (key, text) in texts {
                case = case.with_s(key, text);
            }
            Ok(evaluate_outcomes(entry, &case)?.into_iter().map(|outcome| (outcome.key.to_string(), outcome.value)).collect())
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
