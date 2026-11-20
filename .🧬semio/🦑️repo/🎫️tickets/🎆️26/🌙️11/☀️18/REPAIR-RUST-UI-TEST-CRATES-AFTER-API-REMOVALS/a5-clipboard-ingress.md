# 📋️ a5 Clipboard Ingress, Physical Job Close and Prepared Laws

## Scope

- `semio-ui-wgpu-laws`, `--test original_clipboard_ingress` (`🖱️ui/🧪️tests/📋️clipboard-ingress/🦀️.rs`)
- `semio-ui-wgpu-engine-scene-laws` clipboard laws (`🖱️ui/🧪️tests/♻️physical-job-close/🦀️.rs`), plus their copies in `semio-ui-host-tests` (`🖱️ui/🖥️host/🧪️tests/🔬️window-unit/🦀️.rs`)
- `semio-ui-prepared-laws` (`🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-laws`, which mounts `♻️physical-job-close/🎟️prepared/🦀️.rs`)

## Root causes

### R1. Close turns paid the host's Source origin refresh (`original_clipboard_read_fault_…`)

- `OriginalJobFixtureCaller::turn` serves the resident return birth driver before its callback runs.
- When the worker session was admitted, it set `birth_requested`. That flag is sticky.
- From then on, every served turn whose grant affords it pays an origin refresh of 9 items and 6064 copy bytes.
- The law closed with `stock()`, so `close_owner` saw the refresh charge in `receipt()` and zero in the close step.
- The reference laws close with a per-phase policy of one item, which can never afford the refresh.
- Fix: the fixture gained a `close` policy `{items 1, copy 65536, capacity 65536, release 65536, depth 64}`.
  - The capacity is needed because the session's `Payload` phase quotes 392 capacity bytes.
  - The law closes with this policy, and receipt == step and heap == receipt still hold.

### R2. An already cancelled operation never enters its producer (all "cancelled" clipboard laws)

- This is the documented core contract: `drive_step` "pre-checks cancellation so an already-cancelled operation never even enters the job" (`🧵️job/⏱️context/README.md`).
- It is enforced by:
  - `OriginalJobReturnOrigin::admit`;
  - `OriginalJobReturnReceiver::prepare` and `enter_body`;
  - worker reservation `admit_step`;
  - session admission `enter`.
- The job-core law `📬️outcome/⏸️continuation/🧪️tests/🎟️origin` asserts this explicitly, with zero effect, even for an already ready origin.
- The UI laws expected a `Cancelled` descriptor after a root cancel, which can never arrive.
- `ClipboardIoJob`'s `Cancelled` branch is still correct, for cancellation observed mid-body.
- Making cancelled outcomes claimable would mean doing forward work after cancellation, against that contract and its law.
- So the job core was **not** changed. The laws were ported to the contract instead:
  - `original_clipboard_admission_denial_preserves_text_and_cancelled_write_never_enters_its_producer`: after cancel, one step turn gives `Outcome` with no descriptor, a zero receipt and zero heap, and the text pointer and capacity are kept. Then a paid close.
  - `cancelled_clipboard_input_never_enters_its_producer_and_is_held_until_paid_close` (physical and window-unit): a cancelled caller's origin admit refuses with zero receipt and zero heap for both the denied and the supplied grant. Input is held, then a paid close.
  - `cancelled_clipboard_driver_never_enters_an_admitted_origin` (physical and window-unit): the origin is admitted before the cancel. After it, admit refuses and `drive_step` returns `Preparing` with zero receipt and zero heap; no outcome or continuation. Input is held, then both paid closes.
  - Fixture `clipboardOutcome`: `outcome` is now `never-entered` and `refusedHeapBytes` is 0. `borrowHeapBytes` and `acknowledgementItems` were removed (no TS twin reads them).
  - `prepared` `observe_prepared_step` cancels after the origin is ready. `preparation_observes_cancellation_without_replacing_a_packet` now asserts no outcome and a zero receipt.

### R3. Unreceipted heap in worker session admission (`original_clipboard_session_admission_…`)

