---
name: Playground Chrome Reliability
overview: Fix the currently broken window chrome (navbar, window borders, tabs/chips, sizing) in the procedural3d playground, and close the architectural gap that let it happen by hardening the single shared app-registration path so no program can ever boot with a malformed window layout again.
todos:
 - id: repro
   content: Reproduce the procedural3d chrome bug live in an isolated dev instance; screenshot + compare to a known-good program
   status: completed
 - id: root-cause-fix
   content: Root-cause and fix the missing navbar/window border/chip/sizing regression in framework/renderer/wgpu/rs/lib.rs, coordinating with overlapping open tickets
   status: completed
 - id: harden-builder
   content: Add validation assertions to AppBuilder::build_definition() in framework/plugin/rs/lib.rs (non-empty window_kinds, unique ids, layout window_kind_id cross-references)
   status: completed
 - id: panel-group-enum
   content: Replace PanelTabDefinition.group free-form String with a closed PanelGroup enum shared across framework/core and both renderers
   status: completed
 - id: verify-all-plugins
   content: Run cargo test + wasm32 build across all 24 plugin crates to confirm every AppDefinition satisfies the new invariants; fix any failures
   status: completed
 - id: ticket
   content: Open/reopen the appropriate ticket via repo MCP, keep artifacts in its folder, close with full file summary
   status: completed
isProject: false
---

## Context

The screenshot shows the `procedural3d` playground (`semio · procedural · 3d`) rendering its Flow graph and Inspection panel without any chrome: no top navbar, no window borders/tabs/options chips, and windows appear oversized. Per your clarification, this is a **window-chrome** bug, not a 3D-content bug, and the fix you want is architectural: a single **centralized** mechanism through which every plugin-registers its window kinds/tabs/layout, strict enough that a broken layout like this becomes structurally impossible — not new end-to-end browser tests.

Investigation of the renderer shows the chrome IS already drawn by one shared function per surface:

- [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) `render_navbar` (top bar) and `render_stack` (per-window tab/border/options-chip chrome) are called uniformly for every plugin/window — there is no plugin-specific bypass.
- Two other chrome-layering bugs are already open today against this exact same code path: `FIX-WGPU-WINDOW-OPTIONS-CHIP-Z-ORDER` (chips compositing behind/above the wrong glass tier) and `MARQUEE-CROSSING-WINDOW-SELECTION` (scene disappearing due to draw-list layer conflicts). The procedural3d symptom is very likely the same class of regression (draw-list z-order / layout sizing), currently touched by in-flight work.

The real architectural gap is upstream of rendering, in **registration**: every one of the 24 plugins builds its `AppDefinition` through exactly one function, [framework/plugin/rs/lib.rs](framework/plugin/rs/lib.rs) `AppBuilder::build_definition()` (called via `App::builder(...)`). Today it has a single assertion (non-empty `document`). It does **not** verify:

- `window_kinds` is non-empty, ids are unique, `body_key`s are non-empty.
- Every `window_kind_id` referenced by `default_layout` / `named_layout`s (recursively through `WindowLayoutRoot` → `WindowLayoutAxisNode`/`WindowLayoutStackNode` → `WindowLayoutWindowNode`, see [framework/core/rs/lib.rs](framework/core/rs/lib.rs) lines 133-197) actually exists among the declared `window_kinds`.
- `panel_tabs[].group` is a free-form `String` matched against magic strings scattered across the wgpu renderer (`"workbench"`, `"left"`, `"details"`, `"right"`, `"display"`, `"document"` — see `panel_side_for_group`, `panel_toggle_icon_id` in `framework/renderer/wgpu/rs/lib.rs`) — a typo silently misroutes or drops a panel instead of failing to build.

This is exactly the kind of "wrong layout must not be possible" gap: a plugin can currently declare an inconsistent manifest (dangling window id, bad group string, empty measures) and it will boot with visibly broken chrome instead of refusing to build.

## Plan

### 1. Reproduce and fix the immediate regression

- Boot a clean, isolated `procedural3d` wgpu dev instance (avoiding the ports/processes other concurrent sessions are already using), screenshot it, and capture console/panic output.
- Compare against a known-good program (e.g. `draw` or `puzzle3d`) in the same renderer build to determine whether this is a global regression (affecting all 24 plugins right now) or specific to procedural3d's manifest/layout.
- Root-cause and fix in the shared chrome path (`render_navbar`, `render_stack`, window measure/sizing) in [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs), coordinating with (or reopening) the existing open tickets `FIX-WGPU-WINDOW-OPTIONS-CHIP-Z-ORDER` / `MARQUEE-CROSSING-WINDOW-SELECTION` if the root cause overlaps, per the repo's ticket-reuse rule.

### 2. Harden the one shared registration path (the actual "clean mechanism")

In `AppBuilder::build_definition()` ([framework/plugin/rs/lib.rs](framework/plugin/rs/lib.rs)), add assertions that make an inconsistent app manifest impossible to build (panic with a precise message identifying the offending plugin/window/tab), rather than silently rendering broken chrome:

- At least one `window_kind`; all `window_kind.id` unique and non-empty `body_key`.
- Recursively collect every `window_kind_id` referenced from `default_layout` and all `named_layouts`, and assert each one is declared in `window_kinds`.
- Assert every `panel_tab.body_key` is non-empty (or introduce an explicit "chrome-only" tab constructor for the few tabs — like the Windows/Layout display tabs — that legitimately have no body).

### 3. Replace the free-form panel `group: String` with a closed enum

- Introduce a `PanelGroup` enum (e.g. `Workbench | Details | Display | Settings`) once in [framework/core/rs/lib.rs](framework/core/rs/lib.rs), used by `PanelTabDefinition.group`.
- Update `AppBuilder::panel_tab(...)` to take `PanelGroup` instead of `impl Into<String>`, and update the wgpu (`panel_side_for_group`, `panel_toggle_icon_id`, `LeftPanelKind` matching) and react renderers to match on the enum instead of string comparisons.
- This removes an entire class of silent typo-driven "panel goes missing/misplaced" bugs at compile time instead of at runtime string-matching.

### 4. Verify across all 24 plugins

- Run `cargo test` (and the wasm32 build) across every plugin crate — each already constructs its `AppDefinition` in its own module/tests, so the new `build_definition()` assertions execute automatically for all 24 plugins with no new test files needed (per your "no end-to-end tests" direction).
- Fix any program whose manifest fails the new invariants (this is how latent issues beyond procedural3d, if any, get caught structurally).

### 5. Ticket workflow

- List goals via `repo://goals`, then reopen the most fitting existing open ticket (chip z-order or marquee) if the root cause matches, or open a new ticket otherwise, per the repo's ticket rules.
- Keep all logs/screenshots inside the ticket folder; close with a summary listing every file touched.

## Files most likely touched

- [framework/plugin/rs/lib.rs](framework/plugin/rs/lib.rs) — `AppBuilder::build_definition()` validation.
- [framework/core/rs/lib.rs](framework/core/rs/lib.rs) — new `PanelGroup` enum, `WindowLayout*` cross-reference helpers.
- [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) — root-cause chrome fix (`render_navbar`, `render_stack`, sizing) + enum-based group matching.
- [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx) — mirror enum-based group matching for parity.
- Any plugin crate whose manifest fails the new assertions (found in step 4).
