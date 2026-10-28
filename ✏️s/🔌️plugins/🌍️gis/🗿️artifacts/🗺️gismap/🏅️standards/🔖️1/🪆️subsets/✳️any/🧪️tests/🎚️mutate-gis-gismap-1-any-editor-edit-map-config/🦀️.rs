//! 🎚️ `s.gis.gismap` ✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `gis-gismap-1-any-editor-edit-map-config-state-lane-semantics`: the runner dispatches no oracle role, so every law is asserted inside
//! the subject handlers through `semio_repo_test_host::law::vector` over the report of this crate's
//! production bridge `map_window_config_mutation_report_json`. The oracle handlers answer with the committed after- and before-snapshots read
//! literally, so the reference side exists the moment a second producer does. Handlers are registered by Scenario
//! Outline base id and read their kind from the row.

use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_repo_test_host::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ The committed applied vector of one kind, read literally from `✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures`.
fn vector(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "set-layer-visibility" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/✅️ok/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/✅️ok/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/✅️ok/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/✅️ok/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/✅️ok/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-camera" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/✅️ok/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/✅️ok/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/✅️ok/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/✅️ok/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/✅️ok/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-render-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/✅️ok/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/✅️ok/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/✅️ok/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/✅️ok/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/✅️ok/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-vector-style" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/✅️ok/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/✅️ok/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/✅️ok/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/✅️ok/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/✅️ok/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-lod-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/✅️ok/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/✅️ok/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/✅️ok/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/✅️ok/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/✅️ok/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-layer-stroke-scale" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/✅️ok/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/✅️ok/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/✅️ok/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/✅️ok/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/✅️ok/🎯️outcome/🔣️.json"),
            observable: true,
        },
        other => return Err(format!("no committed vector for {other:?}")),
    })
}

/// 🟰️ The committed no-op vector of one kind: its before-snapshot already holds the value the mutation sets.
fn kept(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "set-layer-visibility" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/🟰️nop/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/🟰️nop/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/🟰️nop/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/🟰️nop/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️layer-visibility/🟰️nop/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-camera" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/🟰️nop/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/🟰️nop/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/🟰️nop/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/🟰️nop/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/🟰️nop/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-render-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/🟰️nop/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/🟰️nop/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/🟰️nop/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/🟰️nop/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🖼️set-render-mode/🟰️nop/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-vector-style" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/🟰️nop/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/🟰️nop/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/🟰️nop/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/🟰️nop/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🎨️set-vector-style/🟰️nop/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-lod-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/🟰️nop/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/🟰️nop/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/🟰️nop/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/🟰️nop/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/🔽️set-lod-mode/🟰️nop/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-layer-stroke-scale" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/🟰️nop/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/🟰️nop/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/🟰️nop/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/🟰️nop/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/📏️stroke-scale/🟰️nop/🎯️outcome/🔣️.json"),
            observable: false,
        },
        other => return Err(format!("no committed no-op vector for {other:?}")),
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

fn keep_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(kept(ctx.row()?)?.after)
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::*;
    use semio_repo_test_host::law::vector;
    use semio_s_artifact_gis_gismap::editor::gis2d::modes::edit::windows::map::config::component::io::text::mutations::map_window_config_mutation_report_json;

    fn report(committed: &Vector) -> Result<String, String> {
        map_window_config_mutation_report_json(committed.before, committed.mutation, committed.after)
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

    pub fn keep(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let committed = kept(kind)?;
        let unchanged = vector::mutate(kind, &report(&committed)?, &committed)?;
        Ok(Outcome::with_raw(unchanged.to_string().into_bytes(), unchanged))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("keep", keep_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("keep", subject::keep);
    }
    built
}
//#endregion 🔖️Registration
