# Presentation Builder Import Repair

Root's installed Vite 7.3.6 native-loader baseline loaded 42 of 43 current Vitest configurations. The sole failure was `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/vitest.config.ts`: it requested `semioAssetsVitePlugin` from the styling theme root, which does not export builder APIs. The sibling `⚙️vite.config.ts` requested four builder APIs from the same wrong owner.

The two configuration files now import directly from `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`; no theme-barrel facade was added. The presentation project's `📋️project.json` now includes that exact external source in `namedInputs.default`, so the cached build invalidates when its direct builder owner changes.

Owned files:

- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/vitest.config.ts`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/⚙️vite.config.ts`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📋️project.json`

Focused validation used installed Vite `loadConfigFromFile` with the explicit `native` loader under Bun. Both current exports loaded:

- Vitest root resolved to the presentation package; the expected test name and five aliases were preserved; the two asset plugins and React/Tailwind plugins materialized.
- Vite root and relative base were preserved; the host HTML, emoji index, asset, favicon, static-deploy, React and Tailwind plugins materialized; all five aliases remained.
- A separate source/input check proved each config has the exact direct builder-owner import and `namedInputs.default` contains that owner exactly once.

This was configuration evaluation only. It ran no Vitest cases, package build, browser, dev server, default bundled loader or editor integration. Root's recorded Animate/presentation-spec alias findings remain separate follow-up scope.
