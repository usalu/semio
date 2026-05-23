# Monorepo Script Refactor

**Status:** Done

## Summary (latest)

- Scripts use the **`.script.ts`** suffix.
- Scripts live **only in the bundle (or workspace) they affect** — e.g. `build.3dm.script.ts` under `semio/client/ui/3dm/ui/`, `dev.desktop.script.ts` under `semio/client/ui/desktop/`.
- Root keeps **workspace-wide** orchestration only: `setup.script.ts`, `start.script.ts`, `postinstall.script.ts`, `lint.script.ts`, `build.workspace.script.ts`, `test.script.ts`, `format.script.ts`, `typecheck.workspace.script.ts`.
- `package.json` / `project.json` delegate with `bun <path/to/*.script.ts>` (no embedded command logic).
- Shared Vite/Storybook runners are **copied per consuming bundle** (`run.vite.script.ts`, `run.storybook.script.ts`) so each site/UI owns its launcher.
- Repo MCP tooling: `repo/client/dev.mcp*.script.ts`, `repo/lint.repo.script.ts`; engine MCP: `semio/client/bin/engine/dev.mcp.engine.script.ts`.

## Verification

- `bun start.script.ts` — exit 0.

## Files (representative)

- Root: `*.workspace.script.ts`, `setup.script.ts`, `start.script.ts`, `postinstall.script.ts`, `lint.script.ts`, `test.script.ts`, `format.script.ts`
- Bundles: `semio/**/**.script.ts`, `elements/ui/*.script.ts`, `coda/client/ui/desktop/*.script.ts`, `repo/**/*.script.ts`
- `package.json`, `project.json`, platform `setup.*` / `start.*` scripts

Repo MCP unavailable; ticket tracked manually.
