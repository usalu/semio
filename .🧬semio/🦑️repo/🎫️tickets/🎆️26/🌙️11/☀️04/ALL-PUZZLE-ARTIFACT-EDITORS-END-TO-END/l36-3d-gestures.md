# L36: 3D editor-gestures failures (l32-3d.log)

Status: fixes 1 and 2 applied. Nothing compiled or run (RAM is taken by the running build). Fix 3 is analysed only, because the clean fix needs measurement first.

Paths below are shortened. `FW` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `ED` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

## 1. `cancelling_clipboard_admission_*`: "the cancelled caller must release its action ingress record"

Root cause: the framework source-close ladder quoted the caller's action ingress, but it never closed it and never counted it as part of being terminal.
- `FW/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs:190` (`maintenance_source_close_demands`) returns `ingress.original_action_ingress_close_demands` once the command scope is not owned.
- `maintenance_source_close_step` (:200) had no branch that calls `ingress.close_original_action_ingress`. That function (`FW/🔌️plugin/🧵️instance/🏪️stores/📥️staging/📮️action-ingress/🦀️.rs:197`) had no caller anywhere in the tree. The step fell through to `close_action_host_events` instead.
- `maintenance_source_terminal_is_empty` (:213) did not check `owns_original_action_ingress`. After cancellation the test's close loop therefore saw "terminal" as soon as the mount, permit and command lease were gone.
- The leaked record is not `ready`: `dispatch_action`/`dispatch_command` return the source with `terminal=false` while the Cut is pending (`…/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:133-135, 282-284`). The `Ready`-scope maintenance ladder (`🪜️maintenance-ladder/🦀️.rs:116`) never picks up a record in that state, so only the `Original`-scope close can retire it. The test's expectation is correct, not stale.

Fix (`FW/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs`):
- The demand branch now applies only when the caller owns ingress: `!owns_original_command_scope(cx) && ingress.owns_original_action_ingress(cx)`. Previously, with no ingress, it returned a zero demand, which hid the overlay, typed-tag and emission demands quoted after it.
- The step has a new branch, placed after host-event-source and before tool-overlay, with the same condition. It runs `ingress.close_original_action_ingress(same, actual)` and reports the receipt as an `original_lifecycle_delta`, the same pattern the birth branch uses.
- `maintenance_source_terminal_is_empty` now also requires `!ingress.owns_original_action_ingress(cx)`.

Callers of the terminal check (natural publication, picker publication, the cleanup-pump drain at `⚛️reactor/…/🧯️cleanup-pumps/🦀️.rs:257-263`) now also drain the caller's ingress record. This is the intended "return every original source" behaviour. The drain's receipt check `step.progress()==actual` still holds because the branch reports the measured delta.

## 2. `one_pick_builds_the_interaction_topology_once…`: builds = 0

Root cause: L17b's fix was lost. In `ED/🧩️play-app/🦀️.rs:383` the bump of `PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS` was gated by `#[cfg(test)]` again (it is like that in HEAD as well). `cfg(test)` is false for the library when the `editor-gestures` integration target links it, so the counter never moves. Small and large both read 0, which is why `small == large` passed and `== 1` failed.

The pick path still builds the topology once per generation: `FW/🔌️plugin/🧵️instance/🖼️render/📥️staging/🧭️interaction/🦀️.rs:120-133` (`app_interaction_topology` memo).

Fix: removed the `#[cfg(test)]` attribute (a one-line targeted replacement). It belongs to the same file as the other lane (`🧩️play-app`), so whoever writes that file next must keep the unconditional bump.

## 3. `one_mutation_publishes_in_a_bounded…`: 133 turns / 1610 units (store = 1485), bound 8

Two size-independent factors multiply.

A. About 1485 one-item store steps per mutation. Each step is its own pump unit.
- `FW/🏪️store/⚙️engine/🧳️bundle/🦀️.rs:~135`: `item_grant = {maximum_items: grant.maximum_items.min(1)}` drives census, item close, fold (`fold_batch_item`), `begin_next` and the phase flips.
- The commit preflight digest (`🧳️bundle/🦀️.rs` `PreflightingCommit`, `stage.digest_owner`) advances `NativeBatchDigest` (`FW/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs`). It always uses `child(grant)`, which sets `maximum_items=1`. It runs:
  1. the `RetainedCloneSource` admission and projection of the whole edit (forwards + inverse + meta), one node per step;
  2. the `ArtifactCanonicalEditSealCursor` seal, one node per step;
  3. `take_edit`;
  4. close.
- Then `Publishing` (`advance_original_commit` for outbound causal message bytes, plus diagnostics) and, after ACK, the closing retirement ladder (`advance_closing_publication`). All of these are counted as `Store` units because no result page is pending (`…/📬️publication/🏃️mounted-run-and-pump/🦀️.rs:216`).
- The cost scales with the node count of the edit, not the document. That fits translate (pose payload) 1485 against delete 1137. B54 measured 17 store units, so the per-node digest/admission ladder is the most likely regression. This is an inference; it is not measured.

B. Only about 12 units fit in one host turn.
- The pump caps each turn at `original_plugin_turn_grant(mounted_policy.preparation, retained)` = `INTERACTIVE_COMPONENT` (64 items) (`🏃️mounted-run-and-pump/🦀️.rs:154,206`).
- Every store unit opens its own native receiving scope (`📬️typed-operation-unit/🦀️.rs:~165`, `with_native_receiving_quoted`). That costs `original_receiving_scope_headroom()` items (`FW/../🧵️job/⏱️context/🛬️owned/🦀️.rs:44-48`: outcome items + 2) on top of the unit's own item.
- 1610 / 133 ≈ 12 units per turn.

Clean fix (not applied blind; it touches the store's accounting laws and the `🥇️one-item-publication…` source law test):
- (a) The digest seal and admission should advance many nodes per step under the issued grant instead of `child()` items=1. The best option is to seal the borrowed staged edit directly, without a retained clone admission.
- (b) `advance_apply_batch` should drain consecutive one-item phases inside one receiving scope while the funded grant allows, as the `Preparing` branch already does with its `for _ in 0..4096` loop. This amortises the scope headroom.
- (a) is needed to reach ≤ 8 turns. (b) alone gives about 1485 / ~60 ≈ 25 turns.

Instrumentation needed to confirm the amplifier before fixing:
- A temporary `[DEBUG]` thread-local histogram in `advance_apply_batch_inner`, keyed by branch: census, item_closing, folding, begin_next, preparation loop, cursor inverse reserve, phase flips, digest {`admission.phase()`, seal cursor, take, release}, original_commit/message, diagnostics, publishing.
- The same for the sub-owner of `advance_closing_publication`.
- Dump it in the `mutation_latency` assertion message for translate on concrete-forest.
- Test line: `-p semio-s-artifact-puzzle-3d-editor --test editor-gestures one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`.

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs` (lines 190, 209, 213)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️play-app/🦀️.rs` (line 383)
