# Wave B16 — a mounted typed operation is runnable at every stage, so no host continuation can park it

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B16, 2026-09-11. Closes wave B14's §6 residual
("a parked typed operation only advances on a turn carrying COMMAND INGRESS"). Guest Rust only.
No git write, ticket not closed, `🗑️generated` written to and never deleted, no wasm build
(the coordinator owns #47), every command run in the foreground.

---

## 1 The scheduling path, file:line

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`

| step | site | what it does |
| --- | --- | --- |
| a host ACK arrives as an event | `:664` `Event::Message{ source: Shell{instance} }` → `plugin_acknowledge_typed_operation_result` | the ONLY way an `AwaitingAck` operation moves on; it is an event the host submits with its **next** continuation, never inside the turn that produced the page |
| the turn's one typed-operation unit | `:1057` `plugin_continue_typed_operations` | gated: `pending_typed_operation_instance` must find an instance answering `has_runnable_typed_operations()`, otherwise **nothing is advanced and no outbox is drained this turn** |
| the page leaves as an effect | `:1379` `route_exchange_output` → `Effect::SendMessage{ Shell }` | what the renderer turns into an acknowledgement (`🔌️PluginRuntime/🟦️.tsx:538 typedOperationAcknowledgements`) |
| the fold | `:1171` `more_work \|\| … \|\| typed_operation_scan.runnable \|\| …`, `:1229` `TurnStatus::MoreWork` | **only the fold crosses the wire** — the host cannot see which source is armed |

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

| symbol | line (post-wave) |
| --- | --- |
| `PluginApp::has_runnable_typed_operations` (trait) | `:11657` |
| `MountedTypedCommandFullOperation::take_result_page` / `acknowledge_result_page` | `:16671` / `:16679` |
| `VcsArtifactApp::next_advanceable_typed_operation` (new) | `:23207` |
| `advance_typed_operation_publication_one` | `:23217` |
| `VcsArtifactApp::has_runnable_typed_operations` | `:25870` |
| `VcsArtifactApp::take_typed_operation_result_page` | `:25952` |

---

## 2 Root cause

> **The guest answered "no runnable typed operations" on every turn that published anything, and the
> acknowledgement that would have un-parked the operation is an event the host only sends while the
> guest is still answering `MoreWork`.**

`MountedTypedCommandFullOperation::has_runnable_work` was

```rust
match self.stage {
    Worker | Publishing | Retiring => true,
    AwaitingAck => !self.result_page_presented,     // ← false the instant the page was handed out
}
```

and `VcsArtifactApp::has_runnable_typed_operations` folded that per operation. So the sequence of a
mutation was:

1. the turn's unit publishes one ladder step and queues a result page;
2. `advance_typed_operation_output` hands that page out — `result_page_presented = true`, stage
   `AwaitingAck`;
3. the SAME turn's second scan (`plugin_continue_typed_operations`, `⚛️reactor/🔄️turn/🦀️.rs:1057`)
   now reads `runnable = false`, so `typed_operation` is unarmed in the fold and, with nothing else
   armed, the turn answers **`TurnStatus::Idle` while the operation is mid-ladder**.

The host cannot read anything else. `settlePluginTurn`'s `hasWork()` is
`(drainOperations || required missing) && status === "more-work"`; the eventless
`drainTypedOperations` poll and a `refresh-ui` settle whose surfaces are already published have
**nothing but the status** to go on, so they return at continuation 0 — exactly what B14 §6 measured
in the browser ("arming `drainTypedOperations` unconditionally after every command changed nothing —
its poll submits an EVENTLESS turn and the actor answers idle at continuation 0"). And a presented
page is deliberately never re-published before its retry deadline (existing law, `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
*"presented result page must not be republished before an explicit retry deadline"*), so once a call
ends there holding the un-sent acknowledgement, **only a later call that re-arms some OTHER source
of the fold** — a new command's `latest_wins_commands` entry — makes the instance runnable again and
drags the parked operation forward. That is the one-command lag, verbatim.

