//! 🦀️ EN 1990 catalogue fixture case — the Rust SUBJECT adapter. Every committed catalogue case is evaluated by the E90.nn kernel, the independent Python oracle (`../../🔮️oracles/⚖️compliance/🐍️.py --catalogue`) is run on the same case, and every value the oracle names must agree within one percent. Step parsing, tolerances and oracle plumbing are the shared norm step layer.

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::{numbers_of, run_oracle, Binding, Paired};
    use semio_repo_test_host::Json;
    use semio_s_artifact_norm_en1990_checks::catalogue::{evaluate_outcomes, Case};
    use semio_s_plugin_norm_compliance::AnnexChoice;

    const ORACLE: &str = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/⚖️compliance/🐍️.py";

    fn case_of(item: &Json) -> Result<Case, String> {
        let mut built = Case::new(&item.str("id"), if item.str("annex") == "De" { AnnexChoice::De } else { AnnexChoice::En });
        for (key, value) in numbers_of(item.get("num")) {
            built = built.with_n(&key, value);
        }
        if let Some(Json::Object(texts)) = item.get("text") {
            for (key, value) in texts {
                if let Json::String(text) = value {
                    built = built.with_s(key, text);
                }
            }
        }
        Ok(built)
    }

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.fixture_case = Box::new(|context, fixture, index| {
            let Json::Array(cases) = fixture else { return Err("fixture is not a list of cases".into()) };
            let item = cases.get(index).ok_or_else(|| format!("fixture has no case at index {index}"))?;
            let entry = item.str("entry");
            let id = item.str("id");
            let kernel = evaluate_outcomes(&entry, &case_of(item)?)?.into_iter().flat_map(|outcome| [(format!("{}.demand", outcome.key), outcome.demand), (format!("{}.capacity", outcome.key), outcome.capacity)]).collect();
            let uri = context.step_input_uris().into_iter().next().ok_or("scenario names no fixture")?;
            let fixture_path = context.input(&uri)?.display().to_string();
            let answer = run_oracle(context, &context.repo_root.join(ORACLE), &["catalogue", fixture_path.as_str()], None)?;
            let oracle = match answer.get(&id) {
                Some(Json::Object(keys)) => keys.iter().filter(|(key, _)| !key.starts_with('_')).flat_map(|(key, pair)| {
                    let Json::Array(pair) = pair else { return Vec::new() };
                    ["demand", "capacity"].iter().zip(pair).filter_map(|(side, value)| if let Json::Number(number) = value { Some((format!("{key}.{side}"), *number)) } else { None }).collect::<Vec<_>>()
                }).collect(),
                _ => return Err(format!("the oracle has no case {id}")),
            };
            Ok(Paired { entry, kernel, oracle, relative: if id.starts_with("form-") { 2e-4 } else { 1e-7 } })
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
