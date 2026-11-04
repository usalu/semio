# 🎮️ Steam Deck Usability Audit (T18d, static)

ui = `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`, sh = `…/📺️renderer/🧑‍🎨engine/🧱️elements/`. The Deck screen (1280×800) counts as desktop; 800 px counts as tablet (`🖱️ui/📱️device/🟦️.ts:26-39`).

## High

1. **Panels can cover the editor.** Panels float with absolute positioning, default 300 px and maximum 600 px (`ui/🖼️Panel/🟦️.tsx:170,385-386,436-440`). The shell never sets `maxSize`, there is no viewport clamp, and panels do not collapse on shrink. They also reopen when a tool is selected (`sh/🏗️dock-assembly/🪟️panel-reveal-and-tool-tabs/🟦️.ts:322-323`).
2. **The text editor cannot be selected by touch.** `sh/✏️TextEditor/🟦️.tsx:50-94` has no `touch-action: none`, so a pointercancel fires and selection never starts.
3. **Text editor and shell menus need a right click.** The context menu is the only route (`TextEditor:104`, `build-shell-context:56-67`, `ContextMenu:449`, Table row menus); there is no visible button.
4. **Mesh suggestions need Alt + right click.** World3d `instance-selection/🟦️.ts:569-591` offers no touch or keyboard route.
5. **Opening an artifact does not move focus.** `🔶️catalog/🟦️.ts:199-214` and `🔸️open-artifact-app/🟦️.ts:70-107` never focus the new editor.
6. **Ctrl+Z is swallowed in Details fields.** `⌨️keyboard-chords/🟦️.ts:16-17` and `document-transfer-ref:54` let the field consume it, so editor history is unreachable while the field has focus.
7. **There is no gamepad layer.** Input relies on Steam Input emulation. This is a documented limitation, not an editor defect.

## Medium

- Double-click is the only way to select a word or open a row (no touch double-tap or long-press).
- Pressing Enter in a draft calls `blur()`, so focus is lost (`input-views:217-225`).
- Escape overlap between fields and the global handler.
- No minimum editor window size.
- Hover-only table states.

## Low

- Tablet footer tab clipping.
- Hover hint without a touch guard.
- Mobile-only canvas hiding.

## Media, Document, Table and Details (T18e)

**High**
- TextEditor traps Tab: it always inserts a tab, even in read-only mode (`✏️TextEditor/🟦️.tsx:388-393,295-298`). Escape does not leave the editor. Fails WCAG 2.1.2. Affects the docx/pptx/text drafts.
- The Details tree has no arrow-key navigation. Items carry `role=treeitem` but no tabIndex and no key handler (`🖱️ui/🌳️Tree/🍃️item/🟦️.tsx:208-211`).

**Medium**
- The media trim sliders only update local state; nothing reaches the app (`🎬️MediaTransportHost/🟦️.tsx:177-178`, `🎬️media/🌐️browser/🎛️host/🟦️.ts:103-104`).
- After cancel, the React media host stays in its loading state forever (`:114,149`).
- Position is shown as raw milliseconds (`:173`).
- Pressing Enter in a table cell blurs the field.

**Low**
- No play/pause/seek shortcuts.
- Range thumbs are small for touch.
- The pdf 1.4 page kit was not traced.

**Positive**
- Object URLs and listeners are released on dispose in both media hosts.
- Table keyboard navigation works: F2, arrow keys, Escape.

## Image Kit, PDF Page and Mesh (T18f)

**High**
- Raster painting, selection and zoom have no keyboard route; only the tool letters and select-all/deselect work (`🖌️Paint2dHost/✍️editing/🟦️.tsx:345-355`).
- PDF/UA 1.7 renders the page as a Canvas2d scene with no text alternative or structure (`📖️pdf/…/7️⃣1.7/…/🖼️page/🪟️windows/🦀️.rs:127-129`; `render_windowed` ignores locale).
- Mesh has no keyboard orbit or zoom: `OrbitControls` is never given `listenToKeyEvents` (`♾️infinite/🌍️world/🎨️r3f/🎬️canvas/🟦️.tsx:281-311`).

**Medium**
- Image alt text gives only the dimensions (`🔌️plugin/🪟️window-kits/🖼️image/🦀️.rs:77,97`).
- Three nested regions share the same label `text.tools` (`Paint2dHost:341,357,365`).
- Strokes are capped at 2048 points with only a status message (`:300`).
- The WebGL context is never released explicitly (no `forceContextLoss`).

**Good**
- A paint stroke is one history step: streamed via `dispatch`, committed once via `dispatchEdit`, and an abort leaves no step.
- Painting supports `touch-none` and pinch.
- Mesh resources are disposed.
