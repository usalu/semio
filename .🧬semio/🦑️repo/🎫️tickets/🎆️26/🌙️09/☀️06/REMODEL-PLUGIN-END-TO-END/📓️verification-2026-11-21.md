# Remodel verification, 2026-11-21

The existing Remodel end-to-end ticket was already open. The current worktree still needs a native workspace test pass, a rebuilt WASM component and descriptor pair, catalog publication, and acceptance on a prepared host.

## Verified

- The full Remodel Vitest config passed 1,405 tests across six files after updating the declaration fixture for the browser test entry, correcting the package root used to validate closure resources, and picking up the first new browser law. The first run found the missing entry and the second found the invalid resource path.
- Remodel TypeScript passed `tsc --noEmit --incremental false -p` with the package's `tsconfig.json`. Fixed the repository observation declaration for non-spilling capture, which returns a real array, and narrowed Rust cache reclaim's JSON partition to its receipt's partition. Regenerated local UI bindings through the original Rust render function. The output differed only in its two missing container roles. Recompiled and passed the canonical native projection law comparing the full output byte-for-byte.
- The repository-managed JavaScript test, TypeScript check, UI projection generate, and component build calls each timed out waiting for the MCP response. The fleet still showed a live planner attributed to the TypeScript test and an independent setup operation with advancing dependency work. The fallback runs used the installed toolchain. No shared process was terminated.
- The pinned offline Cargo check for `semio-hub-remodel-component` from the Hub workspace, `--locked --target wasm32-wasip2`, exited 0 in 11m32s. It emitted existing warnings. A full build of that target and a separate native build of the owned descriptor emitter are in progress.
- The pinned offline native bridge compiled successfully. Its standard libtest binary contains zero tests. The bridge inventory and full native editor/artifact runs remain pending.
- The browser lane passed 27 focused tests covering Remodel fixtures and the shared browser port. Details and limits are in [the browser acceptance report](./📓️browser-acceptance-2026-11-21.md).

## Current work and remaining evidence

- The production QC report read its accepted-frame denominator from staging after the engine had moved the accepted frames into its working set. The report now uses the engine's accepted count across that ownership transfer. The neutral orbit cases and their Rust consumer check the final commit's ratio against the accepted/registered counts. Native execution of this regression is pending.
- A reconstruction checkpoint now includes the presence of ground control, which changes the QC report's warning. The neutral revalidation fixture requires withdrawal after adding the first GCP or changing ingest stride and keeps the result across an identity-only edit. Native execution is pending.
- The committed component JSON still carries `remodeling-qc-report.ops`; the source emits JSON. After linking, run `semio-plugin-describe describe <component.wasm> --out <staging>` to exercise the actual component and emit the descriptor pair. Managed publication additionally needs provenance and a successful component model receipt. The catalog has separate `generate`, `check-generated` and `verify-catalog` targets.
- The browser driver cannot get a host from the managed calls in this environment. A direct loopback listener returns EPERM, and the installed Chromium fails sandbox initialization. Browser contract or fixture tests do not establish the live user journey.
- The calibration lane found the engine approximated one camera for all streams. It is replacing this with per-view calibration in matching, SfM, bundle adjustment, dense and texture stages, and preserving distinct frame verdicts across streams. Native and numeric acceptance of this change is pending.
- The diff lane found the retained API silently skipped unsupported keyed changes. It now refuses these before staging scalar fields. The production cold path stages affected collections before publishing any field. The [diff publication report](./📓️retained-diff-publication-2026-11-21.md) records fixture evidence and pending native coverage.
- The editor now routes the resumable mesh download through its original snapshot lease and GLB cursor. The native legal/publication path and browser download still require execution.
