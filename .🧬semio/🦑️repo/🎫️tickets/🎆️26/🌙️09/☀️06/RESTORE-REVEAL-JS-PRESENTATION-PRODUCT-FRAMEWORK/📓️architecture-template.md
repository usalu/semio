# Architecture template for restoring `🧰️framework/🛍️products/🎤️presentation`

Exploration report (read-only survey of the current repo, 2026-09-06).

## 0. Key findings

- The **print product** (`🧰️framework/🛍️products/📓️print`) is the best full product template: README, product-root barrel, `📦️packages/🟦️typescript` (package.json / 📋️project.json / 📜️script.ts), `🔨️modules/*`, `🎮️commands/*`, `🧪️tests/*`, `🧬️schema/*`.
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage` is a *consumer app* built against `@semio-tech/animate-presentation-core` (`✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts`) and `@semio-tech/animate-js` (`✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️.ts`).
- `reveal.js@^5.2.1` is declared only by `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json`; it is in `bun.lock` and `node_modules`.
- The projektetage consumer is currently **not wired correctly**: its HTML entry is `🌐️index.html` (plugin default expects `🌐️.html`), its entry file is `📦️index.ts` while package.json/vite/vitest reference `🟦️.ts`, the `@semio-tech/framework` alias points at the non-existent `🟦️glue.ts`, the 📋️project.json `namedInputs` glob says `slide/**` instead of `🎞️slide/**`, and there are no root `dev:mit-bestand:projektetage` / `build:mit-bestand:projektetage` scripts.

## 1. Print product tree (template)

```
📓️print/
  README.md            — layout table + command cheatsheet
  🔣️oracle.json        — test-only third-party references summary
  🟦️.ts                — product barrel (`export {};` placeholder)
  🎮️commands/          — command objects the router dispatches to (🔣️.json collection manifest + one dir per command with 🟦️.ts)
  📦️packages/🟦️typescript/  — package.json, 📋️project.json, 📜️script.ts
  🔨️modules/           — implementation modules (🔣️.json manifest; each may have its own 📦️packages/🟦️typescript)
  🔮️oracle/🔣️.json     — detailed oracle registry
  🖼️assets/
  🧪️tests/             — one dir per test case (🥒️.feature + 🟦️.ts + 🧫️fixtures/)
  🧬️schema/🔣️.json + 🟦️.ts
```

README structure: `# 📓️ Print` → `## Layout` table → `## Commands` (states everything runs through `📦️packages/🟦️typescript/📜️script.ts`, that 📋️project.json registers the same entries as nx targets, and `.vscode/🧩️launch.seed.jsonc` as launch configurations; bash block with each `bun ./📜️script.ts <verb>`).

`🎮️commands/🔣️.json` and `🔨️modules/🔣️.json` are `x-semio.kind: "collection"` manifests with `members[]` of `{directory, id, kind, responsibility, (module.productionConsumers)}`.

## 2. Canonical `📦️packages/🟦️typescript` package (print)

package.json:
```json
{
  "$schema": "../../../../../node_modules/nx/schemas/project-schema.json",
  "name": "@semio-tech/print",
  "version": "0.1.0",
  "description": "print · semio LaTeX document framework",
  "type": "module",
  "private": true,
  "license": "LGPL-3.0-or-later",
  "semio": { "role": "framework", "id": "print" },
  "repository": { "type": "git", "url": "https://github.com/usalu/semio.git", "directory": "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript" },
  "bundleKind": "asset",
  "dependencies": { ... },
  "devDependencies": { ... }
}
```

📋️project.json: one `nx:run-commands` target per router verb, `options.cwd` = package dir (emoji path), `options.command` = `bun ./📜️script.ts <verb...>`, `forwardAllArgs: true` where args are consumed, `"cache": false` for non-cacheable targets. `$schema` depth matches nesting (5 × `../` from `📦️packages/🟦️typescript` under a product).

📜️script.ts router pattern:
```ts
#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` router: `bun ./📜️script.ts generate|build|watch|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class BuildScript extends BundleScript { async run(segments: string[]): Promise<void> { ... } }
const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
```
Library helpers available from the repo library barrel: `BundleScript` (`this.root`, `this.repoRoot`), `ScriptRouter`, `runBundleScriptMain`, `runViteBunxDev(root, args, {portEnv, defaultPort, fixedPort})`, `runBun(args, cwd, env)`, `playPollingEnv()`, `playgroundPortEnv(kind)`, `playgroundDevPortString(kind)`, `resolveTestLevel`, `runVitest`.

`🧰️framework/📦️packages/🟦️typescript/📋️project.json` targets: `test`, `test-quick`, `test-long`, `test-exhaustive`.

## 3. Products collection manifest

`🧰️framework/🛍️products/🔣️.json` lists members `💻️os`, `📓️print`, `🦑️repo`, `🖥️server`. Add:
```json
{ "directory": "🎤️presentation", "id": "framework.product.presentation", "kind": "product", "responsibility": "..." }
```
`🧰️framework/🛍️products/AGENTS.md` describes categories (Platform, Playground, Os) — MUST NOT be edited.

## 4. Consumer: `♻️mit-bestand/🎤️präsentation/📅️33.projektetage`

```
📅️33.projektetage/
  🌐️public/                 — static assets ("/x.png")
  🎞️slide/<Chapter>/<Sequence>/<Thought>/<Slide>.ts
  🎨️globals.css
  📦️packages/🟦️typescript/{package.json, ⚙️vite.config.ts, 🌐️index.html, 📋️project.json, 📜️script.ts, 📦️index.ts, 🧪️vitest.config.ts}
```

package.json is the *app* pattern: `scripts.dev/build` → `bun nx run <name>:<target>`, `bundleKind: "application"`, `semio.app` block (`kind`, `aliases`, `packageRoot`, `corePackage`, `definitionExport`, `hostKind`, `port: {dev: 6050, env: "PRAESENTATION_PROJEKTETAGE_PORT"}`), `nx.includedScripts: []`.

📋️project.json: `dev` target with env `PRAESENTATION_PROJEKTETAGE_PORT=6050`, `NODE_OPTIONS=""`, `NX_NATIVE_COMMAND_RUNNER=false`, `NX_TASKS_RUNNER_DYNAMIC_OUTPUT=false`, `NX_TUI=false`, `VSCODE_INSPECTOR_OPTIONS=""`; `build` target.

📜️script.ts: `DevScript` → `runViteBunxDev(this.root, ["--config", "⚙️vite.config.ts", ...segments], { portEnv: playgroundPortEnv("projektetage"), defaultPort: playgroundDevPortString("projektetage"), fixedPort: true })`; `BuildScript` → `runBun(["run", "vite", "build", "--config", "⚙️vite.config.ts", ...segments], this.root, playPollingEnv())`.

⚙️vite.config.ts: `root: bundleRoot`, `base: "./"`, `publicDir: ../../🌐️public`, plugins `[...semioHostHtmlVitePlugin(repoRoot, {title, entry, bodyClass}), semioEmojiIndexHtmlVitePlugin(bundleRoot), ...semioAssetsVitePlugin(repoRoot), tailwindcss(), react()]`, `build: playgroundStaticSiteBuildOptions()`, `server.fs.allow: [repoRoot]`, `resolve.alias` for `@semio-tech/ui-react` → `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`, `@semio-tech/framework` → `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` (NOT `🟦️glue.ts`).

📦️index.ts regions: `🧲️Header` → `🔌️Adapters` → `🔖️spec` → `🔖️Deck` (`import.meta.glob("../../🎞️slide/**/*.ts")`, `loadPresentationFromSlideGlob`) → `🔖️Play` → `🧪️Tests` (`import.meta.vitest`). `mount()` calls `mountPresentation(el, deck, { transition: "fade", slideNumber: false, surfaceChrome: {...} })`.

