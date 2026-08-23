# P4d Retained Fill-worker Envelope Implementation — 2026-08-23

## Pre-edit Reachability and Cap Census

The live fill action has exactly two production entry routes:

1. editor action dispatch at `✏️editor/🦀️component.rs:2326` calls `fill_build_tick`;
2. cached `ArtifactApp::handle` at `✏️editor/🦀️component.rs:2419` calls `fill_build_tick_cached`.

Both action functions call `Puzzle3dPrecomputeSession::enqueue_fill_job`. Before P4d that method called `fill_worker_checkpoint_bytes`, which cloned the request, scene, mesh sources, fill checkpoint, observation, and last checkpoint; sorted a collected mesh `Vec`; serialized the complete state with `serde_json::to_vec`; and only then rejected output above 4 MiB. Thus both UI routes reached whole clone/materialization/serialization before `Effect::SpawnJob`.

Pre-edit explicit limits were 1,000 accepted placements, a post-encode 4 MiB worker envelope, 64 meshes, 196,608 values per positions/indices vector, 393,216 aggregate mesh values, 4 KiB URL, and 4,096 cells per spatial entry. There was no pre-serialization operation/item/process-byte reservation, no <=16 KiB page owner, and no public rejected/terminal take/resume/one-owner close path.

## Implementation Status

Rejection remediation source-audit-ready; not independently accepted.

## 2026-08-23 Independent Rejection Remediation

The first independent source audit rejected four ownership seams. The remediation replaces the
literal maximum reservation with a retained `FillBuilderOwnerCensusCursor`. A session action tick
measures one explicit builder field class, accumulates checked item/backing/page credit, and leaves
the exact `Arc<Mutex<FillBuilder>>` in the admission cursor until the census completes. Only the
measured `credit.items` and `credit.bytes` are passed to the four-slot registry; cap failure restores
the identical source authority before registry mutation.

The existing two UI callers now mount terminal handling through `poll_fill_job`. The session keeps a
strong `FillEnvelopeTerminalHandle`, releases its shallow live builder ticket, and advances one
`close_step` per later tick. A dropped session marks its admitted authority Closed; another mounted
session can take that fixed-slot orphan and continue the same close cursor. Worker exits after token
admission are guarded by `FillEnvelopeWorkerFaultGuard`, so restore mismatch, stale ownership, or
token retrieval faults transition the matching generation to observable terminal fault ownership.

Close no longer discards `authority.fill.take()` as an ordinary final `Arc` drop. It waits for the
session ticket to return, unwraps the last builder authority, and installs a
`FillBuilderRetirementCursor`. Each close grant removes at most one retained builder field/root; the
fixed token page, cancel owner, item scalar, slot shell, and aggregate byte release remain later
separate grants. The terminal witness becomes true only after the slot has been removed and its byte
credit returned.

New discriminating fixtures cover an actual populated owner census cap/+1 with exact pointer
handback, production-mounted terminal close and capacity re-arm, early fault terminalization, and an
interrupted deep retirement that requires multiple grants before terminal-empty. The permanent
verifier now rejects literal maximum credit, an unmounted terminal pump, a missing worker fault
guard, a bulk builder drop, and removal of each new fixture.

### Remediation gates

- Rust 2021 formatting and scoped `rustfmt --check` over the precompute envelope, FillBuilder owner
  cursor, collision-index retirement helper, and the two-caller action file: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS; DENY clean, including the expanded P4d
  mutations.
- `bun 📜️script.ts verify interactivity`: PASS; DENY clean with the existing one recorded test-only
  allowlist finding and zero unlisted findings.
- Production scans: exactly two `.enqueue_fill_job()` action calls; mounted production terminal
  take/close is present; 56-byte token remains; no old `FillWorkerState`, restore bridge, envelope
  serde, blocking wait, runtime, or pool construction was introduced. The remaining FillBuilder
  serde matches are dormant checkpoint/test facilities outside the live envelope route.
- Scoped and whole working, staged, and `HEAD` `git diff --check`: PASS with no output.

Cargo, Nx, Wasm, browser, runtime, network, and root lint were not run. This packet is returned for
independent source audit only; Phase 4 remains RED.

## Implemented Source Boundary

