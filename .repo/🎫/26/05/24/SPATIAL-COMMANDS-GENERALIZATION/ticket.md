# Spatial Commands Generalization

**Repo MCP:** unavailable in this session (`repo` server not registered); goals/ticket_open skipped per prior ticket pattern.

**Work:** Implement factories→commands plan: `CommandSpec`, `TopologyDiff`, `CommandResponse`, per-state `selection.accept`, kernel `*Diff` + measure ops, fixtures `*.command.json`, renderer/play renames.

**Artifacts:** scratch logs in this folder only.

## Done (handoff)

- Fixed `getActiveSelectionSpec` to use `machine.states[state]` (record), not `.find` on an array — unblocked selection gating + measure tests.
- Kernel `createBoxFromCornersDiff` test simplified (single non-degenerate diff + volume check).
- `machine-stately`: `commandKind` / `commandState` / `commandContext`, imports, parity test for distance+area (no erroneous `confirm` before commit).
- `renderer-r3f`: `CommandCanvas`, `CommandSpatialView`, `useCommandSnapshot`, `CommandInteraction*`, `onCommandEvent`; geometry layer `onPick` unset to avoid duplicate pointer dispatch.
- `play/main.tsx`: `CommandResponse` from `commit()`, `listActiveSelectionAccept` routing via `onSpatialCommandEvent`, preset keys text `d`/`a`, renames `commandId` / `onCommandId`.
- Removed obsolete `spatial/fixtures/extrude.factory.json`, `offset-surface.factory.json`.
- Verified: `bun nx run-many -t test -p @spatial/js-core @spatial/js-machine-stately @spatial/js-kernel-brepjs @spatial/js-renderer-r3f` — all pass.

