//! ➗️ `s.mathematical.equation` whole-document identity round trip — Rust adapter. Relocated
//! out of the artifact-level `mutate-equation-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
//! This scenario carries no mutation kind and no vector, so it stays with `✳️any`, the owner of the
//! committed example document, rather than joining any of the three mutation subsets.
//!
//! Recorded no-oracle decision `equation-mutation-semantics` (`../../🔮️oracle/🔣️.json`)
//! covers this scenario too.

use semio_repo_test_host::Adapter;

/// 🗣️ The real committed document this artifact ships as its own example, owned by this subset.
const DSL_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";

#[cfg(feature = "sut")]
mod subject {
    use super::DSL_ASSET;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_mathematical::artifacts::equation::standards::v1::subsets::any::schema::snapshot::equation_identity_report_json;

    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }
    fn text(report: &Json, key: &str) -> Result<String, String> {
        match member(report, key)? {
            Json::String(value) => Ok(value.clone()),
            other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
        }
    }

    /// 🔁️ The real committed document through this subset's own two codecs. The semantic half is
    /// `law::round_trip_preserves`: parsing, printing back and parsing again must not move the
    /// projection. The byte half is `law::carrier_is_exact` — `store::ArtifactDsl`'s own documented
    /// LAW is that canonical `print_dsl` output is a `parse_dsl` fixpoint, so the correct answer for
    /// a second printing IS byte identity. The pack decoding is a separate binary codec, so agreeing
    /// on one snapshot cannot be reached by carrying text bytes across.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let bytes = ctx.fixture_bytes(DSL_ASSET)?;
        let dsl_text = String::from_utf8(bytes).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let report = parse_json(&equation_identity_report_json(&dsl_text).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed.clone()))
    }
}

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
