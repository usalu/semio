fn prepare_test(render:AppRenderOperationContext,snapshot:&Fem2dSnapshot)->bool{let windows=semio_plugin::WindowConfigOwnerRegistry::new(protocol::ActorId("fem2d-preparation-test".into()));let steps=128+snapshot.nodes.len()*64+snapshot.elements.len()*64+snapshot.regions.iter().map(|region|(region.outline.len()+region.holes.iter().map(Vec::len).sum::<usize>())*32+128).sum::<usize>()+snapshot.load_cases.len()*128+snapshot.combinations.len()*128;for _ in 0..steps{let step=prepare_snapshot_read(render,snapshot,&windows,None,render.mounted_policy.preparation);assert!(step.progress.fits(render.mounted_policy.preparation));if step.ready{return true;}if step.progress==Default::default(){return false;}}false}
use super::*;
use semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshotRetiring, Fem2dSnapshotValue};
use semio_s_fem_payload_turns::NumericalStep;
use semio_s_fem_test_support::OriginalNumericalFixture;

fn persist(value: &Fem2dSnapshotValue) -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(Fem2dSnapshot::from_reference(value.clone()).expect("fixture loads as a persistent snapshot"))
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("fixture parses")
}

fn lifecycle_context<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut StepContext<'_>) -> R) -> R {
    let mut caller = OriginalNumericalFixture::new(mounted_test_operation());
    let result = caller.turn(grant, 64, u64::MAX, |cx| Ok(run(cx))).expect("original fixture turn");
    caller.finish();
    result
}

fn close(app: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    lifecycle_context(grant, |cx| {
        cx.with_original_retirement(|cx| close_step(app, cx, grant).map_err(Into::into))
    }).expect("lifecycle close step")
}

fn maintenance(app: u32, grant: RetainedCloneGrant) -> PluginLifecycleStep {
    lifecycle_context(grant, |cx| maintenance_step(app, cx, grant)).expect("lifecycle maintenance step")
}

