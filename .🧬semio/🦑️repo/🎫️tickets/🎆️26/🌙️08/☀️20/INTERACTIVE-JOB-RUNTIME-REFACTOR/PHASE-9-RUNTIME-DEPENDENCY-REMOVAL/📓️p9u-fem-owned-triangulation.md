# P9u — FEM Owned Deterministic Triangulation

## Scope

This packet replaces FEM's sole direct `spade` dependency inside the existing mesh interface. It owns the FEM mesh engine component and FEM Rust manifest only; shared framework, stdio, renderer, and the root-owned FEM analyses component remain untouched.

## Implementation

- Added a first-party deterministic Bowyer–Watson triangulation kernel with stable coordinate ordering, deterministic insertion ranking, explicit counter-clockwise winding, and scale-aware in-circle classification.
- Added deterministic constrained-edge recovery through legal interior edge flips. Outer and hole loops are recovered in sorted order and protected from later flips.
- Preserved non-convex outer boundaries and holes through post-recovery centroid classification.
- Replaced refinement with deterministic boundary subdivision and an aligned interior lattice. Boundary and lattice spacing share the same maximum diagonal target, preventing the boundary slivers found by the first quality run.
- Migrated `MeshJob` from external triangulator handles to persistent owned state. Input subdivision/lattice seeding, point insertion scan/cavity compaction/fan creation, edge indexing, constraint traversal, and face publication retain separate cursors with cancellation/freshness checks, previews, and checkpoints. One insertion primitive is admitted per scheduler call, leaving contention headroom below the 8 ms ceiling.
- Removed the direct `spade = "2.15.1"` manifest row only after the FEM source census reached zero.

## Differential Evidence

Before removing `spade`, the owned kernel ran alongside the legacy public implementation on a square, a non-convex L-domain, and a square with a hole. The differential test compared covered area and positive winding and passed **1/1** (`📝️p9u-fem-spade-differential-rerun.txt`). The initial command used an exact unqualified filter and therefore selected zero tests; that routing mistake is retained in `📝️p9u-fem-spade-differential.txt` and was corrected immediately.

After deletion, the fixture remains as an implementation-independent deterministic replay/golden-area test (100, 75, and 84 square units).

## Verification

- Post-removal FEM test compile: **passed** (`📝️p9u-fem-owned-mesh-compile-1.txt`).
- First owned mesh run: **19 passed, 2 failed** (`📝️p9u-fem-owned-mesh-tests-1.txt`). The failures identified a real 15.87° refinement sliver and one contention-sensitive 24 ms adversarial step; neither threshold was weakened.
- The first 1,024-point runs exposed two genuine monolithic costs instead of being waived as timing noise: whole-cavity point insertion and rebuilding the triangle-edge map for every boundary constraint. Point insertion is now persistent `Scan → Retain → Fan`; a persistent `IndexEdges` stage makes already-present constraints logarithmic lookups. Input preparation is also item-cursor resumable after stage-tagged evidence identified it under contention. The retained diagnostics are `📝️p9u-fem-large-boundary-stage-debug.txt`, `📝️p9u-fem-large-boundary-preparation-debug.txt`, and `📝️p9u-fem-large-boundary-edge-index-debug.txt`; all temporary `[DEBUG]` output was removed afterward.
- The aligned refinement fixture passed **1/1** at the unchanged 25° target (`📝️p9u-fem-refinement-test-2.txt`).
- Final owned mesh suite: **21 passed, 0 failed**, including deterministic replay, cancellation, refinement, holes/non-convex domains, constrained edges, and the unchanged 1,024-point `<8 ms` assertion (`📝️p9u-fem-owned-mesh-tests-final-rerun.txt`).
- Final focused large-boundary gate: **1 passed, 0 failed** (`📝️p9u-fem-large-boundary-final.txt`). A five-run wall-clock experiment under simultaneous multi-agent Rust linking retained one OS-descheduling outlier in `📝️p9u-fem-large-boundary-replay-5x-final.txt`; the authoritative serial mesh run immediately after is the green 21/21 result, and no threshold was weakened.
- FEM source/manifest `spade` census: **zero** (`📝️p9u-fem-spade-zero-census-final.txt`). `cargo tree -p semio-s-plugin-fem -i spade` confirms the package is absent from the resolved graph (`📝️p9u-fem-spade-cargo-tree-final.txt`).
- Native library check after removal: **passed** (`📝️p9u-fem-native-check.txt`).
- Current full FEM execution reached **753/755** after all 21 mesh tests passed. The two red results are outside this seam: the excluded analyses path currently produces `[NaN, NaN, NaN]` for the bundled 3D buckling fixture, and concurrent system load descheduled one otherwise cursor-bounded mesh step for 8.51 ms (`📝️p9u-fem-full-tests-rerun.txt`). The buckling issue reproduces independently (`📝️p9u-fem-upstream-sparse-rerun.txt`); attempted local sparse mitigations were reverted completely, leaving no unrelated sparse diff.
- Release and official wasm results follow from the active routed gates.
