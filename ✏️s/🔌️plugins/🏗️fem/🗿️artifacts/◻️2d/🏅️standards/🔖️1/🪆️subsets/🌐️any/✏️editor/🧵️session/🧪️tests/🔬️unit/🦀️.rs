fn prepare_test(render:AppRenderOperationContext,snapshot:&Fem2dSnapshot)->bool{let windows=semio_framework_plugin::WindowConfigOwnerRegistry::new(protocol::ActorId("fem2d-preparation-test".into()));let steps=128+snapshot.nodes.len()*64+snapshot.elements.len()*64+snapshot.regions.iter().map(|region|(region.outer.len()+region.holes.iter().map(Vec::len).sum::<usize>())*32+128).sum::<usize>()+snapshot.load_cases.len()*128+snapshot.combinations.len()*128;for _ in 0..steps{let step=prepare_snapshot_read(render,snapshot,&windows,None,render.mounted_policy.preparation);assert!(step.progress.fits(render.mounted_policy.preparation));if step.ready{return true;}if step.progress==Default::default(){return false;}}false}
use super::*;

/// 📚️ The session module as the unsplit file held it: the root, then every mounted child in declaration order.
fn session_source() -> String {
    [
        include_str!("../../🦀️.rs"),
        include_str!("../../🪪️session-constants-and-owner-catalog/🦀️.rs"),
        include_str!("../../🏗️model-build-job/🦀️.rs"),
        include_str!("../../🏁️model-build-close/🦀️.rs"),
        include_str!("../../🗂️mounted-state-and-preparation/🦀️.rs"),
        include_str!("../../🔁️mounted-state-step/🦀️.rs"),
        include_str!("../../🧹️mounted-state-close/🦀️.rs"),
        include_str!("../../📍️snapshot-admission-and-registry/🦀️.rs"),
        include_str!("../../💼️job-bridge-and-live-mount/🦀️.rs"),
    ]
    .concat()
}

#[test]
fn mounted_static_sources_use_physical_rhs_and_canonical_identity() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for source in fixture["sources"].as_array().unwrap() {
        let kind = source["kind"].as_str().unwrap();
        let id = source["id"].as_str().unwrap();
        let source = semio_s_fem_analyses_model::ResultSource::from_parts(kind, id.to_owned()).unwrap();
        let wire = semio_s_fem_analyses_model::encode_result_source_identity(&source).unwrap();
        assert_eq!(semio_s_fem_analyses_model::decode_result_source_identity(&wire).unwrap(), source);
    }
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("PcgJobConstruction::new_with_mounted_rhs"));
    assert!(source.contains("AssemblyCsrBuild::new_free"));
    assert!(!source.contains("snapshot.regions.first()"));
    assert!(source.contains("visual_equation_indices(&node.id"));
}

fn mounted_test_operation() -> Operation {
    Operation::new(OperationId(791), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 4)
}

fn drain_mounted_job(job: &mut impl InteractiveJob) {
    let operation = mounted_test_operation();
    let mut sequence = 0;
    let grant = RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 32768, maximum_capacity_bytes: 4 * 1024 * 1024, maximum_release_bytes: 32768, maximum_depth: 4096 };
    for _ in 0..100_000 {
        let mut receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(64, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
        match child_end(job.step(&mut context)) {
            ChildEnd::Complete(_) => return,
            ChildEnd::Fault(detail) => panic!("[DEBUG] mounted job fault: {:?}", detail.map(|detail| String::from_utf8_lossy(&detail).into_owned())),
            ChildEnd::Cancelled => panic!("[DEBUG] unexpected cancellation"),
            _ => {}
        }
    }
    panic!("[DEBUG] mounted job did not complete its bounded opportunities");
}

