use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::schema::mutations::change_exaggeration::ChangeExaggeration;
use crate::schema::mutations::change_imported_features::ChangeImportedFeatures;
use crate::{GisTerrainSnapshot, GIS_3D_TERRAIN_SCHEMA};

#[test]
fn authored_elevation_operation_and_snapshot_round_trip() {
    use protocol::ColdMutation as _;
    use semio_framework_dsl_record::{DslField, FieldValue};
    let fixture = semio_framework_test_fixtures::repo_text!(
        "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏔️replace-elevation-source/🏔️authors/🦠️mutation/🔣️.json"
    );
    let command: GisTerrainMutation = semio_framework_pack_json::from_json_str(
        fixture, semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).expect("authored rectangular mutation");
    let GisTerrainMutation::ReplaceElevationSource(command) = command else { panic!("elevation mutation required") };
    let authored = command.new_elevation_source.unwrap();
    assert_eq!(authored.dimensions(), Some((4, 3)));
    assert_eq!(authored.rows_millimetres[0][0], -3007);
    let operation = |new_elevation_source| GisTerrainMutation::ReplaceElevationSource(
        crate::mutations::ReplaceElevationSource { new_elevation_source }
    );
    semio_os_store_test_support::assert_op_text_binary_equivalence(&operation(Some(authored.clone())));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&operation(None));
    let previous_unit = FieldValue::Record({
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(0, DslField::to_value(&authored.bounds));
        record.fields.insert(1, DslField::to_value(&vec![vec![1250_i16; 33]; 33]));
        record
    });
    assert!(<crate::schema::snapshot::ElevationSource as DslField>::from_value(&previous_unit).is_err());
    let snapshot = GisTerrainSnapshot { elevation_source: Some(authored), ..Default::default() };
    semio_os_store_test_support::assert_dsl_pack_equivalence(&snapshot);
    let replacement = operation(Some(crate::schema::snapshot::ElevationSource {
        missing_samples: None,
        bounds: [7.1, 46.0, 7.2, 46.1],
        rows_millimetres: vec![vec![i32::MIN, -1], vec![0, i32::MAX]],
    }));
    let replaced = protocol::apply_diff_cold(replacement.diff(&snapshot).diff(), &snapshot).unwrap();
    assert_eq!(replaced.elevation_source.as_ref().unwrap().dimensions(), Some((2, 2)));
    let mut only_mask = replaced.elevation_source.clone().unwrap();
    only_mask.missing_samples = Some("04".into());
    let mask_change = operation(Some(only_mask.clone()));
    semio_os_store_test_support::assert_op_text_binary_equivalence(&mask_change);
    let masked = protocol::apply_diff_cold(mask_change.diff(&replaced).diff(), &replaced).unwrap();
    assert_eq!(masked.elevation_source.as_ref(), Some(&only_mask));
    semio_os_store_test_support::assert_dsl_pack_equivalence(&masked);
    let unmask = mask_change.inverse(&replaced).unwrap();
    let unmasked = protocol::apply_diff_cold(unmask[0].diff(&masked).diff(), &masked).unwrap();
    assert_eq!(unmasked.elevation_source, replaced.elevation_source);
    let inverse = replacement.inverse(&snapshot).unwrap();
    assert_eq!(inverse.len(), 1);
    let restored = protocol::apply_diff_cold(inverse[0].diff(&replaced).diff(), &replaced).unwrap();
    assert_eq!(restored.elevation_source, snapshot.elevation_source);
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 });
    semio_os_store_test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_change_exaggeration_op_line_round_trips() {
    semio_os_store_test_support::assert_op_line_round_trip(&GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 3.0 }));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_change_imported_features_op_line_round_trips() {
    semio_os_store_test_support::assert_op_line_round_trip(&GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: Some(crate::schema::imported_map::ImportedMap::from_json(r#"{"positions":[]}"#).unwrap()) }));
}

#[semio_framework_async_macros::async_test]
async fn gis3d_terrain_document_text_round_trips_through_store() {
    let initial = GisTerrainSnapshot { exaggeration: 1.0, imported_map: None, ..Default::default() };
    let envelope = store::os_store::document_materialize::create_document_envelope(GIS_3D_TERRAIN_SCHEMA, "gis3d-demo", initial, None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::ArtifactStore::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(semio_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners::<GisTerrainSnapshot, GisTerrainMutation>().expect("funded bounded document owners")).unwrap_or_else(|(error, _)| panic!("bounded document owners install refused: {error}"));
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 })], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    semio_os_store_test_support::assert_document_text_round_trip(&store).await;
    semio_os_store_test_support::assert_document_pack_round_trip(&store).await;
    use semio_plugin::ArtifactOwnedDisposer;
    let mut disposer = semio_plugin::ArtifactDocumentStoreDisposer::<GisTerrainSnapshot, GisTerrainMutation>::new();
    let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    for _ in 0..100_000 {
        let demand = disposer.retirement_demands(&store, page).expect("the terrain document store quotes its next close turn");
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes.max(page), maximum_depth: demand.depth.max(1) };
        match disposer.close_step(&mut store, grant).expect("terrain document store close step") {
            semio_plugin::PluginLifecycleStep::Progress(progress) => {
                assert!(progress.copied_items <= 1);
                assert!(progress.released_bytes <= grant.maximum_release_bytes);
            }
            semio_plugin::PluginLifecycleStep::AwaitingInput { reason } | semio_plugin::PluginLifecycleStep::Blocked { reason } => panic!("fresh terrain document store close unexpectedly blocked: {reason}"),
            semio_plugin::PluginLifecycleStep::Complete(_) => break,
        }
    }
    assert!(disposer.terminal_is_empty(&store));
    drop(disposer);
    drop(store);
}

/// 🏷️ Archived History bytes are tagged frames, never complete Pack containers, so the writer and the original receiver agree.
#[test]
fn operation_frames_carry_format_and_variant_tag_before_the_record_body() {
    let imported = GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: Some(crate::schema::imported_map::ImportedMap::from_json(r#"{"positions":[]}"#).unwrap()) });
    let replaced = GisTerrainMutation::ReplaceElevationSource(crate::mutations::ReplaceElevationSource { new_elevation_source: None });
    let exaggeration = GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.0 });
    for (operation, tag) in [(&exaggeration, 0_u8), (&imported, 1), (&replaced, 2)] {
        let bytes = encode_op(operation).expect("encode");
        assert_eq!(bytes[..2], [1, tag], "format and variant tag lead the frame");
        assert_eq!(&decode_op(&bytes).expect("decode"), operation);
    }
}
