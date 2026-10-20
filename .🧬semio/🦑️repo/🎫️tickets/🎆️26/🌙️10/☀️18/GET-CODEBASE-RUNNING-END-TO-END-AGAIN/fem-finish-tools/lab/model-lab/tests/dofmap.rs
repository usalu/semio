use semio_s_fem_analyses_model_lab::*;

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
