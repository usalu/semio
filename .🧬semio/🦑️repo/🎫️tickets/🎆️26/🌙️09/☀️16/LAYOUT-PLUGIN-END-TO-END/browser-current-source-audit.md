# Browser Current Source Audit

2026-10-29. Read-only source audit. No activation, server, browser, tests, compiler, cache operation, or live readiness observation performed. Source and journey remain unchanged. Applicable root, products, OS, s, Layout AGENTS and build skill read. Ticket remains owned by parent.

## Blocking Current Renderer Mismatch

The authored journey is a React DOM journey. `browser-journey.mjs:55-84` requires visible panel/window/mode-dock-tab DOM slots and descendant surface canvases. `:146-202` requires window-action-pane, action rows, select-trigger/listbox and execute DOM controls. `:240-258` requires literal inspector row IDs and textarea descendants. These cannot be assumed to describe the current WGPU renderer.

Current WGPU browser host (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts:63-70,277-292`) creates one `semio-wgpu-canvas`, replaces root children with it, and transfers it to OffscreenCanvas. Its ready callback (`:334-345`) publishes the beacon, creates its accessibility mirror and wires canvas events. Thus `ready=layout` may remain meaningful while the journey still cannot find any owner dock tab or window-contained canvas, so boot poll itself is stale for this renderer.

WGPU accessibility mirror (`same target/♿️accessibility-mirror/🟦️.ts:76-81,113-115,237-243`) is clipped to one pixel, uses data-window/data-node-id and generated mirror IDs. It does not reproduce the journey data-slot/window ID hierarchy. Replacing selectors with this mirror without deriving current action delivery and visible canvas ownership would not establish the requested visual UI journey. Keep this as an explicit preflight failure, not a fake-ready success.

The concrete WGPU project manifest `same target/📦️packages/🟦️typescript/📋️project.json` names `@semio-tech/framework-renderer-wgpu`. Its current targets contain no serve/dev target. The older dev project manifest is separately at `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`; activation/serve names are generated rather than literal matches. This audit does not establish a replacement declared route. Parent must resolve current composition/owner metadata before launch; previous reports asserting the React6079 door are not current-route evidence.

## Layout Domain Contract Still Matches

Below paths are relative to `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

- `✏️editor/🧱️create-layout-app/🦀️.rs:38-51` still declares addFrame/addPage, rect/text/image kind select, x/y and exportPng/exportSvg/exportPdf. Journey contract rect251,337 is valid domain input.
- `✏️editor/🎮️commands/🖼️add-frame/🦀️.rs:26-28,50-62` creates frame ID from current page frame count, rect120x64 with fill[0.2,0.24,0.3,1]. The144pixel patch274,354 is strictly inside the authored rect and expected8-bit color51,61,77 matches rounding. Journey correctly derives actual introduced ID rather than hardcoding frame-4.
- `🚪️io/📝️text/📸️snapshot/🦀️.rs:114-158` still defines story-1 Hello layout,400x500 pages page-1/page-2, and the required stock frame IDs.
- `✏️editor/📌️panels/🗿️artifact/🦀️.rs:78` still generates layout-document.page.<id>.
- `✏️editor/📌️panels/🔍️inspection/🦀️.rs:217-229,462-467` still binds patchDocument text commit on blur and generates global.story.story-1.content. This validates domain row keys, not a guarantee of literal current browser DOM IDs.

## Harness Evidence And Remaining Limits

Exact frame/page set deltas, immediate undo/redo with Artifact mounted, keyboard Execute focus and blur story commit are substantive checks in the authored React harness. Their current native event delivery is pending.

`media-oracles.mjs` independently uses jsdom, sharp and pdfjs. SVG/PNG compare baseline and edited144pixel geometry, exact400x500 dimensions, fill tolerance3 and95percent changed pixels. PDF checks page count/dimensions/edited text; journey additionally rejects stock text. This is credible format evidence if real downloads occur. Canvas variance proves some painting only, not complete layout or readable contrast. No text/background contrast ratio assertion exists; current health helper also accepts missing status, consistent with optional channels, so it cannot alone prove a running surface.

Archive export precedes original-only page and marker mutations (`browser-journey.mjs:330-346`); import then demands one new Blueprint owner, saved exact IDs, saved marker and exclusion of both later changes (`:348-372`). This is strong separate-owner restoration logic, pending current renderer action delivery. It proves no restart, reload or multi-user persistence.

No fake-ready writes, internal action dispatch, synthetic catalog credits or direct Vite bypass occur in the journey. Runtime conclusions remain pending. The existing ignored empty-pack console diagnostic is explicit. Browser tool execution must remain ordinary managed Bun, never node or --light; derive the current declared owner route first.


## Independent Version 3 Recheck

2026-10-29. Current authored WGPU changes re-read independently; no code/check/browser/launch executed. This section supersedes the earlier React-only preflight finding for the rewritten harness, while preserving its historical evidence.

Current inference owner source `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs:241-255` now proves genuine serve/dev-layout-wgpu-prototype targets depend on activate-layout-wgpu-prototype; prepare includes finite WGPU wasm and browser/frame/renderer boot producers. The server command includes the catalog port and SEMIO_WGPU_COMPOSITION_PATH. `✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts` supplies the specific generated catalog and completed WGPU configuration. Server script receives Script Original at its main router and passes `this.invocation` and compiler admission derived from that invocation to serveVite. This proves authored propagation at this boundary; deeper launchOwner/sameActualOriginal implementation was not independently re-audited here.

The new singleton canvas and mirror selectors now follow current browser-host and accessibility-mirror source. Mirror :113-116 publishes data-window/data-node-key, :160-180 maps supported focus/blur/activation events to lossless native transport, :208-210 maps input value events, and :237-251 rebuilds nesting from projected depth. The harness uses public DOM interactions; introspection is read-only dumpStructure/dumpFrameStats for contrast. No fake readiness, direct action dispatch, synthetic input catalog, editor API or internal mutation bypass was found.

### Concrete Remaining Race

stageAction immediately asserts aria-expanded after Enter opens the combobox, then relies on fixed200ms sleeps for option close and Escape. Current accessibility-mirror :4 has ACCESSIBILITY_REFRESH_FLOOR_MS=400, and activation merely enqueues lossless work (:166-180); the mirror updates asynchronously through introspection and replaceChildren (:230-260). Therefore old aria-expanded/selected attributes can still be present at these assertions even when the renderer behaves correctly. Poll the fresh trigger/option attributes to the desired value after each native event, including second-open option selection verification. This is a source-demonstrable harness race, not a claim of observed runtime failure. The250ms story delay is also below the refresh floor; later PDF/archive marker checks provide stronger actual-commit evidence than the immediate unchanged DOM draft value.

### Preserved Evidence And Limits

Version3 retains exact create/undo/redo, independent SVG/PNG144pixel baseline geometry, PDF edited text with stock absence, archive-before-divergence, distinct imported owner and exclusion of later original-only page/marker. New contrast oracle ties structure and frame to the same mirror window/key and actual current input text, requires nonempty visible enabled input/bounds, glyphs, opaque linear foreground/background and4.5:1 luminance ratio. It reads renderer projections rather than merely DOM CSS. Runtime freshness between these separate introspection reads is not a transaction guarantee, and actual glyph placement/readability still needs real execution and screenshot review. The harness does not explicitly assert that contrastLabel equals251 at the witness collection point; later geometry tests still enforce the authored rect placement.

Health retains boot/error/alert/console checks and now checks accessible names. Missing optional host status remains accepted and singleton canvas variance alone remains limited painting evidence. Blueprint owner suffix/panel surface IDs and all native event deliveries remain live acceptance items. No current runtime, actual contrast, download, archive import or end-to-end success is claimed.


## Final Asynchronous Mirror Recheck

2026-10-29. Focused source-only recheck; no runtime, launch, test or build executed by this reviewer.

The previously identified Select projection race is closed in authored source. `browser-journey.mjs:166-183` now re-resolves trigger/option locators and polls fresh native projection state after opening, option activation, reopening with selected=true, and Escape. `browser-projection.mjs` repeatedly invokes its supplied reader until the condition succeeds or the15second deadline fails;50ms is a polling interval rather than a presumed render delay. The old150/200ms staging sleeps are absent. Delayed replacement fixtures explicitly cover450/850/600ms and no-publication timeout; execution results remain those recorded by the owning checker, not independently rerun here.

`browser-journey.mjs:234-244` now retains the old story element only as a replacement witness, polls for its disconnection plus the fresh textarea marker value and lost focus, and disposes that handle. This removes the earlier250ms unchanged-draft shortcut. Later independent PDF and archive marker checks remain necessary semantic evidence and are retained. `:274-284` explicitly requires actual X input value251 before matching the same key/window against renderer-owned structure and frame contrast data. The earlier missing witness-value assertion is closed.

No additional concrete defect was found in this focused replacement/polling change. This conclusion closes the two stated authored-source findings only. Native activation/event delivery, rendered contrast, media and archive assertions, selector identities and complete UI acceptance still require the root-owned actual WGPU journey. No readiness, actual UI success or end-to-end pass is claimed.


## Retained Launch Input Defect

2026-10-29. Read-only cross-platform/current-selector pass; no browser/test execution.

`launch-layout.py:15-16` still hardcodes activate-layout-react-prototype and serve-layout-react-prototype. Its probe invokes the current WGPU6179 journey. Thus the documented convenience activate/serve/probe sequence launches a different renderer/listener than the probe requires. Update both target names to current activate-layout-wgpu-prototype/serve-layout-wgpu-prototype (ideally sourced from browser-journey.json renderer contract), preserving managed repo execution/backend forwarding. The genuine direct WGPU owner commands remain correctly declared; this is a stale retained ticket input, not a need for a new target or compatibility shim.

Bounded accompanying inspection found no developer absolute path in the six executable journey/helper/launcher inputs, no hardcoded Chromium executable, and13required contract steps matching13record call sites. repoToolCacheEnv resolves PLAYWRIGHT_BROWSERS_PATH before dynamic Playwright import and preserves an explicit caller override. These checks introduce no further actionable defect and are not runtime results.


## Caller Output Path Bound

2026-10-29. Current browser-journey.mjs sanitizes SEMIO_PROBE_OUT characters but imposes no length cap before constructing outDir and calling mkdirSync. A sufficiently long caller value therefore creates artifact paths above the repository256 UTF-16 path limit on every platform. Add a full absolute output-path bound check (including the longest planned screenshot/download child) or a run-name bound derived from the actual resolved ticket path before creating outputs/launching Chromium. Count JavaScript string UTF-16 units, not bytes, and account for different checkout roots. Default paths inspected here fit: ticket root97units; longest declared default step screenshot156units. The defect concerns the explicitly supported caller override.

All five local journey imports currently resolve to existing files. Managed repoToolCacheEnv browser discovery remains before dynamic Playwright import; no developer absolute executable is selected. Contract still requires13steps and current source has13record calls. Media oracles retain independent jsdom/sharp/pdfjs validation and edited geometry/text assertions. No additional actionable import/parser/step-inclusion defect was identified, and no runtime result is claimed.


## Output Path Finding Closed

Ticket helper now validates full planned output paths before mkdir/browser effects against schema256UTF-16 bound. Normal managed Bun RED66677 failed on absent helper; subsequent GREEN exited0 for8neutral cases on both POSIX/Windows path ports, independently checked with Ajv/node:path. See browser-output-paths.md. No browser launched.
