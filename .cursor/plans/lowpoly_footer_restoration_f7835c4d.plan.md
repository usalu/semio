---
name: Lowpoly Footer Restoration
overview: Restore the lowpoly footer toolbar to parity with the pre-migration TypeScript editor by fixing the root cause (unvendored icon ids collapsing to generic circles), restoring old labels/grouping/icon choices, and bringing back the window engagement rail (Snap/Smooth/command bar/status) that was dropped in the Rust port.
todos: []
isProject: false
---

## Root cause

The footer toolbar is built in [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs) `edit_tools()`/`paint_tools()` (lines 849-964) and rendered by [framework/renderer/react/tool-tree.tsx](framework/renderer/react/tool-tree.tsx). `toolIcon()` in that file falls back to a generic `circle` icon whenever `iconId in ICONS` is false:

```33:35:framework/renderer/react/tool-tree.tsx
function toolIcon(iconId: string): IconName {
	return iconId in ICONS ? (iconId as IconName) : "circle";
}
```

`ICONS` is a **curated whitelist** of ~119 Lucide icons vendored by [ui/asset/script.ts](ui/asset/script.ts) into [ui/asset/icon/generated/icons.ts](ui/asset/icon/generated/icons.ts) (this vendoring system was introduced 2026-06-04, after the old lowpoly TS editor was written, so the old editor never hit this restriction). Verified against the generated file: **every** custom icon id lowpoly's Rust `edit_tools()`/`paint_tools()` reference — `arrow-up-from-line`, `shrink`, `flip-horizontal`, `flip-vertical`, `git-commit-horizontal`, `grip-lines`, `merge`, `grid-2x2`, `paint-bucket`, `pipette`, `unfold`, `scissors`, `paintbrush`, `undo`, `redo`, `triangle`, `eraser`, `maximize` — is **not vendored**, so nearly every footer button renders as an indistinguishable circle. A live screenshot confirms this (`screenshot-react-lowpoly.png`: footer shows a handful of blank dots instead of a rich toolbar).

Additionally, `tool_button`/`tool_toggle`/`tool_collection` in [framework/core/rs/tools.rs](framework/core/rs/tools.rs) are called without labels, so every tool loses its tooltip (`title`/`label` default to `None`); and the 11 edit operations were flattened into loose top-level buttons instead of the old `edit` collection, and paint's UV/history operations were similarly flattened — both structural regressions versus `git show 32693795d:lowpoly/core/js/index.ts` (`buildLowpolyPlayToolbarTools`/`buildLowpolyPlayPaintToolbarTools`, lines 131-213).

Separately, the old editor's `windowEngagement()` (same file, lines 706-736) supplied a Snap/Smooth options rail, a command-line input (`engagementInput`/`engagementSubmit`), quick "possible engagements" (Extrude, Triangulate), and a status line — all rebuilt alongside the toolbar in `rebuildShellMode`/`rebuildPaintMode`. This is completely absent from the Rust port even though the framework already supports it end-to-end (`draw/plugin/rs/lib.rs` uses `App::window_kind_with_engagement` at line 1239, rendered via `windowEngagementToSpec` in [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx)).

## Phase 1 — Vendor the missing icons

In [ui/asset/script.ts](ui/asset/script.ts), add to `VENDORED_ICON_IDS` (all confirmed present in `node_modules/lucide-static/icons/`): `flip-vertical`, `flip-horizontal`, `git-branch`, `git-commit`, `git-merge`, `eraser`, `triangle`, `paintbrush`, `paint-bucket`, `pipette`, `scissors`, `unlink`, `undo`, `redo`, `pen-tool`, `magnet`.

Regenerate via `bun nx run @semio-tech/ui-asset:build` (runs `bun ./script.ts generate all`), which rewrites [ui/asset/icon/generated/icons.ts](ui/asset/icon/generated/icons.ts) (+ `.cs`/`.py` bindings + vendored `.svg` files + README) — do not hand-edit generated output.

## Phase 2 — Restore footer tool labels, icons, and grouping

In [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs):

- Extend `tool_button`/`tool_toggle`/`tool_collection` call sites with a label (these helpers in `framework/core/rs/tools.rs` are only used by lowpoly today, so it is safe to add a `label: impl Into<String>` parameter to each, setting both `label` and `title` on the resulting `ToolNode`).
- `edit_tools()`: add labels to the 4 selection toggles (Mesh/Vertex/Edge/Face) and 3 transform toggles (Move/Rotate/Scale, restoring icon `maximize-2` for Scale to match old); regroup the 11 edit operations back into a `tool_collection("lowpoly-tools-edit", "pen-tool", [...])` (matching old `edit` collection) with labels and restored icons:
  - Extrude `box`, Inset `square`, Flip Normals `flip-vertical`, Bevel `git-branch`, Loop Cut `git-commit`, Merge `git-merge`, Dissolve `eraser`, Subdivide `grid-3x3`, Triangulate `triangle`, Mirror `flip-horizontal`, Decimate `minimize-2` (already vendored).
