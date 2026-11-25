# Browser E2E v11: freeze, load cancel, long-phase progress

## Result
`process-interactions.json`, `process-load-trace.json` and `process-cancel.json` each exit 0 on the rebuilt hub (final runs 09:44 to 09:48). Final screenshot `e2e-process-v11.png` (viewed): shell alive on Concrete Forest, Editor role, Actions panel with Step 4/4, Undo highlighted. The 3D canvas area renders dark in headless Firefox (not investigated). `process-load-trace.json` has no `until`, so its trace is empty; progress evidence comes from the cancel runs. Not verified at runtime: the new Rust laws (see Tests).

## 1. Viewer to Editor "freeze"
Not a main-thread stall. A 250 ms heartbeat in the page stayed at 250 to 300 ms (rare spikes of 1 to 5 s under machine load) during the whole switch on Demo and on Concrete Forest. The 14,000 `[DEBUG] done tag` lines (about 4,500 per 10 min) do not stall the page. Two real causes:
- **Guest panic.** About 22 s after the Viewer switch finished, the guest panicked at `📦️original/🦀️.rs:26:65` ("closed original App cannot serve another live request"). That is when the predecessor's close ladder ends. The live cleanup job's late drain (`runtime_drain_maintenance_source`) asked a closed `OriginalVcsApp` for `maintenance_source_*`, which was not closed-aware. A guest panic kills the whole component, so every later action, Increase and Undo failed. Fix: `OriginalVcsApp` now answers the three `maintenance_source_*` methods as drained once closed. Verified: Viewer switch on Demo, 345 s with no panic.
- **Silent restore.** The switch restores the whole archive into a new instance (32 s on Demo with 5 members, minutes on Concrete Forest) with no progress or cancel, and Editor clicks are refused as busy meanwhile. Fix: the restore now runs as a Tasks-window task with progress and cancel (`restoreDocumentSurfaceV1`); a cancel rolls the successor back and keeps the predecessor, with a "load cancelled" notice. This applies to the primary and spawned role switches.
- Also seen once: `Error: out of memory` right after a Viewer click, during heavy machine memory pressure; not reproduced.

## 2. Load cancel
- Guest: `request_document_archive_cancel` cancels the awaited replacement in the same step as the load, and refuses once the replacement has committed. `CandidateReady` now honours cancel in the ladder (`drive_store_replacement_unit` and its quote). Measured: replacement refused at the next turn after the cancel (22 to 74 ms), previous document kept.
- Defect found by actually cancelling: retiring the in-flight member open failed 27 to 28 s later with `schema close changed Source or Work` (the live cleanup job then faulted and poisoned the instance). Cause: `drive_rejected_members` took the source parent from the loan context, unlike its sibling branches. It now uses the outer `cx` parent. This also takes the rest of the retirement off the failing path; cancel run is now clean. The value-layer check itself (`🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema`) was not touched.
- Shell: banner hides at once on cancel, and the picker returns to the example before the pick (verified "Demo"). The load gate tracks a cancelling load. A load that ends without a document now refuses its action as `document-load.cancelled` or `document-load.failed`.
- Results: three runs before the parent fix failed with the schema error; after it, `process-cancel.json` exits 0 (state after cancel `["Demo",1,""]`, no console error).

## 3. Long-phase progress
The long phase is `OpeningMembers` (stage count stuck at 129 of 135 for 7+ minutes in the old hub). The replacement job now counts the items its own steps paid (`decoded`, wrapper `drive_store_replacement_original`); the load status adds them as discovered and done. Observed banner: "88085 of 88091", then "92131 of 92137" during one load.

