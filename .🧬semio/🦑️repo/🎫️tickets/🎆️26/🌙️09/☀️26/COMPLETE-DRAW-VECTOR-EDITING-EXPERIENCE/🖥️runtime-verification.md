# Draw Runtime Verification

The existing serve task is running Vite at http://127.0.0.1:6064. The Draw activation task completed successfully, including its component build; that build predates the lasso changes.

Using the Browser skill, a new tab was created after the older handle had disappeared. Navigation and reload succeed, but the DOM remains empty. A DOM inspection shows title `semio · os`, readyState `interactive`, and an empty `div#root`; the entrypoint module script is present. This establishes neither a usable editor nor successful interaction.

The Vite log repeatedly reports ECONNREFUSED for `/trusted-catalog/plugin-modules`. Process7679 remains alive as the shared default hub owner, but a listener check found no service on port8787. The owner log shows it is still publishing/building its trusted catalog. It was not stopped or duplicated. The failed catalog connection is observed, but its causal relationship to the empty root has not been established.

The earlier filtered native test task completed: 15 passed, 292 skipped. The current full native library suite is running separately and includes the new lasso tests.

Next: finish the full native suite, rebuild the latest Draw component, inspect the actual rendered UI and run the acceptance workflows with console diagnostics. No end-user browser workflow is verified at this checkpoint.

## Rendered Shell Checkpoint

The earlier tab was no longer part of the browser session. New tab4 navigated to the same preview URL. A DOM read reached readyState complete and showed semio · drawing, Editor, Example/Demo, Canvas, Actions, Search and Utilities. Subsequent snapshots showed the shell with Loading plugins and the hub signed out. The app root is therefore no longer consistently empty, but no stable canvas interaction is established. The hub log progressed through snapshot-core and descriptor-emitter catalog stages. Latest component activation is now running in `activate-stroke-lasso.txt` after a passing native library check.
# Latest Catalog Diagnosis

Corrected descriptor/materialize run session97155 completed successfully in12m25s. It predates the whole-shape conversion changes and is not proof of their browser delivery. A new isolated Draw-only catalog bootstrap is live in session67152, using generated/nx-current and the ticket's generated/draw-hub data root; log `draw-hub-bootstrap-current.txt`. Native suite session48654 has terminated with the lasso fixture failure described in the path-editing notes; the fresh complete suite is session31528 with --no-fail-fast.

Fresh graph generation session53102 succeeded. `draw-current-targets.json` confirms the central descriptor component command and component-dev prerequisite; use generated/nx-current for subsequent commands. The prior graph is only useful for historical diagnostics. Session55411 terminated after12m25s: the stale describe route failed and component-dev captured the now-repaired arc helper path error. The corrected graph and sources are now building in session97155 (`draw-describe-convert.txt`). After a successful descriptor/materialization run, retry the isolated Draw catalog bootstrap.

The targeted describe attempt used a stale ticket-local Nx graph: it invoked the removed crate-local `script.ts describe` route. Current inference in the repository's library `🟨️.mjs` lines901–906 already routes describe through the central descriptor component command and depends on component-dev. No Draw script-router change is needed. Session53102 is generating a fresh graph in generated/nx-current; inspect `draw-current-targets.json` and its error log, then use that graph for future targets. Session55411 may still be finishing its materialize-dev sibling after the failed describe; do not duplicate that work while it remains live.

The shared local hub startup terminated after116m2s: its catalog rejected `s.stdio.csv` because that open target has no verified codec row. A newly opened browser tab at127.0.0.1:6064 still showed Loading plugins and Hub connection signed out. This is a terminal catalog failure, not an ongoing compilation wait.

