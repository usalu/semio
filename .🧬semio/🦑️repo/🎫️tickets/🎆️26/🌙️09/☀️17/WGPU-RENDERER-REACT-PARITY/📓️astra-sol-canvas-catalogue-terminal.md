# Canvas Catalogue Terminal Ownership

Implementation record on 2026-09-20. This packet follows `📓️terra-layout-catalogue-boundary.md` and keeps catalogue parsing in the app while making the generic Canvas2d producer own a complete terminal lifecycle.

## Shared contract

The language-neutral schema and fixture are:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🛒️canvas-catalogue-terminal/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🛒️canvas-catalogue-terminal/🔣️.json`

They require:

- valid and malformed nonempty catalogue data to publish `canvasDragLeave`, then the unchanged raw `canvasDrop`;
- empty catalogue data to publish leave only;
- a foreign drop to publish nothing and retain any existing catalogue hover owner;
- a complete two-action source batch to be refused at the downstream frame owner when only one of 256 slots remains.

The schema is strict. It does not add an optional compatibility field, infer an app kind, or parse Layout JSON in the renderer.

## React producer

`Canvas2dHost` now snapshots the catalogue raw value, local coordinates, and mounted surface size synchronously during the browser `drop` event. It clears its local hover projection, awaits `canvasDragLeave`, and dispatches a nonempty raw `canvasDrop` in `finally`. A leave refusal therefore remains observable but cannot swallow a valid drop. A malformed nonempty value reaches the app unchanged and is refused there after the preview has retired. Foreign MIME data remains inert.

The mounted React contract test validates the shared fixture with Ajv and uses a controlled leave promise to prove that drop has not started before leave settles. It also covers leave rejection, empty data, malformed data, foreign data, and the existing renderer-shaped Layout envelopes.

Focused command:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run '<Canvas2dHost input-contract test>' --config '<renderer-react vitest config>'
```

Receipt: 17/17 passed. The saved output is `🗑️generated/astra-catalogue-terminal/react-focused.log`.

## WGPU source owner

`canvas_catalogue_drop_into` now reserves one complete `[leave, drop]` action slice before publication. The hover owner clears only in the successful publication commit. Empty or out-of-bounds data uses the existing leave-only terminal. A missing base catalogue MIME is treated as a foreign drop and does not cancel the current hover owner.

The actual retained Shell law now requires the normal action sequence `[canvasDragOver, canvasDragLeave, canvasDrop]`. The Scene law fills 255 action-queue slots, proves a two-action terminal reservation refuses without publishing either member, drains the public FIFO front, and then proves the pair publishes in order and clears the hover owner. It does not inspect private queue storage.

These Rust laws await the root-owned native census. No native, wasm, or browser runtime result is claimed here.

## Downstream frame boundary

The source action queue preserves admission of the pair, but the existing frame transaction historically popped and admitted one action at a time. A production collection helper now exposes that exact old seam without changing behavior. Its fail-first law:

1. fills `FrameActionOwners` to 255/256;
2. publishes one real two-action `InputState::reserve_actions` batch;
3. invokes the production collection helper;
4. requires whole-slice refusal before `canvasDragLeave` leaves the source queue.

Native43 produced the required red receipt at `catalogue_terminal_pair_never_partially_enters_the_frame_action_owner`: the old helper transferred the first member instead of refusing the complete pair. The same census also exposed an invalid two-byte reservation in the Scene pressure fixture before that fixture could exercise one-slot pressure. The fixture now reserves the exact controller/action string bytes without changing any queue ceiling.

The reviewed repair design remains progressive. A one-byte countdown on each bounded source action records the remaining members of its explicit reservation batch. Singles carry one and batches carry `N..1`; front removal therefore exposes the remaining tail without a scan or semantic inference. The runtime-owned frame-action authority reserves the complete target slice, then moves and materializes only one source action per frame opportunity into a fixed 16-slot staged owner. The last step publishes the already staged descriptors FIFO as one complete slice. Because the staged owner belongs to the runtime rather than the discardable frame transaction, a superseded frame resumes it and cannot lose the members it already removed from input.

Capacity refusal is transient: with 255 published target actions and a two-action source batch, no source member moves, the current frame can install its 255 earlier actions, and a later frame retries the intact pair. An internal materialization or countdown fault retires at most one staged descriptor per subsequent step and never publishes a prefix. The exact Rust layout check compares the marked action to a test-only copy of the pre-marker fields on the active target; a standalone compiler measurement also observed 48 bytes before and after the marker on the current 64-bit host (`🗑️generated/astra-catalogue-terminal/action-layout.txt`).

The implementation is now complete at that boundary:

- `BoundedAction` carries the explicit reservation countdown without growing its measured target layout;
- ordinary, partial, and detached-claim batches retain the same grouping, while single actions remain one-member batches;
- front inspection validates the complete countdown before a renderer transfer; close/pop-back and token-addressed removal shorten a surviving tail without joining its neighbouring batch;
- `FrameActionOwners` holds one fixed 16-slot staged owner in runtime state, so a discardable frame cannot lose a materialized prefix;
- one transfer opportunity materializes one descriptor; the last opportunity publishes the staged slice FIFO into the 256-slot frame ledger;
- 255/256 pressure returns `Deferred`, leaves both source actions intact, and lets the prior 255 remain drainable;
- a later single appended while a pair is staged stays behind that pair;
- an internal fault and a real close retire at most one staged descriptor per grant, followed by the empty reservation;
- `FrameFinish` advances an already-live deferred cursor rather than faulting or replacing it, so the older owner drains before the new ledger installs;
- the temporary full action-arguments debug serialization was removed from the hot transfer path.

Root's UI56 census passed 645/645 with zero skips. That receipt includes the queue countdown, removal, detached-batch, fault progression, and exact pre-marker layout laws.

Native45 then executed the production-seam fault law and produced the intended second red receipt. After `canvasDragLeave` had moved into the staged owner, an independently recorded `BoundedActionFault::ByteCredits` caused the staged prefix to retire, but the next collection published the orphaned `canvasDrop`; the assertion observed `canvasDrop` where the later independent action was required.

The staged owner now retains an exact `source_remaining` witness. Fault cleanup first retires one staged descriptor per grant, then validates that the source head's reservation countdown equals that witness and retires one source member per grant. Only after both halves of the failed batch are empty does it expose the original fault. A later single stays behind the declared tail and becomes dispatchable after the error is observed. `expected`, staged `len`, and `source_remaining` use `u8`, matching the fixed 16-item ceiling without growing the earlier usize-based batch metadata.

Root's Native46 focused run executed the repaired law successfully. The run selected four laws: the fault-tail law and the live wrong-generation control passed; two unrelated retained-page/Actions diagnostics remained red. The receipt is `🗑️generated/astra-runtime/native-renderer-46.log`. This is direct proof that the production transfer seam no longer publishes the orphaned drop after a staged-prefix fault.

Root's later Native48 full renderer census passed 1,231/1,231 with zero skips. That complete receipt covers source admission, target capacity refusal, staged publication, deferred-owner ordering, source-fault tail retirement, real close, and the canvas terminal laws in one native binary.

## Validation boundary

Verified locally:

- strict shared fixture and schema source;
- React mounted/controlled-promise oracle: 17/17 passed;
- static WGPU ownership and exact source-batch law construction.

Pending root-owned gates:

- fresh activated React/WGPU browser acceptance.