/// 📚️ The session module as the unsplit file held it: the root, then every mounted child in declaration order.
fn session_source() -> String {
    [
        include_str!("../../🦀️.rs"),
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
    let fixture = fixture();
    for source in fixture["sources"].as_array().unwrap() {
        let kind = source["kind"].as_str().unwrap();
        let id = source["id"].as_str().unwrap();
        let source = semio_s_fem_analyses_model::ResultSource::from_parts(kind, id.to_owned()).unwrap();
        let wire = semio_s_fem_analyses_model::encode_result_source_identity(&source).unwrap();
        assert_eq!(semio_s_fem_analyses_model::decode_result_source_identity(&wire).unwrap(), source);
    }
    let source = session_source();
    assert!(source.contains("PcgJobConstruction::new_with_mounted_rhs"));
    assert!(source.contains("AssemblyCsrBuild::new_free"));
    assert!(!source.contains("snapshot.regions.first()"));
    assert!(source.contains("visual_equation_indices(&node.id"));
}

fn mounted_test_operation() -> Operation {
    Operation::new(OperationId(791), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 4)
}

fn drain_mounted_job(job: &mut impl NumericalStep) -> OriginalNumericalFixture {
    let mut caller = OriginalNumericalFixture::new(mounted_test_operation());
    for _ in 0..100_000 {
        match caller.turn(drain_grant(), 64, u64::MAX, |cx| child_end(job.step_plain(cx))).expect("original numerical step") {
            ChildEnd::Complete(_) => return caller,
            ChildEnd::Fault(detail) => panic!("mounted job fault: {:?}", detail.map(|detail| String::from_utf8_lossy(&detail).into_owned())),
            ChildEnd::Cancelled => panic!("unexpected cancellation"),
            _ => {}
        }
    }
    panic!("mounted job did not complete its bounded opportunities");
}

fn drain_mounted_assembly(assembly: &mut AssemblyJob<'static>) {
    let mut caller = drain_mounted_job(assembly);
    caller.turn(drain_grant(), 64, u64::MAX, |cx| {
        assembly.retire_outcome(cx);
        Ok(())
    }).expect("return the original assembly outcome");
    caller.finish();
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
    let value = Fem2dSnapshotValue {
        nodes: (0..count).map(|index| FemNodeValue { id: format!("n{index}"), x: 2.0 * index as f64 / (count - 1) as f64, y: 0.0 }).collect(),
        elements: (1..count).map(|index| FemElement::Bar { id: format!("e{index}"), start: format!("n{}", index - 1), end: format!("n{index}"), material_id: "steel".into(), section_id: "section".into() }).collect(),
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "section".into(), name: "Section".into(), area: 0.01, iy: 0.00001 }],
        supports: (0..count).map(|index| FemSupport { id: format!("f{index}"), node_id: format!("n{index}"), fixed: if index == 0 { vec![FemDof::Tx, FemDof::Ty] } else { vec![FemDof::Ty] } }).collect(),
        load_cases: vec![FemLoadCase { id: "tip".into(), name: "Tip".into(), loads: vec![FemLoad::Nodal { id: "force".into(), node_id: format!("n{}", count - 1), dof: FemDof::Tx, value: 1000.0 }], self_weight: false }],
        ..Default::default()
    };
    let doc = persist(&value);
    let mut builder = MountedModelBuild::new(None);
    assert!((0..100_000).any(|_| builder.step_one(&doc).unwrap()));
    let model = builder.take_complete().unwrap();
    assert_eq!(model.nodes_len(), count);
    assert_eq!(model.elements_len(), count - 1);
    let mut construction = AssemblyJobConstruction::new_mounted(model, mounted_test_operation(), 1);
    assert!((0..100_000).any(|_| construction.step_one().unwrap()));
    let mut assembly = construction.take_complete().unwrap();
    drain_mounted_assembly(&mut assembly);
    let mut loads = MountedLoadBuild::new(job_bridge_and_live_mount::resolved_source(&doc,Some(semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:"tip"})).unwrap());
    assert!((0..100_000).any(|_| lifecycle_context(drain_grant(), |cx| loads.step_one(&doc, &assembly, cx)).unwrap()));
    let mut csr = AssemblyCsrBuild::new_free(assembly).ok().unwrap();
    assert!((0..100_000).any(|_| csr.step_one_funded(drain_grant()).unwrap().0));
    let mut construction = PcgJobConstruction::new_with_mounted_rhs(mounted_test_operation(), csr.take_complete(drain_grant()).unwrap().0, std::mem::take(&mut loads.free)).ok().unwrap();
    assert!((0..100_000).any(|_| construction.step_one().unwrap()));
    let mut pcg = construction.take_complete().unwrap();
    let mut caller = drain_mounted_job(&mut pcg);
    let free = csr.visual_equation_indices("n49", semio_s_fem_model::Dof::Tx).unwrap().1.unwrap();
    let actual = pcg.visual_scalar(free).unwrap().displacement;
    let fixture = fixture();
    let expected = fixture["axialChain"]["tipX"].as_f64().unwrap();
    assert!((actual - expected).abs() < 1e-12);
    assert!(pcg.visual_progress().4);
    caller.close(&mut pcg);
    caller.finish();
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
    let value = Fem2dSnapshotValue {
        nodes: vec![FemNodeValue { id: "root".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "tip".into(), x: 2.0, y: 0.0 }],
        elements: vec![FemElement::Beam { id: "beam".into(), start: "root".into(), end: "tip".into(), material_id: "steel".into(), section_id: "section".into() }],
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "section".into(), name: "Section".into(), area: 0.01, iy: 0.00001 }],
        supports: vec![FemSupport { id: "fixed".into(), node_id: "root".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Rz] }],
        load_cases: vec![FemLoadCase { id: "empty".into(), name: "Empty".into(), loads: vec![], self_weight: false }, FemLoadCase { id: "tip".into(), name: "Tip".into(), loads: vec![FemLoad::Nodal { id: "force".into(), node_id: "tip".into(), dof: FemDof::Ty, value: -1000.0 }], self_weight: false }],
        combinations: vec![FemCombination { id: "tip".into(), name: "Reverse".into(), terms: vec![FemCombinationTerm { case_id: "tip".into(), factor: -2.0 }] }],
        ..Default::default()
    };
    let fixture = fixture();
    for reversed in [false, true] {
        let mut value = value.clone();
        if reversed { value.load_cases.reverse(); }
        let doc = persist(&value);
        for row in fixture["sources"].as_array().unwrap() {
            let source = ResultSource::from_parts(row["kind"].as_str().unwrap(), row["id"].as_str().unwrap().to_owned()).unwrap();
            let mut builder = MountedModelBuild::new(None);
            assert!((0..10_000).any(|_| builder.step_one(&doc).unwrap()));
            let mut construction = AssemblyJobConstruction::new_mounted(builder.take_complete().unwrap(), mounted_test_operation(), 1);
            assert!((0..100_000).any(|_| construction.step_one().unwrap()));
            let mut assembly = construction.take_complete().unwrap();
            drain_mounted_assembly(&mut assembly);
            let tip_y = assembly.visual_equation_indices("tip", semio_s_fem_model::Dof::Ty).unwrap().1.unwrap();
            let tip_rz = assembly.visual_equation_indices("tip", semio_s_fem_model::Dof::Rz).unwrap().1.unwrap();
            let mut loads = MountedLoadBuild::new(job_bridge_and_live_mount::resolved_source(&doc,Some(semio_s_fem_analyses_model::ResultSourceIdentity{kind:source.kind(),id:source.id()})).unwrap());
            assert!((0..100_000).any(|_| lifecycle_context(drain_grant(), |cx| loads.step_one(&doc, &assembly, cx)).unwrap()));
            let mut csr = AssemblyCsrBuild::new_free(assembly).ok().unwrap();
            assert!((0..100_000).any(|_| csr.step_one_funded(drain_grant()).unwrap().0));
            let mut construction = PcgJobConstruction::new_with_mounted_rhs(mounted_test_operation(), csr.take_complete(drain_grant()).unwrap().0, std::mem::take(&mut loads.free)).ok().unwrap();
            assert!((0..100_000).any(|_| construction.step_one().unwrap()));
            let mut pcg = construction.take_complete().unwrap();
            let mut caller = drain_mounted_job(&mut pcg);
            let y = pcg.visual_scalar(tip_y).unwrap().displacement;
            let rz = pcg.visual_scalar(tip_rz).unwrap().displacement;
            assert!((y - row["tipY"].as_f64().unwrap()).abs() < 1e-12);
            assert!((rz - row["tipRz"].as_f64().unwrap()).abs() < 1e-12);
            assert!(pcg.visual_progress().4);
            caller.close(&mut pcg);
            caller.finish();
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
    let fixture = fixture();
    let regions = vec![
        FemRegion { id: "a".into(), name: "A".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], holes: vec![], thickness: 0.1, material_id: "a".into(), mesh_size: 2.0 },
        FemRegion { id: "b".into(), name: "B".into(), outline: vec![[1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], thickness: 0.2, material_id: "b".into(), mesh_size: 2.0 },
    ];
    let materials = vec![FemMaterialValue { id: "a".into(), name: "A".into(), e: 10e9, nu: 0.2, rho: 1000.0 }, FemMaterialValue { id: "b".into(), name: "B".into(), e: 20e9, nu: 0.3, rho: 2000.0 }];
    for reversed in [false, true] {
        let mut value = Fem2dSnapshotValue {
            nodes: vec![FemNodeValue { id: "root".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "edge".into(), x: 1.0, y: 0.0 }],
            regions: regions.clone(), materials: materials.clone(),
            supports: vec![FemSupport { id: "f0".into(), node_id: "root".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "f1".into(), node_id: "edge".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "area".into(), name: "Area".into(), loads: vec![FemLoad::Area { id: "a".into(), region_id: "a".into(), pressure: fixture["regions"]["areaLoads"][0].as_f64().unwrap() }, FemLoad::Area { id: "b".into(), region_id: "b".into(), pressure: fixture["regions"]["areaLoads"][1].as_f64().unwrap() }], self_weight: false }],
            ..Default::default()
        };
        if reversed { value.regions.reverse(); }
        let doc = persist(&value);
        let mesh = |index: usize| TriMesh2 { points: value.regions[index].outline.clone(), tris: vec![[0, 1, 2]] };
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
            assert_eq!(element.thickness, value.regions[index].thickness);
            assert_eq!(element.e, value.materials.iter().find(|material| material.id == value.regions[index].material_id).unwrap().e);
        }
        let free_nodes: Vec<_> = fixture["regions"]["freeCoordinates"].as_array().unwrap().iter().map(|coordinate| {
            (0..model.nodes_len()).filter_map(|index| model.node(index)).find(|node| node.pos[..2] == [coordinate[0].as_f64().unwrap(), coordinate[1].as_f64().unwrap()]).unwrap().id.clone()
        }).collect();
        let mut construction = AssemblyJobConstruction::new_mounted(model, mounted_test_operation(), 1);
        assert!((0..100_000).any(|_| construction.step_one().unwrap()));
        let mut assembly = construction.take_complete().unwrap();
        drain_mounted_assembly(&mut assembly);
        let mut loads = MountedLoadBuild::new(job_bridge_and_live_mount::resolved_source(&doc,Some(semio_s_fem_analyses_model::ResultSourceIdentity{kind:"loadCase",id:"area"})).unwrap());
        assert!((0..100_000).any(|_| lifecycle_context(drain_grant(), |cx| loads.step_one(&doc, &assembly, cx)).unwrap()));
        let mut csr = AssemblyCsrBuild::new_free(assembly).ok().unwrap();
        assert!((0..100_000).any(|_| csr.step_one_funded(drain_grant()).unwrap().0));
        let mut construction = PcgJobConstruction::new_with_mounted_rhs(mounted_test_operation(), csr.take_complete(drain_grant()).unwrap().0, std::mem::take(&mut loads.free)).ok().unwrap();
        assert!((0..100_000).any(|_| construction.step_one().unwrap()));
        let mut pcg = construction.take_complete().unwrap();
        let mut caller = drain_mounted_job(&mut pcg);
        for (node_index, node) in free_nodes.iter().enumerate() {
            for (axis, dof) in [semio_s_fem_model::Dof::Tx, semio_s_fem_model::Dof::Ty].into_iter().enumerate() {
                let index = csr.visual_equation_indices(node, dof).unwrap().1.unwrap();
                let actual = pcg.visual_scalar(index).unwrap().displacement;
                let expected = fixture["regions"]["freeDisplacements"][node_index][axis].as_f64().unwrap();
                assert!((actual - expected).abs() < 1e-14);
            }
        }
        caller.close(&mut pcg);
        caller.finish();
    }
}

