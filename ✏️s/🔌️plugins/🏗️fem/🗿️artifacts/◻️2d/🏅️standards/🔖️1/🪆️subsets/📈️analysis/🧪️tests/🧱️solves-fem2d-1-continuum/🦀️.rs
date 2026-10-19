//! 🧱️ Portable continuum subject; the numerical reference is scikit-fem/SciPy beside this file.

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::parse_json;
    use semio_repo_test_host::Context;
    use semio_repo_test_host::Json;
    use semio_repo_test_host::Outcome;
    use semio_s_artifact_fem_2d::standards::v1::subsets::any::io::text::mutations::fem2d_continuum_report_json;

    /// 🧫️ One declared snapshot or independent committed reference.
    fn fixture(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = ctx.scenario.steps.iter().flat_map(|(_, step)| step.split_whitespace()).find(|token| token.starts_with("shared://") && token.contains(needle)).ok_or_else(|| format!("missing fixture {needle}"))?;
        String::from_utf8(ctx.input_bytes(uri)?).map_err(|error| error.to_string())
    }

    /// 📏️ Normalize response numbers by their committed physical scale.
    fn scaled(value: &Json, divisor: f64) -> Result<Json, String> {
        match value {
            Json::Number(value) if value.is_finite() => Ok(Json::Number(value / divisor)),
            Json::Array(values) => Ok(Json::Array(values.iter().map(|value| scaled(value, divisor)).collect::<Result<_, _>>()?)),
            _ => Err("expected a finite numerical response".to_string()),
        }
    }

    /// ⚖️ Normalize and hold every subject response to the independent committed answer.
    fn project(actual: &Json, expected: &Json, scales: &Json, key: &str) -> Result<Json, String> {
        if let Json::Object(fields) = actual {
            return Ok(Json::Object(fields.iter().map(|(key, value)| {
                let held = expected.get(key).ok_or_else(|| format!("missing independent response {key}"))?;
                Ok((key.clone(), project(value, held, scales, key)?))
            }).collect::<Result<_, String>>()?));
        }
        let scale_key = match key { "displacements" => "translation", "reactions" => "force", "frequenciesHz" => "frequency", "factors" => "factor", _ => "stress" };
        let divisor = match scales.get(scale_key) { Some(Json::Number(value)) if *value > 0.0 => *value, _ => return Err(format!("missing positive scale {scale_key}")) };
        let produced = scaled(actual, divisor)?;
        let held = scaled(expected, divisor)?;
        fn compare(actual: &Json, expected: &Json) -> Result<(), String> {
            match (actual, expected) {
                (Json::Number(actual), Json::Number(expected)) if (actual - expected).abs() <= 1e-6 => Ok(()),
                (Json::Array(actual), Json::Array(expected)) if actual.len() == expected.len() => actual.iter().zip(expected).try_for_each(|(actual, expected)| compare(actual, expected)),
                _ => Err(format!("continuum response differs: {} versus {}", actual.to_string(), expected.to_string())),
            }
        }
        compare(&produced, &held)?;
        Ok(produced)
    }

    /// 🧮️ Solve the authored document through the real complete-geometry static/eigen owners.
    pub fn solves(name: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx| {
            let report = parse_json(&fem2d_continuum_report_json(&fixture(ctx, name)?, matches!(name, "🔺️triangle.snapshot.json" | "🌉️mixed.snapshot.json"))?)?;
            let expected = parse_json(&fixture(ctx, "expected.results")?)?;
            let entry = expected.get(name).ok_or_else(|| format!("missing reference {name}"))?;
            let projection = project(&report, entry.get("answer").ok_or("missing answer")?, entry.get("scales").ok_or("missing scales")?, "")?;
            Ok(Outcome::with_raw(report.to_string().into_bytes(), projection))
        }
    }
}

/// 🧭️ Subject handlers only; the independent reference registers the oracle handlers.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for (suffix, name) in [("triangle", "🔺️triangle.snapshot.json"), ("mixed", "🌉️mixed.snapshot.json"), ("hole", "🕳️hole.snapshot.json"), ("scaled-hole", "📏️scaled-hole.snapshot.json")] {
            built = built.subject(&format!("continuum-{suffix}"), subject::solves(name));
        }
        return built;
    }
    #[cfg(not(feature = "sut"))]
    built
}
