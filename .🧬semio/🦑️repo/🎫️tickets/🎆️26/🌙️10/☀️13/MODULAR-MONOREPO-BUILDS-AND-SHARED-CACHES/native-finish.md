# Native Execution Finish

## Current Evidence

- Inactive private compiler outputs were reclaimed under the current cache-prune and matching Cargo profile leases after process/environment/open-FD checks. Actual removal: 37,968 files, 30,037,753,825 bytes; original private inputs, plans, audit Markdown, provenance and logs remain. Current canonical shared Cargo was retained. See `private-cache-retention.md` for exact scope and proof. Optional old private warm resume now rebuilds those discarded intermediates.
- Current native and Go factory adapters passed the actual Nx verification. The concurrent capacity fixture initially exposed a shared fixture-root lease deadlock; its current source creates a private invocation root. The isolated current capacity gate passed 4 tests / 32 assertions / 1.07 s. Current WASM factory adapter gate passed 10 tests / 102 assertions / 3.43 s. Actual logs: `native-factory-current-green.log`, `capacity-private-current-2.log`, `wasm-factory-current.log`.
- WGPU native exact prototype/release output/cache/runtime fixture passed (`wgpu-mode-native-2.log`). The stable authored `native-build` target's configurations select actual native profile outputs; no target suffix aliases remain in these fixtures.
- WGPU browser gate passed against the actual bootstrap resolver and original invocation (`wgpu-mode-browser-3.log`). This physically exposed and then verified the removal of stale public dev suffix matching, plus current React mount mode paths. Lifecycle command `dev` remains a command, not a compiler mode.
- WGPU WASM complete fixture passed with actual Nx exit 0 (`wgpu-mode-wasm-16.log`). It runs both exact profiles, cold/warm/restored output paths, changed source values, independent Cargo/wasm-bindgen runtime output, environment sanitization, and deliberate failing compiler output refusal. Actual transformed values are printed by the gate. Prototype explicitly uses Trunk `data-cargo-profile="prototype"`, no optimization; release uses `release` and Binaryen z.
- Immutable compiler observation output schema/path fixture passed 1 test / 10 assertions / 73 ms with Ajv and pathe independent oracles (`compiler-observation-current-2.log`). The first three attempts used Nx exec-relative Bun test paths and found no test; these are runner misses, not physical feature-red evidence. The earlier actual Trunk gate rejected the unadmitted observation output guard before this explicit contract.
- Strict native component mode feature had actual rustc red E0425 before generated lookup implementation, then green 1 test / 624 ms, Vitest 3.55 s, Nx 5.1 s (`component-mode-red-2.log`, `component-mode-green.log`). Both selected modes appear in actual rustc env-dep witnesses, independent pathe/runtime paths match, four invalid modes are refused, and emitted source stays byte-neutral. Actual registry regeneration passed 32.5 s (`component-mode-generate.log`).
- Current OS kernel actual prototype build passed Nx exit 0 / 1 minute 4 seconds (`native-kernel-current-build.log`), after direct original-context/result close integration. Semantic compiler test profiles remain distinct from public modes.

## Trunk Compiler Custody

The actual Cargo artifact query retains one exact successful original invocation with unchanged manifest, selected target, argv, compiler configuration input and raw hash. Its staged artifact hash equals the raw Trunk hash. A compiler observation publication is explicitly admitted only below the policy compiler store's `semio-cargo-observations` namespace, only into a fresh directory; ordinary source deliverables retain their existing source-root guard. Original receipts and every staged compiler deliverable survive private target capture removal. No invocation or received receipt is fabricated.

The earlier query passed the automatically loaded repo Cargo config again as `--config`; its observed raw bytes differed. The current query uses Trunk's actual config working directory and the same automatic config loading, and requires the root config's original retained invocation-input hash. Runtime hash binding remains strict. Installed Trunk 0.21.14 ownership was checked against its primary source: https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/pipelines/rust/mod.rs (Cargo working directory and transformation ownership).

The production compiler uses original admission control for capacity, profile lease, finite child execution, publication and continuation. Its owned deadline cancellation remains separate from the original incoming signal. Publication receives the exact original control signal instead of a composed child signal. Invalid inherited mode contradictions fail before compilation.

## Component Mode Consumption

The generated Rust registry emits a strict `plugin_component_directory(mode)` helper containing prototype and release paths and refusing every other value. The headless run and MCP component resolvers consume `env!("SEMIO_BUILD_MODE")`, so rustc tracks the original selected mode rather than silently reading existing release bytes. Existing registry projection test assertions were adapted to this helper; their broader test revealed a stale neutral fixture path and external temporary-output path, now corrected. The broad projection gate currently has a Node Cargo storage JSON import-attribute blocker owned by the Nx sibling; current narrow mode and actual generated artifact gates are green.

## Current Native Protocol Scope

`native-compiler-integration.md` groups actual full prototype4 diagnostics by source owner. Current source recheck found three old blockers already repaired by concurrent contributors (actor POD retirement, copied call failure and payload UTF8 ownership). Those sources were left untouched.