An isolated Draw-only bootstrap was attempted through `bun nx run os-hub:trusted-catalog-bootstrap --packages draw`, with OS_HUB_DATA under this ticket's generated/draw-hub directory. It failed preflight in13.7s because Draw's committed descriptor does not identify its current component-dev deliverable. No candidate hub was published. The targeted Draw describe/materialize-dev run is session55411; after it completes, retry the same supported bootstrap, then start/join the isolated hub using the existing local-hub command and an unused loopback port. Preserve all shared hub processes and data. Logs are `draw-hub-bootstrap.txt` and `draw-describe-drag.txt` under generated.

## Full Native Regression Run

The no-fail-fast run in `tests-shape-all.txt` completed: 327 tests executed, 315 passed and 12 failed. The failures included numeric JSON fixture comparisons (integer versus float representation), fixture setup that emitted a host reset effect without actually loading the document, a paged inspector test that expected unmaterialized rows, an empty-path stroke target, outdated empty-path/degenerate-arc assertions, and two production defects: close-only paths acquiring phantom bounds and Boolean child row IDs failing to resolve.

Corrections use typed fixture comparisons, retained document-envelope loading with an exact snapshot assertion, explicit node page requests, and a drawable stroke target. Production bounds ignore leading Close commands. Boolean child IDs now encode the parent byte length, preserving child IDs containing dots, reserved-looking fragments and Unicode. Regression assertions cover those identifiers. A fresh full no-fail-fast run is active as session 2385 with output `tests-regressions-all.txt`; no passing result is claimed yet.

The isolated Draw catalog bootstrap (67152) terminated unsuccessfully after 5m17s. It reached the hub build and failed on 12 Rust errors, including Send lifetime constraints in artifact authority. This is separate from Draw compilation and prevents the isolated browser runtime from starting. The prior descriptor/component rebuild succeeded, but browser editing remains unverified.

### Final Native Result

Session 6077 completed successfully: **327 tests run, 327 passed, zero skipped**, with assertions taking 2.541 seconds and the Nx task taking 10 seconds. Evidence: `🗑️generated/tests-regressions-final.txt`. This supersedes the prior pending/full-suite-failure statuses. `git diff --check -- ✏️s/🔌️plugins/🖍️draw` also passed. Nx labeled the task flaky after the earlier execution-budget timeout; no claim is made that the timeout itself has been diagnosed.

The user goal and ticket remain open. Remaining work includes multi-layer/group transforms, resize/rotate/snapping, direct canvas node handles, joining separate path layers, fuller text/image editing and a successful browser workflow audit. The isolated hub bootstrap remains blocked by its Rust build errors; no browser editor pass is claimed.

## Selected Movement Validation Status

The new multi-layer/group drag implementation supersedes the earlier single-leaf implementation. TypeScript tests pass (23 tests, 938 assertions, 42 Ajv field cases, 28 publication routes). Native check passed for the first implementation revision, but the final native test run did not compile because a newly referenced shared pixels compositing Rust module is absent. This is separate from the earlier hub build failure. It is not evidence of a passing new native workflow. See `↔️direct-manipulation.md` for the exact pending tests and implementation scope. No native test process remains live from this checkpoint.

### Multi-Selection Native Suite Passed

`tests-selection-move-history.txt` completed successfully: **331 tests executed, 331 passed, zero skipped**. Assertions took 3.611 seconds; the complete Nx task took 2m19s. This verifies selected-group plus selected-child deduplication, unchanged preview snapshots, group/leaf world-space preview, cancellation, preserved framework selection, one publication for multiple transform mutations, and exact undo/redo. The native mixed-parent and invalid-target tests passed as well. This supersedes the previous pending status.

Browser delivery is still pending. A fresh Draw-only `describe materialize-dev` run is rebuilding the matching component and descriptor (`draw-describe-selection-move.txt`). The hub endpoint compile mismatch was fixed by declaring `Json<HubObservabilityV1>` as `admin_observability`'s response type, matching the model it returns; no response data or authentication behavior changed. The hub build is live as session 57930 (`hub-build-observability.txt`); its result has not been claimed. Changed dependency file: `🌎️hub/🏗️bootstrap/🦀️.rs`.

## Browser Editing Audit Started

