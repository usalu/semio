# Accessibility Mirror and Event Audit

## Scope and evidence boundary

This is a read-only source audit of the accessibility activation packet and the fresh checkpoint8 observations supplied to the ticket: Settings opens after its accessibility activation; General receives mirror focus but the panel did not visibly switch after more than 30 seconds; chrome switches expose neither checked state nor a meaningful value; retained spinbuttons lack names; and paragraph elements are empty. No build, browser automation, runtime command, production source, ticket state, or goal state was changed.

The packet records 43 TypeScript tests as passed and lists native commands still owned by the activation job. That is useful coverage evidence, but it is not proof of the browser boot mirror: `🧪️tests/♿️wgpu-accessibility-interaction/🟦️tsx` mounts its local `OracleMirror`, not `🚀️browser-boot/🟦️ts`'s production `accessibilityMirror`. The native tests were not run by this audit.

## Event path and positive findings

The real path is intact and narrow. `🚀️browser-boot/🟦️ts:177-365` obtains the published dump, creates native mirror elements, and emits focus, activation, and value events with `{windowId, windowGeneration, nodeId, nodeKey}`. `🚚️browser-frame-transport/🟦️ts:483-505` validates those addresses and queues them in the lossless lane. `🌐️browser-worker/🦀️rs:592-613` repeats the fixed-credit validation, and `🪟️winit-app/🦀️rs:414-422` passes the decoded event to `ShellState::handle_accessibility_event`.

For a retained document, Shell delegates to `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️rs:460-466`, then `Ui::dispatch_accessibility_event` rejects a non-current generation, zero generation, missing record, or id/key mismatch before `EventRouter::dispatch_accessibility` can focus or fire an action (`🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️rs:2015-2032`; `⚡️events/🦀️rs:1722-1780`). One accepted activate event takes one EventRouter activation arm. The mirror's click handler stops propagation, while keyboard listeners are attached only to the canvas (`🚀️browser-boot/🟦️ts:246-252,408-446`), so mirror activation does not also enter the canvas keyboard path.

The suspected retained-generation reuse after close/reopen is not currently reachable. `UiWindow::new` does initialize `accessibility_generation` to zero (`⚙️engine/🦀️rs:60-114`), but `UiSurfaceRegistry` has admission and lookup only; this source has no slot-removal path (`:421-480`). `close_document_step` retires the document inside the same `UiWindow` (`:1021-1043`), and the next publication increments its accessibility generation (`:944-955`). The existing engine law closes, republishes the same id, and proves the older generation is rejected (`🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️rs:101-127`). A future slot-removal API must carry its registry lifetime into this generation or add a remove/re-admit regression; the current close/reopen route already preserves the counter.

The fresh General-tab observation is not explained by a missing accessibility activation arm. Chrome activation finds the current hit and calls `handle_shell_hit` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️rs:12395-12450`). A current panel-tab id passes `panel_tab_anchor`, calls `select_panel_tab`, and updates the anchor path (`:12670-12678,13389-13450`). `framework.settings.general` is an actual framework leaf (`:103,8223-8232,18600-18620`). The present source does not prove why the observed panel stayed visually unchanged; it needs a generation-correlated ingress-to-published-frame trace. The current test exercises a synthetic search hit, not a real Settings tab.

## Proven defects

### 1. A described native input throws while rebuilding the production mirror

**Path:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:194-260`.

`🚀️browser-boot/🟦️ts:194-202` maps textbox, combobox, slider, and spinbutton projection nodes to `HTMLInputElement`. For every described node, the same function creates a span and calls `element.appendChild(description)` (`:254-260`). Inputs are void elements, so that append raises a DOM hierarchy error. `pull` stores `published = json` before it calls `paint` and catches only JSON parsing (`:304-329`); after this throw, the unchanged dump is skipped on later pulls. A valid retained document with, for example, the contract fixture's described spinbutton (`🖱️ui/🧬️contract/🧫️fixtures/♿️accessibility-projection.json:59-66`) therefore prevents this mirror revision from publishing.

