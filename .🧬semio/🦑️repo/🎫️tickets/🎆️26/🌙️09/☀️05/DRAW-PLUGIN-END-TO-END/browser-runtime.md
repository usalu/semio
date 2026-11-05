# Draw Browser Runtime Acceptance

Current status: no runtime-owned handles remain live. Correct long ShellSync handle 31634 and independent PDF-oracle handle 35419 were deliberately cancelled through exact owning handles on coordinator instruction to fund actual prerequisite validation; neither reached its test body. Full raw outputs remain under generated. The native path repair has observed red/green bounded diagnostic evidence, but its actual 18-case owning regression, actual 5-case PDF-oracle regression, activation/serve and both real folder-backed en/de browser journeys remain unproven. No live screenshots or exported browser artifacts have been produced. Fresh mandatory owning commands are recorded below; no new queue is started until prerequisite readiness is coordinated.

## Current Evidence

The existing interaction and PDF probes retained in this ticket record failures without failing their process. They also force Chromium's macOS Metal backend. They cannot establish a passing cross-platform acceptance result.

Added `@semio-tech/draw-js:browser-acceptance`, which invokes the browser journey through the existing repository owned-process boundary, with an issued five-minute resource control and its cancellation signal. It requires explicit `--serve`, `--locale` (`en` or `de`), and `--output` arguments. Its browser prerequisite is the existing `workspace:deps-browsers`; it passes the existing repository browser-cache path to the child. It uses the existing parity owner's cross-platform software WebGPU launch flags and the existing test-only Playwright, Ajv, Canvas and PDF.js dependencies.

The neutral scenario is `✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔣️.json`, validated against adjacent `📐️.json` by strict Ajv before any browser starts. Its journey requires the Draw ready beacon, the default example's nonempty document, a visible nonempty canvas, a rectangle pointer gesture committing one layer, undo and redo reaching their exact document counts, a downloaded PDF independently accepted by PDF.js with one page and actual path/image/text painting, and a reload preserving the changed layer count and a healthy Draw ready beacon. Page errors and error console messages fail acceptance. The output report and failure screenshot are written only to the explicitly supplied output directory.

## Current Source Contracts

- `✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs` declares `DRAWING_PLAY_SURFACE_ID = "drawing.play.composite"`; the renderer's `Canvas2dHost` exposes the node's actual `surfaceId` in `data-surface-id`. The old ticket probe's `window:drawing-composite` selector was stale and is not reused.
- `✏️editor/🛂️manifest/🦀️.rs` declares `mod+shift+z` for redo. The browser runner maps mod to Meta on macOS and Control elsewhere.
- The same manifest now gives `exportDocument` visible format/size/transparency arguments. Current `ShellHelpers/🪟️window-panes` routes actions with visible arguments into a staged form, so the old probe's row-click-only PDF export is also stale. The new runner opens the row, selects PDF using the current staged select control, then invokes the generated `.action.exportDocument.execute` control.
- React `ShellHost/🖼️canvas-footer/🟠️footer-items` sets the ready beacon to `pluginFilter` after session and example readiness. The runner explicitly selects `plugin=draw` and requires beacon `draw` at every step, including reload.
- `ShellHost/🟦️.tsx` resolves locale from locks/preferences/requested locale and finally `navigator.language`. Existing time-travel browser tests select their explicit locale using a fresh Playwright context's `locale`. This runner uses the same approach and checks the rendered localized status, avoiding an unsupported `locale` query assumption.
- Current subset `📚️examples/🦀️.rs` advertises exactly the `demo` document, explicitly excluding the editor-only `demo-session` command replay. `resolveBootExampleId` falls back to that first example; the ready beacon requires `initialExampleReady`. The runner requires the selected Demo label and a nonempty default document. Current `NavbarExampleSelect` includes the framework's blank option by default, and `ShellHost/🔎️command-bar/🔶️scope` leaves it enabled. Draw's `setActiveExample` command sanctions the empty id as a blank-document reset. The journey switches Demo → blank → Demo, checks zero/restored layer counts, then starts the rectangle/persistence checks. It adds no new example inventory.
- PDF.js globals follow the existing Draw PDF oracle's Canvas polyfill recipe. External parser objects stay private to the browser test oracle.

## Execution

