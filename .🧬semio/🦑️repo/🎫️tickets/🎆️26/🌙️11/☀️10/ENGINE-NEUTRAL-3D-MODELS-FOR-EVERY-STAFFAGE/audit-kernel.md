# Compliance audit: engine-neutral 3D model kernel

Read-only audit. No source file was modified and no state-changing git command was run. The `repo` MCP server was unavailable (connection closed) and was not used. Paths are relative to `🧰️framework/🔨️modules/🧊️3d/🗿️model/` (abbreviated `M/`) unless stated; wiring files are relative to `🧰️framework/🔨️modules/🧊️3d/` (abbreviated `3D/`). Line numbers were taken from the files as they stood at audit time.

Inputs read: root `AGENTS.md`, `3D/AGENTS.md`, and in the ticket folder `design.md` (including Amendments A1-A6), `kernel-core.md`, `kernel-maintenance.md`, `glb-encoder.md`, `fbx-encoder.md`, `model-viewer.md`. Scope: 105 files under `M/` (including caches) plus the wiring files `3D/🧪️tests/🎚️config/🟦️.ts`, `3D/📦️packages/🟦️typescript/package.json`, `3D/📦️packages/🟦️typescript/🗿️model/🟦️.ts`.

## Summary

- Rule 1 has one violation, in the wiring config (comment inside the `test: {` object). No comments exist anywhere in `M/`. All 289 docstrings start with an emoji, but about 60 emoji are reused within a file.
- No npm or bare-specifier import exists in non-test kernel code. Platform APIs only (TextEncoder/TextDecoder, setTimeout, DOM/WebGL2 inside `🖥️viewer`). Test oracles: three, ajv, gltf-validator, glTF-Transform, assimpjs, fflate, playwright, esbuild.
- Largest gaps: software rasteriser and `validateModel`/`tessellateSolid` have no progress/cancellation; `encodeGlb`/`encodeFbx`/viewer are not re-exported from the kernel barrel or the `./model` package subpath; `@gltf-transform/extensions` is undeclared in any package.json.
- Schema-first holds: all 28 `$defs` have twin types and every design field exists. Two twin exports (`LoftSection`, `Operation`) have no `$defs` entry. The schema file is produced by a ticket-folder TypeScript generator.
- Path lengths are fine: max 157 code points (163 UTF-16 units) for `M/`, nothing over 200. Out of kernel scope, 293 files in the ticket folder (Python venvs under `🗑️generated/oracle-tools`) exceed 256 UTF-16 units.

## Top 15 prioritised findings

1. Comment inside a definition: `3D/🧪️tests/🎚️config/🟦️.ts:24-27` (inside `test: {` opened at line 20). Rule 1.
2. Software rasteriser has no progress and no cancellation: `M/🖥️viewer/🖼️software/🟦️.ts:55` `render(frame: Frame, style: RenderStyle): void {`. Rule 6.
3. `validateModel` is exported from the barrel, synchronous, has no `Operation`, and is documented as O(n^2): `M/✅️validation/🟦️.ts:44`; `M/🔺️tessellation/🟦️.ts:54` `tessellateSolid` (barrel export) also takes no `Operation`. Rule 6.
4. Cancellation is coarse: `tessellate` checks only between solids (`M/🔺️tessellation/🟦️.ts:141,146`); `featureEdges` only between meshes (`M/📏️edges/🟦️.ts:15`); one large `deflateZlib` call cannot be interrupted (`M/📤️export/🎬️fbx/🗜️deflate/🟦️.ts:303`). Rule 6.
5. Design says `encodeGlb` and `encodeFbx` are kernel API (names fixed), but the barrel `M/🟦️.ts:3-8` does not export them, nor the viewer; `3D/📦️packages/🟦️typescript/🗿️model/🟦️.ts:1` re-exports only the barrel, so the `./model` subpath cannot encode. Undocumented deviation (not listed in `kernel-core.md` "Deviations"). Rules 3 and design.
6. Exported viewer API uses DOM types: `M/🖥️viewer/🕹️controller/🟦️.ts:40,44,79,244` (`HTMLCanvasElement`, `AbortSignal`), `M/🖥️viewer/🧩️element/🟦️.ts:14` (`CustomElementRegistry`, `CustomElementConstructor`), `M/🖥️viewer/🔶️webgl2/🟦️.ts:37,226`. Browser platform types, not third-party, but a strict reading of rule 3 flags them. Rule 3.
7. Undeclared test oracle: `M/📤️export/🧊️glb/🧪️tests/🧪️oracles/🟦️.ts:8` imports `@gltf-transform/extensions`, declared in no package.json. `@gltf-transform/core` is declared only in the sibling workspace `✏️s/package.json:337`, not in the root or `3D/` package. Rule 2.
8. Missing language-agnostic fixture and oracle for `⏱️operation`: `M/🧪️tests/⏱️operation/🟦️.ts:5` reuses the box fixture and imports no third-party library. Rule 10.
9. Oracle registry `M/🔮️oracles/🔣️.json` lists 2 oracles (gltf-validator, assimpjs) though tests use at least seven; its `"$schema"` (line 2) points to `🧰️framework/🧪️test/🧬️schema/🔣️.json`, which does not exist. Rules 4 and 10.
10. Schema-first is achieved through a generator: the schema is written by the ticket script `T/generate-model-schema.ts` (`writeFileSync`), not hand-authored (inferred from the file contents). Twin exports `LoftSection` (`M/🧬️schema/🟦️.ts:44`) and `Operation` (line 95) have no `$defs` entry. Rule 9.
11. GLB expected values were "recorded from the first reviewed output of this encoder (golden)" (`glb-encoder.md`), so byte expectations are self-generated; third-party oracles check structure, not bytes. Rule 10.
12. Viewer error events carry raw English `Error` objects: `M/🖥️viewer/🕹️controller/🟦️.ts:189,272` `this.emit("error", { error: error as Error })`. Rule 7.
13. German text is ASCII-transliterated in the validator (`M/✅️validation/🟦️.ts:12`, "Laenge", "muessen", "ueber") but uses umlauts in FBX (`M/📤️export/🎬️fbx/🟦️.ts:147`, "nicht unterstützt"). Inconsistent. Rule 7.
14. Docstring emoji reused within files, for example `🧮️` nine times in `M/📤️export/🎬️fbx/🧮️matrix/🟦️.ts` and `📖️` eight times in `M/📤️export/🎬️fbx/📖️reader/🟦️.ts`. Rule 1 (docstring clause).
15. Duplicated math and matrix code: FBX `🧮️matrix` (`multiply`, `invert`, `compose`, `quaternionMultiply`, `conjugate`, `slerp`, `worldMatrices`) duplicates kernel `🧮️math` (`multiply4`, `invert4`, `compose`, `qMul`, `qConj`, `qSlerp`); three world-matrix composers exist (`restWorldMatrices`, `poseModel`, `worldMatrices`). Rule 11.

