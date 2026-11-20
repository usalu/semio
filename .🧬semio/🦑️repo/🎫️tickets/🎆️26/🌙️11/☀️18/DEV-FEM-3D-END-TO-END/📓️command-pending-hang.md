# Command-pending hang on the first fem3d command (static diagnosis)

Status: read-only analysis, no Rust edited, no native test run (disk 97 percent, 30 GB free; the reactor closure is too large for a focused test within budget). The root cause is narrowed to one function but NOT proven; the proof is a single probe (section 5).

## 1. What kind 16 is

`AppCommand::ReadHistory { seq }` (decoder `📡️spr/🧵️channel/🗜️codec/📥️command-decoder/🦀️.rs:26`, cursor decode `🧩️command-decode/🧭️command-cursor/🦀️.rs:32`). Its arm `arm_read_history` (`⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🎯️document-and-child-commands/🦀️.rs:173-185`) is synchronous: `with_instances_mut` + `resolve_ready(instance.app.history_snapshot())`. It awaits no example load, event or host acknowledgement. So hypothesis (c) is ruled out: the fem handler cannot be what stalls the command.

## 2. Key evidence: the reported cursor says exchange_generic is NOT running

`exchange_generic` (`⌨️command-ingress-turn/🦀️.rs:146`) sets `status = CommandPending(advance_command_cursor(cursor))`, and `advance_command_cursor` (`⚛️reactor/🦀️.rs:351-355`) bumps `page_index` by one. Every early return inside `exchange_generic` therefore reports `pageIndex 1` for this one-page command. The observed status carries `pageIndex 0`, the cursor exactly as stored in the retained `Generic` owner (`continue_generic_assembly` line 262 stores the page cursor unadvanced).

A `pageIndex 0` CommandPending can only come from:

1. `drive_command_ingress` line 58: `turn.resume(...)` returned `false`; with no page the status is `Idle` and `command_ingress_while_owned` (`🔌️extension-invocation-and-command-ingress/🦀️.rs:113-118`) rewrites it to `CommandPending(owner cursor)` using `peek()`. This returns before `cancel_for_close`, `exchange_presence`, `exchange_generic` and before the whole page path. Everything after it in the reactor still runs (reconcile "0 opportunities"), which matches the symptom.
2. `cancel_for_close` line 94 (needs `close_instances` containing the owner instance; `resume()` line 86 adds it when `native_close_key(runtime, instance) != retained key`).
3. `finish` line 268 with an owner that no step touched (only happens when no earlier step ran).

So the gates listed in the brief (close_exchange_owner, state.applied, close_original_step, grant maxima at line 156) are not the culprit on continuation turns: they would show `pageIndex 1`. That answers (a) and (b): the retained-grant maxima at line 156 and the `plugin_exchange_turn` event dependency never get reached.

## 3. Prime suspect: `resume()` -> `settle_step` / `lend` never completes

`resume()` (lines 80-88) needs, every turn and BEFORE any command work:

- `step_retiring_command_ingress(cx)` (slot 0), then
- `COMMAND_INGRESS[LIVE].settle_step(cx)` -> `false` aborts (line 82),
- `loan.lend(cx)` -> `false` aborts (line 85).

Both are `OriginalActorReturnSlot` steps (`🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/📦️return/🦀️.rs`):

- `settle_step` (line 25) resets `donor_address = 0` every turn (the owner is `Some`, so `address`/`binding` stay) and needs `settle_place` for home and donor plus one `pay`.
- `lend_step` (line 17) must then `pay(usize)` again for the donor address, `prepare` home (admit_sized / qualify_sized / preissue_capture) and donor (same), then `pay(size_of::<ActorReturnTarget<T>>() + size_of::<Option<T>>() + 1)`.

`pay` (line 14) returns `Ok(false)` when `should_yield() || is_cancelled()`, `maximum_items == 0`, `maximum_copy_bytes < bytes`, `maximum_depth == 0` or `fuel_remaining() == 0`. A retained owner therefore costs every continuation turn roughly 10 or more grant items plus `2 * size_of::<RetainedCommandIngress>()` copy bytes (T is `RetainedCommandIngress`, whose `Generic` variant inlines `PluginCommandIngress`, `OriginalCommandExchangeState` with two `Option<PluginExchangeOutput>`, two `ControlledRetirement` ...). The host's own comment (`🚚️handle-turn-drive/🟦️.ts:375`) says a continuation turn gets only "a fixed ~4 KB / 12 items of lifetime grant (origin refresh)". If a continuation turn (submitted with `{ activation }`, no page, line 377) is granted less than this re-lend ladder needs (items, copy bytes, or depth), `lend` returns `false` on EVERY turn with zero progress and no event can change that: exactly "command-pending, more-work, nothing else happens, forever". A page turn is not affected the same way, which is why the command is admitted (the page turn has the page's own grant and the owner is still born with an empty loan).

