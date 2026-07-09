---
name: panel-ghost-interaction
overview: 'Add a "ghost while interacting" mode to all panels: when any pointer drag or drag-and-drop starts inside a panel, the active control stays fully visible while the rest of the panel fades to 5% opacity and becomes click-through to the canvas below.'
todos:
 - id: ticket
   content: Open a repo MCP ticket (read repo://goals, associate to best goal) for the panel ghost-interaction feature; keep all temp files/logs in the ticket folder
   status: completed
 - id: controller
   content: Add PanelGhostContext + usePanelGhost (state + active-node ref) and usePanelGhostRoot (pointerdown + document move-threshold + up/cancel) in the Interaction Context region of ui/react/index.tsx; bridge existing setActiveInteraction to begin/end
   status: completed
 - id: css
   content: "Add ghost CSS to ui/react/globals.css: dim [data-dim]:not([data-active-interaction]) to 5% with 150ms transition, and pointer-events:auto for the active element under [data-ghost]"
   status: completed
 - id: chrome-refactor
   content: Refactor SidePanel, MobilePanel and Panel to render background/border as a dedicated data-dim chrome layer; tag tab bar, footer, content rows/sections with data-dim (single-level)
   status: completed
 - id: root-wire
   content: Wire each panel root with usePanelGhostRoot handlers, data-ghost attribute, and conditional pointer-events:none in ghost mode
   status: completed
 - id: dnd-hooks
   content: Call ghost begin/end from native tree handleDragStart/handleDragEnd and pointer-palette drag handlers so HTML5 drag-and-drop triggers ghost mode
   status: completed
 - id: remove-old-fade
   content: Remove redundant shouldFade->opacity:0 logic in Slider/Input/Combobox and route them through the unified ghost dimming
   status: completed
 - id: verify
   content: Build/typecheck via nx and manually verify (browser) sliders, resize, tree DnD ghost the panel, canvas receives passed-through clicks, plain clicks do not trigger ghost, and pointer-capture survives mid-drag activation; close ticket with summary
   status: completed
isProject: false
---

# Panel Ghost Interaction Mode

## Goal

When a UI interaction (slider drag, resize, tree drag-and-drop, pointer-palette drag, any pointer drag) starts inside a panel:

- The actively-manipulated element stays at 100% opacity and remains interactive.
- The rest of the panel chrome (background, border, tab bar, other rows, footer) fades to 5% opacity.
- The panel becomes `pointer-events: none` so all other pointer input passes through to the canvas below.
- On pointer up / drag end / cancel, the panel returns to normal (150ms transition, matching existing fades).

Applies to all panels: left/right `SidePanel`, `MobilePanel`, generic `Panel` (and its `Left/Right/Bottom/Middle` variants). For overlay panels (side/bottom/generic) this gives true click-through to `{canvas}`; for `MobilePanel` (stacked, not overlapping canvas) it is visual-only, which is acceptable.

## Key technical constraints (confirmed)

- CSS `opacity` on the panel root would cap all children to that opacity, so we cannot brighten the active child. Instead we dim per-row + chrome and keep the active row at full opacity.
- `pointer-events: none` on the panel root + `pointer-events: auto` on the active element keeps the active element interactive while the rest is click-through (standard overlay pattern).
- No DOM reparenting/portaling: we only toggle classes/attributes on already-mounted nodes, so in-flight pointer capture and gesture state survive when ghost mode activates mid-drag.

## Current state

- Panels live in [ui/react/index.tsx](ui/react/index.tsx): `Panel` (~11078), `SidePanel` (~11364), `MobilePanel` (~11500), `Layout` (~2824).
- There is an inert interaction system: `InteractionContext` / `ActiveInteractionContext` + `setActiveInteraction(elementId, interactionId)` (~2271), consumed by `Slider`/`Input`/`Combobox` to fade _sibling controls to opacity 0_ via `shouldFade`. `InteractionProvider` is never mounted anywhere, so nothing currently fades.
- Tree drag-and-drop: native HTML5 DnD via `handleDragStart`/`handleDragEnd` (~9736/9765) and pointer-palette drag (~9797).
- `usePointerDrag` (~11266) drives `SidePanel`/`Panel` resize via pointer capture.

## Design

Introduce a panel-scoped "ghost" controller that unifies and replaces the inert per-control fade.

### 1. Ghost controller (in Interaction Context region, ~2271)

- Add `PanelGhostContext` exposing `{ active: boolean, begin(targetEl), end() }` and a `usePanelGhost()` hook owning `active` state + a ref to the active DOM node.
- Reuse the existing `InteractionContext`: have the panel provide `setActiveInteraction` so existing tracked controls (`Slider`/`Input`/`Combobox`) automatically participate. `setActiveInteraction(id, interactionId)` -> `begin`; clearing -> `end`.

### 2. Generic drag detection at panel root (new `usePanelGhostRoot()` hook)

- On `pointerDown` inside the panel, record start point + target.
- On document `pointermove` past a small threshold (e.g. 4px) -> `begin(target)` (this covers sliders/resize/any pointer drag, including non-tracked controls).
- On document `pointerup`/`pointercancel` -> `end()`.
- For native HTML5 tree DnD, call `begin`/`end` from `handleDragStart`/`handleDragEnd` and the pointer-palette drag handlers (since native DnD does not emit `pointermove`).

### 3. Active-element tagging + CSS dimming

- On `begin(target)`, walk up from the target to its nearest dimmable row boundary (`[data-dim]`) and set `data-active-interaction` on it; set the panel root `data-ghost="true"`.
- Add `data-dim` to the dimmable, non-nested groups in each panel: the chrome/background layer, tab bar, footer, and each top-level content row/section (tree sections at ~9988, `property-row` at ~3323/9274). Keep `data-dim` single-level (no `data-dim` inside another `data-dim`) to avoid compounding opacity.
- CSS (add to [ui/react/globals.css](ui/react/globals.css)):
  - `[data-ghost="true"] [data-dim]:not([data-active-interaction]) { opacity: 0.05; transition: opacity 150ms; }`
  - `[data-ghost="true"] [data-active-interaction] { pointer-events: auto; }`
- Panel root in ghost mode gets `pointer-events: none` (inline style/class), keeping root `opacity: 1`.

### 4. Refactor panel chrome into a dimmable layer

- `SidePanel` (~11427), `MobilePanel` (~11519), `Panel` (~11078): move `bg-panel`/border off the root onto a dedicated absolute chrome layer marked `data-dim` so the background itself fades to 5% (revealing the canvas) while the active row stays opaque. Tab bars and footers get `data-dim`.
- Wire each panel root with `usePanelGhostRoot()` handlers + `data-ghost` + conditional `pointer-events`.

### 5. Remove the old per-control fade

- Replace the `shouldFade ? 0 : ...` opacity logic in `Slider`/`Input`/`Combobox` (~4954/5263/5498 and their style sites ~3561/4960/5420) with participation in the unified ghost (their row dims to 5% like everything else; the active one stays bright). This eliminates the now-redundant fade-to-0 behavior per the no-legacy/refactor rule.

### 6. Mount the provider

- `Layout` (or each panel) mounts the ghost provider so `setActiveInteraction` is no longer inert. Simplest: each panel hosts its own `usePanelGhost` provider so ghost state is panel-local (interacting in the left panel does not ghost the right panel).

## Verification (per repo rules: confirm runtime, do not assume)

- Open a repo ticket (`ticket_open`) before editing; keep temp logs/screenshots in the ticket folder with `[DEBUG]` prefixed logs.
- `nx` build/typecheck of `@semio-tech/ui-react` and a consuming app.
- Manual via launch.json + browser:
  - Drag a slider in a side panel -> only the slider row stays visible, rest at 5%, canvas visible behind, slider keeps tracking the cursor.
  - Resize handle drag triggers ghost and still resizes.
  - Tree drag-and-drop triggers ghost; drop still works.
  - Clicking/dragging on the dimmed area hits the canvas (confirm with a `[DEBUG]` canvas log).
  - Plain clicks (no movement) on panel buttons do NOT trigger ghost.
- Confirm pointer-capture survival when ghost activates mid-drag (the main runtime risk); if a specific control loses capture, switch that control's drag to document-level listeners (like `Panel` resize already uses).

## Notes / decisions

- Threshold-based activation avoids flicker on plain clicks.
- Ghost is panel-local (per panel root), not global.
- `MobilePanel` click-through is a no-op (no overlapped canvas) but the dim still applies for consistency.