The existing preview process 94715 is live on `http://127.0.0.1:6064/`. A fresh in-app browser tab loaded the real Draw editor while the status read `Remote: detached` and `Hub connection: signed out`; the former Loading Plugins state is no longer the current result. Authentication was not bypassed. The local preview supports editing independently of the unfinished hub build.

Observed through browser DOM snapshots and a screenshot: the Canvas shell, Demo document, catalogue and inspection panels render. Opening Artifact exposes the actual layer tree (Semio Emblem, Orange Wedge, Red Frame, Teal Bars). Clicking Add Rectangle adds `Rectangle normal Drag to window`. Clicking that new row selects it and exposes the inspector with name, visibility/locking, appearance and numeric transform controls.

Concrete usability gaps: the new rectangle has both fill and stroke disabled; it is not selected automatically on creation; the initial demo appears clipped near the upper-left edge rather than fitted into the available canvas. Inspector textboxes/spinbuttons appeared without individual accessible names in the DOM snapshot, although their surrounding tree rows are labeled. These require fixes/validation rather than being accepted as complete. Catalogue row click merely highlights an item; the separate Artifact Add Rectangle action is the creation operation verified here.

A subsequent rename attempt could not find its textbox because the document had returned to the Demo contents (the new rectangle was gone) while component files were rebuilding. No rename or fill edit pass is claimed. The timing is consistent with development hot reload; it does not prove persistence behavior. Further browser action checks should resume after the rebuild completes.

The matching Draw component/descriptor task remains live as **21529**, output `draw-describe-selection-move.txt`; component-dev succeeded, describe/materialize have not yet reported final completion. The old serve process 94715 remains live. The browser binding is valid; tabs may be cleared at a turn boundary, so use tabs.list/new without reinitializing the browser.

The hub build **57930 terminated unsuccessfully** in 3m27s. The observability response type error is absent after the one-line fix, but eleven Send/FnOnce lifetime errors remain in authority/check-in/catalog verification. Evidence: `hub-build-observability.txt`. Do not relaunch that same unchanged build merely to wait. The Draw native suite is complete and passing at 331/331, not running.

### Creation Verification Complete in Native Runtime

The post-fix full suite passed: **335 tests run, 335 passed, zero skipped**, assertion duration 2.028s, Nx task 17.3s (`tests-creation-final.txt`). This includes visible/default-preserving creation paint, immediate selection and distinct repeated pen/polygon drafts. It supersedes the pending native identity-fix status. TypeScript remains passing at 24 tests/948 assertions. `git diff --check` passes.

A fresh matching Draw-only component/descriptor build is now running (`draw-describe-creation.txt`) for browser verification. The previous matching build is terminal-successful and must not be polled or restarted as if it were still active. Browser creation controls should be retested only after the new build completes, because hot reload reset the demo during the prior browser edit attempt. No browser paint/automatic-selection pass is claimed yet.

The new live matching build handle is **6843**. Revalidate this exact session on continuation; do not launch a duplicate. Preview server 94715 was previously confirmed live. Hub build 57930 and native tests 38978 are terminal; the hub still has eleven lifetime errors, while native tests passed.


## Inspector Event Dispatch Verification

The matching creation component build (session 6843, `draw-describe-creation.txt`) completed successfully in 1m59s. A fresh browser preview on port 6064 created a rectangle through Artifact → Add Rectangle. Without selecting another row, Inspection showed that rectangle, teal fill enabled, dark stroke enabled, width 2, round caps and joins. This confirms creation paint and immediate selection in the browser UI.

Editing Name to “Browser Rectangle”, Position X to 120 and Position Y to 80 changed the inputs' local drafts, but the layer tree remained “Rectangle”. Source inspection found every blur-committing Draw input bound `Trigger::Change`; the retained interpreter dispatches `Trigger::Commit` on blur/Enter. Therefore those earlier input values were not proof of document mutation. The added native projected-tree regression failed at the Name field with the exact mismatch (`tests-inspector-events-red.txt`: one failed, 335 skipped). The inspector now binds Commit for ordinary fields, path coordinate fields, and fill/gradient fields; selects and toggles retain Change.

