# Shared Layered Presentation UI

The teaching renderer will import LayeredOverview and OverviewCard directly from framework UI. The shared geometry, RAF follower, glide, veil clip and card tint remain owned by those components. The renderer will load the existing course chrome-and-home CSS and use its exact card cell structure. Root topic pages will contain nested video overviews; empty topics remain honest empty pages. The pipeline controls the semantic selected/revealed path and camera; the browser advances the shared UI clock and records actual DOM geometry.

## Sources

- Framework UI: 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx
- Cards: 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx
- Math: 🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts
- Existing course nesting: 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🏠️home/🟦️.tsx
- Existing course layout: 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🏠️chrome-and-home/🎨️.css

## Implementation

Awaiting hierarchy wire contract agreement with coordinating agent before source edits.

The browser now accepts only the new hierarchy/media protocol. Topic and video pages use shared LayeredOverview; all cards use OverviewCard with OverviewCardOpenChip in footerLeft. The existing course cell/grid CSS is loaded as a tracked source dependency. Every nested level inherits the active normalized camera, and shared controlled reveal/open hooks retain native pointer and keyboard behavior. Full-page video panes retain actual WindowChrome, measured silhouettes and body viewport.

The backend advances Playwright clock for shared requestAnimationFrame transitions, records per-level card/pane/veil geometry and keeps original cursor assets, image decoding, cancellation, source manifests, browser provision and DPR capture. A source-only bundle build passed. Runtime visual/browser checks await coordinating contract update.

## Verification

- Source-only bundle and dependency closure build passed.
- Six actual-browser smoke states passed: root overview, topic hover, energy video overview, video hover, opened video, restored root. PNGs were inspected visually.
- The independent verification agent reported all fourteen production browser frames passing across English and German, compared with a separate shared-component oracle: same glass computed CSS, bottom-left View/Ansehen chips, normalized camera applied to active and inert background grids, identical hover/open pane bounds (less than 0.5px), and inert previews until opening. Six targeted verification tests passed.
- Focus media content measures x=6.375, y=57.5625, width=1587.21875, height=807.234375 at 1600x900, matching the authored fit viewport within browser quantization.
- Silhouette settlement checks compare numeric path tokens within 0.01px because ancestor strip transforms change DOM subtraction precision; actual shared silhouette construction and assets are unchanged.

Modified production files: browser/🟦️.tsx, browser/🟨️render.mjs, browser/🧬️schema.json. No shared framework component was modified.

## Python Pipeline Integration

The main presentation port now resolves videos through the hierarchy catalog, tracks the new renderer in source provenance, and delegates frames, shells and transition batches to presentation/🥞️hierarchy/🖌️render/🐍️.py. The obsolete tiled overview layout and duplicate native flat canvas/shell renderers were removed. Native scientific control and cursor helpers remain in interaction/render. Existing FFmpeg media/audio composition, fixed authored content fit, cancellation and publication are retained.

The hierarchy adapter supports either explicit hierarchy plus pointer states or normalized focus/restore requests. It measures actual DOM topic/video chips, restore control and parent navigation before creating the three-second cursor route. Focus enters topic and video only after distinct native press/release/click events; restore clicks the actual fullscreen control and footer parent. A neutral measured-control-relative waypoint handles the identical Energy and first-video chip positions. Full press stages preserve both presses even at four frames per second. Actual document pointer events are emitted per captured frame for independent verification.

Shared RAF uses per-frame clock deltas; CSS animations accumulate elapsed time instead of restarting. Canonical root frames clear source media and settle the common park camera, so every lesson has identical boundaries. Shell captures retain the actual mouse point with cursor.visible=false, preserving ancestor hover without duplicating the authored scientific cursor.

Python runtime smoke with the existing shared Manim interpreter passed twenty-six hierarchy states for Energy Balance. A fresh twenty-four-frame Physical Fundamentals focus/restore run at 1280x720 and 4fps recorded actual down/up/click pairs for Energy, Physical Fundamentals, the actual fullscreen control and footer parent. First and final root images were pixel-identical.

## Restore Settlement Review

A separate required restoreTiming contract now preserves the three-second total while closing the video after 0.75 seconds and returning to the topic root after 1.5 seconds. Pointer travel to the common park ends at 1.8 seconds, followed by a 1.2-second natural shared-camera hold. Canonical resets apply only to the true normalized endpoints, so the hold renders the actual shared follower rather than repeatedly replacing it. The frame count uses each phase timing contract.

A fresh twelve-frame restore run at 4fps recorded the actual fullscreen and footer press/release/click pairs. The preceding final-hold root pane and canonical endpoint had zero CSS-pixel geometry difference, demonstrating that the shared follower settled before the endpoint reset.

