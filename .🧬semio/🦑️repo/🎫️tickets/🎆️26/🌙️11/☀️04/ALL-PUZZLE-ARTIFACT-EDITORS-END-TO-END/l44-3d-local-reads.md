# L44 3D Local Interaction Reads (Manifest Section D)

This is source analysis only. Nothing was compiled or run, so the fix is UNVERIFIED.
The log is `🗑️generated/l41-3d.log`. The same failures were already in l12 and l32.

Abbreviations:
- P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`
- E3T = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests`

## Cause: the test host never takes back the local-interaction ownership receipt (proven from source)

### How one read runs
1. `begin_local_interaction_query` (P/🧵️instance/🔌️plugin-app-impl/🕹️ephemeral-interaction/🦀️.rs:6) installs a `LocalInteractionLiveQuery`.
2. `Started` needs no advance. `take_reply_admitted` emits it while `!started` (P/🕹️interaction/📡️live/🦀️.rs:235). Seeing `Started` therefore proves nothing about the stage driver.
3. Pages are produced only by `LocalInteraction::advance_local_interaction_query_one` (P/🧵️instance/🏪️stores/📥️staging/🚪️ingress/🦀️.rs:260). It is called from `advance_typed_operation_publication` on every other turn (`local_interaction_query_turn` toggle, P/🧵️instance/🔌️plugin-app-impl/🦀️.rs:326-331).
4. Every advance that makes progress stores `local_interaction_query_receipt = Some((grant, progress))` (ingress:271).
5. While that receipt is `Some`, two gates refuse to do anything:
   - `advance_typed_operation_publication` returns at its third line (plugin-app-impl/🦀️.rs:292);
   - `advance_local_interaction_query_one` returns as well (ingress:261).
6. Only the host clears the receipt, through `PluginApp::take_local_interaction_query_receipt` (plugin-app-impl/🦀️.rs:358). The callers are:
   - production `advance_typed_operation_output_inner` (P/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs:90), which takes it, `consume_retained`s it and ends that turn;
   - the fixture `retain_original_ownership_receipts` (P/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:409), which the fixture `host_turn` calls (E3T/🧰️context/🏗️raw-app-fixture/🦀️.rs:459).

### Why the law hangs
`read_local_interaction_while_rendering` (E3T/🔬️unit/🕹️roster-and-read-laws/🦀️.rs) calls `raw.advance_typed_operation_publication` directly and never takes the receipt:
- Turn 1 advances while the query is not yet started, which is `Blocked` with no receipt, then publishes `Started`.
- Turn 2 is the toggle-off turn.
- Turn 3 is the first real unit. It posts a receipt.
- After that, every unit returns at :292. `has_runnable_typed_operations` stays true because `has_pending_work` holds while `!page_sent`.

That gives 65536 turns with `bytes=0` and no page, which is exactly what the log shows. The receipt gate exists since 2026-10 (`git log -S`), so this law has been stale against the framework contract since then.

### Render-pressure variant
The test's long-lived `OriginalFixtureScope` context is never renewed. Its retained stock is therefore drawn down by one full composite render on each of up to 65536 turns. Once items, depth or copy run out, `close_pending_render_tree_on_original` answers `Ok(None)` forever (`grant.maximum_items==0||grant.maximum_depth==0`, P/🧵️instance/👷️jobs/📥️staging/🖼️render/🦀️.rs). `release_fixture_render` then spends its `maximum_turns` (32768) without `should_yield` (the lent fuel is 65.5e9), and fails with "original fixture render tree release remained pending under its authored caller" (P/🧪️testing/🧪️artifact-app-instance-laws/🔎️declared-verb-probing/🦀️.rs:213).

This is the same root cause (the read never ends), plus the missing per-turn renewal that a host performs. The stock exhaustion is inferred, not observed: no line with the turn count was printed before the panic.

## Fix (applied, test-side, mirrors the fixture `host_turn` and the production exchange)
In `read_local_interaction_while_rendering`, each turn now:
1. captures the caller grant;
2. runs `measure_maintenance_step`;
3. calls `fixture_renew_turn(cx, OriginalFixtureCaller::turn_fuel(), policy.maintenance)`;
4. advances one unit;
5. calls `retain_original_ownership_receipts(raw, policy, …)`, which takes the LocalInteraction receipt back like the host exchange output does;
6. calls `fixture_renew_turn(cx, turn_fuel, caller_grant)`;
7. then publishes and acknowledges as before.

The doc comment now names the receipt round trip. No assertion was loosened and no bound was raised. The framework was not changed, because production already takes the receipt.

## l37 / l38 check
- `original_tool_run()` migration (l37 §4) and l38 `project_owned` (fill projection): neither touches the local-interaction path. The query captures document, config and interaction `snapshot_read` leases only. These failures predate both (present in l12 and l32).
- l38 D's guess that this is shared with A4 is wrong. The cause is the untaken receipt.

## Risks for the next build
- If a page still does not arrive, the next gate to probe is the earlier stages in `advance_typed_operation_publication`: `InstanceExtensions::drive_turn` returning true every turn, or `holds_receipt`. No `[DEBUG]` probe was added, because the cause is proven.
- `retain_original_ownership_receipts` checks that the LocalInteraction grant is within `policy.preparation`. It is (INTERACTIVE_COMPONENT has preparation == maintenance, and the turn is renewed to maintenance).

## Files changed
- E3T/🔬️unit/🕹️roster-and-read-laws/🦀️.rs (`read_local_interaction_while_rendering` and its doc comment)
