use super::*;

#[test]
fn von_neumann_offsets_are_symmetric() {
    Stencil2d::VonNeumann.validate().unwrap();
    Stencil2d::Moore.validate().unwrap();
    Stencil2d::Hex.validate().unwrap();
}

#[test]
fn custom_stencil_rejects_unpaired_offset() {
    let s = Stencil2d::Custom(vec![(1, 0)]);
    assert!(s.validate().is_err());
}

#[test]
fn custom_stencil_rejects_duplicate_and_self_offset() {
    assert!(Stencil2d::Custom(vec![(1, 0), (1, 0), (-1, 0)]).validate().is_err());
    assert!(Stencil2d::Custom(vec![(0, 0)]).validate().is_err());
}

#[test]
fn node_at_and_coords_roundtrip() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔀️topology-contracts/🔣️.json")).unwrap();
    let [width, height]: [usize; 2] = serde_json::from_value(oracle["dimensions2d"].clone()).unwrap();
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
    let topo = Grid2dTopology::new(width, height, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();
    let n = topo.node_at(2, 1).unwrap();
    assert_eq!(topo.coords(n), (2, 1));
    assert_eq!((topo.width(), topo.height()), (width, height));
    for y in 0..topo.height() {
        for x in 0..topo.width() {
            assert_eq!(topo.coords(topo.node_at(x, y).unwrap()), (x, y));
        }
    }
    assert_eq!(topo.node_at(topo.width(), 0), None);
    assert_eq!(topo.node_at(0, topo.height()), None);
}

#[test]
fn open_boundary_drops_out_of_range_arcs() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
    let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();
    let corner = topo.node_at(0, 0).unwrap();
    let mut out = Vec::new();
    topo.for_each_out_arc(corner, |m, _| out.push(m));
    assert_eq!(out.len(), 2); // only east and south exist from the top-left corner
}

#[test]
fn wrap_boundary_connects_opposite_edges() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
    let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Wrap, Boundary::Wrap, None).unwrap();
    let corner = topo.node_at(0, 0).unwrap();
    let mut out = Vec::new();
    topo.for_each_out_arc(corner, |m, _| out.push(m));
    assert_eq!(out.len(), 4); // wraps to all four neighbors
    assert!(out.contains(&topo.node_at(2, 0).unwrap())); // west wraps to x=2
    assert!(out.contains(&topo.node_at(0, 2).unwrap())); // north wraps to y=2
}

#[test]
fn size_one_axis_wrap_self_loops() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
    let topo = Grid2dTopology::new(1, 3, &Stencil2d::VonNeumann, rels, Boundary::Wrap, Boundary::Open, None).unwrap();
    let n = topo.node_at(0, 1).unwrap();
    let mut out = Vec::new();
    topo.for_each_out_arc(n, |m, _| out.push(m));
    // east/west both wrap to the same single column -> self-loop arcs, plus north/south.
    assert_eq!(out.len(), 4);
}

#[test]
fn mirror_boundary_reflects_at_edges() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
    let topo = Grid2dTopology::new(5, 5, &Stencil2d::VonNeumann, rels, Boundary::Mirror, Boundary::Mirror, None).unwrap();
    let corner = topo.node_at(0, 0).unwrap();
    let mut out = Vec::new();
    topo.for_each_out_arc(corner, |m, _| out.push(m));
    assert_eq!(out.len(), 4);
    assert!(out.contains(&topo.node_at(1, 0).unwrap())); // west mirrors back to x=1
    assert!(out.contains(&topo.node_at(0, 1).unwrap())); // north mirrors back to y=1
}

#[test]
fn mask_excludes_inactive_cells_from_arcs() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
    let mut mask = vec![true; 9];
    mask[4] = false; // center of 3x3 inactive
    let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
    let north_of_center = topo.node_at(1, 0).unwrap();
    let mut out = Vec::new();
    topo.for_each_out_arc(north_of_center, |m, _| out.push(m));
    assert!(!out.contains(&topo.node_at(1, 1).unwrap())); // no arc into the masked-out center
    assert_eq!(topo.inactive_cells(), vec![NodeId(4)]);
}

#[test]
fn fixed_outside_restrictions_only_on_boundary_facing_axis() {
    let mut b = ModelBuilder::new();
    let solid = b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::VonNeumann).unwrap();
    let topo = Grid2dTopology::new(2, 2, &Stencil2d::VonNeumann, rels, Boundary::FixedOutside(solid), Boundary::Open, None).unwrap();
    let restrictions = topo.fixed_outside_restrictions();
    // Only x-axis boundary is FixedOutside; every cell touches an x-edge in a 2x2 grid.
    assert_eq!(restrictions.len(), 4);
    assert!(restrictions.iter().all(|&(_, _, p)| p == solid));
}

#[test]
fn in_arc_matches_out_arc_on_open_boundary() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations(&mut b, &Stencil2d::Moore).unwrap();
    let topo = Grid2dTopology::new(4, 4, &Stencil2d::Moore, rels, Boundary::Open, Boundary::Open, None).unwrap();
    for y in 0..4 {
        for x in 0..4 {
            let n = topo.node_at(x, y).unwrap();
            let mut outgoing = Vec::new();
            topo.for_each_out_arc(n, |m, r| outgoing.push((m, r)));
            let mut incoming_as_source: Vec<(NodeId, RelationId)> = Vec::new();
            for oy in 0..4 {
                for ox in 0..4 {
                    let other = topo.node_at(ox, oy).unwrap();
                    let mut theirs = Vec::new();
                    topo.for_each_in_arc(other, |src, r, _slot| theirs.push((src, r)));
                    if theirs.contains(&(n, RelationId(0))) || theirs.iter().any(|&(s, _)| s == n) {
                        for &(s, r) in &theirs {
                            if s == n {
                                incoming_as_source.push((other, r));
                            }
                        }
                    }
                }
            }
            let mut a = outgoing;
            let mut b2 = incoming_as_source;
            a.sort_by_key(|&(m, r)| (m.get(), r.get()));
            b2.sort_by_key(|&(m, r)| (m.get(), r.get()));
            assert_eq!(a, b2, "node ({x},{y}) out-arcs must match in-arc reconstruction on an open boundary");
        }
    }
}