#[test]
fn mounted_admission_exceeds_old_demo_caps_without_synthetic_forcing() {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
    use semio_s_fem_material::planar::FemMaterialValue;
    use semio_s_fem_node::planar::FemNodeValue;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSupport;
    let count = 50;
    let doc = Fem2dSnapshot {
        nodes: (0..count).map(|index| FemNodeValue { id: format!("n{index}"), x: 2.0 * index as f64 / (count - 1) as f64, y: 0.0 }).collect(),
        elements: (1..count).map(|index| FemElement::Bar { id: format!("e{index}"), start: format!("n{}", index - 1), end: format!("n{index}"), material_id: "steel".into(), section_id: "section".into() }).collect(),
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "section".into(), name: "Section".into(), area: 0.01, iy: 0.00001 }],
        supports: (0..count).map(|index| FemSupport { id: format!("f{index}"), node_id: format!("n{index}"), fixed: if index == 0 { vec![FemDof::Tx, FemDof::Ty] } else { vec![FemDof::Ty] } }).collect(),
        load_cases: vec![FemLoadCase { id: "tip".into(), name: "Tip".into(), loads: vec![FemLoad::Nodal { id: "force".into(), node_id: format!("n{}", count - 1), dof: FemDof::Tx, value: 1000.0 }], self_weight: false }],
        ..Default::default()
    };
    let mut builder = MountedModelBuild::new(None);
    assert!((0..100_000).any(|_| builder.step_one(&doc).unwrap()));
    let model = builder.take_complete().unwrap();
    assert_eq!(model.nodes_len(), count);
    assert_eq!(model.elements_len(), count - 1);
    let mut construction = AssemblyJobConstruction::new_mounted(model, mounted_test_operation(), 1);
    assert!((0..100_000).any(|_| construction.step_one().unwrap()));
    let mut assembly = construction.take_complete().unwrap();
    drain_mounted_job(&mut assembly);
    let mut loads = MountedLoadBuild::new(resolved_source(&snapshot,Some(semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:"tip"})).unwrap());
    assert!((0..100_000).any(|_| loads.step_one(&doc, &assembly).unwrap()));
    let mut csr = AssemblyCsrBuild::new_free(assembly).ok().unwrap();
    assert!((0..100_000).any(|_| csr.step_one().unwrap()));
    let mut construction = PcgJobConstruction::new_with_mounted_rhs(mounted_test_operation(), csr.take_complete().unwrap(), std::mem::take(&mut loads.free)).ok().unwrap();
    assert!((0..100_000).any(|_| construction.step_one().unwrap()));
    let mut pcg = construction.take_complete().unwrap();
    drain_mounted_job(&mut pcg);
    let free = csr.visual_equation_indices("n49", semio_s_fem_model::Dof::Tx).unwrap().1.unwrap();
    let actual = pcg.visual_scalar(free).unwrap().displacement;
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let expected = fixture["axialChain"]["tipX"].as_f64().unwrap();
    eprintln!("[DEBUG] FEM2D paged axial chain nodes={count}, elements={}, tip_x={actual:.16e}, expected={expected:.16e}, progress={:?}", count - 1, pcg.visual_progress());
    assert!((actual - expected).abs() < 1e-12);
    assert!(pcg.visual_progress().4);
}