Also noted (lower priority): a silent conditional skip in the GPU test (`M/🖥️viewer/🧪️tests/🔶️gpu/🟦️.ts:16`), four vitest caches inside the kernel, `fallback` runtime path in the viewer, and the frozen pre-maintenance kernel copy in the ticket folder (see rules 4 and 5).

---

## Rule 1: comments inside definitions and docstrings

Method: `grep` for `//`, `/*` and indented `/**` across all `.ts` under `M/` and the wiring files; `perl` extraction of the first grapheme after each `/**`.

Violations:
- `3D/🧪️tests/🎚️config/🟦️.ts:24-27`, inside the `test: {` object literal (opened line 20):
  `    // 🩹️ In-source (`import.meta.vitest`) suite in `../../🟦️.ts` — `include` names ACTUAL TEST FILES,`
  `    // silently collecting zero tests while `nx test` reported success. See the os-dev/replication`
  Lines 26-27 continue the comment. It is inside a definition, so it violates the rule. It also refers to `nx`, which no longer appears in the setup.

Other comments found:
- `M/🖥️viewer/🧪️tests/🕹️controller/🟦️.ts:1` `// @vitest-environment jsdom`: a file-level pragma outside any definition, acceptable.
- No `/* */` block comments, no commented-out code, and no `//` lines inside GLSL shader strings in `M/🖥️viewer/🔶️webgl2/🎨️shaders/🟦️.ts`.

Docstrings:
- Checked: 289 `/**` docstrings across `M/**/*.ts`. All start with an emoji. No multi-line `/**` with the emoji on a later line.
- Indented `/**` (class members only): `M/🖥️viewer/🕹️controller/🟦️.ts:133-570`, `M/🖥️viewer/🖼️software/🟦️.ts:48`, `M/📤️export/🧊️glb/🧱️buffer/🟦️.ts:32,37,74`. These sit above members, not inside function bodies.
- Emoji reused within one file (file header counts as one docstring). Format: file, emoji, count, docstring line numbers:
  - `📏️edges/🟦️.ts` 📏️ x2 (1, 11)
  - `📤️export/🎬️fbx/🎞️animation/🟦️.ts` 📈️ x3 (8, 13, 19)
  - `📤️export/🎬️fbx/🎨️material/🟦️.ts` 🎨️ x2 (1, 10)
  - `📤️export/🎬️fbx/📖️reader/🟦️.ts` 📖️ x8 (1, 9, 11, 13, 19, 103, 108, 113)
  - `📤️export/🎬️fbx/🗜️deflate/🟦️.ts` 🗜️ x7 (1, 9, 252, 302, 313, 330, 424)
  - `📤️export/🎬️fbx/🟦️.ts` 🎬️ x3 (1, 140, 436)
  - `📤️export/🎬️fbx/🦴️mechanism/🟦️.ts` 🦴️ x3 (1, 18, 54)
  - `📤️export/🎬️fbx/🧪️tests/🧰️support/🟦️.ts` 🧫️ x5 (8, 14, 19, 24, 31)
  - `📤️export/🎬️fbx/🧪️tests/🧰️support/🧩️assimpjs/🟦️.ts` 🧩️ x4 (1, 3, 5, 8)
  - `📤️export/🎬️fbx/🧮️matrix/🟦️.ts` 🧮️ x9 (1, 10, 22, 53, 60, 66, 71, 76, 81); 🧭️ x2 (94, 109)
  - `📤️export/🎬️fbx/🧱️records/🟦️.ts` ⏱️ x2 (51, 54); 🏗️ x4 (18, 34, 39, 46)
  - `📤️export/🧊️glb/🟦️.ts` 🧊️ x2 (1, 178)
  - `📤️export/🧊️glb/🧱️buffer/🟦️.ts` 📏️ x3 (11, 13, 32); 📦️ x2 (74, 86); 🔢️ x2 (6, 15); 🧱️ x2 (1, 25); 🧾️ x2 (18, 20)
  - `🔺️tessellation/🌀️revolved/🟦️.ts` 🌀️ x2 (1, 106)
  - `🔺️tessellation/🔺️polygon/🟦️.ts` 🔺️ x2 (1, 19)
  - `🔺️tessellation/🟦️.ts` 🔺️ x2 (1, 129)
  - `🔺️tessellation/🧱️builder/🟦️.ts` 🧱️ x3 (1, 10, 44)
  - `🖥️viewer/🎞️frame/🟦️.ts` 🎞️ x3 (1, 16, 44)
  - `🖥️viewer/🎯️picking/🟦️.ts` 🎯️ x3 (1, 11, 26)
  - `🖥️viewer/📷️camera/🟦️.ts` 🎯️ x2 (56, 128); 🏛️ x3 (14, 24, 36); 📷️ x3 (1, 16, 33); 🧭️ x2 (18, 39); 🧮️ x2 (102, 113)
  - `🖥️viewer/🔶️webgl2/🟦️.ts` 🔶️ x3 (1, 12, 225)
  - `🖥️viewer/🕹️controller/🎨️color/🟦️.ts` 🎨️ x2 (1, 11)
  - `🖥️viewer/🕹️controller/🟦️.ts` 📣️ x2 (59, 133); 🔌️ x2 (39, 415); 🕹️ x2 (1, 77)
  - `🖥️viewer/🖼️software/🟦️.ts` 🖼️ x3 (1, 13, 48)
  - `🖥️viewer/🧩️element/🟦️.ts` 🧩️ x2 (1, 13)
  - `🖥️viewer/🧬️contract/🟦️.ts` 🎨️ x2 (13, 37); 🦴️ x2 (11, 35)
  - `🖥️viewer/🧱️scene/🟦️.ts` 🧱️ x3 (1, 66, 175)
  - `🧬️schema/🟦️.ts` 📐️ x3 (15, 17, 19); 🔁️ x2 (23, 30); 🗿️ x2 (1, 73); 🦴️ x2 (26, 56); 🧊️ x2 (32, 48); 🧵️ x2 (44, 46)
