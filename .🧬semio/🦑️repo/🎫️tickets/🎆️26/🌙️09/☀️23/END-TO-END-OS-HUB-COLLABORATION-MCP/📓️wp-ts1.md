# TS1 — os TypeScript program type-checks with 0 errors (session 14c)

Slice TS1, session 14c, 2026-09-28 17:2x (new slice, no predecessor). Scope: `🧰️framework/🛍️products/💻️os` —
`bunx tsc --noEmit -p tsconfig.json` must report 0 errors; root-fix each error at its cause (no `as any`, no `@ts-ignore`,
no loosened types; fixtures typed from their schema). Host TS + test files only (guest freeze, rules 2/22). Expendable
captures `wp-ts1/generated/`. Rules: `📓️session-14-preamble.md` (+ 13/12).

Status legend: **measured** = ran here, command + capture named; **unverified** = read from source only; **written, not run**.

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

### Session 14c log

- 17:25 read preamble (1–23 + 14b + 14c), AGENTS.md, fleet tail. Baseline tsc 84 errors (29 s).
- 17:32 script.ts typed fixtures + ordered-map schema `oneOf`; `retained-clone-check` rc 0; schema probe rejects the 4 malformed variants.
- 17:37 renderer/plugin/layout test fixes → tsc 0 errors.
- 17:43 layout document-contract: per-file schema+parse asserts pass for all 28 diffs incl. frame patches; only the committed count was stale → 56/28, rc 0.
- 17:44–17:50 vitest via native lane (`wp-ts1/ts1-vitest-react.zsh`, `--fileParallelism=false`, one browser at a time): all green after the icon-svg-lighting slot-10 layout fix; operation-progress 21/21.
- 17:51 final tsc 0 errors. Load 47, swap 11.3/12 GB during the run; no serve/hub/browser left running.
