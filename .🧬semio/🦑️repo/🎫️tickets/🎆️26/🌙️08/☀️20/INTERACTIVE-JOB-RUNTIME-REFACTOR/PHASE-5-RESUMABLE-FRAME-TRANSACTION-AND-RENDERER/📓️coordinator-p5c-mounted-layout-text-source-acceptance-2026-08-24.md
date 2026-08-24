# Coordinator P5c Mounted Layout and Text Source Acceptance

Date: 2026-08-24  
Disposition: **GREEN at the P5c source/static gate; Phase 5 executable gates remain deferred.**

## Scope

This read-only coordinator audit rechecked the live P5c implementation and the exact contract after
`📓️sol-p5c-mounted-layout-text-worker-implementation-2026-08-24.md`. It inspected the retained
layout/text job, UI-wgpu scheduler/publication/consumers, renderer Interpreter mount and permanent
verifier. No production source was changed by this audit.

## Accepted Findings

- `MountedLayoutJob` owns fixed credited node, walk, run, glyph, line, preview, result and atlas-page
  authorities. Admission, node/text walk, unwind, glyph, measurement, arrangement, preview,
  publication and close retain their cursors and advance one semantic unit per opportunity.
- Text work is behind the owned scalar interface and is driven only from the mounted worker job.
  The renderer supplies one fresh-input-priority opportunity through the shared process worker pool
  on a typed lane; no P5c-owned scheduler or direct UI-thread layout step is created.
- The fixed generation-qualified surface registry and per-lane rings replace dynamic surface
  queues. Tree/theme/viewport/layout identities use checked exhaustion and supersession retains
  the prior job/session/rejection authority until incremental close completes.
- Layout results write the inactive per-node slot, revalidate completeness and identity, and commit
  by one accepted-generation swap. Paint, hit testing and scene collection read that same accepted
  layout. Progressive geometry and glyph previews remain freshness-qualified while work is pending.
- The hostile laws exercise node/glyph/depth MAX+1 identity, multipage Unicode, real shared-pool
  lane/thread, cancellation before/after owned text, expired deadline, one-owner close, stale and
  one-swap publication, resize supersession, identity exhaustion, deterministic replay, weighted
  fairness and the <8 ms caller slice.
- The permanent gate inspects live production bodies and rejects 28 structural plus 15 law-body
  mutations; legacy batch layout remains test-only.

## Evidence Run

| Gate | Result |
| --- | --- |
| Direct `interactivityMountedLayoutTextSelfTests` invocation | PASS — `live-source clean; structural-mutations=28; law-body-mutations=15` |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` over 11 P5c Rust files | PASS |
| Scoped unstaged `git diff --check` | PASS |
| Scoped cached `git diff --check` | PASS |
| P5c live source/owner/publication trace | PASS at the source/static gate |

The aggregate `verify interactivity p5c` route subsequently enters other packet audits and is not
the isolated P5c oracle; its unrelated shared-tree live-reconcile failure does not invalidate the
direct P5c self-test above.

## Boundary

P5b and P5c are source-accepted. Phase 5 remains open for P5a, P5d and P5e plus final Cargo,
debug/release/strict-warning, native/Wasm/browser, resize/effect/multi-window, allocation,
cancellation, replay and timing gates on the final quiescent tree.
