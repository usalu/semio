# FP3 — `semio-framework-plugin --lib` to a trustworthy gate

Slice: continue FP2. Inherited claim: **730 passed / 82 failed** (`fp2-round5-suite.txt`).
All runs are `cargo test -p semio-framework-plugin --lib --no-fail-fast` under preamble rule 25
(private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp1`, shared build-dir), one cargo at a
time, foreground, no sub-agents, no wasm.

Started 2026-09-20 (session 5b+).

## 1. Round table

| round | worked | passed | failed | capture |
|---|---|---|---|---|
| 0 (FP2 round 5, inherited claim) | — | 730 | 82 | `fp2-round5-suite.txt` |
| 0′ (FP3 re-measure, same tree) | — | **734** | **81** | `fp3-round0-suite.txt` |
| 1 | command-log recorders made synchronous + a first (retirement-time) migrated append | 736 | 79 | `fp3-round1-suite.txt` |
| 2 | the append moved onto the durable RECEIPT (§3) | **740** | **75** | `fp3-round2-suite.txt` |
| 3 | the settle-receipt fold in `ContractApp::dispatch_typed` (§3.4) | **743** | **72** | `fp3-round3-suite.txt` |
| 4 | the reserved-route envelope echo (§4) | **750** | **65** | `fp3-round4-suite.txt` |
| 4′ | round 4 repeated, byte-identical tree (flakiness measure, §6) | 749 | 66 | `fp3-round4b-suite.txt` |
| 4″ | round 4 serial (`--test-threads=1`, §6.2) | **752** | **63** | `fp3-round4-serial.txt` |

**Net: 734 → 750 passed, 81 → 65 failed in parallel (752/63 serial). 17 laws that were red now
execute and hold; none went red that is not in the measured flaky population of §6.** Three roots
account for all of it: one product defect (§3, 8 laws), one fixture contract the reserved route never
stated (§4, 6 laws) and one fixture-level settle fold (§3.4, 3 laws).

## 2. Re-measure and bucket census

FP2's 730/82 re-measured at **734/81** on the same tree (the movers are the known load-flaky class,
§6). Panic-message census of `fp3-round0-suite.txt` (81 reds):

| count | first panic line |
|---|---|
| 17 | bare ``assertion `left == right` failed`` |
| 3 | `app.message` — `registered fix…` (composed child lane) |
| 3 + 2 + 1 | `interactive-job.output-envelope` on `paste` / `copy` / `cut` |
| 3 | `interactive-job.missing-factory` (`increment`, addressed-window action, active mode-owned command) |
| 2 | `fixture app has no external owner` |
| 2 | `assertion failed: matches!(PluginApp::maintenance_step(…` |
| 2 | `registered fixture did not reach its exact terminal-empty witness` |
| ~48 | one-off per-law reds, each its own question |

The brief's premise for the biggest bucket did **not** survive measurement: of the 17 bare
`left == right` laws only **three** actually read `result.mutations`
(`operation_action_emits_kernel_op_with_true_inverse`, `operation_command_emits_kernel_op_with_true_inverse`,
`amend_dispatch_reports_only_this_dispatch_new_operations`). The rest are heterogeneous, and the
largest coherent root among them turned out to be a **product defect**, not an assertion style — §3.

## 3. The migrated command log never existed (product defect, 8 laws)

`dispatch_emit` appends exactly one `CommandLogEntry` per dispatch (`🔌️plugin/🦀️.rs:24995` for a
zero-operation emit, `:25039` for an edit-linked one). The **mounted typed-operation ladder never
walks `dispatch_emit`**: `publish_mounted_typed_operation_unit` drives `store::begin_apply_batch` /
`config_store::begin_apply_batch` directly, and there was no `record_command` anywhere on that path
(`grep -n "push_log_entry("` finds exactly four call sites: `record_command` and the two
`backfill_command_log` ones).

Consequences, all measured from `fp3-round0-suite.txt`:

- every migrated document command reached the history panel as `backfill_command_log`'s anonymous
  row — `action_id: "apply"`, the verb, its declared `ActionKind`, its own label and its revert
  target all lost (`an_operation_action_appends_one_command_log_entry_linked_to_its_edit`:
  left `"apply"`, right `"increment"`);
- every migrated config-only command the same, as `"configApply"`
  (`an_op_less_view_action_is_logged_with_edit_id_none_and_count_one`: left `"configApply"`, right
  `"select"`);
- a dispatch that published **no** durable lane (a `Mutation` verb whose reducer emitted nothing,
  every `View` verb) produced no row at all — `an_operation_kind_action_with_zero_operations_still_logs_one_entry`
  (0 vs 1) and `view_dispatches_remain_distinct_across_interleaved_entries` (`[]` vs `[1, 1, 1]`);
- `undo`/`revertToCommand` could not find their target row by `action_id`.

This is live-product behaviour, not a fixture artefact: since the typed-command migration the
History panel of every migrated plugin shows `apply` for everything.

### 3.1 The fix, and why it sits on the receipt rather than the retirement

Round 1 put the append on `retire_typed_operation_unit` — the "ONE release site" its own docstring
names. It fixed only the zero-lane case and went **736/79**. The measured reason: an operation mints
its `Ui` page *between* the durable receipt and its retirement, rendering the history body runs
`refresh_cache` → `backfill_command_log`, so by retirement the edit this row owns is already filed
under `apply` and the append correctly declines it as "already logged".

Round 2 moved it onto the durable **receipt** (`ArtifactStoreOneItemAdvance::Published`, the
`Artifact` / `Config` / committed-child arms) — `record_typed_operation_lane` — and kept the
retirement hook only for the no-lane case (`record_settled_typed_operation_command`). A second
durable lane of the same operation folds its edit into the row the first opened, so a verb writing
both stores still gets one row carrying both ids, exactly as `dispatch_emit` does. A coalesced
gesture finds its amended edit already logged and appends nothing (`amended_same_edit`'s rule).
**740/75.**

### 3.2 `record_command` / `push_log_entry` are no longer `async`

Both are `#[cfg(test)]`-free production fns with **no suspension point** (`registry.get`,
`registry.get_command`, `Vec::push`, `HashSet::insert` — `project-semio-async-convention-debt`'s
"fix the callee"). The receipt arms and `retire_typed_operation_unit` are sync, so the append could
not be made from them otherwise. 17 call sites lost their `.await` (16 in `🔌️plugin/🦀️.rs`, one in
`⏯️tool-run/🦀️.rs`); scripted with an exact-line guard and verified at exactly 36 changed lines
(18 pairs) against a pre-edit snapshot.

### 3.3 Laws this turned green

`an_operation_action_appends_one_command_log_entry_linked_to_its_edit`,
`an_op_less_view_action_is_logged_with_edit_id_none_and_count_one`,
`an_operation_kind_action_with_zero_operations_still_logs_one_entry`,
`view_dispatches_remain_distinct_across_interleaved_entries`,
`a_coalesced_gesture_appends_exactly_one_command_log_entry`,
`undo_and_redo_append_entries_and_never_shrink_the_log`,
`revert_to_command_restores_the_snapshot_and_appends_one_entry`,
`view_action_with_inverse_is_revertible_and_backwards_restores_app_runtime_state`.

### 3.4 The settle-receipt fold — three laws, one fixture line, no law rewritten

The three laws that genuinely read `result.mutations` were **not** rewritten. FP2's
`ContractApp::dispatch_typed` already settles and folds the receipt's effects/events/ui-scope back
into the admission; round 3 folds the remaining half the same way, so every law keeps its original
assertion text:

```rust
let after_edit_id = self.0.test_last_edit_id();
if after_edit_id.is_some() {
    let tail_offset = if after_edit_id == before_edit_id { before_tail } else { (0, 0) };
    let settled = self.0.test_result_from_last_edit(verb, meta, tail_offset).await;
    admitted.mutations = settled.mutations;
    admitted.inverse_group = settled.inverse_group;
}
```

It reuses the production `result_from_last_edit` through three `#[cfg(test)] pub(crate)` accessors
(`test_result_from_last_edit`, `test_edit_tail_lengths`, `test_last_edit_id`) and reproduces
`dispatch_emit`'s own `amended_same_edit` tail rule, which is why
`amend_dispatch_reports_only_this_dispatch_new_operations` — the law that asserts a coalesced edit
reports only the one operation this dispatch added — holds too. Green:
`operation_action_emits_kernel_op_with_true_inverse`,
`operation_command_emits_kernel_op_with_true_inverse`,
`amend_dispatch_reports_only_this_dispatch_new_operations`.

This is the brief's "settle-receipt rewrite" done once in the fixture rather than as a
17-law scripted packet — the packet was never needed, because the measured census (§2) found only
three laws of that shape.

## 4. The six clipboard laws — one line, one root

FP1 counted 4 and FP2 counted 6 `interactive-job.output-envelope` reds on `copy` / `cut` / `paste`
and left the bucket unexplained. It is one root, and it is not a route defect.

`run_framework_reserved_job` (`🔌️plugin/🦀️.rs:22004-22007`) commits a framework-RESERVED route only if
the job hands the admitted wire envelope back byte for byte
(`retained_payload_eq_slice(&candidate.output, raw)`), refusing
`interactive-job.output-envelope` otherwise. Every generated `framework_reserved_job!` body honours it
(`output: retained_job_payload(cx, CommitOutput, &self.raw)`, `:16049`). An app may take a reserved
route over through `ArtifactApp::build_reserved_tool_job`, and `TestApp` does for exactly
`copy | cut | paste` — but `TestClipboardReservedJob::step` returned
`RetainedJobPayload::empty(CommitOutput)`, so every clipboard law died at the envelope gate before its
emit was ever examined.

One line in the fixture (`🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`, `TestClipboardReservedJob::step`)
now echoes `self.raw_wire`, and all six turn green at once:
`copy_emits_clipboard_write_effect_with_no_operations`,
`copy_on_empty_selection_is_a_benign_no_operation`,
`cut_removes_label_and_emits_clipboard_write_as_one_undo_unit`,
`paste_materializes_fragment_at_original_anchor`,
`paste_with_no_fragment_arg_is_a_benign_no_operation`,
`paste_with_non_original_anchor_reaches_the_app_placement`.

**The product finding to carry forward:** an app that overrides `build_reserved_tool_job` silently
inherits the reserved route's envelope obligation, and nothing in the trait's signature or its
docstring says so. Any real plugin that takes over `copy`/`cut`/`paste` and returns an empty commit
output gets `interactive-job.output-envelope` at runtime with no hint of the cause.

## 5. The SURFACE fixture packet — measured, scoped, NOT attempted

`editor_fixture_still_mutates_normally` is the only red in `🧬️mutation-fixtures-surface/🦀️.rs`
(the other seven laws in that file pass). It fails `increment:` with
`interactive-job.missing-factory`, and FP2's §4 reading of it holds: `SurfaceEditorFixture` is an
`ArtifactEditor` reached through the registry-less `new_app::<EditorApp<SurfaceEditorFixture>>()`, and
`ArtifactEditor::command_id`'s trait default (`🔌️plugin/🦀️.rs:32388`) answers the literal
`"typed-command"` for every variant — the third copy of that default, beside `ArtifactApp`'s
(`:11951`) and `ArtifactViewer`'s (`:32781`), each documented as "correct but generic".

It needs the whole §2-of-FP2 packet for one law: a `command_id` override, an app-owned bounded factory
for `EditorApp<SurfaceEditorFixture>`, an `Artifact` publication lane, the matching store preparation
authority, a migrated registry and an instance-bound, self-closing fixture. **Deliberately not
attempted in this slice**: the clipboard root above (§4) was six laws for one line and the command-log
root (§3) was eight for one product fix, so the budget went there. Nothing about it was guessed at or
half-landed — the file is byte-identical to how FP2 left it.

## 5. `local_interaction_cold_transaction_receipts_and_encoded_route_rejection`

**It is not FP2's regression and it is not a product defect. It is a law-versus-law conflict a peer's
feature created in this same working tree today, and the two laws cannot both hold.**

The law's first clause sends an encoded `AppCommand::TransactionPrepare` with empty `prepared_ops`
through `PluginCommandIngress::Encoded` and requires an `AppFrame::Error` carrying
`plugin.command-route-state-machine-required`
(`🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:399-403`). That fault has exactly one raise
site — `PagedAppCommandDecodeCursor::step`'s header arm
(`📡️spr/🧵️channel/🦀️.rs:1945`) — and it fires only for a tag `route_field_plan` does not answer.

`git status --short` shows `📡️spr/🧵️channel/🦀️.rs` staged-modified, and `git diff HEAD` on it shows the
whole retained route-decoder mechanism arriving as **new, uncommitted** work, including
`17 => Some((3, true, 2))` — `TransactionPrepare`'s own `prepared_ops` roster plan. The peer landed two
laws of their own alongside it (`📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs`):

- `paged_ingress_admits_every_transaction_route` asserts that an encoded `TransactionPrepare` **with
  `prepared_ops: Vec::new()`** now decodes byte for byte — literally the command this law requires to
  be refused;
- `paged_ingress_still_refuses_an_undeclared_route_by_name` keeps the
  `plugin.command-route-state-machine-required` refusal, by name, for an undeclared tag (22).

So the refusal the old clause guarded is preserved verbatim one module down; what changed is that
`TransactionPrepare` now *has* the "route-specific retained decoder before admission" that the fault's
own message demands. The clause's premise was retired on purpose.

**Left red and untouched, deliberately.** Deciding what the local-interaction law should assert instead
(the new round-trip, or an undeclared kind) is the route owner's call, not a guess from this slice —
and the three transaction-receipt clauses that follow it in the same law still pass on the decoded
route, so nothing about the receipts themselves is in doubt. FP2's §5 item 2 ("the cold route
genuinely admits something it should not") should be struck from the record: the cold route admits
exactly what a law that landed today says it must.

## 6. The "load-flaky" laws are not wall-clock laws — measured, and the brief's cure does not apply

FP1 §4.4 and FP2 §5 record **three** load-flaky wall-clock/turn-budget laws and this slice's brief
asked for "a deterministic basis (injected clock / turn count)". Both the count and the diagnosis are
wrong, and this is the measured correction.

### 6.1 The population is seven, measured by repeating one tree

`fp3-round4-suite.txt` and `fp3-round4b-suite.txt` are two consecutive runs of the **byte-identical
tree** (750/65 and 749/66). Seven laws move between them:

| law | its actual failure |
|---|---|
| `every_inbound_request_row_is_answered_on_the_turn_it_arrives` | `plugin.reactor-close-authority`: **"surface handback free list exhausted"** |
| `retained_operation_continues_after_command_admission_until_publication_and_retirement` | `plugin.internal`: **"instance busy or poisoned: 7"** |
| `a_100kib_measures_section_fits_document_node_cap` | `ui.fixed-capacity`: **"fixed UI admission failed at section-root"** |
| `tool_run_scene_render_carries_the_trace_lane_and_honours_the_echoed_cursor` | the same `ui.fixed-capacity` at `section-root` |
| `n_pending_retained_surfaces_converge_in_bounded_crossings_that_each_carry_something` | "whole patch requires exact typed retirement" |
| `guest_turn_execution_resets_for_every_turn` | `assert_eq!` on a settled turn's microseconds |
| `tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks` | 4 of 771 appends over 2 ms (worst 4.17 ms) |

**Six of the seven are process-global fixed-arena exhaustion, not clocks**: a shared surface handback
free list, a shared instance slot (`7`, hard-coded by several fixtures), and the fixed UI admission
arena behind `paged_text_carrier`'s `try_children` (`🔌️plugin/🦀️.rs:484`). Every test in the binary
draws from the same arenas; which law loses depends on interleaving, which depends on machine load.
An injected clock or turn counter would fix **none** of them.

### 6.2 Serialisation is a partial cure, and that is itself the finding

`fp3-round4-serial.txt` (`-- --test-threads=1`, 141.9 s vs 34 s): **752 passed / 63 failed.** Three of
the seven turn green — the ones that contend for the handback free list, the shared instance slot and
the reconcile surface. `a_100kib_measures_section_fits_document_node_cap` is red **serially** and green
in parallel, so the UI admission arena is not merely contended, it accumulates across tests **in
order** and is never reset between them. That is the residual ±3 on every number in this report, and
it is a real testkit defect (a per-test arena reset), not load noise.

### 6.3 The one genuine wall-clock law

`tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks` asserts a 2 ms
per-append ceiling over 771 real appends. There is no deterministic basis for it that preserves what it
measures — an injected clock would make it assert nothing at all. It was **not** touched, its ceiling is
byte-identical, and the honest cure is to run it off a loaded fleet, not to weaken it.

## 7. `interactive-job.missing-factory` remainder and the singles

Three `interactive-job.missing-factory` reds remain, unchanged in count from FP2:

- `editor_fixture_still_mutates_normally` (`increment:`) — the SURFACE packet, §5;
- `addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler`
  (`addressed window action:`);
- `manifest_mode_command_requires_the_active_structural_owner` (`active mode-owned command:`).

The last two are the manifest-command routes FP2 named: a window-addressed action and a mode-owned
command resolve a factory key the `TestAppCommandFactory` key set does not carry. Not attempted.

Final bucket census of the 65 (`fp3-round4-suite.txt`):

| count | bucket |
|---|---|
| 11 | bare ``assertion `left == right` failed`` (down from 17; the remainder are per-law questions, not one shape — see §2) |
| 4 | fixture close never reaches terminal-empty (FP1 §4.3, untouched by FP1, FP2 and FP3) |
| 3 | `app.message` on the composed-child `compositeEdit` lane |
| 3 | `interactive-job.missing-factory` (above) |
| 2 | `fixture app has no external owner` (envelope decode worker) |
| 2 | `assertion failed: matches!(PluginApp::maintenance_step(…` |
| ~7 | the flaky population of §6 |
| ~33 | one-off per-law reds, each its own question |

## 8. Honest gaps

- **The SURFACE packet (§5) was not attempted.** It is one law for a full fixture packet; the budget
  went to the two roots that were six and eight laws each. The file is byte-identical to FP2's.
- **The UI admission arena is not reset between tests (§6.2).** Found and measured, not fixed. It is
  the reason no number in this report is repeatable to better than ±3, and it is the single highest-
  value item left for the next slice: without it the suite cannot be a gate at all, whatever its
  pass count.
- **`local_interaction_cold_transaction_receipts_and_encoded_route_rejection` (§5 of FP2) is left
  red on purpose**, with the evidence that it is a peer's landed feature retiring the clause's
  premise, not a defect. Resolving it is the route owner's call.
- **No law was deleted, `#[ignore]`d or loosened.** Every ceiling is byte-identical: the 2 ms overlay
  budget, `UI_TEXT_MAX_BYTES`, `UI_BUILT_CHILDREN_MAX`, the 2 MiB bounded thread stack, the
  maintenance grants. Nothing was given a longer timeout.
- **The crate is compile-green at every round** (`fp3-check-1.txt`, `fp3-check-2.txt`, and every suite
  capture compiles the lib + lib-test targets clean).
- **No dependent crate was re-checked, and none needed to be**: every production change is
  crate-private — `record_command` and `push_log_entry` are private methods of `VcsArtifactApp`, the
  three new `MountedTypedCommandFullOperation` fields are private, `retained_job_payload` was already
  `pub(crate)`, and the three new accessors are `#[cfg(test)] pub(crate)`. No public item changed
  signature.
- Every number here is read from a captured run in `🗑️generated/`; nothing is claimed that was not
  executed.

## 9. Files changed

Framework product code (2 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
  - `record_command` and `push_log_entry` are no longer `async` (§3.2), 16 call sites
  - `record_typed_operation_lane`, `record_settled_typed_operation_command`,
    `typed_operation_command_kind` — the migrated command-log append (§3.1)
  - `MountedTypedCommandFullOperation` gains `published_artifact` / `published_config` /
    `command_logged`, set in the `Artifact` / `Config` / committed-child receipt arms
  - `retire_typed_operation_unit` calls the no-lane recorder
  - `test_result_from_last_edit` / `test_edit_tail_lengths` / `test_last_edit_id`
    (`#[cfg(test)] pub(crate)`, §3.4)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` — one `.await` dropped (§3.2)

Framework test fixtures (2 files):
- `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — the settle-receipt fold in both
  `ContractApp::dispatch_typed` and `ContractComposedApp::dispatch_typed` (§3.4);
  `TestClipboardReservedJob::step` echoes its admitted envelope (§4)
- `…/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` — the three new fields in its four
  `MountedTypedCommandFullOperation` initializers

Captures (all in `🗑️generated/`): `fp3-round0-suite.txt`, `fp3-check-1.txt`, `fp3-check-2.txt`,
`fp3-round1-suite.txt` … `fp3-round4-suite.txt`, `fp3-round4b-suite.txt` (repeat of the same tree),
`fp3-round4-serial.txt` (`--test-threads=1`).
