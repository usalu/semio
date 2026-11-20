# Mounted FEM 3D demo verification

The shipped dead-load case exercises consistent member self-weight and tetrahedral stress recovery. The frame and slab are separate structures. The earlier zero lateral response at `n00_l1` came from omitting the fixed-end moments of frame self-weight. The earlier `sc0` stress mismatch came from choosing wedge apexes and shared face diagonals using different point orders in the mounted and synchronous paths. The existing solver corrections account for the moments and use lexicographic footprint rank for tetrahedral connectivity and generated node identities.

The language-neutral paged-demo fixture now supplies independent references for both regressions. The mounted regression checks the published x/y displacement at `n00_l1` and the published von Mises contour at the supported slab corner `sc0`. The existing synchronous comparison still checks correspondence across all analysis nodes.

| Quantity | Independent NumPy result |
| --- | ---: |
| First-storey corner x displacement | −2.958388689859433 × 10⁻⁶ m |
| First-storey corner y displacement | −4.967031479155994 × 10⁻⁶ m |
| Slab corner von Mises stress | 59,312.62075428927 Pa |
| Sum of vertical slab support reactions | 50,288 N |

[The reproducible NumPy oracle](mounted-demo-oracle.py) reads the shipped frame geometry and integrates the Euler–Bernoulli stiffness and distributed gravity with three-point Gauss–Legendre quadrature. It takes the slab's 16-point, 18-triangle footprint as reference input and solves the 54 constant-strain tetrahedra using full symmetric strain tensors. The reference stress averages the element von Mises stresses incident to the corner. NumPy 2.5.4 reproduced the prior vertical fixture values and generated the two new witnesses.

A native probe linked the Rust mesher from the Windows test build and verified all 18 footprint triangles against the oracle inputs. Substituting the exact Rust point values reproduced 59,312.62075428911 Pa.

Changing the NumPy tetrahedral split to use insertion rank raises the computed corner stress to 60,495.59505707049 Pa. This exceeds the mounted assertion tolerance of 0.5932 Pa and preserves the same total slab reaction.

The first Windows native build stopped in semio-plugin-tool-machine before running tests. It was using dependency artifacts compiled before concurrent changes reintroduced the native storage exports and the const projection-admission constructor. The current tree contains both changes. The fresh build linked the mounted session test after the concurrent API changes settled. The Windows native mounted demo regression passed in 12.84 seconds, including the independent displacement and stress assertions and the synchronous comparison across every analysis node.

The next shared compile surfaced two additional issues in the continuous-tool follower. The old vacant byte template could not promote the generic follower to a static reference. Its replacement enumerates every field, quotes the size of the requested field without reading the uninitialized owner, and writes only that field. The retained birth closure also now supports its mutable captures. A pinned-nightly test of the field plan exercised a generic owner with interior mutability and a destructor, verified correct byte quotes and out-of-range refusal, and checked the fully initialized owner.
