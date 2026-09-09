use super::*;

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
        machines: vec![WorkshopMachine {
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
        }],
    };
    snapshot.stock_payload.solid = WorkingSolid::ImportedMesh { mesh_url: "mesh.glb".into() };
    snapshot.step_payloads = vec![
        ProcessStep {
            id: "box".into(),
            label: "Box".into(),
            enabled: true,
            origin: Some(StepOrigin { machine_id: "machine".into(), capability_id: "disc".into() }),
            measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 1.0, depth: 2.0, height: 3.0 }, pose: Pose::default() },
        },
        ProcessStep { id: "cylinder".into(), label: "Cylinder".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Cylinder { radius: 1.0, height: 2.0 }, pose: Pose::default() } },
        ProcessStep { id: "sphere".into(), label: "Sphere".into(), enabled: true, origin: None, measure: ProcessMeasure::Attach { component: WorkingSolid::Sphere { radius: 1.0 }, pose: Pose::default() } },
        ProcessStep { id: "solid".into(), label: "Solid".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: WorkingSolid::ImportedSolid { solid_handle: "solid-1".into() }, pose: Pose::default() } },
        ProcessStep { id: "drill".into(), label: "Drill".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 0.4, pose: Pose::default() } },
    ];
    snapshot.tool_solids.push(snapshot.stock_solid.clone());
    snapshot.resolved_up_to = Some(4);
    snapshot
}

fn retained_pack(snapshot: &Process3dSnapshot) -> Vec<u8> {
    const TOKEN: &[u8] = b"process.process3d.pack v1";
    let raw = encode_process3d_snapshot_binary(snapshot);
    let mut bytes = Vec::with_capacity(12 + TOKEN.len() + raw.len());
    bytes.extend_from_slice(&store::semio_format::BINARY_MAGIC);
    bytes.extend_from_slice(&(TOKEN.len() as u32).to_le_bytes());
    bytes.extend_from_slice(TOKEN);
    bytes.extend_from_slice(&raw);
    bytes
}

#[test]
fn retained_string_cursor_enforces_one_byte_grants_and_hostile_boundaries() {
    fn grant(cursor: &mut Process3dRetainedStringCursor, byte: u8) -> Result<Option<String>, String> {
        let mut reader = store::ByteReader::new(std::slice::from_ref(&byte));
        let value = cursor.step(&mut reader);
        assert_eq!(reader.position(), 1, "one retained string grant consumes one byte opportunity");
        value
    }

    let mut exact = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert_eq!(grant(&mut exact, 3).expect("exact length admission"), None);
    assert_eq!(grant(&mut exact, b'a').expect("exact byte one"), None);
    assert_eq!(grant(&mut exact, b'b').expect("exact byte two"), None);
    assert_eq!(grant(&mut exact, b'c').expect("exact byte three"), Some("abc".into()));
    assert!(exact.terminal_is_empty());

    let mut plus_one = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert!(grant(&mut plus_one, 4).expect_err("maximum plus one must fail before producer copy").contains("fixed byte credit"));
    assert_eq!(plus_one.take_partial(), "", "maximum plus one returns its empty pre-copy owner");
    assert!(plus_one.terminal_is_empty());

    let mut malformed = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert_eq!(grant(&mut malformed, 1).expect("malformed length admission"), None);
    assert!(grant(&mut malformed, 0xff).expect_err("malformed UTF-8 must fail at its byte boundary").contains("utf-8"));
    assert!(malformed.terminal_is_empty());

    let mut truncated = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert_eq!(grant(&mut truncated, 2).expect("truncated length admission"), None);
    assert_eq!(grant(&mut truncated, b'x').expect("truncated first byte"), None);
    assert!(truncated.step(&mut store::ByteReader::new(&[])).is_err(), "truncation must remain a resumable read failure");
    assert_eq!(truncated.take_partial(), "x", "interrupted string bytes return through the exact handback owner");
    assert!(truncated.terminal_is_empty());

    let mut overflowing_length = Process3dRetainedStringCursor::with_maximum_bytes(3);
    for _ in 0..(usize::BITS / 7) {
        assert_eq!(grant(&mut overflowing_length, 0x80).expect("bounded length byte"), None);
    }
    assert!(grant(&mut overflowing_length, 0x80).is_err(), "overlong retained length must fail without payload allocation");
    assert_eq!(overflowing_length.take_partial(), "");
    assert!(overflowing_length.terminal_is_empty());
}