- Stale `[DEBUG]` probes from 26/11/10 (`debug_probe` with a thread-local `HashMap` plus `format!`) allocated inside admission turns. They were already noted in the GET-NORM-PLUGIN ticket as noise to strip.
- Removed them, restoring the plain functions:
  - `🧵️job/⏱️context/🦀️.rs` (`debug_probe`);
  - `📬️outcome/⏸️continuation/🎟️origin/🦀️.rs` (`admit`, `debug_state`);
  - `📬️outcome/⏸️continuation/🎟️source/🦀️.rs` (`admit_original_return_table`, `claim_original_sized_return`);
  - `📬️outcome/👷️worker/🦀️.rs` (`admit_step`);
  - `🛍️products/💻️os/…/🔌️plugin/👷️job/🚪️session/🦀️.rs` (`admit_pending`);
  - `🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs`.
- After that, the law still could not finish admission, because its bare turns never serve the return-table birth. It now calls `admit_return_table()` and runs the admission and session close loops on `refreshed_turn`.
- Heap == whole-turn receipt is kept.
- The session close uses a new fixture policy `sessionClose` (`{items 1, copy 65536, capacity 65536, release 65536, depth 64}`). It replaces the 4096-copy semantic loan, which could never fund the 18160-byte `Payload` close.

### R4. Law harness slips found on the way

- `ClipboardCaller::finish` kept a clone of the root token, so the root close was `Blocked(SharedAlias)`. It now drops the clone before retiring.
- Close-turn receipts compared against `cx.retained_progress()`, which is the caller's cumulative spend. They now compare against the turn's own delta.
- `🎟️prepared/🦀️.rs` used `fixture_text!` with a path relative to another crate's manifest. It now uses file-relative `include_str!`.
- `prepared_close_gate::<ValueError>` is now annotated.

### R5. Production bug: `PreparedRenderJob::close_step`

- It ran the effect admission before checking its own state.
- A not-yet-closing job, or an already terminal one, therefore reported `Pending{0}` under a non-fitting grant, instead of `Blocked` or `Complete{0}`.
- It now checks `closing` and `terminal_is_empty` first, in the same order as `close_granted`.

### Prepared outcome law port

- `prepared_outcome_loans_preserve_original_custody_and_paid_descriptor_acknowledgement` now steps through `OriginalJobReturnOrigin` + `drive_step` under a `close_source` caller.
- Kept from the original law:
  - the denied grant posts the fixture `deniedReceipt`;
  - Complete and Fault loans are borrowed against the job;
  - the first `acknowledge_outcome` turn charges `acknowledgementItems` with zero heap;
  - the second turn removes the descriptor;
  - the origin and the job get paid closes.
- Each drive turn's business receipt is bounded by the policy plus the origin refresh quote, the same bound as the `observe_prepared_step` helper.

## Results (latest logs)

| Crate | Result |
|---|---|
| `semio-ui-wgpu-laws` `original_clipboard_ingress` (`a5-clipboard-3`) | **2/2 pass** |
| `semio-ui-wgpu-engine-scene-laws` `--lib clipboard` (`a5-scene-7`) | **4/4 pass**. The admission law also had to drop its leftover root clones (`witness`) before retiring. |
| `semio-ui-prepared-laws` (`a5-prepared-3`) | 33/38; open items below |
| `semio-ui-host-tests` `--lib clipboard` (`a5-host-2`) | Every clipboard law there fails inside `OriginalJobFixtureCaller::admit` (`authority_actual None` vs `Some(0)`), including laws I did not touch. That crate's harness does not feed `JobPayloadAuthority::admit`'s allocation witness. It is pre-existing and outside this port. |
| `semio-framework-job-worker` (`a5-worker-1`) | 7 pass, 23 fail |

On the worker crate failures:

- Most are "original owner retained beyond declared finite close control" or session-close asserts.
- My only change in that crate's dependency graph is removing the debug probes, which does not change behaviour.
- Other agents changed the job core concurrently:
  - `📬️outcome/🪙️budget` (`renewable`);
  - `🛬️receiving/🌱️birth` (`clock_begin` now requires `Ok(true)`).
