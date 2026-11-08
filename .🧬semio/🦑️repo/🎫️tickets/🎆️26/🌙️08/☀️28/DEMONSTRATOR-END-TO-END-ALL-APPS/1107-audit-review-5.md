# 1107 Audit review-5: store crates (read-only)

Scope: `semio-os-store-ledger`, `semio-os-store-space-history`, `semio-os-history-record-decode`, `semio-os-store-command`.
Method: foreground reading and grep only. No cargo, no tests run, no edits, no git modifying commands.
Paths below are relative to these roots:

- S = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store` (ledger = `📒️ledger`, space-history = `📜️space-history`, command = `📬️command`)
- D = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode` (record-set crate)
- V = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value` (work-accounting helpers)
- J = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job` (step driver helpers)

Coverage: read in full: ledger `🦀️.rs` cursor/revision section (lines 95-335), `♻️retirement`, `📥️lifted/♻️retirement`, `📥️lifted/🔗️read`, `📨️messages/admission.rs`, `📨️messages/🦀️.rs` retirement and pending-report sections, command `🛬️receiving/🪜️cursor`, command `♻️retirement`, space-history `🚦️native` (retirement and encode head). Grep-only: record-set decode crate, space-history `🛂️validation`, `🫴️receiving`, ledger publication and replay folders. Not read: ledger `🏁️publication/*` internals, `📨️messages/🔁️replay`.

## Findings

### 1. Quadratic revision-digest walk on every amend (ledger)
`S/📒️ledger/🦀️.rs:127`:
`for operation in edit.inverse.iter().skip(self.inverse)`

`edit.inverse` is a `PagedList` (protocol `Edit`, `🧰️framework/🔨️modules/📡️replication/🎮️mutation/✏️edit/🦀️.rs:225`). Its `PagedIter` (`V/📋️list/🦀️.rs:172-186`) overrides only `next` and `next_back`, not `nth`. `Skip` therefore calls the default `nth`, which steps `self.inverse` times through `owner.get`.

Failure: a typing run coalesced into one edit (the case the doc comment at `🦀️.rs:~150-160` names as "O(appended)") walks the whole already-hashed inverse prefix on each amend. For N keystrokes that is sum of i = O(N²) `get` calls on the UI-driven path. The doc claim of O(appended) is false for the inverse list. Forwards and meta use slice indexing (lines 124, 130) and are fine.
Fix: a positional iterator (`iter_from(k)`) or a stored tail cursor for `inverse`.
Confidence: high for the walk; unmeasured for the constant factor.

### 2. Unfunded destructive retirement in the production encode path (space-history)
`S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:80-83`:
`fn retire_projection(value) { let mut retirement = IntrinsicRetirement::new(value); while !retirement.terminal_is_empty() { retirement.close_step(256); } }`

It is the destructor passed to `DecodedValue::new` at `:314` (projection) and `:328` (`retire_record`). The return value of `close_step` is discarded, no effect or fuel is involved, and the whole tree is freed in one uninterruptible Drop. `IntrinsicRetirement`'s own `Drop` does the same (`V/🔁️codec/♻️retirement/🦀️.rs:51`, `close_step(256)` loop).

Failure: (a) the freeing work is recorded nowhere, so fuel and the work counter are charged zero for a tree of arbitrary size; (b) the constant 256-item grant is the "per-call constant standing in for a missing turn" that W2 forbids; (c) a large space-history projection cannot be paced or cancelled once dropped, which breaks the interaction-friendly rule.
Fix: make the projection a closable owner threaded with the native effect, or assert the projection is small and bounded before the drop.
Confidence: high that it is unfunded and constant-sized. Whether a real drop is large enough to matter is unverified.

### 3. Pending child turn reports zero work, then fails as a receiving invariant (command)
`S/📬️command/🛬️receiving/🪜️cursor/🦀️.rs:43`: `settled()` maps any `Pending` to `Err(invariant("original command close stayed pending ..."))` with `work: 0`.
It is used at `:228` inside `close_on_original`, where `receive_effect` runs the operation.

The Pending turn may already have performed effect work (the child reports it through `effect.performed`). With `work: 0` returned while `effect.performed_work() > 0`, `run_producer` (`V/🛬️decode/🏭️construction/📦️effect/🦀️.rs:46`) sets `work_reporting` to "receiving work report differs from its original performed receipt". The close therefore fails with a misleading invariant instead of returning Pending and resuming.
Work is still charged once, from the effect, so this is not a double charge. It is a wrong error and a lost resume.
Confidence: medium. I did not trace every consumer of `work_reporting`.

### 4. Dropped debit error in `resume_effect` (value, used by the decode path)
`V/🛬️decode/🏭️construction/📦️effect/🦀️.rs:111`:
`let _=effect.performed(u64::try_from(owed).unwrap_or(u64::MAX));`

If the owed work is refused (budget exhausted), the error is discarded and the turn still returns `turn.work`, which includes the owed amount. Compare `act` in the command cursor (`:105`), which turns the same refusal into `WorkLimit`.
Failure: the outcome claims work that the effect never recorded, so the one debit is short. This is "dropped validation" plus "unrecorded work".
Confidence: high on the code. I did not verify that `resume_effect` is reachable from the four crates. Outside the four crates, so flagged for the value owner.