- Emoji fit was not judged beyond this check.

## Rule 2: runtime dependencies

Non-test code (`M/**/*.ts` excluding `🧪️tests`, `🧰️support`, `node_modules`):
- Third-party runtime imports: none. Every import is relative (`./`, `../`). No `require(`. No dynamic `import(`.
- Node built-ins (`node:*`): none in non-test code.
- Platform (framework-provided) APIs, not behind an owned interface unless stated:
  - `TextEncoder`/`TextDecoder`: `M/📤️export/🎬️fbx/🧱️records/🟦️.ts:125`, `M/📤️export/🎬️fbx/📖️reader/🟦️.ts:17`, `M/📤️export/🧊️glb/🧱️buffer/🟦️.ts:88`.
  - `setTimeout`/`Date.now` (cooperative yielding): `M/🔺️tessellation/🟦️.ts` (`yieldToLoop`, `tessellate` clock), `M/📏️edges/🟦️.ts:17`, `M/🖥️viewer/🧱️scene/🟦️.ts` (`sceneOfAsync`).
  - Browser APIs, confined to `M/🖥️viewer/`: `customElements`, `HTMLElement`, `document.createElement`, `ResizeObserver`, `matchMedia`, `requestAnimationFrame`, `globalThis.devicePixelRatio`, `ImageData` (`🕹️controller/🟦️.ts:117,204,209,448,455,456`), `WebGL2RenderingContext` (`🔶️webgl2/🟦️.ts:15,38`). The viewer's `RenderBackend` interface (`🧬️contract/🟦️.ts:84`) is the owned seam; WebGL2 and software backends implement it.
  - FBX compression is behind the owned `Compressor` interface (`M/📤️export/🎬️fbx/🗜️deflate/🟦️.ts:9-11`). The default `ownedCompressor` is owned. No zlib, fflate, or pako at runtime.
- Verdict: no runtime third-party dependency. Platform APIs are used directly (acceptable per "system libraries provided by the frameworks"), except the DOM types in exported signatures (rule 3).

Test imports (third-party oracles, by test folder):
- `M/🧪️tests/`: `three` in box, ellipsoid, capsule, revolve, extrude, loft, skinning, pose-animation, feature-edges, provenance, polygon (through `🧰️support`), plus three `BufferGeometryUtils` and `RoundedBoxGeometry` (support and box). `ajv` in validation, provenance and `🧰️support/🧊️solids`. `repeat` and `operation` import only vitest; `repeat` uses three indirectly via `🧰️support`.
- `M/📤️export/🧊️glb/🧪️tests/🧪️oracles/🟦️.ts`: `gltf-validator`, `@gltf-transform/core`, `@gltf-transform/extensions`, `three/examples/jsm/loaders/GLTFLoader.js`, `node:fs`. `🧊️vectors` uses `node:crypto` (SHA-256) and `node:fs`.
- `M/📤️export/🎬️fbx/🧪️tests/`: `three-roundtrip` (three, `FBXLoader.js`); `second-reader` (three); `deflate` (`fflate`, `node:zlib`); `🧰️support/🧩️assimpjs` (`assimpjs`, loaded through a variable-named dynamic `import()` that greps miss, and `node:crypto`, `node:fs`, `node:url`); `structure` (`node:fs`); `rejections` (none).
- `M/🖥️viewer/🧪️tests/`: `frame`, `picking`, `camera`, `raster`, `controller` import three; `🔶️gpu` imports `playwright`, `esbuild`, `node:path`, `node:url`; `webgl2` and `scene` import no third-party module directly (`🧰️support` imports three).
- Config: `3D/🧪️tests/🎚️config/🟦️.ts` imports `vitest/config`, `node:path`, `node:url`, and a monorepo package (`🏃️process`). Test-only.