#[test]
fn mounted_cantilever_sources_match_committed_oracle() {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombination;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
    use semio_s_fem_material::planar::FemMaterialValue;
    use semio_s_fem_node::planar::FemNodeValue;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSupport;
    let doc = Fem2dSnapshot {
        nodes: vec![FemNodeValue { id: "root".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "tip".into(), x: 2.0, y: 0.0 }],
        elements: vec![FemElement::Beam { id: "beam".into(), start: "root".into(), end: "tip".into(), material_id: "steel".into(), section_id: "section".into() }],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "section".into(), name: "Section".into(), area: 0.01, iy: 0.00001 }],
        supports: vec![FemSupport { id: "fixed".into(), node_id: "root".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Rz] }],
        load_cases: vec![FemLoadCase { id: "empty".into(), name: "Empty".into(), loads: vec![], self_weight: false }, FemLoadCase { id: "tip".into(), name: "Tip".into(), loads: vec![FemLoad::Nodal { id: "force".into(), node_id: "tip".into(), dof: FemDof::Ty, value: -1000.0 }], self_weight: false }],
        combinations: vec![FemCombination { id: "tip".into(), name: "Reverse".into(), terms: vec![FemCombinationTerm { case_id: "tip".into(), factor: -2.0 }] }],
        ..Default::default()
    };
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for reversed in [false, true] {
        let mut doc = doc.clone();
        if reversed { doc.load_cases.reverse(); }
        for row in fixture["sources"].as_array().unwrap() {
            let source = ResultSource::from_parts(row["kind"].as_str().unwrap(), row["id"].as_str().unwrap().to_owned()).unwrap();
            let mut builder = MountedModelBuild::new(None);
            assert!((0..10_000).any(|_| builder.step_one(&doc).unwrap()));
            let mut construction = AssemblyJobConstruction::new_mounted(builder.take_complete().unwrap(), mounted_test_operation(), 1);
            assert!((0..100_000).any(|_| construction.step_one().unwrap()));
            let mut assembly = construction.take_complete().unwrap();
            drain_mounted_job(&mut assembly);
            let tip_y = assembly.visual_equation_indices("tip", semio_s_fem_model::Dof::Ty).unwrap().1.unwrap();
            let tip_rz = assembly.visual_equation_indices("tip", semio_s_fem_model::Dof::Rz).unwrap().1.unwrap();
            let mut loads = MountedLoadBuild::new(resolved_source(&snapshot,Some(semio_s_fem_analyses_model::ResultSourceIdentity{kind:source.kind(),id:source.id()})).unwrap());
            assert!((0..100_000).any(|_| loads.step_one(&doc, &assembly).unwrap()));
            let mut csr = AssemblyCsrBuild::new_free(assembly).ok().unwrap();
            assert!((0..100_000).any(|_| csr.step_one().unwrap()));
            let mut construction = PcgJobConstruction::new_with_mounted_rhs(mounted_test_operation(), csr.take_complete().unwrap(), std::mem::take(&mut loads.free)).ok().unwrap();
            assert!((0..100_000).any(|_| construction.step_one().unwrap()));
            let mut pcg = construction.take_complete().unwrap();
            drain_mounted_job(&mut pcg);
            let y = pcg.visual_scalar(tip_y).unwrap().displacement;
            let rz = pcg.visual_scalar(tip_rz).unwrap().displacement;
            eprintln!("[DEBUG] FEM2D source={source:?}, reversed={reversed}, tip_y={y:.16e}, tip_rz={rz:.16e}, progress={:?}", pcg.visual_progress());
            assert!((y - row["tipY"].as_f64().unwrap()).abs() < 1e-12);
            assert!((rz - row["tipRz"].as_f64().unwrap()).abs() < 1e-12);
            assert!(pcg.visual_progress().4);
        }
    }
}

