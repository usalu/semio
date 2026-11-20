# Mounted House and Concrete Forest capacity

The published House asset contains eight solids, 63 authored nodes and 63 supports. The raft alone generates 221 footprint vertices and 384 triangles at the authored mesh resolution. The historical mounted admission left 32 footprint vertices and 42 triangles after admitting the authored nodes. Concrete Forest contains 20 nodes, 20 frames and two supports, so it fits the historical ceilings.

The native Rust mesher produced the following counts from the unchanged House assets. The running vertex count merges coincident positions with the synchronous node identity tolerance.

| Solid | Footprint vertices | Footprint triangles | Layers | Tetrahedra | Running merged nodes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Raft | 221 | 384 | 1 | 1,152 | 442 |
| Ground slab | 165 | 280 | 1 | 840 | 607 |
| West wall | 101 | 144 | 1 | 432 | 764 |
| East wall | 101 | 144 | 1 | 432 | 921 |
| South wall | 108 | 160 | 1 | 480 | 1,078 |
| North wall | 110 | 168 | 1 | 504 | 1,239 |
| Attic slab | 165 | 280 | 1 | 840 | 1,473 |
| Roof | 34 | 36 | 16 | 1,728 | 1,939 |
| Total | | | | 6,408 | 1,939 |

The FEM3D numerical child now uses paged analysis identifiers, solid storage and per-node equation, load, reaction, stress and mass values. The analysis capacities come from the physical model owner. The visual solver allocates result pages after meshing reveals the actual node count, and it closes only the pages it admitted. The separate authoring limits still cover the authored House geometry.

The language-neutral mounted example law checks completed dead-load publication for both assets, analysis-node count and vertical support equilibrium. NumPy independently integrated the published House volumes and attic surface pressure to 2,131,298.9 N. It summed the Concrete Forest member loads over the lengths of the published frames to 684,631.2580208586 N.

Native mounted verification is pending. The working tree's physical model and sparse scalar owners now have larger paged limits. The mounted triangulation owner passed its native House raft regression and all 17 mesh tests.




[The independent House solve](mounted-house-oracle.py) takes the exact native Rust footprint coordinates and connectivity, including every wall opening. NumPy assembles the tetrahedral equations and solves them with Jacobi-preconditioned conjugate gradients. The relative residual was 4.19429255 × 10⁻⁵ after 512 iterations. The solve reached a relative tolerance of 10⁻⁸ after 693 iterations and yielded a vertical support reaction of 2,131,298.899923497 N. The previous fixed 512-iteration limit therefore also prevented the House solve from converging. To rerun the oracle, pass the native Cargo build directory containing `debug/build` as `--build-dir`.