const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: SESSION_CLOSE_COPY_BYTES, maximum_capacity_bytes: 0, maximum_release_bytes: SESSION_CLOSE_RELEASE_BYTES, maximum_depth: SESSION_CLOSE_DEPTH };

/// 🎫️ Independent app instances keep exact pending leases through sibling close and slot reuse.
#[test]
fn mounted_app_admission_isolates_colliding_ids_until_original_children_return() {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 };
    let render = |app_instance_id| AppRenderOperationContext { app_instance_id, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32], mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
    let snapshot = persist(&Fem2dSnapshotValue::default());
    for app in [1, 33] { let mut ready=false;for _ in 0..128{if prepare_test(render(app), &snapshot){ready=true;break;}}assert!(ready); }
    let (first, second, retained) = MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let first = registry.app_slots.lookup(1).unwrap();
        let second = registry.app_slots.lookup(33).unwrap();
        (first, second, registry.pending[second.index()].unwrap())
    });
    assert_ne!(first.index(), second.index());
    assert!(matches!(close(1, RetainedCloneGrant::default()), PluginLifecycleStep::Progress(progress) if progress == Default::default()));
    for _ in 0..16 { if terminal_is_empty(1) { break } close(1, grant); }
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
    for app in [33, 65] { for _ in 0..16 { if terminal_is_empty(app) { break } close(app, grant); } assert!(terminal_is_empty(app)); }
    MOUNTED.with(|registry| { let registry = registry.borrow(); assert!(registry.app_slots.is_empty()); assert_eq!(registry.reserved_bytes, 0); assert_eq!(registry.reserved_items, 0); });
}

