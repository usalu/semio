# Renderer Fixture Context Audit

The current production FrameBuild caller already passes its original context through `ShellState.render_chrome_step` and propagates its `OriginalTurnFailure`. The native owner migrated eight private Shell render helpers and Interpreter document/layout rendering to pass that same caller context. This removes the former locally fabricated document/layout step context.

The remaining fixture callers found by the execution-web search are in EngineCanvas retained surface tests, Shell chrome/actions/search/shortcuts/accessibility/time-travel/presented-input/tool-run/input/window-measures tests, and Interpreter UI command-wiring tests. They still invoke `render_ui_document_step`, `close_ui_document_one` or `render_chrome_step` without the original test context. Close and chrome now return a Result, which success fixtures must explicitly acknowledge; refusal laws must retain the actual original failure and paired receipt rather than asserting on a surrogate enum.

The current actual context contract is `StepBudget::new(fuel, absoluteDeadlineUs, retainedGrant)` and `StepContext::new(operation, generation, budget, ownedCancel, clockFn, previewRecipient, retainedRecipient)`. For persistent fixture callers, `StepContext::with_payload_authority` borrows the original admitted cancellation and payload ledger while keeping the same declared identity, clock, preview and retained receipt recipient. Those callers must retain their authority across successive paint or close opportunities; constructing a new cancellation/ledger inside each Interpreter call would repeat the production defect the native owner removed.

A fixture helper should belong to the test caller, accept its authored finite grant, preserve one identity/cancellation/ledger and recipient across the loop, and explicitly retire those owned fixture authorities after the law closes the retained documents/jobs. Reusing an arbitrary global root cancellation or increasing a denied test grant would invalidate the original-owner law. The existing UI worker retirement policy allows one item, 65,536 copy/capacity bytes, 262,144 release bytes and depth 64 per opportunity; fixture grants must remain authored and visible.

The immutable Nx daemon/runtime acceptance gate remains the execution-web priority. This audit does not claim that fixture migration or renderer runtime compilation has completed.


## Current owned renderer adaptation

FrameBuildCursor.close_step and FrameTransaction.close_step now require the original borrowed StepContext and return OriginalTurnFailure. The existing frame-job ActiveFramePhase::Build caller forwards that same context and uses the received-child path, preventing a second physical debit. Cursor closure narrows through the existing retained loan rather than enlarging policy. World context closure debits its original context itself; world rejections and priced prepared uploads quote demand and pass through one original primitive helper, preserving blocked, failure, actual progress and CPU. AppFrameAfterChrome directly closes priced input/upload owners through that context instead of allocating a fresh retirement job without admission. Unknown EngineCanvas/raw draw authority retains the explicit unpriced refusal while the native owner migrates that distinct contract.

Normal successful FrameBuild completion now drives World.close_step(context) after transferring the wake token and before asserting physical terminal emptiness; the native owner requires five backing buffers and request strings to be retired under actual release credits, so append completion alone cannot authorize dropping the context.

A cfg-test caller owner at `🧪️tests/🎟️original-turn/🦀️.rs` keeps one admitted cancellation/ledger, fixed original identity/clock, preview recipient and physical receipt recipient across successive opportunities. It explicitly acknowledges only funded previous receipts and retires its true authority after retained children close. The first Shell window-pane-chrome law now forwards this same fixture caller through the paint loop and World retirement. Existing portable frame-close corpus remains the declared grant source and independent allocator/runtime oracle source. The remaining stale renderer fixture callers are not yet migrated. Actual canonical `@semio-tech/framework-renderer-wgpu:test-wgpu-unit` is running; no compile/runtime green is claimed.

Touched renderer paths: engine target root `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`; frame-job `🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs`; cfg-test original-turn owner; Shell `🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs`. Native owns World/EngineCanvas child contracts and public prepared quote/witness exports; their concurrent changes are preserved.


## Post-Restart Original Fixture Caller Migration

The fixture caller migration now covers 40 actual fixture functions/helpers across 17 files (including the previously migrated pane-chrome law). Each authored caller keeps one admitted cancellation, ledger, fixed original identity/clock, preview sequence and physical receipt recipient throughout its paint or close loop. Success-returning helpers explicitly retire their actual World3dBuildContext under the same caller before returning UI/input evidence; fixed contexts nested in corpus loops are retired inside that original scope. Close/chrome Result callers explicitly receive the real original failure, and boolean document/private-helper calls retain their existing boolean contract while forwarding the original context. No new migration script remains.