## 5. Styling helpers (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts`)

- `playgroundStaticSiteBuildOptions(overrides?)` → `{ target: "esnext", outDir: "dist", emptyOutDir: true, ...overrides }`.
- `semioEmojiIndexHtmlVitePlugin(rootDir, fileName = "🌐️.html")` — sets rollup input and rewrites `/` to the emoji index. Canonical HTML entry filename is `🌐️.html`.
- `semioHostHtmlVitePlugin(repoRoot, { title, entry, rootId?, bodyClass?, csp?, loading?, cnameHost? })` → favicon plugin + `.nojekyll`/`CNAME` markers + `transformIndexHtml` (order `pre`) that fully replaces the document.
- `semioAssetsVitePlugin(repoRoot)` → serves/copies `🧰️framework/🔨️modules/🖼️assets` at `/🖼️assets/*`.

## 6. Root wiring

- `package.json.workspaces`: flat list of explicit package paths (one per package.json). Add `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript`.
- `package.json.scripts`: `dev:print`, `build:print`, `dev:mit-bestand:demonstrator`, `build:mit-bestand:demonstrator` exist; add `dev:mit-bestand:projektetage`, `build:mit-bestand:projektetage`, `test:presentation` etc.
- `nx.json` and `tsconfig.json` need no per-project entries.
- `.vscode/🧩️launch.seed.jsonc` is the source for `.vscode/launch.json`. Entry shape:
```json
{ "name": "🛠️dev📽️projektetage", "type": "node-terminal", "request": "launch", "command": "bun nx run @semio-tech/mit-bestand-praesentation-projektetage:dev", "cwd": "${workspaceFolder}", "env": { "PRAESENTATION_PROJEKTETAGE_PORT": "6050" }, "presentation": { "group": "3_dev", "order": 210 } }
{ "name": "📦️build📽️projektetage", ..., "command": "bun nx run @semio-tech/mit-bestand-praesentation-projektetage:build", "presentation": { "group": "4_build", "order": 130 } }
{ "name": "🧪️test🖨️print⚡️quick", ..., "command": "bun nx run @semio-tech/print:test-quick" }
```
- `🔒️dependencies.json`, `🧅️layering.json`, `🚚️migration.json` are generated ratchets; regenerate via `bun ./📜️script.ts verify <x> write-baseline`, never hand-edit upward.

## 7. reveal.js presence

| Location | Finding |
|---|---|
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json` | `"reveal.js": "^5.2.1"` (only declaring package) |
| `bun.lock` | resolved `reveal.js@5.2.1` |
| `node_modules/reveal.js` | present |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/{🟦️.tsx,🎨️.css}` | React + reveal.js renderer (descendant of the old framework renderer) |
| `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts` | declarative core (descendant of the old `framework-presentation-core`, 3133 lines) |