## Tests
- Rust: `progress_laws` and `stage_laws` in `semio-plugin-document-media` pass (2/2). New, not run: `a_cancel_at_any_point_of_a_load_restores_the_previous_document_or_is_refused_after_the_commit` and `a_whole_document_load_counts_the_records_its_replacement_decoded` (`🧪️folder-reload-route`). Their crate `semio-plugin-tests-06` does not compile (76 stale-API errors, other agents' area). The registration law (closed original reads as drained) was added to `actual_native_window_app_opens_and_closes_with_its_original_registered_sources`; it compiles, but the test fails earlier at `registration/🧪️tests/🦀️.rs:54` (receipt accounting, not this change). `guest-check.sh`: instance docs, ladders, time-travel, render, app and construction crates check clean.
- TS: vitest `surface-document` 16/16 (restore progress/cancel/rollback); bun `document-load-gate` 20/20 (load-end refusal fixture rows, cancelling gate, running status); vitest example switching 11 (restore after unapplied load). tsc over the changed files: no new errors.

## Environment findings
- Firefox was repeatedly SIGKILLed by macOS under swap exhaustion (about 29 GB of leaked `semio-ff-*` profiles on a 99 % full disk, now cleaned). The harness removes its profile, takes `BIDI_PORT`, stamps console error stacks, stops Vite HMR reloads in the page, and fails fast when the browser dies; `run-e2e-retry.sh` repeats. `FF_PREFS` with `wasm_optimizingjit=false`, one content process and no Fission was used for the runs (slower guest).
- Diagnosis tip: `localStorage.SEMIO_RUNTIME_DIAGNOSTICS=1` (harness `PRELOAD`) prints guest `[TRACE]` failure causes.
- The published hub is the final build (no Rust `[DEBUG]` left in source); all temporary `[DEBUG]` TS prints were removed.

## Changed files (under `🧰️framework/🛍️products/💻️os/🔨️modules/` unless noted)
Rust, `🔌️plugin/`:
- `🧵️instance/🧱️construction/📦️original/🦀️.rs`
- `🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs`
- `🧵️instance/📄️docs/📥️staging/🧱️components/🦀️.rs`
- `🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`
- `🪜️maintenance-ladder/🦀️.rs`
- `⏪️time-travel/🧵️instance/🔄️reprojection-turns-and-base-watch/🦀️.rs`
- `⏪️time-travel/🧵️instance/🦀️.rs`
- `🗃️document/🧾️store-replacement-vocabulary/🦀️.rs`
- `🗃️document/🔁️store-replacement-admission-and-member-drive/🦀️.rs`
- `🗃️document/🗄️document-archive-load/🦀️.rs`
- `🗃️document/🧰️store-replacement-initializer-and-retirement/🦀️.rs`
- `🧵️instance/🔌️plugin-app-impl/🦀️.rs`
- `🧵️instance/🖼️render/📥️staging/📜️history-view/🦀️.rs`
- `🧪️tests/🧪️folder-reload-route/🦀️.rs`

TS, `📺️renderer/🧑‍🎨engine/`:
- `🧱️elements/🏛️ShellHost/🛫️spawned-session/🟠️switch-plugin-app/🟦️.ts`
- `🧱️elements/🏛️ShellHost/🛫️spawned-session/👁️switch-spawned-role/🟦️.ts`
- `🧱️elements/🏛️ShellHost/🛫️spawned-session/🔶️document-services/🟦️.ts`
- `🧱️elements/🏛️ShellHost/🛫️spawned-session/🟦️.ts`
- `🧱️elements/🏛️ShellHost/🔀️surface-switch/📄️document/🟦️.ts`
- `🧱️elements/🏛️ShellHost/⚡️host-effects/📥️document-load-effects/🟦️.ts`
- `🧱️elements/🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🟦️.ts`
- `🧱️elements/🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🚀️program-dispatch/🟦️.ts`
- `🧱️elements/🏛️ShellHost/🧩️shell-core-state/🟦️.ts`
- `🧱️elements/🏛️ShellHost/⏳️document-load-gate/🟦️.ts`
- `🧱️elements/🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts`
- `🧪️tests/📄️surface-document/🟦️.ts`
- `🧪️tests/⏳️document-load-gate/🟦️.ts` and `🔣️.json`
- `🧪️tests/🔒️shell-option-locks/📚️example-offers-and-switching/🟦️.ts`

Ticket folder: `verify-process-firefox-bidi.ts`, `run-e2e-retry.sh`, `run-e2e-repeat.sh`, `guest-check.sh`, `guest-tests.sh`, `hub-rebuild-v11.sh`, `vitest-surface.config.ts`, `tsconfig-v11.json`, `process-viewer-probe*.json`, `process-viewer-forest-probe.json`, `e2e-process-v11.png`, `e2e-process-v11-cancel.png`.
