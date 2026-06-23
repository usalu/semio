# Write Toolbar Specs for Sketchpad

- **Goal**: SKETCHPAD-IMPROVEMENTS
- **Status**: open
- **Client**: copilot-chat
- **LLM**: opus-4-6

## Prompt

Write comprehensive specs for the sketchpad toolbar system documenting architecture, components, registration, layout, sizing, groups, sub-tools, per-app sections, and interactions.

## Plan

1. Read all toolbar-related code across the sketchpad codebase
2. Document toolbar architecture, component hierarchy, registration contract, layout system, sizing normalization, group mechanics, sub-tool dropdown, per-app toolbar sections, and interaction semantics
3. Write specs in the existing `compose/js/sketchpad/README.md` under `### Toolbar`

## TODOs

- [x] Gather toolbar implementation context from elements.tsx, Sketchpad.tsx, shared.ts, and all app files
- [x] Write comprehensive toolbar specs into README.md
- [x] Verify no TS errors introduced

## Changes

- `compose/js/sketchpad/README.md`: Replaced the `### Toolbar` section with comprehensive specs

## Summary

Wrote comprehensive toolbar specs covering: dual-zone layout (tools zone + settings zone), seam-anchored positioning, component hierarchy (ToolbarZone > ToolbarGroup > ToolbarItem > ToolbarDivider), registration contract via PanelSection.toolbarGroup, group ordering and rendering, sub-tool dropdown mechanics, settings zone content routing, CSS sizing normalization via data-slot selectors, per-app toolbar sections (Home, Kit, Design, Type, Quality, Feedback), toolbar state management, and interaction invariants.