- `⏳️precompute/🦀️component.rs` replaces `FillWorkerState` and its scene/mesh/checkpoint clones, sort, full JSON encode, full JSON decode, and isolated-session reconstruction with a process-wide fixed registry.
- The registry has four generation-keyed slots. Each operation reserves at most 65,536 semantic items and 256 pages of 16 KiB (4 MiB); process aggregate credit is 16 MiB. Item, operation, operation-byte, and process-byte rejection occurs before the source `Arc<Mutex<FillBuilder>>` is transferred.
- `enqueue_fill_job` takes the exact source authority by value from the session. On contention or admission rejection it restores that exact `Arc` synchronously; after successful admission the registry owns the transferred authority and the session receives only a shallow ticket to the same pointer.
- The UI-visible job input/checkpoint is a fixed 56-byte identity token carrying slot, registry generation, job, operation, generation, and base revision. It contains no scene, mesh, preview, fill plan, or checkpoint payload.
- `FillEnvelopeTokenCursor` decodes one fixed token field after each `JobCtx::tick` grant. Once decoded, each later grant calls exactly one `drive_step`; cancellation and operation/base-revision/generation checks precede FillBuilder mutation.
- The checkpoint-facing FillBuilder publication returns empty state/output bytes. The retained shared FillBuilder is the source of preview, progress, and final state, so no whole preview/checkpoint/result is encoded by a worker turn.
- Terminal completion, cancellation, and fault remain in the generation slot. `take_terminal_fill_job` returns a public handle with an observable reason, `resume`, atomic checked-out Drop handback, `close_step`, and `terminal_is_empty`. Close releases one shallow FillBuilder owner, cancel owner, fixed page owner, item-credit scalar, and final slot/byte credit on separate grants.
- Slot generations never wrap: `u64::MAX` permanently exhausts that slot. A closed slot is re-armed only by a later reservation; stale tokens cannot observe or mutate the replacement authority.

No compatibility synchronous bridge was retained. The two action files were not edited; their existing `Option` result receives synchronous exact owner handback on `None` and submits the fixed token on `Some`.

## Direct Fixtures

The precompute component now contains discriminating fixtures for:

- exact source pointer identity across admission and token restore;
- one retained worker opportunity;
- the four-operation boundary, fifth-operation rejection, close-driven capacity re-arm, and stale-generation ABA rejection;
- item cap + 1 and byte cap + 1 with exact pointer handback;
- checked-out terminal Drop handback, observable cancellation reason, and one-owner close;
- one-field-per-grant token decode; and
- proof that job checkpoint bytes are the fixed token rather than whole-scene JSON.

The FillBuilder fixture was updated to assert shared preview publication without serialization.

## Permanent Verifier

The existing root `📜️script.ts` interactivity verifier now reads the precompute component, checkpoint-facing FillBuilder region, and the one action file containing both call sites. Its permanent predicate denies changed/dynamic caps, clone-before-admission, missing item/byte preflight, whole serde, whole-token decode, missing base freshness, more than one drive opportunity, missing terminal retrieval/Drop handback, missing fixtures, whole preview publication, or a caller census other than exactly two. Thirteen faithful mutations are reconstructed from live source and all are rejected before the baseline is evaluated.

## Permitted Gates

- `rustfmt --edition 2021 --check` on both Rust files: PASS.
- Bun TypeScript parser over root `📜️script.ts`: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS, DENY clean. The permanent P4d mutations and baseline ran.
- `bun 📜️script.ts verify interactivity`: PASS, DENY clean.
- Production source scans: zero `FillWorkerState`, `restore_fill_worker_state`, or `fill_worker_checkpoint_bytes`; no production whole serde/block_on/pool construction in the envelope path; exactly two `enqueue_fill_job()` action call sites; fixed empty FillBuilder preview/complete publications present.
- Scoped and whole working-tree, staged, and `HEAD` `git diff --check`: PASS with no output.

Cargo, Nx, Wasm, browser, runtime, network, and root lint were not run, as required.

## Honest Residuals

- P4d owns only the fill-worker envelope. It does not claim that existing FillBuilder collision/narrow-phase work has been accepted under P4b; a `FillBuilder::step` remains the indivisible semantic opportunity behind this envelope.
- Collision/index/preview renderer math and files were not changed.
- Compile/runtime validation was intentionally not run, so this is a source-only packet awaiting independent audit.
- The broader Phase 4 vertical slice remains RED for the collision/index/preview packets listed in the preceding Sol status-gap audit. No ticket or phase acceptance is claimed here.