/// 🧵️ A genuine Store snapshot lease prevents registry release while its mounted child is borrowed.
#[semio_framework_async_macros::async_test]
async fn mounted_app_close_keeps_borrowed_original_child_and_full_digest_authority() {
    use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
    let snapshot = persist(&Fem2dSnapshotValue::default());
    let envelope = store::os_store::document_materialize::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA,
        "mounted-child-affinity",
        Fem2dSnapshot::default(),
        None,
        &ArtifactPackGenesisProducer,
    );
    let mut document = store::ArtifactStore::<Fem2dSnapshot, Fem2dMutation>::new::<ArtifactPackGenesisProducer>(
        envelope,
        protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),
    ).await.unwrap();
    document.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Fem2dSnapshot, Fem2dMutation>().unwrap()).unwrap_or_else(|(error, _)| panic!("original owners refused: {error}"));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 };
    let render = AppRenderOperationContext { app_instance_id: 101, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(document.generation()), canonical_base_revision: document.content_revision(), mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
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
    assert!((0..16).any(|_| matches!(close(render.app_instance_id, grant), PluginLifecycleStep::Blocked { .. })), "request metadata may retire first, while the original borrowed child must block the key release");
    assert!(!terminal_is_empty(render.app_instance_id));
    assert!(MOUNTED.with(|registry| registry.borrow().app_slots.lookup(render.app_instance_id).is_some()));
    drop(borrowed);
    let mut disposer = semio_plugin::bounded_document_store_disposer::<Fem2dSnapshot, Fem2dMutation>();
    for _ in 0..4096 {
        if !terminal_is_empty(render.app_instance_id) {
            match close(render.app_instance_id, grant) {
                PluginLifecycleStep::AwaitingInput { .. } => {}
                PluginLifecycleStep::Blocked { reason } => panic!("mounted child blocked after restoring its borrow: {reason}"),
                _ => continue,
            }
        }
        if disposer.terminal_is_empty(&document) { break; }
        let demand = disposer.retirement_demands(&document, 1 << 20).unwrap();
        let funded = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(1 << 20), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        lifecycle_context(funded, |cx| {
            semio_plugin::app::original_lifecycle_effect_step(cx, funded, demand, |_, effect| {
                let step = disposer.close_step(&mut document, effect)?;
                Ok((step, disposer.terminal_is_empty(&document)))
            })
        }).unwrap();
    }
    assert!(terminal_is_empty(render.app_instance_id));
    assert!(disposer.terminal_is_empty(&document));
}

