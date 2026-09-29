# TS1 — os TypeScript program type-checks with 0 errors (session 14c)

Slice TS1, session 14c, 2026-09-28 17:2x (new slice, no predecessor). Scope: `🧰️framework/🛍️products/💻️os` —
`bunx tsc --noEmit -p tsconfig.json` must report 0 errors; root-fix each error at its cause (no `as any`, no `@ts-ignore`,
no loosened types; fixtures typed from their schema). Host TS + test files only (guest freeze, rules 2/22). Expendable
captures `wp-ts1/generated/`. Rules: `📓️session-14-preamble.md` (+ 13/12).

Status legend: **measured** = ran here, command + capture named; **unverified** = read from source only; **written, not run**.

## Session 14e — CAD suite load failures + real `@types/bun` (coordinator 08:1x)

| # | item | state | evidence |
|---|---|---|---|
| 1a | import cycle through `📔️registry` (`runtime/renderer → actions → typology → artifact → registry → semio → geometry/preview → actions`) | **fixed at the layer boundary**: (i) the 09-08 extraction hoisted test-only static imports into sources' `🧪️Tests` regions — spatial-kernel `📐️geometry` imported cad `🎬️actions` in production; `wp-ts1/ts1-cad-test-imports.ts` moves every test-only name into its suite's own imports (actions 1, artifact 38, geometry 6, spatial 4) and out of bag/type; geometry's production-used `SpatialKernel`/`SpatialPreviewKernel` become a header `import type`; (ii) spatial-kernel `PreciseSpatialKernelMath.executeAction` delegated up into cad `executeActionCapability` — exactly what `🎬️actions`' own fallback does when a kernel has no native `executeAction` hook → removed with its import. `ts1-import-cycles.ts`: registry/renderer "no cycle" | `cad-vitest-2.txt` |
| 1b | `import.meta.dir` under vitest | **fixed**: cad + spatial-kernel registrations pass `{ url: import.meta.url }`, suites declare `TestSource = { readonly url: string }` and read `new URL(relative, source.url)` (`ts1-cad-test-source-url.py`). The same Bun-only `directory` is passed at 116 sites in 69 files repo-wide (22 suites read it) — follow-up, several are guest-linked | — |
| 1c | flow wasm `/@fs/` in node | **fixed**: `🧊️3d` `ensureBrepWasmLoaded` addresses `flow_core_bg.wasm` by its module URL (`new URL(…, import.meta.url)`, Vite's own asset form); an `http(s)` URL goes to the bindings, a `file:` URL (node/Bun test run) is read from disk (`fetch` has no portable `file:`); the unused `*.wasm?url` ambient in framework removed. `🧊️3d` is not in the os program → no `s` boot | `cad-vitest-semio-2.txt` |
| 1d | CAD suites now | runtime 2/2 (was: load failure), renderer 67/69 (was: load failure), brepjs 30/30 (was 29/30), geometry 41/44 (was 40/44), semio 1/6 — **real reds by owner** (CAD TS engine, no active slice): semio ×5 `brep_invoke translate: unknown brep_invoke method` — the TS `SemioBrepKernel` calls `translate`/`rotate`, the Rust dispatcher in `🌊️flow/📐️brep-geometry/🦀️.rs` has neither (guest code, frozen); geometry ×3 + renderer ×2 AEC typology catalog: `loadTypology("building.building.slab")` → null, `resolveTypologyStyle` falls back to the hash colour (`#3ecca2` ≠ authored `#8B7355`), `from_building` yields 0 objects — geometry and registry each own a copy of the model-definition asset state and caches (same ephemeral keys), the runtime suite sees energy typologies while geometry's lookups miss building/structure ones. Latent (not failing): `runtime → extension → @semio-tech/cad-js index → runtime` | `cad-vitest-2.txt`, `cad-vitest-semio-2.txt` |
| 2 | framework/library/repo/hub on the installed `@types/bun` 1.4.2 | **done**: the hand-written `🏃️process/🌿️environment/🟦️.d.ts` deleted (its header asked for exactly this once `@types/bun` installs); library/repo/hub `types` gain `bun` (framework already saw all `@types`), the file leaves every `files`/`include`; the obsolete inline `ImportMeta.dir` declarations in ui-react's and window-kits' `📜️script.ts` removed; root `lib` += `DOM.AsyncIterable` (the env `BunReadableStream` that blocked it is gone; os drops its duplicate `lib`). 133 unique real errors (the "~230" double-counted library ⊂ repo): 22 `bun:test` calls wrote options before the body → `ts1-bun-test-options-order.ts` puts them where `@types/bun` declares (`test(label, fn, { timeout })`); ~89 typed-matcher mismatches (`ts1-bun-matchers.py`): readonly/literal arrays compare as `toEqual<readonly string[]>`, untyped fixture values as `toEqual<typeof expected>` (bun's own typed-comparison overload), `any` handles get the type they produce (`expect<string>(…)`), missing expectations are guarded; source-admission-io `row.expected.*` likewise; real fixes: Bun.build has no `write` option, `args.kind === "entry-point"` never matched (Bun reports `entry-point-build`, measured), `SyncSubprocess.signal` → `signalCode` (the old check always read `undefined`), `Bun.serve().port` → `url.port`, lease/services children typed `Bun.Subprocess<"pipe","pipe","pipe">`, the fetch stub keeps `preconnect`, execa 1.x typed at its CommonJS boundary, a looped `spawnSync` annotated (assertion-signature CFA), the MCP live-agent harness child typed (its "declared twice" workaround comment is now false), the hub foundation owner-graph law's own compiler program gains `bun` types (it failed on `Bun`) | **measured**: all 20 package tsconfigs 0 errors non-incremental (`r7/summary.txt`), os/hub again after the last two edits (`b2`); suites touching runtime-relevant edits 21/22 + hub foundation law 1/1 (`bun-tests-b.txt`, `bun-test-hub-foundation-2.txt`); the other reds in the 17-suite batch (`bun-tests-a.txt`: rust-physical ×14, empty-facet authoring ×12, typescript-path ×6, schema-invariants ×5, readme-move ×5, kind-only launch seed, hub moved-owner imports, registry catalog closure) sit in suites where TS1 only added erased type arguments — pre-existing tree/taxonomy/launch drift, owners R10 (launch/taxonomy) and the library suite owners |

## Session 14d — standalone package tsconfigs (resumed 2026-09-29 07:1x)

Goal (coordinator 07:0x): every package's own tsconfig type-checks with 0 errors; fix config/lib/types/paths at the root, real
type errors properly; no suppression, no `any` escapes. Captures ONLY under `.🧬semio/🌐hub/s14-ts1-logs/` (rule 26 sweep took
`wp-ts1/generated/`; the 14c captures named below are gone, their numbers stand as recorded). Runner
`wp-ts1/ts1-tsc-packages.zsh <round> slug=tsconfig…` (sequential, nice 15, load-gated).

| # | package tsconfig | r0 07:13 | now | root cause → fix |
|---|---|---|---|---|
| 1 | machine, 3d | 49, 49 | **0, 0** | `rootDir: "."` (+ unused `baseUrl`) on a noEmit program whose entry imports across the repo (TS6059) → removed |
| 2 | framework | 4 | **0** | `lines-and-columns` ships types outside its `exports` → `paths` to its `.d.ts` (same pattern as `dom-accessibility-api`); pixels selection test cast JSON `number[][]` to `SelectionShape` → fixture admitted through its schema (`compile<…>`, guard) |
| 3 | library | 18 | **0** | standalone config without `allowJs` imported the library's `🟨️.mjs` modules (TS7016) → extends the root baseline + `allowJs` (as the repo program) |
| 4 | test | 39 | **0** | standalone config without Bun types (Bun, `import.meta.dir`, `bun:sqlite`) → extends root, `types: ["bun", "node"]` (`@types/bun` 1.4.2 is installed), includes `📜️script.ts` too |
| 5 | assets | 1160 | **0** | no `extends` (ES5 defaults: TS1501/2802/5097…) → extends root, `types: ["bun", "node"]`; its stale `@types/node ^20` devDependency (no `node:sqlite`) aligned to the workspace `^22.19.6`, `bun install` (lockfile: assets entry + the orphan 20.19.43 row gone; it also recorded stdio-png's already-present devDependencies), orphan nested `node_modules/@types/node` removed; `bun install --frozen-lockfile --dry-run` rc 0 |
| 6 | hub admin | 577 | **0** | standalone config without node/bun types for its tests → extends root, `types: ["bun", "node", "react", "react-dom", "vite/client"]`, `📜️script.ts` no longer excluded; root `lib` += `DOM.Iterable` (NodeList iteration; os keeps its own `DOM.AsyncIterable` override — root AsyncIterable breaks the env-d.ts `BunReadableStream` in repo, 39 errors, measured). **Real bug:** `SpacesPage` `createSpace(…).then(loadSpaces)` fed the terminal receipt in as the page cursor (`cursor=[object Object]`) → `.then(() => loadSpaces())`; regression test red on the old code (`hub-admin-test-red.txt`), suite 22/22 green |
| 7 | cad + 4 extensions | 342, 282×4 | **0, 0×4** | 09-08 test-extraction codemod left `registerTests1(vitest, dependencies: any, …)` with type-only names passed as values (TS2693 — also a runtime missing-export binding) and destructured values used as types (TS2749), plus 58 `type X = any` aliases. One-off codemod `wp-ts1/ts1-cad-test-deps.ts` (TypeScript API): each of the 14 source modules exports `<Dir>TestDependencies` (`typeof` of every injected value), stops passing type-only names, and its suite takes that type + `import type`s of the real refs (342 → 28). `wp-ts1/ts1-cad-manual.py` for the rest: since the 09-03 preview split brepjs called preview-private `readVec3`/`faceNormalFromPoints`/`derivedFacePoints` (**runtime ReferenceErrors**) → exported from `🧮️preview` + imported; suites destructured `preciseSpatialKernelMath` (7) and used `aabbVolume` from the brepjs module that never exported them (`M` was `undefined`) → static import from their owner `🧮️preview` (`__actionsTestKernel` retired, it only fed that); `parseCadBounds` returned `number[]` for its `[x,y,z]` interface → `parseCadBoundsCorner`; `import.meta.env.VITEST === true` (always false, the env is a string) → `=== "true"`; `StatelyMachineSpec` exported for its suite; configs lose unused `baseUrl`/duplicate options, get own `tsBuildInfoFile` (machine, 3d, cad×5 no longer share root's) |
| 8 | os, repo (regression after root lib edit) | 0, 0 | **0, 0** | — |

**Final (08:00, non-incremental `tsc --incremental false`, `wp-ts1/ts1-tsc-all.zsh r5`): all 20 tracked package tsconfigs 0
errors** (cad ×5, hub, hub admin, framework, machine, ui-react, assets, 3d, os, renderer-react, window-kits, repo, vscode,
library, coordinator, test) — `.🧬semio/🌐hub/s14-ts1-logs/r5/summary.txt`.

Runtime proof (native lane): pixels editing `bun test` 58/58; hub admin vitest 22/22 (+ new SpacesPage law, red on the old code);
cad extensions 2/1/2/1 green; cad core: artifact interactions 145/145, spatial + stately + inferences + actions 32/32, geometry
40/44, brepjs 29/30, semio 1/6, runtime + renderer suites fail at module load — **all cad reds pre-existing and outside TS1's
edits**: `source.directory` undefined (the codemod passes Bun's `import.meta.dir`, absent under vite-node) ×2, AEC typology data
not registered ×3, flow wasm `/@fs/` URL unfetchable in node ×5, and a registry import cycle (`runtime → actions → typology →
artifact → 📔️registry` leaves `interactionCompileCacheClear` undefined at `artifact:1014`; TS1 changed no static import of those
five modules — `git diff -U0` import lines 0). Captures `cad-vitest-*.txt`. None of the edited cad/admin sources is in the os
program (`--listFilesOnly`), so no `s` boot applies (rule 20).

Follow-up — **done in 14e item 2** (numbers below were the measured estimate): migrating framework/library/repo/hub from the hand-written env `🌿️environment/🟦️.d.ts` to real
`@types/bun` (its own header asks for it; `@types/bun` is installed) costs library 97 / repo 103 / framework 22 / hub 7 real type
errors (bun:test options, `Bun.spawn` overloads, `FileSink`) — `.🧬semio/🌐hub/s14-ts1-logs/exp/*.txt`.

## Session 14c

| # | item | state | evidence |
|---|---|---|---|
| 0 | baseline | **measured** 84 errors 17:26 (coordinator 17:2x: 93 — hub-document-sweep ×6 + program-matrix ×3 were already fixed by S18 when TS1 started) | `wp-ts1/generated/tsc-os-0.txt` |
| 1 | os tsc 0 errors | **measured** 0 errors, rc 0 (17:37 after the type fixes; 17:51 final, after every edit incl. items 3–4) | `tsc-os-1.txt`, `tsc-os-2.txt` |
| 2 | `[DEBUG]` status lines in `💻️os/📦️packages/🦀️rust/📜️script.ts` | **done** — 4 status lines drop the tag (history-completion, catalog-read-ownership, retained-clone, exact member admission); `/usr/bin/grep -c DEBUG` = 0 | `retained-clone-check-1.txt` (line printed untagged) |
| 3 | runtime proof of every touched suite | **measured green**: `retained-clone-check` rc 0; ordered-map schema probe (fixture true; lookup+value / insert−value / duplicate−value / unknown kind all false); `verify layout-document-contract` rc 0 (56 snapshots, 28 diffs); operation-progress `bun test` 21/21; renderer-react vitest 4 jsdom files 30/30, 4 Chromium files 17/17, engine-contract 6 touched tests 6/6 (708 skipped by `-t`); renderer-wgpu vitest 2 files 4/4 | `retained-clone-check-1.txt`, `wp-ts1/ts1-ordered-map-schema-probe.ts`, `layout-document-contract-2.txt`, `operation-progress-1.txt`, `vitest-react-1..4.txt`, `vitest-wgpu-1.txt` |
| 4 | two runtime reds found while proving | **root-fixed, measured green**: (a) layout `parseFramePatch` rejected every Rust-shaped `FramePatch` (`parseSchemaRecord` throws `unknown field` on `rotation`/`locked`/`visible`) + document-contract committed count stale 50/25 → 56/28 (rotate-frame, update-grid, set-frame-flags); (b) icon-svg-lighting WebGPU oracle: `GPUPipelineError: Vertex attribute slot 10 … not present` — production `WORLD3D_SHADER` gained `@location(10) emissive_cutoff` overnight, the oracle's instance layout lacked it | `layout-document-contract-1.txt` (red: count) → `-2.txt` green; `vitest-react-2.txt` (red) → `vitest-react-3.txt` 3/3 |
| 5 | rule 20 (shell bundle) | **not reached** — no edit is shell-bundle code: all are tests / test support / the kernel task router / a fixture schema; the layout diff module is reached in the os program only through its document-contract test (`tsc --explainFiles`), and `@semio-tech/layout-js` has no importer. No boot needed | `layout-diff-importers.txt` |

### Error causes and origins (84 at 17:26)

| file | n | cause | fix | origin |
|---|---|---|---|---|
| `💻️os/📦️packages/🦀️rust/📜️script.ts` | 53 | `new Ajv2020().compile(schema)` is `ValidateFunction<unknown>`; `assert(validate(x))` narrows the `any` fixture to `unknown` | schema-typed fixtures (region `🧬️RetainedCloneFixtures`, 4 fixtures, `compile<T>`); ordered-map schema tightened to `oneOf` lookup (no value) / insert\|duplicate (value required) so the type is the schema | retained-clone-check, overnight Codex peer (auto-commit 3b2f1181d27, 28th 11:43) |
| `🧪️tests/🔬️engine-contract` | 8 | 5× camera state lacks `WorldParsedCameraState.projectionFrame`; 3× `.component.value` on the `Component` union | `projectionFrame: "content"` (merge law uses `"preserveCamera"` and now asserts merge keeps it); `toMatchObject({ type: "text", value })` | `projectionFrame` + those asserts overnight (3b2f1181d27) |
| `🧪️tests/🖱️world3d-interaction` | 4 | JSON camera `number[]` passed where the gizmo pose needs `readonly [n,n,n]` (invalid `as` casts) | `vector3` / `gizmoPose` helpers (length-checked tuples) | lines from auto-commit 5bcb2da23da (27th 21:54) |
| `🧪️tests/🎨️world3d-glb-material` | 4 | `new Map([[FrontSide, …]])` infers key `0\|2` from `three`'s literal consts; `.get(material.side: Side)` | `Map<Side\|Wrapping\|TextureFilter, string>` | test created overnight (3b2f1181d27); imports `three` directly — NOT R10's `@semio-tech/ui-react` re-export |
| `🔌️plugin/⏳️operation-progress/🧪️tests` | 3 | JSON row unions vs `toBe` | `compile<OperationCancellationArgsV1\|CancellationOutcomeV1>`, branch on the guard | overnight (3b2f1181d27) |
| `🧪️tests/🩺️wgpu-runtime-diagnostics` | 3 | owned `OwnedBuildPlugin` handed to real Vite `createServer` (overload falls to `ResolvedConfig`) | `as Plugin` at the owned↔tool test seam (precedent: `🎨️styling/🧪️tests/🧩️suite`) | overnight (3b2f1181d27) |
| `🧪️tests/🧩️block-list-presentation` | 2 | `exact` is not a `ByRoleOptions` key (@testing-library/dom 10.4.1; ByRole name matching is always `===`) | option removed (same semantics, verified in `dom.cjs.js` `matches`) | 27th 11:30 (6b8089dcb21) |
| `🧪️tests/📤️asset-cancellation` | 2 | `page.evaluate` arg `any` → implicit-any row | `(stages: readonly string[])` | overnight |
| `🎬️MediaTransportHost/🧪️tests/♻️lifecycle` | 1 | JSON `outputPort: string` vs `"playback:out"` | fixture admitted through its schema (`compile<BrowserInput>`, exported from `🌐️browser.tsx`) before use | overnight |
| `🧪️tests/🎨️icon-svg-lighting` | 1 | `Float32Array<ArrayBufferLike>` vs `GPUAllowSharedBufferSource` | `Float32Array<ArrayBuffer>` | overnight |
| `🧪️tests/🎨️chrome-palette` | 1 | write to readonly `UiTheme.id` | spread copy with `id` | overnight |
| `🧪️tests/🎥️world3d-camera-framing` | 1 | `projectionFrame` missing | `projectionFrame: "content"` | type change overnight |
| layout `🧬️schema/🔺️diff/🟦️.ts` | 1 | `FramePatch` interface gained `rotation`/`locked`/`visible`, `parseFramePatch` did not (also a runtime fault, item 4a) | `boolean` parser + the 3 fields in schema order | W4 `wp-w4/w4-layout-diff-schema.py` (27th 18:38) |

### Session 14e log

- 08:1x cycle analysis (`ts1-import-cycles.ts`), test-region import move, `executeAction` delegate removed → registry cycle gone; module-URL fixtures; brep wasm by module URL. 08:26 CAD suites load (runtime/renderer). 08:3x Bun migration: 133 unique errors → options order codemod, matcher alignment, real fixes; env d.ts deleted; root `DOM.AsyncIterable`. 08:53 all 20 configs 0; 08:5x bun suites (edited-runtime 21/22, hub law fixed → 1/1).

### Session 14d log

- 07:1x resumed; r0 inventory (12 failing package configs, 2 912 errors). 07:2x machine/3d/framework/library/test/assets green; `bun install` (assets `@types/node`). 07:3x hub admin green + SpacesPage fix + red/green law. 07:4x root lib `DOM.Iterable` (AsyncIterable reverted after repo regression 39). 07:5x cad codemod + manual set → 0. 08:00 all 20 configs 0 (non-incremental). 08:0x cad/admin/pixels runtime runs.

### Session 14c log

- 17:25 read preamble (1–23 + 14b + 14c), AGENTS.md, fleet tail. Baseline tsc 84 errors (29 s).
- 17:32 script.ts typed fixtures + ordered-map schema `oneOf`; `retained-clone-check` rc 0; schema probe rejects the 4 malformed variants.
- 17:37 renderer/plugin/layout test fixes → tsc 0 errors.
- 17:43 layout document-contract: per-file schema+parse asserts pass for all 28 diffs incl. frame patches; only the committed count was stale → 56/28, rc 0.
- 17:44–17:50 vitest via native lane (`wp-ts1/ts1-vitest-react.zsh`, `--fileParallelism=false`, one browser at a time): all green after the icon-svg-lighting slot-10 layout fix; operation-progress 21/21.
- 17:51 final tsc 0 errors. Load 47, swap 11.3/12 GB during the run; no serve/hub/browser left running.
