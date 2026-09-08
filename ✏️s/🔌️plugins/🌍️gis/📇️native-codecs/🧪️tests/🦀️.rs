//! 🧪 Literal factory identity and actual typed-codec parity.

use geo::BoundingRect;
use semio_framework_os_kernel as store;
use semio_framework_os_kernel::{ArtifactPack, DslValue, FromValue, Mutation, MutationDiff};
use semio_framework_plugin::{ArtifactInferenceExecutionRequest, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};
use semio_s_plugin_gis::artifacts::gismap::{gis_map_inference_service, gis_map_snapshot_with_derived_children, infer_gis_map_controlled, GisMapSnapshot};
use semio_s_plugin_gis::native_codecs::native_codec_factory_receipts;

fn assert_zero_history<P, M>(parsed: store::ParsedDocumentText<P, M>, document_id: &str, dialect: &semio_framework::ArtifactDialect) {
    let envelope = parsed.envelope.into_owners();
    assert_eq!(envelope.id, document_id);
    assert_eq!(envelope.dialect.as_ref(), Some(dialect));
    assert!(envelope.vcs.edits.is_empty());
    assert!(envelope.vcs.changes.is_empty());
    assert!(envelope.vcs.checkpoints.is_empty());
    assert!(envelope.vcs.alternatives.is_empty());
    let cursor = envelope.cursor.expect("genesis cursor");
    assert!(cursor.applied_edit_ids.is_empty());
    assert!(cursor.redo_edit_ids.is_empty());
    assert!(cursor.checkpoint_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    let document_ids: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️fixtures/🌱️artifact-document-id-v1/🔣️.json")).unwrap();
    let document_id = document_ids["cases"].as_array().unwrap().iter().find(|row| row["accepted"] == true).unwrap()["documentId"].as_str().unwrap();
    let snapshot = GisMapSnapshot::default();
    let pack = snapshot.encode_pack();
    let (envelope, _) = store::semio_format::unwrap_binary(&pack).unwrap();
    assert_eq!(envelope.envelope_id(), <GisMapSnapshot as store::ArtifactDsl>::envelope_id());
    assert_ne!(envelope.envelope_id(), <GisMapSnapshot as store::ArtifactDsl>::EXTENSION);
    assert_eq!(GisMapSnapshot::decode_pack(&pack).unwrap(), snapshot);
    let receipts = native_codec_factory_receipts().expect("complete inert GIS closure");
    assert_eq!(receipts.len(), fixture["receipts"].as_array().unwrap().len());
    for (receipt, expected) in receipts.into_iter().zip(fixture["receipts"].as_array().unwrap()) {
        let identity = receipt.identity();
        assert_eq!(identity.plugin_id, fixture["pluginId"]);
        assert_eq!(identity.package_id, fixture["packageId"]);
        assert_eq!(identity.package_version, fixture["packageVersion"]);
        assert_eq!(identity.factory_id, expected["factoryId"]);
        assert_eq!(identity.artifact_kind, expected["kind"]);
        assert_eq!(identity.schema, expected["schema"]);
        assert_eq!(identity.extension, expected["extension"]);
        assert_eq!(identity.capability, expected["capability"]);
        let hash = identity.protocol_sha256.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hash, expected["protocolSha256"]);
        let (codec, genesis) = receipt.into_codec_and_genesis().expect("actual typed factory and genesis match immutable receipt");
        assert_eq!(codec.schema, identity.schema);
        assert_eq!(codec.extension, identity.extension);
        assert_eq!(codec.pack_schema_hash, identity.pack_schema_hash);
        assert_ne!(codec.pack_schema_hash, [0; 32]);
        let declared = match identity.schema {
            "gis.map" => store::ArtifactCodec::of::<GisMapSnapshot, semio_s_plugin_gis::artifacts::gismap::GisMapMutation>(identity.schema),
            "gis.terrain" => store::ArtifactCodec::of::<semio_s_plugin_gis::artifacts::gisterrain::GisTerrainSnapshot, semio_s_plugin_gis::artifacts::gisterrain::GisTerrainMutation>(identity.schema),
            _ => panic!("unhandled GIS schema"),
        };
        assert_eq!(codec.schema, declared.schema);
        assert_eq!(codec.extension, declared.extension);
        assert_eq!(codec.pack_schema_hash, declared.pack_schema_hash);
        let record = match identity.schema {
            "gis.map" => <GisMapSnapshot as ArtifactPack>::record_spec().expect("GIS map structural pack schema"),
            "gis.terrain" => <semio_s_plugin_gis::artifacts::gisterrain::GisTerrainSnapshot as ArtifactPack>::record_spec().expect("GIS terrain structural pack schema"),
            _ => unreachable!(),
        };
        assert_eq!(codec.pack_schema_hash, store::os_pack::schema_hash(&record));
        let observed_record = serde_json::json!({
            "keyword": record.keyword.as_deref(),
            "fields": record.fields.iter().map(|field| serde_json::json!({ "id": field.id, "key": field.key, "optional": field.optional })).collect::<Vec<_>>(),
        });
        assert_eq!(observed_record, expected["packRecord"]);
        assert!(std::ptr::fn_addr_eq(codec.compile_dsl, declared.compile_dsl));
        assert!(std::ptr::fn_addr_eq(codec.print_mirror, declared.print_mirror));
        assert!(std::ptr::fn_addr_eq(codec.edit_text_from_envelope, declared.edit_text_from_envelope));
        assert!(std::ptr::fn_addr_eq(codec.apply_ops_binary, declared.apply_ops_binary));
        let dialect: semio_framework::ArtifactDialect = match identity.schema {
            "gis.map" => semio_s_plugin_gis::artifacts::gismap::GISMAP_DIALECT.into(),
            "gis.terrain" => semio_s_plugin_gis::artifacts::gisterrain::GISTERRAIN_DIALECT.into(),
            _ => panic!("unhandled GIS genesis dialect"),
        };
        let files = genesis(document_id, &dialect).await.expect("package-owned GIS genesis");
        match identity.schema {
            "gis.map" => assert_zero_history(store::parse_document_pack::<GisMapSnapshot, semio_s_plugin_gis::artifacts::gismap::GisMapMutation>(&files.pack, &files.spr).await.unwrap(), document_id, &dialect),
            "gis.terrain" => assert_zero_history(
                store::parse_document_pack::<semio_s_plugin_gis::artifacts::gisterrain::GisTerrainSnapshot, semio_s_plugin_gis::artifacts::gisterrain::GisTerrainMutation>(&files.pack, &files.spr).await.unwrap(),
                document_id,
                &dialect,
            ),
            _ => unreachable!(),
        }
        let hostile = semio_framework::ArtifactDialect { artifact_kind: dialect.artifact_kind.clone(), standard: dialect.standard.clone(), subset: "strict".into() };
        assert!(genesis(document_id, &hostile).await.is_err(), "a package genesis factory must reject a substituted dialect");
        for row in document_ids["cases"].as_array().unwrap().iter().filter(|row| row["accepted"] == false) {
            let hostile_id = row["documentId"].as_str().unwrap();
            assert!(genesis(hostile_id, &dialect).await.is_err(), "GIS genesis admitted hostile document id {}", row["id"]);
        }
    }
}

