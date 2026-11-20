# Publication receipt refusal on `setActiveExample` (analysis, no source edits)

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`.

## Who sets what

| Field | Setter | Where |
|---|---|---|
| `original_receiving` | `advance_typed_operation_output_with_leftover` line 76 | `⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs`. `before` and `recipient` are sampled at line 68 (`cx.retained_progress()` and `cx.original_recipient_identity()`). `posted = after - before` around the single `app.advance_typed_operation_publication` call (line 69). `family = output.original_receipt_family()` (line 36), `grant = original`. It is set only if `posted != default`. |
| `constructor_receipt`, `preparation_receipt`, `retirement_receipt`, `history_command_receipt`, `tool_run_receipt` | `PluginExchangeOutput::collect_original_receipts` lines 29-35 | Takes AT MOST ONE family, in priority order constructor > preparation > retirement > history > tool. It is called before (line 67) and after (line 76) the advance. |
| Sources of the receipts | `🧵️instance/🔌️plugin-app-impl/🦀️.rs:300-318` | constructor: `🏪️stores/📥️staging/🚪️ingress:30`. preparation: `👷️jobs/📥️staging/🧱️components:405`. retirement: per-operation `original_retirement_receipt`. history: time-travel ledger `command_receipt`. tool: tool-run ledger. |
| Exchange fold-through | `⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🏁️exchange-finish/🦀️.rs` lines 44-50 (first advance), 64-70 (pump loop) and 118-133 | Copies the fields into the final `PluginExchangeOutput`. |
| Check | `collect_original_publication_receipts` (`⚛️reactor/🔄️turn/🩹️reconcile-drive-and-publication-receipts/🦀️.rs:156-160`) | Called from `route_exchange_output` (same file :176), which the command-ingress turn calls. |

Not a family: the `typed_receipt=true` in the diagnostic is `original_typed_command.receipt`. That is an `OriginalEmissionReceipt` (`🎮️command/🎟️typed/🦀️.rs:26,76`), not one of the five publication families. The op-binary producer, `capture_command`, `produce_action` and `stepped`/`built` consume `cx` retained progress DIRECTLY (`consume_retained`). They never set any of the five family receipts.

## Structural defect (the likely cause)

`receiving.before + receiving.posted == cx.retained_progress()` and `receiving.recipient == cx.original_recipient_identity()` are evaluated at COLLECTION time. In the generic command exchange the collection does not happen right after the step that posted:

1. `exchange_generic` (`⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs`):
   - Line 169-170: `plugin_exchange_turn` runs `finish_exchange`, which runs the advance (and `before`/`posted`). The produced output is parked in `*output = Some(produced)` when `state.applied`.
   - Lines 176-178: the exchange owner (`close_exchange_owner`), the decoded command owner and the command slot are retired. This explicitly calls `cx.consume_retained(...)` (line 178: `copied_items:1, copied_bytes`), so `cx.retained_progress()` moves past `before+posted`. This is condition (c).
   - The retirement can yield (`step_until_slice` or `return Ok(())`), so the parked output is routed on a LATER turn with another `StepContext`. This is condition (b), a different `original_recipient_identity` and a fresh progress counter. Condition (c) fails again.
   - Line 181: only then `route_exchange_output` -> `collect_original_publication_receipts` runs.
2. The other call sites collect immediately after producing, in the same `cx`: `exchange_presence` line 150-152, the `!applied` branch at line 170, `🔄️turn/⏱️turn-execution-and-poll-entry:111`, `🔄️turn/📥️receiving:64`, `🔄️turn/📨️event-dispatch:213` and `🌀️poll-kernel-turn:59`. Those are consistent.

So conditions (b) and (c) fail deterministically for ANY applied generic exchange whose finish advance posted nonzero retained progress. Exchanges that posted nothing have `original_receiving = None` and skip the check, which is why it is rarely seen. The fix of `admit_original_construction_settled` (Ready arm falls through and constructs) makes the typed action-production/construction consume progress inside the advance step (`action_source=true`, `typed_production=true`: host-event or action-ingress driven from `plugin-app-impl:271-297`). So `posted != 0` now happens for the first time on the typed command exchange.

Condition (a) can also fail: if the advance posted progress through a non-family owner (typed action production, `drive_turn`, host event routes, reserved commit), then `receiving.family = None` and zero families are present. That would be refused too. Which of (a), (b), (c) fired is not distinguishable without the `[DEBUG] receipt-refusal families=... receiving_family=... recipient_ok=... before=... posted=... retained=...` line. Read it from the browser console:
- `families=[]`, `receiving_family=None`: (a) (non-family posting).
- `recipient_ok=false`: (b).
- `recipient_ok=true`, families OK, `before+posted != retained`: (c).

## Proposed fix (NOT applied)

In `exchange_generic`, bind receipts in the same `cx` that posted, right after production and before the input retirement parks the output:

```rust
Ok(mut produced)=>{ if state.applied {
    if let Err(refusal)=collect_original_publication_receipts(&mut produced,cx){return Err(retain_refused_publication(cursor.instance,produced,refusal,cx))}
    *output=Some(produced) } else { ... }}
```

`collect_original_publication_receipts` `take()`s every receipt field, so the later `route_exchange_output` at line 181 sees `original_receiving = None` and skips the check. It still routes frames, effects and events. This keeps the "same recipient" invariant meaningful and fixes (b) and (c). It does NOT fix (a). If the debug line shows `families=[]` the right fix is different: the typed action production posting must either not be counted in `posted` (sample `before` after the production, or make the production consume itself outside the `before/posted` window) or be given a family.

Not applied: it needs `.rs` edits under `🧰️framework` (would restart the running wasm build), and the discriminating debug line was not available, so the fix could not be confirmed as the single cause.

## Tests

Existing:
- `⚛️reactor/🔄️turn/🎟️receipt/🧪️tests/🦀️.rs` (`original_reactor_publication_receipts_preserve_all_five_tuples_and_same_recipient_once` and the refusal cases). It calls `collect_original_publication_receipts` directly with a synthetic `output` and a same-`cx` posting, so it does not cover the "retire between advance and collect" ordering.
- `⚛️reactor/🧪️tests/🔬️command-ingress-terminal` only does source checks.
- `🧪️tests/🔬️plugin-runtime-runtime-close-budget` and the typed-operations `response-tests.rs` cover the family fold-through.

Native run (`cargo test -j 4 -p semio-plugin-reactor-turn original_reactor_publication_receipts`, env as specified): does NOT compile right now. A dependency fails: `semio-framework-value-snapshot-projection` at `🧰️framework/🔨️modules/🌱️value/🧮️array/📍️placement.rs:38` (E0596, `state` is behind `&` but `state.authenticate(effect)` needs `&mut`). That is a concurrent in-flight edit by someone else, not this analysis. Re-run once it is fixed. `C:/b/fem3d-rcpt` was deleted afterwards.