#[test]
fn mounted_regions_share_boundary_nodes_and_preserve_both_materials() {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
    use semio_s_fem_material::planar::FemMaterialValue;
    use semio_s_fem_node::planar::FemNodeValue;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSupport;
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let regions = vec![
        FemRegion { id: "a".into(), name: "A".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], holes: vec![], thickness: 0.1, material_id: "a".into(), mesh_size: 2.0 },
        FemRegion { id: "b".into(), name: "B".into(), outline: vec![[1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], thickness: 0.2, material_id: "b".into(), mesh_size: 2.0 },
    ];
    let materials = vec![FemMaterialValue { id: "a".into(), name: "A".into(), e: 10e9, nu: 0.2, rho: 1000.0 }, FemMaterialValue { id: "b".into(), name: "B".into(), e: 20e9, nu: 0.3, rho: 2000.0 }];
    for reversed in [false, true] {
        let mut doc = Fem2dSnapshot {
            nodes: vec![FemNodeValue { id: "root".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "edge".into(), x: 1.0, y: 0.0 }],
            regions: regions.clone(), materials: materials.clone(),
            supports: vec![FemSupport { id: "f0".into(), node_id: "root".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "f1".into(), node_id: "edge".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "area".into(), name: "Area".into(), loads: vec![FemLoad::Area { id: "a".into(), region_id: "a".into(), pressure: fixture["regions"]["areaLoads"][0].as_f64().unwrap() }, FemLoad::Area { id: "b".into(), region_id: "b".into(), pressure: fixture["regions"]["areaLoads"][1].as_f64().unwrap() }], self_weight: false }],
            ..Default::default()
        };
        if reversed { doc.regions.reverse(); }
        let mesh = |index: usize| TriMesh2 { points: doc.regions[index].outline.clone(), tris: vec![[0, 1, 2]] };
        let mut builder = MountedModelBuild::new(Some(mesh(0)));
        assert!((0..10_000).any(|_| builder.step_one(&doc).unwrap()));
        builder.close_lane = 7;
        assert!((0..10_000).any(|_| builder.close_step(SESSION_OWNER_PAGE_BYTES).0));
        builder.attach_region(1, mesh(1));
        assert!((0..10_000).any(|_| builder.step_one(&doc).unwrap()));
        let model = builder.take_complete().unwrap();
        assert_eq!(model.nodes_len(), 4);
        assert_eq!(model.elements_len(), 2);
        for index in 0..2 {
            let semio_s_fem_static::Elements::Tri3Cst(element) = model.element(index).unwrap() else { panic!("continuum owner"); };
            assert_eq!(element.thickness, doc.regions[index].thickness);
            assert_eq!(element.e, doc.materials.iter().find(|material| material.id == doc.regions[index].material_id).unwrap().e);
        }
        eprintln!("[DEBUG] FEM2D two regions reversed={reversed}, nodes={}, triangles={}", model.nodes_len(), model.elements_len());
        let free_nodes: Vec<_> = fixture["regions"]["freeCoordinates"].as_array().unwrap().iter().map(|coordinate| {
            (0..model.nodes_len()).filter_map(|index| model.node(index)).find(|node| node.pos[..2] == [coordinate[0].as_f64().unwrap(), coordinate[1].as_f64().unwrap()]).unwrap().id.clone()
        }).collect();
        let mut construction = AssemblyJobConstruction::new_mounted(model, mounted_test_operation(), 1);
        assert!((0..100_000).any(|_| construction.step_one().unwrap()));
        let mut assembly = construction.take_complete().unwrap();
        drain_mounted_job(&mut assembly);
        let mut loads = MountedLoadBuild::new(resolved_source(&snapshot,Some(semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:"area"})).unwrap());
        assert!((0..100_000).any(|_| loads.step_one(&doc, &assembly).unwrap()));
        let mut csr = AssemblyCsrBuild::new_free(assembly).ok().unwrap();
        assert!((0..100_000).any(|_| csr.step_one().unwrap()));
        let mut construction = PcgJobConstruction::new_with_mounted_rhs(mounted_test_operation(), csr.take_complete().unwrap(), std::mem::take(&mut loads.free)).ok().unwrap();
        assert!((0..100_000).any(|_| construction.step_one().unwrap()));
        let mut pcg = construction.take_complete().unwrap();
        drain_mounted_job(&mut pcg);
        for (node_index, node) in free_nodes.iter().enumerate() {
            for (axis, dof) in [semio_s_fem_model::Dof::Tx, semio_s_fem_model::Dof::Ty].into_iter().enumerate() {
                let index = csr.visual_equation_indices(node, dof).unwrap().1.unwrap();
                let actual = pcg.visual_scalar(index).unwrap().displacement;
                let expected = fixture["regions"]["freeDisplacements"][node_index][axis].as_f64().unwrap();
                eprintln!("[DEBUG] FEM2D regions reversed={reversed}, node={node}, dof={dof:?}, actual={actual:.16e}, expected={expected:.16e}");
                assert!((actual - expected).abs() < 1e-14);
            }
        }
    }
}

const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_capacity_bytes: 0, maximum_release_bytes: 4_096, maximum_depth: SESSION_CLOSE_DEPTH };

/// 🎫️ Independent app instances keep exact pending leases through sibling close and slot reuse.
#[test]
fn mounted_app_admission_isolates_colliding_ids_until_original_children_return() {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 };
    let render = |app_instance_id| AppRenderOperationContext { app_instance_id, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32], mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
    let snapshot = Fem2dSnapshot::default();
    for app in [1, 33] { let mut ready=false;for _ in 0..128{if prepare_test(render(app), &snapshot){ready=true;break;}}assert!(ready); }
    let (first, second, retained) = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let first = registry.app_slots.lookup(1).unwrap();
        let second = registry.app_slots.lookup(33).unwrap();
        (first, second, registry.pending[second.index()].unwrap())
    });
    assert_ne!(first.index(), second.index());
    assert!(matches!(close_step(1, RetainedCloneGrant::default()), PluginLifecycleStep::Progress(progress) if progress == Default::default()));
    for _ in 0..16 { if terminal_is_empty(1) { break } close_step(1, grant); }
    assert!(terminal_is_empty(1));
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        assert_eq!(registry.app_slots.lookup(33), Some(second));
        let current = registry.pending[second.index()].unwrap();
        assert_eq!(current.identity, retained.identity);
        assert_eq!(current.shell, retained.shell);
    });
    let _=prepare_test(render(65),&snapshot);
    let reused = MOUNTED.with(|registry| registry.borrow().app_slots.lookup(65).unwrap());
    assert_eq!(reused.index(), first.index());
    assert_ne!(reused.generation(), first.generation());
    MOUNTED.with(|registry| assert!(!registry.borrow_mut().app_slots.release(1, first)));
    for app in [33, 65] { for _ in 0..16 { if terminal_is_empty(app) { break } close_step(app, grant); } assert!(terminal_is_empty(app)); }
    MOUNTED.with(|registry| { let registry = registry.borrow(); assert!(registry.app_slots.is_empty()); assert_eq!(registry.reserved_bytes, 0); assert_eq!(registry.reserved_items, 0); });
    println!("[DEBUG] FEM2D app IDs 1/33 own independent pending jobs; funded close preserves sibling identity and stale reuse cannot release app 65");
}

