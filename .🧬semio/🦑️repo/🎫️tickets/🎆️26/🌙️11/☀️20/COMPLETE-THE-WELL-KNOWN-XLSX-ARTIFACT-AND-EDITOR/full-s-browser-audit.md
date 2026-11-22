# Full S browser acceptance audit

The previous full S attempts did not reach a listening server. The `publication-v3-acceptance-on-ready.log` and `publication-v3-io-matrix-on-ready.log` watchers exited with ESRCH while waiting for PID 326. The sealed retry host exited 130. Their output is not a browser acceptance result. Older status descriptions in verification.md reporting active hosts are historical.

The exact `@semio-tech/plugin-registry:session-stdio-xlsx` managed build did complete, publishing `playground-session:stdio-xlsx` and two generated catalogs (three outputs). This is the standalone XLSX session, not the full S session; it cannot establish full S activation. Nor does a successful offline acceptance contract or an oracle-only Chromium workbook read establish the natural-file journey. Live source churn means an old output or receipt must be admitted by the managed graph for its current inputs before reuse. Do not manually substitute the standalone catalog, skip task dependencies, or infer freshness from file presence.

The supported lowest-assumption route is to retain the managed cold target, which prepares and activates the full S session and dependencies and can reuse genuinely matching managed results. Run from the checkout root in a live repository session:

```sh
S_OS_PORT=6070 S_LOCAL_ONLY=1 SEMIO_VITE_HMR=0 repo run @semio-tech/framework-os-dev:dev-s-react-prototype
```

The ticket runner implements that command and records the managed child PID under `🗑️generated/host.pid`:

```sh
repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts host
```

After HTTP readiness, run these separate gates with the same live host PID as the second argument. Their own status and logs are in the ticket generated directory:

```sh
repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts acceptance-on-ready HOST_PID
repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts acceptance-de-on-ready HOST_PID
repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/run-browser-io.ts io-matrix-on-ready HOST_PID
```

The first two invoke the actual `natural-file` XLSX acceptance in English and German; the last selects `stdio-office/xlsx` in the Hub I/O matrix. A live zero exit and the generated evidence for each gate are required. The separate standalone pane on port 6272 is not a substitute for these full-shell gates. `SEMIO_REPO_SEAL=live` was used on the prior retry to select current implementation instead of the older explicitly promoted seal; set it for this run if the current implementation is still not promoted. Full selected graph admission and Office browser component preparation remain prerequisites; their cost and cached reuse depend on the current exact graph, component receipts and source fingerprints. The earlier graph scope lookup found no reusable exact XLSX snapshot, but the later standalone session receipt is narrower and cannot prove the full S closure. No new build or browser test was run during this audit.

## Current readiness and journey coverage

At this audit both ports 6070 and 6272 refused connections. Browser prototype sessions contain `layout` and `stdio-xlsx` but no full S session. Neither React activation directory exists. The browser modules directory contains only shard and vendor; no Office module was present. The standalone session contains its artifact manifest and all three declared files. These facts do not establish current-source freshness. The full S cold target remains the supported route once the already-running component producer completes; an excluded-dependencies warm serve is not justified by these assets.

The natural-file acceptance script checks the actual picker import, editor mutation, exported undo and redo fingerprints, durable kept UUID and edited download, close and reopen, shell reload and reopening of that same UUID, plus invalid input refusal. It requires all nine step verdicts and no faults. English and German are separate invocations. The Hub XLSX I/O matrix provides a complementary original-file reopen and export. The standalone pane checks creation, A1 edit, undo, redo and valid downloads, but does not cover full-shell durable reload. No separate XLSX-specific browser history shortcut was found in the inspected test trees.

## Current shared graph wait

At the 01:05–01:07 fleet snapshot the graph leader was PID 45226, with 305,221 distinct observations and 568 completed owner checks. The host log last changed that owner count at graph-slice second 1,946; it was still 568 at second 2,482, over eight minutes later. The parity retry explicitly waits for the leader on the same fact store. Fleet and lease diagnostics reported no graph slot waiter, no lease waiter or deadlock, and no capacity waiter. The current hold is therefore inside the leader load, rather than an observed external lease or capacity gate.

