# wgpu type-check, tests and neighbour laws (2026-11-23)

Lane: continuation of the P5a Rust fixes. One cargo process at a time through `repo cargo --store=shared --configuration=prototype`, `-j 2`, gated on `memory_pressure` free of at least 40 % and load of at most 20 (`wgpu-check.sh` in this folder takes `check|test <crate-dir> [cargo args]`; `wgpu-tests-chain.sh` queues the test-crate checks; logs under `🗑️generated/wgpu-<mode>-<crate>.txt`, summaries `🗑️generated/wgpu-summary*.txt`).

## 1. P5a crates and the two blockers

### Root cause: `wasmtime-wasi-io` / `wasmtime-wasi` cannot find their WIT and witx files
- `wasmtime::component::bindgen!({ path: "wit", .. })` (`wasmtime-wasi-io-47.0.4/src/bindings.rs`) and `wiggle::from_witx!({ witx: ["witx/p0/wasi_unstable.witx"], .. })` (`wasmtime-wasi-47.0.4/src/p0.rs`, `p1.rs`) read a file or directory through a proc macro, not through `mod`, `#[path]` or `include!`. The files exist on disk, but the offloaded compilation ships only what `reachedFilesV1` finds (the `.rs` tree and `include*!` targets) plus the dependency file of an earlier compile, so a first compile on the remote side saw no `wit` directory: `failed to read path for WIT [...]`. Same defect class as the earlier `include_wgsl!` fix.
- Fix in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts`: new `BINDINGS` scan. The `path:` of a `bindgen!` or `generate!` macro and the `witx:` of a `from_witx!` or `wasmtime_integration!` macro (a string or an array, relative to the manifest) are reached files. A directory travels as all files below it; a `.witx` file travels with its sibling files because its `use` declarations name them. Test `🧪️tests/🗺️paths/🟦️.ts`, "the WIT and witx files of a bindgen!, generate! or from_witx! macro are reached": 5 pass, 0 fail. The wider `🧪️tests/🟦️.ts` offload suite: 40 pass, 1 fail (`a unit whose recorded local cost is below a round trip ... the workspace root is found from any directory below it`, an existing cwd assumption, unrelated).
- Result: `wasmtime-wasi-io` and `wasmtime-wasi` compile in the frame-transaction check (they were the first errors of the previous p5a run).

### Root cause: unimported `NativeRecordKey`
The peer `[DEBUG]` line in `🌱️value/🛬️decode/🧳️owned/🌳️value/🔀️variant/🦀️.rs` is gone and the file now imports `NativeRecordKey`; `semio-framework-value-native-owned` compiled in the run. No edit needed.

### Errors found and fixed while checking
- `🪝️shell-window-actions`: `use semio_renderer_wgpu_shell_plugins::PendingExtensionInvocation` was gated on `wasm32` while the `Effect::InvokeExtension` arm that builds it is not (stale-overwrite regression); the gate is removed.
- `semio-os-store-initialization-replay` (`🏪️store/🏗️initialization/🔁️replay/🦀️.rs`): a peer's `[DEBUG]` `eprintln!("{:?}", turn)` needed `Debug` on `enum Turn`; `#[derive(Debug)]` added (the peer line stays theirs).

### Type-check results (`repo cargo ... check`, `-j 2`)
| crate | result |
| --- | --- |
| `🧾️frame-transaction` (with `frame-build`, `frame-actions/runtime-apply` and the whole dependency closure) | exit 0 (warnings only: unused imports in frame-transaction `🦀️.rs` lines 11, 14, 21 are peer leftovers) |
| `📨️runtime-mailbox` | exit 0 |
| `🪀️shell-view-panels` (with shell-pointer, shell-window-actions, shell-plugins, shell-state and the other Shell staging crates it pulls) | exit 0 |
| `🍱️shell-view-chrome` | see section 2 |
So all nine P5a Rust edits type-check; the P5a law test stays green (1 pass, 0 fail, rerun after the law-file edits of section 4).

## 4. Neighbour laws

### `interactivity-mounted-engine-surface-lifetime`
Green (1 pass, 0 fail) when this lane started: the test already runs through `bun:test`, reads the live crate trees (`policyModuleTreeFiles` with `crates: true`) and compares mutations against the baseline. No change needed.

