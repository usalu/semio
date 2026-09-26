//! 🦀️ wfc bitmap 1 exhaustive mutation case — Rust adapter, the SUBJECT half.
//!
//! The oracle half is `🐍️.py` beside this file, an independent Python second implementation of the same ten
//! kinds. This adapter replays each committed quintet the scenario's doc string addresses through this subset's
//! production codec and `Mutation` implementation (`bitmap_mutation_report_json`) and asserts, in role, the laws of
//! `law::vector`: the applied snapshot is the committed after-snapshot, the produced delta is the committed `🔺️diff`,
//! the diagnostics are the committed `🎯️outcome`'s, the vector moves the document, and the mutation's own inverse
//! restores the before-snapshot. The parity phase then compares the snapshot it answers with the reference's. The `identity-round-trip` scenario decodes the committed
//! before-snapshot through the production JSON codec and re-encodes it; the document must survive unchanged.
//!
//! @see ../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🧬️vector/🦀️.rs

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::{self, vector::{self, Leaves}};
    use semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::mutations::{bitmap_mutation_report_json, bitmap_snapshot_json_round_trip};

    fn report(leaves: &Leaves) -> Result<String, String> {
        bitmap_mutation_report_json(&leaves.before, &leaves.mutation, &leaves.after)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let leaves = Leaves::read(ctx)?;
        let applied = vector::mutate(ctx.row()?, &report(&leaves)?, &leaves.vector(true))?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let leaves = Leaves::read(ctx)?;
        let restored = vector::inverse(ctx.row()?, &report(&leaves)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_fixture_uris().into_iter().next().ok_or_else(|| "the round-trip scenario names no committed snapshot".to_string())?;
        let text = String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("{uri}: {error}"))?;
        let reencoded = bitmap_snapshot_json_round_trip(&text)?;
        let reparsed = parse_json(&reencoded)?;
        law::round_trip_preserves(&reparsed, &parse_json(&text)?)?;
        Ok(Outcome::with_raw(reencoded.into_bytes(), reparsed))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration by Scenario Outline base id; each handler reads its kind from the row and its vector from the
/// row's doc string. The subject half is `sut`-gated so the oracle-only build never links the subset crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("identity-round-trip", subject::identity_round_trip);
    built
}
//#endregion 🔖️Registration
