//! 🎚️ `s.shooting.shooting` ✏️editor/🎚️config state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `shooting-shooting-1-any-editor-config-state-lane-semantics`: the runner dispatches no oracle role, so every law is asserted inside
//! the subject handlers through `semio_repo_test_host::law::vector` over the report of this crate's
//! production bridge `shooting_config_mutation_report_json`. The oracle handlers answer with the committed after- and before-snapshots read
//! literally, so the reference side exists the moment a second producer does. Handlers are registered by Scenario
//! Outline base id and read their kind from the row.

use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_repo_test_host::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ The committed applied vector of one kind, read literally from `✏️editor/🎚️config/🧫️fixtures`.
fn vector(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "replace-config" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace/✅️replace/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace/✅️replace/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace/✅️replace/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace/✅️replace/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace/✅️replace/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-shot-selection" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/☑️set-shot/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/☑️set-shot/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/☑️set-shot/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/☑️set-shot/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/☑️set-shot/✅️set/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-center-model" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎯️set-center/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎯️set-center/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎯️set-center/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎯️set-center/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎯️set-center/✅️set/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-fit-revision" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔢️set-fit/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔢️set-fit/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔢️set-fit/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔢️set-fit/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔢️set-fit/✅️set/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-camera" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-defaults" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔧️set-defaults/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔧️set-defaults/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔧️set-defaults/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔧️set-defaults/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🔧️set-defaults/✅️set/🎯️outcome/🔣️.json"),
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
    use semio_repo_test_host::law::vector;
    use semio_s_artifact_shooting_shooting::editor::shooting::config::component::io::text::mutations::shooting_config_mutation_report_json;

    fn report(committed: &Vector) -> Result<String, String> {
        shooting_config_mutation_report_json(committed.before, committed.mutation, committed.after)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let committed = vector(kind)?;
        let applied = vector::mutate(kind, &report(&committed)?, &committed)?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let restored = vector::inverse(kind, &report(&vector(kind)?)?).expect("valid retained mutation inverse fixture")?;
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
