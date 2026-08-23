# Sol Independent P4d Retained Fill-worker Envelope Audit — 2026-08-23

## Verdict

**REJECT — source-only.**

The packet removes the two UI callers' whole-state serde bridge and replaces it with a fixed
56-byte generation token. Its worker decode and drive boundaries are materially better. The live
authority still does not satisfy exact admission or terminal ownership:

1. production passes the declared maximum item/byte values into `reserve` without measuring or
   structurally bounding the retained `FillBuilder` graph; and
2. the public terminal handle is never taken or closed by production, while session Drop only
   cancels a token and cannot advance the slot to terminal-empty.

In addition, the claimed one-root close can be the final `Arc<Mutex<FillBuilder>>` release and thus
ordinary-drop the complete dynamic planner graph in one close grant. The 13 verifier mutations do
not reject any of these live defects. P4d and Phase 4 remain RED.

No production source was edited by this audit. No Cargo, Nx, Wasm, browser, runtime, or network gate
was run.

## Evidence read

- `📓️sol-read-only-p4-puzzle3d-vertical-slice-status-gap-audit-2026-08-23.md`
- `📓️p4d-retained-fill-worker-envelope-implementation-2026-08-23.md`
- the complete production and test regions of the Puzzle 3D precompute envelope and `FillBuilder`
- both fill-build action functions
- the full P4d verifier predicate and all 13 mutation reconstructions in root `📜️script.ts`
- scoped and whole working-tree diffs

## Exact live census

The production-only census truncates each Rust file at its first `#[cfg(test)]` boundary.

| Route | Production result |
|---|---|
| `.enqueue_fill_job()` | **Exactly 2**, both in `fill-build-tick/🦀️component.rs`, ordinary and cached |
| `.take_terminal_fill_job()` | **0** |
| `FillWorkerState` | **0** |
| `restore_fill_worker_state` | **0** |
| `fill_worker_checkpoint_bytes` | **0** |
| whole `serde_json::{to_vec,from_slice}` in the live precompute envelope | **0** |
| `drive_step` in `drive_fill_envelope` | **Exactly 1** |

The old full checkpoint methods remain on `FillBuilder`, with source-test callers only. They are not
reachable from the new worker envelope. The scoped `FillBuilder` diff changes checkpoint, preview,
and completion outputs to empty vectors; it does not alter collision formulas or ordering.

## Properties that pass

- Fixed constants are present: four slots, 16 KiB pages, 256 pages/4 MiB per operation, 65,536
  claimed items per operation, and 16 MiB aggregate claimed bytes.
- The token is exactly 56 bytes: magic/header, slot, registry generation, job, operation,
  generation, and base revision. Slot epochs reject stale ABA tokens and never wrap.
- The UI callers no longer clone/serde the scene, mesh catalog, checkpoint, or fill state. A failed
  slot/declared-credit reservation returns the same `Arc` pointer to the session.
- `FillEnvelopeTokenCursor` advances one token field after each `JobCtx::tick`; the first fill drive
  occurs on a later tick.
- `drive_fill_envelope` checks cancellation and operation/generation/base revision before its one
  `drive_step` call. Lock contention is reported as Blocked rather than spinning.
- Direct fixtures cover four slots/+1, declared item/+1, declared byte/+1, pointer identity, slot
  reuse ABA, one-field token decode, checked-out terminal-handle Drop handback, and fixed-token
  checkpoint shape.
- The public handle exposes reason, take, resume, close step, and terminal witness.

These passing source properties do not establish that the retained planner fits the credits or that
the mounted product returns those credits.

## Blocking findings

### 1. The item and byte admission is a constant claim, not an exact source admission

`enqueue_fill_job` calls:

```rust
registry.reserve(
    job,
    operation,
    FILL_ENVELOPE_MAX_ITEMS,
    FILL_ENVELOPE_MAX_BYTES,
    fill,
    ...,
)
```

No production cursor counts or reserves the retained builder's actual items, string capacities,
mesh/index pages, or bytes. `reserve` consequently proves only that the literal constants fit their
own constants.

The moved `FillBuilder` remains a large dynamic graph containing Fixtures, many `Vec`s,
`BTreeMap`s, `BTreeSet`s, `HashMap`s, `HashSet`s, nested Strings, meshes, spatial-index owners,
preview arrays, candidates, and collision state. Those owners are neither born under the 65,536/
4 MiB ledger nor validated against it before transfer. A builder above either advertised cap is
accepted exactly like an empty builder. Four such owners can exceed the claimed 16 MiB process
authority while `aggregate_bytes` still reports exactly 16 MiB.

