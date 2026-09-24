//! 🫧️ `s.writer.writer` ✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `writer-writer-1-any-editor-edit-main-transient-state-lane-semantics`: the runner dispatches no oracle role, so every law is asserted inside
//! the subject handlers through `semio_s_plugin_stdio_test_oracle::law::vector` over the report of this crate's
//! production bridge `writer_main_window_transient_mutation_report_json`. The oracle handlers answer with the committed after- and before-snapshots read
//! literally, so the reference side exists the moment a second producer does. Handlers are registered by Scenario
//! Outline base id and read their kind from the row.

use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ The committed applied vector of one kind, read literally from `✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures`.
fn vector(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "set-engagement-input" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/💬️set-engagement-input/✅️set-engagement-input-applied/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/💬️set-engagement-input/✅️set-engagement-input-applied/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/💬️set-engagement-input/✅️set-engagement-input-applied/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/💬️set-engagement-input/✅️set-engagement-input-applied/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/💬️set-engagement-input/✅️set-engagement-input-applied/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-editor-selection" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/📐️set-editor-selection/✅️set-editor-selection-applied/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/📐️set-editor-selection/✅️set-editor-selection-applied/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/📐️set-editor-selection/✅️set-editor-selection-applied/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/📐️set-editor-selection/✅️set-editor-selection-applied/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/📐️set-editor-selection/✅️set-editor-selection-applied/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-lint-generation" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/🔍️set-lint-generation/✅️set-lint-generation-applied/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/🔍️set-lint-generation/✅️set-lint-generation-applied/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/🔍️set-lint-generation/✅️set-lint-generation-applied/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/🔍️set-lint-generation/✅️set-lint-generation-applied/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/🧫️fixtures/🔍️set-lint-generation/✅️set-lint-generation-applied/🎯️outcome/🔣️.json"),
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
    use semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::transient::writer_main_window_transient_mutation_report_json;

    fn report(committed: &Vector) -> Result<String, String> {
        writer_main_window_transient_mutation_report_json(committed.before, committed.mutation, committed.after)
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