/// 🎬️ The authored demo publishes a converged static Live Load reaction after mounted meshing and PCG.
#[semio_framework_async_macros::async_test]
async fn mounted_demo_static_publishes_live_load_reaction() {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA;
    use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;

    let fixture = fixture();
    let law = &fixture["demoStatic"];
    let source = semio_s_fem_analyses_model::ResultSource::from_parts(
        law["kind"].as_str().unwrap(), law["id"].as_str().unwrap().to_owned(),
    ).expect("typed Live Load identity");
    let expected_node = law["supportNode"].as_str().unwrap();
    let expected = law["reactionTy"].as_f64().unwrap();
    let tolerance = law["absoluteTolerance"].as_f64().unwrap();
    let demo = Fem2dSnapshotRetiring::new(default_fem2d_snapshot().expect("bundled demo loads"));
    let source = job_bridge_and_live_mount::resolved_source(&demo, Some(
        semio_s_fem_analyses_model::ResultSourceIdentity { kind: source.kind(), id: source.id() },
    )).expect("authored Live Load");
    let envelope = store::os_store::document_materialize::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(
        FEM_2D_SCHEMA, "mounted-demo-static", default_fem2d_snapshot().expect("demo genesis"), None, &ArtifactPackGenesisProducer,
    );
    let mut document = store::ArtifactStore::<Fem2dSnapshot, Fem2dMutation>::new::<ArtifactPackGenesisProducer>(
        envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),
    ).await.expect("original demo store");
    document.install_document_store_owners_exact(
        store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Fem2dSnapshot, Fem2dMutation>().unwrap(),
    ).unwrap_or_else(|(error, _)| panic!("original owners refused: {error}"));
    let grant = drain_grant();
    let lifecycle_grant = RetainedCloneGrant { maximum_items: 1, ..grant };
    let app = 102;
    let render = AppRenderOperationContext {
        app_instance_id: app,
        base_revision: semio_framework_job::RevisionId(17),
        generation: semio_framework_job::Generation(document.generation()),
        canonical_base_revision: document.content_revision(),
        mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: lifecycle_grant, maintenance: lifecycle_grant, close: lifecycle_grant },
    };
    assert!((0..16).any(|_| prepare_test(render, &demo)), "the original demo snapshot must be admitted");
    let read = document.snapshot_read().expect("original demo snapshot lease");
    let (owner, identity) = MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let app_slot = registry.app_slots.lookup(app).unwrap().index();
        let slot = registry.snapshot_owners[app_slot].unwrap().index();
        let pending = registry.pending[slot].take().unwrap();
        let owner = registry.shells[pending.shell as usize].clone();
        let mut state = MountedState::new(pending.identity, read, pending.admitted_items);
        state.source = source;
        *owner.borrow_mut() = Some(state);
        registry.current[slot] = Some(CurrentSession { app_instance_id: app, shell: pending.shell, identity: pending.identity, previous_valid: None });
        (owner, pending.identity)
    });
    let mut original = OriginalNumericalFixture::new(identity.operation(OperationId(identity.operation.0 ^ 0x5eed), semio_framework_job::Generation(identity.generation.0 + 3)));
    let (stage, validated, published, reaction, stages, mut last, fault) = {
        let mut mounted = owner.borrow_mut();
        let state = mounted.as_mut().expect("admitted mounted owner");
        let mut stages = vec![state.stage];
        let mut last = None;
        let mut fault = None;
        for _ in 0..250_000 {
            let turn = original.turn(grant, 128, u64::MAX, |cx| state.step(cx));
            if stages.last() != Some(&state.stage) { stages.push(state.stage); }
            match turn {
                Ok(JobStep::Done(_)) => break,
                Ok(JobStep::Running(Some(progress))) => last = Some(progress),
                Ok(JobStep::Failed(detail)) => {
                    fault = Some(String::from_utf8_lossy(&detail).into_owned());
                    break;
                }
                Err(error) => {
                    fault = Some(format!("{error:?}"));
                    break;
                }
                _ => {}
            }
        }
        let reaction = state.visual.fields.iter().find(|field| field.node_id == expected_node).map(|field| field.reaction[1]);
        (state.stage, state.visual.validated_final, state.visual_current.is_some(), reaction, stages, last, fault)
    };
    let mut disposer = semio_plugin::bounded_document_store_disposer::<Fem2dSnapshot, Fem2dMutation>();
    let mut last_close = None;
    let mut last_store_close = None;
    for _ in 0..32_768 {
        if !terminal_is_empty(app) {
            let step = original.turn(lifecycle_grant, 128, u64::MAX, |cx| {
                cx.with_original_retirement(|cx| close_step(app, cx, lifecycle_grant).map_err(Into::into))
            }).expect("demo mounted close turn");
            let awaiting_store = matches!(step, PluginLifecycleStep::AwaitingInput { .. });
            last_close = Some(step);
            if !awaiting_store { continue; }
        }
        if disposer.terminal_is_empty(&document) { break; }
        let demand = disposer.retirement_demands(&document, lifecycle_grant.maximum_copy_bytes).expect("demo store close demand");
        let step = original.turn(lifecycle_grant, 128, u64::MAX, |cx| {
            Ok(semio_plugin::app::original_lifecycle_effect_step(cx, lifecycle_grant, demand, |_, effect| {
                let step = disposer.close_step(&mut document, effect)?;
                Ok((step, disposer.terminal_is_empty(&document)))
            }).expect("demo store retires"))
        }).expect("demo store close turn");
        last_store_close = Some((demand, step));
    }
    let app_empty = terminal_is_empty(app);
    let store_empty = disposer.terminal_is_empty(&document);
    assert!(app_empty && store_empty,
        "demo owners must retire after {stage:?}, fault {fault:?}, final validated {validated}, published {published}, {expected_node} Ty {reaction:?} N, mounted terminal {app_empty}, last close {last_close:?}, mounted close (cursor, mesh, model) {:?}, store terminal {store_empty}, phase {}, displaced {}, last Store close {last_store_close:?}",
        owner.borrow().as_ref().map(|state| (state.close_cursor, state.mesh.is_some(), state.model_build.is_some())),
        document.close_owned_phase_witness(),
        document.close_displaced_witness(),
    );
    original.finish();
    if let Some(progress) = last.as_mut() { progress.truncate(160); }
    let last = last.as_deref().map(String::from_utf8_lossy);
    assert_eq!(stage, MountedStage::Complete, "demo stages {stages:?}, last progress {last:?}, fault {fault:?}");
    assert!(stages.contains(&MountedStage::Mesh) && stages.contains(&MountedStage::CommitReady));
    assert!(validated && published, "demo final result must replace the preview");
    assert!(reaction.is_some_and(|actual| (actual - expected).abs() <= tolerance), "{expected_node} Ty: {reaction:?}, expected {expected} N");
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
    assert_eq!(maintenance(u32::MAX, TEST_GRANT), PluginLifecycleStep::Complete(Default::default()));
    assert!(terminal_is_empty(u32::MAX));
}

