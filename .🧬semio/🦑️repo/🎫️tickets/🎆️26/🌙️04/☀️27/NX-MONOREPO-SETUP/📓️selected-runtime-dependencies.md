# Selected Runtime Dependencies

Timings are Nx-reported task durations and exclude project graph discovery unless explicitly stated.

## User Correction

The user supplied a real Flow compilation log from an unrelated development launch and explicitly requested correct dependencies and compilation limited to the selected app/renderer. Puzzle 3D must not acquire Flow as a runtime prerequisite. React renderer commands must not acquire the WGPU compiler.

## Confirmed Sources

Before this correction, playgroundPreparationTargets injected a baseline of surface, editor and Flow WASM engines into every React preparation, then adds every composition-linked factory and, for a host, every playground engine. The old buildEngineWasm helper repeated the same unconditional engine scheduling and was still called by the collaboration harness. These broad lists are independent of the selected app's actual engine declarations.

React consumes selected plugin components through the common browser plugin host, so plugin component requirements must be distinguished from compiling the renderer itself. Removing a genuinely consumed component would break startup. The correction will eliminate blanket engine edges, derive any required browser engine from the selected declaration, and verify both the installed Nx task closure and actual package imports. Rust compilation dependencies will also be inspected for unwanted Flow edges.

## Implemented Correction

React preparation now uses only the selected playground's authored browser-engine requirements. The baseline surface/editor/Flow list, composition-wide factory engine list, and host-wide engine union were removed. WGPU retains its renderer compiler producer. The obsolete unconditional engine helper and collaboration call were removed; selected Vite optimization metadata follows the same app selection.

The font exporter now has its own Cargo package, `semio-framework-os-font-assets`, and cacheable Nx build. Its only dependency is pinned `typst-assets=0.14.2`. Infinite no longer declares the exporter binary or its font-only render feature/dependency. The existing fonts artifact producer depends explicitly on the standalone tool and hashes its publication code plus prerequisite output, rather than the full Infinite source tree. The launch seed points to the new producer.

## Recorded Validation

- Neutral selected-runtime cases failed before the engine correction: Puzzle 3D React scheduled four unwanted browser engines.
- Selected-runtime and all 122 real playground/profile component dependency cases passed after the correction (344 ms), with independent Cargo TOML parsing and graphlib closures.
- The installed Nx Puzzle 3D React development task graph contained 15 tasks and no Flow, surface, editor, or WGPU renderer compiler task.
- Cargo's actual WASI production/build closures contained 79 crates for Puzzle 3D and 102 for the Puzzle plugin, with no Flow or WGPU/Vello crate in either.
- The new standalone-font dependency test failed before implementation and passed after it. Cargo resolves exactly two crates: the exporter and typst-assets.
- Actual Nx fonts execution passed in 1.4 seconds, with zero of two tasks restored from Nx. The exporter compilation took 0.56 seconds and staged one executable.
- The new font output is identical to the pre-change artifact: 17 fonts, 8,757,072 bytes, SHA-256 `05c4bbb7d07a3ee0c77274d546a3a4c5942366ccbc82eedad18b4885aa21fc5a`. An independent Python struct parser validated every length and the final offset.
- A prior full cache-contract run passed the new installed Nx selected task-closure checks, then failed on an existing Flow output assertion comparing equivalent project-relative versus workspace-relative paths. That assertion now compares resolved physical output paths while retaining exact ownership checks. The full rerun is pending.

Plugin WASI components are still required by the shared browser plugin host. These are distinct from the removed unrelated browser-engine and renderer compilation steps. This report does not claim a successful React application boot or the completion of the wider monorepo caching goal until the remaining runtime and cache checks finish.

## Cache and Launch Qualification

- The repeat fonts run restored both tasks from Nx (2/2 hits, 219 ms).
- After moving only the newly produced font-tool deliverable into ticket-owned storage, Nx restored it without compiling (1/1 hit, 167 ms). Independent SHA-256 and file-mode comparisons matched the original executable and ownership receipt.
- Registry generation succeeded and refreshed the VS Code launch entries.
- Actual Puzzle 3D React preparation scheduled only the selected plugin compiler, with no Flow engine or WGPU renderer compiler. The selected plugin failed in shared window-config Rust code: one String/Option shadowing error at retained window-config line 1300, plus three accesses to private DocumentStoreOwners fields at lines 1381, 1392 and 1393. These are outside the dependency correction. No successful current-source React startup is claimed.

