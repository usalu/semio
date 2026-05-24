# Spatial Interactions And Actions

**Repo MCP:** unavailable in this session; `ticket_open` / `repo://goals` / `ticket_close` skipped (same pattern as SPATIAL-COMMANDS-GENERALIZATION).

**Work:** Rename command → interaction across TS + fixtures + schema; `ActionSpec` → `EffectSpec` with `effects` arrays; register pure `Action`s + `ActionRegistry` / `InteractionRegistry`; collapse `commit.operation` to `{ kind: "action", action, params }`; remove legacy JSON normalizers and `history.excludeEvents`; delete orphaned `spatial/fixtures/factory.json` variants.

**Scratch:** `migrate-fixtures.mjs` in this folder (fixture mass-migration).

## Summary (handoff completion)

- **`spatial/js/core/index.ts`:** Interaction runtime, `EffectSpec` + `{ op: "action" }`, `ActionRegistry` / `InteractionRegistry`, tests extended (action override, registry parity with `buildBoxInteractionSpec`, interaction-local undo snapshots).
- **`spatial/js/machine-stately/index.ts`:** `StatelyStateEngine.send` passes `kernel`, `actions`, `topology` to `applyTransition`; `StatelyAdvance.interactionKind`; catalog view `interactionId` / `interactionVersion`.
- **`spatial/js/machine-stately/machine.json`:** Regenerated field names aligned with catalog view.
- **`spatial/js/renderer-r3f/index.tsx`:** `InteractionRepl`, `InteractionCanvas`, `InteractionSpatialView`, `SpatialPick*`, `useInteractionRuntime` / `useInteractionSnapshot`; REPL props `interactionId` / `onInteractionId`.
- **`spatial/js/renderer-r3f/play/main.tsx`:** Preset + runtime wiring for interactions.
- **`spatial/schema/json/interaction.json`:** Renamed from `factory.json`; `spatial.interaction/v1`, `effects`, unified commit operation, `{ op: "action" }` effect, removed `history` and `box.transform`.

## Verification

- `bun nx run @spatial/js-core:test` — **44** tests passed.
- `bun nx run @spatial/js-machine-stately:test` — **8** tests passed.
- `bun nx run @spatial/js-renderer-r3f:test` — **12** tests passed.
- Lint targets for these projects are not defined in Nx (`:lint` missing).

## Reopened: Renderer Selection And Hover

**Repo MCP:** still unavailable in this session (`repo://goals` server not registered), so this ticket note tracks the follow-up.

**Work:** extend `spatial/js/renderer-r3f/index.tsx` with topology-kind pick toggles, hover/selection highlighting, wire/shell/cell-specific target bounds, and click disambiguation for overlapping selectable 3D targets.
