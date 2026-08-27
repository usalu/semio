# Demonstrator Current-Source Browser Proof

## 10:25 Current Status (Supersedes Earlier Process Notes)

The earlier preview and unfinished compiler sessions disappeared before 10:20. No owned compiler is intentionally paused. Aggregate build 23 and frontend build 1 completed; Procedural build 25 and native Flow build 2 did not reach a completion footer or test results and are not passing evidence.

The fresh aggregate's Aggregator route still showed an infinite loading canvas and a WebAssembly panic. The ticket-local Node JSPI probe captured the guest stderr: Puzzle3D and CAD both register retained `setLocale` factories without declaring the matching manifest action, so the authority join reports `generated_migrated=false`. Both also omit `setTerminology`. Factory/schema/owner identities match; the authority guard must remain strict. Sourcing, Process and GIS accepted `instance-open` in this probe, but no content or interaction claim follows from that alone. Logs: `🧪️aggregator-component-boot-red.log`, `🧪️five-app-component-boot-red.log`.

The terminal-boot JSON regression reproduced the masked-error bug: `failed boot` incorrectly returns a loading canvas (`🧪️terminal-boot-canvas-red-2.log`). The shell now lets terminal errors and crashed/quarantined recovery content through the loading boundary; the full renderer suite is running.

The full renderer gate passed **473/473** under the long profile (`🧪️terminal-boot-canvas-green-long.log`). The initial default-profile run hit the overall 15-second process budget; it is not passing evidence. Puzzle3D and CAD now declare both configuration actions, and their fixture-backed tests require exactly one migrated declaration for each retained tool. Aggregate build 26 is running serially with two build jobs.

Browser verification is currently paused: the initial preview navigation raced server startup and produced a connection-refused page. The next reload was rejected by the browser URL policy; a read-only URL check identified the browser-generated `data:text/html` error page as the current URL. No workaround, alternate browser, or raw browser-control mechanism was used. Preview session 56371 is now listening on 6036. Component-level diagnostics can continue independently; they do not replace the outstanding visual proof.

### Component Surface And Close Checks

Using build 23 through the real generated Node JSPI bridge, Sourcing published its four declared window bodies and closed in 49 bounded turns (`🧪️sourcing-component-surfaces-close-1.log`). GIS published its main map and three panel bodies and closed in 103 turns (`🧪️gis-component-surfaces-close-1.log`). Process published a nonempty mesh, a 19-node catalogue, inspection, and its Timber Beam document, but **did not publish Workshop** (`🧪️process-component-surfaces-1.log`). These are component-boundary checks, not visual browser proof or full interactions.

The new language-neutral 53-node catalogue test reproduced an exact rejection at node 33, despite only 478 items and 918,272 bytes used: the old node cap was 32, below the panel shape. A second test reproduced silently discarded producer faults. Both failures are recorded in `🧪️catalogue-surface-red-2.log`; red 1 was only a test compile error. The node authority now uses one shared document/reconciler limit of 128, while retaining the 8 MiB surface, 32 KiB page, and 32 MiB aggregate byte bounds. Producer/reconcile failures are reported once before their exact owners retire; render errors are forwarded to the shell. A cap-overflow regression additionally checks bounded refusal and cleanup. Green tests are running. Build 26 started before these UI changes, so it is only a candidate for the CAD/Puzzle manifest fix, not the final publication.

Storage fell below 200 MiB during the previous overlapping builds. After confirming no process used the exact targets, three superseded, reproducible `debug/incremental` caches were removed from the aggregate, GIS-contract and Puzzle-runtime ticket targets. Source, dependency artifacts, and every log were preserved. At 10:22, 42 GiB was free; this entire change in free space is not attributed solely to the cache removal. Future owned heavy compilations run serially.

## Required Publication

The final browser pass requires newly successful aggregate and standalone Procedural builds, followed by a fresh frontend build. Builds 21/22 both failed against the concurrently changing Flow/neural ownership boundary and published no components. Current-source check 21 reached Flow and reported 14 outdated mutable-map calls. Check 22 passed Flow and exposed four Procedural displacement-owner mismatches; these now retain the returned shared layout owner directly. Check 23 passed the complete demonstrator crate graph in 7m23s (`🧪️aggregate-current-check-23.log`). Aggregate build 23 compiled successfully in 24m50s and is executing descriptor extraction/materialization. Procedural build 24 resolved an older dependency graph while another writer added serde_json-based macro authority checks, then failed with 19 missing-serde_json diagnostics. The dependency was already present when inspected, so no duplicate fix was applied; fresh build 25 is session 88676 (`🧪️procedural-build-25.log`). Aggregate build 18 predates the semantic-census and document allowance corrections and is not a final verification candidate.

Production preview: `http://127.0.0.1:6036/`. The previous stale proof tab has been closed.

### 08:00 Publication Update

Aggregate build 23 completed materialization successfully; the published core is 139,649,235 bytes, timestamp 2026-08-27 07:56:56. Its best-effort native descriptor extraction hit the interpreter epoch deadline, so descriptor freshness is **not** claimed. The frontend rebuild is session 11580 (`🧪️frontend-current-build-1.log`) and still uses the prior standalone Procedural artifact until build 25 publishes. Early five-app browser checks may diagnose this fresh aggregate, but they do not count as final six-app proof.

The three queued native checks did not execute tests: the layout run failed on an un-awaited async Brep registry; the subsequent two runs used the older DSL dependency resolution and failed on serde_json. The Brep manifest now awaits its registry before wrapping the owned result. Fresh native VCS checks are session 33309 (`🧪️flow-vcs-native-2.log`).