/// 🧵️ A genuine Store snapshot lease prevents registry release while its mounted child is borrowed.
#[semio_framework_async_macros::async_test]
async fn mounted_app_close_keeps_borrowed_original_child_and_full_digest_authority() {
    use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
    let snapshot = Fem2dSnapshot::default();
    let mut document = store::ArtifactStore::<Fem2dSnapshot, Fem2dMutation>::new(store::os_store::document_materialize::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA, "mounted-child-affinity", snapshot.clone(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
    document.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Fem2dSnapshot, Fem2dMutation>().unwrap()).unwrap_or_else(|(error, _)| panic!("original owners refused: {error}"));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 };
    let render = AppRenderOperationContext { app_instance_id: 101, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(document.generation()), canonical_base_revision: document.content_revision(), mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
    assert!(prepare_test(render, &snapshot));
    let read = document.snapshot_read().unwrap();
    let owner = MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let slot = registry.app_slots.lookup(render.app_instance_id).unwrap().index();
        let pending = registry.pending[slot].take().unwrap();
        let owner = registry.shells[pending.shell as usize].clone();
        *owner.borrow_mut() = Some(MountedState::new(pending.identity, read, pending.admitted_items));
        registry.current[slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell: pending.shell, identity: pending.identity, previous_valid: None });
        owner
    });
    assert!(!prepare_test(render, &snapshot));
    let mut changed = render;
    changed.canonical_base_revision[0] ^= 1;
    assert!(prepare_test(changed, &snapshot), "same numeric revision and generation cannot suppress a different original canonical digest");
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let slot = registry.app_slots.lookup(render.app_instance_id).unwrap().index();
        let pending = registry.pending[slot].take().unwrap();
        registry.release_credit(pending.shell);
        registry.release(pending.shell);
        let current = registry.current[slot].take().unwrap();
        assert!(registry.retain_retirement(current.shell, current.app_instance_id));
    });
    let borrowed = owner.borrow_mut();
    assert!((0..16).any(|_| matches!(close_step(render.app_instance_id, grant), PluginLifecycleStep::Blocked { .. })), "request metadata may retire first, while the original borrowed child must block the key release");
    assert!(!terminal_is_empty(render.app_instance_id));
    assert!(MOUNTED.with(|registry| registry.borrow().app_slots.lookup(render.app_instance_id).is_some()));
    drop(borrowed);
    for _ in 0..4096 { if terminal_is_empty(render.app_instance_id) { break } assert!(!matches!(close_step(render.app_instance_id, grant), PluginLifecycleStep::Blocked { .. })); }
    assert!(terminal_is_empty(render.app_instance_id));
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<Fem2dSnapshot, Fem2dMutation>();
    for _ in 0..4096 {
        if disposer.terminal_is_empty(&document) { break }
        let demand = disposer.retirement_demands(&document, 1 << 20).unwrap();
        let funded = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(1 << 20), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        disposer.close_step(&mut document, funded).unwrap();
    }
    assert!(disposer.terminal_is_empty(&document));
    println!("[DEBUG] FEM2D original Store snapshot child remains owned while borrowed; full digest change admits a replacement and child-first funded close returns exact Store custody");
}