The regression suite exposed an extraction mistake while removing the obsolete native flat renderer: the native SVG icon helper body was accidentally omitted. Its original asset-based helper was restored with the cache decorator attached to the icon helper, and the scientific button helper remains uncached. This cleanup issue did not affect the actual browser rendering path.

## Final Owned File List

All paths below are relative to 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/.

- 🌐️browser/🟦️.tsx: actual shared nested overview/card composition, inherited normalized camera, explicit hierarchy selection, full-pane native WindowChrome and cursor visibility. Topic-specific thumbnail artwork was edited independently by the coordinating agent.
- 🌐️browser/🟨️render.mjs: shared component aliases, source course CSS provenance, hierarchy/media batch acceptance, deterministic shared RAF and accumulated CSS animation time, semantic-release ordering against the prior DOM, per-frame actual pointer-event capture and layered DOM metrics.
- 🌐️browser/🧬️schema.json: hierarchy/media wire contract and cursor.visible.
- 🥞️hierarchy/🖌️render/🐍️.py: new actual-browser Python hierarchy adapter, real DOM control probes, normalized/direct frame batches, focus/restore choreography, canonical media isolation and scientific shell capture.
- 🐍️.py: hierarchy video resolution, source provenance, new rendering adapter delegation and phase-specific finite frame counts. Authored scientific media/audio composition remains intact. Obsolete flat overview geometry and native presentation canvas helpers were removed.
- 🖱️interaction/🖌️render/🐍️.py: removed duplicate flat presentation rendering, retaining asset-based native scientific icon/button/cursor helpers; repaired the icon-helper extraction regression identified by tests.
- 🔣️.json: added separate restoreTiming after the coordinator authored the hierarchy catalog.
- 🧬️schema.json: required restoreTiming and shared timing-schema definition.

## Final Scope And Verification State

No shared framework LayeredOverview, geometry, veil, card, chrome or course grid CSS source was modified. The teaching renderer imports and executes those actual sources. No Git mutation, worktree, AGENTS edit, external runtime dependency or compatibility fallback was introduced. Temporary browser/media evidence remains under the ticket generated folder for the coordinating agent to verify and remove after retaining final artifacts.

Validated: actual browser source bundle; independent shared-component oracle and fourteen bilingual production frames; actual first-video click route at low frame rate; exact common root pixel boundaries; naturally settled restore camera before the canonical endpoint; syntax of all changed Python integration modules. The independent regression agent separately reported all five encoded lesson presentation tests passing before the final restore-only timing adjustment and is completing native scientific helper/popup tests. The coordinating agent owns final physical-source media verification and ticket closure. Production implementation is frozen unless a new test failure requires repair.

## Final Catalog Isolation And Handoff Asset

Choreography now uses an explicit ordered tuple of eight named stages; JSON object serialization order cannot change stage semantics. The independent reversed-key-order regression was observed failing before this correction.

Browser batches now require a catalog payload, validated against the parent presentation schema. The browser executes that captured payload while retaining the authored catalog path in source provenance. The coordinating agent separately added Python catalog capture across a lesson and finite multi-lesson assembly, preserving concurrent BIM and Optimization catalog work.

Stable-catalog probes demonstrated browser raster variation despite recursively identical measured DOM metadata, exact strip-transform strings, identical card geometry and identical computed glass/font metrics. Repeated raw PNGs varied by small RGB amounts; disabling the GPU did not resolve this, and the bounded Chromium deterministic-mode probe timed out during screenshot capture. No experimental browser flags were adopted. Earlier successful pixel comparisons did not establish general browser raster determinism.

The Python browser port now publishes a shared actual-UI handoff asset for the common canonical root state. Every requested frame still executes the actual UI and returns its independently measured DOM and native pointer-event metadata. After capture, canonical root output images reuse the bytes of the first actual UI raster for their versioned state. This is asset reuse at a shared video handoff, not custom UI drawing or a claim that browser rasterization is deterministic.

The bounded process-memory cache retains at most sixteen assets under a reentrant lock. Identity includes the full captured catalog, explicit locale, viewport, hierarchy/cursor state and content identity of all tracked source code, CSS, fonts and assets. Only the authored catalog-file bytes are excluded because the captured catalog payload already supplies that exact input; its source path remains tracked. UI source identity is checked before and after browser capture, and a concurrent code/asset change rejects handoff publication. Returned handoff metadata exposes the identity and whether actual UI bytes were reused.

Additional owned production file: 🌐️browser/🐍️.py, limited to the handoff asset helpers and render_batch publication. The coordinating agent owns its captured-catalog source-discovery packet change. Retained investigation input: ticket/probe-layered-raster.mjs. All generated probe images and logs remain under the ticket generated directory for final cleanup.