The leader remained alive with no child processes and approximately 11 seconds of cumulative user plus system CPU at inspection; this and the static counter do not identify its current asynchronous wait or prove deadlock. Earlier ticket evidence found expensive discovery and memo checks in different workers; it cannot identify this worker’s specific operation. `repo doctor --only=graph` reports no published generation and recommends `repo show projects` to publish one, which would be a separate full graph load, not an already-authoritative light recovery. Preserve the existing load and observe its result; there is no evidence-backed non-destructive bypass of the same full authority. No peer process was interrupted.

### Current host refusal

The full S host ended with status 1 after the graph worker refused three starts because it could not resolve `@semio-tech/puzzle-wasm`. The repository workspace symlink exists and points to the authored package. The declared generated entrypoint `pkg/semio_puzzle.js` and its containing output directory are absent. The English acceptance watcher observed the host exit and produced no journey verdict.

### Cold host retry

The graph now declares both generated Puzzle package names and their shared declaration output. A production graph regression passed against the source inventory with generated Puzzle artifacts absent. The renewed managed S host continues checking the live inventory with no repeat of the earlier Puzzle resolution error. Its English and German natural-file tests and Hub I/O matrix all watch the same host. The repository-selected browser cache contains executable Chromium and the headless shell, and Playwright resolves from the acceptance test. The browser checks have yet to receive a successful HTTP response from the managed host.

### Graph worker progress behind the owner total

The repository's live slice diagnostic matched worker PID 62627 to the full S plan at 303,899 observed sources and 580 completed owner checks. It showed the active owner `@semio-tech/framework-os-dev` move from `source-closure` to `compile-closure`. Raw interned rows rose from 154,527 to 164,517 in 29.6 seconds while the completed-owner counter stayed fixed. The owner counter advances only after the entire check completes. The report is shared by both workers, so a sample must match that PID. The live lease view reported no capacity or lease wait for the active worker.

### Captured Office Browser Component

The captured Office WASIp2 guest passed the production prototype transpiler under a repository capacity grant. Its first Chromium load refused a Preview2 import because the shim directory was below the guest module and the generated URL had no relative prefix. A neutral three-case fixture reproduced that refusal with the independent Bun module resolver for same-directory and nested shims. The existing sibling layout succeeded. The materializer now emits relative URLs in all three layouts. The complete native transpiler suite passed five cases with 13 assertions, including the byte-for-byte jco tree comparisons. The fresh production transpile and Chromium JSPI replay both exited zero. Chromium exposed all required production actor exports and returned the byte-identical semantic Pack from the native Office emitter, SHA-256 `29f7fd8f689bcf37cd9a5c030f6c786ef23589e9e21891b4ece0010ecb127e55`. This supplemental component check did not exercise the shell or editor UI.

### Shared Browser Product and Variant Activation

The issued standalone session lists exactly the Office and Stdio modules and selects `s.stdio.xlsx@ecma-376/*#editor` for React on port 6272. The session's three declared files and its ownership manifest exist. The prototype browser module store is shared across variants, and each variant has a separate React activation. At this inspection, the shared store held an authenticated Layout module, the shard and Preview2 support. It lacked the Office and Stdio product directories. Neither React activation existed. The standalone and full S managed requests continue independently.

The compiler pipeline publishes the browser product after admitting the linked model and its actual source provenance under the compilation and product leases. The captured component's Chromium result supplies independent browser execution evidence. It does not supply the managed product receipt or either React activation.

A second PID-matched sample measured 100,258 to 102,015 interned rows in 39.35 seconds while the full S graph worker was resolving the Trinity Jack compile closure. The completed-owner count held at 4,409 during that work and later reached 4,503. The selected graph remains active.

### Cold Host Recovery After a Replicated Graph Module Arrived

Both managed browser hosts and the Office browser publisher exited with reference `15e12f66` when the live graph loader first referenced its owner-checkpoint module. The importer changed at 18:00:41.678 UTC and the module arrived at 18:01:48.378 UTC. The worker initially requested a fresh invocation, then the immediate retry still could not open the module. Both host readiness watchers observed the old hosts exit and produced no browser verdict. The checkpoint test suite now passes all seven cases with 52 assertions, including reuse of exact observations, interrupted writers, scope validation, and age and quota limits. Both managed browser hosts and the dedicated Office publisher restarted with the module present. The four browser journeys are watching the new host processes. Both published Office descriptor files still match the verified native pair byte for byte.
