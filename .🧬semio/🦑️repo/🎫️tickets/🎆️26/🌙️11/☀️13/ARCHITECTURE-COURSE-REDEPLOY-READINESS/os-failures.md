# OS product failures

None of the involved sources was modified in the last hours by anyone else: every file is byte-identical to HEAD (`git diff HEAD` empty) with mtimes of Oct 16 to Nov 12, so nothing was in-flight and all were fixed at the source.

## 1. `framework-os:typecheck`
Root cause chain (a stack of drifted crates the wasm dependency `semio-framework-os-flow-browser-package:wasm` compiles, plus TypeScript drift). Fixed one by one:
- `OriginalConstructionAdmission::has_failure` was renamed to `is_failed` / `failure_untaken` in the job crate; `🎬️media-export/🦀️.rs` (lines 107, 116) now calls `failure_untaken`.
- `🧵️instance/👷️jobs/📥️staging/🏷️reserved/🦀️.rs`: two identical `handoff` definitions (E0592); the older duplicate removed.
- `🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`: `receive_archive` took `u64` but both callers hold `u128` work; it takes `u128` and converts once.
- `🌊️flow/🕸️wasm/📡️protocol/🦀️.rs`: private `FlowFailureOwners` leaked through a public associated type (E0446); made `pub`.
- `🌊️flow/🕸️wasm/🧵️dag-input/🦀️.rs`: `pause()` and the control `Err` now carry `(ValueError, control)`; four call sites use the error part.
- `🌊️flow/🕸️wasm/🎞️actions-b/📤️drawing-export-and-snapshots`: `close_step` now runs through `cold_effect` like its siblings; `fault_message` covers `BitmapTraceError::Refused`.
- `🌉️mcp/📦️packages/🦀️rust/📜️script.ts`: the shebang sat on line 2 (TS18026); moved to line 1.
- `✍️write/🟦️.ts` (store worker): `response.arrayBuffer()` typed through `BinaryFetchTimeoutResponse` like the poll lane.

Result: the wasm task now builds (`semio-framework-os-flow-browser-package:wasm:prototype` ok). The TypeScript stage of `typecheck` still fails with about 150 pre-existing diagnostics across the os package (not touched): missing generated modules (`🤖️generated`, `@semio-tech/flow-core`, `@semio-tech/framework-surface-rs`, jcoprobe bundles), an out-of-date schema export surface (`InferenceRetained*`, shell `generated` types), Bun `SQLQueryBindings` test typings, xstate and renderer test typings. None of the 14 rewired files appears in the diagnostics. This needs its own ticket.

## 2. `framework-os:test`
Initially 762/765, the folder-write suite crashed before running, so the real figure was larger. Final: 38 files, 782/782 tests.
- `folder-write-acknowledgment`: the test iterated `corpus.cases`, but the fixture v2 holds `native` and `worker` sections. Fixed the test to read `corpus.worker` (`🧪️tests/💾️folder-write-acknowledgment/🟦️.ts`). This executed 17 worker tests previously hidden, two of which failed:
  - `folder-archive-restore`: the Nov 4 write law adds one divergence read before every write, so three reads, not two, is the correct count (`reads.count` 2 to 3; test stale).
  - `config-facet-fold` "holds an envelope that arrives before the folder's first answer": product bug. `pollFolderOnce` refused the first answer as `external-divergence` because the held facet envelope sits in `pendingMutations`, so it never folded. A waiting backlog is no baseline edit; `📁️folder/🟦️.ts` now skips the divergence refusal while `facetBacklog` is non-empty.
- `app-channel-codec` (2 failures): `SubmitMediaExport` carries `checkpoint_id` (`null`) in the product and shared wire vectors; the two test literals lacked it (`🧪️tests/🧾️app-channel-codec/🟦️.ts`).
- `actor-transport` codec bridge: two causes. The test stand-in reactor lacked `admitActorInput` (bridge now admits a wire before a codec call); stand-in extended in `🧬️component-codec-reply/🟦️.ts`. The generated bridge crashed with a TypeError on a component with no `codec` export, instead of the fixture law `actor-codec.unexported`; product fixed in `🌉️component-bridge-source/🟦️.ts` (`codecCall` rejects before admitting).

## 3. `renderer-wgpu:test`
Fixed (not yet re-run to green, see Unverified): `📦️packages/🟦️typescript/📜️script.ts` `assertRendererOutputOwnership` asserted `dist/wasm-<profile>` while the project declares `{projectRoot}/.🧾️store/wasm.json` since Oct 20 (the compiler publishes into the content-addressed store and writes that receipt). The assertion now pins the declared output; the unused `rustPackageRoot` removed.

## Rewired test files
- ShellHelpers `command-rejection`, `renderer-types-and-forms`, `runtime-admission`, `router-plugin-faults`: `framework-renderer-react:test` with `SEMIO_TEST_LEVEL=long`: 229 of 231 files pass, 3252/3252 tests, the four rewired files green (3, 29, 16, 1 tests). The two failing files (`hidden-tab-surfaces`, `node-graph-hosts`) cannot resolve generated `flow_core.js` wasm bindings; unrelated build-graph gap, not touched.
- store-sync `command-rejection`: in-source suite of the os root `🟦️.ts` (23 tests), green in `framework-os:test`.
- plugin `shardclient-*`: covered by `actor-transport` (179 tests), green.
- docklayoutstore `plugin-source`: `@semio-tech/framework:test` (6 files, 370 tests; docklayoutstore 91) green after fixing the missing JSON import attribute in `🖱️ui/🧬️contract/🌱️values/🔤️text-chunks/🟦️.ts`.
- plugin poll composition: see Unverified.

## Verification and open items
Runs in the retired tree `/Users/Shared/semio` (all exit codes from `repo run <project>:<target> --skip-remote-cache`): `framework-os:test` 0 (38 files, 782/782); `framework-renderer-react:test` with `SEMIO_TEST_LEVEL=long` 1 (3252/3252 tests, 2 files unresolved generated wasm bindings); `framework:test` 0 (6 files, 370 tests); `framework-os:typecheck` 1 (wasm stage now green, about 150 TypeScript diagnostics remain); `framework-os-dev:test` 1 (242 passed, 2 failed tests plus 1 failed file, none rewired).
All edits were confirmed byte-present in `/Users/ueli/Documents/semio` (grep of each change); the new tree was not re-run: `repo run @semio-tech/framework-renderer-wgpu:test` there failed twice with exit 1, "graph service could not answer (timeout)" (cold graph).

Unverified: `renderer-wgpu:test` green after the assertion fix (never ran to completion); the post-edit os tests in the new tree; poll composition passed only by absence from the `framework-os-dev` failure list.
New unrelated failures in `framework-os-dev:test`, not investigated: `🧪️time-travel/🚀️boot/🧪️tests` (`variant` of undefined catalog row), `🧹️config` module-graph bounds (561 > 40) and a missing ui styling module in the browser entry graph.