## Active Process Sequencing

The host has 32 GiB RAM. Native Flow test compiler **PID 50108** (parent sccache 50105, Cargo 2015, session 17513) was temporarily paused to avoid overlapping heavy stdio code generation. It has now been **resumed with SIGCONT** after aggregate build 23 finished stdio and advanced to Flow/CAD/Puzzle/Sourcing. No owned compiler remains intentionally paused. The queued Generator native test is session 31610, and the shared-snapshot regression is session 35423. Aggregate build 23 is session 54288; Procedural build 24 is session 24163. The unrelated stdio compiler PID 12850 belonged to another task and was not controlled.

Storage audit during compilation: approximately 19 GiB free; active aggregate cache 29 GiB, active native Flow cache 2.6 GiB, prior GIS cache 2 GiB, reusable tracker/Puzzle cache 1.8 GiB. The clean skill was inspected but its whole-workspace command was not run because it would delete active ticket artifacts and evidence. No caches or logs were removed.

## Ticket Infrastructure Probe

No repo MCP tools are exposed in this session. The ticket-local `📜️script.ts` additionally tried the configured local stdio server using the installed MCP SDK. Its connection closed before resources could be read (`🧪️repo-mcp-probe-2.log`); no ticket/goal mutations were made. The first inline probe failed because Nx's shell reconstruction removed its JavaScript quotes, so the file-backed probe is Consumer/Server evidence, while `🧪️repo-mcp-probe.log` is only a command-quoting failure. Preserve the probe script and logs with the ticket.

## Route Checks

| Route | App | Authored Example | Window Content | Panels | Interaction | Console |
| --- | --- | --- | --- | --- | --- | --- |
| `generator` | Procedural3D | `hexagonal-mushroom-column` | Pending | Pending | Pending | Pending |
| `koordinator` | CAD | `hexagonal-cut-concrete-forest-left` | Pending | Pending | Pending | Pending |
| `aggregator` | Puzzle3D | `concrete-forest` | Pending | Pending | Pending | Pending |
| `aussuchen` | Sourcing | `demo-stock` | Pending | Pending | Pending | Pending |
| `bearbeiten` | Process3D | `timber-beam-joinery` | Pending | Pending | Pending | Pending |
| `verfolgen` | GIS2D | `reuse-map` | Pending | Pending | Pending | Pending |

Each route needs observed non-placeholder window content, its declared panel bodies, at least one state-changing interaction, no persistent loading state, and a fresh console inspection. Switching routes must also remain functional after the prior actor closes. A canvas count without visible content is insufficient.

## Current Automated Evidence

- Current Wasm source graph: `cargo check -p semio-s-plugin-demonstrator --lib --target wasm32-wasip2` passed through the scoped Nx executor, `🧪️aggregate-current-check-23.log`.
- Browser renderer after the incomplete terminal-surface correction: 469/469 passed, `🧪️renderer-terminal-surface-final.log`. Focused PluginRuntime: 40/40, `🧪️idle-surface-green.log`.
- UI runtime after semantic-census and allowance fixes: 82/82 passed, `🧪️ui-runtime-registered-gate-3.log`.
- Mounted tracker after the close fix and final formatting: 20/20 passed, `🧪️mounted-tracker-full-nextest-final.log`.
- Generator catalogue identities: 1/1 passed after reproducing the four export entries colliding on one key, `🧪️catalogue-identity-red.log` and `🧪️catalogue-identity-green.log`.
- Fresh launcher/build-pipeline gates: demonstrator 5/5 and OS dev 49/49, `🧪️demonstrator-launch-gate-final.log` and `🧪️os-dev-gate-final.log`.
- The tracker suite covers six surface shapes, four nested numeric settings controls, a 19-node/14-interactive-row document, exact action/value wire data, admission refusal, and terminal cleanup without stale publication.

## Unverified Boundary

Flow layout edits now use an incremental ordered-map cursor, with immutable previous roots retained for undo/redo and cancellation. Document replacement transfers the layout root without traversing it. Fixture, action, and session teardown transfer exact owners into the shared typed retirement frontier. `📍️ordered-layout.json` supplies independent JSON-map expected results and cancellation-at-each-boundary inputs; native tests are running in `🧪️flow-ordered-layout-test-1.log`. Flow itself passed check 22, but these changes have not yet passed the native tests or browser proof. Procedural 2D/3D replay teardown also transfers displaced widgets, synapses, and strings into the same typed frontier instead of dropping nested ordered owners synchronously.

The host currently stops refresh continuations when all requested surfaces already have retained trees. Verify that a real interaction updates visible content, rather than returning an older retained tree while incremental reconciliation remains pending. No speculative refresh-policy change has been applied.

The Generator native geometry test is also queued (`🧪️generator-mushroom-native-1.log`). Code inspection found older eager FlowHost paths still assigning/releasing neural dictionaries and temporary trees without the newly explicit retirement API (examples: `apply_eval_outputs_json`, `set_neuron_params`, `evaluate_step`). Treat these as unverified runtime candidates, not confirmed observed failures; use the native test/backtrace and fresh browser before widening changes. The existing `neural::ColdOwner` is explicitly batch-only and must not substitute for retained cursor work.

A new shared-snapshot regression is queued in session 35423 (`🧪️flow-shared-snapshot-red.log`), reusing the ordered-layout fixture. `FlowSnapshotRetirement` currently retains its `Arc` when `try_unwrap` fails, so two retirement readers can hold each other indefinitely. The regression is added but the production correction is deliberately not applied yet; obtain the failing test, then replace the shared-reference handoff with an atomic consuming final-owner operation and rerun.
