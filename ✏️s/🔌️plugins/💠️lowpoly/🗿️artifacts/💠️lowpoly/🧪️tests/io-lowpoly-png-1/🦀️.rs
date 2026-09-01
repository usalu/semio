//! 🦀️ Lowpoly PNG Pillow-oracle case — Rust subject adapter. Ticket
//! `26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS`.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_lowpoly::artifacts::lowpoly::io::export::serializers::artifacts::png::v1_2::any as export;
    use semio_s_plugin_lowpoly::artifacts::lowpoly::io::import::deserializers::artifacts::png::v1_2::any as import;
    use semio_s_plugin_lowpoly::artifacts::lowpoly::LowpolySnapshot;

    /// 🧫️ Reads the fixture named by the scenario rather than duplicating its document in Rust.
    fn document(ctx: &Context) -> Result<LowpolySnapshot, String> {
        let spec = ctx.doc_json()?;
        let uri = spec.str("document");
        let bytes = ctx.fixture_bytes(&uri)?;
        serde_json::from_slice::<LowpolySnapshot>(&bytes).map_err(|error| format!("fixture {uri} is not a valid LowpolySnapshot: {error}"))
    }

    /// 📷️ Exports the real PNG byte stream, keeps `deserialize_bytes()`'s original round-trip law,
    /// and reports the pre-encode raster facts for Pillow to independently verify from those bytes.
    pub fn roundtrip_png(ctx: &Context) -> Result<Outcome, String> {
        let original = document(ctx)?;
        let png = export::serialize(&original).map_err(|error| format!("serialize(png) failed: {error}"))?;
        let bytes = export::serialize_bytes(&original).map_err(|error| format!("serialize_bytes(png) failed: {error}"))?;
        let imported = import::deserialize_bytes(&bytes).map_err(|error| format!("deserialize_bytes(png) failed: {error}"))?;
        if imported != original {
            return Err(format!("round trip through png did not return the committed document unchanged:\n  before = {original:?}\n  after  = {imported:?}"));
        }
        let pixel_count = (png.width as usize).checked_mul(png.height as usize).and_then(|count| count.checked_mul(4)).ok_or_else(|| "png dimensions overflow RGBA byte length".to_string())?;
        if png.pixels.len() != pixel_count {
            return Err(format!("serialize(png) produced {} RGBA bytes for {}×{} pixels", png.pixels.len(), png.width, png.height));
        }
        let text = png.text_chunks.first().ok_or_else(|| "serialize(png) produced no tEXt chunk".to_string())?;
        let projection = serde_json::json!({
            "width": png.width,
            "height": png.height,
            "pixelFormat": "RGBA",
            "pixels": png.pixels,
            "textChunkKeyword": text.keyword,
        });
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ The Python oracle consumes this subject's raw PNG output under the feature's
/// `@oracle-input-subject-raw` contract.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("roundtrip-png", subject::roundtrip_png);
    }
    built
}
//#endregion 🔖️Registration
