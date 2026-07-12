# Os App Playground Ui Refactor — Phase Status

Plan: /Users/ueli/.claude/plans/refactor-the-os-app-logical-koala.md

Update this file only from a gate-runner session (S1, S2, S14).

## Current phase: Phase 3 (WF3) — T-F/T-G/T-J (next)

| Phase | Status | Gate |
|---|---|---|
| 0 Foundations (S1) | done | G0: `bun ./script.ts verify gate` ✅ (dep-cruiser + registry check + renderer-react lint + os-dev plugin lint + check-no-px), `framework-renderer-react:test` ✅ (93 tests), `framework-os-core:test` ✅ (16 tests, previously broken target now fixed) |
| 1 Contracts freeze (WF1) | done | G1: renderer tests ✅ (94), dep-cruiser ✅, verify gate ✅. Typecheck: `ui-react:typecheck` has pre-existing/unrelated failures (duplicate `ThreeEvent` export in ui/index.tsx, tsconfig quirks) — zero errors in any WF1-touched file. |
| 2 Parallel tracks (WF2: T-A..T-I) | done | See Gate 2 below |
| 3 Parallel (T-F,T-G,T-J) | not started | G3: see adapted scope note below |
| 4 Integration (WF4) | not started | full gate checklist |

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
- **Cargo verification is currently blocked repo-wide** by an unrelated concurrent session's work: a new `os-hub-storage-sqlite` crate (sqlx-sqlite) conflicts with the existing `os-hub` crate (rusqlite) at the native `sqlite3` linkage level — `cargo check` fails for *any* crate right now, confirmed even on `semio-framework-core`. Flagged as `task_d155e2eb` (urgent). Verified T-D/T-E/T-H's Rust changes via careful textual review instead (macro shape, field access paths, Default derives, no duplicate bundle definitions, even_window_layout wiring) — could not compile-verify. **Re-run `cargo check -p semio-framework-core -p semio-framework-plugin -p note-plugin -p puzzle-plugin` as the first step of the next session once the sqlite conflict clears.**
- This directly overlaps WF3's planned T-G (hub `HubStore` trait + Postgres) — since another session is already actively building hub storage abstractions, **T-G's scope is narrowed to backbone dedup only** (`framework/product/os/core` + `os/dev` middleware); do not touch `framework/product/os/hub/**` until task_d155e2eb resolves and the hub direction is clear.

## Next: Phase 3 (WF3) — T-F (ts-rs mirror) / T-G (backbone dedup, hub scope dropped) / T-J (os-shell panel swap)
