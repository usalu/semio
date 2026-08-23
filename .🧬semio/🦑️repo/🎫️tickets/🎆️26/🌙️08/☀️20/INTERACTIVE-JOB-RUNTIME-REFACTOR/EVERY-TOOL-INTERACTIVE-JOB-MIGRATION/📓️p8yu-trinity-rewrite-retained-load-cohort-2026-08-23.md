# Phase 8 Trinity Rewrite Retained Load Cohort — 2026-08-23

## Disposition

**SOURCE-AUDIT-READY, not accepted.** This packet advances exactly the Trinity Rewrite envelope
caller. It does not edit the shared fail-closed definition, accepted Trinity Jack domain owners, or
another plugin/tool cohort. Cargo, Nx, native, Wasm, browser, network, and runtime timing were not
run by instruction, so compile/runtime acceptance remains RED.

The independent audit rejection is remediated source-only. The Rewrite adapter no longer
stringifies `Err((fault, _page))` while dropping the exact returned page. A shared read-only
preflight now validates generation, handle, close state, fixed page capacity and byte capacity
before invoking the 4 KiB producer callback. The public result is a typed
`TrinityRewriteEnvelopePageAdmission` carrying a machine-readable fault and a shallow handle to
the untouched caller `Uint8Array`; it exposes pointer identity, take, exact-handle retry,
one-owner close and terminal emptiness.

## Exact boundary and architecture

The former `TrinityRewriteArtifactVcs::new(envelope_json: Option<String>)` synchronously accepted a
whole JSON String, called `reject_whole_buffer_artifact_envelope_ingress`, constructed a mutable
`TrinityGraphStore`, and exposed direct whole-command/projection JSON methods. That bridge is gone.

The Rewrite bridge now owns `VcsArtifactApp<EditorApp<TrinityJackPlayApp>>`, the already accepted
Trinity Jack schema-first domain authority. Its public lifecycle is only:

1. `beginEnvelopeLoad(maximum_pages, maximum_bytes)` with exact nonzero `64 pages / 256 KiB`
   ceilings and a generation-tagged operation handle;
2. `admitEnvelopePage(handle, Uint8Array)` returns a typed admission. Stale/closing/capacity
   rejection occurs before `copy_to`; the result retains the same caller page reference for
   `isSamePage`, `takePage`, `retryEnvelopePage`, or one-owner `closeStep`. Accepted input is
   copied once into an inline `4 KiB` page only inside the preflighted producer callback;
3. `sealEnvelopeLoad(handle)` without a whole-buffer materialization;
4. `pollEnvelopeLoad(handle)`, which advances one `maintenance_step(1, 4 KiB)` turn and exposes
   Pending/Progress/Ready/Cancelled/Fault;
5. exact first/duplicate/stale completion acknowledgement;
6. `cancelEnvelopeLoad(handle)`; and
7. `closeStep()`, which grants one item and one page of byte credit and withholds completion until
   the shared authority reports terminal ownership.

No Rewrite compatibility wrapper remains for `dispatchText`, `dispatchBinary`, `projectionJson`,
`envelopeJson`, or the whole-buffer constructor. The domain decode, nested Jack owner retirement,
initializer, last-valid generation swap, cancellation, displaced-store retirement, and fixed
operation registries remain the single accepted Jack/framework implementation rather than a
second Rewrite implementation.

## Fixed ownership and adversarial evidence

The Rewrite source adds exact cap fixtures covering both maximum values, zero, aggregate `+1`,
page `+1`, and bytes `+1`. Its live generic caller-page owner is exercised with pointer/content
identity, retry by value, stale generation/slot ABA, checked-out Drop preserving the external raw
authority, and one-owner close. The permanent verifier requires preflight before `copy_to`, the
typed result/fault, shallow caller identity, take/retry/close/terminal methods, and all semantic
fixtures, together with the accepted Jack initializer/cancel and fixed-registry fixtures.

The permanent verifier checks/mutations cover:

- whole-buffer constructor resurrection;
- dynamic post-lift page input;
- generation erasure from the operation handle;
- fixed array replacement with a growable buffer;
- typed rejected-result erasure;
- ordinary rejected-owner drop;
- byte-cloning instead of retaining the caller page reference;
- copying before the shared preflight;
- missing take, retry, or one-owner close;
- unsealed submission;
- `run_to_completion` polling;
- missing exact terminal ACK;
- cancellation by dropping the handle;
- bulk close;
- missing cap/+1 fixture;
- missing fixed-registry exact plus-one handback; and
- missing one-page interrupted close.

All **299** verifier self-tests pass. These are source-verifier tests; the Rust fixture was authored
but not executed because builds were explicitly closed.

## Exact census and gates

The dated pre/post census is
`p8yu-next-live-placeholder-census-2026-08-23.md`.

| Gate | Result |
| --- | --- |
| production whole-buffer symbol census | **PASS**: decreased from 14 to **13** Rust occurrences: one shared definition + 12 live callers; Trinity Rewrite zero |
| edition-2021 scoped `rustfmt --check` | **PASS** on the Rewrite adapter and the two minimal shared authority files |
| Bun TypeScript/parser + permanent verifier self-test | **PASS: 299 clean** |
| live Trinity Rewrite verifier predicate | **PASS**: the full ledger has no Rewrite-specific failure |
| deterministic ledgers | **PASS**: byte-identical `p8yu-trinity-rewrite-ledger-a/b.json`, SHA-256 `f12335f43c5f7e2fc790aa11282cf2f2525062ce76cbe71e8571a1aac6ecb5ce` |
| full tool-job verifier | expected global **RED**: **0/884**, 18 failure classes; no Rewrite-specific failure |
| broad interactivity DENY | **RED outside this packet**: four concurrent prepared-raster predicate findings; no Rewrite finding |
| scoped working/staged/HEAD diff checks | **PASS** |
| whole working diff check | **PASS** |
| whole staged/HEAD diff checks | **RED outside this packet**: a Phase 3 raster audit has a blank line at EOF and the shared user prompt has pre-existing trailing whitespace |
| Cargo/Nx/native/Wasm/browser/network/runtime | **not run by instruction** |

The structural live census truly decreased by one. The remaining twelve callers are Dag, Flow,
FEM 2d, FEM 3d, Procedural 2d, Procedural 3d, CAD, Puzzle 5d, Puzzle 3d, Shooting, Process 3d, and
Raster. This packet does not begin any of them.
