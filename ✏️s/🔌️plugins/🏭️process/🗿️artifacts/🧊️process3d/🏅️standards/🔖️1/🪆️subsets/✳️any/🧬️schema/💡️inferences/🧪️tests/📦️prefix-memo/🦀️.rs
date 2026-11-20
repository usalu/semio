use super::*;

#[test]
fn long_timeline_keeps_recent_prefixes_and_replays_evicted_stock() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("neutral memo fixture");
    let count = fixture["emptySteps"].as_u64().unwrap() as usize;
    let solid = |name: &str| {
        let [width, depth, height] = std::array::from_fn(|index| fixture[name][index].as_f64().unwrap());
        WorkingSolid::Box { width, depth, height }
    };
    let stock = Stock { id: "stock".into(), label: "Stock".into(), solid: solid("stockSize"), pose: Pose::default() };
    let mut steps = (0..count).map(|index| ProcessStep {
        id: format!("cut-{index}"),
        label: "Cut".into(),
        enabled: true,
        origin: None,
        measure: ProcessMeasure::Cut { tool: solid("cutSize"), pose: Pose::default() },
    }).collect::<Vec<_>>();
    steps.push(ProcessStep {
        id: "attach".into(),
        label: "Attach".into(),
        enabled: true,
        origin: None,
        measure: ProcessMeasure::Attach { component: solid("attachmentSize"), pose: Pose::default() },
    });
    let scene = ProcessWorkingScene { stock, steps };
    let mut session = ProcessKernelReplay::new();
    let last = replay_process(&mut session, &scene, None).expect("replay succeeds").expect("attachment restores material");
    assert_eq!(session.tables.memo.len() as u64, fixture["memoCapacity"].as_u64().unwrap());
    let enabled = scene.steps.iter().collect::<Vec<_>>();
    let signatures = prefix_signatures(session.stock_signature, &enabled);
    let first = fixture["oldestRetainedPrefix"].as_u64().unwrap() as usize;
    assert_eq!(session.tables.order.iter().copied().collect::<Vec<_>>(), signatures[first..]);
    let stock = replay_process(&mut session, &scene, Some(0)).expect("rewind succeeds").expect("stock restored");
    assert!((session.kernel().volume(&stock).unwrap() - fixture["stockVolume"].as_f64().unwrap()).abs() < 1e-9);
    assert!((session.kernel().volume(&last).unwrap() - fixture["restoredVolume"].as_f64().unwrap()).abs() < 1e-9);
    assert_eq!(replay_process(&mut session, &scene, None).unwrap(), Some(last));

    let mut preview = ProcessPreviewReplay::default();
    let (mesh, volume) = preview.evaluate(&scene, None).unwrap();
    assert!(mesh.is_some());
    assert!((volume - fixture["restoredVolume"].as_f64().unwrap()).abs() < 1e-9);
    assert_eq!(preview.replay.kernel.registry_len(), 1);
    let restored = preview.replay.tables.memo[signatures.last().unwrap()].clone();
    let (mesh, volume) = preview.evaluate(&scene, Some(count)).unwrap();
    assert!(mesh.is_none());
    assert_eq!(volume, 0.0);
    assert_eq!(preview.replay.kernel.registry_len(), 1);
    let (_, volume) = preview.evaluate(&scene, Some(0)).unwrap();
    assert!((volume - fixture["stockVolume"].as_f64().unwrap()).abs() < 1e-9);
    assert_eq!(preview.replay.kernel.registry_len(), 2);
    let (_, volume) = preview.evaluate(&scene, None).unwrap();
    assert!((volume - fixture["restoredVolume"].as_f64().unwrap()).abs() < 1e-9);
    assert_eq!(preview.replay.tables.memo[signatures.last().unwrap()], restored);
    assert_eq!(preview.replay.kernel.registry_len(), 2);
}
