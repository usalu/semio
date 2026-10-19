//! 🧬️ Geometry media and owner tests of the CAD editor unit tests: the imported typed owners, the intrinsic and owned geometry media, the pricing and the parent operation pack. See [`super`].

use super::*;
use semio_framework_test_fixtures::fixture_path;

#[test]
fn cad_imported_geometry_batches_preserve_exact_typed_owners_and_controlled_close() {
    use semio_framework_os_kernel::MemberStoreOwnedBatch;
    use semio_framework_value::{retained_clone::RetainedCloneGrant, retirement::RetireOwned};
    use {semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation, semio_s_artifact_stdio_semio_brep::schema::mutations::create_vertex::CreateVertex, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation, semio_s_artifact_stdio_semio_model::schema::mutations::insert_element::InsertElement};
    fn close<M: RetireOwned + semio_framework_value::ToValue>(operations: Vec<M>) {
        let pointer = operations.as_ptr();
        let oracle = serde_json::to_value(semio_framework_value::ToValue::to_value(&operations)).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
        let (mut batch, birth) = MemberStoreOwnedBatch::try_new(operations, grant).unwrap_or_else(|(error, _)| panic!("exact imported typed owner authority: {error:?}"));
        assert!(birth.fits(grant));
        assert_eq!(batch.mutations::<M>().unwrap().as_ptr(), pointer);
        assert_eq!(serde_json::to_value(semio_framework_value::ToValue::to_value(batch.mutations::<M>().unwrap())).unwrap(), oracle);
        assert_eq!(batch.close_granted(RetainedCloneGrant::default()).unwrap().progress(), Default::default());
        for _ in 0..100000 {
            if batch.terminal_is_empty() { break; }
            assert!(batch.close_granted(grant).unwrap().progress().fits(grant));
        }
        assert!(batch.terminal_is_empty());
    }
    let text = std::fs::read_to_string(fixture_path!("../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧪️tests/🧫️fixtures/📦️set-object-applied/⬅️before.obj")).expect("../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧪️tests/🧫️fixtures/📦️set-object-applied/⬅️before.obj").leak();
    let imported = semio_s_artifact_cad_cad_io::import_obj_object(text).unwrap();
    let corpus: serde_json::Value = serde_json::from_str(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json").leak()).unwrap();
    let mesh = semio_s_artifact_cad_cad_model::geometry::mesh_from_owned_brep(&imported.geometry).unwrap();
    assert_eq!(mesh.indices.len() / 3, corpus["intrinsicGeometry"]["triangleCount"].as_u64().unwrap() as usize);
    assert_eq!(imported.geometry.vertices.len(), corpus["intrinsicGeometry"]["analyticVertexCount"].as_u64().unwrap() as usize);
    let first = &imported.geometry.vertices[0];
    close(vec![SemioBrepMutation::CreateVertex(CreateVertex { id: first.id.clone(), point: first.point, tol: first.tol, at: None })]);
    close(vec![SemioModelMutation::InsertElement(InsertElement { element: imported.element, at: None })]);
}

#[test]
fn cad_imported_geometry_model_operation_wire_preserves_original_forward_inverse() {
    use protocol::OpBinary;
    use semio_framework_os_kernel::{ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use {semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelElement, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation, semio_s_artifact_stdio_semio_model::schema::mutations::insert_element::InsertElement, semio_s_artifact_stdio_semio_model::schema::mutations::remove_element::RemoveElement, semio_s_artifact_stdio_semio_model::io::binary::mutations::prepared_operation_wire_source};
    let fixture: serde_json::Value = serde_json::from_str(std::fs::read_to_string(fixture_path!("../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/💾️binary/🧬️mutations/🫳️borrowed/🧪️tests/🧫️fixtures/🔣️.json")).expect("../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/💾️binary/🧬️mutations/🫳️borrowed/🧪️tests/🧫️fixtures/🔣️.json").leak()).unwrap();
    for row in fixture["elements"].as_array().unwrap() {
        let element: SemioModelElement = semio_framework_pack_json::from_json_str(&serde_json::to_string(row).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let inverse = SemioModelMutation::RemoveElement(RemoveElement { id: element.id.clone() });
        for operation in [SemioModelMutation::InsertElement(InsertElement { element }), inverse] {
            let expected = operation.encode_op().unwrap();
            assert!(prepared_operation_wire_source(&operation).is_some(), "original imported Model operation requires exact borrowed receipt authority");
            for maximum in [1, 3, 7, 64, 256] {
                let mut cursor = ArtifactPreparedOperationCursor::default();
                let mut actual = Vec::with_capacity(expected.len());
                for _ in 0..expected.len() * 4 + 128 {
                    let mut output = [0; 256];
                    let (denied, heap) = semio_framework_io_sqlite_snapshot_testing_backing::measure(|| cursor.advance(prepared_operation_wire_source(&operation).unwrap(), &mut output, RetainedCloneGrant::default()).unwrap());
                    assert_eq!(denied, ArtifactPreparedOperationProgress::default());
                    assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
                    let (progress, heap) = semio_framework_io_sqlite_snapshot_testing_backing::measure(|| cursor.advance(prepared_operation_wire_source(&operation).unwrap(), &mut output, RetainedCloneGrant::one_payload_turn(maximum, 64)).unwrap());
                    assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
                    assert!(progress.written_bytes <= maximum.min(64));
                    actual.extend_from_slice(&output[..progress.written_bytes]);
                    if progress.complete { break; }
                }
                assert_eq!(actual, expected);
                assert_eq!(SemioModelMutation::decode_op(&actual).unwrap(), operation);
                let (closed, heap) = semio_framework_io_sqlite_snapshot_testing_backing::measure(|| cursor.close(RetainedCloneGrant { maximum_items: 1, ..Default::default() }).unwrap());
                assert!(closed.complete);
                assert_eq!((heap.bytes, heap.released_bytes), (0, 0));
            }
        }
    }
}

/// 🎞️ Actual CAD media import publishes its exact composed child and retains the file's geometry.
#[semio_framework_async_macros::async_test]
async fn cad_intrinsic_geometry_media_preserves_actual_file_owner() {
    use semio_framework_plugin::app::ArtifactEditor;
    use semio_framework_value::DslValue;
    let corpus: Value = semio_framework_pack_json::from_json_str(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json").leak(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(corpus["intrinsicGeometry"]["childPublication"], "insertElement");
    assert_eq!(corpus["intrinsicGeometry"]["geometryOwner"], "persistedArtifact");
    let text = std::fs::read_to_string(fixture_path!("../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧪️tests/🧫️fixtures/📦️set-object-applied/⬅️before.obj")).expect("../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧪️tests/🧫️fixtures/📦️set-object-applied/⬅️before.obj").leak();
    let mut mounted = new_app().await;
    let scene = mounted.snapshot().expect("mounted snapshot");
    let children = mounted.test_child_content_view();
    let history = empty_history();
    let media_type = MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh };
    let mut meshes = Vec::new();
    for (index, payload) in [
        MediaPayload::Structured { schema: "obj.3.0.geometry".into(), json: text.into() },
        MediaPayload::Intrinsic { schema: "obj.3.0.geometry".into(), value: DslValue::String(text.into()) },
        MediaPayload::Intrinsic { schema: "obj.3.0.geometry".into(), value: DslValue::Bytes(text.as_bytes().to_vec()) },
    ].into_iter().enumerate() {
        let operation = semio_framework_plugin::AppOperationContext { app_instance_id: TEST_INSTANCE, parent_document_id: scene.id.clone(), operation_id: index as u64 + 1, generation: 1, canonical_base_revision: [0; 32], authoring_seed: format!("cad-media-{index}") };
        let view = ArtifactView::with_children(&scene, &history, children.clone()).bound_to_operation(operation);
        let media = Media { media_type: media_type.clone(), payload };
        assert!(<CadPlayApp as ArtifactEditor>::import_media("geometry:in", &media, &ArtifactView::new(&scene, &history)).is_err(), "an uncomposed view cannot publish a geometry child");
        let mut cancelled = <CadPlayApp as ArtifactEditor>::import_media("geometry:in", &media, &view).expect("cancellable owned geometry import");
        let quoted = |demand: semio_framework_value::RetirementDemand, items: usize| semio_framework_value::RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes.max(65_536), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let preparing = quoted(cancelled.child_preparation_demands(65_536).expect("child preparation quote"), 0);
        assert!(matches!(cancelled.prepare_child_one(preparing).unwrap(), semio_framework_plugin::app::ChildEmitPreparationStep::Pending(progress) if progress == Default::default()));
        let closing = quoted(cancelled.child_close_demands(65_536).expect("child close quote").expect("a cancelled import still owns its geometry child"), 0);
        assert_eq!(cancelled.close_child_one(closing).unwrap(), Some(semio_framework_plugin::PluginLifecycleStep::Progress(Default::default())));
        let mut retired = false;
        for _ in 0..4096 {
            let Some(demand) = cancelled.child_close_demands(65_536).expect("child close quote") else { retired = true; break; };
            let grant = quoted(demand, 1);
            match cancelled.close_child_one(grant).unwrap() {
                None => { retired = true; break; },
                Some(semio_framework_plugin::PluginLifecycleStep::Progress(progress)) => { assert!(progress.copied_items <= 1 && progress.released_bytes <= grant.maximum_release_bytes); },
                Some(step) => panic!("cancelled geometry child must reach its exact empty witness: {step:?}"),
            }
        }
        assert!(retired && cancelled.child_preparations.is_empty() && cancelled.child_emits.is_empty());
        assert_eq!(mounted.snapshot().unwrap(), scene, "cancellation publishes no parent or child prefix");
        let mut emit = <CadPlayApp as ArtifactEditor>::import_media("geometry:in", &media, &view).expect("actual valid owned geometry file");
        assert_eq!(emit.artifact_mutations.len(), 1);
        let CadMutation::CreateBrep(created) = &emit.artifact_mutations[0] else { panic!("exact parent topology creation") };
        let created = created.clone();
        assert_eq!(created.target.dialect.subset, corpus["intrinsicGeometry"]["geometrySubset"].as_str().unwrap());
        assert!(emit.transaction.is_some());
        assert!(emit.config_mutations.is_empty());
        assert!(emit.effects.is_empty());
        let mut ready = false;
        for _ in 0..4096 {
            let demand = emit.child_preparation_demands(65_536).expect("child preparation quote");
            let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(65_536), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            match emit.prepare_child_one(grant).expect("bounded media child preparation") {
                semio_framework_plugin::app::ChildEmitPreparationStep::Ready(_) => { ready = true; break; },
                semio_framework_plugin::app::ChildEmitPreparationStep::Pending(progress) => { assert!(progress.fits(grant)); },
                semio_framework_plugin::app::ChildEmitPreparationStep::Refused(fault, _) => panic!("media child preparation: {}", fault.message),
            }
        }
        assert!(ready);
        assert_eq!(emit.child_emits.len(), corpus["intrinsicGeometry"]["atomicChildren"].as_u64().unwrap() as usize);
        let geometry_child = &emit.child_emits[0];
        assert_eq!(geometry_child.slot, corpus["intrinsicGeometry"]["geometrySlot"].as_str().unwrap());
        assert_eq!(geometry_child.child_id, created.child_id);
        assert!(geometry_child.ops.is_empty(), "the imported topology is the child's genesis, not an operation");
        let genesis = geometry_child.genesis.as_ref().expect("actual retained topology genesis");
        let geometry = <semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&genesis.initial_pack).expect("genesis carries the imported topology");
        let child = &emit.child_emits[1];
        assert_eq!(child.slot, corpus["intrinsicGeometry"]["childSlot"].as_str().unwrap());
        assert_eq!(child.child_id, semio_s_artifact_cad_cad_model::snapshot::cad_pane_model(&scene, CadPaneId::Shape).unwrap().child_id);
        assert_eq!(child.ops.len(), 1);
        let mutation: SemioModelMutation = protocol::OpBinary::decode_op(&child.ops[0]).expect("canonical child operation");
        let SemioModelMutation::InsertElement(insert) = mutation else { panic!("exact insert-element media mutation") };
        assert_eq!(insert.element.geometry, semio_s_artifact_stdio_semio_model::schema::snapshot::GeometryRef::Brep { brep_id: created.child_id.clone() });
        meshes.push(semio_s_artifact_cad_cad_model::geometry::mesh_from_owned_brep(&geometry).expect("retained topology projects after importer disposal"));
    }
    for payload in [
        MediaPayload::Intrinsic { schema: "obj.3.0.geometry".into(), value: DslValue::Null },
        MediaPayload::Intrinsic { schema: "obj.3.0.geometry".into(), value: DslValue::String("invalid geometry".into()) },
        MediaPayload::Binary { format_kind: "obj.3.0.geometry".into(), blob_hash: "addressed external owner".into() },
    ] {
        let media = Media { media_type: media_type.clone(), payload };
        assert!(<CadPlayApp as ArtifactEditor>::import_media("geometry:in", &media, &ArtifactView::new(&scene, &history)).is_err());
    }
    drop(children);
    close(&mut mounted);
    for mesh in meshes {
        assert_eq!(mesh.indices.len() / 3, corpus["intrinsicGeometry"]["triangleCount"].as_u64().unwrap() as usize);
        assert_eq!(mesh.indices.iter().copied().collect::<std::collections::BTreeSet<_>>().len(), corpus["intrinsicGeometry"]["positionCount"].as_u64().unwrap() as usize);
        assert_eq!(mesh.vertex_ids.iter().filter(|id| **id != u32::MAX).count(), corpus["intrinsicGeometry"]["analyticVertexCount"].as_u64().unwrap() as usize);
        assert_eq!(mesh.positions.len() / 3, (corpus["intrinsicGeometry"]["positionCount"].as_u64().unwrap() + corpus["intrinsicGeometry"]["analyticVertexCount"].as_u64().unwrap()) as usize);
        assert_eq!(mesh.face_ids.len(), mesh.indices.len() / 3);
        mesh.validate_component_references().unwrap();
        let bounds: Vec<Vec<f32>> = [false, true].into_iter().map(|maximum| (0..3).map(|axis| mesh.positions.chunks_exact(3).map(|point| point[axis]).reduce(|left, right| if maximum { left.max(right) } else { left.min(right) }).expect("owned imported positions")).collect()).collect();
        let expected_bounds: Vec<Vec<f32>> = serde_json::from_str(&corpus["intrinsicGeometry"]["bounds"].to_string()).unwrap();
        assert_eq!(bounds, expected_bounds, "geometry resolves after the import kernel is gone");
    }
}

/// 🧊️ The mounted import retains actual geometry as one undoable parent and two-child publication.
#[semio_framework_async_macros::async_test]
async fn cad_owned_geometry_media_settles_renders_and_undo_redoes_atomically() {
    use semio_framework_plugin::PluginApp;
    let corpus: serde_json::Value = serde_json::from_str(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json").leak()).unwrap();
    let mut app = new_app().await;
    let before = app.snapshot().unwrap();
    let before_model = pane_model(&app, CadPaneId::Shape).await;
    let text = std::fs::read_to_string(fixture_path!("../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧪️tests/🧫️fixtures/📦️set-object-applied/⬅️before.obj")).expect("../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧪️tests/🧫️fixtures/📦️set-object-applied/⬅️before.obj").leak();
    let pricing: serde_json::Value = serde_json::from_str(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎞️media-pricing.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎞️media-pricing.json").leak()).unwrap();
    let large_source = format!("{}{}", pricing["prefix"].as_str().unwrap().repeat(pricing["repeat"].as_u64().unwrap() as usize), text);
    assert!(large_source.len() > pricing["closeMaximumBytes"].as_u64().unwrap() as usize);
    let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "obj.3.0.geometry".into(), json: large_source } };
    let outcome: Result<(), String> = async {
        PluginApp::import_media(&mut app, "geometry:in", media, &meta(semio_framework_os_kernel::LOCAL_ACTOR_ID)).await.map_err(|error| format!("{error:?}"))?;
        settle(&mut app).await;
        let after = app.snapshot().unwrap();
        if after.breps.len() != before.breps.len() + 1 { return Err("import did not create exactly one retained geometry sibling".into()); }
        let after_model = pane_model(&app, CadPaneId::Shape).await;
        if after_model.elements.len() != before_model.elements.len() + 1 { return Err("import did not insert exactly one model element".into()); }
        let geometry_id = after.breps.last().unwrap().child_id.clone();
        let element = after_model.elements.last().unwrap();
        if element.geometry != (semio_s_artifact_stdio_semio_model::schema::snapshot::GeometryRef::Brep { brep_id: geometry_id.clone() }) { return Err("model geometry reference escaped its exact retained sibling".into()); }
        let children = app.test_child_content_view();
        let panes = semio_s_artifact_cad_cad_play::CadComposedPanes::compose(&after, &children);
        let pane = panes.pane(CadPaneId::Shape);
        let object = pane.objects.iter().find(|object| object.id == element.id).ok_or("imported object is absent from composed rendering")?;
        let mesh = object_mesh_data(object, pane.geometry(CadPaneId::Shape));
        let positions = mesh.positions.chunks_exact(3).collect::<Vec<_>>();
        let bounds: Vec<Vec<f32>> = [false, true].into_iter().map(|maximum| (0..3).map(|axis| positions.iter().map(|point| point[axis]).reduce(|left, right| if maximum { left.max(right) } else { left.min(right) }).unwrap_or(f32::NAN)).collect()).collect();
        if mesh.indices.iter().copied().collect::<std::collections::BTreeSet<_>>().len() != corpus["intrinsicGeometry"]["positionCount"].as_u64().unwrap() as usize || mesh.vertex_ids.iter().filter(|id| **id != u32::MAX).count() != corpus["intrinsicGeometry"]["analyticVertexCount"].as_u64().unwrap() as usize || serde_json::to_value(&bounds).unwrap() != corpus["intrinsicGeometry"]["bounds"] { return Err(format!("rendered actual imported geometry differs from the independent OBJ witness: {bounds:?}, positions={}", positions.len())); }
        let mut kernel = cad_brep_kernel();
        let solids = semio_s_artifact_cad_cad_inferences::pane_world_solids(&mut kernel, std::slice::from_ref(object), pane.geometry(CadPaneId::Shape));
        let export = export_solids_as(&mut kernel, &solids, semio_s_artifact_cad_cad_io::CAD_SOLID_EXPORT_DIALECT_OBJ, "owned-geometry").ok_or("owned topology did not export")?;
        let exported = semio_s_artifact_cad_cad_io::import_obj_object(export.data.as_str().ok_or("OBJ export was not text")?).ok_or("owned OBJ export did not parse")?;
        let exported_mesh = semio_s_artifact_cad_cad_model::geometry::mesh_from_owned_brep(&exported.geometry)?;
        let exported_bounds: Vec<Vec<f32>> = [false, true].into_iter().map(|maximum| (0..3).map(|axis| exported_mesh.positions.chunks_exact(3).map(|point| point[axis]).reduce(|left, right| if maximum { left.max(right) } else { left.min(right) }).unwrap_or(f32::NAN)).collect()).collect();
        if exported_mesh.positions.len() / 3 != positions.len() || exported_bounds != bounds { return Err("OBJ export substituted geometry outside the exact retained topology child".into()); }
        let mut unavailable = pane.geometry(CadPaneId::Shape).cloned().ok_or("owned pane geometry projection absent")?;
        unavailable.owned_breps.clear();
        unavailable.owned_meshes.insert(geometry_id.clone(), std::sync::Arc::new(semio_framework_plugin::MeshData::default()));
        if !object_mesh_data(object, Some(&unavailable)).positions.is_empty() || !semio_s_artifact_cad_cad_inferences::pane_world_solids(&mut kernel, std::slice::from_ref(object), Some(&unavailable)).is_empty() { return Err("unavailable declared topology was replaced by invented typology geometry".into()); }
        drop(panes);
        drop(children);
        let rows = member_rows(&mut app).await;
        if rows.len() != 1 || rows[0].mutations.len() != 3 { return Err(format!("import does not publish exactly one atomic three-lane history row: {rows:?}")); }
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, "undo", TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        if app.snapshot().unwrap() != before || pane_model(&app, CadPaneId::Shape).await != before_model { return Err("undo did not restore the exact parent and model before geometry creation".into()); }
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, "redo", TEST_INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
        if app.snapshot().unwrap() != after || pane_model(&app, CadPaneId::Shape).await != after_model { return Err("redo did not restore the exact parent and model geometry ownership".into()); }
        let heads = PluginApp::child_head_packs(&app).await.map_err(|fault| format!("{fault:?}"))?.into_iter().map(|entry| (entry.slot, entry.child_id, entry.head_pack)).collect::<Vec<_>>();
        let archive = PluginApp::document_archive(&mut app.0).await.map_err(|fault| format!("{fault:?}"))?;
        if !archive.members.iter().any(|member| member.owner.slot == "breps" && member.owner.child_id == geometry_id) { return Err("recursive archive omitted the exact geometry owner".into()); }
        PluginApp::begin_document_archive_load(&mut app.0, 94, archive).map_err(|fault| format!("{fault:?}"))?;
        let mut terminal = None;
        for _ in 0..1_000_000 {
            let status = PluginApp::poll_document_archive_load(&mut app.0, 94, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.map_err(|fault| format!("{fault:?}"))?;
            if matches!(status.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) { terminal = Some(status); break; }
            PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).map_err(|fault| format!("{fault:?}"))?;
            std::thread::yield_now();
        }
        let terminal = terminal.ok_or("geometry archive did not reach its bounded terminal witness")?;
        if terminal.state != protocol::DocumentArchiveLoadState::Ready { return Err(format!("geometry archive failed: {}", String::from_utf8_lossy(&terminal.fault))); }
        PluginApp::acknowledge_document_archive_load(&mut app.0, 94).map_err(|fault| format!("{fault:?}"))?;
        let reloaded_heads = PluginApp::child_head_packs(&app).await.map_err(|fault| format!("{fault:?}"))?.into_iter().map(|entry| (entry.slot, entry.child_id, entry.head_pack)).collect::<Vec<_>>();
        if app.snapshot().unwrap() != after || pane_model(&app, CadPaneId::Shape).await != after_model || reloaded_heads != heads { return Err("archive reload did not preserve exact parent, model, and topology child heads".into()); }
        let children = app.test_child_content_view();
        let panes = semio_s_artifact_cad_cad_play::CadComposedPanes::compose(&after, &children);
        let pane = panes.pane(CadPaneId::Shape);
        let object = pane.objects.iter().find(|object| object.id == element.id).ok_or("reloaded geometry is absent from the composed pane")?;
        if object_mesh_data(object, pane.geometry(CadPaneId::Shape)) != mesh { return Err("archive reload altered actual rendered topology".into()); }
        drop(panes);
        drop(children);
        Ok(())
    }.await;
    close(&mut app);
    outcome.expect("mounted retained geometry media contract");
}

#[test]
fn cad_media_pricing_retains_exact_source_through_progress_cancel_and_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎞️media-pricing.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎞️media-pricing.json").leak()).unwrap();
    let source = format!("{}{}", fixture["prefix"].as_str().unwrap().repeat(fixture["repeat"].as_u64().unwrap() as usize), fixture["suffix"].as_str().unwrap());
    let unit = fixture["maximumUnitBytes"].as_u64().unwrap() as usize;
    let maximum = fixture["closeMaximumBytes"].as_u64().unwrap() as usize;
    let close_grant = |work: &CadMediaWork, items: usize| {
        use semio_framework_job::InteractiveJob as _;
        let copy = work.next_close_copy_byte_demand().unwrap().max(maximum);
        semio_framework_value::RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: copy, maximum_capacity_bytes: work.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: work.next_close_release_byte_demand().unwrap(), maximum_depth: work.next_close_depth_demand().unwrap().max(1) }
    };
    let expected_bytes = fixture["sourceBytes"].as_u64().unwrap() as usize;
    let expected_checksum = fixture["sourceChecksum"].as_str().unwrap().parse::<u64>().unwrap();
    assert_eq!(source.as_bytes().len(), expected_bytes);
    assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&source).unwrap()).unwrap().into_bytes(), source.as_bytes());
    for form in fixture["forms"].as_array().unwrap() {
        for cancel_at in fixture["cancelAtBytes"].as_array().unwrap().iter().map(|row| row.as_u64().unwrap() as usize).chain(std::iter::once(expected_bytes)) {
            let payload = match form.as_str().unwrap() {
                "intrinsicText" => MediaPayload::Intrinsic { schema: "cad.pricing.test".into(), value: semio_framework_value::DslValue::String(source.clone()) },
                "intrinsicBytes" => MediaPayload::Intrinsic { schema: "cad.pricing.test".into(), value: semio_framework_value::DslValue::Bytes(source.as_bytes().to_vec()) },
                "structuredText" => MediaPayload::Structured { schema: "cad.pricing.test".into(), json: source.clone() },
                _ => panic!("unknown declared media form"),
            };
            let mut work = CadMediaWork::new("geometry:in".into(), Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload }, None);
            let pointer = work.borrowed_source().unwrap().as_ptr();
            let cancel = semio_framework_job::root_cancel_token();
            let mut sequence = 0;
            let before = work.priced_bytes;
            assert_eq!(work.price_source(0).unwrap(), 0);
            assert_eq!(work.priced_bytes, before);
            let mut zero_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut zero = semio_framework_job::StepContext::new(semio_framework_job::OperationId(99), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(0,100_000,UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut zero_retained);
            assert!(!work.price_step(&mut zero).unwrap());
            assert_eq!(work.priced_bytes, before);
            let mut expired_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut expired = semio_framework_job::StepContext::new(semio_framework_job::OperationId(99), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1,0,UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut expired_retained);
            assert!(!work.price_step(&mut expired).unwrap());
            assert_eq!(work.priced_bytes, before);
            assert_eq!(expired.fuel_remaining(), 1);
            for _ in 0..fixture["maximumPricingSteps"].as_u64().unwrap() {
                if work.priced_bytes >= cancel_at { break; }
                let before = work.priced_bytes;
                let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
                let mut context = semio_framework_job::StepContext::new(semio_framework_job::OperationId(99), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fixture["grantWorkUnits"].as_u64().unwrap(), 100_000, UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut context_retained);
                assert!(!work.price_step(&mut context).unwrap());
                let charged = work.priced_bytes - before;
                assert_eq!(context.fuel_remaining(), 0);
                assert!(charged <= unit);
                assert_eq!(work.priced_bytes - before, charged);
                assert_eq!(work.borrowed_source().unwrap().as_ptr(), pointer);
                assert!(!work.consumed);
            }
            assert_eq!(work.priced_bytes, cancel_at);
            if cancel_at == expected_bytes {
                assert!(work.pricing_complete);
                assert_eq!(work.priced_bytes, expected_bytes);
                assert_eq!(work.price_checksum, expected_checksum);
            }
            let mut checkpoint = [0u8; 24];
            assert_eq!(work.checkpoint(&mut checkpoint, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).unwrap(), 24);
            let price_before = (work.priced_bytes, work.price_checksum, work.pricing_complete);
            work.restore(&checkpoint).unwrap();
            assert_eq!((work.priced_bytes, work.price_checksum, work.pricing_complete), price_before);
            semio_framework_async::poll::resolve_ready(cancel.cancel());
            let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(semio_framework_job::OperationId(99), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fixture["grantWorkUnits"].as_u64().unwrap(), 100_000, UNBOUNDED_STEP_GRANT), cancel.clone(), || Some(0), &mut sequence, &mut context_retained);
            assert_eq!(work.price_step(&mut context).unwrap_err().code.0, "interactive-job.cancelled");
            assert_eq!((work.priced_bytes, work.price_checksum, work.pricing_complete), price_before);
            assert_eq!(work.borrowed_source().unwrap().as_ptr(), pointer);
            assert!(!work.consumed);
            work.begin_close();
            assert_eq!(semio_framework_job::close_with_self_funded_context(close_grant(&work, 0), |cx| work.close_step(cx)).expect("original fixture close turn"), semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() });
            for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap() {
                let grant = close_grant(&work, 1);
                match semio_framework_job::close_with_self_funded_context(grant, |cx| work.close_step(cx)).expect("original fixture close turn") {
                    semio_framework_job::InteractiveJobCloseStep::Complete { .. } => break,
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } => { assert!(progress.copied_items <= 1); assert!(progress.released_bytes <= grant.maximum_release_bytes); },
                    step => panic!("priced source blocked exact bounded close: {step:?}"),
                }
            }
            assert!(work.terminal_is_empty());
        }
    }
    let mut work = CadMediaWork::new("geometry:in".into(), Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "cad.pricing.test".into(), json: "x".repeat(fixture["refusedBytes"].as_u64().unwrap() as usize) } }, None);
    let pointer = work.borrowed_source().unwrap().as_ptr();
    assert_eq!(work.price_source(unit).unwrap_err().code.0, "cad.media-input-capacity");
    assert_eq!(work.borrowed_source().unwrap().as_ptr(), pointer);
    assert_eq!(work.priced_bytes, 0);
    work.begin_close();
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap() {
        let grant = close_grant(&work, 1);
        match semio_framework_job::close_with_self_funded_context(grant, |cx| work.close_step(cx)).expect("original fixture close turn") {
            semio_framework_job::InteractiveJobCloseStep::Complete { .. } => break,
            semio_framework_job::InteractiveJobCloseStep::Pending { progress } => { assert!(progress.copied_items <= 1); assert!(progress.released_bytes <= grant.maximum_release_bytes); },
            step => panic!("refused source blocked exact bounded close: {step:?}"),
        }
    }
    assert!(work.terminal_is_empty());
}

