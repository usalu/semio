# Integration — restored reveal.js presentation product

Coordinator pass after the four implementer reports (`📓️report-a-core.md`, `📓️report-b-react.md`, `📓️report-c-consumer.md`, `📓️report-d-tests-launch.md`).

## Result

| Check | Command | Outcome |
|---|---|---|
| Core tests | `bun ./📜️script.ts test quick` in `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript` | 56/56 |
| Renderer tests | `bun ./📜️script.ts test quick` in `…/🎯️targets/⚛️react` | 147/147 (3 files) |
| Language-agnostic markdown case | `bun ./📜️script.ts test parity exhaustive --project test-framework-products-presentation-e109a8-markdown-html-compilation` | 14/14, parity 7/7 |
| Projektetage tests | `bunx vitest run --config 🧪️vitest.config.ts` in the projektetage package | 28/28 |
| Projektetage build | `bun ./📜️script.ts build` in the projektetage package | `✓ built`, exit 0, `dist/🌐️.html` + 1466 copied asset files |
| Projektetage dev | `node node_modules/vite/bin/vite.js --config … --port 6050` | reveal.js mounted, 39 sections, no console errors; title slide, Recherche overview and Bauteilportal slides render with all images loaded |

## Two blockers found after the reports, and their fixes

### 1. Blank page in the browser — top-level-await cycle

The slide files statically import `@semio-tech/mit-bestand-praesentation-projektetage-spec`, and that alias pointed at the entry module `🟦️.ts`, which awaits every slide at top level through `import.meta.glob`. In the browser this is an ESM deadlock (the entry waits for a slide, the slide waits for the entry's async evaluation), so `#root` stayed empty without any error. Vitest never showed it because its module runner tolerates the cycle.

Fix: the `🔖️spec` region (meta, intro, catalogue, Baukomponenten) moved into `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/🔖️spec.ts`; the entry `🟦️.ts` imports what its deck and tests need from it and re-exports it. Both the vite and the vitest alias of the projektetage now resolve to `🔖️spec.ts`; the renderer's test alias keeps pointing at the entry because its in-source tests need `deck`.

### 2. Build exits 127 (node) / 9 (bun) after `✓ built`

Bisected to the `ui-assets-build` plugin: Node 24.14.1 on Windows hard-crashes (no `exit` event, no exception) in `fs.cpSync(dir, dest, { recursive: true })` whenever the *source* path contains astral-plane characters, i.e. every emoji directory of this repository (`🧰️framework/🔨️modules/🖼️assets/📃️list` alone reproduces it; ASCII sources copy fine; single-file `cpSync` is fine).

Fix: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts` gained `copyDirectoryTree` (readdir + copyFileSync) and the three recursive `cpSync` sites (`ui-assets-build`, `tile-proxy-build`, `static-dir-build`) use it. This also unblocks every other app built through `semioAssetsVitePlugin`.

## Other integration notes

- A concurrent writer appended a duplicated six-line tail to the projektetage entry while it was being split; the file was truncated back to its `//#endregion 🧪️Tests` marker and syntax-checked.
- `.claude/launch.json` gained a `mit-bestand-projektetage` preview entry that runs vite through node directly, because the preview harness ships bun 1.3.13 which cannot execute the `.bin/vite` shell shim used by `runViteBunxDev`.
- `bun ./📜️script.ts verify dependencies` already fails with 102 unrelated new dependencies from other in-flight work (`temp/brepkit`, …); the baseline was deliberately not regenerated here. The new product adds `reveal.js`, `pdfjs-dist` (already in the lockfile) and the test-only remark stack.
- The nx project graph is currently broken by an unrelated duplicate puzzle test project, so all verification ran through the package routers directly.
- The `✏️s/🔌️plugins/🎞️animate` plugin was left untouched; it still carries its own copy of the renderer for the on-hold s presentation mechanism.
- Not restored: the July `framework/product/presentation/rs` crate (a VCS-backed tile-deck document) and the playground `play-host.tsx`; both depended on the deleted playground/platform products and are superseded by the animate artifact standard.