### `interactivity-live-reconcile` (was 0 tests, then `per-surface-credit-cap made no source mutation`)
Now green (1 pass, 0 fail, about 70 s). All fixed without weakening:
- A mutation row lacked its trailing comma (`render-fault-wire-erased`), turning two rows into an index expression (`undefined is not an object`).
- The harness follows the P5a pattern: one baseline of the live source, a mutation counts only when it yields a failure that is new against the baseline, every problem is collected into one error. Rows that replaced only the first occurrence use `replaceAll` where the needle also occurs in a duplicated copy (`kernelTurn`, `tableKit`), because a single replace left the duplicate satisfying the law: `aggregate-credit-cap`, `deep-retire-overflow`, `semantic-byte-fiction`, `missing-patch-*-fixture`, `missing-turn-patch-*`, `dynamic-turn-patch-retirement`, `duplicate-turn-patch-token-publication`, `dynamic-table-row-owner`.
- Source drift re-anchored to the live code (law files `⚖️laws/🔬️interactivity-audit/🖥️mounted/**`):
  - Table kit: `TableRows`, `TableRowsView` and `close_table_rows_view_one` no longer exist; rows are windowed through `TableWindowKit::render_rows` and `table_window_row` on the shared `TreeWindows` ledger. The test mounts `🪟️window-kits/📊️table/🦀️.rs` and the three `table_kit_*` window fixtures; the law requires `UiFixedList` cells, fallible `try_row_action`, `claim_window` and `sliced` before rows, and forbids `ui_wgpu::wgpu`, `Vec<TableRow`, `entries.to_vec()`, `entries.iter().collect` and `serde_json` in the windowed section. Obsolete mutations were replaced by windowed twins (`whole-table-row-collect`, `missing-table-window-claim`, `missing-table-window-slice`, `missing-table-window-first-paint-fixture`, `missing-table-window-heavy-rows-fixture`, `renderer-table-row-action`, `whole-table-row-json`).
  - Turn patch transport: the bridge boundary starts at `to_actor_turn_result_in_place` (the effects encoding precedes the owner take); `match to_actor_turn_result(..)` became `let bridged .. in_place` plus `match bridged`; the third explicit `close_ui_turn_patch_transport_one()` site is now the lease `Drop` handback, which the law requires together with `.take_owner()`.
  - The WIT bridge (`kernel_ui_patch_to_wit`) is read from `🧬️wit-bridge/🦀️.rs` instead of its first `.rs` child.
  - Published ack: `Event::PatchAck {` handler, `apply_issued_ack(.., |ack| .. mark_published_ack(ack))`, ordered before the mark.
  - Mounted render: boundary `let mounted = match native_close_key(..)` to `drive_turn_reconcile`; requires `reserve_mounted(surface, key)` and `PATCHES.with(|patches| patches.defer(surface))`; `drive_one(&self, cx: &mut StepContext<'_>)`.
  - Shell document retirement: boundary `drain_retained_document_arenas` to `close_document_one`.
  - Strengthened: the credit cap needle now ends in `;` (it was a prefix of the doubled mutation text, so `aggregate-credit-cap` was falsely accepted); `rejection-generation-before-terminal` moved to the live `mark_rejected` shape.

## 2. wgpu test crates (`check --tests`; no test was executed)

Test-target type-check (`repo cargo ... check --tests -j 2`). Running the tests (`cargo test`) needs full codegen of the whole closure and was not started: the host was saturated (load 15 to 29, compiler-image reclaims, other lanes' compiles) and each check took 20 to 60 minutes.

| crate | result |
| --- | --- |
| `🍰️shell-tests-input-3` | exit 0 |
| `🥧️shell-tests-docs-1` | exit 0 after fix (below) |
| `👷️frame-job` | exit 0 after fix (below) |
| `🪤️runtime-mailbox-tests` | exit 0 |
| `🥪️shell-tests-docs-2` | one error fixed (below); re-check queued (`wgpu-tests-chain3.sh`), result in `🗑️generated/wgpu-summary.txt` |
| `🪜️engine-attach-tests-2` | two errors fixed (below); re-check queued (`wgpu-tests-chain4.sh`); the last attempt stopped on a peer's in-flight `[DEBUG]` borrow in `🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs:92`. `fit_graph_action_persists_the_fitted_camera_like_a_wheel_zoom` has not run |

The "known unresolved symbols" (`operation_publications_from_json`, `take_media_actions`, `push_media_action`, `pinned_app_for`, `relay_open_plan`) no longer fail: they live in `shell-dock` (`⚓️dock/🔑️storage-prefs-and-i18n`), `shell-plugins` (`🧲️plugins/🔖️identity-and-types`) and `program-bridge` (re-exported through its crate root) and resolve in docs-1 and docs-2. The real errors were different:
- `engine-attach-tests-2` (`⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs`): `DagHost::camera_content_coverage` is now the free function `flow::dag::camera_content_coverage(&viewport, &graph)` (call site updated); `DagGraphState::handle_id_for_channel` was `pub(super)` but is a typed-channel read the engine test needs, so it is now `pub` with a docstring (`♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🕸️graph/📓️graph-state-journal-and-hits/🦀️.rs`).
- `shell-tests-docs-1` (`🪟️wgpu-time-travel/📐️band-layout-and-focus`, `👥️notices-peer-edits-and-finalize`): `PanelsViewCtx::of` and `chrome_accessibility_nodes` take `&mut ShellState`; helpers now take `&mut ShellState`.
- `shell-tests-docs-2` (`🔗️hub-projection-workspace/⏱️frame-pumps-and-edi..`): `frame_pump(shell: &mut ShellState)` passed `&mut shell`; now passes `shell`.
- `frame-job` (`🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs`): `admit_pending_batch(config, &mut output, cx)` returns `bool` with the session in `output`, and `acknowledge_outcome(cx)` replaces the retained-step closure; the fixture follows both.
- Peer `[DEBUG]` code that blocked dependencies: `🧬️guest-contract` `dispatch_value_step`/`dispatch_cleanup_value_step` (`dbg_cause(&error)` on a generic `E`: converted to `OriginalTurnFailure` first) in `⚛️reactor/🎟️context/🌉️call/🦀️.rs`; `🔁️replay` `Turn` Debug (section 1).

## 3. Renderer merge compile issues
`WORLD3D_POINTS_SHADER` (`semio-ui-render`/draw), `render_widget` and `BlockListRole` (`scene-tables`/`block-list-and-diff`), the `engine-attach` `registry-and-node-graph` tuple mismatch and the Shell source-boundary overruns no longer occur: every crate above compiles through those dependencies (frame-transaction, runtime-mailbox, shell-view-panels, shell-view-chrome, engine-attach inside engine-attach-tests-2), apart from the two test-file errors listed in section 2.
