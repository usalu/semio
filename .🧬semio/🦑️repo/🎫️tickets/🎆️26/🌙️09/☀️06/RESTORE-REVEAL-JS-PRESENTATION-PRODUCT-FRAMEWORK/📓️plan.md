# Plan — Restore the reveal.js presentation product framework

## Findings

- The old product lived at `framework/product/presentation/{core, renderer/react, rs}` and was fullest at commit `5b0fd1dd3a` (2026-07-03, #124). Its tree is exported to `🗑️generated/old-framework-124/`, the projektetage of that time to `🗑️generated/old-projektetage-124/`.
- Commits #125/#126 (2026-07-04) moved the code into the `s` animate plugin; #305 (2026-07-18) deleted the last remnants. The code kept evolving there until 2026-09-02. The **latest reveal.js-based version** is therefore:
  - core: `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts` (3133 lines, zero imports) + `🧪️index.test.ts`
  - renderer: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/{🟦️.tsx (9406), 🎨️.css (1406), 🔨️modules/📝️markdown-html-compiler/🟦️.ts (403), 🔨️modules/🔌️pdf-canvas-port/🟦️.ts (182)}`
  - jsdom polyfills: `✏️s/🔌️plugins/🎞️animate/🪨️tests/🟦️.ts`
  - vitest config: `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️tests/🟦️.ts`
- The consumer `♻️mit-bestand/🎤️präsentation/📅️33.projektetage` currently fails to build (vite exits 9, 0 modules) because its package uses `🌐️index.html`/`📦️index.ts` while its configs reference `🌐️.html`/`🟦️.ts` and a non-existent `🟦️glue.ts`.
- nx's project graph is currently broken by an unrelated duplicate project (puzzle tests) — verify with direct `bun ./📜️script.ts …` / `bunx vitest` / `bun run vite build`, not `bun nx run`.

## Decision

Restore the product from the latest reveal.js code (the animate descendant), under the new architecture. The animate plugin is left untouched (it is on hold; other agents may work there). The projektetage is repointed to the restored product.

## Target layout

```
🧰️framework/🛍️products/🎤️presentation/
  README.md                                   layout table + commands + domain model (from old AGENTS.md, but README)
  🟦️.ts                                       product barrel: export * from "./📦️packages/🟦️typescript/🟦️.ts"
  📦️packages/🟦️typescript/
    package.json                              @semio-tech/presentation (library, semio.role framework, id presentation)
    📋️project.json                            test, test-quick, test-long, test-exhaustive
    📜️script.ts                               TestScript → runVitest(this.root, rest, "🧪️tests/🟦️.ts")
    🟦️.ts                                     core (ported), header docstring names @semio-tech/presentation
    🧪️tests/🟦️.ts                             vitest config: in-source (includeSource ["🟦️.ts"]) + include ["🧪️tests/🧭️slide-glob-assembly/🟦️.ts"]
    🧪️tests/🧭️slide-glob-assembly/🟦️.ts       ported 🧪️index.test.ts importing "../../🟦️.ts"
    🎯️targets/⚛️react/
      package.json                            @semio-tech/presentation-react (bundleKind ui), deps: @semio-tech/presentation, @semio-tech/framework, @semio-tech/ui-react, react, react-dom, reveal.js ^5.2.1, pdfjs-dist ^5.4.296; devDeps @types/react, @types/react-dom, @types/reveal.js, @vitejs/plugin-react, typescript, vitest, jsdom
      📋️project.json                          test ladder
      📜️script.ts                             TestScript
      🟦️.tsx                                  renderer (ported), imports from "@semio-tech/presentation"
      🎨️.css                                  ported 🎨️.css
      🔨️modules/📝️markdown-html-compiler/🟦️.ts
      🔨️modules/🔌️pdf-canvas-port/🟦️.ts
      🧰️vitest.setup.ts                       ported 🪨️tests polyfills
      🧪️tests/🟦️.ts                           vitest config (jsdom, aliases, includeSource renderer + modules, setupFiles)
  🧪️tests/
    📝️markdown-html-compilation/🥒️.feature    language-agnostic case
    📝️markdown-html-compilation/🟦️.ts         adapter: subject = owned compiler, oracle = unified/remark-parse/remark-gfm/remark-rehype/rehype-stringify
    📝️markdown-html-compilation/🧫️fixtures/…
  🔮️oracle/🔣️.json                            oracle registry (schemaVersion 2, like print)
```

Package names: `@semio-tech/presentation`, `@semio-tech/presentation-react` (no collisions).

## Consumer changes (`♻️mit-bestand/🎤️präsentation/📅️33.projektetage`)

- `📦️packages/🟦️typescript/📦️index.ts` → `🟦️.ts`; `🌐️index.html` → `🌐️.html` (script src `./🟦️.ts`).
- Imports: `@semio-tech/animate-presentation-core` → `@semio-tech/presentation`; `@semio-tech/animate-js` → `@semio-tech/presentation-react` (entry + all 27 slide files).
- `🎨️globals.css`: import `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css`, `@source` that directory.
- `⚙️vite.config.ts` + `🧪️vitest.config.ts`: aliases `@semio-tech/presentation-react` → target `🟦️.tsx`, `@semio-tech/presentation` → product package `🟦️.ts`, `@semio-tech/framework` → `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`, keep `@semio-tech/ui-react`, spec alias → `🟦️.ts`. Order matters: the longer `-react` alias before the shorter one.
- `package.json`: dependencies `@semio-tech/presentation`, `@semio-tech/presentation-react`; exports "." → "./🟦️.ts"; add `test` script + nx test target.
- `📋️project.json`: namedInputs `🎞️slide/**/*.ts`; add `test` target.

## Root wiring

- `package.json` workspaces: add both product packages. Scripts: `dev:mit-bestand:projektetage`, `build:mit-bestand:projektetage`, `test:presentation`.
- `🧰️framework/🛍️products/🔣️.json`: add `🎤️presentation` member.
- `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json` (kept in sync by hand, both files): add `🧪️test🎤️presentation⚡️quick|🌕️long|🌌️exhaustive` and `🧪️test🎤️presentation⚛️react⚡️quick` entries next to the print test ladder; keep the projektetage dev/build entries; add `🧪️test📽️projektetage`.
- `bun install` to link the workspaces (updates bun.lock).

## Verification

1. `bun ./📜️script.ts test quick` in both product packages passes.
2. `bun run vite build --config ⚙️vite.config.ts` in the projektetage package produces `dist/` without errors; `bunx vitest run --config 🧪️vitest.config.ts` passes.
3. Dev server on port 6050 renders the deck (browser check).

## Work split

- Agent A — product scaffold + core package + root wiring.
- Agent B — react renderer target.
- Agent C — projektetage consumer port.
- Agent D — language-agnostic markdown test + oracle registry + launch entries.
