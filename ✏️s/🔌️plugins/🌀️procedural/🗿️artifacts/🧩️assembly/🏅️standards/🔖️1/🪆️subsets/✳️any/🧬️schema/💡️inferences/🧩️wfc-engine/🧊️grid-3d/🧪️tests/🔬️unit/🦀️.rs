
use super::*;

#[test]
fn face6_edge18_vertex26_offset_counts() {
    assert_eq!(Stencil3d::Face6.offsets().len(), 6);
    assert_eq!(Stencil3d::Edge18.offsets().len(), 18);
    assert_eq!(Stencil3d::Vertex26.offsets().len(), 26);
}

#[test]
fn all_built_in_stencils_validate() {
    Stencil3d::Face6.validate().unwrap();
    Stencil3d::Edge18.validate().unwrap();
    Stencil3d::Vertex26.validate().unwrap();
}

#[test]
fn node_at_and_coords_roundtrip() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
    let topo = Grid3dTopology::new(3, 4, 5, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
    let n = topo.node_at(1, 2, 3).unwrap();
    assert_eq!(topo.coords(n), (1, 2, 3));
    assert_eq!(topo.node_at(3, 0, 0), None);
}

#[test]
fn open_boundary_corner_has_three_neighbors() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
    let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
    let corner = topo.node_at(0, 0, 0).unwrap();
    let mut out = Vec::new();
    topo.for_each_out_arc(corner, |m, _| out.push(m));
    assert_eq!(out.len(), 3);
}

#[test]
fn wrap_boundary_connects_all_axes() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
    let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Wrap, Boundary::Wrap, Boundary::Wrap, None).unwrap();
    let corner = topo.node_at(0, 0, 0).unwrap();
    let mut out = Vec::new();
    topo.for_each_out_arc(corner, |m, _| out.push(m));
    assert_eq!(out.len(), 6);
}

#[test]
fn mask_excludes_inactive_voxels() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Face6).unwrap();
    let mut mask = vec![true; 27];
    mask[13] = false; // center of 3x3x3
    let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Face6, rels, Boundary::Open, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
    let neighbor = topo.node_at(1, 1, 0).unwrap(); // directly below center
    let mut out = Vec::new();
    topo.for_each_out_arc(neighbor, |m, _| out.push(m));
    assert!(!out.contains(&topo.node_at(1, 1, 1).unwrap()));
    assert_eq!(topo.inactive_cells(), vec![NodeId(13)]);
}

#[test]
fn in_arc_matches_out_arc_on_open_boundary() {
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    let rels = declare_stencil_relations_3d(&mut b, &Stencil3d::Vertex26).unwrap();
    let topo = Grid3dTopology::new(3, 3, 3, &Stencil3d::Vertex26, rels, Boundary::Open, Boundary::Open, Boundary::Open, None).unwrap();
    for z in 0..3 {
        for y in 0..3 {
            for x in 0..3 {
                let n = topo.node_at(x, y, z).unwrap();
                let mut outgoing = Vec::new();
                topo.for_each_out_arc(n, |m, r| outgoing.push((m, r)));
                let mut reconstructed = Vec::new();
                for oz in 0..3 {
                    for oy in 0..3 {
                        for ox in 0..3 {
                            let other = topo.node_at(ox, oy, oz).unwrap();
                            let mut theirs = Vec::new();
                            topo.for_each_in_arc(other, |src, r, _slot| theirs.push((src, r)));
                            for &(s, r) in &theirs {
                                if s == n {
                                    reconstructed.push((other, r));
                                }
                            }
                        }
                    }
                }
                outgoing.sort_by_key(|&(m, r)| (m.get(), r.get()));
                reconstructed.sort_by_key(|&(m, r)| (m.get(), r.get()));
                assert_eq!(outgoing, reconstructed, "voxel ({x},{y},{z}) out-arcs must match in-arc reconstruction");
            }
        }
    }
}
