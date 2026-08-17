# Final summary — Mutation Outcomes, Merge Policies and First-Class Conflicts

Master plan `📋️master-plan.md`; frozen contract `📋️contract-freeze.md` (C1–C10); worker rules `📋️worker-brief.md`.

## What shipped

**Every mutation now produces a diff along with messages.** `Mutation::diff` / `MutationKind::diff` return
`MutationOutcome<D> { diff, messages: Vec<MutationMessage> }`; `MutationMessage` carries
`{ level: Severity, code: FaultCode, message, target, op_index }`. `validate` is gone from
`Mutation`, `MutationKind` and `CompositeMutationKind` — its checks live in the `🔺️diff` leaf as
messages, so a check can no longer be silently skipped. Enforced invariants: `Fatal ⇒ diff = D::default()`,
`Error ⇒ empty diff`, messages deterministic in `(op, base)`.

**Exactly seven message codes**, gate-enforced, no per-plugin invention:
`mutation.target-missing` (Error) · `mutation.no-op` (Warning) · `mutation.partial` (Warning) ·
`mutation.clamped` (Warning) · `mutation.duplicate-id` (Fatal) · `mutation.invariant` (Fatal) ·
`mutation.cascade` (Info).

**Merge policies.** `MergePolicy { LaissezFaire, Normal (default), Vigilant }` — LaissezFaire rejects only
Fatal, Normal rejects Error+Fatal, Vigilant rejects Warning+. A local dispatch that the policy rejects is
refused **atomically**: nothing is applied, nothing is recorded, and the caller gets a typed
`VcsError::Rejected { policy, messages }`.

**Conflicts are first-class.** `📡️spr/⚔️conflict/`: `Conflict { id, kind, status, messages, actors, timestamp }`
with `ConflictKind::{Quarantined { policy, envelopes }, Degraded { edit_ids }}`,
`ConflictStatus::{Open, Accepted, Discarded}`, `ConflictResolution::{Accept, Discard}`, plus `EditMessages`,
`DispatchReport`, `MergeReport`. Persisted in `.spr` (`HistoryConflict`, `HistoryOpMeta.messages`).

**Chronological merge.** `applied_edit_ids` is kept sorted by edit HLC. `ingest_remote` HLC-sorts an incoming
batch, splices it at its timestamp position, re-evaluates the whole affected suffix (including local edits that
the newly-inserted earlier edit invalidates), and applies the policy to the worst level over that suffix —
accepting (recording a `Degraded` conflict when work was lost) or **quarantining** the batch as an open
`Conflict` the user resolves with Accept (re-run under LaissezFaire) or Discard. Forward ops and
`MutationMeta` are never rewritten; only diffs, inverses and messages are recomputed.

The dev's worked example holds: A modifies a part, B deletes it earlier by timestamp ⇒ A's modification
yields `mutation.target-missing` with an empty diff; under Normal/Vigilant the merge is quarantined, under
LaissezFaire it applies and a `Degraded` conflict records what was dropped.

**The CRDT layer is gone**, per CLAUDE.md: `📡️spr/🔀️crdt/**`, `MergeStrategyKind`, `ConflictRule`,
`merge_concurrent_diffs`, `db_conflict::ResolutionPlan`, `Mutation::{merge_strategy, conflict_rule, reconcile}`,
`ReconcileReport`/`ReconcileSeverity`, `SpaceConflict`, `assert_crdt_*` — all deleted, zero residue.
`Severity` is now the single level vocabulary (`Hint` → `Info`, order reversed so `derive(Ord)` is the level order).

**End to end for all non-legacy tech**: kernel spine · `#[derive(Mutations)]` / `#[derive(CompositeMutation)]`
(both mirror files) · store · `.spr` · channel (`CHANNEL_VERSION` 10 → 11; `AppCommand` 30/31/32,
`AppFrame` 23/24, extended `Invocation`/`Error`) + TS mirror with matching tags · guest SDK · Rust wasmtime
host · TS browser host · React shell (Conflicts panel, rejected-dispatch toast, feed tones, history level
badges, merge-policy setting) with **en + de** strings · hub + db authority (`SubmitOptions.policy`,
outcome-gated pipeline rejecting before any WAL append).

## Verification actually executed

