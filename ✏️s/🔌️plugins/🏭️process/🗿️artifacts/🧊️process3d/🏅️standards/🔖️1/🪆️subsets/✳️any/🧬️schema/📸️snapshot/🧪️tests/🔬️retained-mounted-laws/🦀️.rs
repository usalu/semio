use super::*;
use crate::{ProcessMeasure, StepOrigin, StockQuantity};

fn capability(id: &str, recipe: MeasureRecipe) -> Capability {
    Capability {
        id: id.into(),
        label: id.into(),
        icon_id: "tool".into(),
        recipe,
        parameters: vec![
            CapabilityParameter { id: "first".into(), label: "First".into(), value: 1.0 },
            CapabilityParameter { id: "second".into(), label: "Second".into(), value: 2.0 },
            CapabilityParameter { id: "third".into(), label: "Third".into(), value: 3.0 },
        ],
        rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "first".into(), margin: 0.1 }, CapabilityRule::Max { quantity: StockQuantity::MaxDimension, parameter: "third".into(), margin: 0.2 }],
    }
}

fn complete_snapshot() -> Process3dSnapshot {
    let mut snapshot = crate::empty_process3d_snapshot();
    snapshot.workshop = Workshop {
        machines: vec![
            WorkshopMachine {
                id: "machine".into(),
                label: "Machine".into(),
                icon_id: "machine".into(),
                catalog_id: Some("catalog".into()),
                capabilities: vec![
                    capability("disc", MeasureRecipe::DiscCut { diameter: "first".into(), kerf: "second".into() }),
                    capability("blade", MeasureRecipe::BladeCut { kerf: "first".into(), length: "second".into(), depth: "third".into() }),
                    capability("pocket", MeasureRecipe::PocketCut { diameter: "first".into(), depth: "second".into() }),
                    capability("bore", MeasureRecipe::BoreDrill { radius: "first".into(), depth: "second".into() }),
                    capability("cylinder", MeasureRecipe::CylinderAttach { radius: "first".into(), length: "second".into() }),
                    capability("box", MeasureRecipe::BoxAttach { width: "first".into(), depth: "second".into(), height: "third".into() }),
                ],
            },
            WorkshopMachine { id: "bare".into(), label: "Bare".into(), icon_id: "plus".into(), catalog_id: None, capabilities: Vec::new() },
        ],
    };
    snapshot.stock_payload.solid = WorkingSolid::ImportedMesh { mesh_url: "mesh.glb".into() };
    snapshot.stock_label = "Stock ünïcode".into();
    snapshot.step_payloads = vec![
        ProcessStep {
            id: "box".into(),
            label: "Box".into(),
            enabled: true,
            origin: Some(StepOrigin { machine_id: "machine".into(), capability_id: "disc".into() }),
            measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 1.0, depth: 2.0, height: 3.0 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } },
        },
        ProcessStep { id: "cylinder".into(), label: "Cylinder".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Cylinder { radius: 1.0, height: 2.0 }, pose: Pose::default() } },
        ProcessStep { id: "sphere".into(), label: "Sphere".into(), enabled: true, origin: None, measure: ProcessMeasure::Attach { component: WorkingSolid::Sphere { radius: 1.0 }, pose: Pose::default() } },
        ProcessStep { id: "solid".into(), label: "Solid".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: WorkingSolid::ImportedSolid { solid_handle: "solid-1".into() }, pose: Pose::default() } },
        ProcessStep { id: "reference".into(), label: "Reference".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Reference { reference_id: "ref-1".into() }, pose: Pose::default() } },
        ProcessStep { id: "drill".into(), label: "Drill".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 0.4, pose: Pose::default() } },
    ];
    snapshot.tool_solids.push(snapshot.stock_solid.clone());
    snapshot.resolved_up_to = Some(4);
    snapshot
}

fn admit(session: &mut Process3dMountedPackSession, value: u8) {
    loop {
        if let Some(exact) = session.next_retained_allocation_bytes().expect("process3d source allocation query") {
            assert!(session.reserve_retained_allocation(exact).expect("process3d source allocation grant").progressed);
            continue;
        }
        session.admit_byte(value).expect("one admitted snapshot byte");
        return;
    }
}

