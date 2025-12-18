---
slug: UI-STATE-MACHINE
summary: 'Finish the UI state machine with proper states, guards, menus, and context'
prompt: 'Finish the UI state machine with proper states, guards, menus, and context'
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.891Z'
commit: '0000000000000000000000000000000000000000'
iterations: []
---

# Previously

The existing `machines.ts` has a unified `sketchpadMachine` that handles:

- Y.js sync for Kit data persistence
- App state for Home, Kit, Type, Design, Quality apps
- Transaction management for undo/redo
- Tutorial state

However, the machine is "flat" - all events are handled at the root level without proper hierarchical state modeling. The user started designing a proper statechart with:

- Navigation hierarchy: Idle → Home → Kit → Design/Type/Quality → Docs
- Parallel states within each app for interaction modes
- Context menu states for different entities

# Plan

1. Create a new `uiMachine` in `machines.ts` that models the UI navigation and interaction states
2. The machine should have:
   - Top-level navigation states: `idle`, `home`, `kit`, `design`, `type`, `quality`, `docs`
   - Each app state has parallel regions for:
     - `interaction`: Idle → Hovered → Selected + context menu substates
     - `panels`: Manages panel visibility
     - `tool`: Active tool state (for Design/Type)
   - Context menu substates for specific entity types (Piece, Connection, Type, Design, Port, etc.)
3. Add proper guards for state transitions
4. Keep the existing data management (Y.js sync) in `sketchpadMachine`, this new machine focuses on UI states

# Changes

- Added `uiMachine` to `machines.ts` with proper hierarchical state modeling
- Follows the user's fragment structure with corrections (typo: `menu.cloe` → `menu.close`)
- Uses parallel states for concurrent concerns (interaction, panels, tools)
- Implements proper context menu states for each entity type
