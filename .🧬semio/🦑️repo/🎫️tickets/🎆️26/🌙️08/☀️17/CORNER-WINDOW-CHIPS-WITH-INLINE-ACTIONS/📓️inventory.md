# Corner Window Chips — Inventory

## Goal
`R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS`

## Current architecture
- React `WindowChrome`: top-left title chips + U-gap + top-right controls (`enlarge`/`close`)
- React `ModeDockStack`: all tabs in one top-left strip; stack-level Focus/Close
- Silhouette geometry already supports N chip spans per edge
- Footer already has bottom-left / bottom-right chip cells
- wgpu `Dock`: tabs left + controls chip right; no corner groups
- TUI: title tab top-left + controls tab top-right + flat stack strip

## Target
- Per-tab inline actions: focus, new window, close (tooltips + hotkeys)
- Four corner tab groups per stack; drag between corners
- One `activeId` per stack across all corners
- Schema field `corner` on window layout nodes (default `topLeft`)
- React + wgpu + TUI end-to-end

## Key files
- `framework/ui/packages/rust/targets/wgpu/component.rs` — WindowLayout schema
- `framework/manifest/component.ts` — TS WindowLayout mirror
- `framework/ui/packages/typescript/targets/react/index.tsx` — WindowChrome + runtime layout
- `framework/ui/elements/Canvas/component.tsx` — Mode dock
- `framework/os/renderer/engine/elements/ShellHelpers/component.tsx` — layout seed
- `framework/os/renderer/engine/elements/ShellHost/component.tsx` — Mode wiring
- `framework/os/renderer/engine/elements/Dock/component.rs` — wgpu dock
- `framework/ui/tui/component.rs` + `framework/ui/elements/Window/tui component.rs` — TUI chrome