#[test]
fn input_authority_round_trips_and_rejects_hostile_lengths() {
    let job = FEM2D_JOB_TAG | 19;
    let identity = MountedIdentity { app_instance_id: 7, base_revision: semio_framework_job::RevisionId(11), generation: semio_framework_job::Generation(13), canonical_base_revision: [23; 32], operation: OperationId(job), job };
    assert_eq!(decode_input(job, &encode_input(3, identity)), Some((3, identity)));
    assert_eq!(decode_input(job, &encode_input(3, identity)[..INPUT_BYTES - 1]), None);
    assert_eq!(decode_input(job + 1, &encode_input(3, identity)), None, "a host job-id substitution must not inherit the retained owner");
}

#[test]
fn mounted_close_and_capacity_are_fixed_and_terminal_witnessed() {
    let mut registry = MountedRegistry::new();
    let mut admitted = Vec::new();
    for _ in 0..SESSION_SHELL_CAPACITY {
        admitted.push(registry.allocate().expect("exact fixed slot"));
    }
    assert!(registry.allocate().is_none(), "capacity plus one must fail without growing");
    let returned = admitted.pop().expect("slot");
    registry.release(returned);
    assert_eq!(registry.allocate(), Some(returned), "fixed free ring deterministically reuses the returned slot");
    let identity = MountedIdentity { app_instance_id: 1, base_revision: semio_framework_job::RevisionId(2), generation: semio_framework_job::Generation(3), canonical_base_revision: [4; 32], operation: OperationId(5), job: 5 };
    let first = registry.app_slots.claim(1).unwrap();
    registry.current[first.index()] = Some(CurrentSession { app_instance_id: 1, shell: returned, identity, previous_valid: None });
    let second = registry.app_slots.claim(33).unwrap();
    assert_ne!(first.index(), second.index(), "exact app keys occupy independent slots even when their old modulo buckets collide");
    assert!(registry.current[second.index()].is_none());
    assert_eq!(maintenance_step(u32::MAX, TEST_GRANT), PluginLifecycleStep::Complete(Default::default()));
    assert!(terminal_is_empty(u32::MAX));
}

#[test]
fn snapshot_lease_is_preceded_by_a_fixed_pending_admission_and_idle_polling_reuses_it() {
    let render = AppRenderOperationContext { app_instance_id: 2_000_000_007, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32], mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, maintenance: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, close: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 } } };
    let snapshot = Fem2dSnapshot::default();
    assert!((0..64).any(|_| prepare_test(render, &snapshot)), "empty schema census completes incrementally");
    let first = MOUNTED.with(|registry| { let registry = registry.borrow(); registry.pending[registry.app_slots.lookup(render.app_instance_id).unwrap().index()].expect("pending admission") });
    for _ in 0..1_025 {
        assert!(prepare_test(render, &snapshot));
    }
    let retained = MOUNTED.with(|registry| { let registry = registry.borrow(); registry.pending[registry.app_slots.lookup(render.app_instance_id).unwrap().index()].expect("same pending admission") });
    assert_eq!(first.shell, retained.shell);
    assert_eq!(first.identity, retained.identity);
    assert!(matches!(close_step(render.app_instance_id, TEST_GRANT), PluginLifecycleStep::Progress(progress) if progress.copied_items == 1));
    for _ in 0..8 { if terminal_is_empty(render.app_instance_id) { break } close_step(render.app_instance_id, TEST_GRANT); }
    assert!(terminal_is_empty(render.app_instance_id));
}

