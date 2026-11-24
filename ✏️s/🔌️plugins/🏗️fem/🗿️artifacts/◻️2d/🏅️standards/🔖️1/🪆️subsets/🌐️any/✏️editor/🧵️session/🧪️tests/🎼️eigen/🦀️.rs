use super::*;
use semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::{
    snapshot::Fem2dSnapshotValue,
    structure::{FEM_2D_SCHEMA, FemDof, FemLoadCase, FemSection, FemSupport},
};
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_fem_material::planar::FemMaterialValue;
use semio_s_fem_node::planar::FemNodeValue;
use semio_s_fem_test_support::OriginalNumericalFixture;

const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 4096, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 4 << 20,
    maximum_release_bytes: 1 << 20, maximum_depth: 128,
};

fn cantilever(fixture: &serde_json::Value) -> Fem2dSnapshot {
    let n = |key: &str| fixture[key].as_f64().unwrap();
    let model = Fem2dSnapshotValue {
        nodes: vec![
            FemNodeValue { id: "root".into(), x: 0.0, y: 0.0 },
            FemNodeValue { id: "tip".into(), x: n("length"), y: 0.0 },
        ],
        elements: vec![FemElement::Beam {
            id: "beam".into(), start: "root".into(), end: "tip".into(),
            material_id: "steel".into(), section_id: "section".into(),
        }],
        materials: vec![FemMaterialValue {
            id: "steel".into(), name: "Steel".into(), e: n("youngsModulus"), nu: 0.3, rho: n("density"),
        }],
        sections: vec![FemSection {
            id: "section".into(), name: "Section".into(), area: n("area"), iy: n("inertia"),
        }],
        supports: vec![FemSupport {
            id: "fixed".into(), node_id: "root".into(), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Rz],
        }],
        load_cases: vec![FemLoadCase {
            id: "compression".into(), name: "Compression".into(), self_weight: false,
            loads: vec![FemLoad::Nodal {
                id: "load".into(), node_id: "tip".into(), dof: FemDof::Tx, value: -n("compression"),
            }],
        }],
        ..Default::default()
    };
    Fem2dSnapshot::from_reference(model).expect("persistent cantilever fixture")
}

/// 🎻️ The retained editor stages preserve the independent modal and buckling eigenvalues.
#[semio_framework_async_macros::async_test]
async fn mounted_eigenvalues_match_the_independent_cantilever() {
    let law: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🧪️tests/🧫️fixtures/🎼️secondary-matrix/🔣️.json"
    )).expect("cantilever eigen law");
    for (family, expected) in [(ResultMode::Modal, "modalHz"), (ResultMode::Buckling, "bucklingFactor")] {
        let envelope = store::os_store::document_materialize::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(
            FEM_2D_SCHEMA, "cantilever", cantilever(&law), None,
            &ArtifactPackGenesisProducer,
        );
        let mut document = store::ArtifactStore::<Fem2dSnapshot, Fem2dMutation>::new::<ArtifactPackGenesisProducer>(
            envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),
        ).await.expect("original cantilever store");
        document.install_document_store_owners_exact(
            store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Fem2dSnapshot, Fem2dMutation>().unwrap(),
        ).unwrap_or_else(|(error, _)| panic!("{error}"));
        let identity = MountedIdentity {
            app_instance_id: 1, base_revision: semio_framework_job::RevisionId(7),
            generation: semio_framework_job::Generation(document.generation()),
            canonical_base_revision: document.content_revision(),
            operation: OperationId(9876), job: 9876,
        };
        let read = document.snapshot_read().expect("original model snapshot");
        let mut model = MountedModelBuild::new(None);
        assert!((0..4_096).any(|_| model.step_one(&read).expect("mount cantilever model")));
        let model = model.take_complete().expect("model owner");
        let mut session = MountedState::new(identity, read, 64);
        session.family = family;
        session.source = (family == ResultMode::Buckling).then_some(ResolvedResultSource::LoadCase(0));
        session.stage = MountedStage::PrepareAssembly;
        session.visual_dirty = false;
        session.assembly_build = Some(AssemblyJobConstruction::new_mounted(model, identity.operation(identity.operation, identity.generation), 1));
        let mut original = OriginalNumericalFixture::new(identity.operation(identity.operation, identity.generation));
        for _ in 0..200_000 {
            if session.stage == MountedStage::CommitReady { break; }
            let step = original.turn(GRANT, 128, u64::MAX, |cx| session.step(cx)).expect("mounted numerical turn");
            if let JobStep::Failed(detail) = step {
                panic!("eigen run failed: {}", String::from_utf8_lossy(&detail));
            }
        }
        assert_eq!(session.stage, MountedStage::CommitReady);
        assert!(session.visual.converged && session.mode_shape_peak > 0.0);
        let eigenvalue = session.eigenvalue.expect("solved eigenvalue");
        let observed = if family == ResultMode::Modal { eigenvalue.sqrt() / std::f64::consts::TAU } else { eigenvalue };
        let expected = law[expected].as_f64().unwrap();
        assert!((observed / expected - 1.0).abs() < 1e-7, "{observed} vs {expected}");
        let mut disposer = semio_plugin::bounded_document_store_disposer::<Fem2dSnapshot, Fem2dMutation>();
        let grant = RetainedCloneGrant { maximum_items: 1, ..GRANT };
        for _ in 0..32_768 {
            if session.terminal_is_empty() && disposer.terminal_is_empty(&document) { break; }
            if !session.terminal_is_empty() {
                let mut demand = RetirementDemand {
                    copy_bytes: SESSION_CLOSE_COPY_BYTES, release_bytes: SESSION_CLOSE_RELEASE_BYTES,
                    depth: SESSION_CLOSE_DEPTH, ..Default::default()
                };
                session.child_close_demands(&mut demand).expect("session close quote");
                let step = original.turn(grant, 128, u64::MAX, |cx| {
                    Ok(semio_plugin::app::original_lifecycle_step(cx, grant, demand, |cx, grant| {
                        let step = session.close_step(cx, grant)?;
                        Ok((step, session.terminal_is_empty()))
                    }).expect("mounted eigen close"))
                }).expect("original close turn");
                if !session.terminal_is_empty() && !matches!(step, PluginLifecycleStep::AwaitingInput { .. }) {
                    continue;
                }
            }
            if !disposer.terminal_is_empty(&document) {
                let demand = disposer.retirement_demands(&document, grant.maximum_release_bytes).expect("store close demands");
                original.turn(grant, 128, u64::MAX, |cx| {
                    Ok(semio_plugin::app::original_lifecycle_effect_step(cx, grant, demand, |_, effect| {
                        let step = disposer.close_step(&mut document, effect)?;
                        Ok((step, disposer.terminal_is_empty(&document)))
                    }).expect("store retirement"))
                }).expect("original store close turn");
            }
        }
        assert!(session.terminal_is_empty());
        assert!(disposer.terminal_is_empty(&document));
        original.finish();
    }
}
