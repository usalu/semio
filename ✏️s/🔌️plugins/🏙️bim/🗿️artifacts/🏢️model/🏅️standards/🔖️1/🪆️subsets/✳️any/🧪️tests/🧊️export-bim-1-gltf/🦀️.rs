//! 🧊️ BIM export case `export-bim-1-gltf`, Rust adapter (subject role only). The three.js measurement lives in `🟦️.ts` beside this file: its GLTFLoader parses the committed GLB and
//! counts and measures the scene graph. This adapter answers the same scenario from the subject: it builds the typed glTF document of the committed house and reports the same table
//! (node, mesh, primitive, triangle and material counts, element nodes per kind and storey, world bounds of the 32-bit vertices through the node chain).
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

/// 📸️ The committed house model.
const SNAPSHOT: &str = "shared://🏗️ifc/🏠️house/📸️snapshot/🔣️.json";

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::SNAPSHOT;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::gltf::{model_to_gltf, projection::project};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

    /// 🧊️ `{nodes, meshes, primitives, triangles, materials, kinds, storeys, bounds}` of the glTF export of the committed house.
    pub fn export_gltf_house(ctx: &Context) -> Result<Outcome, String> {
        let bytes = ctx.input_bytes(SNAPSHOT)?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let table = project(&model_to_gltf(&snapshot).0).to_json();
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; the id is the feature's `@id-*` tag.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("export-gltf-house", subject::export_gltf_house);
    }
    built
}
//#endregion 🔖️Registration
