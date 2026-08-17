# Compose Technology Leakage Eradication

## Verdict
Path/package coupling from `./compose` into modern trees (`framework` modules, `os`, `s`, `.storybook`) is cleared.
Compose remains a legacy island at repo root; discovery + workspace generation skip it.

## Done
- Storybook: deleted compose scopes/stories/kit-store; removed aliases and CSS `@source`s
- UI styling/vite: removed compose palette emit, scan roots, sketchpad stub plugins
- OS core: removed compose-only `osBaselineArtifact` / workflow / VCS registration shim
- UI react lint allowlists: no longer scan compose sketchpad
- Reasoning DSL: renamed `compose.metabolism.*` → `metabolism.*`; kit-path=`embedded`
- Framework assets: deleted `🏛️compose/`, compose logos/images, `metabolism/.compose`
- Workspaces scan: skips `compose` like discovery
- UI demos: `compose.sketchpad.*` ids → `demo.*`
- Repo lib: dropped `@compose/` from `INTERNAL_PREFIXES`; vscode extension no longer special-cases `@compose/`

## Keep
- Puzzle domain `🌉️compose` (composition engine) — not the compose technology
- Verb uses of “compose” / “composition”

## Residual (follow-up)
Repo product still contains many historical `compose` strings (Go CLI scopes/bundles/statutes, Neo4j DB name `compose`, micro-commit cache filenames `compose-*`, tests using `compose/` example paths, bundle label `🏘️compose`). Isolation skips are in place; full rename of monorepo meta away from the old “compose” repo identity is a separate ticket.
