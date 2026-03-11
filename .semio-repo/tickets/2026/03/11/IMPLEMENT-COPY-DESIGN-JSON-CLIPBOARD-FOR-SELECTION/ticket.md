---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Verified Design diagram Cmd/Ctrl+C handler copies Design JSON ({pieces, connections}) matching design.json structure. No selection copies all pieces+connections. Selection copies selected pieces+connections between them. Build passes with 0 TS errors, 15/15 unit tests pass.
## Changes
- `semio/js/sketchpad/Kit.tsx`:
  - Added `hasAnySelection(selection)` utility to check if any dimension of KitAppSelection has items.
  - Added `buildSelectionKit(kit, selection)` function that creates a filtered Kit JSON subset from selection with transitive dependency resolution.
  - Added keyboard handler in `KitDiagramInner` for Cmd/Ctrl+C that copies selection-filtered Kit JSON or full kit JSON.
  - Made diagram wrapper div focusable with auto-focus.
- `semio/js/sketchpad/Design.tsx`:
  - **REVISED**: `handleDiagramKeyDown` Cmd/Ctrl+C handler now builds a `{ pieces, connections }` payload from the active design data.
  - No selection → copies all pieces + all connections.
  - Selection → copies selected pieces + connections where both connected and connecting pieces are in the selected set, plus explicitly selected connections.
  - Uses `resolveSelectionEntryGuid` for robust GUID extraction from selection entries.
  - Passes payload directly to `copyJsonToClipboard` for clipboard write.

## Log
- Gathered repo context via `./semio-repo/cli/cli tree "copy clipboard kit json diagram"`.
- Found existing closed ticket `2026/03/11/IMPLEMENT-COPY-JSON-CLIPBOARD-COMMAND`.
- Opened new ticket under goal `SKETCHPAD-IMPROVEMENTS`.
- Read Kit.tsx: KitAppSelection interface, KitDiagramInner component, selection hooks, diagram rendering.
- Read Sketchpad.tsx: `copyJsonToClipboard` command handler, KitStore.snapshot() returning Kit object.
- Read semio.ts: KitSchema matching kit_metabolism.json top-level structure (guid, name, types, designs, ports, tags, concepts, files, folders, authors, qualities, etc.)
- Verified kit_metabolism.json top-level keys match Kit schema.
- Implemented `buildSelectionKit` with transitive dependency resolution.
- Implemented Cmd/Ctrl+C keyboard handler in Kit diagram.
- Implemented Cmd/Ctrl+C keyboard handler in Design diagram.
- TSC: 0 errors.
- Unit tests: 15/15 pass.
- Reopened: verified implementation against user requirements. Design diagram Cmd/Ctrl+C at Design.tsx:7734-7762 copies `{ pieces, connections }` matching design.json structure. No selection = all pieces + connections. Selection = selected pieces + connections between them. Build passes (0 errors). 15/15 unit tests pass.

## Todos
- [x] Understand clipboard code
- [x] Understand kit JSON export format
- [x] Understand selection state in diagram
- [x] Implement selection-based clipboard copy in Kit diagram
- [x] Implement Cmd+C in Design diagram (Kit JSON - original, replaced)
- [x] CORRECTION: Change Design diagram Cmd+C to copy Design JSON (pieces + connections)
- [x] No selection = all pieces + connections from active design
- [x] Selection = selected pieces + connections between them
- [x] JSON matches design.json structure: `{ pieces: [...], connections: [...] }`
- [x] Run TSC (0 errors) and unit tests (15/15 pass)
- [x] Update ticket and close

## Plan (Revised)
1. ~~Read existing clipboard implementation in Sketchpad.tsx.~~
2. ~~Understand Kit selection state (KitAppSelection) and diagram nodes.~~
3. ~~Verify kit snapshot matches kit_metabolism.json structure.~~
4. ~~Add `buildSelectionKit` helper for creating filtered Kit JSON from selection.~~
5. ~~Add Cmd/Ctrl+C keyboard handler in Kit diagram (KitDiagramInner).~~
6. **REVISED**: Change Design diagram Cmd+C handler to build Design JSON (`{ pieces, connections }`) from design data, not Kit JSON.
7. When no selection → all pieces + all connections. When pieces/connections selected → selected pieces + connections involving selected pieces.
8. Validate with TSC and unit tests.
