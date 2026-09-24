//! 🦀️ glTF 2.0 `@SUBSET@` subset mutation case — Rust adapter for the @COUNT@ kinds of the
//! `gltf-2-0-@SHORT@` catalog (`../../🔮️oracles/🔣️.json`), which own @SURFACE@. Every leaf stays
//! physically owned by `♾️any`'s aggregate mutation root and is reached by import.
//!
//! The oracle performs each kind by independent GLB/JSON-tree manipulation
//! (`../../../♾️any/🔮️oracles/🦀️.rs`); the subject fully parses the committed `⬅️before.gltf` into
//! `GltfSnapshot`, dispatches through the leaf's own typed `apply()`, and re-serializes from the
//! model alone. The feature's spec names the input fixture and the inverse: either the kinds that
//! undo the mutation, applied the same way on both sides, or — for a kind whose payload cannot
//! carry what it removed — the top-level members restored from the original document.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf, restore_members};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within};

//#region 🔖️Kinds
const KINDS: &[&str] = &[
    @KINDS@,
];
//#endregion 🔖️Kinds

//#region 🔖️Spec
/// 📏️ The `semantic-gltf-v1` writer freedom every glTF mutation case is measured under.
const GLTF_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "generator", "copyright"];

/// 🧫️ A work-directory copy of the spec's own `fixture`; the committed fixture is never written to.
fn mutable_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(&spec.str("fixture"), Some("input.gltf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// ↩️ The spec's inverse kinds, in application order.
fn inverse_specs(spec: &Json) -> Vec<Json> {
    match spec.get("inverse") {
        Some(Json::Array(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// ↩️ The top-level members the inverse restores from the original document, when it does.
fn restored_members(spec: &Json) -> Option<Vec<String>> {
    match spec.get("restore") {
        Some(Json::Array(items)) => Some(
            items
                .iter()
                .filter_map(|item| match item {
                    Json::String(member) => Some(member.clone()),
                    _ => None,
                })
                .collect(),
        ),
        _ => None,
    }
}
//#endregion 🔖️Spec

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, &spec)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_gltf(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_gltf(&input)?, &[], GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, &spec)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = match restored_members(&spec) {
        Some(members) => restore_members(&mutated, &input, &members)?,
        None => inverse_specs(&spec).iter().try_fold(mutated, |current, inverse| oracle_apply_mutation(&current, inverse))?,
    };
    let projection = project_gltf(&restored)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_specs, mutable_input, restored_members};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{@IO@};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::mutations::{@MODULES@};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::snapshot::@SNAPSHOT@;
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

@HELPERS@

    //#region 🔖️Dispatch
    /// 📐️ One kind through its leaf's own typed `apply()`.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
@ARMS@
            other => Err(format!("unrecognised mutation kind {other:?}")),
        }
    }

    /// ↩️ The named top-level members copied back from `before`.
    fn restore(before: &GltfSnapshot, mutated: &GltfSnapshot, members: &[String]) -> Result<GltfSnapshot, String> {
        let mut restored = mutated.clone();
        for member in members {
            match member.as_str() {
                "scene" => restored.document.scene = before.document.scene,
                "scenes" => restored.document.scenes = before.document.scenes.clone(),
                "nodes" => restored.document.nodes = before.document.nodes.clone(),
                "skins" => restored.document.skins = before.document.skins.clone(),
                "animations" => restored.document.animations = before.document.animations.clone(),
                "meshes" => restored.document.meshes = before.document.meshes.clone(),
                "accessors" => restored.document.accessors = before.document.accessors.clone(),
                "images" => restored.document.images = before.document.images.clone(),
                "bufferViews" => restored.document.buffer_views = before.document.buffer_views.clone(),
                "buffers" => {
                    restored.document.buffers = before.document.buffers.clone();
                    restored.buffers = before.buffers.clone();
                }
                other => return Err(format!("no restore rule for member {other:?}")),
            }
        }
        Ok(restored)
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Handlers
    fn params(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, &spec)?;
        let before = parse_gltf_document(&input)?;
        let after = apply_kind(&before, &spec.str("kind"), &params(&spec))?;
        let bytes = serialize_gltf_document(&after);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, &spec)?;
        let before = parse_gltf_document(&input)?;
        let mutated = apply_kind(&before, &spec.str("kind"), &params(&spec))?;
        let restored = match restored_members(&spec) {
            Some(members) => restore(&before, &mutated, &members)?,
            None => inverse_specs(&spec).iter().try_fold(mutated, |current, inverse| apply_kind(&current, &inverse.str("kind"), &params(inverse)))?,
        };
        let bytes = serialize_gltf_document(&restored);
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built
}
//#endregion 🔖️Registration
