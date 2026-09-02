//! 🦀️ SEQUENCE whole-document identity round trip — Rust adapter. Relocated out of the
//! artifact-level `mutate-sequence-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
//! This scenario carries no mutation kind and no vector, so it stays with `✳️any`, the owner of the
//! committed example document, rather than joining either mutation subset.
//!
//! Recorded no-oracle decision `sequence-step-graph-mutation-semantics`
//! (`../../🧪️oracle/🔣️.json`) covers this scenario too: the committed grammar and the committed
//! artifact disagree, so a second implementation would be refused by clause.

use semio_repo_test_host::Adapter;

/// 📄️ This subset's own committed real sequence artifact.
const SEQUENCE_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";

#[cfg(feature = "sut")]
mod subject {
    use super::SEQUENCE_ASSET;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_sequence::artifacts::sequence::dsl::{parse_dsl, print_dsl};
    use semio_s_plugin_sequence::artifacts::sequence::mutations::encode_sequence_projection_json;
    use semio_s_plugin_sequence::artifacts::sequence::SequenceSnapshot;
    use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, round_trip_preserves};

    fn projection(snapshot: &SequenceSnapshot) -> Result<Json, String> {
        parse_json(&encode_sequence_projection_json(snapshot))
    }

    /// 🔁️ The identity law on the real committed bytes. The carrier is deliberately byte-exact:
    /// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` is this codec's OWN output — a semio-native
    /// envelope no foreign writer ever produced — so reproducing it exactly is the correct answer.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(SEQUENCE_ASSET)?;
        let committed = String::from_utf8(input.clone()).map_err(|error| format!("the committed sequence artifact is not UTF-8: {error}"))?;
        let decoded = parse_dsl(&committed).map_err(|error| format!("identity-round-trip: the committed sequence artifact does not parse: {error:?}"))?;
        let printed = print_dsl(&decoded);
        carrier_is_exact(printed.as_bytes(), &input)?;
        let reparsed = parse_dsl(&printed).map_err(|error| format!("identity-round-trip: this codec's own output does not parse back: {error:?}"))?;
        let after = projection(&reparsed)?;
        round_trip_preserves(&after, &projection(&decoded)?)?;
        Ok(Outcome::with_raw(printed.into_bytes(), after))
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
