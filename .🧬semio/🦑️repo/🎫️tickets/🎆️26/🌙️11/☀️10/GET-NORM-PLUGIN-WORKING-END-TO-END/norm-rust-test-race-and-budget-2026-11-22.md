# Norm Rust Test Race, Budget and Failure Retirement (2026-11-22)

Result: `cargo test -p semio-hub-norm` (prototype profile, parallel threads) is green, 15 of 15. The surface_render binary (2 tests in parallel) finishes in 9.62 s, down from 40-44 s.

## 1. Roster race (root cause was production ordering, not a global roster)
- The owner mutation roster is per-assembly (`self.owner_mutations`); the `OnceLock` registries are `cfg(test)` / contribution-only and not involved.
- Real cause: `WireMutationRosterEntry::owner` resolves payload schemas through `registered_input_schema_document`, the process-wide schema catalog. `PluginBuilder::try_build` built the roster (`extend_contributions`) BEFORE `publish_declared_catalogs`, so entries were `Declared` or `Refused` depending on what a concurrent assembly had published, and the editor and viewer providers of one assembly could disagree (`conflicting owner mutation roster entry ... vdi3805/v1#add-curve`).
- Fix: `extend_contributions` now runs after `publish_declared_catalogs` in `🏗️builder/🦀️.rs`, so every roster row is resolved against this assembly's own published schemas (deterministic, and also correct for single-instance production, where rows were previously resolved before their schemas existed).

## 2. Budget
- Instrumented every render: each call cost a constant ~0.5 s, including refused oversize renders (8 apps x ~26 renders = ~40 s). Cause: `render_registered_fixture_app` admits `FixtureUiArena` per call, and the outermost admit opens and closes a whole real UI arena Source (up to 100k funded turns).
- Fix: the render test holds one `FixtureUiArena::admit()` for the whole test; per-call admits are nested and free. No framework behaviour changed.

## 3. Failure retirement
- Added `OriginalFixtureScope::retire_failure(OriginalTurnFailure)` in `🧪️testing/🎟️caller/🦀️.rs`: paid, bounded retirement of a returned refusal through the scope's own context (`RetainedCloneClose` + `close_original_effect_step`, same pattern as the failure-law tests).
- `std::mem::forget` in `🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs` replaced by `scope.retire_failure(refusal)`.

## Browser rebuild
None: only Rust framework test helper, plugin builder ordering and a test file changed. (The builder ordering touches the plugin assembly, so the wasm component embeds it, but roster content for browser use only becomes more correct; rebuild of `norm` already pending covers it.)
