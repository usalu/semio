---
name: Wgpu Windows and Widget Interaction Parity
overview: Fix the confirmed root-cause bug that makes Tree/Table/VirtualFileSystem rows unclickable and unhoverable in the wgpu renderer, then close the concrete window/dock chrome gaps versus React (drag-to-dock, join-corner resize, measures-rail resize, close/layout persistence).
todos:
 - id: scroll-hit-order-fix
   content: Fix render_scroll_region (widgets.rs) and the two inline Table/VFS scroll wrappers (scenes.rs) to register the ScrollRegion hit before content, so row/chevron/action hits win over the wrapping scroll hit
   status: completed
 - id: hit-order-regression-test
   content: Add a regression test for InputState::hit_at proving an inner content hit wins over an outer overlapping ScrollRegion hit registered afterward
   status: completed
 - id: tree-interaction-verify
   content: Verify and fix any remaining gaps in tree expand/collapse, hover fill/commands, label click/selection_change, and drag-and-drop now that hits reach their targets
   status: completed
 - id: tree-visual-diff
   content: Screenshot-diff tree indentation guide lines/chevrons against React for a document-heavy plugin and fix any real mismatch found
   status: completed
 - id: table-vfs-interaction-verify
   content: Verify Table selectRow and VFS selectRows/openInstance/drag-drop now work end-to-end for representative plugins
   status: completed
 - id: dock-tab-drag
   content: Implement tab drag-to-reorder and cross-stack drag-to-dock with drop-zone detection and a floating drag preview, plus Escape-to-cancel
   status: completed
 - id: dock-stack-drag
   content: Add a hit target on the tab-bar gap for whole-stack drag, matching React's startStackDrag
   status: completed
 - id: dock-join-corner-resize
   content: Add join-corner dual-axis resize where a row-split and column-split meet
   status: completed
 - id: dock-measures-resize
   content: Add a drag handle for measures-rail width resize, reusing the side-panel resize pattern
   status: completed
 - id: dock-close-persistence
   content: Make close dispatch/persist a layout update instead of only mutating local DockState; round-trip stack activeId from the wire layout; add a layout-change persistence hook; apply named_layouts selection instead of only listing them
   status: completed
 - id: verify-all
   content: cargo test, WASM rebuild, wgpu E2E for tree/table/vfs/dock-heavy plugins, and screenshot-diff vs React
   status: completed
isProject: false
---

# Wgpu Windows and Widget Interaction Parity

## Context

Two areas were audited against the React renderer (the "premigration" reference other wgpu-parity plans already compare against): the Tree/Table/VirtualFileSystem widgets, and dock/window chrome. Both have real, verified gaps — not just cosmetic ones.

The prior [`wgpu_chrome_visual_parity_f5d2cbd0.plan.md`](.cursor/plans/wgpu_chrome_visual_parity_f5d2cbd0.plan.md) marked all Tree todos "completed", but re-verification against the actual running code shows the interaction layer is broken by a single shared bug, not missing implementation.

## 1. Root-cause bug: scroll regions swallow every hit inside them (highest priority)

`hit_at` in [ui/wgpu/rs/input.rs:166-171](ui/wgpu/rs/input.rs) resolves overlapping hits by taking the **last-registered** match:

```166:171:ui/wgpu/rs/input.rs
pub fn hit_at(&self, x: f32, y: f32) -> Option<&HitTarget<E>> {
    self.hit_targets
        .iter()
        .rev()
        .find(|target| target.rect.contains(x, y))
}
```

`render_scroll_region` in [ui/wgpu/rs/widgets.rs:1398-1424](ui/wgpu/rs/widgets.rs) renders its content (which registers row/chevron/action hits) **then** registers its own full-area `ScrollRegion` hit:

```1412:1423:ui/wgpu/rs/widgets.rs
    render_content(content_bounds, ctx);
    ctx.draw.pop_scissor();
    ctx.input.register_hit(HitTarget {
        rect: bounds,
        ...
        kind: HitKind::ScrollRegion,
        ...
    });
```

