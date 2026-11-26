//! 🌿️ Native control replay for the registered serializer.
use std::future::Future;
use std::path::PathBuf;
use std::task::{Context, Poll, Waker};
use semio_framework::io_schema::IoPayload;
use semio_framework_value::{NativeDecodeControl, ValueRefusalKind};
use semio_os_kernel::io::io_mechanism::{ArchiveChildren, IoRunControl, Serializer};
use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::ModelIntoGbxml;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

fn read_model(case: &str) -> String {
    let fixture = fixtures();
    let directory = if case == "🏠️house" { "🌿️gbxml" } else { "💡️inferences/🌡️energy-envelope" };
    std::fs::read_to_string(fixture.join(directory).join(case).join("📸️snapshot/🔣️.json")).unwrap()
}

fn fixtures() -> PathBuf {
    PathBuf::from("✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
}

fn export(case: &str, stop: Option<usize>) -> Result<(String, usize), ValueRefusalKind> {
    let snapshot = decode_model_snapshot_json(&read_model(case)).unwrap();
    let mut checkpoints = 0;
    let mut progress = |_| {
        checkpoints += 1;
        Some(checkpoints) != stop
    };
    let mut native = NativeDecodeControl::new(1 << 25, &mut progress);
    let mut control = IoRunControl::decoding(&mut native, Default::default());
    let archive = ArchiveChildren::default();
    let result = match std::pin::pin!(ModelIntoGbxml::serialize(&snapshot, &archive, &mut control))
        .poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("native probe yields only through the IO authority"),
    };
    drop(control);
    drop(native);
    let result = result.map_err(|error| error.cause.kind())?;
    let IoPayload::Text(xml) = result.value else { panic!("expected XML text") };
    Ok((xml, checkpoints))
}

#[test]
fn serializer_cancellation_prevents_publication() {
    for limit in [1, 4, 12, 28] {
        assert_eq!(export("🏠️box", Some(limit)), Err(ValueRefusalKind::Canceled));
    }
}

#[test]
fn serializer_emits_the_committed_gbxml_files() {
    for case in ["🏠️box", "🏘️zoning", "🧱️stack", "🏠️house"] {
        let golden = std::fs::read_to_string(fixtures().join("🌿️gbxml").join(case).join("gbxml.xml")).unwrap();
        let (xml, checkpoints) = export(case, None).unwrap();
        assert_eq!(xml.len(), golden.len(), "{case}");
        assert!(xml == golden, "serializer differs from {case} golden");
        assert!(checkpoints > 28, "{case}");
    }
}

#[test]
#[cfg(feature = "funded-tests")]
fn producer_pages_retire_under_the_original_caller() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, RetainedCloneGrant};
    use semio_plugin::app::ArtifactOutputChunks;
    use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::GbxmlPages;
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
    let text = "Räume & 雪 ".repeat(2000);
    let tree = XmlNode::Element { name: "gbXML".into(), attrs: Vec::new(), children: vec![XmlNode::Text { text }] };
    let expected = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gbXML>{}</gbXML>\n", "Räume &amp; 雪 ".repeat(2000));
    let stock = RetainedCloneGrant { maximum_items: 1 << 19, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 256 };
    let grant = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 17, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 128 };
    for operation in [1, 2] {
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(operation), Generation(1), stock, || Some(1), 1000, 1 << 22);
        let mut producer = GbxmlPages::default();
        let mut original = None;
        assert!(caller.turn(grant, |cx| ArtifactOutputChunks::original_birth_admit(ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, &mut original, cx)).unwrap());
        assert!(caller.turn(grant, |cx| producer.receive(&mut original, cx)).unwrap());
        assert!(original.is_none());
        let denied = RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant };
        assert!(!caller.turn(denied, |cx| producer.advance(&tree, cx)).unwrap());
        let mut sealed = false;
        for turn in 0..4096 {
            if operation == 2 && turn == 20 {
                caller.cancel().cancel_now();
                assert!(caller.turn(grant, |cx| producer.advance(&tree, cx)).is_err());
                break;
            }
            sealed = caller.turn(grant, |cx| producer.advance(&tree, cx)).unwrap();
            if sealed { break; }
        }
        assert_eq!(sealed, operation == 1);
        if sealed {
            let chunks = caller.turn(grant, |cx| producer.take(cx)).unwrap().unwrap();
            assert!(chunks.chunks_remaining() > 1);
            let mut output = Vec::new();
            while let Some(page) = chunks.take_chunk().unwrap() {
                output.extend(page);
            }
            assert_eq!(output, expected.as_bytes());
        }
        for _ in 0..8192 {
            if producer.terminal_is_empty() { break; }
            caller.retirement_turn(grant, |cx| producer.close_step(cx)).unwrap();
        }
        assert!(producer.terminal_is_empty());
        caller.retire(stock);
    }
}