| Gate / suite | Result |
|---|---|
| `bun ./📜️script.ts verify mutation-outcome-law` (7 new gates) | **passed, 0 breaches** (was 182) |
| `cargo test -p semio-framework-os-kernel --lib` | **987 passed / 0 failed** |
| `cargo test -p semio-framework --lib` | **137 / 0** |
| `cargo test -p semio-framework-plugin-host` | **43 / 0** (incl. wasmtime e2e policy test) |
| `cargo test -p semio-framework-os-kernel-db --lib` | **424 / 0** |
| `cargo test -p semio-framework-os-run --lib` | **15 / 0** |
| Plugin crates `cargo check` | **21 / 33 clean**; the 29 errors in the other 12 are foreign (verified: zero mention this ticket's vocabulary) |
| wasm builds (`wasm32-wasip2`) | dag, cad, process3d + 4 extensions, trinity, writer — **0 errors** |
| TS | `framework-os` 334/336, `framework-renderer-react` 311/320, `ui-react` 515/525 — all deltas pre-existing; root `tsc` 19 errors, byte-identical to baseline |
| Residue greps | `SpaceConflict` 0 · CRDT vocabulary 0 · `Severity::Hint` 0 · unconverted diff leaves (non-gltf) 0 |

New gates: `policyMutationOutcomeBreaches`, `policyMutationMessageCodeBreaches`,
`policyNoCrdtVocabularyBreaches`, `policyNoValidateOverrideBreaches`, `policySeverityInfoBreaches`,
`policyMergePolicyParityBreaches`, `policyDeriveMirrorBreaches` — wired into `verify gate`, with
`.vscode/launch.json` entries and a `mutation-outcome-law` nx target.

## Real defects found and fixed (not just migration)

1. **Chronological determinism did not hold** (`📓️h1-determinism-fix.md`) — `ingest_remote` discarded only the
   arriving batch on rejection, so an already-committed edit that the rebase proved invalid stayed committed:
   whichever conflicting edit *arrived* first won. Second, `ConflictId`/`Conflict.timestamp` were hashed from
   the ticking store clock, leaking call order into conflict identity. Both fixed; the merge is now a pure
   function of the envelope set plus the policy.
2. **`WorkflowMutation::{RemoveNode,RemoveParameter,RemoveInput}::inverse`** returned its re-create op first,
   but inverses are applied reversed — so cascades were restored before the node existed (`📓️h2-…`).
3. **DSL value codec** — `deserialize_newtype_struct` blanket-forwarded to `deserialize_any` (newtype-wrapped
   scalars could not round-trip) and `deserialize_enum` silently dropped every tuple-variant field (`📓️i1-…`).
4. **Robustness** (`📓️j1-…`) — a ghost edit id was silently skipped during replay (losing an edit instead of
   failing); `ConflictId` could be minted from an empty id list; the conflict list grew unbounded under
   repeated rejected batches. All fixed with tests that fail without the fix.
5. **wasm-gated code was never compiled by any lane** (`📓️k1-…`) — 15 sites broken by this ticket's API
   changes, invisible to native `cargo check`. Fixed and verified with real wasm builds.
6. **The merge-policy control was wire-dead** (`📓️k2-…`) and **remote-origin merge reports were dropped**
   (`📓️l1-…`) — the collaborative half of the feature never reached the UI. Both wired, with tests.

## Known gaps (honest)

- **No live browser proof** (`📓️j2-runtime-proof.md`). Repeated dev-server boots died under contention from
  this ticket's own concurrent cargo lanes. Policy enforcement *is* proven at the host layer by a real
  wasmtime e2e test; the shell path is proven by vitest and code trace, not by a live click-through. Worth
  re-running once the tree is quiet.
- **12 plugin crates still fail `cargo check`** — 29 errors, all foreign, belonging to the live tickets
  `26/08/16/FULL-STDIO-…` (`DwgSnapshot`/`SvgSnapshot`/`Mp4Track` reshaping), the viewer/editor split
  (`SemanticMutation<…PlaySnapshot>`), and UI work (`UiNode`/`ui_wgpu`/`InteractionView`).
- **116 gltf diff leaves** under `🗄️stdio/🗿️artifacts/🧊️gltf/**` are a separate typed-sparse-operation
  architecture owned by `26/08/16/FULL-STDIO-…` and are documented in a shrink-only gate allowlist. They are
  not on the `Mutation::diff` path; the allowlist entry names the ticket so it can be shrunk when that lands.
- `AppFrame::MergeReport`/`Conflicts` reach the shell via the `ApplyEnvelopes` reply path; the hub-relay
  `backbone-worker.ts` protocol has no equivalent frame, so a hub-mediated peer conflict still needs a wire
  widening in a follow-up.
- Lane naming drifted from the master plan when the coordinator was resumed after a session limit
  (`w3-a/b/c` → `w3-norm`, `3-G` → `w3-fem-layout` / `w3-procedural-remodel` / `w3-note-shooting`).
  Audit 3 flagged these as "missing reports"; they are present under the resumed names.