## Rule 3: exported API that exposes external types

- Root barrel `M/🟦️.ts:3-8` exports only owned types and functions (`export type *` from the schema twin; `validateModel`, `tessellate`, `tessellateSolid`, `restWorldMatrices`, `silentOperation`, `CancelledError`, `ModelInvalidError`, `poseModel`, `sampleMaterials`, `featureEdges`, math helpers). Signatures use only owned types, plus `number[]`/`Float32Array` built-ins. Verdict: none found.
- Encoders: `encodeGlb` (`M/📤️export/🧊️glb/🟦️.ts:179`) and `encodeFbx`/`encodeFbxDetailed` (`M/📤️export/🎬️fbx/🟦️.ts:141,437`) use owned types (`Operation`, `GlbOptions`, `FbxOptions`, `FbxResult`, `Compressor`) and return `Uint8Array`. Verdict: no external type. But see the barrel gap below.
- Viewer, platform (DOM/WebGL) types in exported signatures. Not third-party libraries, but outside the codebase, so flagged under a strict reading:
  - `M/🖥️viewer/🕹️controller/🟦️.ts:40` `BackendFactory = (canvas: HTMLCanvasElement, …)`
  - `M/🖥️viewer/🕹️controller/🟦️.ts:44,79` `ModelViewerOptions.canvas`, `ModelViewer.canvas: HTMLCanvasElement`
  - `M/🖥️viewer/🕹️controller/🟦️.ts:244` `setDocument(document, signal?: AbortSignal)`
  - `M/🖥️viewer/🧩️element/🟦️.ts:14` `defineModelViewerElement(…, registry: CustomElementRegistry = customElements): CustomElementConstructor`
  - `M/🖥️viewer/🔶️webgl2/🟦️.ts:13,37` `WebGl2Backend` with public `readonly canvas: HTMLCanvasElement`; `🔶️webgl2/🟦️.ts:226` `createWebGl2Backend(canvas: HTMLCanvasElement, …)`
  - `M/🖥️viewer/🖼️software/🟦️.ts:49` `present(canvas: {…}, ImageDataClass: new (…) => unknown)`: structural, acceptable.
- Barrel gap (design deviation, not a type leak): encoders and viewer are absent from `M/🟦️.ts:3-8`. `3D/📦️packages/🟦️typescript/🗿️model/🟦️.ts:1` (`export * from "../../../🗿️model/🟦️.ts"`) therefore cannot reach GLB/FBX/viewer through `@semio-tech/framework-3d-js/model`. Clients must use deep paths. Rule "MUST reexport explicitly if the client needs it" applies.

## Rule 4: leftover temporary code

- `[DEBUG]`, `console.`, `TODO`, `FIXME`, `XXX`, `debugger`: none in `M/` or the wiring files.
- `.only`, `.todo`, `xit`, `xdescribe`, `skipIf`, `runIf`: none. The `.skip` hit is `M/🖥️viewer/🧪️tests/🔶️gpu/🟦️.ts:16` `return ctx.skip();` inside `catch` when Chromium cannot be launched. No reason string is written in the code; the reason is implicit. The kernel-maintenance report records it as "1 skipped (Playwright GPU test, no Chromium)".
- Commented-out code: none.
- Generated caches inside the source tree (inferred: vitest cache, gitignored by `.gitignore:530 node_modules/`, but present in the replicated worktree):
  - `M/node_modules/.vite/vitest/fc406c8545aa0b35e82c1cad6075846fde7c9c8c/results.json`
  - `M/📤️export/🎬️fbx/node_modules/.vite/vitest/d7aa1eda937ae00a9d08bfde356bc5874825ebb2/results.json`
  - `M/📤️export/🧊️glb/node_modules/.vite/vitest/1f610234f6f50369f2e82d758d19e1d79fcdd614/results.json`
  - `M/🖥️viewer/node_modules/.vite/vitest/bb6dbff2eec10b576daa5e60034c3d99d250eeeb/results.json`
- Oracle registry with a dangling reference: `M/🔮️oracles/🔣️.json:2` `"$schema": "../../../🧪️test/🧬️schema/🔣️.json"` (target missing); the file lists two oracles (line 7 `gltf-validator-glb`, line 42 `assimpjs-fbx`), though tests import more (see rule 2). Its `_comment` (line 3) claims both packages are root devDependencies; `@gltf-transform/*` is not in the root package.json.
- Scratch-looking material in the ticket folder, out of kernel scope: `🗑️generated/kernel-maintenance/` (dbg.ts, oldfail.ts, volumes-*.json, repo-run.txt, viewer-fixtures-before/, fx/, old/). Rules say to delete `🗑️generated` at ticket close.

## Rule 5: legacy and compatibility constructs