Why this is a regression candidate: the `size_of` terms grow whenever a peer adds a field to `CommandIngressOwner::Generic`, `OriginalCommandExchangeState` or `PluginExchangeOutput` (all inline in `T`). A growth past the continuation copy grant silently turns every retained command into a permanent `resume() == false`. The peer change to `original_call_start_progress` (`🌀️poll-kernel-turn/🦀️.rs:23`) only changes the receipt baseline (`🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:425`), not any gate, and is not involved.

Secondary candidates that give the same `pageIndex 0` signature (verify with the probe):

- `native_close_key(runtime, key.instance()) != Some(key)` at line 86: the retained key no longer matches the instance lifetime (instance reopened, lifetime slot replaced) -> `cancel_for_close` -> `retire_command_ingress` may stall on `retirement_should_yield()` forever. Needs an instance generation change between the page turn and the continuations.
- `should_yield()` true on every continuation (`deadline_exceeded` is `now_us().is_none_or(...)`, `⏱️context/🦀️.rs:415`): a guest clock that returns `None` in the browser makes every `pay` refuse. The page turn would then also refuse (`admit_first_generic_page` line 242), so this is less likely.

Ruled out by reading: `plugin_exchange` line 116 `constructing` branch (the constructor must have admitted all four catalogs before an instance is visible, `🧵️instance/🧱️construction/🧺️sources/🦀️.rs:88`); `PluginCommandIngress::advance_original` grant gates (would yield `pageIndex 1`).

## 4. Proposed fix (not applied)

The invariant "a retained owner can always be re-lent from a bare continuation turn" must be a quoted law, not an accident of struct sizes:

1. Keep the retained command owner out of the inline return slot: store `RetainedCommandIngress` behind a fixed small handle (a `Box`-free fixed slot index, e.g. the 24 byte `NativeCloseKey` + a stage enum) so `size_of::<Option<T>>()` is constant and tiny, and keep the bulky `Generic` payload (`command`, `exchange`, `output`) in its own already Source-funded home that is addressed, not copied, by the loan. Then the per-turn re-lend ladder costs a constant few items and a few dozen bytes.
2. Until then, give the host's continuation turn the quoted grant: `settle_step + lend_step + return` demands (`pay` bytes `2*size_of::<RetainedCommandIngress>()` plus the place preissue demands) added to the origin refresh in the shard actor's continuation grant, and add a unit test that a one-page `ReadHistory` completes with `continuation grant == origin refresh + resume ladder quote` (and fails closed with a typed fault, not a silent `CommandPending`, when below it).
3. Independently: `resume()` returning `false` for more than N consecutive turns with an unchanged cursor must raise a typed fault (`plugin.command-resume-starved`) instead of `CommandPending`; the host then fails fast with the cause instead of waiting 300 s.

Test to add (native, language-agnostic fixture): `OriginalJobFixtureCaller` (as in the `return/🦀️.rs:37` test) driving `drive_command_ingress` with a one-page `ReadHistory`: turn 1 with the page, then continuations with `grant == origin refresh quote`; assert `CommandComplete` within a small bound, plus a size assertion on `size_of::<RetainedCommandIngress>()` against the continuation copy grant.

## 5. One probe that proves it (do not touch the existing probes)

Add one `[DEBUG] ` log in `CommandIngressTurn::resume` (`⌨️command-ingress-turn/🦀️.rs:80`) that names which of `settle_step`, `peek().is_none()`, `lend`, or the `native_close_key` mismatch returned false, together with `cx.retained_grant()`, `cx.fuel_remaining()` and `size_of::<Option<RetainedCommandIngress>>()`. Expected under this diagnosis: `lend` returns false on every continuation, `grant.maximum_items` small (about 12 minus other phases) and/or `maximum_copy_bytes` below `2*size_of::<RetainedCommandIngress>()`. A cheaper discriminator that needs no code change: the existing `[DEBUG] exchange_generic: seq ...` probe (line 149) must print on page turn 1 and NEVER on continuation turns; if it prints on continuations, this diagnosis is wrong and the status cursor should then show `pageIndex 1`.
