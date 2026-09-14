# Repo Context for the Print Visualization Library

Read-only exploration by a Sonnet explorer (2026-09-05), persisted by the coordinator.

- Root `print/dist` is empty and unreferenced by any config: stale leftover of today's merge. Real product: `🧰️framework/🛍️products/📓️print/`.
- `temp/merge/print` no longer exists; the merge ticket `MERGE-PRINT-AND-MIT-BESTAND-INTO-CURRENT-BRANCH` documents its triage (`_full.mjs`, `_sheet.mjs` dropped).
- External LaTeX consumers: `♻️mit-bestand/📋️bericht/📋️zwischenbericht/📋️zwischenbericht.tex`, `♻️mit-bestand/📋️bericht/📎️anhang/📈️skalierung.tex`, `♻️mit-bestand/📋️bericht/📑️forschungsbericht/📑️forschungsbericht.tex`.
- No other chart/visualization implementation exists in the repo (no recharts/d3/plotters; `📊️` folders are metrics, CSV plumbing, a Table UI element, icons). Print's taxonomy is the single visualization taxonomy.
- `reports/` = cross-language KPI/export benchmark outputs; `storybook-static/` = built Storybook. Both unrelated.

## Environment findings (coordinator)
- `node_modules` was stale (July); `bun install --frozen-lockfile` relinked workspaces, but the root `postinstall` (`bun ./📜️script.ts setup postinstall`) fails: `Cannot find module './🤖️generated/🎚️ui-axes.ts'` from `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`. Every TS script importing the shared repo library (→ `@semio-tech/framework` barrel → manifest) fails at load, including all print scripts.
- Generators of the missing files all import that same library (circular bootstrap): `🖱️ui/📦️packages/🦀️rust/📜️script.ts generate` (ui-axes), `🧰️framework/📦️packages/🦀️rust/📜️script.ts` typegen (`🪪️manifest.ts`), `🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` typegen (`📜️ui-contract.ts`). The whole `**/🤖️generated/` layer (icons, tokens, playgrounds, plugins, actor, session) is gitignored and absent on this Windows checkout.
- `tectonic` not on PATH; MiKTeX (`xelatex`, `lualatex`) is. The compilation module downloads tectonic 0.16.9 into `.🧬semio/🦑️repo/⚡️cache/tectonic/`.