A second, compounding defect on the same path: `advance_typed_operation_publication_one` picked the
turn's single unit with a blind `tool_operations.next_id_from(cursor)`, and
`take_typed_operation_result_page` inspected exactly ONE slot per turn the same way. With several
operations mounted (the battery's steady state: camera/hover/engagement verbs alongside the edit),
the turn's only unit was regularly spent on an `AwaitingAck` operation that has no step of its own,
and a queued page could wait several turns for its cursor to come round.

---

## 3 The fix

| file | change |
| --- | --- |
| `🔌️plugin/🦀️.rs` `:16679` region | **`MountedTypedCommandFullOperation::has_runnable_work` removed.** A mounted operation is runnable until it is removed — there is no longer a per-operation predicate to get wrong. |
| `:25870` | `has_runnable_typed_operations`'s first clause is now `!self.tool_operations.is_empty()`. This makes `has_pending_typed_operations ⇒ has_runnable_typed_operations` **structurally total** (`runnable` is `pending` plus `has_runnable_artifact_envelope_decode`). |
| `:11657` | the trait docstring states the new contract: the fold is the only thing a status-only host call can read, so a mounted operation counts at EVERY stage, an unacknowledged presented page included. |
| `:23207` | **new `VcsArtifactApp::next_advanceable_typed_operation`** — the turn's one publication unit is spent on the first operation from the rotating cursor that is NOT `AwaitingAck`, so an operation waiting on the host's event never starves a sibling that has a step. |
| `:25952` | `take_typed_operation_result_page` scans from its cursor for the first operation that actually owns an unpresented page for this receiver, instead of inspecting one slot and giving up. The `!result_page_presented` guard is kept verbatim — **nothing is ever republished**. |
| `:24741` | `typed_operation_result_state_for_test`'s fourth element is now `stage == AwaitingAck` (it was the deleted predicate); the two assertions that read it were updated from `false` to `true`. |
| `🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs:531,549` | the two `assert!(!mounted.has_runnable_work())` that encoded the old contract became `assert_eq!(mounted.stage, …AwaitingAck)`. The non-republication witness next to them (`take_result_page().is_none()` / the `deliveries` count) is untouched and still green. |

No constant changed, no table grew, no host poll was added, no fault was swallowed, and no
acknowledgement protocol changed: a page is still handed out once and ACKed once. The only thing
that changed is **what the turn tells the host about itself** — plus which operation the turn's
existing unit grant is spent on.

### Why this is enough

The host already continues while the guest says `more-work` (B14). With the fold armed for the whole
life of a mounted operation, the acknowledgement always rides the next continuation of the SAME call;
`drainTypedOperations` and every `refresh-ui` settle now drive the ladder too, and the
`OperationCompleted` frame is handed back by `advance_typed_operation_output` on the very
continuation that queues the terminal page (the completion witness is pushed in the same call that
queues the `Terminal` lane — `🔌️plugin/🦀️.rs:23917` — and `take_typed_operation_completion` runs
immediately after the advance in the same turn).

---

## 4 Laws

Bodies in `🔌️plugin/🦀️.rs` beside wave B8's, entry points in
`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` over the same `KeyedTestApp` /
`compositeEdit` fixture B0 and B8 use.

`drive_host_call_with_deferred_acks` (`:17506`) is the host's own call **with the ACK round trip the
reactor really has**: an acknowledgement minted for a page this continuation presented is submitted
at the TOP of the NEXT continuation, never inside the turn that produced it. `stop_at_idle` picks
which host call it is — `false` is `settlePluginTurn` (which also continues while it owes an
acknowledgement), `true` is every call that can read nothing but the status: the eventless
`drainTypedOperations` poll and a `refresh-ui` settle.

1. `a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work` (body
   `test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work` `:17546`) — drives ONLY
   continuations of the admitting call, no second command, and requires that no continuation ever
   ends owing typed-operation work while the turn reports none runnable.
2. `a_status_only_host_call_finishes_every_typed_operation_it_admitted` (body
   `test_typed_operation_completes_under_a_status_only_host_call` `:17568`) — the same admission
   driven by a call that gates purely on `more-work`, followed by ONE eventless drain call; nothing
   may be left pending for the next command.

**Written first, run at HEAD — both FAILED:**

```
running 2 tests
test …::a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work ... FAILED
test …::a_status_only_host_call_finishes_every_typed_operation_it_admitted ... FAILED

assertion `left == right` failed: continuation Some(32) of 34 ended owing typed-operation work while
the turn reported none runnable; it left 0 of 64 typed-operation slots live after 34 host continuations []
  left: Some(32)
 right: None

a status-only call of 30 continuations (parked at Some(29)) and one eventless drain call of 1 left
typed-operation work for the next command; it left 1 of 64 typed-operation slots live after 31 host
continuations ["128:AwaitingAck:presented=true"]

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 656 filtered out; finished in 0.03s
```

`128:AwaitingAck:presented=true` is the whole defect in one string, and it is the same census shape
B8's red produced: the operation handed out its page, the turn answered `Idle`, the status-only call
returned, and a second status-only call could not touch it either — it sat there for the next
command.

An earlier instrumented run of the same fixture (temporary `[DEBUG] b16 …` tap in the driver, added,
run and **removed**) printed the transition exactly:

```
[DEBUG] b16 continuation=27 lane=none  runnable=true  pending=true
[DEBUG] b16 continuation=28 lane=Fault runnable=false pending=true     ← page handed out, turn goes Idle
[DEBUG] b16 continuation=29 lane=none  runnable=false pending=false    ← only because THIS driver still delivered the ACK
```

**After the change — the two laws plus every law that shares the path:**

```
running 15 tests
test …::a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work ... ok
test …::a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot ... ok
test …::a_status_only_host_call_finishes_every_typed_operation_it_admitted ... ok
test …::every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable ... ok
test …::reserved_undo_actor_ingress_admits_undeclared_window_kind ... ok
test …::reserved_undo_browser_note_without_inverse_pops_chrome_resize ... ok
test …::reserved_undo_first_turn_admits_spawn_job_and_drive_commits_history_route ... ok
test …::reserved_undo_host_json_export_admits_isolated_spawn_job ... ok
test …::reserved_undo_invocation_does_not_require_window_ownership ... ok
test …::reserved_undo_pops_chrome_top_shell_then_publishes_history_patch ... ok
test …::reserved_undo_pops_shell_then_falls_through_to_document_store ... ok
test …::reserved_undo_reaches_done_within_host_drive_contract ... ok
test …::reserved_undo_replay_shell_command_survives_wire_roundtrip ... ok
test …::typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id ... ok
test component::reactor::reconcile_budget_tests::a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 644 filtered out; finished in 0.60s
```

i.e. **B8's two slot-retirement laws, B0's pre-admission law, B2's mixed-surface law and the whole
`reserved_undo` set are green with this wave.**

---

## 5 Verification (foreground, tails quoted)

| command | result |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 never_parks_a_turn status_only_host_call typed_operation_slot release_their_exact_slot typed_operation_ingress_pre_admits reserved_undo a_nakagin_scale_mixed_surface_turn_…` | `test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 644 filtered out; finished in 0.60s` |
| the two new laws with the fix neutralised | `test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 656 filtered out; finished in 0.03s` (tails in §4) |
| whole crate, with the wave (`--skip a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`, B8 §6's known order-dependent SIGABRT flake) | `test result: FAILED. 511 passed; 147 failed; 0 ignored; 0 measured; 1 filtered out; finished in 31.23s` — `🗑️generated/wave-B16-plugin-lib-wave3.txt` |
| whole crate, fix neutralised (baseline) | `test result: FAILED. 509 passed; 150 failed; 0 ignored; 0 measured; 0 filtered out; finished in 33.46s` — `🗑️generated/wave-B16-plugin-lib-baseline.txt` |
| **failure-NAME diff, wave vs baseline** | **regressions: (empty)**. Baseline-only: this wave's two laws, plus `component::reactor::turn::turn_execution_tests::guest_turn_execution_resets_for_every_turn`, which **passes in isolation** (`test result: ok. 1 passed`) and is one of the retained-heap order-dependent measurements `📓️2026-09-10-order-dependent-tests-audit.md` records — the `--skip` shifts what the shared process has allocated by the time it runs, so it is not attributable to this wave. |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 paste duplicate delete add_object catalogue` | `test result: ok. 46 passed; 0 failed; 0 ignored; 0 measured; 656 filtered out; finished in 0.82s` — includes `leftover_copy_paste_clones_selected_object`, `open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change`, `the_add_object_dialog_offers_every_object_kind_of_both_examples`, `kinds_tree_object_drag_data_carries_object_kind_and_mesh_url`, `the_catalogue_pages_an_over_wide_kind_catalog_without_exceeding_the_fixed_page`, `create_delete_object_inverse_law`, `create_duplicate_id_is_fatal_and_never_applies` and the `delete_object`/`delete_reference`/`delete_target_volume`/`add_object_vortex` mutation oracles |
| whole puzzle3d lib | `test result: FAILED. 699 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 64.29s` — `🗑️generated/wave-B16-puzzle3d-lib.txt` |
| **the same 3 with the fix neutralised** | `test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 699 filtered out; finished in 1.65s` — identical set and identical messages, so **all three are pre-existing**: `every_advertised_engagement_verb_is_implemented` (`typing clear must empty the framework-owned selection, left: 1 right: 0` — B13/B15's selection region), `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` (`worst turn 4.422584ms over 26 turns exceeds this artifact's own unoptimized budget 2ms`), `two_instances_converge_disjoint_object_edits_via_backbone` (`module.vcs: remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized`) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings` / `Finished dev profile [unoptimized] target(s)` — **0 errors** |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Finished dev profile [unoptimized] target(s) in 1.13s` — **0 errors**, guest target |

The 147 pre-existing plugin-crate failures are the peer refactor B0 §5, B2 §4 and B8 §6 all recorded
(`interactive-job.missing-factory`, `app-definition.invalid: app id testkit-txn …`, and the
`terminal-empty shallow-shell witness` ones those two cause downstream), unchanged in membership.
No wasm build was run.

### Two peer-churn unblocks (additive, no hunk of theirs reverted)

The plugin crate's test build did not compile at two points during this wave, both from peers
mid-refactor:

1. `ui_history_panel` grew a fifth parameter `command_page: u32` in `🔌️plugin/🦀️.rs:9907` while its
   seven call sites in `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` still passed four.
   After ~7 minutes of polling with no change, each call site was given the first page (`, 0)`).
   Whoever owns the paging work should replace those with whatever page their own law wants.
2. `GuestTerminalOwner::{terminal_is_empty, release_terminal}` changed its error type from
   `&'static str` to `semio_framework::Fault` in `⚛️reactor/🚪️lifetime/🦀️.rs:58-59` ahead of its test
   impls. That one the peer landed themselves within three minutes; nothing was touched here.

---

## 6 Probe verdicts this should flip on #47

:6013 serves wasm **#46**, which predates this guest change, so **no probe was run** — a probe there
can only re-measure B14's state. On #47 the prediction is, per verdict:

| verdict | at #46 (B14 §5) | predicted on #47 | why |
| --- | --- | --- | --- |
| `catalogue-add-object-kind` | `FAIL before=1 after=1` | **PASS** | the add's own call now keeps `more-work` armed through the whole publication ladder, so its `OperationCompleted` lands inside the call the probe reads `after=` from |
| `volume-brush-add-target-volume` | `FAIL before=0 after=0` | **PASS** | same path, same operation shape |
| `duplicate-selection` | `FAIL before=2 after=2` | **PASS** | the edit already landed at #46 (B14 measured the document ACCUMULATING) — only the one-command lag kept `after=` equal to `before=` |
| `delete-selection` | `FAIL before=4 after=4` | **PASS** | as above |
| `duplicate-reselects-clone` | FAIL | **PASS if the duplicate's interaction lane publishes in the same call** — it is the `InteractionWrite` unit of the same operation, so it rides the same fix; if it stays red, the remaining defect is the selection write itself (B13/B15's region), not the scheduling |
| `gumball-scene-delta` | FAIL | **likely PASS** — a transform is the same mutating typed-operation ladder; if it stays red, look at the relocate/transform publication lanes, not the turn status |
| `relocate-pose-delta` | FAIL | **likely PASS**, same reasoning |
| `clipboard` | FAIL | **PASS for the paste half** (paste is an ordinary mutating operation); the copy half is host clipboard plumbing this wave does not touch |
| `catalogue-drag-drop` | PASS (B14) | **stays PASS**, and for its own reason now rather than by borrowing the click's add — B11 §2c's dispatch thread is still open |