#[test]
fn cad_imported_geometry_parent_operation_pack_keeps_exact_forward_inverse_owners() {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let factory=<CadPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().unwrap();
    let input=[
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/⚡create-energy-model/⚡️rehandles/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/➕create-node/🌱️appends-node-3/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏛️create-structure-classic/🏛️rehandles/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏛️create-structure-classic/🏛️rehandles/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏛️create-structure-classic/🏛️rehandles/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏛️create-structure-classic/🏛️rehandles/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏢create-building-model/🏢️rehandles/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🏷️rename-node/🔤️relabels-the-root-node/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/👁️change-reference-hidden/🙈️hides/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💣delete-structure-classic-model/🏚️vacates/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/💥delete-building-model/🏚️vacates/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📍move-reference/📍️moves/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📎replace-references/🔄️swaps/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📏change-reference-width/↔️widens/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📐️create-drawing/📐️appends-drawing-2/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔌delete-energy-model/🔌️vacates/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔒change-reference-locked/🔓️unlocks/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖇️replace-reference-media/🖼️reattaches/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-node/🚫️removes-node-2/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-brep/🧊️inserts-topology-at-middle/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧨delete-shape-model/🕳️vacates-the-shape-slot/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-brep/🧹️removes-middle-topology/📸️snapshot/⬅️before/🔣️.json").leak()),
        (std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🦠️mutation/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🦠️mutation/🔣️.json").leak(),std::fs::read_to_string(fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/📸️snapshot/⬅️before/🔣️.json")).expect("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/📸️snapshot/⬅️before/🔣️.json").leak()),
    ];
    assert_eq!(input.len(),21);
    for (input,before) in input {
        let operation:CadMutation=semio_framework_pack_json::from_json_str(input,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&operation)).unwrap(),serde_json::from_str::<serde_json::Value>(input).unwrap());
        let before:CadSnapshot=semio_framework_pack_json::from_json_str(before,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut operations=protocol::Mutation::inverse(&operation,&before).unwrap();operations.push(operation);
        for operation in operations {
        let expected=semio_s_artifact_cad_cad_mutations::standards::v1::subsets::any::io::binary::mutations::encode_op(&operation).unwrap();
        assert!(factory.operation_wire_source(&operation).is_some(),"original parent mutation requires exact borrowed installed Pack authority");
        for maximum in [1,3,7,64,256] {
            let mut cursor=store::ArtifactPreparedOperationCursor::default();let mut bytes=Vec::new();let mut complete=false;
            for _ in 0..20000 {
                let demand=cursor.next_capacity_byte_demand().unwrap();
                let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:demand,maximum_release_bytes:0,maximum_depth:64};
                let mut output=[0;256];
                let (step,heap)=semio_framework_io_sqlite_snapshot_testing_backing::measure(||cursor.advance(factory.operation_wire_source(&operation).unwrap(),&mut output[..maximum],grant).unwrap());
                assert_eq!((heap.bytes,heap.released_bytes),(step.retained_capacity_bytes,step.released_bytes));assert!(step.copied_bytes<=64 && step.processed_items<=1);bytes.extend_from_slice(&output[..step.written_bytes]);
                if step.complete{complete=true;break}
            }
            assert!(complete);assert_eq!(bytes,expected);
            let mut closed=false;
            for _ in 0..20000 {
                let demand=cursor.next_close_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand,..Default::default()};
                let (step,heap)=semio_framework_io_sqlite_snapshot_testing_backing::measure(||cursor.close(grant).unwrap());assert_eq!((heap.bytes,heap.released_bytes),(step.retained_capacity_bytes,step.released_bytes));if step.complete{closed=true;break}
            }
            assert!(closed);
            assert_eq!(semio_s_artifact_cad_cad_mutations::standards::v1::subsets::any::io::binary::mutations::decode_op(&bytes).unwrap(),operation);
        }
        }
    }
}
