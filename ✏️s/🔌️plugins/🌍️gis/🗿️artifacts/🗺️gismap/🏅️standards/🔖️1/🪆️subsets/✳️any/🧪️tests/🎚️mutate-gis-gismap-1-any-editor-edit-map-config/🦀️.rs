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
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/✅️set-layer/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/✅️set-layer/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/✅️set-layer/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/✅️set-layer/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/✅️set-layer/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-camera" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/✅️set/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-render-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/✅️set-render/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/✅️set-render/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/✅️set-render/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/✅️set-render/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/✅️set-render/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-vector-style" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/✅️set-vector/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/✅️set-vector/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/✅️set-vector/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/✅️set-vector/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/✅️set-vector/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-lod-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/✅️set-lod-mode/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/✅️set-lod-mode/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/✅️set-lod-mode/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/✅️set-lod-mode/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/✅️set-lod-mode/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-layer-stroke-scale" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/✅️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/✅️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/✅️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/✅️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/✅️set/🎯️outcome/🔣️.json"),
            observable: true,
        },
        other => return Err(format!("no committed vector for {other:?}")),
    })
}

/// 🟰️ The committed no-op vector of one kind: its before-snapshot already holds the value the mutation sets.
fn kept(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "set-layer-visibility" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/🟰️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/🟰️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/🟰️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/🟰️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/👁️set-layer-visibility/🟰️set/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-camera" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/🟰️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/🟰️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/🟰️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/🟰️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎥️set-camera/🟰️set/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-render-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/🟰️set-render/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/🟰️set-render/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/🟰️set-render/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/🟰️set-render/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🖼️set-render-mode/🟰️set-render/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-vector-style" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/🟰️set-vector/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/🟰️set-vector/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/🟰️set-vector/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/🟰️set-vector/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🎨️set-vector-style/🟰️set-vector/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-lod-mode" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/🟰️set-lod-mode-no/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/🟰️set-lod-mode-no/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/🟰️set-lod-mode-no/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/🟰️set-lod-mode-no/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/🔽️set-lod-mode/🟰️set-lod-mode-no/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-layer-stroke-scale" => Vector {
            before: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/🟰️set/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/🟰️set/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/🟰️set/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/🟰️set/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧫️fixtures/📏️set-layer-stroke-scale/🟰️set/🎯️outcome/🔣️.json"),
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
    use semio_s_artifact_gis_gismap::editor::gis2d::modes::edit::windows::map::config::mutations::map_window_config_mutation_report_json;

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
        let restored = vector::inverse(kind, &report(&vector(kind)?)?)?;
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
