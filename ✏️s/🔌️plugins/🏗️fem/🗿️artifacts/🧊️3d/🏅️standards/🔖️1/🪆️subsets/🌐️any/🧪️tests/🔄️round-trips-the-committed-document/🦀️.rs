//! 🧊 `s.fem.fem3d` whole-document identity round trip — Rust SUBJECT adapter. Relocated out of
//! the artifact-level `mutate-fem3d-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
//! This scenario carries no mutation kind and no vector, so it stays with `✳️any`, the owner of the
//! committed example document, rather than joining any of the five mutation subsets.
//!
//! The reference is `🐍️.py` beside this file — a second implementation that reads the same
//! derived model. This adapter registers the SUBJECT half only.

use semio_repo_test_host::Adapter;

/// 🗣️ The real committed document this artifact ships as its own example, owned by this subset.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";

/// 🧫️ The same derived steel frame model every fem3d mutation subset case shares, as its own
/// local copy.
#[cfg(feature = "sut")]
const DERIVED_ASSET: &str = "local://🧊️steel-frame.snapshot.json";

#[cfg(feature = "sut")]
mod subject {
    use super::{DERIVED_ASSET, DSL_ASSET};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_fem::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::fem3d_mutation_report_json;
    use semio_s_plugin_fem::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::fem3d_identity_report_json;

    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }

    fn text(report: &Json, key: &str) -> Result<String, String> {
        match member(report, key)? {
            Json::String(value) => Ok(value.clone()),
            other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
        }
    }

    /// 🧭️ The one report the production bridge produces for a `(base, mutation)` pair.
    fn report_of(scenario: &str, base: &str, mutation: &str, after: &str) -> Result<Json, String> {
        parse_json(&fem3d_mutation_report_json(base, mutation, after).map_err(|error| format!("{scenario}: the input did not reach this subset's own codec: {error}"))?)
    }

    /// 🧭️ A payload that reaches the bridge's decode without applying an edit: it names the
    /// analysis settings the derived model already holds, so `base` and `snapshot` agree and only
    /// the decode is exercised.
    const IDENTITY_PROBE: &str = "{\"mutation\":\"updateAnalysisSettings\",\"settings\":{\"modalCount\":3,\"bucklingCount\":3,\"deformationScale\":300.0}}";

    /// 🔁️ Two identities in one scenario, because they can only be asserted in two different
    /// places.
    ///
    /// The CARRIER identity is Rust-only and asserted here in role, on the artifact's own committed
    /// example: `law::round_trip_preserves` for the semantic half, `law::carrier_is_exact` for the
    /// byte half — deliberately the fixpoint law rather than the wave's no-pass-through tripwire,
    /// because `store::ArtifactDsl` documents canonical `print_dsl` output as a `parse_dsl`
    /// fixpoint. The pack decoding is a separate binary codec, so agreeing on one model cannot be
    /// reached by carrying text bytes across.
    ///
    /// The MODEL identity is what the Python reference can also produce: the nine members this
    /// subset's own JSON codec reads out of the derived real frame.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = ctx.fixture_bytes(DSL_ASSET)?;
        let text_value = String::from_utf8(committed.clone()).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let report = parse_json(&fem3d_identity_report_json(&text_value).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        let derived = String::from_utf8(ctx.fixture_bytes(DERIVED_ASSET)?).map_err(|error| format!("identity-round-trip: the derived model is not UTF-8: {error}"))?;
        let probe = report_of("identity-round-trip", &derived, IDENTITY_PROBE, &derived)?;
        let base = member(&probe, "base")?;
        Ok(Outcome::with_raw(base.to_string().into_bytes(), base.clone()))
    }
}

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        return built.subject("identity-round-trip", subject::round_trip);
    }
    #[cfg(not(feature = "sut"))]
    built
}