#[test]
fn mounted_idle_maintenance_completes_without_releasing_the_live_app() {
    let fixture = fixture();
    let expected = &fixture["idleMaintenance"];
    let app = 2_000_000_008;
    let lease = MOUNTED.with(|registry| registry.borrow_mut().app_slots.claim(app).unwrap());
    for _ in 0..expected["opportunities"].as_u64().unwrap() {
        assert_eq!(maintenance_pending(app), expected["livePending"].as_bool().unwrap());
        assert_eq!(terminal_is_empty(app), expected["liveTerminal"].as_bool().unwrap());
        assert_eq!(maintenance_demands(app).unwrap(), RetirementDemand::default());
        let outcome = match maintenance(app, TEST_GRANT) {
            PluginLifecycleStep::Complete(progress) if progress == Default::default() => "complete",
            actual => panic!("unexpected live maintenance step: {actual:?}"),
        };
        assert_eq!(outcome, expected["liveOutcome"].as_str().unwrap());
        assert_eq!(MOUNTED.with(|registry| registry.borrow().app_slots.lookup(app)), Some(lease));
    }
    close(app, TEST_GRANT);
    assert!(terminal_is_empty(app));
    let outcome = match maintenance(app, TEST_GRANT) {
        PluginLifecycleStep::Complete(progress) if progress == Default::default() => "complete",
        actual => panic!("unexpected terminal maintenance step: {actual:?}"),
    };
    assert_eq!(outcome, expected["closedOutcome"].as_str().unwrap());
}