Because it's registered last, the `ScrollRegion` hit always wins over every row/chevron/action hit underneath it — the tree, table, and VFS body all resolve to "scroll region" for any click or hover, no matter where inside they occur. The exact same pattern (content hits registered, then a wrapping `ScrollRegion` hit registered after) is duplicated in two more places:

- Table body: [framework/renderer/wgpu/rs/scenes.rs:782-803](framework/renderer/wgpu/rs/scenes.rs) — row `selectRow` hit at 782-793, scroll hit registered after at 796-803.
- VirtualFileSystem body: [framework/renderer/wgpu/rs/scenes.rs:1420-1437](framework/renderer/wgpu/rs/scenes.rs) — row hit at 1420-1427, scroll hit registered after at 1430-1437.

This single bug is why the user sees: Tree not collapsible (chevron hit shadowed), no hover (hover uses the same `hit_at`/`hovered_id` path), no visible selection interaction (label click shadowed), and it also silently breaks Table row selection and VFS row selection/drag-drop — none of which were previously identified as broken.

**Fix**: in all three call sites, register the wrapping `ScrollRegion` hit **before** rendering the content, so content hits (pushed later) are found first by the reverse-iterating `hit_at`. Confirm this doesn't regress wheel-scroll dispatch ([framework/renderer/wgpu/rs/shell.rs:974-984](framework/renderer/wgpu/rs/shell.rs)) or the outer panel-level scroll hit (already correctly ordered before content at [shell.rs:2786-2794](framework/renderer/wgpu/rs/shell.rs)) — only the _inner_ scroll wrappers around widget content have the bug.

## 2. Tree verification pass (post-fix)

Structs, mapping, and rendering are already largely correct (`ui/wgpu/rs/widgets.rs:130-163` data model, `framework/renderer/wgpu/rs/interpreter.rs:222-251` wire mapping, `widgets.rs:1111-1396` render/chevron/guide-line code) — they were simply never reachable via pointer input. After the fix:

- Verify chevron expand/collapse actually toggles `collapsed_sections` (handlers already exist at [shell.rs:1297-1309](framework/renderer/wgpu/rs/shell.rs)).
- Verify hover fill and `tree_hover_commands`/`tree_unhover_commands` dispatch (existing logic at [shell.rs:1399-1417](framework/renderer/wgpu/rs/shell.rs)).
- Verify label click dispatches `item.event` and `selection_change` (existing logic at [shell.rs:1374-1380](framework/renderer/wgpu/rs/shell.rs)).
- Verify drag-and-drop initiation for `draggable` items (existing logic at [shell.rs:855-908, 1519-1580](framework/renderer/wgpu/rs/shell.rs)).
- Screenshot-diff indentation guide lines and chevron icons against React's `IndentationLines`/`TreeDocumentGutter` ([ui/js/react/index.tsx:9648-9707](ui/js/react/index.tsx)) for a document-heavy plugin (e.g. `s`, `lowpoly`) — fix any remaining depth/spacing mismatch found only now that the tree is actually interactive.
- Verify Table (`selectRow`) and VFS (`selectRows`/`openInstance`/drag-drop) click/hover now work end-to-end for a plugin using each (e.g. `vcs`/`forms` for Table, `s` for VFS).

## 3. Dock/window chrome parity

React's dock shell (`Mode`, [ui/js/react/index.tsx:19543-21324](ui/js/react/index.tsx)) supports drag-to-dock (tab reorder, cross-stack drag, drop zones, ghost preview), join-corner resize, measures-rail resize, and layout-mutation persistence. Wgpu's `dock.rs`/`shell.rs` only implement a fixed split tree with tab-click, maximize toggle, and local close — confirmed by direct code audit, not the stale claims in older plans.

### 3.1 Tab and stack drag-to-dock