#[test]
fn mounted_snapshot_region_has_zero_whole_string_reader_edges() {
    let source = include_str!("../../🦀️.rs");
    let retained = source.split_once("//#region 🔖️RetainedStructuralReader").expect("retained snapshot region start").1.split_once("//#endregion 🔖️RetainedStructuralReader").expect("retained snapshot region end").0;
    assert_eq!(retained.matches(concat!("read_str", "_lp")).count(), 0, "mounted snapshot reader must have no whole-string edge");
}

#[test]
fn retained_snapshot_reader_yields_between_structural_fields_and_hands_off_exactly_once() {
    let expected = complete_snapshot();
    let bytes = retained_pack(&expected);
    let operation = semio_framework_job::OperationId(u64::MAX - 81);
    let generation = semio_framework_job::Generation(17);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut reader = Process3dRetainedSnapshotReader::new(8_192);
    let mut grants = 0;
    loop {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        grants += 1;
        if reader.step(&bytes, &mut context).expect("retained structural step") {
            break;
        }
        assert_eq!(context.fuel_remaining(), 0);
        assert!(grants < 8_192, "retained structural cursor must converge");
    }
    assert!(grants >= 256, "snapshot fields, string bytes, and nested items must not collapse into one whole decode grant");
    assert_eq!(reader.take(), Some(expected));
    assert!(reader.take().is_none(), "retained snapshot handoff is exact");
    assert!(reader.terminal_is_empty());
}

#[test]
fn deepest_nested_snapshot_cursor_closes_without_populated_drop() {
    let expected = complete_snapshot();
    let bytes = retained_pack(&expected);
    let operation = semio_framework_job::OperationId(u64::MAX - 82);
    let generation = semio_framework_job::Generation(18);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut reader = Process3dRetainedSnapshotReader::new(8_192);
    for _ in 0..96 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        if reader.step(&bytes, &mut context).expect("nested retained structural step") {
            break;
        }
    }
    let partial = reader.take_rejected().expect("partial snapshot handback");
    assert!(reader.terminal_is_empty());
    drop(reader);
    let mut retirement = store::SnapshotRetirementFactory::retire(&crate::spr::Process3dSnapshotRetirementFactory, std::sync::Arc::new(partial));
    for _ in 0..8_192 {
        if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
            break;
        }
    }
    assert!(retirement.terminal_is_empty());
    drop(retirement);
}

#[test]
fn every_machine_capability_parameter_and_rule_substate_interrupts_into_exact_retirement() {
    use Process3dRetainedSnapshotPhase::*;
    let phases = [
        MachineId,
        MachineLabel,
        MachineIcon,
        MachineCatalogTag,
        MachineCatalogId,
        MachineCapabilityCount,
        CapabilityId,
        CapabilityLabel,
        CapabilityIcon,
        CapabilityRecipeTag,
        CapabilityRecipeField(0),
        CapabilityRecipeField(1),
        CapabilityRecipeField(2),
        CapabilityParameterCount,
        ParameterId,
        ParameterLabel,
        ParameterValue,
        CapabilityRuleCount,
        RuleTag,
        RuleQuantity,
        RuleParameter,
        RuleMargin,
        MachineComplete,
    ];
    let bytes = retained_pack(&complete_snapshot());
    for (index, target) in phases.into_iter().enumerate() {
        let operation = semio_framework_job::OperationId(u64::MAX - 1_000 - index as u64);
        let generation = semio_framework_job::Generation(100 + index as u64);
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut preview_sequence = 0;
        let mut reader = Process3dRetainedSnapshotReader::new(8_192);
        for _ in 0..8_192 {
            if reader.phase == target {
                break;
            }
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            assert!(!reader.step(&bytes, &mut context).expect("drive to retained snapshot substate"), "target substate must occur before completion");
        }
        assert_eq!(reader.phase, target, "every catalogued nested substate must be reachable");
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
        assert!(!reader.step(&bytes, &mut context).expect("interrupt one retained snapshot substate"));
        assert_eq!(context.fuel_remaining(), 0, "one substate consumes exactly one grant");
        let partial = reader.take_rejected().expect("interrupted substate exact handback");
        assert!(reader.terminal_is_empty());
        drop(reader);
        let mut retirement = store::SnapshotRetirementFactory::retire(&crate::spr::Process3dSnapshotRetirementFactory, std::sync::Arc::new(partial));
        for _ in 0..8_192 {
            if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
                break;
            }
        }
        assert!(retirement.terminal_is_empty(), "interrupted nested substate must close incrementally to terminal-empty");
        drop(retirement);
    }
}
