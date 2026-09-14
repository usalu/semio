# Agent C — projektetage consumer port to `@semio-tech/presentation` / `@semio-tech/presentation-react`

Ticket: `2026/09/06/RESTORE-REVEAL-JS-PRESENTATION-PRODUCT-FRAMEWORK`.
Package dir (`$P`): `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript`.

## Result

| Check | Command | Outcome |
|---|---|---|
| Vite build | `bun run vite build --config ⚙️vite.config.ts` in `$P` | `✓ built in 21.45s`, `dist/🌐️.html` + `dist/assets/*` written. Process then exits non-zero (see “Known environment quirk”). |
| Vite build (JS API) | `bun 🗑️generated/c-probe.ts` | `BUILD OK`, exit 0, same output. |
| Vitest | `bunx vitest run --config 🧪️vitest.config.ts` in `$P` | 1 file, **28 tests passed**, exit 0. |
| Router | `bun ./📜️script.ts test` in `$P` | 28 tests passed, exit 0. |

Workspace linkage is in place (`node_modules/@semio-tech/presentation`, `…/presentation-react` symlinks exist after Agent A’s root wiring + `bun install`); no extra `bun install` was needed from this agent.

## Changed files

Renames (`mv`, no git commands used):

- `$P/📦️index.ts` → `$P/🟦️.ts`
- `$P/🌐️index.html` → `$P/🌐️.html`

Edits:

1. `$P/🌐️.html` — `<script type="module" src="./🟦️.ts">` (was `./js/index.ts`, a path that never existed). `lang="de"`, title and the CSP meta kept.
2. `$P/🟦️.ts` — `@semio-tech/animate-presentation-core` → `@semio-tech/presentation`, `@semio-tech/animate-js` → `@semio-tech/presentation-react` (3 static import sites, 1 dynamic `import()`, 1 re-export). Header docstring now names `@semio-tech/presentation`. `import "../../🎨️globals.css";` kept.
3. `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎞️slide/**/*.ts` — 27 files, same specifier replacement (33 occurrences of the core specifier in total across entry + slides).
4. `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎨️globals.css`
   - `@import "../../../🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css";`
   - `@source "../../../🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react";`
   - **also fixed** the first line: `@import "…/🖱️ui/🎨️globals.css"` → `@import "../../../🧰️framework/🔨️modules/🖱️ui/🧵️.css"`. `🧰️framework/🔨️modules/🖱️ui/🎨️globals.css` does not exist in the repo; the ui entry stylesheet is `🧵️.css` (it imports `🎨️styling/🖌️ui.css` → `tailwindcss`, plus `🌐️globals-ui.css`). Without this the tailwind plugin aborted the build with `Can't resolve '../../../🧰️framework/🔨️modules/🖱️ui/🎨️globals.css'`.
   - custom `abschluss` rules and `@source "."` untouched.
   - The file sits 3 levels below the repo root, so the relative prefix is `../../../` (not `../../`).
5. `$P/⚙️vite.config.ts` — alias list replaced, in this order (vite matches string `find` by prefix, so `-react` must precede the bare name):
   `@semio-tech/ui-react` → `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`;
   `@semio-tech/presentation-react` → `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`;
   `@semio-tech/presentation` → `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts`;
   `@semio-tech/framework` → `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` (was the non-existent `🟦️glue.ts`);
   spec alias → `resolve(dir, "🟦️.ts")`.
   Added `define: { "import.meta.vitest": "undefined" }` so the in-source test block is stripped from the bundle (same as the demonstrator). Plugin list, `entry: "./🟦️.ts"`, `publicDir`, `server.fs.allow` unchanged.