Browser DOM also confirmed that native accessibility labels were dropped by the shared InputView and SelectView. The renderer also omitted the authored disabled state on these controls. A language-neutral fixture and mounted React regression now cover text, number, long text, color and select controls in EN/DE, enabled and disabled; production changes await the red run. The initial fundamental test invocation selected no tests; corrected invocation uses the existing long test level. This is not a passing test result.

Current pending work: full native inspector regression run 42468, React red regression run 95267, and matching Draw component build 8838. Canvas framing remains unchanged and still requires a real fit-to-content implementation.


The corrected React oracle (`dom-accessibility-api` computes the actual mounted control name) failed all 20 cases before the renderer fix and passed **20/20** afterward (`tests-editable-controls-red-oracle.txt`, `tests-editable-controls.txt`; 13.94s Vitest total, 514ms assertions). InputView now preserves authored labels and disabled state for ordinary and multiline inputs; SelectView preserves labels and disabled state. The earlier run using the owned render helper's nonexistent `getByLabelText` was a harness error, not a product regression.

The first full post-binding native suite compiled and started 336 tests, then the runner killed it at its 15-second fundamental budget. It reported no assertion result before termination; this is **not** a pass. A targeted inspector rerun is pending (59534), and the component build remains pending (8838). No native or browser success is inferred from the passing React regression.


## Completed Inspector Browser Verification

The matching build 8838 completed successfully in 5m13s (`draw-describe-inspector-events.txt`). The native binding regression 59534 passed one test, 335 skipped, in 126ms of assertions (`tests-inspector-events-targeted.txt`). The second full native run 68681 again exceeded the unchanged 15-second runner budget after starting 336 tests (`tests-inspector-events-final.txt`), with no assertion failure or successful full-suite result reported. It also reported a package-cache lock wait. Both full runs are terminal; do not poll or restart them blindly.

On the rebuilt browser at port 6064:

- Add Rectangle immediately showed named inputs in Inspection, with teal fill and dark 2-unit stroke enabled.
- Changing Name to “Verified Rectangle” and blurring updated the actual Artifact layer row.
- Position X=420 and Position Y=320 were committed by blur. Selecting Orange Wedge changed the inspector to that layer, and selecting Verified Rectangle again restored the name and both numeric values. This verifies document updates, not merely input drafts.
- Locking the layer showed disabled Name, Position X/Y and Blend Mode controls in the accessibility snapshot. Unlocking restored editable controls.
- History → Undo reverted the last unlock: the Locked toggle was pressed and property fields were disabled. History → Redo restored the unlocked, editable state while preserving name and coordinates.

The earlier first attempt was interrupted by a preview reset back to Demo; stylesheet HMR entries were present in the preview log, but the precise reset cause was not isolated. The repeat journey above completed. No browser pass is claimed yet for multi-selection dragging, path-node edits, gradient edits, camera fitting, or property Enter/Escape handling. History showed seven pending operations for the journey; unchanged-value blur potentially creates an extra history entry and needs a dedicated no-op regression.

Files changed in this checkpoint: Draw inspector implementation, selection fixture and native tests; shared renderer Interpreter implementation; renderer editable-controls language-neutral fixture and React test; existing renderer Vitest include list; this runtime verification note and `📷️canvas-framing.md`. The React oracle uses the existing third-party `dom-accessibility-api` only in tests. No runtime dependency, script, or executable command was added. Relevant `git diff --check` passed.


## Initial Framing Checkpoint

The rebuilt component/descriptor sequence passed, but the live Demo still opens too small and off-center. The initial empty-document fit may be consumed before Demo loading. See `📷️canvas-framing.md` for exact passing tests, runtime evidence, pending native handle and next investigation. Initial framing must not be marked complete.
