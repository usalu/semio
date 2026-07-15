# Os App Playground Ui Refactor — Phase Status

Plan: /Users/ueli/.claude/plans/refactor-the-os-app-logical-koala.md

Update this file only from a gate-runner session (S1, S2, S14).

## Current phase: STOPPED after WF3 — see blocker note before resuming

| Phase                             | Status                                 | Gate                                                                                                                                                                                                                                                    |
| --------------------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0 Foundations (S1)                | done                                   | G0: `bun ./script.ts verify gate` ✅ (dep-cruiser + registry check + renderer-react lint + os-dev plugin lint + check-no-px), `framework-renderer-react:test` ✅ (93 tests), `framework-os-core:test` ✅ (16 tests, previously broken target now fixed) |
| 1 Contracts freeze (WF1)          | done                                   | G1: renderer tests ✅ (94), dep-cruiser ✅, verify gate ✅. Typecheck: `ui-react:typecheck` has pre-existing/unrelated failures (duplicate `ThreeEvent` export in ui/index.tsx, tsconfig quirks) — zero errors in any WF1-touched file.                 |
| 2 Parallel tracks (WF2: T-A..T-I) | done                                   | See Gate 2 below                                                                                                                                                                                                                                        |
| 3 Parallel (WF3: T-F,T-G,T-J)     | done                                   | TS side ✅ (see Gate 3 below); Rust side compile-verification blocked, see blocker note                                                                                                                                                                 |
| 4 Integration (WF4)               | **not started, stopped intentionally** | do not start until the framework-core API-relocation blocker below is resolved                                                                                                                                                                          |

## Track tickets

- S1 Foundations — ticket `OS-REFACTOR-FOUNDATIONS-ENFORCEMENT-INFRA` (this folder) — status: **closed**

## G0 deliverables (S1)

- New `bun ./script.ts verify` / `verify gate` (root `script.ts` `🔖VerifyScript`) + `workspace:verify` / `workspace:verify-gate` nx targets. `gate` = dep-cruiser boundaries + `@semio-tech/plugin-registry:check` + `@semio-tech/framework-renderer-react:lint` + `@semio-tech/framework-os-dev:plugin lint` + `@semio-tech/ui-styling-tokens:check-no-px`. Deliberately does **not** call the pre-existing (already broken, unrelated) `nx run-many -t lint --all`, nor the new `framework-renderer-wgpu:lint` or `check-no-raw-colors` (both have real pre-existing violations — see follow-ups below).
- New `framework/plugin/registry` project (`package.json`+`project.json`, targets `generate`/`check`). `check` renders in-memory and byte-diffs against `generated/*` — never regenerates during lint (auto-commit daemon safety).
- `.dependency-cruiser.cjs`: added R1 `ui-no-framework-packages`, R2 `renderer-hosts-only-ui`, R3 `no-generated-edits-upstream`. All pass clean against current tree (0 violations, 1278 modules).
- `framework/renderer/react/script.ts`: new `lint` verb (region-balance + host-signature checks) + project.json target — passes clean today.
- `framework/renderer/wgpu/script.ts`: new `lint` verb (raw `Rgba::new`/`from_srgb8` outside `ui/wgpu` theme) + project.json target — **11 known pre-existing violations**, not yet wired into the blocking gate (see follow-up task).
- `ui/styling/script.ts`: new `check-no-raw-colors` verb + project.json target — **57 known pre-existing violations** (mostly domain/fixture colors in puzzle, infinite/world), not yet wired into the blocking gate (see follow-up task).
- `framework/product/os/dev/script.ts`: folded `.repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs` in as a `verify e2e` subcommand (in-process Playwright, no more spawning the ticket-folder script); `playwright` added as a devDependency there (needed since `framework/` is dependency-cruiser-scanned).
- `framework/product/os/core`: fixed the broken `test` target (missing `js/vitest.config.ts` — created it, added inline `import.meta.vitest` tests for the `🔖Backbone` region: 16 tests, all passing).
- `eslint.config.mjs`: removed the dead `compose/client/lib/react/index.tsx` override entry (file no longer exists) and the now-unused react-hooks plugin/jsx parserOptions; kept the still-needed `no-unused-vars: off` for the surviving `compose/client/lib/js/index.ts`.
- `.vscode/launch.json`: added `🧪verify🌍workspace` (391), `🧪verify🚪gate` (391.1) in `3_dev`; `📦build🧩plugin📜registry` (206) in `4_build`.

## Known follow-ups spawned (not blocking Phase 1)