Shell icon advancement now enters the synchronous render_chrome_step under its actual incoming original frame context and respects exhausted fuel. The detached async settle lane no longer fabricates icon advancement authority. The native workstream owns the scene/batch methods and the parent owns the broader OsHost/router original root lifetime. This authored migration remains pending actual Rust compilation/runtime acceptance.

Touched fixture files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🛟️panel-window-reservation/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`

The actual camera-source canonical Nx route after restart passed 14 tests and 300 assertions in 321ms; its route currently runs the frame-close/source/independent SQLite laws. It is not a renderer Rust runtime green.

## Browser Original Teardown Ingress

The browser already receives its original admission, identity, epoch, finite fuel, absolute deadline and five ownership axes through OriginalActorGuestEntry. Its old closeOriginalFrame callback only closed frame-build aliases; TypeScript then retired that original entry before the separate closeStep began host/world teardown. The browser now moves its same admitted guest entry into one temporary owned slot, forwards the genuine dispatch_cleanup_step context through close_step_on_original and receives the full Result<bool, OriginalTurnFailure> from its own OsHostRetirement. The same entry returns before the scalar receipt is published. Original context retirement follows full child teardown. The public closeStep is now a terminal witness only. Browser ticks and teardown no longer reap a global abandoned host under a different caller's context; the parent owns the global original-source lifecycle repair.

The existing canonical test-original-frame-source now includes both its former shard-grant corpus and the actual OriginalBrowserFrameSource harness. Schema-first closeStages require frame/assets/world/host-terminal progress before context/context-terminal closure. The handcrafted portable corpus uses twelve genuine scalar callback returns, keeping original operation71/generation3, deadline12345 and exact conservation across both presentation generations. Ajv independently validates the corpus; SQLite independently confirms close-stage order and remaining fuel/copy/capacity. The source harness also requires the actual browser callback to forward its incoming context to its own retained host and rejects a global caller-free reaper.

Physical red `browser-original-host-close-red.log`: the schema rejected the old nine-turn/missing-stage corpus. After its handcrafted update, `browser-original-host-close-current.log` passed the actual browser Source law and the existing checked-host refusal law; strict TypeScript compilation also completed before the runtime tests. The combined canonical gate remains red because three pre-existing shard-grant laws fail (old 112-byte fixture versus the current 108-byte wire buffer, stale resource-override source marker and stale batch schema). The parent owns those shard/root laws. This is not a WASM/native renderer runtime green.

Additional touched sources: browser-worker/🦀️.rs; WGPU package canonical 📜️script.ts; frame-worker/🎟️original/🧬️schema/🔣️.json; its 🧪️tests/🟦️.ts and handcrafted 🧫️fixtures/🔣️.json. The native owner added the ticket's browser-original-source Nx runner route to execute this existing canonical command. No external runtime dependency or new migration script was introduced.

## Complete Current Browser and Shard Source Gate

`browser-original-host-close-green.log` passed the complete current canonical source target: strict TypeScript compilation, five tests across two files, 98 assertions, 931ms total. The three shard failures were corrected against the actual current source and existing authored wire bytes. The 112-byte wire corpus was already correct: the independent Node Buffer encoder omitted the actual Budget.max_frames u32 and wrote the original authority in the wrong field order. Actual Budget::pack_encode has eight fields totaling 38 bytes; tag/actor contribute nine bytes, RetainedTurnInput contributes 64 and the empty envelope varint contributes one. The repaired independent Buffer produces the existing exact 112-byte hex without changing the expected fixture. Source custody checks now inspect IssuedShardTurn's actual neutral IssuedRetainedBatch dereference and the neutral original_input accessor. Its existing activeRemaining and closedRecipient corpus fields now have explicit closed schema properties, matching the actual receiver's retained active spans and post-finish refusal; no conservation expectations were enlarged.

Concurrent original-output integration added one genuine output acknowledgement return to the browser law. This final green preserves that thirteen-return corpus and its exact copiedItems12/copy6/capacity26/release26/work9 receipt, remaining fuel8/copy122/capacity230 and the same original deadline12345. The browser retained-host close call is again `retired.close_step(cx)?` under the agreed actual host Result contract, preserving the concurrent original-output work. Native/WASM GPU runtime remains separate acceptance.

Additional files touched for these source repairs: plugin host shard `🎟️grant/🧪️tests/🟦️.ts` and `🎟️grant/🧬️schema/📦️batch.json`. The existing wire fixture and neutral batch conservation fixture were preserved.
