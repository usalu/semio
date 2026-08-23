# P6i FEM Live Visual Publication Repair Contract

Date: 2026-08-24
Owner: `/root` coordinator
Verdict: **PREPARED — implementation begins after P6g and composes P6h progress.**

## Purpose

Mount deterministic, bounded FEM preview construction and publication for 2D and 3D. P6g supplies
the operation/session and exact progress-to-surface invalidation. P6i replaces whole visual-layer
sort/traversal/encoding with retained fixed-page jobs and carries one immutable latest valid visual
lease through the renderer.

## Current Boundary

`Fem2dLiveVisual` and the accepted layer vocabulary exist. P6g repairs exact job-progress dirtying
and keeps visual state behind a lazy snapshot read. The remaining encoder still sorts and traverses
complete region, assembling-element, field, load, support, glyph, contour, and validation
collections in one call. Historical 256-field timing is useful but not an adversarial mounted bound.

FEM3d has numerical examples but no accepted mounted, four-tier, cursor-built visual publication
equivalent. Phase 6 requires 2D first and then 3D.

## Visual Schema

Define dimension-specific owned visual schemas with stable identifiers and no default language.
English and German labels are first-class locale entries. Shared semantic states are:

- unmeshed;
- coarse mesh;
- refined mesh;
- assembling;
- solving unconverged;
- solving converged;
- validated final; and
- faulted/cancelled while preserving last valid.

The schema covers region/element IDs, mesh nodes/elements/quality, active assembly element,
loads/supports, displacement vectors, residual vectors/norms, reaction vectors, scalar contours,
mode shapes/eigen estimates, convergence/tolerance, quality tier, progress, provisional/final label,
operation/model/document generations, and exact renderer layer identities.

Every dynamic string/ID/value/page/control owner and current-plus-candidate visual backing has a
declared exact maximum and admission credit.

## Retained Visual Build

One mounted `FemVisualJob` per operation generation advances through:

1. `ReserveSnapshot`;
2. `ReadProgressScalar`;
3. `OrderRegionKey`;
4. `BuildRegion`;
5. `OrderElementKey`;
6. `BuildMeshElement`;
7. `BuildAssemblyMark`;
8. `BuildLoadGlyph`;
9. `BuildSupportGlyph`;
10. `BuildDisplacementEntry`;
11. `BuildResidualEntry`;
12. `BuildReactionEntry`;
13. `BuildContourEntry`;
14. `BuildModeEntry`;
15. `BuildLabelEntry`;
16. `SealPages`;
17. `ValidateFreshness`;
18. `PublishLease`; and
19. `RetireDisplacedLease`.

One grant processes one key/scalar/glyph/vector entry/contour cell/page/control owner. Ordering uses
an owned incremental stable order/index built during admitted model/session construction; no full
`sort`, `collect`, recursive walk, or complete JSON/scene encoding remains on the mounted route.

The numerical session exposes immutable generation-tagged views/pages. The visual job never clones
complete solver vectors or holds a mutable solver borrow across scheduling. It retains only exact
admitted page leases until candidate publication/close.

## Publication and Freshness

Progress may coalesce to the newest visual generation. Displacing an unfinished candidate cancels
its token and moves every owner into retained close; it does not clear/drop the graph. Checkpoint,
commit, and final result publications remain lossless and are not visual-preview slots.

Validate operation, model revision, document generation, surface token/generation, numerical
preview sequence, and renderer scene generation immediately before publication. A stale visual may
retire but never dirty or replace a newer surface.

Atomic publication swaps one sealed immutable page lease. The previous valid preview remains
visible during build, cancellation, failure, memory pressure, and solver restart. Final validated
results are visually distinct from all provisional tiers.

## Renderer Mount

The exact FEM editor/viewer surface consumes the immutable visual lease through the accepted
prepared-frame/page path. It does not decode/clone/sort the whole visual on the UI thread.

Job progress coalesces one generation-validated dirty render for the exact instance/surface. A
progress event for an old operation cannot dirty a reused instance. Completion publishes the final
lease and invalidates once even without unrelated user input.

The accessible progress overlay exposes stage, completed/total units, residual/tolerance where
applicable, quality tier, provisional/validated status, cancel/retry/discard controls, and localized
screen-reader text. Keyboard and focus behavior follow the existing accessible operation overlay.

## 2D and 3D Separation

Land and independently accept 2D first. The 3D packet uses a separate schema/catalog and exact caps
for tetrahedra/hexahedra, three-component vectors, 3D glyphs/contours, and mode shapes. It may reuse
only the dimension-neutral page/session/publication interface.

Removing a 3D-only schema field in a verifier mutation must fail only the 3D fixture; a forged 2D
catalog cannot authorize 3D ownership.

## Admission and Close

Extend the P6g/P6h process inventory for live solver view pages, ordered indexes, visual candidate,
sealed current/displaced leases, localized label backing, renderer packet/output, fault, and every
control owner.

Maximum +1 rejects before transferring the solver view and returns the exact producer/lease. Actual
fixed backing or observed capacity is credited. Standard-container estimates and post-hoc length
measurement are forbidden.

Public take/resume/close covers active builder, rejected candidate, published-but-unclaimed lease,
displaced lease, renderer packet, and lost handle. One close grant releases one page/index/string/
glyph/control owner. Terminal-empty is exhaustive and all process/page/item/byte/control counters
must be zero.

## Hostile Fixtures

Add 2D and 3D fixtures for:

- zero/max/max+1 regions, nodes, elements, fields, glyphs, contour cells, modes, IDs, localized
  strings, pages, candidates, and output slots;
- visibly distinct unmeshed/coarse/refined/assembling/unconverged/converged/final frames;
- progress-driven repeated render without user input;
- stale progress after operation restart and instance/surface generation reuse;
- cancellation/fault/panic/drop at every build/publication/close phase;
- latest-wins candidate displacement with exact old-owner retirement;
- full visual/result/terminal registries and exact producer handback;
- window/document/app close and dropped-handle partial close recovery;
- last-valid lease preservation during memory pressure and failure;
- deterministic byte-identical layers across worker counts;
- numerical-to-visual value correspondence for displacement/residual/reaction/mode fixtures;
- native and real browser worker mounted display; and
- first substantive preview below 50 ms, active cadence at least every 33 ms under load, UI callback
  p99 at most 2 ms, and no worker/UI step at or above 8 ms.

Every property has a faithful production-source verifier mutation.

## Acceptance Gates

Source handoff requires exact mounted caller census, scoped rustfmt/diff, verifier self-test/live
focused success, deterministic visual ledgers, and independent Terra audit. Final acceptance
requires serialized debug/release/strict warnings, native/both Wasm, real browser worker,
allocation/cancel/fault/close stress, numerical parity, worker-count replay, screenshot/semantic
accessibility fixtures, and timing on the same final tree.

P6i and Phase 6 remain RED until both dimension packets and runtime gates pass.
