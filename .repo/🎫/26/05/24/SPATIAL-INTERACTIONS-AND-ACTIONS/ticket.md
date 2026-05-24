# Spatial Interactions And Actions

**Repo MCP:** unavailable in this session; `ticket_open` / `repo://goals` skipped (same pattern as SPATIAL-COMMANDS-GENERALIZATION).

**Work:** Rename command → interaction across TS + fixtures + schema; `ActionSpec` → `EffectSpec` with `effects` arrays; register pure `Action`s + `ActionRegistry` / `InteractionRegistry`; collapse `commit.operation` to `{ kind: "action", action, params }`; remove legacy JSON normalizers and `history.excludeEvents`; delete orphaned `spatial/fixtures/factory.json` variants.

**Scratch:** `migrate-fixtures.mjs` in this folder (fixture mass-migration).
