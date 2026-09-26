//! 🦀️ wfc2d 1 exhaustive mutation case — Rust adapter, the SUBJECT half.
//!
//! The oracle half is `🐍️.py` beside this file, an independent Python second implementation of the same fifteen
//! kinds. This adapter replays each committed quintet the scenario's doc string addresses through this subset's
//! production codec and `Mutation` implementation (`wfc2d_mutation_report_json`) and asserts, in role, the laws of
//! `law::vector`: the applied snapshot is the committed after-snapshot, the produced delta is the committed `🔺️diff`,
//! the diagnostics are the committed `🎯️outcome`'s, the vector moves the document, and the mutation's own inverse
//! restores the before-snapshot. The parity phase then compares the snapshot it answers with the reference's.
//!
//! @see ../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🧬️vector/🦀️.rs

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_artifact_wfc_2d::standards::v1::subsets::any::schema::mutations::wfc2d_mutation_report_json;
    use semio_s_plugin_stdio_test_oracle::law::vector::{self, Leaves};

    fn report(leaves: &Leaves) -> Result<String, String> {
        wfc2d_mutation_report_json(&leaves.before, &leaves.mutation, &leaves.after)
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
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration by Scenario Outline base id; each handler reads its kind from the row and its vector from the
/// row's doc string. The subject half is `sut`-gated so the oracle-only build never links the subset crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    built
}
//#endregion 🔖️Registration
