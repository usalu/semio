//! 🫧️ `s.wfc.bitmap` ✏️editor/🫧️transient state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `wfc-bitmap-1-any-editor-transient-state-lane-semantics`: the runner dispatches no oracle role, so every law is asserted inside
//! the subject handlers through `semio_s_plugin_stdio_test_oracle::law::vector` over the report of this crate's
//! production bridge `bitmap_transient_mutation_report_json`. The oracle handlers answer with the committed after- and before-snapshots read
//! literally, so the reference side exists the moment a second producer does. Handlers are registered by Scenario
//! Outline base id and read their kind from the row.

use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ The committed applied vector of one kind, read literally from `✏️editor/🫧️transient/🧫️fixtures`.
fn vector(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "set-solve" => Vector {
            before: include_str!("../../✏️editor/🫧️transient/🧫️fixtures/👁️set-solve/✅️set-solve-applied/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🫧️transient/🧫️fixtures/👁️set-solve/✅️set-solve-applied/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🫧️transient/🧫️fixtures/👁️set-solve/✅️set-solve-applied/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🫧️transient/🧫️fixtures/👁️set-solve/✅️set-solve-applied/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🫧️transient/🧫️fixtures/👁️set-solve/✅️set-solve-applied/🎯️outcome/🔣️.json"),
            observable: true,
        },
        other => return Err(format!("no committed vector for {other:?}")),
    })
}
//#endregion 🔖️Vectors

//#region 🔖️Oracle
fn literal(text: &str) -> Result<Outcome, String> {
    Ok(Outcome::with_raw(text.as_bytes().to_vec(), parse_json(text)?))
}

fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(vector(ctx.row()?)?.after)
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(vector(ctx.row()?)?.before)
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::*;
    use semio_s_plugin_stdio_test_oracle::law::vector;
    use semio_s_artifact_wfc_bitmap::editor::bitmap::transient::bitmap_transient_mutation_report_json;

    fn report(committed: &Vector) -> Result<String, String> {
        bitmap_transient_mutation_report_json(committed.before, committed.mutation, committed.after)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let committed = vector(kind)?;
        let applied = vector::mutate(kind, &report(&committed)?, &committed)?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let restored = vector::inverse(kind, &report(&vector(kind)?)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration
