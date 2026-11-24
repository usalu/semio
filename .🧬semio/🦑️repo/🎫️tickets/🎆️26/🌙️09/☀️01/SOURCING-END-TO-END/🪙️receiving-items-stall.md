# 🪙️ Receiving-step items stall of `setContributions` (2026-11-23)

Stall text: `waiting on the turn grant cannot fund one receiving step [0, 500710, 48846219]` (items, copy, fuel).

## Measured natively (temporary `[DEBUG]` print, removed again)

`the_real_demonstrator_pack_settles_through_the_typed_set_contributions_operation`, at the receiving-quote site of `publish_mounted_typed_operation_unit`:
`retained.maximum_items` is always 1 (`original_lifecycle_grant` caps it at `.min(1)`), `scope_items` (`original_receiving_scope_headroom`) is 10 items / 1770 B natively,
and the turn's `phase.maximum_items` (`cx.retained_grant()`) ranges 52..64. So natively `min(1, phase - 10) = 1`.
The item quote is `0` only when `phase.maximum_items <= scope_items` (10). Copy 500710 and fuel 48.8M are healthy, so the browser's turn items are consumed/held, not the policy (64 per turn, `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`).

## Root cause

The unit subtracted the first-claim scope headroom (10 items, copy, work) from the turn grant on every call. When the decoder already holds its receiving scope
(the resident birth service claims the first scope at quantum one and keeps 32 reserved body items, `admit_original_receiving_scope` / `RESERVED_BODY_ITEMS`), that claim is already deducted from
`cx.retained_grant()` and no further first claim happens (`original_receiving_outcome` only claims when `!owner_loaned`). Subtracting the headroom a second time
drives the item quote to 0 as soon as the held claim leaves 10 or fewer items, and the unit then never advances (4096 units, no ladder movement).

## Fix

- `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs`: new `StepContext::original_receiving_scope_held()`.
- `.../📬️typed-operation-unit/🦀️.rs`: headroom is `(0,0,0)` when the scope is already held; the first-claim headroom applies only when a claim is still to be made.
  The stall label now matches its array: `(items, copy bytes, fuel)`.

## Verification

Native curation lib suite: see ticket close summary. Not reproduced natively with the browser's held scope (the fixture caller has no resident birth service), so the browser outcome is verified only by a wasm rebuild.
If the stall persists, the item quote is still 0 with a held scope: the turn's `phase.maximum_items` is then 0 and the host turn grant itself must be examined.

## Silent publication stall

Symptom: `typed operation 'setContributions' advanced 4096 publication units without moving its ladder` with no `waiting on` suffix.

### The last wait is never cleared

`LAST_PUBLICATION_WAIT` (`🧵️retained-command/🔄️full-operation/🦀️.rs`) is a thread-local `(operation, wait)` that is only ever overwritten by `note_publication_wait`; nothing clears it. A stall without a suffix therefore means no note was ever recorded for that operation id in that build, i.e. a no-progress path returned `Ok(())` silently (the old sites noted only: unfunded quote, receiving scope not admitted, preparation refused, `Blocked`, `AwaitingAck`).

### Permanent typed diagnostics added (`note_publication_wait(operation, reason, [figures])`)

- `📬️typed-operation-unit/🦀️.rs` (`publish_mounted_typed_operation_unit`): acknowledged result page retirement without progress; terminal page plan retirement yield / cannot enter; ui and completion outbox full; completion take unfunded or yielded; worker completion not ready; tool settlement incomplete; tool settlement close unfunded; stream abort frontier adoption unfunded; `pending.is_closing()`; returned snapshot read return without progress; window config store batch not admitted. The temporary `[DEBUG] unit unfunded` eprintln is removed.
- `🏃️mounted-run-and-pump/🦀️.rs`: pump yield before a unit; phase items cannot fund one receiving step.
- `🔄️full-operation/🦀️.rs`: `admit_actor_capture` yield or cancel; `note_publication_progress` when the store checkpoint did not advance (this covers the `Progress`/`PreparationProgress` arms that previously recorded nothing, the prime suspect for a silent loop).
- `🔁️stall-witness-and-unit-advance/🦀️.rs`: `advance_mounted_window_config_refresh` not admitted / still in progress.
- `🖼️render/📥️staging/🛠️tool-machine/🦀️.rs`: `settle_tool_operation` capture stage not funded.
- `🔌️plugin-app-impl/🦀️.rs`: `typed_operation_diagnostic` now prints `wait=` per mounted operation, so fixture settlement refusals show the last wait too.
- `advance_acknowledged_result_page` shares `original_lifecycle_effect_step_at` (no operation id); its no-progress result is noted at its call site.
- Store owner `admit_store_batch` returning `None` is noted at its call site (items, copy bytes, depth).

### Native reproduction

New law `the_real_demonstrator_pack_settles_under_the_browser_grant` (curation `🔬️unit`): per-turn grant depth 64, 520 KiB copy, 16 MiB capacity, 1 MiB release, 64 items, through new fixture seam `arm_host_turn_grant` (narrows every `fixture_renew_turn`; the lifetime stock stays the fixture's; receipt checks accept a narrowing of the authority only while armed). It settles natively, as do the two existing laws. Lowering the caller *stock* instead (first attempt) starves the lifetime items to 1 and stalls in command admission; that is a fixture artefact, not the browser case.

Not reproduced: the missing factor is the resident birth service that holds the receiving scope in the browser; the fixture caller has none. No native root cause found beyond the earlier held-scope headroom fix.

### Next

Rebuild wasm and read the stall suffix: it now names the state and figures of whichever path loops.
