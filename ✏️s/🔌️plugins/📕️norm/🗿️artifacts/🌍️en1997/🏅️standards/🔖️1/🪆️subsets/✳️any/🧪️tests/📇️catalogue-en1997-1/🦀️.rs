//! 🦀️ EN 1997 catalogue fixture case — the Rust SUBJECT adapter. Every committed catalogue case is evaluated by the catalogue kernel, the independent Python oracle (`../../🔮️oracles/⚖️compliance/🐍️.py --catalogue`) is run on the same case, and every value the oracle names must agree within one percent. Step parsing, tolerances and oracle plumbing are the shared norm step layer.

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::{numbers_of, run_oracle, Binding, Paired};
    use semio_repo_test_host::Json;
    use semio_s_artifact_norm_en1997_checks_catalogue::catalogue::{evaluate_outcomes, Case};
    use semio_s_plugin_norm_compliance::AnnexChoice;

    const ORACLE: &str = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/⚖️compliance/🐍️.py";

    fn case_of(item: &Json) -> Result<Case, String> {
        let Some(body) = item.get("case") else { return Err("catalogue item has no case".into()) };
        let mut built = Case::new("fixture", if body.str("annex") == "De" { AnnexChoice::De } else { AnnexChoice::En });
        for (key, value) in numbers_of(body.get("num")) {
            built = built.with_n(&key, value);
        }
        if let Some(Json::Object(texts)) = body.get("text") {
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
            let cases = fixture.array("cases");
            let item = cases.get(index).ok_or_else(|| format!("fixture has no case at index {index}"))?;
            let entry = item.str("entry");
            let kernel = evaluate_outcomes(&entry, &case_of(item)?)?.into_iter().map(|outcome| (outcome.key.clone(), outcome.value())).collect();
            let request = Json::Object(vec![("cases".into(), Json::Array(vec![item.clone()]))]).to_string();
            let answer = run_oracle(context, &context.repo_root.join(ORACLE), &["--catalogue"], Some(&request))?;
            let oracle = numbers_of(answer.array("cases").first().and_then(|first| first.get("values")));
            Ok(Paired { entry, kernel, oracle, relative: 0.01 })
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
