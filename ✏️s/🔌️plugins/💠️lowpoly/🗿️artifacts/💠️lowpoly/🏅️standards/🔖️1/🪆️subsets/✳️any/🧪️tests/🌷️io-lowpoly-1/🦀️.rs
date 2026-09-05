//! 🦀️ Lowpoly IO round-trip case — Rust subject adapter. Ticket
//! `26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS`.
//!
//! Recorded no-oracle decision `lowpoly-io-native-round-trip`
//! (`../../🔮️oracle/🔣️.json`): exports the committed fixture
//! through this subset's own `🚪️io/📤️export/🧵️serializers/…/serialize_bytes` for each non-PNG
//! `stdio.*` format `import_stdio_kinds()`/`export_stdio_kinds()` declare, imports the produced
//! bytes back through the matching `deserialize_bytes`, and requires the round trip to return the
//! committed document unchanged. Gated behind the generated host's `sut` feature — this case links
//! this subset's own plugin crate directly (it must, to reach `serialize_bytes`/`deserialize_bytes`),
//! unlike `🧭️mutate-lowpoly-1`, which replays committed vectors without linking it.
//!
//! Four of the eight declared formats — `dwg`, `gltf`, `las`, `stl` — are committed, HONEST stubs:
//! `LowpolyObject.mesh` is a content-addressed handle
//! (`store::ArtifactChild<SemioMeshSnapshot>`), never embedded geometry, so a synchronous
//! `&LowpolySnapshot -> …` serializer genuinely cannot reach real mesh vertices for those four
//! formats. Their own leaf doc comments (`../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/…`) name the
//! exact reason, and the production crate's own `unimplemented_geometry_formats_error_honestly_instead_of_lying`
//! unit test (`../../🚪️io/🦀️.rs`) already asserts `serialize_bytes` returns `Err` for all four —
//! this case asserts that SAME explicit error rather than a round trip for those four rows; the other
//! four (`json`, `obj`, `ply`, `txt`) still exercise the real round trip.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Vocabulary
/// 🏷️ Every declared non-PNG format. PNG is independently decoded by the sibling
/// `🟩️io-lowpoly-png-1` Pillow case.
const FORMATS: &[&str] = &["dwg", "gltf", "json", "las", "obj", "ply", "stl", "txt"];

/// 🚫️ The four formats whose exporter is a committed, HONEST stub: `LowpolyObject.mesh` is a
/// content-addressed handle, never embedded geometry, so `serialize_bytes` unconditionally returns
/// `Err` for these — see the module doc comment and each leaf's own doc comment.
const STUB_FORMATS: &[&str] = &["dwg", "gltf", "las", "stl"];
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

    /// 🚫️ For a `STUB_FORMATS` member: `serialize_bytes` must return the committed HONEST error
    /// (content-addressed mesh handle unreachable at this layer), never a silent pack-envelope lie
    /// and never a successful round trip — that would mean the architecture gap this format's own
    /// leaf doc comment names has closed, which is real news this scenario must be told about by
    /// hand, not something a weakened assertion should paper over.
    fn serialize_must_fail_honestly(format: &str, snapshot: &LowpolySnapshot) -> Result<Outcome, String> {
        let result = match format {
            "dwg" => export::dwg::v_ac1018::any::serialize_bytes(snapshot),
            "gltf" => export::gltf::v2_0::any::serialize_bytes(snapshot),
            "las" => export::las::v1_0::any::serialize_bytes(snapshot),
            "stl" => export::stl::v_ascii::any::serialize_bytes(snapshot),
            other => return Err(format!("no stub serializer registered for format {other:?}")),
        };
        match result {
            Ok(_) => Err(format!(
                "serialize_bytes({format}) succeeded, but {format} is a committed honest-stub exporter expected to error because LowpolyObject.mesh is a content-addressed handle unreachable at this layer -- if this now succeeds, the architecture gap has closed and this scenario must be promoted to a real round trip"
            )),
            Err(error) => {
                let message = error.to_string();
                if !message.contains("unavailable at the LowpolySnapshot layer") || !message.contains("not implemented") {
                    return Err(format!("serialize_bytes({format}) failed as expected, but with an unexpected message {message:?} -- expected the honest content-addressed-handle stub wording"));
                }
                let projection = semio_repo_test_host::parse_json(&format!("{{\"format\":\"{format}\",\"error\":{message:?}}}"))?;
                Ok(Outcome::with_raw(message.into_bytes(), projection))
            }
        }
    }

    pub fn roundtrip(format: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let original = document(ctx)?;
            if super::STUB_FORMATS.contains(&format) {
                return serialize_must_fail_honestly(format, &original);
            }
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
/// `Examples` table), mirroring `🧭️mutate-lowpoly-1`'s registration loop.
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
