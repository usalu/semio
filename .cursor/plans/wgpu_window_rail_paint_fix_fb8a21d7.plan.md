---
name: WGPU Window Rail Paint Fix
overview: Fix the WGPU renderer so the per-window "Command" (engagement) and "Window Options" (measures) foldable rails are actually visible and match the React reference renderer's placement, default state, and visibility rules.
todos:
 - id: open-ticket
   content: Open a repo MCP ticket for the WGPU window rail paint-order/parity fix
   status: completed
 - id: fix-paint-order
   content: Route folded/collapsed rail chip draws to the overlay DrawList instead of the main draw list in shell.rs
   status: completed
 - id: fix-side-swap
   content: Swap Window Options rail to right edge and Command rail to left edge to match React
   status: completed
 - id: fix-default-fold
   content: Default measures_folded to true to match React's default folded state
   status: completed
 - id: fix-visibility-gating
   content: Add windowEngagementChromeVisible-equivalent gating and hide-when-measures-expanded guard to the engagement rail
   status: completed
 - id: rebuild-and-verify
   content: cargo check, rebuild wgpu wasm, run verify-wgpu-playgrounds-e2e.ts on s and draw, screenshot-compare vs React
   status: completed
 - id: close-ticket
   content: Close the ticket with a summary of files changed
   status: completed
isProject: false
---

## Root cause (confirmed by reading the code directly)

The rails are **not missing from the code** — `render_window_measures_rail` (`framework/renderer/wgpu/rs/shell.rs:3989`) and `render_window_engagement_rail` (`framework/renderer/wgpu/rs/shell.rs:4269`) already exist, are already wired into the per-window draw loop (`shell.rs:3497-3522`), and were marked "completed" in a prior plan ([.cursor/plans/wgpu_renderer_chrome_parity_a00a09a5.plan.md](.cursor/plans/wgpu_renderer_chrome_parity_a00a09a5.plan.md)). The bug is a **paint-order/z-layer bug** that makes the folded/collapsed chip invisible:

- When folded, the "Window Options" chip is drawn directly onto the shared `draw` list at `content.x, content.y + 8` (`shell.rs:4011-4029`), and `content` is **not shrunk** (no space reserved) — same for the collapsed "Command" chip at `shell.rs:4291-4315`.
- Immediately afterward, in the same per-window loop iteration, `render_window_content` (`shell.rs:3654`) runs the active scene (node-graph, text editor, world3d, etc.), which paints an **opaque `theme.canvas_clear` background** over that same `content` rect — e.g. `framework/renderer/wgpu/rs/scenes.rs:697` and `:1017` — onto the **same `draw` list**, after the chip.
- Because both are on the same list and the scene paints later, the scene background covers the chip. Both rail functions already receive an `overlay: &mut Option<&mut DrawList>` parameter (used elsewhere for nested widget popovers) but never use it for the folded/collapsed chip itself — that list is composited strictly after everything in `draw` (`render_chrome` in `shell.rs:2705-2741`: `draw` painted first via `render_main_window`, then `render_overlay`/`render_tree_drag_overlay` on `overlay` last).

**Fix**: route the folded/collapsed chip draw calls (`render_chrome_group(draw, ...)` at `shell.rs:4021` and `:4306`) onto the `overlay` list instead of `draw` (falling back to `draw` only if no overlay slot is available), so the chip always paints on top of window content. Hit-target registration (`input.register_hit`) is independent of the draw list and needs no change.

## Secondary parity gaps vs React reference (`ui/js/react/index.tsx`)

Once rails are actually visible, these divergences from the React chrome remain and should be fixed for true parity:

1. **Sides are swapped.** React: Window Options overlay is top-**right** (`ui/js/react/index.tsx:4179-4181`), Command/engagement overlay is top-**left** (`:4201-4203`). WGPU currently does the opposite: measures rail anchored at `content.x` (left, `shell.rs:4020/4033`), engagement rail anchored at `content.x + content.w - ...` (right, `shell.rs:4300-4301/4318-4323`). Swap both anchors in `shell.rs` to match.
2. **Default fold state differs.** React measures start **folded** (`measuresFolded` defaults `true`, `ui/js/react/index.tsx:15334`). WGPU defaults `measures_folded.get(window_id).unwrap_or(false)` (`shell.rs:4005`) — starts **unfolded**. Change the fallback default to `true`.
3. **Missing visibility gating on the Command rail.** React only shows the engagement chrome when `windowEngagementChromeVisible` is true (session active, input has a value, or the zone is activated — `ui/js/react/index.tsx:14686-14695`), and hides it entirely while measures are expanded (`!measuresExpanded` guard, `:15568`). WGPU's `render_window_engagement_rail` shows the chip whenever any engagement data exists (`shell.rs:4282-4290`) with no measures-expanded check. Add the equivalent gating in `shell.rs` before rendering the collapsed chip and the expanded rail.

## Files to change

- [framework/renderer/wgpu/rs/shell.rs](framework/renderer/wgpu/rs/shell.rs)
  - `render_window_measures_rail` (~3989-4120): draw folded chip to `overlay` list; swap anchor to right edge; default fold to `true`.
  - `render_window_engagement_rail` (~4269-4427): draw collapsed chip to `overlay` list; swap anchor to left edge; add visibility gating (mirroring `windowEngagementChromeVisible`) and measures-expanded guard.

## Verification

- `cargo check -p semio_wgpu_renderer` (or equivalent crate name) after edits.
- Rebuild the WGPU wasm bundle (`framework/renderer/wgpu/script.ts wasm` per existing convention).
- Run the existing WGPU E2E suite ([.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts](.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts)) against `s` and `draw` (the current producers of `measures`/`engagement` data), asserting: folded/collapsed chips are visible and clickable on load, unfold/expand shows the rail contents on the correct (React-matching) side, and window content underneath no longer paints over the chip.
- Manual screenshot diff of `s` and `draw` in WGPU vs React to confirm side/placement now match.
- Per workspace rules, this work happens inside a ticket (`.repo/🎫/26/07/06/...`) — open a new ticket for this fix (no existing open ticket covers it; `WGPU-THEME-CURSOR-PARITY` is cursor-icon-only) and close it with a summary listing the files touched.