#[test]
fn snapshot_census_completes_within_one_opportunity_and_rejects_exact_plus_one_without_partial_credit() {
    let render = AppRenderOperationContext { app_instance_id: 2_000_000_006, base_revision: semio_framework_job::RevisionId(29), generation: semio_framework_job::Generation(31), canonical_base_revision: [37; 32], mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, maintenance: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, close: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 } } };
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot();
    let mut census = SnapshotAdmissionCursor::new();
    let mut units = 0;
    while census.step_one(&snapshot) == Ok(false) {
        units += 1;
    }
    assert!(units > 1 && units < SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY, "the bundled demo census is many single-owner units but fits one opportunity: {units}");
    assert!(prepare_test(render, &snapshot), "the demo census is drained and its lease shell admitted in the opportunity that opened it");
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let slot = registry.app_slots.lookup(render.app_instance_id).unwrap().index();
        assert!(registry.preflight[slot].is_none(), "a completed census releases its cursor");
        assert!(registry.pending[slot].is_some(), "the lease shell is admitted once the census completes");
    });

    let mut exact = SnapshotAdmissionCursor::new();
    exact.items = SESSION_MAXIMUM_INPUT_ITEMS - 1;
    exact.bytes = SESSION_MAXIMUM_INPUT_BYTES - 1;
    assert_eq!(exact.charge(1, 1), Ok(()));
    let before = (exact.items, exact.bytes);
    assert_eq!(exact.charge(1, 0), Err(b"fem2d.session-admission-exceeded" as &'static [u8]));
    assert_eq!((exact.items, exact.bytes), before, "plus one returns the exact unchanged census owner");
    assert!(matches!(close_step(render.app_instance_id, TEST_GRANT), PluginLifecycleStep::Progress(progress) if progress.copied_items == 1));
    for _ in 0..8 { if terminal_is_empty(render.app_instance_id) { break } close_step(render.app_instance_id, TEST_GRANT); }
    assert!(terminal_is_empty(render.app_instance_id));
}

#[test]
fn process_owner_inventory_admits_exact_maximum_and_returns_exact_credit() {
    let catalog = MountedProcessOwnerCatalog::fixed();
    let mut seen = [false; 30];
    for claim in catalog.claims {
        let index = claim.class as usize;
        assert!(!seen[index], "every simultaneous owner class is inventoried exactly once");
        assert!(claim.roots != 0, "every class retains at least one fixed backing root");
        seen[index] = true;
    }
    assert!(seen.into_iter().all(|present| present));
    let exact = catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES).expect("the enumerated working set exactly fits its process authority");
    assert_eq!(exact, (SESSION_MAXIMUM_ITEMS, SESSION_MAXIMUM_BYTES));
    assert_eq!(catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS + 1, SESSION_MAXIMUM_INPUT_BYTES), Err(b"fem2d.session-process-input-credit-exceeded" as &'static [u8]));
    assert_eq!(catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES + 1), Err(b"fem2d.session-process-input-credit-exceeded" as &'static [u8]));

    let mut overflow = catalog;
    overflow.claims[MountedOwnerClass::OutputPages as usize].roots += 1;
    assert_eq!(overflow.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES), Err(b"fem2d.session-process-admission-exceeded" as &'static [u8]));

    let mut registry = MountedRegistry::new();
    let shell = registry.allocate().expect("fixed process shell");
    assert!(registry.reserve_credit(shell, exact.0, exact.1));
    let rejected = registry.allocate().expect("distinct fixed shell retains a failed request");
    assert!(!registry.reserve_credit(rejected, exact.0 + 1, exact.1), "plus one cannot partially change the registry credit");
    assert_eq!((registry.credit_items[rejected as usize], registry.credit_bytes[rejected as usize]), (0, 0));
    registry.release(rejected);
    registry.release_credit(shell);
    registry.release(shell);
    assert_eq!((registry.reserved_items, registry.reserved_bytes), (0, 0));
    let returned: [u16; SESSION_SHELL_CAPACITY] = std::array::from_fn(|_| registry.allocate().expect("all fixed shells returned"));
    assert_eq!(&returned[SESSION_SHELL_CAPACITY - 2..], &[rejected, shell], "failed and completed admissions return their exact shells in FIFO order");
}

#[test]
fn interrupted_model_close_releases_one_retained_root_per_grant() {
    let mut build = MountedModelBuild::new(None);
    while !build.model.admit_node_one(1, SESSION_OWNER_PAGE_BYTES).unwrap().complete {}
    build.model.push_node(semio_s_fem_model::Node { id: "deep-node".repeat(128), pos: [0.0; 3] }).ok().unwrap();
    build.region_node_ids.push("deep-region-node".repeat(128));
    let first = build.close_step(4_096);
    assert!(!first.0);
    assert_eq!(build.model.nodes_len(), 1);
    assert_eq!(build.model.node(0).unwrap().id.capacity(), 0, "the first grant releases only the nested string backing");
    assert_eq!(build.region_node_ids.len(), 1, "a distinct retained root survives the interrupted close");
    while build.model.nodes_len() != 0 {
        assert!(!build.close_step(4_096).0);
    }
    assert_eq!(build.region_node_ids.len(), 1, "later close lanes remain untouched while the model page retires");
    while !build.close_step(4_096).0 {}
}

#[test]
fn mounted_revision_restart_keeps_cancel_before_spawn() {
    let source = session_source();
    let source = source.as_str();
    let admission = &source[source.find("fn prepare_snapshot_admission(").expect("admission")..source.find("pub fn reconcile(").expect("reconcile")];
    let reconcile = &source[source.find("pub fn reconcile(").expect("reconcile")..source.find("pub fn with_live_visual").expect("visual boundary")];
    assert!(reconcile.find("Effect::CancelJob").expect("cancel effect") < reconcile.find("Effect::SpawnJob").expect("spawn effect"));
    assert!(admission.contains("checked_add(1)"));
    assert!(admission.contains("reserve_credit(shell, process_items, process_bytes)"));
    assert!(reconcile.contains("take_snapshot_read()"));
    assert!(reconcile.contains("retiring.iter().any(Option::is_none)"));
}

#[test]
fn commit_identity_requires_base_revision_and_generation() {
    let operation = Operation::new(OperationId(1), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 4);
    assert_eq!(semio_framework_job::validate_commit(&operation, operation.base_revision, operation.generation), CommitValidation::Accepted);
    assert!(matches!(semio_framework_job::validate_commit(&operation, semio_framework_job::RevisionId(9), operation.generation), CommitValidation::Stale { .. }));
    assert!(matches!(semio_framework_job::validate_commit(&operation, operation.base_revision, semio_framework_job::Generation(9)), CommitValidation::Stale { .. }));
}

#[test]
fn source_contract_keeps_one_semantic_child_step_and_live_visual_consumer() {
    let source = session_source();
    let source = source.as_str();
    for needle in [
        "FemJobGraph::new",
        "MeshJob::new",
        "AssemblyJobConstruction::new_mounted",
        "PcgJobConstruction::new",
        "commit_authority_matches",
        "Effect::SpawnJob",
        "Effect::CancelJob",
        "JobPlacement::Isolated",
        "with_live_visual",
        "take_snapshot_read",
        "SESSION_MAXIMUM_ITEMS",
        "SESSION_MAXIMUM_BYTES",
        "MountedProcessOwnerCatalog",
        "MountedOwnerClass::AssemblyDofStrings",
    ] {
        assert!(source.contains(needle), "missing mounted FEM contract {needle}");
    }
    for forbidden in ["semio_s_artifact_fem_2d_engine::fem2d_engine::build_model(", "AssemblyJob::new_owned(", ".into_full_matrix(", "PcgJob::new(", "pending_node_ids", ".fixed.iter().map"] {
        assert!(!source.contains(forbidden), "mounted FEM route restored bulk constructor {forbidden}");
    }
}

#[test]
fn mounted_region_generated_element_identity_collision_is_refused() {
    use semio_s_fem_material::planar::FemMaterialValue;
    use semio_s_fem_node::planar::FemNodeValue;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection;
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let collision = &fixture["generatedIdentityCollision"];
    let doc = Fem2dSnapshot {
        nodes: vec![FemNodeValue { id: "a".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "b".into(), x: 1.0, y: 0.0 }],
        elements: vec![FemElement::Bar { id: collision["authoredElementId"].as_str().unwrap().into(), start: "a".into(), end: "b".into(), material_id: "m".into(), section_id: "s".into() }],
        materials: vec![FemMaterialValue { id: "m".into(), name: "M".into(), e: 10e9, nu: 0.2, rho: 1000.0 }],
        sections: vec![FemSection { id: "s".into(), name: "S".into(), area: 0.01, iy: 1e-5 }],
        regions: vec![FemRegion { id: collision["regionId"].as_str().unwrap().into(), name: "Area".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], holes: vec![], thickness: 0.1, material_id: "m".into(), mesh_size: 2.0 }],
        ..Default::default()
    };
    let mesh = TriMesh2 { points: doc.regions[0].outline.clone(), tris: vec![[0, 1, 2]] };
    let mut builder = MountedModelBuild::new(Some(mesh));
    let refusal = (0..10_000).find_map(|_| match builder.step_one(&doc) { Err(detail) => Some(detail), Ok(true) => panic!("colliding numerical elements were accepted"), Ok(false) => None }).expect("bounded refusal");
    assert_eq!(String::from_utf8(refusal).unwrap(), collision["diagnostic"].as_str().unwrap());
    assert_eq!(builder.model.elements_len(), 1);
    println!("[DEBUG] Generated region element identity collision refused before numerical insertion");
}