- Those failures need their own pass.

## Open in `semio-ui-prepared-laws`

1. `prepared_outcome_…` fails in its first origin turn: `WorkLimit "original Source clock declined new construction"`.
   - The `close_source` caller uses a constant `Some(1)` clock.
   - The fresh `clock_begin` change, `matches!(recorded_clock_allows(..), Ok(true))` in `🛬️receiving/🎟️native-turn/🌱️birth`, now declines it.
   - This is the same family as a2's item 1.
2. `input_drop_…` and `packet_drop_…` see a leaked process permit counter (6495666177).
   - This cascades from (1): the panicking law unwinds with a live job, under the shared `prepared_process_guard`.
3. `prepared_original_atlas_owner_…` and `prepared_original_abandoned_owners_…` release exactly 392 bytes more than the law's size formula.
   - No bytes are born during the close, so the input really owns 392 more bytes than `layers.capacity()*size_of::<DrawLayer>() + PreparedFixedPage<RenderDirective> + PreparedRenderInput`.
   - `DrawList` now also owns `scene_passes`, `shared_borders`, `glass_regions` and the stacks.
   - The formula has to be re-derived from those owners. Counting births alone was not enough.

## Follow-up: prepared-laws and host-tests green

### `semio-ui-prepared-laws`

1. **"Source clock declined new construction".**
   - The cause was not the clock. `recorded_clock_allows` needs `fuel > 0` and a `false` yield decision.
   - The law admitted its return origin with the UI worker policy grant, which can never fund the receiving frame and claim. It looped until the caller's fuel was gone, and the clock then declined.
   - The outcome law now uses the same caller as the shared `observe_prepared_step` helper: STOCK, `now_ms`, `u64::MAX`, 1<<20 fuel and `birth_returns`. It admits its origin with STOCK.
   - Removed the unused per-step `fuel` and `deadlineUs` from fixture `preparedOutcome`. No TS twin reads them.
2. **The 392-byte formula.**
   - The atlas and abandoned-owner laws now derive the input's owned bytes from the net heap that `PreparedRenderInput::new(.., DrawList::default(), ..)` actually births.
   - This replaces the hand formula `layers.capacity()*size_of::<DrawLayer>() + PreparedFixedPage<RenderDirective>`.
   - `DrawList` now owns several PagedVecs and stacks, so the formula was stale. The close must still release exactly those bytes (+ the abandonment box), plus anything the close itself births.
3. **Permit leak.**
   - `PreparedRenderInput::close_abandoned_step` closes whichever input is abandoned. Two process-global races let stale abandoned owners survive:
     - the abandoned-owner law started without draining earlier abandoned owners;
     - `preparation_observes_cancellation_without_replacing_a_packet` ran without `prepared_process_guard`.
   - A later permit law then saw a live permit (0x1832c0001).
   - The abandoned law now drains first, and the cancellation law takes the guard right after its cancel is awaited.

### `semio-ui-host-tests`

- `OriginalJobFixtureCaller::admit` failed because the window-unit `ClipboardCaller` used the 1-item UI policy grant as the caller's whole Source stock.
  - The Source's own birth then could not fund `JobPayloadAuthority::admit`, which left `actual = None`.
  - `ClipboardCaller::new` now takes the shared `close_source::STOCK` (mounted from `🧵️job/🧪️testing/♻️close`), like the physical twin.
- Turn receipts were compared against the caller's cumulative `cx.retained_progress()`. They now use per-turn deltas, in the close law and in the admission `advance` closure.

### Final `test result` lines

- `semio-ui-prepared-laws` (`a5-prepared-8`): `test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `semio-ui-host-tests` (`a5-host-4`): `test result: ok. 60 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `semio-ui-wgpu-engine-scene-laws --lib clipboard` (`a5-scene-7`): `test result: ok. 4 passed; 0 failed; …; 10 filtered out`
- `semio-ui-wgpu-laws --test original_clipboard_ingress` (`a5-clipboard-3`): `test result: ok. 2 passed; 0 failed`

### Files changed in the follow-up

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🎟️prepared/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🔬️window-unit/🦀️.rs`
