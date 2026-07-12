# Os App Playground Ui Refactor — Phase Status

Plan: /Users/ueli/.claude/plans/refactor-the-os-app-logical-koala.md

Update this file only from a gate-runner session (S1, S2, S14).

## Current phase: Phase 1 — Contracts freeze (next)

| Phase | Status | Gate |
|---|---|---|
| 0 Foundations (S1) | done | G0: `bun ./script.ts verify gate` ✅ (dep-cruiser + registry check + renderer-react lint + os-dev plugin lint + check-no-px), `framework-renderer-react:test` ✅ (93 tests), `framework-os-core:test` ✅ (16 tests, previously broken target now fixed) |
| 1 Contracts freeze (S2) | not started | G1: typecheck all ✅, workspace:lint ✅, os dev boot ✅ |
| 2 Parallel tracks (T-A..T-I) | not started | per-track verify gate + workspace:verify |
| 3 Parallel (T-F,T-G,T-J) | not started | G3: workspace:verify ✅, cargo test --workspace ✅ |
| 4 Integration (S14) | not started | full gate checklist |

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

## Next: Phase 1 (S2) — Contracts freeze

Land skeletons in the 5 hot files per the plan (D1-D8): `framework/core/js/index.ts`, `os-shell.tsx`, `ui/js/react/index.tsx`, `ui-interpreter.tsx`, `repo/lib/js/index.ts`. This is the next session to run.

**⚠️ Blocker observed at the end of S1 (do not start Phase 1 until re-checked):** `git status` at S1 close-out shows heavy, currently-uncommitted, in-progress concurrent activity across almost every plugin crate's `lib.rs` (cad, flow, forms, gis, imperative, layout, lowpoly, mathematical, note, procedural, process, puzzle, raster, reasoning, s, sequence, shooting, trinity, vcs) plus deletions of several `app_*.rs`/`mod.rs`/multi-file-crate submodules, and modifications to exactly the Phase-1/Phase-2 hot files: `framework/core/js/index.ts`, `framework/core/rs/lib.rs`, `framework/plugin/rs/lib.rs`, `framework/renderer/react/os-shell.tsx`, `framework/renderer/react/index.test.ts`, `framework/wit/world.wit`, `ui/js/react/index.tsx`. New sibling tickets appeared: `.repo/🎫/26/07/12/FLATTEN-MULTI-FILE-CRATES-AND-PACKAGES-INTO-SINGLE-LIB-RS-INDEX-TS-ENTRIES` and `.repo/🎫/26/07/12/LOCALE-AWARE-PLUGIN-MANIFEST-LABELS`. These are `git log`-invisible (uncommitted), i.e. another live session is actively mid-edit on the same hot files this refactor's Phase 1 needs. **S2 must re-run `git status`/`git log` on these paths before claiming Phase 1, and either wait for that work to land or renegotiate the ownership matrix against it** — do not blind-edit into a session that's already there.