fn ingest(bytes: &[u8]) -> Process3dMountedPackSession {
    let mut session = process3d_mounted_pack_session(bytes.len(), 8_192).expect("process3d mounted preflight");
    for byte in bytes {
        admit(&mut session, *byte);
    }
    session.seal().expect("exact snapshot seal");
    session
}

fn grant(session: &mut Process3dMountedPackSession) -> bool {
    loop {
        if let Some(exact) = session.next_retained_allocation_bytes().expect("process3d retained allocation query") {
            assert!(session.reserve_retained_allocation(exact).expect("process3d retained allocation grant").progressed);
            continue;
        }
        return session.grant().expect("one retained snapshot grant");
    }
}

fn close(session: &mut Process3dMountedPackSession) {
    session.request_cancel();
    let admitted_allocation_bytes = session.retained_allocated_bytes();
    let mut released_allocation_bytes = 0;
    for _ in 0..100_000 {
        let maximum_bytes = session.next_retained_release_allocation_bytes().unwrap_or(0);
        let step = session.close_step(1, maximum_bytes).expect("process3d retained session close");
        if let mounted::RetainedTypedPackCloseStep::Pending { released_bytes, .. } = step {
            released_allocation_bytes += released_bytes;
        }
        if step == mounted::RetainedTypedPackCloseStep::Complete {
            assert!(session.terminal_is_empty());
            assert_eq!(released_allocation_bytes, admitted_allocation_bytes);
            return;
        }
    }
    panic!("process3d retained session did not reach terminal-empty close");
}

#[test]
fn derived_pack_identity_and_text_pack_equivalence() {
    let snapshot = complete_snapshot();
    let hash = store::os_store::test_support::assert_pack_schema_identity(&snapshot);
    assert_eq!(hash, store::os_pack::schema_hash(&Process3dSnapshot::__dsl_spec()));
    store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
    store::os_store::test_support::assert_dsl_pack_equivalence(&crate::empty_process3d_snapshot());
}

#[test]
fn mounted_session_round_trips_the_derived_pack_one_grant_at_a_time() {
    let expected = complete_snapshot();
    let bytes = store::ArtifactPack::encode_pack(&expected);
    let mut session = ingest(&bytes);
    let mut grants = 0;
    while !grant(&mut session) {
        grants += 1;
        assert!(grants < 1_000_000, "process3d retained canonical route must converge");
    }
    assert!(grants >= 256, "snapshot fields, string characters and nested items must not collapse into one whole decode grant");
    assert_eq!(session.take(), Some(expected), "every typed owner must round-trip exactly");
    assert!(session.take().is_none(), "retained snapshot handoff is exact");
    close(&mut session);
}

#[test]
fn foreign_header_is_rejected_before_semantic_allocation() {
    let bytes = store::ArtifactPack::encode_pack(&complete_snapshot());
    let mut session = process3d_mounted_pack_session(bytes.len(), 8_192).expect("process3d hostile header preflight");
    admit(&mut session, bytes[0]);
    assert!(!session.semantic_allocated());
    assert_eq!(session.admit_byte(bytes[1] ^ 0xff), Err(bytes[1] ^ 0xff));
    assert!(!session.semantic_allocated());
    close(&mut session);
}

#[test]
fn every_interrupted_grant_closes_into_exact_retirement() {
    let bytes = store::ArtifactPack::encode_pack(&complete_snapshot());
    let mut total = 0;
    {
        let mut session = ingest(&bytes);
        while !grant(&mut session) {
            total += 1;
        }
        drop(session.take());
        close(&mut session);
    }
    for interrupt in (0..total).step_by(13).chain([total - 1]) {
        let mut session = ingest(&bytes);
        for _ in 0..interrupt {
            assert!(!grant(&mut session), "interrupt {interrupt} must occur before completion");
        }
        close(&mut session);
    }
}

#[test]
fn mounted_region_has_no_batch_decoder_edge() {
    let source = include_str!("../../🦀️.rs");
    let region = source.split_once("//#region 🔖️MountedTypedSnapshotOwner").expect("mounted region start").1.split_once("//#endregion 🔖️MountedTypedSnapshotOwner").expect("mounted region end").0;
    for edge in [concat!("decode_", "document"), concat!("decode_", "pack"), concat!("__dsl_", "from_record"), concat!("read_str", "_lp")] {
        assert_eq!(region.matches(edge).count(), 0, "mounted snapshot owner must not reach {edge}");
    }
}