This agent changed UI flex and mounted-layout private close helpers to Result and the actual mounted-layout job to the original StepContext close contract, preserving primary/receipt failure ownership and one retained debit. The rejected prepared input now takes an explicit grant and returns Result. OS owned-schema close/release/step propagates OriginalTurnFailure under the same original context rather than reading `.progress()` from Result or retaining removed CloseStep::Refused variants. Current OS kernel library compilation is green. UI engine→Interpreter/Shell original context transfer and neutral caller fixtures remain in progress; no UI engine compile success is claimed yet. Finite original-turn and original-close native laws are running with an authored 600,000 ms original orchestrator envelope.

## Changed Source Inventory For This Finish

Workspace-relative paths (excluding unrelated contributor changes and generated output):

- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: ticket prune + WGPU three mode gates + strict component mode acceptance.
- Ticket `private-reclaim/📜️script.ts`, `private-cache-retention.md`, `wgpu-mode/📜️script.ts`, this report and `native-compiler-integration.md`.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts`: private fixture roots.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`, its new `📦️observation/🧬️schema/🔣️.json`, `📦️observation/🧪️tests/🟦️.ts`, `📦️observation/🧪️tests/🧫️fixtures/🔣️.json`, and both public TypeScript native-build facades: explicit observation output ownership and exports.
- WGPU `🏗️compiler/🦀️native/📜️script.ts`, `🏗️compiler/🌐️wasm/📜️script.ts`, server TS/config/script, native entrypoint script and module/catalog schemas/tests, TS package script: exact modes, stable config ownership, original controls and actual mount/output paths.
- Repo caching fixture owners `🧊️native-renderer-outputs`, `🧊️wasm-outputs`, `🧊️browser-serving` (TS and neutral vectors), native-runtime initial public-mode vector adaptation, `cache-contracts/🟦️.ts` current caller controls.
- Repo runtime discovery schema TS/JSON, selection JSON, tests TS, verification TS stable wasm configuration lookup; native-host TS public mode signature (Nx owns mjs/declarations/metadata).
- GIS Rust `📜️script.ts`: strict public mode/compiler environment contradiction, canonical Cargo mode output.
- Hub WGPU renderer test script/collaboration TS: exact mode unions and React selection.
- OS plugin registry projection TS; strict component vectors/test; generated-projection test assertions/fixture path/ticket output root. Headless bootstrap Rust and MCP workspace Rust strict component lookup consumption.
- UI WGPU `📐️flex/🦀️.rs`, `📌️mounted_layout/🦀️.rs`, `🎟️prepared/🦀️.rs`; OS `🏪️store/📦️codec/🧬️owned-schema/🦀️.rs`: current close protocol integration. Current UI engine caller work is not yet in this inventory.

No AGENTS file, modifying Git operation, worktree, external process interruption or ticket closure was performed. Parent owns full workspace verification and completion.

## Later Current-Source Qualification

- Current finite original-turn failure source/type/native gate passed: source 3 tests with independent JSON Patch and complete Value/Fault/paired receipt custody checks; Nx native target 5.3 s / task 4.6 s (`native-original-turn-current-4.log`). The exact three defining native laws executed successfully.
- Current finite original-close source custody native law passed: Nx 3.1 s / task 2.7 s (`native-original-close-current-3.log`). Both runs used original `SEMIO_ORCHESTRATOR_BUDGET_MS=600000` and explicit ticket output input, without a cloned production envelope.
- The first actual native UI wgpu-engine build had one current removed `checked_out_error` API error. Current borrower uses `checked_out_failure` and retains the complete actual primary and paired receipt refusal in its original session; the borrowed public retirement accessor exposes that session owner as well as controlled close failures. Fresh actual native UI build passed Nx 25.6 s / task 25.1 s (`native-ui-current-build-2.log`).
- Concurrent job fixture work introduced and adopted the persistent `OriginalJobFixtureCaller`: explicit source admission, original identity/clock/cancellation/payload authority, measured birth and paid terminal retirement. Provisional local turn wrappers were fully removed to retain this stronger source. `native-job-fixture-integration.md` is the historical actual red inventory, not an outstanding current native-law failure claim.
- Actual WGPU prototype crate compilation ran 3 minutes 7 seconds and found four newer kernel history storage edges (`wgpu-prototype-current-build.log`). Current source already corrects the HistoryEdits namespace and the flush collection type (three errors). The retained config source still received a PagedList in its Vec source iterator. No Vec collection or whole-list reversal fallback was added; its ordered paged source must retain paid preparation and page retirement.
- Current renderer production edits remove Interpreter's fabricated document/layout contexts and pass the existing original frame context through eight Shell render methods. The web sibling owns remaining renderer FrameBuild/neutral test callers. These edits still need actual full WGPU source qualification after the kernel paged source blocker.

Additional exact touched paths for this later phase:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`: checked-out full failure borrow and session-owned failure accessor; other ongoing close changes were preserved.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`: direct original document/layout context.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`: original context through the eight private render methods.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📪️outcome/🟦️.ts`: standard Node JSON import attribute exposed by actual registry projection test.
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/📏️close-demand/🦀️.rs`: actual failed allocation receipt is paired OriginalTurnFailure under a declared test context; blocked/complete receipts retain current enum shape.
- `🧰️framework/🔨️modules/🧵️job/♻️retirement/👷️authority/🧪️tests/🦀️.rs`: one initial original-context drain; subsequent concurrent persistent caller implementation retained.

The ticket observation acceptance command is now also registered in both launch files through `wgpu-mode/📜️script.ts observation`.