## Additional Checks and Remaining Failures

The 3m20s cache suite rerun passed native command contracts, all 705-project inventory checks, and the new Cargo font closure test. It caught missing VS Code launch entries for the inferred font-tool check/test targets. Both entries were added to the launch seed alongside the exporter build; registry regeneration passed.

The OS-dev quick suite ran 154 tests: 120 passed, 28 skipped, 6 failed. Three failures concern existing config graph hygiene (a forbidden discovery import, 58 modules against a 40-module limit, and esbuild/Bun source-map differences). Three concern existing component URL rewriting and manual catalog build orchestration. The direct engine-removal changes do not touch those implementations. A native esbuild graph probe traced the config's discovery import through the general playground module, and compiler tooling through extension installation/materialization; several conditional test modules also appear in the bundler graph. These need separate dependency-boundary work rather than weakening the limits.

## Font Command Input Boundary

The new exporter initially imported the broad repo tooling barrel, which brought plugin installation and conditional plugin tests into its Nx command-source hash. A neutral forbidden-input fixture failed on that dependency. Both font commands now import only the shared routing and owned-process boundaries. Nx source discovery and independent esbuild metadata both pass the plugin/test exclusion check; Cargo still resolves exactly the same two crates (focused test duration 1.6 seconds).

A subsequent full suite failed in the earlier native-runtime cancellation test: it queried a child/grandchild PID immediately after signalling shutdown. Both recorded PIDs were absent when inspected later, consistent with asynchronous process exit/reaping rather than a lasting orphan. The test now waits up to five seconds for the same exact PIDs to leave the process table, retaining the failure for a surviving process; no production cancellation behavior was weakened. Focused native runtime and the final font command executions are being requalified.

Final narrow-command runtime checks passed: font publication ran in 1.2 seconds with 0/2 Nx hits; it reproduced both original files exactly. The resolved Nx graph now has no plugin or test-source inputs on either font target. The standalone Cargo check also passed (793 ms). The focused native-runtime suite passed in 26.6 seconds after the bounded process-table wait, including both profile restorations, native Rust consumption, HTTP responsiveness and owned cancellation.

The final font repeat restored 2/2 tasks from Nx in 298 ms. Moving the final exporter deliverable out of its output directory and rerunning restored it with 1/1 hits in 163 ms; independent checks again matched every file's SHA-256 and executable permissions.

The final-source full suite passed all new selected-runtime and font checks, editor coverage, 705-project inventory, and native cancellation. A second older Flow assertion at cache-contracts line 1036 still compared equivalent output declarations as literal strings; it now applies the same validated physical-path comparison as the earlier ownership assertion. No producer path or ownership requirement changed. The corrected full rerun is active.

## Package Project Identity and Configuration Inputs

New package manifests at the Puzzle Rust producer and Flow build producer independently overrode their authored Nx project names. Native Nx inferred the npm names instead, invalidating preparation/editor references. Each manifest now sets its explicit `nx.name`; its npm import name remains unchanged. The neutral selected-runtime cases also verify both names through installed Nx's package-project builder and reproduce the unassigned-name collision in memory.

The focused Puzzle identity check first failed with `@semio-tech/puzzle-wasm` instead of `@semio-tech/puzzle-plugin`, then passed. A later full suite passed editor coverage and the 706-project/7294-target inventory before failing on the analogous Flow engine name; Flow's authored identity is now explicit too. The first focused Flow RED attempt exposed an incorrect package-manager argument in the test oracle rather than the name assertion; that oracle now uses native Nx's `getPackageManagerCommand`. The full-suite Flow failure is the recorded behavioral RED.

Authored playground selection now lives in `🎮️playground/🧭️selection/🟦️.ts`. Port and generated-catalog consumers no longer import taxonomy discovery. Existing selection semantics and neutral source-manifest fixtures remain unchanged. The OS-dev quick rerun confirms the forbidden-discovery graph check passes. Its remaining failures are graph size/Bun source-map parity, component URL rewriting, two manual catalog orchestration assertions, and an atomic-save watcher assertion. The quick suite reported 120 passed, 28 skipped and 6 failed; no overall pass is claimed.

A disk-full event temporarily prevented edits. The target file was verified intact, and only idle disposable fixture output under this ticket's `🗑️generated/nx` was removed after checking for open handles. No shared compiler cache or another task's process was modified.
