//! 🦀️ Lowpoly IO round-trip case — Rust subject adapter. Ticket
//! `26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS`.
//!
//! Recorded no-oracle decision `lowpoly-io-native-round-trip`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`): exports the committed fixture
//! through this subset's own `🚪️io/📤️export/🧵️serializers/…/serialize_bytes` for each non-PNG
//! `stdio.*` format `import_stdio_kinds()`/`export_stdio_kinds()` declare, imports the produced
//! bytes back through the matching `deserialize_bytes`, and requires the round trip to return the
//! committed document unchanged. Gated behind the generated host's `sut` feature — this case links
//! this subset's own plugin crate directly (it must, to reach `serialize_bytes`/`deserialize_bytes`),
//! unlike `mutate-lowpoly-1`, which replays committed vectors without linking it.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Vocabulary
/// 🏷️ Every declared non-PNG format. PNG is independently decoded by the sibling
/// `io-lowpoly-png-1` Pillow case.
const FORMATS: &[&str] = &["dwg", "gltf", "json", "las", "obj", "ply", "stl", "txt"];
//#endregion 🔖️Vocabulary

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_lowpoly::artifacts::lowpoly::io::export::serializers::artifacts as export;
    use semio_s_plugin_lowpoly::artifacts::lowpoly::io::import::deserializers::artifacts as import;
    use semio_s_plugin_lowpoly::artifacts::lowpoly::LowpolySnapshot;

    /// 🧫️ The committed fixture the scenario's own doc string names, parsed as a real
    /// `LowpolySnapshot` (not a hand-transcribed Rust literal) so the fixture stays the one place
    /// this document's shape is stated.
    fn document(ctx: &Context) -> Result<LowpolySnapshot, String> {
        let spec = ctx.doc_json()?;
        let uri = spec.str("document");
        let bytes = ctx.fixture_bytes(&uri)?;
        serde_json::from_slice::<LowpolySnapshot>(&bytes).map_err(|error| format!("fixture {uri} is not a valid LowpolySnapshot: {error}"))
    }

    /// 🔁️ Export through the named format's `serialize_bytes`, then import the produced bytes back
    /// through the matching `deserialize_bytes` — both real production functions, never a second
    /// bespoke grammar.
    fn export_then_import(format: &str, snapshot: &LowpolySnapshot) -> Result<(Vec<u8>, LowpolySnapshot), String> {
        let bytes = match format {
            "dwg" => export::dwg::v_ac1018::any::serialize_bytes(snapshot),
            "gltf" => export::gltf::v2_0::any::serialize_bytes(snapshot),
            "json" => export::json::v_rfc8259::any::serialize_bytes(snapshot),
            "las" => export::las::v1_0::any::serialize_bytes(snapshot),
            "obj" => export::obj::v3_0::any::serialize_bytes(snapshot),
            "ply" => export::ply::v1_0::any::serialize_bytes(snapshot),
            "stl" => export::stl::v_ascii::any::serialize_bytes(snapshot),
            "txt" => export::txt::v_utf_8::any::serialize_bytes(snapshot),
            other => return Err(format!("no serializer registered for format {other:?}")),
        }
        .map_err(|error| format!("serialize_bytes({format}) failed: {error}"))?;
        let imported = match format {
            "dwg" => import::dwg::v_ac1018::any::deserialize_bytes(&bytes),
            "gltf" => import::gltf::v2_0::any::deserialize_bytes(&bytes),
            "json" => import::json::v_rfc8259::any::deserialize_bytes(&bytes),
            "las" => import::las::v1_0::any::deserialize_bytes(&bytes),
            "obj" => import::obj::v3_0::any::deserialize_bytes(&bytes),
            "ply" => import::ply::v1_0::any::deserialize_bytes(&bytes),
            "stl" => import::stl::v_ascii::any::deserialize_bytes(&bytes),
            "txt" => import::txt::v_utf_8::any::deserialize_bytes(&bytes),
            other => return Err(format!("no deserializer registered for format {other:?}")),
        }
        .map_err(|error| format!("deserialize_bytes({format}) failed: {error}"))?;
        Ok((bytes, imported))
    }

    pub fn roundtrip(format: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let original = document(ctx)?;
            let (bytes, imported) = export_then_import(format, &original)?;
            if imported != original {
                return Err(format!("round trip through {format} did not return the committed document unchanged:\n  before = {original:?}\n  after  = {imported:?}"));
            }
            let projection = semio_repo_test_host::parse_json(&serde_json::to_string(&imported).map_err(|error| error.to_string())?)?;
            Ok(Outcome::with_raw(bytes, projection))
        }
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration is by full expanded scenario id (`roundtrip-<format>`, from the feature's own
/// `Examples` table), mirroring `mutate-lowpoly-1`'s registration loop.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for format in FORMATS {
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("roundtrip-{format}"), subject::roundtrip(format));
        }
    }
    built
}
//#endregion 🔖️Registration
