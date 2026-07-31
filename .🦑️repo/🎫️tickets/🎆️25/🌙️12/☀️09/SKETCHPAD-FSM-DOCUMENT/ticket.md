# Ticket

## Todos

# Previously

The sketchpad has two state machines:

1. `sketchpadMachine` - handles data and Y.js sync with flat event handling using guards
2. `uiMachine` - has hierarchical navigation states (idle → home → kit → design/type/quality) with parallel interaction states

The `sketchpadMachine` uses guards to control event acceptance but doesn't enforce proper state constraints. For example:

- `DESIGN.CLEAR_HOVER` only checks `hasDesignHover` but doesn't verify we're in Design state
- `DESIGN.DELETE_SELECTED` requires selection but doesn't verify Design app is open
- Context menu events can fire without hover state
- Transaction abort can happen without active transaction (guarded but flat)

The `uiMachine` already has good structure but isn't connected to the actual data flow.

# Plan

1. Merge `uiMachine` hierarchical structure into `sketchpadMachine`
2. Replace flat event handling with state-scoped transitions
3. Add parallel states for:
   - Navigation (home/kit/design/type/quality/docs)
   - Interaction per app (idle/hovered/selected/contextMenu)
   - Modal (none/commandPalette/search)
   - Drag (idle/dragging)
   - Transaction (idle/active)
4. Move app-specific events into their respective state nodes
5. Use guards only for data validation, not state constraints
6. Run sketchpad tests to verify no regressions ✅️ All 5 tests passed

# Changes

## Modified `sketchpadMachine` to use hierarchical navigation states

### Root-level parallel structure

- Added `type: "parallel"` to root machine
- Created `navigation` parallel state with hierarchical sub-states: `home`, `kit`, `design`, `type`, `quality`, `docs`

### Global events (always available)

- Navigation: `NAVIGATE`, `NAVIGATE_BACK`, `NAVIGATE_FORWARD`
- Settings: `SET_THEME`, `SET_LANGUAGE`, `SET_EXPERTISE`, `SET_MODE`, `SET_LAYOUT`, `TOGGLE_FULLSCREEN`, `SET_PANEL_SIZE`
- Sync: `CHANGE`, `Y_UPDATE`
- Tutorial: `TUTORIAL.*` events
- Transactions: `TRANSACTION.*` events with guards

### State-scoped events (only available in respective states)

- **home**: `HOME.*` events
- **kit**: `KIT.SYNC`, `KIT.TOGGLE_PANEL`, `KIT.SET_*`, `KIT.SELECT_*`, `KIT.DESELECT_*`, `KIT.CLEAR_*`
- **design**: `DESIGN.SYNC`, `DESIGN.TOGGLE_PANEL`, `DESIGN.SET_*`, `DESIGN.SELECT_*`, `DESIGN.DESELECT_*`, `DESIGN.CLEAR_*`, `DESIGN.FOCUS_*`, `DESIGN.DELETE_SELECTED`
- **type**: `TYPE.SYNC`, `TYPE.TOGGLE_PANEL`, `TYPE.SET_*`, `TYPE.SELECT_*`, `TYPE.DESELECT_*`, `TYPE.CLEAR_*`, `TYPE.FOCUS_*`, `TYPE.HOVER_*`, `TYPE.ADD_*`, `TYPE.REMOVE_*`
- **quality**: `QUALITY.TOGGLE_PANEL`, `QUALITY.TOGGLE_BENCHMARK`

### Navigation INIT events (available globally for direct URL navigation)

- `KIT.INIT` → transitions to `kit` state
- `DESIGN.INIT` → transitions to `design` state
- `TYPE.INIT` → transitions to `type` state

### Added navigation state selectors

- `selectNavigationState` - returns current navigation state
- `selectIsInHome`, `selectIsInKit`, `selectIsInDesign`, `selectIsInType`, `selectIsInQuality`, `selectIsInDocs`

### Constraint enforcement

- **DESIGN.DELETE_SELECTED** now requires `hasDesignSelection` guard AND being in design state
- App-specific events are only processed when in the correct navigation state
- This prevents invalid state transitions (e.g., selecting a piece when not in design view)

## Documentation updates

- Updated `AGENTS.md` with new hierarchical state machine architecture
- Marked `uiMachine` as legacy (functionality merged into `sketchpadMachine`)

## Test results

All 5 sketchpad tests pass:

- Home
- Kit
- Type
- Design
- Docs

## Changes

## Log

## Summary

# Summary

Migrate sketchpad FSM to hierarchical states with proper constraints