- `paint_tools()`: add labels to the 4 paint toggles (Brush/Eraser/Fill/Eyedropper, restoring icons `paintbrush`/`eraser`/`paint-bucket`/`pipette`); regroup Unwrap/Mark Seam/Clear Seam into `tool_collection("lowpoly-paint-uv", "grid-3x3", [...])` with icons `grid-3x3`/`scissors`/`unlink`; regroup Undo/Redo into `tool_collection("lowpoly-paint-history", "undo", [...])` with icons `undo`/`redo`.
- Fix the document tree's face hover-reveal "Flip normal" action icon (line ~619, currently `flip-horizontal`) to `flip-vertical` so it matches the Edit Tools "Flip Normals" button that dispatches the same `flipFaces` command.

## Phase 3 — Restore the window engagement rail

In [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs) `create_lowpoly_app()`:

- Build a `WindowEngagement` mirroring old `windowEngagement()`: `options` = Move/Rotate/Scale toggles (`setTransformTool`, duplicating the footer transform toggles as in old code) + Snap (`magnet`, command `snap`) + Smooth (`sun`, command `toggleSmooth`); `input` = placeholder `"extrude, inset, mirror, decimate"` with `onSubmit: lowpoly_cmd("engagementSubmit", None)` (no `onChange` handler, matching old); `possible_engagements` = Extrude/Triangulate quick actions; `status` = a representative selection/tool summary string.
- Replace `.window_kind(LOWPOLY_PLAY_WINDOW_MAIN, ...)` and `.window_kind(LOWPOLY_PLAY_WINDOW_UV, ...)` with `.window_kind_with_engagement(...)` passing this engagement (both window kinds, matching old behavior of attaching the same engagement to Model and UV windows).
- Add an `"engagementSubmit"` handler in `handle_command` that trims/lowercases the submitted value and re-dispatches it as a command via `self.handle_command(&value, None, document_json, view_state)` (mirrors old `run(value)` in `f8376e848`); `engagementInput` needs no handler since it falls through to the existing no-operation default (matches old, which had no listener either).
- Note: unlike the old reactive TS engine, the Rust `PluginApp` trait has no per-render engagement hook (only `tools()` is dynamic), so `status`/`options` pressed-state will be a static snapshot rather than live-updating — this matches the existing limitation already present in `draw`'s engagement (`draw/plugin/rs/lib.rs:1228-1231` uses a fixed placeholder string), so it is consistent with current framework capabilities rather than a new gap.

## Phase 4 — Verification

- `cargo test -p lowpoly-plugin --lib` (extend/adjust the `edit_tools_include_extrude`/`paint_tools_include_brush` tests to also assert on the new collection ids `lowpoly-tools-edit`/`lowpoly-paint-uv`/`lowpoly-paint-history` and that labels are present).
- `bun nx run @semio-tech/ui-asset:build` and diff `ui/asset/icon/generated/icons.ts` to confirm the 16 new icons were vendored cleanly.
- Rebuild the lowpoly wasm plugin and run the React E2E sweep: `.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS/verify-react-playgrounds-e2e.ts --plugin lowpoly`.
- Manual live-browser screenshot of the footer in both Edit and Paint modes to confirm real icons render (no stray circles), tooltips show labels on hover, edit operations collapse into a single grouped button, and the window engagement rail (Snap/Smooth/command bar) appears at the top of the Model/UV windows.
- Update the ticket `.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS/important.md` with a summary of the icon-vendoring root cause and files touched.
  </plan>
  <todos>[{"id": "vendor-icons", "content": "Add 16 missing icon ids to VENDORED_ICON_IDS in ui/asset/script.ts and regenerate via bun nx run @semio-tech/ui-asset:build"}, {"id": "tool-helper-labels", "content": "Add label parameter to tool_button/tool_toggle/tool_collection in framework/core/rs/tools.rs"}, {"id": "restore-edit-tools", "content": "Restore labels, icons, and edit collection grouping in lowpoly edit_tools()"}, {"id": "restore-paint-tools", "content": "Restore labels, icons, and uv/history collection grouping in lowpoly paint_tools()"}, {"id": "fix-flip-icon-consistency", "content": "Fix document face flip-normal action icon to flip-vertical for consistency with edit tool"}, {"id": "restore-window-engagement", "content": "Build WindowEngagement (Snap/Smooth/command input/possible engagements/status) and wire via window_kind_with_engagement; add engagementSubmit handler"}, {"id": "verify-footer-fix", "content": "Run cargo tests, rebuild wasm, run React E2E for lowpoly, manually verify footer and engagement rail in browser, update ticket"}]</todos>
  </CreatePlan>