### 5. Latent Pending-forever paths (job driver, outside the four crates)
- `J/♻️retirement/🎟️context/🦀️.rs:9-10` (`close_original_quoted_retained_step`): returns `Pending{progress: default}` with zero work when `fuel_remaining() < work` or any demand exceeds the remaining grant. Unlike `close_original_admission` (`:19-26`), it has no `retained_turn_untouched()` refusal. A quote larger than one turn's fuel therefore returns Pending every turn, forever.
- `V/🛬️decode/🏭️construction/📦️effect/🦀️.rs:186`: `receive_effect` returns `Pending{work:0}` when `prepare_work` returns `None`. Same spin shape if the quote never admits.
I found no caller of `close_original_quoted_retained_step` in the four store crates (grep of the store roots), so this is latent for other callers. The four crates themselves contain no confirmed `Pending{work:0}` spin.

### 6. "Not admitted" returns Progress with zero work instead of a typed refusal (W4 deviation)
Examples:
- `S/📒️ledger/🦀️.rs:98, 101` (`ArtifactStoreOperationRowsRetirement::close_step`)
- `S/📒️ledger/♻️retirement/🦀️.rs:38` (`release_empty`): a Forwards backing is released as one piece (`:42-44`), never chunked; `:98, 101` (operation rows)
- `S/📒️ledger/♻️retirement/🦀️.rs:217` (decoded edit)
- `S/📒️ledger/📥️lifted/🔗️read/🦀️.rs:59, 128, 132, 155` (edit retirement, history metadata; `:155` releases the sum of three Vec capacities in one piece)

If a caller re-polls with the same fixed grant, a single un-chunkable release that exceeds the per-turn release budget returns Progress with zero work forever. Under the job driver this is normally pre-empted: `close_original_admission` (`J/♻️retirement/🎟️context/🦀️.rs:22-24`) refuses with WorkLimit when a demand cannot fit an untouched turn, so the inner branch is reached only from fixed-grant drivers (`envelope_close_turn`, `AuthoringWallet`).
Confidence: medium. This is a deviation from W2 and W4 ("chunk, do not cap", "typed refusal, never Pending forever"). It is not a confirmed spin under the job driver.

### 7. Drop assertions that panic on non-terminal drops (retirement leak risk)
- `S/📒️ledger/♻️retirement/🦀️.rs:140` (operation rows) and `:262` (decoded edit)
- `S/📒️ledger/🦀️.rs:508-512` (pending report)
- `S/📒️ledger/📥️lifted/🔗️read/🦀️.rs:72-76` (edit retirement) and `:171-175` (history metadata)
- `S/📬️command/🛬️receiving/🪜️cursor/🦀️.rs:286`

Each `assert!` fires if the owner is dropped before its terminal state, unless the thread is already panicking. The decoded-edit close error path (`♻️retirement/🦀️.rs:248-251`) returns `Err` while the owner keeps its state. A caller that drops the owner on that error without reclaiming it aborts.
Failure: an error or cancellation path that drops a half-closed owner panics instead of retaining it.
Confidence: the asserts are certain. Whether any caller drops early is unverified.

### 8. Diagnostic admission candidate stays wedged after an error (ledger messages, low)
`S/📒️ledger/📨️messages/admission.rs:94-95`: on `advance` error the candidate is restored to `original_admission`. `original_append_pending()` stays true, and every ledger mutator returns None or Err while it is set (`S/📒️ledger/📨️messages/🦀️.rs:417, 423, 437, 444, 482, 490, 498`). The only release is retirement (`:147`). The test at `admission.rs:136` asserts this by design.
The hydration caller (`S/🧾️document/📜️history/💧️hydration/🚶️hydration-step/✉️edit-decoding/🦀️.rs:255-264`) rejects and returns, so there is no spin. The ledger stays unusable until retired.
Confidence: high on behaviour, low on severity.

### 9. Rule deviation outside the four crates (note only)
`S/🧾️document/📜️history/💧️hydration/🚶️hydration-step/✉️edit-decoding/🦀️.rs:256-257` debits by hand (`cx.consume_retained(progress)`, `cx.consume_fuel(turn.work)`) instead of `receive_original_retained_work` or `with_original_effect`. It is not a double charge, but it bypasses the single-debit primitive the brief mandates.

### 10. Unchecked invariants on reachable-looking paths (unverified)
- `S/📒️ledger/🦀️.rs:252, 271, 299` (`edits.find_near(..).expect("validated cursor edit exists")`)
- `S/📒️ledger/📥️lifted/🔗️read/🦀️.rs:44` (`into_original().ok().expect("settled original inverse")`)

These panic if the "validated" invariant is broken by a replay or merge path. I did not trace replay to confirm.

## Checked and not faulted
- Work settlement in `RetirementPollExit::settle` (`V/♻️retirement/🧬️contract/🦀️.rs:19-27`): a progressing turn with no recorded work pays exactly one unit, and the poll carries `effect.performed_work()`. No double charge seen in the retirement closes I read.
- Depth decrements (`grant.maximum_depth - 1`) in the retirement closes are preceded by admission checks that imply depth ≥ 1. The decode `frame_step` (`D/🗂️file/🦀️.rs:68`) decrement I did not confirm against its caller.
- No confirmed `Pending{work:0}` loop and no leak inside the four crates.

## Top finding
Quadratic inverse-digest walk on every amend: `S/📒️ledger/🦀️.rs:127` (`edit.inverse.iter().skip(self.inverse)`), which makes a coalesced typing run O(N²) and contradicts its own O(appended) doc.