The direct +1 fixture calls `FillEnvelopeRegistry::reserve` with synthetic numeric arguments. It
does not construct a cap or cap+1 builder and therefore cannot discriminate this defect.

### 2. Terminal take/close is test-only; the mounted route strands every completed slot

The exact production census finds zero calls to `take_terminal_fill_job`. Both UI actions only
`poll_fill_job` then call `enqueue_fill_job`. When a request exists, `enqueue_fill_job` sees an
observation and returns `None`, including when that observation is terminal; it never transfers the
terminal owner to a close pump.

`Puzzle3dPrecomputeSession::Drop` calls only `fill_cancel.cancel_now()`. It does not drive the
cancelled authority, acquire the public handle, or pump `close_step`. If the worker future has
already ended or is dropped, no remaining owner is guaranteed to observe that cancellation. The
static four-slot registry and all claimed credits can therefore remain occupied indefinitely. Four
completed documents can make the fifth production operation permanently unavailable.

Malformed token decode, restored-token mismatch, and failure while reacquiring the token after a
nonterminal drive also return a worker `Fault` without a retained worker guard that transitions the
matching slot into `Terminal(Fault)` and schedules its close.

### 3. `close_step` can ordinary-drop the complete planner graph

Close cursor zero executes `authority.fill.take()` and lets the returned `Arc` drop immediately.
The public API permits a terminal handle to outlive its session. If the session is dropped before
cursor zero, that take is the final `Arc` and recursively destroys all dynamic `FillBuilder`
collections in one grant. If the session remains live, closing the registry merely moves the same
unbounded final destruction to the later ordinary session Drop.

The final close grant also takes and drops the entire authority shell while decrementing byte
credit, releasing the checked-out `Arc<AtomicBool>`, request shell, and remaining scalars together.
Thus the implementation does not prove one owner/scalar per close grant or terminal-shallow Drop.

### 4. The permanent evidence accepts the three defects

All 13 mutations are reconstructed from live text: page cap, dynamic slots, missing byte check,
clone-before-admission, whole serde, whole-token decode, missing base freshness, a second drive,
missing terminal take, missing handle Drop, missing identity fixture, whole preview, and one lost UI
caller.

The predicate does not require:

- actual item/byte accounting from the retained builder before `reserve`;
- a cap/+1 builder fixture rather than synthetic requested counts;
- a production terminal take/close pump or realm/session terminal witness;
- a retained fault guard for early worker exit; or
- cursorized retirement of the builder's nested owners and a terminal-shallow authority Drop.

The verifier therefore reports no P4d-specific failure for an implementation whose mounted slots
never close and whose advertised credits do not describe their owners.

## Gates rerun

| Gate | Result |
|---|---|
| Rust-2021 `rustfmt --check` on precompute, FillBuilder, and both action functions' file | PASS |
| Bun TypeScript parse/transpile of root `📜️script.ts` | PASS |
| 13 P4d mutations via `verify interactivity --self-test` | P4d mutations completed without a P4d predicate error; command later RED on two concurrent P1 DB predicates |
| broad `verify interactivity --plain --deny` | RED only on the same two concurrent P1 DB predicates; no P4d finding |
| exact production caller/old-bridge census | PASS for the counts recorded above |
| scoped and whole `git diff --check` | PASS |

The concurrent broad-gate failures are not attributed to P4d and do not change this independent
P4d rejection.

## Exact repair packet

1. Replace the literal maximum arguments with a schema-first retained admission/build cursor. Count
   and reserve every actual builder/page/string/item capacity before the registry takes ownership;
   refuse cap+1 with exact source handback. Alternatively, make `FillBuilder` itself a fixed paged
   authority whose construction already proves those caps. Do not scan an already-unbounded graph
   in one UI call.
2. Mount terminal processing on the live action/app/worker lifecycle. Completion, cancellation,
   stale input, malformed restore, worker fault, app/session Drop, and realm close must transfer the
   exact token/slot/builder into a retained close pump. Production must call take/resume/close and
   prove all four slots plus aggregate credits terminal-empty.
3. Replace `authority.fill.take()` ordinary destruction with a retained nested retirement cursor,
   one page/string/map entry/root or scalar per grant. The terminal authority/session Drop must be
   definitionally shallow and assert the close witness.
4. Add mounted fixtures for an actual 65,536-item/4 MiB authority and +1 source handback, four
   completed operations followed by a fifth successful admission, early-fault and session-drop
   close, interrupted close, final-owner pointer identity, and exact terminal-empty/credit-zero.
   Extend the permanent mutation matrix to remove each live admission and close seam independently.

After source repair, repeat this source audit. Build/native/Wasm/browser/runtime evidence remains a
separate mandatory gate when authorized.