- `repo tools exec --light node -- --check ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs` passed.
- Strict Ajv compilation and validation accepted the neutral browser fixture.
- Both browser JavaScript modules passed Node syntax checking. The installed PDF.js worker source confirms vector painting can be folded into `OPS.constructPath`; the oracle checks its embedded paint operator as well as direct painting operations.
- `repo run @semio-tech/framework-os-dev:activate-draw-react-prototype` was started in command session `63322`, then cancelled through that exact owned handle at the coordinator's instruction. It exited 1 while still at `door.capacity`, after 1772 seconds of critical host memory pressure. No producer or target body was admitted. No admission bypass was used.
- HTTP probes to `127.0.0.1:6064` and `127.0.0.1:6070` both reported no server. No live browser success or runtime defect has yet been established in this session.
- `repo dependencies lock --help` confirms the owner-scoped lock refresh interface. An initial `--scope s` attempt was cancelled before admission (session `22110`, exit 1) after confirming scopes address source owner paths. The corrected `repo dependencies lock --scope ✏️s` was also cancelled before admission (session `12056`, exit 1) at the coordinator's instruction after 200 seconds of critical host memory pressure (6144 MB requested). Neither command modified its lock. The Draw package explicitly declares its existing test-only Playwright, PDF.js and Canvas dependencies at the workspace ranges. The coordinator completed the exact three-line Draw workspace metadata update in `✏️s/bun.lock` and verified that its parsed devDependencies exactly match `package.json`; all three package resolutions already existed and remain unchanged. Full owning lock refresh verification remains pending. No raw install was used.

Once the normal activation and serve lane is ready, execute:

```text
repo run @semio-tech/draw-js:browser-acceptance -- --serve http://127.0.0.1:6064/ --locale en --output .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🗑️generated/browser-en
```

Repeat with explicit German locale and a separate output directory. This is an unverified live gate until those executions pass; syntax validation is not runtime proof. The reload step intentionally exposes missing persistence rather than silently treating an in-memory undo stack as a persisted artifact.

## Third Goal Turn Revalidation

Authoritative activation handle `63322` remains running at `door.capacity` after 1435 seconds, requesting 3311 MB. Admission still reports critical host memory pressure (1468 MB below its protected floor), six running reservations and two ahead. No component producer, preparation, activation, or browser server has run in this invocation. Fresh bounded HTTP probes again found no server on ports 6064 or 6070.

All presently grounded browser harness corrections are implemented: current canvas surface, supported explicit locale route, actual default Demo and blank/demo switching, current staged PDF action, meaningful PDF.js painting, issued cancellation, repository browser provision/cache, healthy reload beacon. Further source repairs would be speculative without admitting the real producers and observing a live failure. The en/de live journeys, default example load/switch, history, real export and persisted rectangle after reload remain outstanding. This is an external execution impasse, not a passing end-to-end result.

## Resume Commands

Activation handle `63322` is terminal (cancelled before admission, exit 1). Both lock attempts are terminal. No peer process was changed. The ticket remains open and the live gates remain unproven.

Resume the normal owner activation, then run the source-defined serve route in its own terminal. The serve route accepts the explicit specific catalog root through the existing script option; activation/catalog producers must finish first.

```text
repo run @semio-tech/framework-os-dev:activate-draw-react-prototype
repo 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts serve draw react prototype --catalog ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated
repo run @semio-tech/draw-js:browser-acceptance -- --serve http://127.0.0.1:6064/ --locale en --output .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🗑️generated/browser-en
repo run @semio-tech/draw-js:browser-acceptance -- --serve http://127.0.0.1:6064/ --locale de --output .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END/🗑️generated/browser-de
```

The acceptance commands provision browsers through their owning prerequisite and pass the repository browser cache to the child. Remove the ticket's `🗑️generated` output directory after inspecting/persisting the final evidence, when the overall ticket is completed. No generated browser outputs exist from this blocked invocation.

## Resumed Execution

The coordinator resumed the goal after host memory recovered. Activation command `repo run @semio-tech/framework-os-dev:activate-draw-react-prototype` is now admitted in handle `73072` with six jobs; the real source admission and bootstrap/parser scan are running. The earlier memory impasse describes the prior attempt and no longer blocks this invocation. The compiled guest/descriptor worker owns its shared component prerequisite and coordinates through the same repository producer. Live browser outcomes remain pending until activation and serve complete.

### Real Persistence Contract