#[test]
fn gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/💡️inference-control/🔣️.json")).unwrap();
    let features = &fixture["snapshot"];
    let snapshot = gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: FromValue::from_value(DslValue::from(&features["positions"])).unwrap(),
        routes: FromValue::from_value(DslValue::from(&features["routes"])).unwrap(),
        regions: FromValue::from_value(DslValue::from(&features["regions"])).unwrap(),
        ..Default::default()
    });
    let pack = snapshot.encode_pack();
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 65_536, work_units: 64, recursion_depth: 16 };
    let request =
        ArtifactInferenceExecutionRequest { policy: b"gis-map-v1", budgets: &budgets, cancellation_id: "neutral-control", previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold, canonical_payload: &pack, dependencies: &[] };
    let mut observed = Vec::new();
    let output = infer_gis_map_controlled(&request, &mut |work| {
        observed.push(work);
        Ok(())
    })
    .unwrap();
    assert_eq!(serde_json::to_value(&observed).unwrap(), fixture["checkpoints"]);
    assert_eq!(output.canonical_payload, gis_map_inference_service().infer(&request).unwrap().canonical_payload);
    let value = semio_framework_os_kernel::pack_rt::decode_wire_value(&output.canonical_payload).unwrap();
    let inferred = <semio_s_plugin_gis::artifacts::gismap::standards::v1::subsets::any::schema::inferences::GisMapInference as FromValue>::from_value(value).unwrap();
    assert_eq!([inferred.position_count, inferred.route_count, inferred.region_count], ["positionCount", "routeCount", "regionCount"].map(|key| fixture["expected"][key].as_u64().unwrap() as usize));
    let proposal = inferred.bounds_proposal(&snapshot, fixture["proposalJobId"].as_str().unwrap()).unwrap();
    let expected: semio_s_plugin_gis::artifacts::gismap::mutations::GisMapMutation = FromValue::from_value(DslValue::from(&fixture["proposal"])).unwrap();
    assert_eq!(proposal, expected);
    let updated = proposal.diff(&snapshot).diff().apply(&snapshot).unwrap();
    assert_eq!(updated.regions.len(), snapshot.regions.len() + 1);
    let restored = proposal.inverse(&snapshot).iter().rev().try_fold(updated.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state)).unwrap();
    assert_eq!(restored, snapshot);
    assert!(inferred.bounds_proposal(&updated, fixture["proposalJobId"].as_str().unwrap()).is_err());
    assert!(inferred.bounds_proposal(&snapshot, "untrusted-job").is_err());
    for rejection in fixture["proposalRejections"].as_array().unwrap() {
        let mut candidate = inferred.clone();
        let mut base = snapshot.clone();
        let mut job_id = fixture["proposalJobId"].as_str().unwrap();
        match rejection["case"].as_str().unwrap() {
            "wrong-job" => job_id = "not-a-job",
            "duplicate-id" => {
                base.regions.push(semio_s_plugin_gis::artifacts::gismap::MapFeature { id: format!("inference-{job_id}"), data: DslValue::Null });
                candidate.region_count = base.regions.len();
            }
            "stale-count" => candidate.position_count += 1,
            "no-bounds" => candidate.bounds = None,
            "non-finite" => candidate.bounds.as_mut().unwrap().lon_min = f64::NAN,
            "out-of-range" => candidate.bounds.as_mut().unwrap().lon_min = -181.0,
            "reversed" => candidate.bounds.as_mut().unwrap().lat_min = 49.0,
            _ => panic!("unhandled proposal rejection"),
        }
        assert_eq!(format!("{:?}", candidate.bounds_proposal(&base, job_id).unwrap_err()), rejection["error"].as_str().unwrap(), "{}", rejection["case"]);
    }
    let bounds = inferred.bounds.unwrap();
    let points = std::iter::once((features["positions"][0]["data"]["lon"].as_f64().unwrap(), features["positions"][0]["data"]["lat"].as_f64().unwrap()))
        .chain(features["routes"][0]["data"]["points"].as_array().unwrap().iter().map(|point| (point[0].as_f64().unwrap(), point[1].as_f64().unwrap())))
        .collect::<Vec<_>>();
    let independent = geo::MultiPoint::<f64>::from(points).bounding_rect().unwrap();
    assert_eq!([bounds.lon_min, bounds.lon_max, bounds.lat_min, bounds.lat_max], [independent.min().x, independent.max().x, independent.min().y, independent.max().y]);
    assert_eq!([bounds.lon_min, bounds.lon_max, bounds.lat_min, bounds.lat_max], ["lonMin", "lonMax", "latMin", "latMax"].map(|key| fixture["expected"]["bounds"][key].as_f64().unwrap()));
    for interruption in fixture["interruptions"].as_array().unwrap() {
        let mut calls = 0;
        let error = match interruption["error"].as_str().unwrap() {
            "cancelled" => "cancelled",
            "expired" => "expired",
            _ => panic!("unhandled interruption error"),
        };
        let output = infer_gis_map_controlled(&request, &mut |work| {
            calls += 1;
            if work == interruption["at"].as_u64().unwrap() {
                return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new(error, "request stopped"));
            }
            Ok(())
        });
        assert_eq!(output.err().unwrap().code, error);
        assert_eq!(calls, interruption["calls"].as_u64().unwrap(), "no work occurs after the first caller interruption");
    }
    println!("[DEBUG] GIS controlled inference: literal proposal=1 inverse=1 geo=1 interruption=3 rejection=7; no hub approval authority");
}
