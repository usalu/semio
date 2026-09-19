# 📓️ Status — Semio Tech Play

## 2026-09-19 — session 1 (manual ticket bookkeeping: repo MCP `ticket_open` answered "invalid tool params")

### Done
- `🏢️semio-tech/🎡️play` created (design: `📋️design.md`). 58 panes = every playground registry row except the
  `s` host shell and the six `entwerfen-mit-bestand-*` re-skins. English + `native` terminology locks.
- Schema-first pane catalog `🔨️modules/🧩️runtime/🔣️.json` + `🧬️schema/🔣️.json`, registered as schema scope
  `semio-tech.play.runtime` (`bun ./📜️script.ts schema generate`; the committed catalog was already stale vs disk,
  so the regeneration also carries peers' unrelated scope churn).
- Activation: 26 pane lanes (greedy cover of the 53-component union) merged into `dist/♻️activation/dev`;
  the `s` host lane was tried first and fails on the unlinkable stdio component. Framework:
  `ActivationComponentSpec.installDirectory` so the freshness report reads each extension's own lane dir.
- Dev server `bun ./🔨️modules/🧩️runtime/📜️script.ts serve` on :6033; acceptance via
  `bun nx run @semio-tech/semio-tech-play:test-e2e` (overview + one boot test per app).
- Wiring: root `package.json` workspace + `dev:semio-tech:play` / `build:semio-tech:play`; launch seed entries
  (`🛠️dev🏢️semio-tech🎡️play`, activate, serve, `📦️build…`, `⚖️gate…🎭️e2e`); taxonomy kinds `semio-tech`,
  `members-of-semio-tech`, area `clean`, layer `implementation`, schema scope level `product-module`;
  UI primitives allowlist + chrome-i18n scanned root.
- Unit tests (19, green): pane coverage vs registry, every descriptor editor app reachable, Ajv schema
  validation, icons registered, activation receipt rule, grid geometry, live-pane budget, locks.

### Acceptance history
- Run 1: 50/59 — eight apps dropped the boot `setActiveExample` (undeclared), gis3d boot panic.
- Run 2: 59/59 after the ShellHost example gate and the gisterrain fixture fix.
- Run 3 (2 s settle, refused inputs fail): 48/59 — see `📓️app-boot-defects.md`; puzzle5d fixed since,
  the rest delegated per plugin group.
- Run 4 (strict, after all plugin fixes + full play activation): **59/59 passed** (5.2 min).
- Release: `bun nx run @semio-tech/semio-tech-play:build` succeeded (136 min cold, 172 tasks) →
  `dist/site` 17 428 files; served statically, cad/generation3d/puzzle3d/gis2d/fem3d/energy/note/flow reach
  ready; only 404 is the dev-only `/__semio/agent-bridge`.

### Fixes outside play (found on the way)
- `📐️cad` picking: missing `CadEdgeCurve` import (peer commit 13:48) broke every cad wasm build.
- Demonstrator vite config imported `../../../🔨️modules/📦️site/🗺️tile-serve-mode` (one level too high) since 09-17.

### Open follow-ups
- Raster composite canvas shows no visible demo content in the browser pane; JSON export + mutation JSON bridge
  still serialize whole Raster snapshots.
- Flow test app never finishes closing; rename/patch-widget paths still drop live hosts.
- Architect/Writer derived children load empty (content not persisted in the parent pack); writer demo asset
  child id ≠ target id. CAD pane content resolves from a shipped-example catalogue only.
- Pre-existing red suites the agents measured (unchanged by this ticket): cad 17, shooting 48, writer 44,
  process3d 32, raster editor 25, block2d schema 37.
- `verify layering` already fails repo-wide (222 files over baseline); play adds the same reference kinds the
  demonstrator has (UI allowlist, taxonomy, package.json).
- The committed schema catalog was stale; `schema generate` output includes peers' scopes.

### Known parity debt (same findings in `♻️mit-bestand/🧺️demonstrator`)
- `verify taxonomy report --scope`: `⚛️*.tsx`/`🪧️brand.ts`/`🎨️globals.css` leaf names and unregistered
  `🔨️modules/*` directory kinds.
- tsc: `OwnedBuildPlugin` vs Vite `PluginOption`, `createUiErrorBoundary` vs React JSX, untyped `🟨️.mjs`.

## 2026-09-19 23:35 — session 3 (goal: play shows ALL plugins, finished end to end for all plugins)

Session 2's fix fleet died with a process restart (partial logs: `🗑️generated/stdio-a`, `stdio-ifc-zip`,
`stdio-semio`; baseline `🗑️generated/baseline/summary.tsv`: 52 ok / 111 not ok of 163 plugin crates).
Relaunched as fleet v2 under `📋️fleet-brief-v2.md`; every agent writes `🗑️generated/<topic>/REPORT.md`:
- Opus fix agents: `xcut-toolproof` (tool factory proof `generated_migrated=false`, 94 failures, 15 crates),
  `xcut-dict` (neural `final Dictionary ownership`, 179 failures; owns flow-extension-*, imperative-*, playbook),
  `norm-a`, `norm-b`, `stdio-a2`, `stdio-b2`, `stdio-semio2`, `block-puzzle`, `engineering` (fem, energy,
  process, cad, sourcing, gis), `knowledge` (note, writer, forms, mathematical, reasoning, dag, trinity,
  architect), `media` (sequence, animate, vcs, shooting, space, demonstrator), `design` (flow, procedural,
  draw, raster, layout, remodel, lowpoly, wfc).
- Sonnet audits: `audit-coverage` (plugins/apps without a pane: stdio, space, extensions), `audit-parity`
  (demonstrator → play capability gaps), `audit-visual` (per-pane screenshots + content verdicts on :6033).
Recovery after a restart: read each topic's REPORT.md / RECIPES.md / STATUS.md, re-run crates, relaunch the rest.
