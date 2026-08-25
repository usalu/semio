# Phase 1.5 and Phase 6 Current Read-Only Gap Scout

Date: 2026-08-25

Scope: read-only current-tree review. This scout used `rg` and `sed` over the governing plan,
the master status, the canonical Phase 1.5 ticket, the FEM ticket/contracts/audits, the current
P6i/P6h source and the permanent verifier. It made no source edit and ran no Cargo, Nx, Wasm,
browser, runtime, or timing command.

## Verdict

- **Phase 1.5 remains open.** Its compiler-complete exit gate is not established on the current
  shared tree. This is a global async-semantics repair/matrix problem, not a newly found FEM
  engine regression.
- **P6h and P6i are currently source/static accepted.** The formerly concrete fixed-owner,
  pre-fuel, typed-field/render, draw-permit, and ordinary-Drop counterexamples have explicit
  production repairs and a strengthened P6i verifier. I found no new concrete source-only gap in
  the inspected mounted FEM route.
- **Phase 6 itself remains open for the final serialized executable matrix.** Source/static
  acceptance is not a substitute for current-tree native, release, Wasm, replay, cancellation,
  timing, allocation-pressure, and browser confirmation.

## Historical and Current Accepted Source Gates

### Phase 1.5

The canonical ticket is `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP`; its status is `open` and its declared
exit gate is an in-scope, compose-excluded workspace `--all-targets` compiler-clean result plus a
refreshed async census. The similarly named
`PHASE-1.5-ASYNC-SEMANTICS-CORRECTION` directory is evidence-only and has no ticket metadata, so
it cannot close the canonical phase.

Accepted historical sub-gates include the FEM product de-async packet:

- `r21-fem-product.md` records a zero-`async fn`/zero-`.await` owned FEM engine census (apart
  from the intentionally asynchronous framework boundaries), focused native/release evidence,
  FEM suite evidence, and the later WASI-P2/component-descriptor correction.
- The current source confirms the narrow engine result: `rg` reports **0** `async fn` and **0**
  `.await` occurrences under `✏️s/🔨️modules/🏗️fem/⚙️engine`.
- The FEM plugin wrapper still contains 221 non-test `async fn` declarations and 154
  `resolve_ready` calls. The inspected declarations are SDK/trait and application-boundary
  territory; this count is not evidence that the engine's numerical micro-cursor code reverted.
  It must nevertheless remain subject to the Phase 1.5 compiler gate.

The last recorded honest Phase 1.5 residue is in `📌️status.md`: `semio-s-plugin-stdio`,
`semio-framework-os-infinite`, `semio-framework-surface`, `semio-framework-plugin`, and the
recursive-async remainder in `semio-s-imperative`. Its documented primary shape is a
Future-typed bare `let` binding that needs def-use tracing, deliberately outside the old
span-local codemod. These counts are historical, not a current compiler claim.

### Phase 6

P6h was re-accepted after its earlier page-at-once/micro-cursor RED audit. The current
source/static evidence is:

- `coordinator-third-p6h-source-acceptance-2026-08-24.md`: P6h source/static GREEN, with the
  70-mutation isolated gate and scoped format/diff checks; executable gates explicitly deferred.
- `codex-p6i-ordinary-drop-narrow-independent-reaudit-2026-08-25.md`: P6i ordinary-Drop path
  GREEN, with 101 P6i and 70 P6h hostile mutations. This supersedes the immediately preceding
  ordinary-Drop RED report.
- The permanent verifier's live predicate now requires: pre-allocation backing claims; pre-work
  fuel order; typed World3d status consumption; exact draw permit reservation; fixed recovery
  slots; `MountedState` replacement-and-transfer on ordinary Drop; `MountedJob` publication,
  transfer/restoration, and recovery close one owner at a time.

Current source agrees with that trace. `MountedState::drop` replaces and transfers a nonterminal
owner to its reserved recovery slot; `MountedJob::drop` publishes `Recover`, cancels, transfers
the matching shell state when available, and restores on a failed handoff. `recover_abandoned_one`
then rediscover/takes the exact identity, drives one `close_step`, restores nonterminal state, and
only releases recovery/process/shell state after terminal-empty. The mounted numerical/visual
route also retains fixed page/slot structures and pre-fuel single-opportunity stages required by
the P6i exact predicate.

