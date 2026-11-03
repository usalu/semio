//! 🌍️ EN 1997 geotechnics design cases — the Rust SUBJECT adapter. Every case of the committed geotechnics case file (annex, design approach and situation named by the case, partial factors read from the NDP registry rows) is fed to the production catalogue evaluators through `oracle_wiring::geotechnics_registry`; the projection is `<case>.<quantity>`. The independent role is the Python adapter beside this file, which runs the groundhog and scipy oracle.

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_plugin_norm_compliance::oracle_driver;

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.comparison = Box::new(|context, fixture| {
            let text = String::from_utf8(context.input_bytes(fixture)?).map_err(|error| format!("{fixture}: {error}"))?;
            let registry = semio_s_artifact_norm_en1997_checks_oracle_wiring::geotechnics_registry();
            let evaluated = oracle_driver::evaluate_case_text(&text, fixture, registry);
            Ok(oracle_driver::comparison_values(&evaluated, registry))
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