6. `$P/🧪️vitest.config.ts` — same alias set; `include: []`, `includeSource: ["🟦️.ts"]`, `coverage.include: ["🟦️.ts"]`, `environment: "node"` kept — the `import.meta.vitest` block is a pure model test (`countArrangements`, `collectPresentationSlides`, `expandThoughtSlides`, …) and never touches `document`; the only `document` use is the `mount()` guard outside the test block. jsdom is not needed and the run confirms it.
7. `$P/package.json` — dependencies `@semio-tech/presentation` + `@semio-tech/presentation-react` (`workspace:*`, replacing `@semio-tech/animate-js`); `exports["."] = "./🟦️.ts"` (already correct); added `"test": "bun nx run @semio-tech/mit-bestand-praesentation-projektetage:test"`.
8. `$P/📋️project.json` — `namedInputs.default` glob fixed to `{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎞️slide/**/*.ts`; added the `test` target (`bun ./📜️script.ts test`, `forwardAllArgs: true`, same `cwd`).
9. `$P/📜️script.ts` — added `TestScript` (`resolveTestLevel(segments)` → `runVitest(this.root, rest, "🧪️vitest.config.ts")`), registered as `test`; header docstring now `<dev|build|test>`; `dev` / `build` untouched.
10. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` — in the single projektetage entry-file record, the path field `…/📦️packages/🟦️typescript/📦️index.ts` and the basename field `📦️index.ts` became `…/🟦️.ts` and `🟦️.ts`. No other record touched; file re-validated as JSON. **Note:** the record’s content hash `8c7779b3d6…` was left as-is (only the filename was in scope); the file’s bytes changed with the specifier rewrite, so if that fixture is hash-checked it will need a baseline regeneration by whoever owns the purity ratchet.

Files explicitly not touched by this agent: `✏️s/**`, `.vscode/**`, root `package.json`, and the two product packages.

## Repo preconditions hit along the way (not consumer bugs)

- `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts` was missing, breaking every bundle that pulls `@semio-tech/ui-react` → `@semio-tech/assets`. Regenerated locally with `bun ./📜️script.ts generate-metabolism` in `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript` (gitignored output, 31 artifacts).
- `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes.ts` was likewise missing and `bun ./📜️script.ts generate` refused to render it (`missing external 🔣️shortcodes.json snapshot` — the pinned gemoji snapshot is declared `external-emoji-shortcodes` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` and is gitignored). It appeared in the working copy at 13:22 while this agent was investigating — another agent or a background task generated it. After that the projektetage bundle resolved cleanly. Anyone starting from a fresh clone will hit this before any ui-react app can build.

## Known environment quirk (pre-existing, not caused by this port)

`bun run vite build` prints `✓ built in …` and writes a complete `dist/`, then the process dies with `error: "vite.exe" exited with code 9`; the same build driven by `node node_modules/vite/bin/vite.js` finishes `✓ built` and exits `127`. Nothing is printed after the success line — a silent teardown crash in a native module. It is **not** the tailwind plugin: a probe build with `tailwindcss()` removed exits 9 the same way (`🗑️generated/c-probe-notw.txt`). The identical build through vite’s JS API (`build({ configFile })`) exits 0 (`🗑️generated/c-probe-out.txt`), so the artifacts are correct and only the CLI exit status is wrong. Consequence: `bun ./📜️script.ts build` and the nx `build` target report failure despite producing a correct bundle. The pre-existing `🗑️generated/baseline-build.txt` shows the same `code 9` before any of this ticket’s changes. `♻️mit-bestand/🧺️demonstrator` exits `1` with three real, unrelated build errors, so it could not serve as a clean control.

## Logs

Under `🗑️generated/`: `c-build.txt` (CLI build), `c-build-node.txt` (node CLI build), `c-build-script.txt` (router build), `c-probe-out.txt` (JS-API build, exit 0), `c-probe-notw.txt` (tailwind-free probe), `c-test.txt` (`bunx vitest run`), `c-test-script.txt` (`bun ./📜️script.ts test`), `c-demonstrator-build.txt` (control), `c-probe.ts` (the JS-API probe used to surface the swallowed CLI errors).