## Live Gaps or Regressions

### No newly demonstrated P6 source/static regression

I traced the current P6i recovery bodies and their production call points, including the direct
state Drop, queued/running job Drop, contended-shell rediscovery, recovery restore, terminal
credit release, and the associated law. The previously reported cancellation/assertion-only Drop
body is absent. No additional source-only ownership/caller failure was found in this bounded
review.

This is deliberately narrower than a runtime assertion: the recovery failure branches retain
defensive `mem::forget` plus a panic after an invariant-breaking recovery-slot collision. The
source proof treats such a collision as unreachable because reservation, identity and owner-slot
checks precede transfer. A runtime fault-injection matrix must still exercise that invariant rather
than treating the static predicate as proof of process behavior.

### Phase 1.5 source work still exists

The canonical Phase 1.5 ticket remains open and there is no current shared-tree compiler evidence
that its full exit gate is met. Existing reports explicitly identify:

1. the def-use async residue in `os-infinite`, `surface`, and `framework-plugin`;
2. the `semio-s-imperative` recursive `async fn` cycle decision; and
3. target/profile gates that cannot be inferred from earlier native-only or isolated reports.

The old Phase 1.5 reports also describe concurrent plugin churn; a new repair packet must begin
with a fresh structured compiler baseline and only touch error-primary spans, not reuse the stale
historical error totals.

## Runtime and Final-Matrix-Only Gates

Neither this scout nor the latest P6i ordinary-Drop re-audit ran executable gates. The remaining
Phase 6 matrix must establish, on a quiescent shared tree:

- native debug and release compilation/tests, strict warnings, numerical reference tolerances,
  real 2D/3D examples, and product rendering;
- both Wasm targets/component-descriptor path as applicable;
- mounted stale-edit cancellation, injected fault/panic, recovery-slot collision behavior,
  allocation pressure and close-drain behavior;
- deterministic replay across actual worker counts, coarse preview below 50 ms, and every
  callback/worker-step below 8 ms; and
- native and browser-visible prepared-snapshot publication and accessibility fields.

Phase 1.5 additionally needs the canonical compose-excluded compiler/all-targets gate and a
refreshed async census. The broad compiler gate must be serialized after active Rust source work;
historical workspace totals are explicitly non-deterministic and do not constitute a current pass.

## Exact Next Packets

### P15 Def-Use Async Residue Repair

One Sol High source packet should own only the current structured-diagnostic primary spans for
`semio-framework-os-infinite`, `semio-framework-surface`, and `semio-framework-plugin`, plus a
separate explicit decision/repair for the `semio-s-imperative` recursion. It must:

1. take a fresh per-crate JSON diagnostic baseline after peer source activity settles;
2. trace each bare Future local to its assignment and choose a genuine await or de-async only
   after inspecting that callee's suspension path;
3. make trait/macro changes in lockstep, with no name-keyed rewrite and no compatibility bridge;
4. add/retain language-agnostic laws and third-party differential evidence where the affected
   feature requires it; and
5. stop at packet-local compiler/source gates, leaving the broad workspace matrix serialized.

### P6 Final Matrix (No Further P6 Source Packet Identified)

After the P6 source tree is quiescent, assign one serialized build owner to rerun the P6 runtime
and platform matrix above. If that matrix exposes a concrete source defect, open a new bounded P6
repair packet from the failing reproduction; do not pre-emptively alter the currently accepted
P6i/P6h source route.

## Evidence Read

- `📌️status.md` and `📋️master.md` in this master ticket.
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/🎫️ticket.json` and `📓️r21-fem-product.md`.
- `RESUMABLE-FEM-JOB-GRAPH/🎫️ticket.json`, P6i repair contract, P6h acceptance,
  P6i fixed-owner/revoked-family/ordinary-Drop audits, and P6f report.
- Current `📜️script.ts` P6i/P6h verifier dispatch and exact predicate.
- Current FEM numerical engine and mounted FEM3D session source, inspected with `rg`/`sed`.