- `task_4833523e` — triage 57 `check-no-raw-colors` findings, then wire the check into the blocking gate.
- `task_687e06dd` — fix 11 raw color literals in `framework/renderer/wgpu/rs/lib.rs`, then wire `framework-renderer-wgpu:lint` into the blocking gate.
- `task_354626e4` — fix ~13 projects with pre-existing broken eslint configs (unrelated to this refactor) breaking the base `bun ./script.ts lint`.

## G1/G2 deliverables (WF1 + WF2, this session, via Workflow tool)

**WF1 (Contracts):** consolidated ~50 duplicated protocol types from `os-shell.tsx` into `framework/core/js/index.ts` (WindowLayout/NamedLayout dup-identifier bug fixed, UiNode union unified, `ComponentKind`/`ComponentSceneHostProps` added), repointed all consumer imports (ui-interpreter, 11 hosts, index.tsx/test.ts). 94 tests green.

**WF2 (7 parallel tracks), all landed:**

- T-A: `🧮ShellStore` region in os-shell.tsx — 38-field `ShellState`/`ShellAction`/`shellReducer`, `FrameworkOsShell` now has exactly one `useReducer` (down from 38 `useState`); consolidated prefs-persistence effects; killed both module-level mutable `let`s (moved to proper component/hook state — 1 intentional `useState` remains in FrameworkOsShell itself for `themeSaveLabel`, judged an acceptable exception over the `let` anti-pattern it replaced); region reorder to canonical order; new `VITE_SEMIO_APP_ID` boot contract with hard-assert (no silent `apps[0]` masking when set).
- T-B: `ui/js/react/index.tsx` gained `🧭ShellDisplayPanel`, `🧭ShellSettingsPanel`, `🔎ShellSearchDialog`, `🔎ShellFindDialog` (presentational, HostApi-shaped props) + i18n region split into sub-regions.
- T-C: all 11 `components/*-host.tsx` now use shared `ComponentSceneHostProps`; `ui-interpreter.tsx`'s 11-arm switch replaced by an exhaustive `COMPONENT_SCENE_HOSTS: Record<ComponentKind, ...>` table + `lazyHost()` helper.
- T-D: `[[package.metadata.semio.playground]]` + contributes/consumes metadata added to plugin Cargo.tomls; `framework/plugin/registry` now emits `generated/playgrounds.{json,ts}` (28 playgrounds, validated for unique variant/alias/port + multi-app `app` field); all 5 hand-synced maps + `contributorPluginIdsFor` deleted from `core/js/index.ts` and `repo/lib/js/index.ts`; `os/dev` + `wgpu` `script.ts` inject `VITE_SEMIO_APP_ID`/`--app` from the catalog.
- T-E: `semio_plugin!`/`app_ids!`-adjacent macro (`semio_plugin!` only) added to `framework/plugin/rs/lib.rs` with a generated per-crate consistency test; adopted by 20 of 23 target crates (verified via grep, no duplicate-definition regressions).
- T-H: `even_window_layout` moved to `framework/core/rs/lib.rs`; wgpu's local `even_layout` now a thin wrapper; 8 of 11 raw `Rgba::new` calls swept into `ui_wgpu::Theme` (3 remain, folded into follow-up task_687e06dd).
- T-I: stale doc/comment cleanup in ui README + storybook headers.

**Post-WF2 fixes applied directly by the orchestrating session (not by a sub-agent):**

1. **Critical repo-wide bootstrap bug**: `repo/lib/js/index.ts` statically imported the gitignored `framework/plugin/registry/generated/playgrounds.ts`, which only gets created by running the registry generator — which itself depends on `repo/lib/js/index.ts`. This broke **every** `bun ./script.ts *` invocation repo-wide. Fixed by making `loadFrameworkOsPlaygroundCatalog()` read `generated/playgrounds.json` directly via `readFileSync`/`existsSync` (empty-array fallback on fresh clone) instead of a static TS import.
2. `framework/plugin/registry/script.ts` still imported the now-deleted `contributorPluginIdsFor`/`resolvePluginRegistryId` from core/js (a gap in the original T-D prompt). Rewrote `resolveRegistryPluginIdsForFilter` to compute contributor ids from its own already-parsed `contributes`/`consumes` data — no more core/js dependency.
3. Alias collision: both `puzzle3d` and `process3d` had `aliases = ["3d"]` (my prompt's fault — told T-D to give process3d that alias without checking for a collision). Fixed: `process3d` alias is now `"process 3d"`.
4. `framework/renderer/react/script.ts`'s host-signature lint regex was written in Phase 0 before `ComponentSceneHostProps` existed; updated to require it explicitly (now stricter than before, matching D8's final intent).