#[test]
#[cfg(feature = "funded-tests")]
fn original_surface_index_matches_the_canonical_site_plan() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, OriginalTurnFailure, RetainedCloneGrant};
    use semio_os_kernel::InferenceWorkFailure;
    use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::{OriginalSurfaces, plan_of};
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::{instance, kinds};
    let model = decode_model_snapshot_json(&read_model("🧱️stack")).unwrap();
    let failed = |error| -> OriginalTurnFailure {
        match error {
            InferenceWorkFailure::Original(error) => error,
            other => panic!("surface inference failed: {other:?}"),
        }
    };
    instance::probe::<{ kinds::ENERGY | kinds::LEVELS }, _>(None, &model, |inferred| {
        let expected = plan_of(&model, inferred).unwrap();
        let stock = RetainedCloneGrant { maximum_items: 1 << 19, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 256 };
        let grant = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 17, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 128 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), stock, || Some(1), 1000, 1 << 22);
        let mut work = OriginalSurfaces::default();
        let mut done = false;
        for _ in 0..8192 {
            done = caller.turn(grant, |cx| work.advance(&model, inferred, cx).map_err(failed)).unwrap();
            if done { break; }
        }
        assert!(done);
        let rows: Vec<_> = (0..work.extent()).filter_map(|ordinal| work.get(ordinal)).collect();
        assert_eq!(rows.len(), expected.surfaces.len());
        for (row, expected) in rows.iter().zip(expected.surfaces) {
            let (space, _) = model.spaces.get_index(row.surface.room).unwrap();
            let canonical = &inferred.energy_envelopes[space].surfaces[row.surface.side];
            assert_eq!(canonical.id, expected.cad[0]);
            assert_eq!((row.area, row.azimuth, row.tilt, row.corner, row.width, row.height),
                        (expected.area, expected.azimuth, expected.tilt, expected.rect.corner, expected.rect.width, expected.rect.height));
        }
        for _ in 0..8192 {
            if work.terminal_is_empty() { break; }
            caller.retirement_turn(grant, |cx| work.close_step(cx).map_err(failed)).unwrap();
        }
        assert!(work.terminal_is_empty());
        caller.retire(stock);
    }).unwrap();
}

#[test]
#[cfg(feature = "funded-tests")]
fn original_zone_index_matches_the_canonical_thermal_groups() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, OriginalTurnFailure, RetainedCloneGrant};
    use semio_os_kernel::InferenceWorkFailure;
    use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::{OriginalZones, plan_of};
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::{instance, kinds};
    let model = decode_model_snapshot_json(&read_model("🏘️zoning")).unwrap();
    let failed = |error| -> OriginalTurnFailure {
        match error {
            InferenceWorkFailure::Original(error) => error,
            other => panic!("zone planning failed: {other:?}"),
        }
    };
    instance::probe::<{ kinds::ENERGY | kinds::LEVELS }, _>(None, &model, |inferred| {
        let expected = plan_of(&model, inferred).unwrap();
        let stock = RetainedCloneGrant { maximum_items: 1 << 19, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 256 };
        let grant = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 17, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 128 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), stock, || Some(1), 1000, 1 << 22);
        let mut work = OriginalZones::default();
        let done = (0..8192).any(|_| caller.turn(grant, |cx| work.advance(&model, inferred, cx).map_err(failed)).unwrap());
        assert!(done);
        assert_eq!(work.len(), expected.zones.len());
        for (index, zone) in expected.zones.iter().enumerate() {
            let original = work.get(&model, inferred, index).unwrap();
            let cad = original.owner_variant.map_or_else(|| original.owner.trim_start_matches(':').to_owned(), |suffix| format!("{}#{suffix}", original.owner));
            assert_eq!(cad, zone.cad);
            assert_eq!((original.heating, original.cooling, original.outdoor_air), (zone.heating, zone.cooling, zone.outdoor_air));
        }
        for (room, (id, _)) in model.spaces.iter().enumerate() {
            let group = work.member(room).map(|ordinal| &expected.zones[ordinal].xml);
            assert_eq!(group, expected.zone_of.get(id));
        }
        for _ in 0..8192 {
            if work.terminal_is_empty() { break; }
            caller.retirement_turn(grant, |cx| work.close_step(cx).map_err(failed)).unwrap();
        }
        assert!(work.terminal_is_empty());
        caller.retire(stock);
    }).unwrap();
}

