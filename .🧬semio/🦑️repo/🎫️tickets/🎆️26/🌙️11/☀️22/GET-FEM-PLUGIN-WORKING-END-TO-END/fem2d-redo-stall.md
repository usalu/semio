# FEM 2D: redo stall and wedged solve after undo (status: browser root cause narrowed, native repro, fix NOT verified)

## Browser facts (wasm of 19:52, 6086, own probes `matrix-v3/rd-probe*.mjs`, outputs deleted with `C:\t\rd*`)

| Scenario | Result |
|---|---|
| add node, wait for the solve (16 s), Ctrl+Z, later Ctrl+Shift+Z (quiet sampling or harness-style tree polling) | undo lands in 3-5 s, redo lands in 3 s. History works. After the redo the results show `Updating results... Failed 0 / 0 Previous Results` and do not recover within 20 s |
| add node, Ctrl+Z 1 s later (the add's solve still running), then Ctrl+Shift+Z (the matrix `hist.*` recipe) | undo lands after 5 s, **redo never lands in 95 s**, results frozen on `Updating results... Failed 0 / 0` for the whole time |
| same wedge, then a NEW edit (addNode) | the edit lands (about 5 s), but results stay `Failed 0 / 0`: **no solve is ever spawned again** |
| same wedge, then Ctrl+Z again | nothing lands in 19 s ("later undos fail") |
| same wedge, then select tree rows e5, n2, e6 | the tree row highlights but Inspection keeps the Summary: **selection no longer reaches Inspection** (the coordinator's cascade) |
| console during all of it | only `input #8 undo refused: dispatch-failed ... interactive-job.stale` (the undo's typed publication cancelled because the store moved on); no panic, no trap |

So the 2D failure is not keyboard-specific and not the redo stack: a solve that is **in flight when a history verb moves the document** ends up cancelled, the next generation is never bound/spawned, and every later interaction that needs the guest to make progress (redo, undo, new solves, selection to Inspection) starves behind it. `Failed 0 / 0` is the cancelled previous generation drawn stale (`FaultedCancelled` renders as "Failed"); it is a cancellation, not a legitimate singular-model failure (the label is misleading and should read as a superseded run).

## Native reproduction (law, `🔁️results-settle`)

`$A/◻️2d/.../✏️editor/🧪️tests/🔁️results-settle/🦀️.rs`, run through `$T22/rd-law.sh` (private lane `C:\t\rd`):

- Pump now behaves like the host job registry: ended jobs keep their entry (a later `CancelJob` reaches a terminal job), `running()` counts only live jobs, `assert_settles` returns early.
- New law `undo_and_redo_pressed_over_the_edits_running_solve_settle_2d`: edit, 30 host rounds (the edit's solve is really in flight), undo, 30 rounds, redo, settle. Result (log `law-2d-n`): the document counts follow (undo and redo land), but **the results do not settle: `jobs left 2`** after 600 rounds, i.e. the two solves cancelled mid-flight never terminate (the final solve converged). This is the native twin of the stuck solve in the browser.
- Earlier laws (`undo_redo_between_settled_solves...`, `a_hidden_results_window...`, `history_verbs_interleaved...`, `redo_during_the_undo_re_solve...`) pass their bodies natively but never had a solve in flight (the first spawn needs more than a few host rounds, only one `SpawnJob` was ever emitted), so they are only regression guards, not the repro.
- Added `undo_and_redo_of_a_cascading_delete_restore_and_remove_the_same_entities_2d` in `🕰️history-undo-and-alternatives` (delete a node with its elements, undo, redo, twice).

## Changes made (2D, aligning it with its 3D twin; not yet proven to remove the wedge)

- `🧵️session/💼️job-bridge-and-live-mount`: `MountedBoundedJob::cancel` now records the `Cancelled` terminal (3D `MountedJob::cancel` does) and cancels only the state of its OWN identity (it cancelled whatever state sat in the fixed shell slot, an ABA risk; `step` already had the guard). Also `replay_input` (a peer's new `BoundedJob` trait item broke the 2D and 3D build for over 20 minutes; trivially implemented in both: `self.input.as_deref()`; 3D file `🧵️session/⚙️mounted-job`).

## Open / not done

1. Root cause of the non-terminating cancelled solve: the added `[DEBUG] leftover job=... step=...` print in the law (run `law-2d-o`/`p`) names what the leftover jobs answer to `step()` (Running(None), Running(Some), Failed, Err). Result: see "Final run" below.
2. Browser verification of any fix needs the coordinator's next wasm publish. Browser facts above are pre-fix; `matrix-v3 run.mjs 2d --only hist.` on 6086 (2d run of this session: `hist.undo-key` PASS, `redo-shift` FAIL 36->36, `redo-y` FAIL, `buttons` FAIL click timeout, `command-log` PASS, `palette-undo-redo` FAIL undo:false).
3. Pre-existing, not mine: every law that completes a real solve aborts at app close with `the receipted close releases every born root: left 21576 right 21704` and `retained job payload abandoned original pages or operation ledger` (identical numbers in every test, also for a plain first solve + close, `law-2d-g`). The law bodies finish before it, but the test process exits non-zero, so these laws cannot be reported green until the framework close accounting is fixed (peer area: job payload pages / receipted close).
4. Gumball law `a_gumball_move_edited_in_history_replays_its_downstream` (coordinator request): reproduced red natively (`law-2d-gc`, crate `semio-s-artifact-fem-2d-editor-interaction --lib`): "the standalone store retires to its terminal-empty shell". It builds a standalone `ArtifactStore`, overwrite-commits a replay and retires it; that crate does not depend on the session crate or on anything I changed (my edits are the session job bridge, `replay_input` in the two session job files and test files). Cause is in the store replay/retirement code that peers touched in the last 25 h (`🏪️store/🔁️replay/*`, `🔗️read/♻️retirement`, `🔗️read/🔒️registry`, `📬️paged-one-item/*`, `🏗️initialization/🔁️replay`). Not fixed by me.