Current native/demo sessions acquire the transient `shell.primary-session` owner; preview editing alone does not attach durable document storage. Current Draw preview can use the existing real sync folder UI: `s-sync-status` → `framework.sync.folder` → `framework.sync.folder.path` → Attach. `openSyncTarget` resolves the focused guest's document identity and opens it with the explicit `persistedLocalOnly` folder binding. The dev server's ordinary `/semio-backbone` writes the canonical `.semio/events.semio` event log. The device's existing local-folder config remembers the binding and offers the existing `s-folder-reconnect` user gesture after reload. The browser journey now uses that actual attachment and reconnection path, checks a saved acknowledgment and nonempty on-disk event log, and retains the rectangle after reconnection. It does not inject storage or expect an unbound preview to auto-persist. Its folder is `document` beneath the explicit ticket output directory; the longest expected persistence path is checked against 256 characters.

The ordinary folder transport and canonical-bootstrap wire bridge serialize `folder://${binding.path}` into an encoded HTTP query parameter. `backbone-storage` retrieves the opaque native path with an exact tag-prefix slice and passes it to native filesystem operations. It performs no WHATWG URI authority or pathname decoding. Current SyncAttachTarget and neutral backbone builders incorrectly prepend `/` to drive-qualified Windows paths, corrupting this shared native-path contract. This is a confirmed source integration defect independently of live execution, rather than evidence of a live Draw failure.

Schema and platform vectors were added before implementing the repair to the already registered `ShellSync/🧪️tests/🧩️component/🟦️.tsx` suite. They cover forward/backslash drive paths, drive root, forward/backslash UNC roots, POSIX, spaces, Unicode and literal percent sequences. Existing Node `path.win32`/`path.posix` dirname supplies the independent path oracle; transport query encoding is checked separately for exact recovery. Shared framework ancestors `🧰️framework/🛍️products/AGENTS.md` and `🧰️framework/🛍️products/💻️os/AGENTS.md` were read before edits. The new schema/fixture absolute paths are 151 UTF-16 code units.

Initial regression attempt: `repo run @semio-tech/framework-renderer-react:test -- quick "--testNamePattern=sync attach target"`, handle `4232`. It reached the same shared dependency bootstrap wait as activation and never executed a test body before its terminal failure below. Later source inspection also established that this attempt's `quick` level would omit the ShellSync suite; the fresh corrected command is recorded below. The initial schema-first stage held implementation for that result; the later bounded actual-builder diagnostic independently established the defect before the repair. The current blocker is shared dependency preparation, not the earlier host memory shortage.

At the coordinator's instruction, a separate normal registered-tool diagnostic was started in handle `49409`: `repo tools exec bun -- -e` imports the actual existing `buildFolderBackboneUri`, calls it for `C:/work/Zeichnung ä 50%`, and prints its payload after the `folder://` tag alongside the exact expected native path and Node `path.win32.normalize` observations. It was not a test runner or a replacement for the owning regression. That attempt waited for ordinary machine CPU admission and was subsequently cancelled as recorded below.

### Native Path Diagnostic and Repair

The coordinator authorized a bounded light read-only builder observation after verifying import purity. A TypeScript AST inspection found 18 local-only modules in the existing builder's runtime import closure, no bare runtime imports, and only bounded codec/hash table initialization, TextEncoder/TextDecoder, immutable constants and local collections. The `import.meta.vitest` branch is inactive in an ordinary `bun -e` invocation; no test framework, process, server or installer starts. The ordinary queued diagnostic `49409` was cancelled before admission, exit 1; it was not retried as a test under light mode.

The actual existing builder was then evaluated through `repo tools exec --light bun -- -e`. Before the repair it returned `folder:///C:/work/Zeichnung ä 50%`, whose opaque payload was `/C:/work/Zeichnung ä 50%` rather than the native `C:/work/Zeichnung ä 50%`. Node `path.win32.normalize` independently observed the corrupt native result `\\C:\\work\\Zeichnung ä 50%` versus the expected `C:\\work\\Zeichnung ä 50%`. This is observed failing diagnostic behavior, distinct from an owning regression result.

After the already-authored schema and regression vectors, the narrow repair introduced one neutral native-path helper and used it in both tag builders and the sync target UI. Current Rust WGPU sync attachment already uses `format!("folder://{path}")` / `format!("file://{path}")`, and its native receivers strip those exact tags into native paths; they do not contain the erroneous slash insertion. The ordinary TS worker folder writers and bootstrap bridge likewise serialize the native binding path directly. Those correct implementations were left unchanged.