- Deprecation markers, `legacy`, `compat`, `shim`, `migrat*`, `@ts-`, `eslint`: none in `M/` or the wiring.
- Migration scripts: none in `M/`.
- `as any` / `: any`: only in tests and `M/🧪️tests/🧰️support/🟦️.ts:9`, `M/🧪️tests/✅️validation/🟦️.ts:8-13`.
- "fallback" code paths:
  - Runtime fallback to the software rasteriser when WebGL2 is unavailable: `M/🖥️viewer/🕹️controller/🟦️.ts:42,47,188` (`fallback` option, `if (this.options.fallback === false)`), `:448` (`this.backend.present(...)`), docstring `M/🖥️viewer/🖼️software/🟦️.ts:48`. This is a runtime path, not legacy. Flagged because the rule names "fallback".
  - Numeric or geometric defaults named "fallback" (not compatibility): `M/🧮️math/🟦️.ts:16` (`normalize(..., fallback)`), `M/🔺️tessellation/🧵️loft/🟦️.ts` and `🌀️revolved` ("fallback Z" for the lateral axis), `M/🧬️schema/🟦️.ts:44` (docstring).
- Stale tool reference: `3D/🧪️tests/🎚️config/🟦️.ts:26` ("nx test"). Stale claim: the same comment says the suite is in-source (`import.meta.vitest`) at `../../🟦️.ts`. Lines 24-27 are a historical note.
- Ticket folder, to delete at close (out of kernel scope): `🗑️generated/kernel-maintenance/old/model/` holds 12 frozen copies of pre-maintenance kernel files, used by `T/kernel-maintenance-regress.ts` to compare outputs. This is a legacy comparison copy.

## Rule 6: progress and cancellation for expensive operations

Exported or potentially long-running functions. "Partial" means the loop checks are coarser than the work.

| Function (file:line) | Async | Operation param | report | cancelled() checked | Verdict |
|---|---|---|---|---|---|
| `tessellate` (`M/🔺️tessellation/🟦️.ts:130`) | yes | yes | yes, per solid (line 142), final (198) | per solid (141, 146) | Partial: one solid cannot be interrupted; `validateModel` runs first without progress |
| `tessellateSolid` (`🔺️tessellation/🟦️.ts:54`) | no | no | no | no | Violation if treated as expensive (barrel export) |
| `tessellateLoft`, `tessellateExtrude`, `tessellateRevolve`, `tessellateCapsule`, `tessellateEllipsoid`, `tessellateBox` (`🧵️loft:13`, `🏗️extrude:23`, `🌀️revolved:52,69,124`, `📦️box:19`) | no | no | no | no | Violation if treated as expensive; not in barrel |
| `triangulatePolygon` (`M/🔺️tessellation/🔺️polygon/🟦️.ts:20`, O(n^2) ear clipping) | no | no | no | no | Violation (exported module function) |
| `validateModel` (`M/✅️validation/🟦️.ts:44`, O(n^2) edge pairs per `kernel-core.md`) | no | no | no | no | Violation (barrel export) |
| `featureEdges` (`M/📏️edges/🟦️.ts:12`) | yes | yes | yes, per mesh (16) | per mesh (15) | Partial: edge loop in `linesOf` has no checks |
| `encodeGlb` (`M/📤️export/🧊️glb/🟦️.ts:179`) | no (sync) | yes | `step()` at phases, per primitive (218), per clip (257), end (276-278) | yes, inside `step` | Compliant at phase granularity; a single primitive or clip packs without checks |
| `encodeFbx` / `encodeFbxDetailed` (`M/📤️export/🎬️fbx/🟦️.ts:141,437`) | no (sync) | yes | per node (285), per primitive (213), per channel (361), serialize (429), end (432) | `check()` at the same points; `writeNode` checks (`🧱️records:184`) | Partial: one array deflate (`🗜️deflate/🟦️.ts:303`) is uninterruptible |
| `deflateZlib`, `inflateZlib`, `deflateRaw`, `inflateRaw` (`M/📤️export/🎬️fbx/🗜️deflate/🟦️.ts:253,303,331,425`) | no | no | no | no | Violation if treated as expensive; exported module functions |
| `readFbx` (`M/📤️export/🎬️fbx/📖️reader/🟦️.ts:20`) | no | no | no | no | Violation if treated as expensive |
| `buildScene` / `sceneOf` / `sceneOfAsync` (`M/🖥️viewer/🧱️scene/🟦️.ts:67,176,182`) | generator / yes | yes | per mesh node (114), end (171) | yes (113) | Compliant; `sceneOfAsync` yields every 12 ms |
| `SoftwareBackend.render` (`M/🖥️viewer/🖼️software/🟦️.ts:55`) | no | no | no | no | Violation (named explicitly by the rule) |
| `WebGl2Backend` render path (`M/🖥️viewer/🔶️webgl2/🟦️.ts`) | no | no | no | no | GPU path; no progress or cancellation (low) |
| `ModelViewer.setDocument` (`M/🖥️viewer/🕹️controller/🟦️.ts:244`) | yes | yes (`part()` 250) | yes, via `progress` event | `AbortSignal` and `disposed` (250) | Compliant: `tessellate` at 0 to 0.5, `sceneOfAsync` at 0.5 to 1 |
| `poseModel`, `sampleMaterials` (`M/🦴️mechanism/🟦️.ts:101,114`) | no | no | no | no | Cheap per call; acceptable |
| `validateModel` callers in the viewer (indirect) | no | no | no | no | Inherits the violation above |

## Rule 7: text for humans

