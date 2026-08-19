# Wgpu Glue Retained-Node and Fuel-Quota Gaps (Z1)

## RetainedSurface.node — bug, fixed

**Finding:** `RetainedSurface.node` was write-only. On desync (`apply_ui_patch` else branch), `pending_rejections` was queued but the retained snapshot was never re-inserted into the per-turn `out` map.

**Downstream effect:** `ProgramBridge/🧊️component.rs::render_with_document` reads `ExchangeOutcome::surfaces.get(body_key)`. A missing key yields `"plugin has not yet painted surface … this turn"` — not a silent freeze on the last GPU frame. The shell does not keep a separate per-surface `UiNode` cache across turns for plugin bodies; it depends on each `SurfaceVisible` exchange returning the resolved node.

**Design intent (item 4):** Reuse the previous full-body tree on rejected/missed patches while queueing `PatchRejected`.

**Fix:** On any non–full-body patch path, after `pending_rejections.insert`, clone `retained.node` into `out` when a prior snapshot exists. Merged the two identical desync branches. Removed `#[allow(dead_code)]` on `node`.

## RegistryQuotas.fuel — not a bench dimension, removed from schema

**Finding:** `fuel` was deserialized from the scale-fixture registry but `turn_budget_of` always used `BENCH_FUEL` (200M). Siblings `deadline_ms` / `max_effects` / `max_patch_bytes` / `max_frames` are record-derived and enforced via `actor_budget_from_turn_budget` on every `tick_and_dispatch` grant.

**Decision:** Do not wire per-record fuel — debug wasmtime + wasip2 overhead dwarfs the generator's 100K–900K production-shaped values and would fuel-starve almost every turn (documented rationale preserved on `BENCH_FUEL`).

**Fix:** Removed `fuel` from `RegistryQuotas`, from `ScaleFixtureRecord.quotas` in dev `📜️script.ts`, and from `scaleFixtureRecord()` output. Regenerated `🧫️fixtures/🔌️scale/🤖️generated/🔣️registry.json` (2550 records, seed 1). `scale-fixture check` passes.

## Verification

- `cargo check -p semio-framework-os-renderer-wgpu` — success; no `dead_code` on the two former gaps
- `bun nx run workspace:scale-fixture-check` — fresh

## Files

- `🧰️framework/…/🧊️wgpu/📦️glue.rs`
- `🧰️framework/…/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/…/🧫️fixtures/🔌️scale/🤖️generated/🔣️registry.json`