- Tab hits are registered with `drag_axis: None` and no drag start ([dock.rs:577-584](framework/renderer/wgpu/rs/dock.rs)); the tab-bar gap area is draw-only with no hit at all ([dock.rs:587-592](framework/renderer/wgpu/rs/dock.rs)), unlike React's `startTabDrag`/`startStackDrag` ([ui/js/react/index.tsx:20474-20540](ui/js/react/index.tsx)).
- Add press-and-move-threshold drag initiation on tab hits (reuse the `InputState` drag primitives already used for split-resize, [shell.rs:807-819](framework/renderer/wgpu/rs/shell.rs)) and a hit on the gap region for whole-stack drag.
- Add drop-zone detection (tab-insert position within a stack, half-panel body split, root split) mirroring React's zones ([ui/js/react/index.tsx:20230-20290](ui/js/react/index.tsx)), plus a floating drag-ghost preview ([ui/js/react/index.tsx:20362-20392](ui/js/react/index.tsx)).
- Implement the corresponding `DockState` mutations (move window between stacks/create new stack/split), extending the existing mutation API pattern (`set_stack_active`, `close_active_in_stack`, `apply_split_drag` at [dock.rs:89-154](framework/renderer/wgpu/rs/dock.rs)).
- Escape cancels an in-progress drag, matching [ui/js/react/index.tsx:21004-21021](ui/js/react/index.tsx).

### 3.2 Join-corner (dual-axis) resize

- Wgpu only supports single-axis split-handle drag ([dock.rs:440-497](framework/renderer/wgpu/rs/dock.rs)); React resizes both adjacent axes at a shared corner ([ui/js/react/index.tsx:21037-21060](ui/js/react/index.tsx)). Add corner hit targets where a row-split and column-split meet and drive both `apply_split_drag` calls together.

### 3.3 Measures-rail resize

- `measures_width` exists in state ([shell.rs:209-210](framework/renderer/wgpu/rs/shell.rs)) but no drag handle is registered, unlike React's `WindowMeasuresResizeHandle` ([ui/js/react/index.tsx:13220-13275](ui/js/react/index.tsx)). Add a resize hit + drag handling analogous to the existing side-panel resize (`panel.resize.left/right`, [shell.rs:797-805, 2815-2833](framework/renderer/wgpu/rs/shell.rs)).

### 3.4 Close, active-id round-trip, and layout persistence

- Close only mutates the local `DockState` stack ([dock.rs:107-129](framework/renderer/wgpu/rs/dock.rs), dispatched at [shell.rs:1212-1216](framework/renderer/wgpu/rs/shell.rs)) with no command dispatch, unlike React's `onWindowClose` which updates the persisted layout tree ([framework/renderer/react/os-shell.tsx:1611-1624](framework/renderer/react/os-shell.tsx)).
- Stack `activeId` is dropped on load — wgpu always defaults to the first child window ([dock.rs:240-248](framework/renderer/wgpu/rs/dock.rs)) instead of round-tripping from the wire layout.
- No equivalent of React's `onLayoutChange` — `DockState` mutations (splits, closes, and the new drag-to-dock moves from 3.1) are ephemeral in wgpu today. Add a layout-change hook that persists the current `DockNode` tree back through the same channel the plugin/session uses (mirror whatever `shellLayout` persistence React uses in `os-shell.tsx`).
- Apply `named_layouts` when selected instead of only listing them as text ([shell.rs:532-558](framework/renderer/wgpu/rs/shell.rs)).

## 4. Verification

- `cargo test` for `ui_wgpu` and `semio-framework-renderer-wgpu` (add regression tests for `hit_at` ordering with a scroll region + overlapping content hit, and for the new dock drag/resize mutations).
- Rebuild WASM bindings for `ui_wgpu` and `semio-framework-renderer-wgpu`.
- Run the existing wgpu E2E harness ([.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts](.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts)) for plugins covering each surface: a tree-heavy plugin (`s` or `lowpoly`), a Table plugin (`vcs`/`forms`), a VFS plugin (`s`), and any plugin with a multi-window/multi-panel layout for the dock work.
- Screenshot-diff wgpu vs React for the same plugins to confirm tree collapse/hover/selection, table/VFS row interaction, and dock drag/resize/close chrome now match.
