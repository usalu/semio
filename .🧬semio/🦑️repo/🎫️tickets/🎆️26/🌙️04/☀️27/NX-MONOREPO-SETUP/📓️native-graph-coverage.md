# Native Graph Audit Coverage

The 2026-09-09 native graph contains 698 projects: 380 emoji project configurations, 69 Cargo-only projects, 248 inferred test cases, and one npm-only flow binding. The previous audit calls only the core repository plugin, so its 449-project inventory omits the separate test plugin and npm inference. Its two output findings therefore describe a subset, not the full monorepo.

The audit must consume Nx’s fully resolved graph, including target defaults, package inference, and every configured plugin. Preserve target configuration provenance so findings point to existing sources. Validate against native Nx output with a language-neutral multi-provider fixture and the actual graph.

The browser default cannot be replaced with a hardcoded source constant: registry generation selects the sole plugin manifest declaring host metadata and its playground variant. The existing registry dependency remains justified.

Reference: [Nx graph API](https://nx.dev/docs/reference/devkit/createProjectGraphAsync). Implementation and validation are in progress.

A native `nx exec` probe was rejected by a project-graph cycle between the OS kernel and value derive crate. The reverse edge is a Cargo dev dependency; Cargo permits this pattern. A targeted `repo:audit` probe is being used while the graph inference issue is recorded for repair. No dependency was removed or ignored.

## Complete Inventory Regression

The native regression first failed because the prior inventory omitted the npm binding and inferred test cases. The revised inventory consumes the graph and configuration source maps already constructed by its outer Nx invocation, preserving resolved inputs, outputs, dependencies, configurations and executors. It does not launch another project scan or scheduler. The focused native comparison passed across 698 projects, including every target and existing source location. Log: `🗑️generated/native-inventory-green.log`.

The expanded audit also exposes ten inferred `@nx/js:release-publish` targets that bypass the repository script interface. Their release contracts remain to be reconciled; the old all-public-command assertion is expected to identify this newly visible debt. No publishing operation was invoked.

The focused inventory took 2m32s. Its artifact registry still compared every output pair and repeatedly normalized long Unicode paths. An indexed overlap implementation and a 10,000-owner normalization-budget regression are being developed with native Python as the collision oracle.

## Indexed Artifact Ownership Qualified

The 10,000-owner regression first rejected the pairwise implementation after it exceeded one million Unicode normalizations. The prefix index then passed with 280,000 normalizations (28 per owner, including mutable-store validation). Python's independent pairwise oracle matched every reported parent/child, duplicate-owner-path, sibling, case-folded and Unicode-normalized collision, preserving finding order. Native focused execution passed in 1.3s (473ms target).

The actual expanded `repo:audit` then passed in 15.2s (14.2s target): **698 projects, 6,575 commands, 7,588 artifact/store entries and 12 findings**. The previous full-coverage probe took 2m34s (2m32s target); these runs were on a shared loaded machine, not a controlled benchmark. Ten findings concern inferred publishing executors, and two remain the WGPU/OS-dev missing output contracts. Machine inventories were regenerated in `🗑️generated/nx`; the full suite is still running.

## Full Suite Result and Probe Scope Correction

The full suite failed after 4m09s at its existing all-public-commands assertion. It reported the ten newly visible inferred publishing executors; the full-provider comparison, 10,000-owner/Python overlap test, artifact accounting, native dependency/bootstrap, cancellation and prior browser contracts had passed before that assertion. Later lifecycle/compiler/discovery assertions were not reached in this run. Log: `🗑️generated/native-inventory-complete-suite.log`. The earlier 1m52s receipt/private-storage suite predates these changes and is not a pass for the current tree.

Further inspection of Nx 23.2.0's `exec/exec.js` and `commands-runner/create-command-graph.js` corrected the initial cycle diagnosis: an unqualified `nx exec` selects **every project**, then recursively executes project dependencies. The Cargo dev-dependency cycle is legitimate input to the project graph. It must not be removed merely to make a broad diagnostic invocation succeed. Targeted `repo:audit` executions worked; future one-off probes should use a selected target or explicit project selection with dependency execution disabled. No graph edge was modified.

Native Vite's bundled-config loader also wrote transient fixture modules beneath the nearest ancestor `node_modules/.vite-temp`, outside the ticket. It deletes each module in a `finally` block, so the log alone does not establish a persistent leak. The browser fixture nevertheless needs its own empty `node_modules` directory so temporary config compilation remains inside its private ticket workspace. This fixture isolation fix is next.

## Native Executor Classification Correction

The ten publishing findings were **false positives from the new audit rule**, not ten confirmed Nx bypasses. The supplied plan explicitly permits native tools and plugin executors inside Nx. Source maps attribute these targets to `nx/core/package-json`, and `@nx/js:release-publish` is executed by the native Nx task graph. The audit must still enforce repository-authored script commands, but it must not treat every native executor as an external scheduler. The blanket executor rejection was removed; executor/configuration/provenance fields remain in the complete inventory.

The language-neutral coverage fixture now identifies side-effecting native executors. Its test constructs their actual task objects with native `createTaskGraph` and checks native `isCacheableTask` returns false. Nx 23.2.0 defaults an unspecified target cache flag to false in `create-task-graph.js`; no new wrapper or publishing implementation is needed. No publishing operation is run by this check. The prior full-suite failure is retained as evidence of the overbroad audit rule, not presented as a pass. Corrected full-suite validation is pending.

## Private Vite Fixture Storage Qualified

The new native regression failed on the missing private bundled-config cache. Creating a private package manifest and empty private `node_modules` directory then passed: Vite's actual bundled-config stack points inside the ticket, native `resolveConfig` chooses the private optimizer directory, and real HTTP/HMR, failure preservation, cancellation and port release still pass. Focused run: 5.0s (4.0s target). Logs: `🗑️generated/native-vite-storage-{red,green}.log`. The fixture removes only its own temporary workspace; no shared dependency store is changed.

A read-only native hash probe is now checking an inferred Rust test's resolved file inputs against its local Cargo dependency manifests. The testing plugin's declared input patterns appear narrower than the native subject's dependency closure; actual hash coverage is pending before treating this as a confirmed cache defect.

## Corrected Full Validation Complete

The complete native `repo:test` suite passed in **2m27s**, two tasks, including all-provider inventory, native publishing cacheability, indexed artifact ownership/Python parity, private Vite storage/HTTP/HMR, dependency/bootstrap isolation, editor/playground, lifecycle/compiler, discovery and cancellation contracts. Log: `🗑️generated/native-inventory-complete-suite-native-executors.log`.

The corrected actual audit passed in **10.3s** (9.7s target): **698 projects, 6,576 commands, 7,588 artifact/store entries, two missing-output findings**. One concurrent target addition explains the command-count increase. These counts include every native provider. The two findings do not constitute proof that every cache input and runtime behavior has been qualified; the inferred native-test input investigation and broader renderer/dev/CI/platform/lease work remain open.

The first hash probe could not collect evidence: calling native graph construction inside Bun made Nx launch its plugin worker with Bun, which rejected a top-level-await ESM require. The normal outer Nx executable uses Node and does not have this failure. The diagnostic is being rerun with Node through the same Nx target. No dependency cache was reset or graph edge changed.
