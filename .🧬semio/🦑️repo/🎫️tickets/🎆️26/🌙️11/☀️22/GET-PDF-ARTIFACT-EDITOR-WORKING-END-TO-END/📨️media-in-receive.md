# MediaIn receive: one host turn per receiving action, fixed at the scope plan (2026-11-24)

Follows `🚪️pdf-browser-open-export-stall.md` (rounds 1 and 2). Every cargo run went through `native-cargo.sh`, always filtered, via `shard-fuel-run.sh` (the three laws now run un-ignored).

## Cause

The typed receive of the ~100 byte `MediaArtifactDescriptor` (`OriginalPackReceiving`, phase 6) did one receiving action per host turn. Round 2 blamed "one receiving scope per host turn settled at every turn boundary". That is not the invariant, it is a consequence of something else.

History (`git log -S`):

| Commit | Date | What |
| --- | --- | --- |
| `c2477fa4da5` | 2026-11-08 | `OriginalReceivingScope::open` refuses while the previous scope "still owes its enclosing semantic receipt". Invariant: exact custody, a scope settles against its receipt (`receive_original_receiving_work` / `settle_receiving_scope`) before the next one opens. It is per scope, not per turn. |
| `8fdb3e10b97` | 2026-11-10 | `with_native_receiving_quoted`: the scope claims exactly the quoted grant and work plus its control, "so the held scope credit never takes the rest of the turn or Source stock". |
| `2f4d8f89820` | 2026-11-11 | Pack phase 6 moves from `with_native_receiving` (automatic half-turn share, `NativeReceivingOriginal::advance` loops many actions) to the exact quote of the single next action (`original_receiving_scope_quote`: 3 items per depth, the demand's copy bytes) and `units = min(fuel/2, 2^20)` as work. `advance`'s own loop (stop on denied quote, pending, ready output) was left in place but could never run twice: the grant held exactly one action. |

Why chaining several scopes inside one turn (round 2's `MEDIA_IN_MOVES_PER_TURN`) did not help either: `OriginalReturnScopePlan::quote` (`🧵️job/📬️outcome/🪙️budget/📦️stock/🎟️scope`), when the escrow row already held credit covering items, copy bytes, capacity and release for the quote, **clamped the quoted work to the row's remaining body work** (`quantum_work.min(body_work)`) and ran without topping up. After the first action spent `n` work, the next scope got `2^20 - n` work, but the effect inside asks its full `units = 2^20` (`with_allocated_effect` answers `None`: "work quote not admitted"). Nothing topped the work credit up until the next turn's observation refreshed the row, so the second move of a turn did nothing. Measured natively with temporary traces: `quantum_work=1045040` (= 2^20 - 3,536 spent) against a requested `1048576`, `retired_native ran none`, same demand every turn.

So the safety invariant (exact custody, exact quotes, bounded held credit) was never about one scope per turn. It stays intact below.

A second defect hid behind the first for the wide shard grant: the closing of the receiver (phase 8: `retired_native`, the receiver's close steps) hit the same clamp. And the `MediaIn` chain only counted retained stock as progress, so retirement steps (which spend fuel, not retained stock) ended the chain.

## Fix

1. `OriginalReturnScopePlan::quote`: the "reuse the held credit as is" shortcut applies only when the held body work covers the quoted work; otherwise the plan falls through to the top-up path (and still falls back to the held credit when the top-up is not affordable). An exact quote is honoured or refused, never silently shrunk.
2. `StepContext::with_native_receiving_slice(out, floor, slice, op)` (`🧵️job/⏱️context`, `sliced_receiving_quote`): the automatic half-turn share, never above `slice` on any axis, raised to the next indivisible `floor`. A held scope credit stays a bounded slice of Source stock under any turn grant (the wide fixture grant otherwise claimed 1M items and 128 MB capacity for good); inside it `NativeReceivingOriginal::advance` performs as many receiving actions as grant, slice and work admit and yields on a denied quote, real Pending, ready output or yield.
3. `OriginalPackReceiving` phase 6 uses it with `RECEIVING_SLICE` = 4,096 items, 4 MiB copy, 16 MiB capacity, 4 MiB release (the pending action keeps its pinned exact quote). This covers every typed Pack receive on command ingress (descriptor, manifest, dispatch capture), for every plugin, not only PDF. The manifest metadata receiver already used the automatic share.
4. `MediaIn` chain (`plugin-exchange`): a move counts as progress when it spends fuel or moves retained stock (retirement steps spend fuel).
5. Un-ignored laws (PDF editor unit tests): `media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_the_{shard_budget,interactive_grant}`, `a_quiet_retained_command_is_faulted_as_stalled_by_the_reactor` and the new `a_progressing_media_in_is_never_faulted_as_stalled`.

### Stalled-command law, rewritten
Once `MediaIn` is received in a few turns it no longer stays quiet (it never emits `Effect::SpawnJob` in the fixture either: its import is the guest's own typed operation, 11 to 31 work units and 1 to 21 items per turn, never an idle turn), so the law cannot be driven through it. It is now a pure reactor law over the counting seam (`note_command_turn`, `command_stalled`, re-exported through `reactor_turn_laws`): the 48th consecutive quiet turn exhausts a limit of 48, a progressing turn restarts the count, another command or instance starts its own. A companion law shows a progressing `MediaIn` completes with an idle limit of 8 and is never faulted. Note for the follow-up: with an idle limit of 1 a `MediaIn` is faulted (isolated quiet turns exist) and its faulted custody then does not retire within 512 turns in the fixture; that is the fault path's cleanup, not exercised in production (limit 16,384) and not changed here.

## Numbers (native, real `PluginRuntime`, `two-pages.pdf`, 50,000,000 fuel per turn)

| | Before | After |
| --- | --- | --- |
| Descriptor parse (phase 0) | 323 turns | 1 turn (round 2 chain) |
| Typed descriptor receive (phase 6) | > 1,013 turns, never finished (30 s wall) | 2,525 receiving actions (3,536 work, 1.27 MB copied) in **one call, one turn** under the wide shard grant; 25 turns under the 64-item interactive grant (about 47 actions per turn: 26 + 13 + 6 + 2 over four scopes) |
| Receiver retirement (phase 8) and payload, metadata | never reached | wide grant: all of it inside turn 1 and 2 (580 and 211 chained moves, 1,024 cap) |
| Whole `MediaIn` completes | never | **157 turns** under the shard grant, **442 turns** under the interactive 64-item grant |
| Peak work in one turn | 12 to 28 | at most 2^20 (asserted) |

Turn bounds asserted: reserved import operation live within 64 turns (`SHARD_MOUNT_TURN_LIMIT`), whole open within 512 turns (`SHARD_MEDIA_IN_TURN_LIMIT`; 157 and 442 measured), no turn above `SHARD_TURN_WORK_BOUND` (2^20). The full browser open (`shard_fuel_open_*`, up to the mounted document) stays within its 512 turn law.

## What is still wasteful (not changed)

- 2,525 receiving actions and 1.27 MB of copies to receive 100 bytes: the nested schema receiver pays about 25 actions per byte. A cheaper receiver for small flat records (or a flat command header) is the next saving.
- After the receive the import runs as 11 to 31 work units and 1 to 21 items per turn for about 100 (wide grant) to 400 (64 items) turns. Each turn is cheap, but the shell sees no output until the end; a larger import slice per turn is the next saving.

## Re-verify in the browser after the next wasm rebuild

1. `open-en` / `open-de`: `file-opened` is published; the lane warning, if it fires, names `(operation: media-in, stage: continuation k/…)`: expect at most about 450 continuations at the 64-item grant and no `plugin.command-stalled`.
2. Wall time of Open File for `two-pages.pdf` and a large PDF (shell pacing: turns that answer no output back off 25 to 250 ms after 15 s of quiet; the receive now produces progress every turn).
3. `transfer` (Export Document / Import Document) and new-document edit and Save stay green.
4. Other plugins' Open File (any `MediaIn`, config command, pure command): these share the Pack receiver, the scope plan change and the chain.

## Files changed

- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/📦️stock/🎟️scope/🦀️.rs` (plan clamp).
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`, `⏱️context/🛬️owned/🦀️.rs` (`with_native_receiving_slice`, `sliced_receiving_quote`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📥️pack/🦀️.rs` (phase 6 slice, `RECEIVING_SLICE`).
- `…/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs` (chain progress criterion, doc).
- `…/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs`, `⚛️reactor/🔄️turn/🦀️.rs`, `🔌️plugin/🦀️.rs` (`command_stalled`, `note_command_turn` seam).
- PDF editor unit tests (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`): the four laws, `shard_media_in_open`.
- Ticket: `run-binary.sh` (runs the built test binary directly, no cargo), this note.
