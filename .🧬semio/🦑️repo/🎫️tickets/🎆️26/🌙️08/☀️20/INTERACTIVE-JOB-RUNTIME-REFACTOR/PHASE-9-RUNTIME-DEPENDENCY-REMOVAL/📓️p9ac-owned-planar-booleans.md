# P9ac Owned Planar Booleans

## Outcome

`semio-framework-2d` now implements its planar path booleans with an owned, bounded, deterministic line-arrangement kernel. The public `boolean_paths` and `boolean_paths_many` signatures, operation names, flattened `PathSegment` representation, empty-result errors, independent input-contour fill semantics, output hole winding, and running multi-input semantics are preserved.

The direct optional `geo = 0.29` dependency, `geo` feature edge, and all `geo` source imports are removed. Cargo.lock no longer attributes `geo` to `semio-framework-2d`; the now-unreferenced `geo`, `geo-types`, `i_overlay`, `earcutr`, `geographiclib-rs`, `float_next_after`, and `i_*` overlay packages were removed by Cargo. `robust`, `rstar`, and `spade` remain in the workspace lock through unrelated packages.

## Differential Oracle

Before implementation, the original `geo 0.29.3` path was run against a fixture census. The complete raw `Result<Vec<PathSegment>, DrawingError>` output is retained in `📝️p9ac-2d-geo-oracle.txt`.

The oracle covers:

- all operations: union, intersection, difference, XOR;
- overlapping, disjoint, shared-edge, shared-vertex, contained, and identical rectangles;
- exterior and hole winding, including contained difference/XOR;
- reversed input winding;
- duplicate vertices and zero-area collinear contours;
- deterministic multi-contour ordering;
- running three-input operations.

Owned tests retain these fixtures as exact output checks where coordinate order is contractual and semantic/topological checks elsewhere. Additional owned regression coverage verifies two disjoint polygons each with a hole, collinear overlap, intermediate-empty XOR recovery, self-crossing even-odd regularization, and coordinates translated to `1e12`.

## Kernel

The owned implementation:

1. parses finite `Move`/`Line`/`Close` contours with the original independent-filled-contour semantics;
2. computes proper and collinear intersections among all source edges;
3. splits edges into a canonical bounded planar arrangement;
4. classifies both sides of each atomic edge under the selected operation;
5. keeps and orients only result-boundary edges;
6. traces manifold contours with deterministic face turns;
7. removes collinear vertices, normalizes winding/start points, and orders each exterior immediately before its holes;
8. retains oriented intermediate regions so `boolean_paths_many` preserves holes and can recover from an empty intermediate XOR.

The implementation is bounded at 4,096 input edges and 65,536 atomic arrangement edges, returning `DrawingError::InvalidInput` beyond either bound. It accepts finite linear contours. As in the previous implementation, non-linear `PathSegment` variants are not interpreted by the boolean layer. Degenerate linear contours are regularized away. A scale- and ULP-aware tolerance resolves coincident/intersection nodes deterministically.

## Verification

| Gate | Result | Evidence |
|---|---:|---|
| Original geo oracle | pass, 1 oracle test | `📝️p9ac-2d-geo-oracle.txt` |
| Owned dual-parity iterations | pass, 7 fixture families | `📝️p9ac-2d-owned-dual-parity-2.txt` |
| Full native debug package | pass, 23/23 | `📝️p9ac-2d-owned-debug-test-post-timing.txt` |
| Repository Nx target | pass, 23/23 | `📝️p9ac-2d-owned-nx-test.txt` |
| Native release check | pass | `📝️p9ac-2d-owned-release-check.txt` |
| `wasm32-unknown-unknown` check | pass | `📝️p9ac-2d-owned-wasm-check.txt` |
| No-default-features check | pass | `📝️p9ac-2d-owned-no-default-check.txt` |
| Booleans-only feature check | pass | `📝️p9ac-2d-owned-booleans-only-check.txt` |
| Locked workspace metadata | pass | `📝️p9ac-2d-owned-locked-metadata.json` |
| Focused clippy with `-D warnings` and `--no-deps` | pass | `📝️p9ac-2d-owned-clippy.txt` |
| Focused rustfmt | pass | `📝️p9ac-2d-owned-rustfmt-final.txt` |
| Focused diff check | pass | `📝️p9ac-2d-owned-diff-check.txt` |
| Direct/source geo census | zero | `📝️p9ac-2d-owned-geo-source-census.txt` |
| Owned dependency tree geo census | zero | `📝️p9ac-2d-owned-dependency-tree.txt` |
| `[DEBUG]` census in 2D module | zero | `📝️p9ac-2d-owned-debug-census.txt` |

Native/release/wasm builds emit only the pre-existing `async_fn_in_trait` warnings from `semio-framework-os-kernel`; the focused owned clippy gate is green.

## Interactive Timing

A temporary timing census (removed from source after measurement) ran 1,000 overlapping-rectangle unions in the debug test binary:

- total: 24,294 µs;
- maximum individual call: 331 µs;
- mean: approximately 24.3 µs/call.

Raw evidence is retained in `📝️p9ac-2d-owned-interactive-timing.txt`. This is a representative small interactive contour, not a universal upper bound. Large inputs are explicitly bounded by edge/arrangement limits and remain synchronous under the preserved public API.

## Disk Isolation

All builds used ticket-local targets:

- geo oracle: `🧪️target-p9ac-2d-oracle` — 368 MiB;
- owned debug/release/wasm/clippy: `🧪️target-2d-owned-geo` — 819 MiB.

The paths and sizes are retained in `📝️p9ac-2d-owned-target-disk.txt`.

## Files Changed

- `🧰️framework/🔨️modules/◻2d/🔀️booleans/🦀️component.rs`
- `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/Cargo.toml`
- `Cargo.lock` (Cargo-resolved removal of the retired dependency edge and unused transitive packages; the workspace also contains concurrent lockfile edits outside this slice)

No product, Puzzle, FEM, renderer, or job source was touched.