#[test]
fn snapshot_lease_is_preceded_by_a_fixed_pending_admission_and_idle_polling_reuses_it() {
    let render = AppRenderOperationContext { app_instance_id: 2_000_000_007, base_revision: semio_framework_job::RevisionId(17), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32], mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, maintenance: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, close: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 } } };
    let snapshot = persist(&Fem2dSnapshotValue::default());
    assert!((0..64).any(|_| prepare_test(render, &snapshot)), "empty schema census completes incrementally");
    let first = MOUNTED.with(|registry| { let registry = registry.borrow(); registry.pending[registry.app_slots.lookup(render.app_instance_id).unwrap().index()].expect("pending admission") });
    for _ in 0..1_025 {
        assert!(prepare_test(render, &snapshot));
    }
    let retained = MOUNTED.with(|registry| { let registry = registry.borrow(); registry.pending[registry.app_slots.lookup(render.app_instance_id).unwrap().index()].expect("same pending admission") });
    assert_eq!(first.shell, retained.shell);
    assert_eq!(first.identity, retained.identity);
    assert!(matches!(close(render.app_instance_id, TEST_GRANT), PluginLifecycleStep::Progress(progress) if progress.copied_items == 1));
    for _ in 0..8 { if terminal_is_empty(render.app_instance_id) { break } close(render.app_instance_id, TEST_GRANT); }
    assert!(terminal_is_empty(render.app_instance_id));
}

#[test]
fn snapshot_census_completes_within_one_opportunity_and_rejects_exact_plus_one_without_partial_credit() {
    let render = AppRenderOperationContext { app_instance_id: 2_000_000_006, base_revision: semio_framework_job::RevisionId(29), generation: semio_framework_job::Generation(31), canonical_base_revision: [37; 32], mounted_policy: semio_plugin::MountedOwnerPolicyV1 { preparation: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, maintenance: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 }, close: semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 } } };
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("bundled demo loads"));
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
    assert!(matches!(close(render.app_instance_id, TEST_GRANT), PluginLifecycleStep::Progress(progress) if progress.copied_items == 1));
    for _ in 0..8 { if terminal_is_empty(render.app_instance_id) { break } close(render.app_instance_id, TEST_GRANT); }
    assert!(terminal_is_empty(render.app_instance_id));
}