#[test]
#[cfg(feature = "funded-tests")]
fn normalized_original_names_match_the_reference_collision_order() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, OriginalTurnFailure, RetainedCloneGrant};
    use semio_os_kernel::InferenceWorkFailure;
    use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::{
        GbxmlNameSource, GbxmlText, GbxmlTextPart, Names, OriginalNames,
    };
    struct Raw<'a>(&'a [&'a str]);
    impl GbxmlNameSource for Raw<'_> {
        fn name_source_identity(&self) -> usize { self.0.as_ptr() as usize }
        fn name_count(&self) -> usize { self.0.len() }
        fn with_name<R>(&self, index: usize, read: impl FnOnce(GbxmlText<'_>) -> R) -> Option<R> {
            let parts = [GbxmlTextPart::Text("surface-"), GbxmlTextPart::Name(self.0.get(index)?)];
            Some(read(GbxmlText::Parts(&parts)))
        }
    }
    let long = "Wände".repeat(40);
    let raw = ["Wall/4", "Wall_4-2", "Wall_4", "Wall/4", "Wall/4-2", long.as_str(), long.as_str()];
    let source = Raw(&raw);
    let failed = |error| -> OriginalTurnFailure {
        match error {
            InferenceWorkFailure::Original(error) => error,
            other => panic!("name planning failed: {other:?}"),
        }
    };
    let stock = RetainedCloneGrant { maximum_items: 1 << 19, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 256 };
    let grant = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 17, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 128 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), stock, || Some(1), 1000, 1 << 22);
    let mut planner = OriginalNames::default();
    assert!((0..8192).any(|_| caller.turn(grant, |cx| planner.advance(&source, cx).map_err(failed)).unwrap()));
    let mut reference = Names::default();
    for (index, raw) in raw.iter().enumerate() {
        let expected = reference.id("surface", raw);
        let base = Names::default().id("surface", raw);
        let suffix = planner.suffix(index).unwrap();
        let observed = if suffix == 1 { base } else { format!("{base}-{suffix}") };
        assert_eq!(observed, expected);
    }
    for _ in 0..8192 {
        if planner.terminal_is_empty() { break; }
        caller.retirement_turn(grant, |cx| planner.close_step(cx).map_err(failed)).unwrap();
    }
    assert!(planner.terminal_is_empty());
    caller.retire(stock);
}

#[test]
#[cfg(feature = "funded-tests")]
fn original_controller_emits_canonical_gbxml_pages() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, RetainedCloneGrant};
    use semio_plugin::app::ArtifactOutputChunks;
    use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::OriginalGbxml;
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::{instance, kinds};

    let stock = RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 256 };
    let work_grant = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1 << 17, maximum_capacity_bytes: 1 << 17, maximum_release_bytes: 1 << 17, maximum_depth: 128 };
    let cases = if std::env::var_os("BIM_GBXML_HOUSE").is_some() { &["🏠️house"][..] } else { &["🏠️box", "🏘️zoning", "🧱️stack"][..] };
    for &case in cases {
        let model = decode_model_snapshot_json(&read_model(case)).unwrap();
        let golden = std::fs::read(fixtures().join("🌿️gbxml").join(case).join("gbxml.xml")).unwrap();
        instance::probe::<{ kinds::ENERGY | kinds::LEVELS }, _>(None, &model, |inferred| {
            let mut caller = OriginalJobFixtureCaller::admit(OperationId(1), Generation(1), stock, || Some(1), 1024, 1 << 27);
            let mut work = OriginalGbxml::default();
            let mut original = None;
            assert!(caller.turn(work_grant, |cx| ArtifactOutputChunks::original_birth_admit(ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, &mut original, cx)).unwrap());
            assert!(caller.turn(work_grant, |cx| work.receive(&mut original, cx)).unwrap());
            let mut progress = Vec::new();
            let finished = (0..1_000_000).any(|_| {
                let stage = work.stage();
                if progress.last() != Some(&stage) { progress.push(stage); }
                caller.turn(work_grant, |cx| {
                    for _ in 0..64 {
                        if work.advance(&model, inferred, cx)? { return Ok(true); }
                        if cx.should_yield() || work.stage() != stage { break; }
                    }
                    Ok(false)
                }).unwrap_or_else(|error| panic!("{case} failed in {stage}: {error:?}"))
            });
            assert!(finished, "{case} did not finish: last stage {}", work.stage());
            assert_eq!(progress.len(), 7);
            let output = caller.turn(work_grant, |cx| work.take(cx)).unwrap().unwrap();
            assert!(output.is_sealed());
            assert!(output.chunks_remaining() > 1);
            let mut observed = Vec::new();
            while let Some(chunk) = output.take_chunk().unwrap() { observed.extend(chunk); }
            if observed != golden {
                let offset = observed.iter().zip(&golden).position(|(a,b)| a != b).unwrap_or(observed.len().min(golden.len()));
                assert!(observed == golden, "{case} differs at {offset}\nactual: {}\nexpected: {}",
                    String::from_utf8_lossy(&observed[offset.saturating_sub(50)..(offset + 150).min(observed.len())]),
                    String::from_utf8_lossy(&golden[offset.saturating_sub(50)..(offset + 150).min(golden.len())]));
            }
            for _ in 0..1_000_000 {
                if work.terminal_is_empty() { break; }
                caller.retirement_turn(work_grant, |cx| work.close_step(cx)).unwrap();
            }
            assert!(work.terminal_is_empty());
            caller.retire(stock);
        }).unwrap();
    }
}