- Validator messages (`M/✅️validation/🟦️.ts:10-40`): all bilingual `{ en, de }`. German is ASCII-transliterated, for example line 12 `"bad-type": { … de: "Der Wert hat den falschen Typ oder die falsche Laenge." }`; also "muessen" (13, 27), "ueber" (21), "beruehrt" (29, 30), "Schluesselzeiten" (39). FBX German uses umlauts (`M/📤️export/🎬️fbx/🟦️.ts:147`: "nicht unterstützt"). Inconsistent.
- Mechanism warnings (`M/🦴️mechanism/🟦️.ts:13-14`): bilingual.
- FBX `FbxEncodeError` and notices (`M/📤️export/🎬️fbx/🟦️.ts:40-42`, `🦴️mechanism:61`): bilingual. Exception: `M/📤️export/🎬️fbx/🟦️.ts:176` `fail("node-order", (error as Error).message, "Knoten müssen ihren Eltern folgen.")` uses a raw English JavaScript message as the `en` text.
- Viewer: all human text is client-supplied through `ViewerStrings`; there is no default language (`M/🖥️viewer/🧩️element/🟦️.ts:45` throws if `strings` is missing). Raw English error objects reach the client through `M/🖥️viewer/🕹️controller/🟦️.ts:189,272` `emit("error", { error })`. Clients that display it show English. Recommend a bilingual `Text` error or a code.
- English-only error classes and messages (developer-facing, acceptable per rule, listed):
  - `CancelledError` (`M/🔺️tessellation/🟦️.ts:20`, message "cancelled").
  - `ModelInvalidError` (`🔺️tessellation/🟦️.ts:28`, "invalid model: <code> at <path>").
  - `FbxCancelledError` (`M/📤️export/🎬️fbx/🟦️.ts:35`, "encodeFbx cancelled").
  - `SceneError` (`M/🖥️viewer/🧱️scene/🟦️.ts:10,20,25-31,68,79-94`).
  - Plain `Error` in GLB (`M/📤️export/🧊️glb/🟦️.ts:41-225`), FBX reader (`📖️reader:22,66,71,90`), deflate (`🗜️deflate:339,369,384,416,426,428`), viewer (`🕹️controller:254,287,300,350,426`, `🎞️frame:56`, `🔶️webgl2:39,52,62`, `🖼️software:57`, `🧩️element:45`), mechanism (`🦴️mechanism:103,118,128`), math-less paths.
- Hard-coded user-facing strings in viewer CSS (`🕹️controller/🟦️.ts:74`, `🧩️element`) are not human text.

## Rule 8: path length