#[test]
fn process_owner_inventory_admits_exact_maximum_and_returns_exact_credit() {
    let catalog = MountedProcessOwnerCatalog::fixed();
    let mut seen = [false; MountedProcessOwnerCatalog::fixed().claims.len()];
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
    let fixture = fixture();
    let collision = &fixture["generatedIdentityCollision"];
    let value = Fem2dSnapshotValue {
        nodes: vec![FemNodeValue { id: "a".into(), x: 0.0, y: 0.0 }, FemNodeValue { id: "b".into(), x: 1.0, y: 0.0 }],
        elements: vec![FemElement::Bar { id: collision["authoredElementId"].as_str().unwrap().into(), start: "a".into(), end: "b".into(), material_id: "m".into(), section_id: "s".into() }],
        materials: vec![FemMaterialValue { id: "m".into(), name: "M".into(), e: 10e9, nu: 0.2, rho: 1000.0 }],
        sections: vec![FemSection { id: "s".into(), name: "S".into(), area: 0.01, iy: 1e-5 }],
        regions: vec![FemRegion { id: collision["regionId"].as_str().unwrap().into(), name: "Area".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], holes: vec![], thickness: 0.1, material_id: "m".into(), mesh_size: 2.0 }],
        ..Default::default()
    };
    let doc = persist(&value);
    let mesh = TriMesh2 { points: value.regions[0].outline.clone(), tris: vec![[0, 1, 2]] };
    let mut builder = MountedModelBuild::new(Some(mesh));
    let refusal = (0..10_000).find_map(|_| match builder.step_one(&doc) { Err(detail) => Some(detail), Ok(true) => panic!("colliding numerical elements were accepted"), Ok(false) => None }).expect("bounded refusal");
    assert_eq!(String::from_utf8(refusal).unwrap(), collision["diagnostic"].as_str().unwrap());
    assert_eq!(builder.model.elements_len(), 1);
}

fn drain_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 4096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 4 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 4096 }
}

#[test]
fn shell_credit_checks_actual_reserved_bytes_on_each_target() {
    let law = &fixture()["shellCredit"];
    let mut registry = MountedRegistry::new();
    let expected = law[if usize::BITS == 32 { "admitted32" } else { "admitted64" }].as_array().unwrap();
    assert_eq!(expected.len(), law["slots"].as_u64().unwrap() as usize);
    for (shell, row) in expected.iter().enumerate() {
        let admitted = registry.reserve_credit(shell as u16, 1, SESSION_MAXIMUM_BYTES);
        assert_eq!(admitted, row.as_bool().unwrap());
        assert_eq!(registry.credit_bytes[shell], if admitted { SESSION_MAXIMUM_BYTES } else { 0 });
    }
    registry.release_credit(0);
    assert!(registry.reserve_credit(0, 1, SESSION_MAXIMUM_BYTES));
}

/// 🕸️ The mesh edge index vectors keep exactly three fixed backing roots in the process inventory, sized by the declared triangle maximum; losing a root changes the exact process credit.
#[test]
fn mesh_edge_index_vectors_claim_three_fixed_roots_and_the_exact_credit_depends_on_them() {
    let catalog = MountedProcessOwnerCatalog::fixed();
    let claim = catalog.claims[MountedOwnerClass::MeshEdgeIndexVectors as usize];
    assert_eq!(claim.roots, 3);
    assert_eq!(claim.items, 3 * SESSION_MAXIMUM_MESH_TRIANGLES + 2 * (12 * SESSION_MAXIMUM_MESH_TRIANGLES + 3));
    let exact = catalog.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES).expect("the enumerated working set exactly fits its process authority");
    let mut lost = catalog;
    lost.claims[MountedOwnerClass::MeshEdgeIndexVectors as usize].roots -= 1;
    let credit = lost.credit(SESSION_MAXIMUM_INPUT_ITEMS, SESSION_MAXIMUM_INPUT_BYTES).expect("fewer roots still fit");
    assert_ne!(credit, exact, "the credit returned to the process authority follows the claimed roots");
}