Two collateral readings worth asserting on #47:

1. **`[DEBUG] typed-operation slots instance=N live=L/64 peak=P` (B8's line) should show a SMALLER
   peak**, not a larger one: operations now finish inside their own call instead of accumulating one
   parked operation per command.
2. **`b14 settled … continuations=` should rise for command settles and fall to ~0 for drain polls.**
   A command's settle now carries the whole ladder (each publication unit is one ACK round trip by
   protocol, so expect tens of continuations, all of them PROGRESSING — every one carries an
   acknowledgement), and the drain poll should have nothing left to find. If a command settle is seen
   **quiescing at 128** (B14's `PLUGIN_UI_QUIESCENT_CONTINUATIONS`), that bound — not this fix — is
   what to revisit next: it is a publication-free streak counter and a healthy ladder never produces
   one, but a Worker stage that cannot finish inside a turn's `INTERACTIVE_TURN_WORKER_PUMPS` slice
   would.

---

## 7 Remaining risks

- **`has_runnable_typed_operations` is now true for the whole life of a mounted operation**, so an
  operation whose acknowledgement the host never sends keeps the actor answering `more-work` instead
  of going quiet. That is the intended trade (the host owes that ACK, and it can only know it owes it
  from this status), and it is bounded by the renderer's own 128/4096 continuation faults, which name
  the owning call. The alternative — re-presenting the page — is forbidden by the existing
  *"presented result page must not be republished before an explicit retry deadline"* law.
- **This is guest Rust; nothing changes in the browser until `component-release` is rebuilt AND
  materialised.** The coordinator owns that run (#47).
- **`a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` remains the crate's binary-killing
  flake** (B8 §6). It aborted two of the full runs in this wave at 145 B/turn against its 64 B ceiling;
  it is order-dependent and pre-existing, and it is why the full-crate comparison above uses `--skip`.
- **The three puzzle3d reds are untouched and unrelated** (§5), and two of them sit in regions other
  live waves own.

## 8 Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `MountedTypedCommandFullOperation::has_runnable_work`
  removed; `has_runnable_typed_operations` (trait doc + `VcsArtifactApp` impl);
  `next_advanceable_typed_operation`; `advance_typed_operation_publication_one`'s selector;
  `take_typed_operation_result_page`; `typed_operation_result_state_for_test`; the two law bodies and
  their `drive_host_call_with_deferred_acks` driver.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  — the two law entry points, the two `typed_operation_result_state_for_test` assertions, and the
  seven `ui_history_panel` call sites unblocked (§5).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`
  — the two assertions that encoded the old per-operation predicate.
- `🗑️generated/` — `wave-B16-plugin-lib-wave.txt`, `wave-B16-plugin-lib-wave2.txt`,
  `wave-B16-plugin-lib-wave3.txt`, `wave-B16-plugin-lib-baseline.txt`, `wave-B16-puzzle3d-lib.txt`
  and the extracted failure-name lists `wave-B16-names-{wave,wave3,baseline}.txt`.
- this report.

No temporary `[DEBUG]` logging was left anywhere: the `[DEBUG] actual …` lines that remain are the two
laws' own `#[cfg(test)]` measurement lines, matching the convention B0 and B8 already use in this
module. `🗑️generated` was written to and never deleted.