- Measured all 105 files under `M/`, including `node_modules` caches. Absolute path prefix: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/` (62 code points).
- Maximum absolute path: 157 code points, 163 UTF-16 units (Windows counting), 187 UTF-8 bytes. Longest file: `M/📤️export/🎬️fbx/node_modules/.vite/vitest/d7aa1eda937ae00a9d08bfde356bc5874825ebb2/results.json`. Paths over 200 characters: none. Paths over 256 UTF-16 units: none.
- Longest repo-relative path (from `/Users/ueli/Documents/semio/`): 129 code points, same file.
- Longest non-cache file: `M/📤️export/🎬️fbx/🧫️fixtures/🎞️rotation-continuity/🔣️.json`, 138 code points absolute. The four cache files are the longest overall.
- Ticket folder copies of kernel source (`🗑️generated/kernel-maintenance/old/model/…`, 12 files): max 219 UTF-16 units; not over 256, so the copy clause is not violated.
- Out of kernel scope, ticket folder: 7215 files; 293 are over 256 UTF-16 units (max 299). All are under `🗑️generated/oracle-tools/venv-documents` and `venv-314` (Python packages). This violates the 256 path rule and the "delete generated" rule.

## Rule 9: schema-first consistency

- Schema `M/🧬️schema/🔣️.json` `$defs` (28): Animation, BoxSolid, CapsuleSolid, Channel, EllipsoidSolid, ExtrudeSolid, Issue, Joint, LoftSolid, Mat4, Material, Mesh, ModelDocument, Node, Pose, Primitive, PrimitiveSolid, Quality, Repeat, RevolveSolid, Skin, Slug, Solid, Text, TransformPatch, Variant, Vec2, Vec3, Vec4.
- Twin `M/🧬️schema/🟦️.ts`: each of the 28 has an exported type of the same name (`Slug` line 12, `Text` 14, `Vec2-4` 16-20, `Mat4` 22, `TransformPatch` 24, `Joint` 27, `Node` 29, `Repeat` 31, `BoxSolid` 35, `EllipsoidSolid` 37, `CapsuleSolid` 39, `RevolveSolid` 41, `ExtrudeSolid` 43, `LoftSolid` 47, `Solid` 49, `PrimitiveSolid` 51, `Primitive` 53, `Mesh` 55, `Skin` 57, `Material` 59, `Variant` 61, `Pose` 63, `Channel` 65, `Animation` 72, `ModelDocument` 74, `Quality` 90, `Issue` 92, `Vec` names). No `$defs` name is missing from the twin.
- Twin exports with no `$defs` name (mismatch, reverse direction):
  - `LoftSection` (`M/🧬️schema/🟦️.ts:44`). The schema defines the section inline in `LoftSolid.sections` (items: `at`, `rx`, `ry`, `exponent`, `rotation`).
  - `Operation` (`M/🧬️schema/🟦️.ts:95`), a runtime callback interface. Justifiable, but not in the schema.
- Design block `semio.model/v1` (`design.md`): every named field exists in the schema. ModelDocument: `$schema, version, id, title, provenance, nodes, solids, meshes, skins, materials, variants, poses, animations` (yes). Node: `id, parent, translation, rotation, scale, joint, mesh, skin, tags` (yes). Joint: `type, axis, min, max, twistMin, twistMax` (yes). Primitive: `mode, material, positions, normals, colors, joints, weights, indices` (yes) plus `solids` added by Amendment A5 (in schema and twin, not in the design block text). Solid: `id, node, material, tags, transform, repeat, weights` (yes, through `SolidBase`) plus `kind` (in schema, not in the design block, documented in `kernel-core.md`). Box, Ellipsoid, Capsule (`lateralScale`), Revolve (`sweepRad`, `lateralScale`), Extrude (`holes`, `taperTo`), Loft (`sections`, `capStart`, `capEnd`): all present. Repeat, Mesh, Skin, Material (`alphaMode`, `doubleSided`, `unlit`), Variant, Pose, Channel, Animation (`events`): all present.
- Design vs twin types: `Mat4` is a 16-tuple in the twin; design says `number[16][]`. Compatible.

Schema-first observation (inferred): `M/🧬️schema/🔣️.json` is produced by `T/generate-model-schema.ts` (ticket folder, `writeFileSync`), with the schema assembled in TypeScript object literals. The JSON is the stated source of truth (twin header line 2), but the authoring form is code.

## Rule 10: tests per feature

Language-agnostic fixtures are under `M/🧫️fixtures/<case>/🔣️.json` (kernel) and `M/🖥️viewer/🧫️fixtures/`, `M/📤️export/🧊️glb/🧫️fixtures/`, `M/📤️export/🎬️fbx/🧫️fixtures/`.

| Feature | Fixture | Third-party oracle | Status |
|---|---|---|---|
| Box, rounded box (`📦️box`) | `M/🧫️fixtures/📦️box/🔣️.json` | three `BoxGeometry`, `RoundedBoxGeometry` | ok |
| Ellipsoid (`🥚️ellipsoid`) | `🧫️fixtures/🥚️ellipsoid` | three `SphereGeometry` (scaled) | ok |
| Capsule (`💊️capsule`) | `🧫️fixtures/💊️capsule` | three `CapsuleGeometry` | ok |
| Revolve (`🌀️revolve`) | `🧫️fixtures/🌀️revolve` | three `LatheGeometry` | ok |
| Extrude (`🏗️extrude`) | `🧫️fixtures/🏗️extrude` | three `ExtrudeGeometry`, `ShapeUtils` | ok |
| Loft (`🧵️loft`) | `🧫️fixtures/🧵️loft` | three `CylinderGeometry`, `Vector3`, `Quaternion` | ok |
| Repeat (`🔁️repeat`) | `🧫️fixtures/🔁️repeat` | three via `🧰️support` (`weldedGeometry`, indirect) | ok, indirect |
| Skinning (`🦴️skinning`) | `🧫️fixtures/🦴️skinning` | three `Skeleton`, `SkinnedMesh` | ok |
| Pose and animation (`🎞️pose-animation`) | `🧫️fixtures/🎞️pose-animation` | three keyframe tracks, `Quaternion` | ok |
| Joint limits (`🦴️mechanism`) | in `pose-animation` | three `Quaternion` | ok, covered by pose-animation |
| Validation (`✅️validation`) | `🧫️fixtures/✅️validation` | ajv (schema only) | ok for schema, not for semantic checks |
| Feature edges (`📏️edges`) | `🧫️fixtures/📏️feature-edges` | three `EdgesGeometry` | ok |
| Progress and cancellation (`⏱️operation`) | none (uses `📦️box` fixture) | none | missing fixture and oracle |
| Provenance (`🧬️provenance`) | `🧫️fixtures/🧬️provenance` | three `BoxGeometry`, `EdgesGeometry`; ajv | ok |
| Polygon triangulation (`🔺️polygon`) | `🧫️fixtures/🔺️polygon` | three `ShapeUtils` | ok |
| Math (`🧮️math`) | none | none | no test folder; indirect only |
| Builder, segments (`🧱️builder`, `📏️segments`) | none | none | indirect only |
| Viewer frame, picking, camera, raster | `🧫️fixtures/🎞️frame`, `🎯️picking`, `📷️camera`, `🖼️raster`; rig | three | ok |
| Viewer scene (`🧱️scene`) | `🧫️fixtures/🧱️rig`, `🎞️frame` | none directly (three via `🧰️support`) | indirect |
| Viewer WebGL2 (`🔶️webgl2`) | rig | none (compares with own software rasteriser) | no third-party oracle |
| Viewer controller (`🕹️controller`) | rig | three imported, jsdom | fixture ok, oracle use unclear |
| Viewer element (`🧩️element`) | none | none | no test file |
| Viewer GPU (`🔶️gpu`) | rig | playwright, esbuild (Chromium, skipped if absent) | conditional |
| GLB vectors | `🧊️minimal-triangle`, `🧊️rig-full`, `🧊️rig-bent-pose`, `🧊️rig-wave-quarter` (`M/📤️export/🧊️glb/🧫️fixtures/`) | gltf-validator, @gltf-transform/core and extensions, three GLTFLoader (`🧪️oracles`) | ok; byte expectations golden |
| FBX structure | `static-cube`, `rigged-arm`, `rotation-continuity`, `provenance` | owned second reader; three in `second-reader` | ok |
| FBX deflate | `🗜️deflate/🔣️.json` | fflate, node:zlib | ok |
| FBX rejections | `procedural` | none (contract test) | ok |
| FBX three roundtrip | `rigged-arm` | three `FBXLoader` | ok |
| FBX assimp | all FBX fixtures | assimpjs (dynamic import) | ok |
| FBX material, animation (partly), mechanism, matrix, records | via fixtures above | via three roundtrip and assimp | no dedicated test folder |

Features without any fixture or any third-party oracle: `⏱️operation` (both), `🧮️math` (both), `🧩️element` (both), `🔶️webgl2` (oracle). Golden-only expectations: GLB vector values (`glb-encoder.md`).

## Rule 11: duplicated code

- Math and matrix (kernel vs FBX; same algorithms, different names):
  - `M/🧮️math/🟦️.ts:58` `compose` vs `M/📤️export/🎬️fbx/🧮️matrix/🟦️.ts:54` `compose` (same TRS matrix; FBX normalises the quaternion first).
  - `M/🧮️math/🟦️.ts:68` `multiply4` vs `M/📤️export/🎬️fbx/🧮️matrix/🟦️.ts:11` `multiply` (same loop).
  - `M/🧮️math/🟦️.ts:77` `invert4` (adjugate) vs `M/📤️export/🎬️fbx/🧮️matrix/🟦️.ts:23` `invert` (Gauss-Jordan): two algorithms for one job.
  - `M/🧮️math/🟦️.ts:23` `qMul` vs `🧮️matrix:67` `quaternionMultiply` (identical formula).
  - `M/🧮️math/🟦️.ts:29` `qConj` vs `🧮️matrix:72` `conjugate` (identical).
  - `M/🧮️math/🟦️.ts:44` `qSlerp` vs `🧮️matrix:82` `slerp` (same algorithm, same 0.9995 threshold).
  - `normalize` (`🧮️math:16` vector; `🧮️matrix:61` quaternion) and `dot` (`🧮️math:13` vec3; `🧮️matrix:77` vec4): same names, different arity and semantics.
  - `DEGREES = 180 / Math.PI`: `M/📤️export/🎬️fbx/🦴️mechanism/🟦️.ts:12` and `🧮️matrix/🟦️.ts:8`.
- World matrices: `M/🔺️tessellation/🟦️.ts:37` `restWorldMatrices`, `M/🦴️mechanism/🟦️.ts:145` (inside `poseModel`), and `M/📤️export/🎬️fbx/🧮️matrix/🟦️.ts:120` `worldMatrices`. Three parent-chain compositions with different guards.
- Constants: `CREASE_COS = Math.cos((35 * Math.PI) / 180)` at `M/🔺️tessellation/🏗️extrude/🟦️.ts:9` and `M/🔺️tessellation/🌀️revolved/🟦️.ts:9`.
- `clamp`: `M/🧮️math/🟦️.ts:112` and `M/🖥️viewer/🕹️controller/🟦️.ts:72` (identical).
- Error helpers: `fail` at `M/📤️export/🎬️fbx/🟦️.ts:81` and `M/🖥️viewer/🧱️scene/🟦️.ts:19` (both throw wrappers; different error classes).
- Little-endian byte writers: `ByteWriter` (`M/📤️export/🎬️fbx/🧱️records/🟦️.ts:71`, growing `DataView`, u8/u32/i32/f32/f64) and `BufferBuilder` (`M/📤️export/🧊️glb/🧱️buffer/🟦️.ts:26`, per-accessor `DataView`) plus GLB header writes in `packGlb` (`🧱️buffer:93-104`). Same job, separate implementations.
- Checksums (Adler-32, CRC, SHA): Adler-32 only, `M/📤️export/🎬️fbx/🗜️deflate/🟦️.ts:42` (no duplicate). SHA-256 only in tests.
- Test support duplicates:
  - `fixture` defined three times: `M/🧪️tests/🧰️support/🟦️.ts:7`, `M/📤️export/🎬️fbx/🧪️tests/🧰️support/🟦️.ts:25`, `M/🖥️viewer/🧪️tests/🧰️support/🟦️.ts:10`.
  - `validateSchema`: `M/🧪️tests/🧬️provenance/🟦️.ts:11` and `M/🧪️tests/🧰️support/🧊️solids/🟦️.ts:13`.
  - GLB `vectors`, `directory`, `quiet` declared in both `M/📤️export/🧊️glb/🧪️tests/🧪️oracles/🟦️.ts:20-24` and `🧊️vectors/🟦️.ts:16-20`.
  - `localMatrix` in FBX `🔍️second-reader/🟦️.ts:30` and `🧍️three-roundtrip/🟦️.ts:19`.
  - Bounds: `M/🧪️tests/🧰️support/🟦️.ts:68` `bounds` and `M/🖥️viewer/🎞️frame/🟦️.ts:162` `boundsOf` (different domains).

## Out of scope observations

- Ticket folder `🗑️generated/oracle-tools/venv-*`: 293 files over 256 UTF-16 units (max 299). Path rule violation.
- The ticket `🗑️generated` folder still holds the frozen kernel copies and scratch files. Delete at close, per rules.
- The wiring config comment (rule 1) and `includeSource: ["../../🟦️.ts"]` (resolves to `3D/🟦️.ts`, which exists) were not changed.

## Method and limits

- All greps and counts were read-only (`grep`, `find`, `perl`, `sed`, `wc`, `git check-ignore`). No test or build was run.
- Rule 6 cancellation granularity was read from the call sites, not measured at runtime.
- "Golden" status of GLB vectors and the "legacy" reading of frozen copies are inferred from documents and file names; the contents were not re-verified against a second implementation.
- Platform API classification is a judgement; DOM types in signatures are flagged under the strict reading of rule 3.
