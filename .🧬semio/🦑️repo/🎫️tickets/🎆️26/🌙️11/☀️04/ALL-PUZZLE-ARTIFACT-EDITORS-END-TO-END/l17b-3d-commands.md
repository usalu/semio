# L17b: 3D command, budget and fixture cluster

Status: edits applied, test verification pending the shared native runner (disk on /home was full, 0 bytes available to non-root, so the cold build could not link).

## Causes and fixes

1. Fixture receipts ("registered fixture tool run receipt differs from its original caller authority", 8 accept_suggestion_* tests and others)
   - Cause: `host_turn` in the raw app fixture renewed the publication turn with the fixture caller's own grant (caller.json, 512 MiB), while `retain_original_ownership_receipts` requires tool-run and history receipts to equal `policy.maintenance` (INTERACTIVE_COMPONENT).
   - Fix: publication runs under `policy.maintenance`, then the caller grant is restored.
   - File: `.../3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️context/🏗️raw-app-fixture/🦀️.rs` (`host_turn`).

2. "command wire exceeds its declared maximum" (145 KB import, chunked pick, over-budget import)
   - Cause: the Puzzle3d editor used the framework default producer (`RUNTIME_COMMAND_WIRE_MAXIMUM`, 64 KiB) for every command, so a whole-file `importSnapshot` could not be encoded. The 5D editor already declares `PUZZLE_IMPORT_RAW_BYTES` for it.
   - Fix: `Puzzle3dPlayApp::original_action_command_producer` returns an `OriginalActionCommandBridge` with `PUZZLE3D_IMPORT_RAW_BYTES` for `ImportSnapshot` and the runtime default otherwise. The law in `🔬️action-producer` now asserts the plugin declares its import wire budget.
   - Files: `.../✏️editor/🧩️play-app/🦀️.rs`, `.../✏️editor/🧪️tests/🔬️action-producer/🦀️.rs`.

3. "one pick builds the interaction topology exactly once" (left 0 right 1)
   - Cause: `PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS` was bumped under `#[cfg(test)]`, which is false in the lib when integration targets (`editor-gestures`) link it.
   - Fix: the counter is bumped unconditionally.
   - File: `.../✏️editor/🧩️play-app/🦀️.rs`.

4. "puzzle3d registers one window config owner" and `Some(19)` vs `Some(1)` window-option generations
   - Cause: `window_config_capture` answers `None` until the native partition birth is pumped (`ensure_for_view`), and a born partition starts at a store generation above 0, so absolute `Some(1)` is outdated.
   - Fix: the fixture gains `Puzzle3dApp::window_config_generation`, which pumps the partition birth first. The window-option laws compare against the born generation plus one.
   - Files: raw-app-fixture `🦀️.rs`, `.../✏️editor/🧪️tests/🔬️panels/🪟️window-option-laws/🦀️.rs`.

5. puzzle-3d `snapshot-sqlite`
   - `borrowed_scale_matches_neutral_list_shape`: `Puzzle3dScale::from_value` accepted lists of four or more items (`>= 3`); it now requires exactly three. File: `.../🧬️schema/📚️document-model/🦀️.rs`.
   - `declared_txt_preserves_complete_fields_and_every_word`: the bun helper ran `db.exec("")`, refused by current bun ("SQL string mustn't be blank"); the script now execs only a non-empty edit. File: `.../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`.

6. Source-string law tests (hostile static laws, leftover-commit, retained wiring)
   - Emulating `editor_source()` and `routes_to_one_grant_reducer` by script against the current tree: every marker and arm is present (`"importSnapshot" => bound!(... Puzzle3dWindowCommandWork ...)`, `createAttraction`, `focusSelection`, ...) and no arm routes to `BoundedFirstStepCommandWork`. The retained-wiring test was already updated in the tree. These failures in the old log are therefore stale and need a rerun to confirm.

## Not mine (left for L17a or framework)
ordered-map retire panics, terminal-empty witness, close custody, mutation-latency turn budget (184 turns vs 8), "typed result page reached Drop", task-resume "original command scope requires its actual decoder recipient", local-interaction read termination.

## Incident
While applying fix 1 and 4, a write hit `ENOSPC` and truncated the raw-app-fixture file to 0 bytes. It was restored from git HEAD plus the intended edits (byte-identical to the staged copy) as soon as space reappeared. /home is ext4 with reserved blocks: free 14 GB, available 0.

## Verification queue
Appended to `🗑️generated/l12-req-3d` (runner log is `/dev/shm/l17-3d.log`, look for `L12-STEP <line>`): `-p semio-s-artifact-puzzle-3d --test snapshot-sqlite`, and `-p semio-s-artifact-puzzle-3d-editor --test` each of `editor-budgets`, `editor-defects`, `editor-manifest`, `editor-action-producer`, `editor-surface`, `editor-gestures`. The first cold build was still compiling when this note was written, so before/after counts (314 pass / 107 fail baseline) are not yet measured. Remaining-failure triage must be redone from that log; expect the lifecycle cluster (L17a) and mutation-latency to remain.
Additional fixture detail: `Puzzle3dApp::window_config_generation` was changed to stringify faults (`Fault::from(format!("{error:?}"))`) and to return through `fault_of` on a refused partition birth.

## Status at 15:46
No rerun results yet: /dev/shm/l17-3d.log is still in the cold compile (no L12-STEP or test result lines). Nothing is verified by a test run.