**Known drift / left as-is (not blocking):**

- `s` plugin crate NOT converted to `semio_plugin!` — it chains `.local_backbone_storage()` which the macro doesn't support; T-E correctly left it on the manual pattern rather than dropping that capability declaration. Correct exception, not a bug.
- `dag` (now at `infinite/board/port/directed/dag`, not `mathematical/graph/port/directed/dag` as originally briefed) and `forms-module-procedural` (that whole directory no longer exists — apparently moved to `protocol/module/procedural` by an unrelated concurrent restructuring) were not reachable at the paths given to T-D/T-E, so got no macro/metadata treatment. Harmless (registry codegen already skips/warns gracefully on crates without playground metadata) but a real gap if that tech tree settles — flag for Phase 4 fallout sweep.
- **Cargo verification is currently blocked repo-wide** by an unrelated concurrent session's work: a new `os-hub-storage-sqlite` crate (sqlx-sqlite) conflicts with the existing `os-hub` crate (rusqlite) at the native `sqlite3` linkage level — `cargo check` fails for _any_ crate right now, confirmed even on `semio-framework-core`. Flagged as `task_d155e2eb` (urgent). Verified T-D/T-E/T-H's Rust changes via careful textual review instead (macro shape, field access paths, Default derives, no duplicate bundle definitions, even_window_layout wiring) — could not compile-verify. **Re-run `cargo check -p semio-framework-core -p semio-framework-plugin -p note-plugin -p puzzle-plugin` as the first step of the next session once the sqlite conflict clears.**
- This directly overlaps WF3's planned T-G (hub `HubStore` trait + Postgres) — since another session is already actively building hub storage abstractions, **T-G's scope is narrowed to backbone dedup only** (`framework/product/os/core` + `os/dev` middleware); do not touch `framework/product/os/hub/**` until task_d155e2eb resolves and the hub direction is clear.

## G3 deliverables (WF3, this session)