Minimal repair: create the description span as a visually hidden sibling in the current mirror parent, retain its id on the input's `aria-describedby`, and never make it a child of the projected element. Preserve the same stable id and tree order.

Required ingress regression: drive the real browser-boot mirror with a worker accessibility dump containing a described spinbutton, then assert that refresh completes, the native number input references the sibling description, and a second unchanged pull remains live. The test must call the production mirror rather than `OracleMirror`.

### 2. Chrome projects visual selection as neither switch state nor tab selection

**Path:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22177-22218,23035-23052,27283-27318`.

The footer paints each panel entry as `HitKind::Toggle` and supplies its live `active` value from the anchor state (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️rs:23035-23052`). Anchor panel tabs are painted as `HitKind::PanelTab`, with a live row-specific active id (`:22177-22218`). Yet `chrome_accessibility_nodes` gets `checked` only from `widget_maps.toggle_metas`, unconditionally gives `selected: None`, and supplies no other active-state projection (`:27283-27318`). Chrome group items are not populated through that widget metadata path. The browser faithfully omits an ARIA attribute when the optional state is absent (`🚀️browser-boot/🟦️ts:215-237`). This directly accounts for the observed Settings `role=switch` lacking `aria-checked`, and all actual `role=tab` nodes also lack `aria-selected`.

Minimal repair: derive chrome state from the same Shell state that painted the hit: use anchor visibility/path for panel toggle checkedness and the row's active path entry for `PanelTab` selection. Do not infer either state from an unrelated widget metadata table.

Required ingress regression: render the actual footer Settings control, activate it through the decoded accessibility event, publish the next chrome dump, and assert `role=switch` with `aria-checked=true`. Then activate the actual `framework.settings.general` panel tab and assert its anchor path and `role=tab`/`aria-selected=true` in the next mirror dump. This both covers the untested General path and separates a dispatch fault from a later frame-publication fault.

### 3. The shell producer loses visible labels at the control/text boundary

**Paths:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4452-4617,17030-17076`; `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:194-237`.

The panel assembler writes a Field's label onto its container record but calls `self.node(&field.child)` before giving the child any accessible name (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️rs:4452-4560`). Its Select, Input, Slider, and NumberStepper arms all publish their child `PanelRecord` with the default `label: None` (`:4561-4617`). A labelled field therefore produces a labelled group containing an unnamed native control; the browser mirror has no `aria-labelledby` relationship to bridge that gap. The engagement builder also writes its visible stepper label as a preceding `UiNode::Text` but publishes the `NumberStepper` without a label (`:17030-17076`, `:4604-4610`). This is a source explanation for the observed unnamed retained spinbuttons.

The text half has the same projection break. The assembler preserves a `UiNode::Text` label (`:4528-4535`), but the mirror turns a `paragraph` into an empty `<p>` and only sets `aria-label`; it never writes `textContent` (`🚀️browser-boot/🟦️ts:194-237`). That matches the empty paragraph nodes in the fresh inspection and makes the semantic text depend on a label attribute rather than actual paragraph content.

Minimal repair: make a Field's authored label/description name and describe its single form-control child in the published accessibility projection, and give labelled standalone controls such as the engagement stepper an explicit `AccessibilitySpec.label`. Materialize a paragraph's label as its text content while retaining the data attributes needed by the probe.

Required ingress regression: construct the actual Shell Field and labelled engagement stepper builders, publish/reconcile them through the retained document path, and inspect the production mirror. Assert that each focusable form control has a native accessible name, the described relation reaches the control, and a paragraph contains its announced text. This should be a real document-to-mirror test, not a projection-only fixture.

## Decision boundary

The three defects above are independently source-proven. The browser observation that General focuses but the panel body stays on Settings does not yet identify a fourth defect: its semantic activation branch is present, while the ticket already has a documented serial frame-publication delay. Instrument the proposed real panel-tab regression at event acceptance, anchor-path mutation, chrome-dump publication, and first rendered body change before changing event routing. No evidence here supports suppressing diagnostics or changing the generation/keyboard/exact-once guards.