Exact shared source files:

- `🧰️framework/🔨️modules/🛰️backbone/🛣️path/🟦️.ts`: pure `absoluteBackbonePathV1` and `backbonePathDirectoryV1`, preserving POSIX, drive and UNC native roots and literal characters.
- `🧰️framework/🔨️modules/🛰️backbone/🟦️.ts`: existing file/folder tag builders consume the helper and reexport it.
- `🧰️framework/📦️packages/🟦️typescript/🛰️backbone/🟦️.ts`: explicitly exports the two own helper functions through `@semio-tech/framework/backbone`; no external types are exposed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🟦️.tsx`: existing typed sync target consumes the same native path and parent helper.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🧪️tests/🧫️fixtures/🎯️sync-attach-target.json`: platform-neutral vectors authored before implementation.
- Adjacent `📐️sync-attach-target.json`: new corpus JSON schema, accepted by strict Ajv.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🧪️tests/🧩️component/🟦️.tsx`: existing registered suite validates the schema, native platform parent oracle, literal tag payload and independent URL query roundtrip.

Post-repair light diagnostic of the actual builder returned exact native payloads for `C:/work/Zeichnung ä 50%`, `\\\\server\\share\\Zeichnung ä %20.semio`, and `C:\\work\\document.semio`, with `preserved: true` in all three observations. The own parent helper's results exactly matched Node `path.win32.dirname` for those observations. Literal `%20` was not decoded. This is passing bounded diagnostic evidence only; owning regression handle `4232`, activation `73072` and the actual en/de browser journeys remain required and unproven while shared preparation advances.

### Owning Regression Attempt Ended Before Tests

Handle `4232` is now terminal, exit 1. Its last observed activity was `dependencies:current-input`; no graph, Vitest suite or sync path test body ran. A subsequent handle lookup confirmed that the process is closed. The polling summary accidentally discarded the first failure-cause line while retaining its final owned-process failure tail; the precise bootstrap cause is therefore unavailable from this attempt and is not inferred from the separate compiled-guest worker's failure. This failed bootstrap is not a red path assertion or a passing regression. Future polls retain their full raw tool result before filtering display output. The owning path regression must be run fresh after the coordinator's shared dependency repair.

Activation `73072` remains alive, waiting on the shared dependency build (last observed wait 3031 seconds), without running its target body. The compiled-guest worker separately reported its own preparation failure; the IO worker now owns the next finite shared preparation attempt. No activation timeout/restart or peer-process mutation was performed here.

The coordinator subsequently instructed cancellation of only idle activation `73072` to release its six CPU reservations for the real normal-capacity dependency regression. That handle was cancelled through its own input channel and exited 1, still waiting for shared dependency preparation after 3103 seconds of wait / approximately 3220 seconds total. No activation or producer target body had run. The complete terminal tool result was retained in session storage before summarization. This was explicit release of a known idle reservation for prerequisite repair, not a timeout restart or a target failure. Both runtime-owned handles are now terminal; the agent remains active for fresh owning path regression, activation and the full durable-folder browser journeys after repair. No peer process was changed.

### Correct Owning Regression Level

Current React renderer test configuration selects only `quickTestSuite` at `fundamental` or `quick`; the registered ShellSync component suite belongs to `engineTestSuites`, selected at `long`. Therefore the required fresh owning command after shared dependency repair is:

```text
repo run @semio-tech/framework-renderer-react:test -- long 🔄️ShellSync "--testNamePattern=sync attach target"
```

The existing owning TestScript forwards positional filters to Vitest, so the unique `🔄️ShellSync` source-path filter focuses the registered suite instead of importing every long renderer suite. The earlier quick attempt never reached tests and is not coverage evidence. The corrected command must report actual schema/native-path test assertions from the ShellSync suite before any pass can be claimed. No test-level/source import changed in the worktree during this correction.

### Fresh Corrected Owning Attempt

After the execution owner reported seven genuine normal-capacity native prerequisite cases green with unchanged depth/work limits, the corrected command was started in fresh handle `31634`:

```text
repo run @semio-tech/framework-renderer-react:test --parallel=1 -- long 🔄️ShellSync "--testNamePattern=sync attach target"
```

Its full stdout/stderr is retained at `🗑️generated/path-regression.log`. Initial log evidence shows ordinary `door.capacity` admission wait (3323 MB requested; host memory below the protected floor, nine running reservations and three ahead). No body or assertion has executed yet. No stale handle, admission bypass or underlying test runner was used.

The same handle was subsequently admitted at one job / 3323 MB after 178 seconds of ordinary capacity wait. Its real source metadata/frontier scan reached 279360 observed operations, then entered `build:dependencies`. Current evidence is shared dependency-producer wait (260 seconds at the latest revalidation, approximately 740 seconds total). No renderer target body, ShellSync test assertion or browser has run yet. This is not evidence that the shared production pipeline has consumed the seven-case repair successfully.

### Immediate Browser Execution Readiness Recheck

Revalidated original handle 31634: live shared build:dependencies wait, no actual named ShellSync body or result. Raw output remains generated/path-regression.log. No restart. The current ServeScript still accepts serve draw react prototype, reads the existing activation receipt, and uses the declared Draw React port via the catalog, with an explicit catalog flag consumed by the routing entry. Browser owner still uses issued cancellation control and the repository Playwright browser cache. Read-only module resolution from the actual browser .mjs entry resolved Playwright 1.64.1, PDF.js 5.7.284, @napi-rs/canvas 1.0.11 and Ajv 8.20.0 in the current workspace. This verifies module presence only; browser installation, native canvas loading and actual UI remain unproven. The planned absolute folder database path is 143 UTF-16 characters, within the 256-character limit. No additional grounded setup defects found. Retain the existing owning regression, then activation/serve and both locale real-folder journeys.

### ShellSync Windows Reachability Audit

The current renderer project test target has no platform restriction. Its TestScript resolves the explicit long level and forwards remaining positional/name filters to runVitest. The actual config under renderer/engine/targets/react includes elementSuite(🔄️ShellSync, component, tsx) in engineTestSuites; long chooses that list and normalizes backslashes to forward slashes. No platform gating occurs in the config or the ShellSync component suite. The named describe is 🎯️ sync attach target and contains 17 corpus cases plus the schema law, so the expected focused nonzero count is 18. Windows and POSIX fixture platform fields select Node path oracle semantics; they do not skip host execution. This is structural reachability evidence only; exact handle 31634 has not reached a test body.

### Browser Proof Strengthening After Review

Authored neutral schema/fixture requirements first for minimum current event-log growth and changed rendered PDF pixels. The browser gate now refuses an existing document folder, records fresh attached log bytes before the gesture, and requires later saved rectangle/history events to increase them. It downloads the pre-gesture Demo PDF and edited PDF through the same real UI; the existing independent PDF.js plus native canvas oracle renders each and returns only a SHA-256 pixel digest. Equal rendered images fail even when both contain Demo painting. No runtime dependencies were added. Test-only worker observers now report error/unhandledrejection to the already monitored console and fail if observer installation rejects; observer completion is awaited at each note. Source grounding: shard request-transport forwards worker-fault to onActorTrap while current React runtime-singletons initial hook is a no-op, so relying exclusively on page errors was insufficiently grounded. Worker initialization before observer installation remains covered by readiness/fault checks and required successful real actions, rather than claimed exhaustive fault injection. Node syntax checks passed; actual owning browser execution remains pending shared dependencies.

The strengthened PDF oracle additionally requires positive rendered dimensions and visible nonwhite pixels in both files, and reports the actual nonzero changed pixel count plus dimensions/digests. Neutral schema/fixture now declares guest fault kinds worker-fault/trap. A pre-navigation test-only native Worker subclass observes those actual protocol messages, error and messageerror without replacing messages or application handlers; this closes the deliberate trap-message gap left by onActorTrap no-op. Per-worker error/unhandledrejection observation remains additional evidence. No shared runtime mutation was necessary. Syntax and strict schema validation pass; actual browser gate remains pending, and failed/repeated journeys must choose a new generated output directory because document freshness is required.

### Owning PDF Oracle Regression

Added schema-first neutral oracle corpus (browser/🔮️cases.json and 📐️oracle.json) for same PDF, metadata-only change, edited rectangle, empty page and malformed document. Browser/🔮️check.mjs constructs minimal standards PDF inputs, while independent PDF.js/native canvas provides actual parsing/rasterization; same raster must refuse the changed-pixel assertion and edited geometry must pass it. New live browser-oracle-check target routes through the same issued BrowserAcceptanceScript owned child control without launching Chromium or adding dependencies. Exact normal owning command: repo run @semio-tech/draw-js:browser-oracle-check --parallel=1. Handle 35419 is currently capacity queued, raw output generated/pdf-oracle.log. Syntax only passed; no actual case/result yet. Original long ShellSync handle 31634 remains live; no restart or peer changes.

PDF oracle handle 35419 was directly revalidated live after source frontier496640 and build:dependencies entry; no oracle case body occurred. On parent instruction, cancelled only this idle owning reservation with its exact handle to fund actual prerequisite validation. Terminal exit1 after about380s repository command/~470s total; full raw output remains generated/pdf-oracle.log. This is a deliberate cancellation, not test failure/pass. Mandatory fresh owning command remains repo run @semio-tech/draw-js:browser-oracle-check --parallel=1 once shared prerequisites are ready. Original long ShellSync31634 remains retained.

Original long ShellSync handle 31634 was directly revalidated live/no test body, then cancelled only by exact owning handle on parent instruction to release an idle CPU reservation for actual portable prerequisite fixture/strict validation. The reason is prerequisite funding and a fresh-source owning rerun afterward, not elapsed age. Terminal exit1; no path-case green or red can be claimed. Full raw output remains generated/path-regression.log. Mandatory fresh owning command: repo run @semio-tech/framework-renderer-react:test --parallel=1 -- long 🔄️ShellSync "--testNamePattern=sync attach target". Expected actual focused count18. Do not enqueue activation/browser until prerequisite validation is funded and shared dependencies are ready. Both owning PDF oracle and ShellSync must be freshly rerun before real en/de journeys. No own live handles remain; peers untouched.

Oracle reviewer follow-up: strict neutral schema now requires all five unique named relation cases with exactly five total; replacing every row with same-PDF is refused. The actual check also requires metadata-only source PDF bytes to differ before comparing their identical raster, and requires same-PDF source bytes to agree. Read-only strict schema preflight accepted the real fixture and refused duplicated same-PDF corpus; actual owning case execution is still pending the fresh mandatory route.

### Browser Binary and Native Canvas Preflight

Read-only/no-launch check from the actual browser module: installed Playwright1.64.1 expects Chromium1248. The inherited PLAYWRIGHT_BROWSERS_PATH is C:/git/semio/node_modules/.cache/ms-playwright; its chrome.exe and chromium_headless_shell-1248/chrome-headless-shell-win64/chrome-headless-shell.exe exist (headless216950272bytes), with installation/dependency markers. The registry-owned default .🧬semio/🦑️repo/⚡️cache/tools/ms-playwright currently has revision1234 only, so expected1248 is absent there if no override is inherited. This is a concrete stale default-cache prerequisite, not a proven failure of the current inherited environment. The declared browser-acceptance target already depends on workspace:deps-browsers; that zero-touch owner routes native dependency sync browsers after deps-js-all, so it must refresh the managed browser revision before body execution. No direct install or host mutation was attempted. Native @napi-rs/canvas-win32-x64-msvc1.0.11 binding resolves and its skia.win32-x64-msvc.node file exists (27456512bytes). Presence does not prove native loading/rendering; the mandatory owning oracle gate supplies that proof.

## Exact Runtime-Owned Source Inventory

All paths exist and their absolute path lengths are within256characters. Shared Draw project input widening was coordinated with IO; lock metadata belongs root. Include these paths in eventual combined ticket closure after all mandatory real gates pass.

- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/📐️.json (64)
- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/📐️oracle.json (70)
- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔣️.json (64)
- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️cases.json (69)
- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️check.mjs (68)
- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️oracle.mjs (69)
- ✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs (63)
- ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/package.json (74)
- ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📋️project.json (77)
- ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts (74)
- 🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts (86)
- 🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json (98)
- 🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts (85)
- 🧰️framework/🔨️modules/🏃️process/📋️project.json (63)
- 🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🔣️.json (77)
- 🧰️framework/🔨️modules/🛰️backbone/🛣️path/🟦️.ts (63)
- 🧰️framework/🔨️modules/🛰️backbone/🟦️.ts (55)
- 🧰️framework/📦️packages/🟦️typescript/🛰️backbone/🟦️.ts (70)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🟦️.tsx (111)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🧪️tests/🧩️component/🟦️.tsx (133)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🧪️tests/🧫️fixtures/🎯️sync-attach-target.json (151)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🧪️tests/🧫️fixtures/📐️sync-attach-target.json (151)
