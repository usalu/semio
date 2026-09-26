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