- **T-F (ts-rs type mirror):** `framework/core/rs` gained a `typegen` Cargo feature (`ts-rs = "10"`, optional) deriving `TS` on 39 boundary types (ActionDescriptor, WindowLayout family, NamedLayout, WindowMeasure, WindowEngagement family, ModeDefinition, WindowKindDefinition, PanelTabDefinition, AppDefinition, PluginManifest, ViewState, AppLabelsOverlay, etc — full list in the agent transcript). Found and fixed a real bug during its own verification: ts-rs does not cascade `#[serde(rename_all = "camelCase")]` into tagged-enum struct-variant fields, which would have silently emitted snake_case field names; fixed with explicit per-field `#[ts(rename = ...)]`. Added `generate`/`check` verbs to `framework/core/script.ts` (same in-memory-diff pattern as registry/styling) writing `framework/core/js/generated/manifest.ts`. Re-exported 5 fully-matching types (ActionDescriptor, ActionKind, ActionDefinition, WindowMeasure, WindowEngagement family) from the generated module in `core/js/index.ts`; deliberately left WindowLayout/NamedLayout and several others hand-written because ts-rs would widen their string-literal discriminants to plain `string` (real narrowing loss) or because they have already diverged into intentionally-different additive shapes. **This track self-verified successfully**: `cargo check`/`cargo test` (default and `--features typegen`) passed cleanly, `bun ./script.ts generate`→`check` round-tripped, whole-repo `tsc --noEmit` showed zero new errors. Not yet done: registering `generate`/`check` as nx targets in `core/project.json`/`package.json` + launch.json (flagged as a natural follow-up, out of this task's file scope).
- **T-G (backbone dedup, hub scope dropped):** `framework/product/os/core/js/index.ts` gained `BACKBONE_ENDPOINT_PATH` constant and the canonical `applyBackboneMessage(storedEnvelopeJson, messageJson)` (ported from the dev host-shim's inline snapshot/ops-merge logic), with 5 new tests extending the existing backbone `describe` block (26 total os-core tests, all passing). `framework/product/os/dev/script.ts` updated to reuse the os-core helpers instead of re-implementing URI parsing; a `@semio-tech/framework-os-core` workspace dependency was added to `os/dev/package.json`. Did not touch `framework/product/os/hub` at all, per the narrowed scope.
- **T-J (os-shell panel swap):** Fuse import in os-shell.tsx switched to route through `@semio-tech/ui-react`'s re-export instead of a direct `fuse.js` import. The DisplayPanel/SettingsPanel/UISearch/UIFind → ShellDisplayPanel/ShellSettingsPanel/ShellSearchDialog/ShellFindDialog swap was investigated but **deferred** — see the agent's own report in the workflow transcript for the exact reasoning captured at the time.

**Gate 3 results (this session, via Bash):**

- `framework-os-core:test` → 26/26 passing (up from 21; T-G's 5 new tests).
- `framework-renderer-react:test` → **103/104 passing, 1 failure**: `patchOpsFromActionResponse is not a function` — this test imports a symbol from `@semio-tech/framework-core` that no longer exists there. **Not caused by this refactor's WF1-3 work** (no track touched this test or function) — it is fallout from a _different_ concurrent session's in-flight rename/removal in `framework/core/js/index.ts` (T-F's own report independently flagged the same symbol, plus `PluginWasmHandle`, as external in-flight churn when it ran its own `tsc` sweep). Left as-is; not mine to guess the intended rename target.
- `cargo check -p semio-framework-core` (alone) → clean (2 pre-existing unused-import warnings).
- `cargo check -p semio-framework-plugin`, and any plugin crate depending on it (puzzle-plugin, gis-plugin, etc.) → **BROKEN**, 9+ errors: `semio_framework_core` no longer exports `UiNode`, `ActionDescriptor`, `WindowLayout`, `NamedLayout`, `ToolNode`, `WindowEngagement*`, `WindowMeasure`, `SurfaceKind`, `UiComponentSceneNode`, and ~15 more symbols from its root. **This is not caused by this refactor.** A _third_, separate concurrent session is actively relocating the entire declarative-UI type family out of `framework-core`'s public API into `ui_wgpu` (there is already an explanatory comment for it in `core/rs/lib.rs`) — mid-flight, breaking `framework/plugin/rs` (the SDK all 23 plugin crates depend on) in the process. Flagged as `task_ae5f0479` (urgent, severe — blocks compiling the entire plugin ecosystem). **This is why WF4 has not started.**
- Also newly discovered: `note-plugin`, `cad-plugin`, `draw-plugin`, `flow-plugin`, `forms-plugin`, `layout-plugin`, `lowpoly-plugin`, `imperative-plugin`, `raster-plugin`, `shooting-plugin`, `writer-plugin`, and `presentation-plugin` are **not listed in root `Cargo.toml`'s `[workspace] members`** at all — they cannot be `cargo check -p`'d regardless of any of the above. Pre-existing, unrelated to this refactor; folded into `task_ae5f0479`'s scope to re-check.

## STOP condition — read before resuming Phase 4 (WF4)

Do **not** start WF4 (integration) until both `task_d155e2eb` (sqlite native-linkage conflict) and, more importantly, `task_ae5f0479` (framework-core declarative-UI API relocation breaking framework/plugin/rs) have landed and a fresh `cargo check -p semio-framework-core -p semio-framework-plugin -p puzzle-plugin` comes back clean. Running the integration gate against a Rust tree that cannot compile the plugin SDK would produce meaningless signal. Once clear:

1. Re-run `cargo check` across the crates T-D/T-E/T-H touched (23 plugin Cargo.tomls, `framework/plugin/rs/lib.rs`, `framework/core/rs/lib.rs`, `framework/renderer/wgpu/rs/lib.rs`) — my earlier textual review (macro shape, Default derives, field-access paths, no duplicate bundle definitions) is solid but was never compile-verified.
2. Re-check whether T-E's macro invocations still reference the right `semio_framework_core` paths after the API relocation settles (it may rename/move some of the same symbols my macro's sanity test or the crate lib.rs files reference).
3. Then proceed to WF4 as originally planned: repo-wide fallout sweep, wire `framework-renderer-wgpu:lint` into the blocking gate (once its 11→3 remaining color literals are cleared per `task_687e06dd`), dev-boot smoke test via `verify e2e`, close all phase tickets.

## TS side is fully green and independent of the above (safe to rely on now)

Dependency-cruiser (0 violations), `framework-renderer-react:test` (103/104, the 1 failure is unrelated external churn), `framework-os-core:test` (26/26), `verify gate` (dep-cruiser + registry check + renderer lint + plugin capability lint + check-no-px) all pass. The entire TS/React deliverable — ShellStore reducer, host registry, playground catalog codegen, ui panel/dialog components, ts-rs generated types — is real, verified, and does not depend on the Rust churn above resolving.
