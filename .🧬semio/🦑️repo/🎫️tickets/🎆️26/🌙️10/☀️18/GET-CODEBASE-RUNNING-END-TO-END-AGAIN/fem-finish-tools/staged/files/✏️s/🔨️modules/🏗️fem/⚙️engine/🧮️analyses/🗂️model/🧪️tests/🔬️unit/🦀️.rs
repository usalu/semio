use super::*;
use semio_s_fem_elements2d::BeamEb2;

fn assembly_physical_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️physical-owners/🔣️.json")).expect("assembly physical fixture")
}

/// 🧱 The mounted model moves as small metadata while payload pages require explicit admission.
#[test]
fn assembly_triplet_pages_mounted_model_has_no_inline_payload_arrays() {
    let fixture = assembly_physical_fixture();
    let bytes = size_of::<MountedAnalysisModel>();
    assert!(bytes <= fixture["mountedModel"]["maximumInlineBytes"].as_u64().unwrap() as usize);
}


type MountedAdmissionQuery = fn(&MountedAnalysisModel, usize) -> Result<Option<usize>, MountedAnalysisFault>;
type MountedAdmissionStep = fn(&mut MountedAnalysisModel, usize, usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault>;

fn mounted_fixture_owners() -> [(&'static str, MountedAdmissionQuery, MountedAdmissionStep); 3] {
    [
        ("node", MountedAnalysisModel::next_node_allocation_bytes, MountedAnalysisModel::admit_node_one),
        ("element", MountedAnalysisModel::next_element_allocation_bytes, MountedAnalysisModel::admit_element_one),
        ("support", MountedAnalysisModel::next_support_allocation_bytes, MountedAnalysisModel::admit_support_one),
    ]
}

fn mounted_fixture_coordinates(model: &MountedAnalysisModel, kind: &str) -> (usize, usize, usize) {
    match kind {
        "node" => (model.nodes.admitted, model.nodes.values.len(), model.nodes.values.capacity()),
        "element" => (model.elements.admitted, model.elements.values.len(), model.elements.values.capacity()),
        "support" => (model.supports.admitted, model.supports.values.len(), model.supports.values.capacity()),
        _ => unreachable!(),
    }
}

fn close_mounted_fixture(model: &mut MountedAnalysisModel) -> (usize, bool) {
    let mut released = 0;
    for _ in 0..4_096 {
        let before = model.physical_backing_bytes().into_iter().sum::<usize>();
        let step = model.close_step(MOUNTED_ANALYSIS_BACKING_BYTES);
        let after = model.physical_backing_bytes().into_iter().sum::<usize>();
        released += step.2;
        assert!(step.1 <= 1 && step.2 <= MOUNTED_ANALYSIS_BACKING_BYTES);
        assert_eq!(before, after + step.2);
        if step.0 { return (released, model.terminal_is_empty()); }
    }
    (released, false)
}

/// 🎟️ Every model backing is separately queried, admitted and physically retired under exact grants.
#[test]
fn assembly_triplet_pages_mounted_model_admits_one_backing_and_preserves_refused_grants() {
    let fixture = assembly_physical_fixture();
    for (kind, query, admit) in mounted_fixture_owners() {
        for target_name in fixture["mountedModel"]["targets"].as_array().unwrap() {
            let maximum = fixture["mountedModel"]["logicalLimits"][kind].as_u64().unwrap() as usize;
            let target = match target_name.as_str().unwrap() { "empty" => 0, "one" => 1, "maximum" => maximum, _ => unreachable!() };
            let mut model = MountedAnalysisModel::new();
            assert_eq!(model.physical_backing_bytes(), [0; 3]);
            let mut allocated = 0;
            let mut allocations = 0;
            let mut complete = false;
            for _ in 0..4_096 {
                let before = model.physical_backing_bytes();
                let coordinates = mounted_fixture_coordinates(&model, kind);
                let demand = query(&model, target).expect("exact model allocation demand");
                if let Some(bytes) = demand {
                    for refused in [0, bytes.saturating_sub(1)] {
                        assert_eq!(admit(&mut model, target, refused), Ok(MountedAnalysisAdmission::default()));
                        assert_eq!(model.physical_backing_bytes(), before);
                        assert_eq!(mounted_fixture_coordinates(&model, kind), coordinates);
                    }
                }
                let step = admit(&mut model, target, demand.unwrap_or(0)).expect("model admission");
                let after = model.physical_backing_bytes();
                assert_eq!(after.into_iter().sum::<usize>(), before.into_iter().sum::<usize>() + step.allocated_bytes);
                assert!(step.allocated_bytes <= MOUNTED_ANALYSIS_BACKING_BYTES);
                if step.allocated_bytes != 0 {
                    assert_eq!(mounted_fixture_coordinates(&model, kind).0, coordinates.0);
                    allocations += 1;
                }
                allocated += step.allocated_bytes;
                if step.complete { complete = true; break; }
            }
            assert!(complete);
            assert_eq!(mounted_fixture_coordinates(&model, kind).0, target);
            assert_eq!(mounted_fixture_coordinates(&model, kind).1, 0);
            for _ in 0..target {
                match kind {
                    "node" => model.push_node(Node { id: String::new(), pos: [0.0; 3] }).unwrap(),
                    "element" => model.push_element(BeamEb2 { id: String::new(), start: String::new(), end: String::new(), e: 1.0, area: 1.0, iy: 1.0, density: 1.0 }.into()).unwrap_or_else(|_| panic!("admitted element")),
                    "support" => model.push_support(MountedAnalysisSupport::new(String::new())).unwrap_or_else(|_| panic!("admitted support")),
                    _ => unreachable!(),
                }
            }
            let before_close = model.physical_backing_bytes();
            for _ in 0..1_024 {
                let step = model.close_step(0);
                assert_eq!(step.2, 0);
                if step.0 || step.1 == 0 { break; }
            }
            assert_eq!(model.physical_backing_bytes(), before_close);
            let (released, terminal) = close_mounted_fixture(&mut model);
            eprintln!("mounted model {kind}/{target_name}: allocations={allocations}, allocated={allocated}, released={released}, terminal={terminal}");
            assert_eq!(allocated, released);
            assert!(terminal);
            if target == maximum && kind != "support" { assert!(allocated > MOUNTED_ANALYSIS_BACKING_BYTES); }
        }
    }
}

/// 🛑 A partially admitted model retains its first fault and every real backing for cancellation.
#[test]
fn assembly_triplet_pages_mounted_model_partial_admission_and_fault_retain_every_backing() {
    let fixture = assembly_physical_fixture();
    for (kind, _, admit) in mounted_fixture_owners() {
        let maximum = fixture["mountedModel"]["logicalLimits"][kind].as_u64().unwrap() as usize;
        for cutoff in 1..=5 {
            let mut model = MountedAnalysisModel::new();
            let mut allocated = 0;
            for _ in 0..cutoff {
                allocated += admit(&mut model, maximum, MOUNTED_ANALYSIS_BACKING_BYTES).expect("partial model admission").allocated_bytes;
            }
            let before = model.physical_backing_bytes();
            let coordinates = mounted_fixture_coordinates(&model, kind);
            let expected = Err(MountedAnalysisFault::Capacity { requested: maximum + 1, maximum });
            assert_eq!(admit(&mut model, maximum + 1, MOUNTED_ANALYSIS_BACKING_BYTES), expected);
            assert_eq!(admit(&mut model, maximum, MOUNTED_ANALYSIS_BACKING_BYTES), expected);
            assert_eq!(model.physical_backing_bytes(), before);
            assert_eq!(mounted_fixture_coordinates(&model, kind), coordinates);
            let (released, terminal) = close_mounted_fixture(&mut model);
            assert_eq!(allocated, released);
            assert!(terminal);
        }
    }
}

#[test]
fn mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing() {
    let mut rejected = Vec::<f64>::new();
    assert!(!reserve_exact_owner_page(&mut rejected, MOUNTED_OWNER_PAGE_BYTES / size_of::<f64>() + 1));
    let observed = rejected.capacity() * size_of::<f64>();
    let pointer = rejected.as_ptr();
    assert!(observed > MOUNTED_OWNER_PAGE_BYTES);
    assert_eq!(close_vec_owner_step(&mut rejected, observed - 1), Err(()));
    assert_eq!(rejected.as_ptr(), pointer, "an interrupted close preserves the exact rejected backing");
    assert_eq!(close_vec_owner_step(&mut rejected, observed), Ok(Some((1, observed))), "the exact rejected allocation retires through the retained close helper");
    assert_eq!(rejected.capacity(), 0);
}

// #region 🔖️DofMapLaws
/// 🧭️ The DOF index hash is FNV-1a 64 over the node id bytes and one DOF index byte; the published FNV vectors pin it.
#[test]
fn dof_index_hash_matches_published_fnv_1a_vectors() {
    let published: [(&str, u64); 4] = [("", 0xcbf2_9ce4_8422_2325), ("a", 0xaf63_dc4c_8601_ec8c), ("b", 0xaf63_df4c_8601_f1a5), ("foobar", 0x8594_4171_f739_67e8)];
    for (text, digest) in published {
        for dof in Dof::ALL {
            let expected = (digest ^ dof.index() as u64).wrapping_mul(0x100_0000_01b3);
            assert_eq!(DofMap::hash(text, dof), expected, "{text:?} {dof:?}");
        }
    }
}

fn dof_fixture_order(nodes: usize) -> Vec<(String, Dof)> {
    (0..nodes).flat_map(|node| Dof::ALL.into_iter().filter(move |dof| (node + dof.index()) % 4 != 3).map(move |dof| (format!("node-{node}"), dof))).collect()
}

/// 🔎️ Every indexed lookup agrees with a scan of the order, absent owners miss, and the index never exceeds one half load.
#[test]
fn dof_map_lookup_matches_a_scan_of_the_order() {
    for nodes in [0, 1, 2, 17, 150, 700] {
        let order = dof_fixture_order(nodes);
        let map = DofMap::from_order(order.clone());
        assert_eq!(map.len(), order.len());
        assert_eq!(map.slots.len(), 2 * order.len());
        assert_eq!(map.is_empty(), order.is_empty());
        for (index, (node_id, dof)) in order.iter().enumerate() {
            assert_eq!(map.get(node_id, *dof), Some(index), "{node_id} {dof:?}");
            assert_eq!(map.order[index], (node_id.clone(), *dof));
        }
        for node in 0..nodes {
            for dof in Dof::ALL {
                let scanned = order.iter().position(|(node_id, owner)| *node_id == format!("node-{node}") && *owner == dof);
                assert_eq!(map.get(&format!("node-{node}"), dof), scanned);
            }
        }
        assert_eq!(map.get("absent", Dof::Tx), None);
        assert_eq!(map.get(&format!("node-{nodes}"), Dof::Ty), None);
        assert_eq!(map.slots.iter().filter(|slot| slot.is_some()).count(), order.len());
    }
}

/// 🎟️ The mounted construction places one owner into reserved pages; a missing reserved slot, a duplicate or a full index hands the owner back.
#[test]
fn dof_map_push_places_into_reserved_pages_and_refuses_without_changing_either_owner() {
    let maximum = 40;
    let mut map = DofMap::from_order(Vec::new());
    assert_eq!(map.get("a", Dof::Tx), None);
    assert_eq!(map.push(("a".into(), Dof::Tx)), Err(("a".into(), Dof::Tx)), "no reserved order slot");
    while map.order.capacity() < maximum { map.order.reserve_capacity_one(maximum, 4096).unwrap(); }
    assert_eq!(map.push(("a".into(), Dof::Tx)), Err(("a".into(), Dof::Tx)), "no initialized index slot");
    let slot_count = DofMap::slot_count(maximum).unwrap();
    while map.slots.capacity() < slot_count { map.slots.reserve_capacity_one(slot_count, 4096).unwrap(); }
    while map.slots.len() < slot_count { map.slots.push_reserved(None).unwrap(); }
    for index in 0..maximum {
        let owner = (format!("n{}", index / 4), Dof::ALL[index % 4]);
        assert_eq!(map.push(owner.clone()), Ok(()));
        assert_eq!(map.get(&owner.0, owner.1), Some(index));
        let before = (map.len(), map.slots.iter().filter(|slot| slot.is_some()).count());
        assert_eq!(map.push(owner.clone()), Err(owner), "duplicate owner");
        assert_eq!((map.len(), map.slots.iter().filter(|slot| slot.is_some()).count()), before);
    }
    assert_eq!(map.len(), maximum);
    assert_eq!(map.push(("overflow".into(), Dof::Tz)), Err(("overflow".into(), Dof::Tz)), "order page admission is exhausted");
    let mut tight = DofMap::from_order(vec![("a".into(), Dof::Tx)]);
    tight.order.reserve_capacity_one(2, 4096).unwrap();
    assert_eq!(tight.slots.len(), 2);
    assert_eq!(tight.push(("b".into(), Dof::Tx)), Ok(()));
    assert_eq!(tight.push(("c".into(), Dof::Tx)), Err(("c".into(), Dof::Tx)), "the index has no vacant slot left");
}
// #endregion 🔖️DofMapLaws
