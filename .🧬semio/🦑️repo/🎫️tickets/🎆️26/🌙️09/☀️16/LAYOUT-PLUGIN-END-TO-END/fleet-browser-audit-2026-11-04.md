# Retained Layout Browser Source Audit

2026-11-04. Read-only source audit; no source changes, build, test, activation, server or browser execution. Shared ticket remains owned by root.

## Result

No actionable selector/API/runtime coupling defect was proved in the inspected current source. This is bounded source review, not runtime acceptance.

## Grounded Checks

- browser-journey.mjs:57–81 reads the current data-node-key/data-window mirror and dock selection. The production mirror assigns those attributes at renderer/engine/targets/wgpu/accessibility-mirror/🟦️.ts:113–118 (full taxonomy path below). Dock axis-solving-and-stack-rendering/🦀️.rs:266 constructs dock.tab.<path>.<window> identities.
- browser-journey.mjs:125–129 activates through focus and Enter. The mirror carries native keyboard activation at :170–180 and native value/blur messages at :163–164 and :208–210. Its :244–251 preserves retained depth ancestry, supporting argument-row descendant input selection. Its :259 replaces published DOM; the journey uses delayed publication polling for select and story edits.
- browser-journey.mjs:147–190 uses window-action-pane, action.<id>.arg.<name>, and .action.<id>.execute. Shell staging/window-actions/window-actions-and-commands/🦀️.rs:238–240 constructs the argument IDs, :410 constructs the execute suffix, and :429 publishes window-action-pane.
- Layout create-layout-app/🦀️.rs:38–51 declares addFrame/addPage, select kind options rect/text/image, numeric x/y and exportPng/exportSvg/exportPdf, matching the retained journey contract. Layout artifact panel/🦀️.rs:78 builds layout-document.page.<pageId>; inspection panel/🦀️.rs:464 includes global.story.<storyId>.content. Demo DSL contains story-1 Hello layout and required frame IDs.
- browser-journey.mjs:212–224 listens for actual downloads; current WGPU host-io/🟦️.ts:133–144 mounts and clicks an anchor backed by a Blob and download filename; :545–551 dispatches download-media-export and request-file-open. Its :189 clicks the actual file input, consistent with journey :353–357 filechooser observation.

## Source Roots

Renderer engine: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine. Mirror and host I/O are below 🎯️targets/🧊️wgpu/♿️accessibility-mirror and 🚪️host-io. Shell is below 🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧰️window-actions-and-commands. Dock is below 🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🧮️axis-solving-and-stack-rendering.

Layout editor: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor. Manifest is 🧱️create-layout-app/🦀️.rs; panels are 📌️panels/🗿️artifact/🦀️.rs and 📌️panels/🔍️inspection/🦀️.rs.

Inspected root/product/os/layout AGENTS and build skill. Commands were repo tools exec --light node for filesystem reads/report write and repo tools exec --light git -- grep for read-only search. No generated output retained. Live mirror ownership, actual dispatch, contrast, export validity and archive restore remain the root runtime gate.

## Later Retained Journey Route/Owner Recheck

Catalog layout row remains pluginIdlayout, app s.layout.layout@1/*#editor, WGPU6179, examples🎬️demo/demo-session. Journey default query plugin=layout and encoded🎬️demo matches boot-query/🟦️.ts:8–28/:62–65. Actual blueprint window kind is layout-blueprint in Layout modes/edit/windows/blueprint/🦀️.rs:21 and TS:21; WGPU dock session topology:68–70 allocates <kind>-N, preserving blueprint regex and dot-free identifier assumption. Explicit browser context en-US feeds Locale.fromLanguageTag through WGPU browserboot:22 (subject to declared locks, as authored). Earlier mirror native activation/value/blur and staged IDs remain source-consistent.

No new proven stale selector or API mismatch. Import fresh-owner window creation remains live acceptance uncertainty: regex matches actual naming, but this bounded audit does not claim to prove import allocation semantics merely from dock allocator. Retained acceptance is unchanged. No browser/build/test or source changes.

## Demo Edited Media Source Ownership

No concrete source mismatch found for retained demo SVG/PNG/PDF path. Actual demo DSL link-missing has state missing and no proxy/embedded Drawing image. Export pdf-png-and-package-encoding/🦀️.rs:46 marks missing-or-proxyless image as placeholder. Faithful-primitives/🦀️.rs:87–88 bypasses decoder for placeholder; :122 creates painted placeholder path. Sampled Drawing image owner refusal therefore is not reached by this demo. New rect and story-marker edits introduce no sampled image.

Retained export-tool-job/🧵️retained/🦀️.rs:45 requires actual snapshot_owner pointer equality to input.snapshot and carries Arc clone of that same original into LayoutExportRequest. :40 takes actual supplied output chunks once; :47 returns sealed same queue with paid metadata. Source audit does not establish actual runtime delivery, but no detached-document adoption mismatch is visible in this path. Acceptance remains unchanged; no source edits/build/browser/tests.
