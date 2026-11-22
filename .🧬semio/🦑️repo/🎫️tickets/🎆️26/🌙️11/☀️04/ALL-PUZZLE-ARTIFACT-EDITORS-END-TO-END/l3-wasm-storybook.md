# L3: puzzle WASM package and Storybook 2d/3d/5d (2026-11-22)

## Result
Storybook puzzle/2d, puzzle/3d and puzzle/5d all launch on port 6010 natively. Playwright (under bun) ran against them:

| Suite | Scope | Result |
| --- | --- | --- |
| `🧪️tests/◻️storybook-2d` (12 tests) | puzzle/2d | 11 passed, 1 failed (lasso select) |
| `🧪️tests/🧊️storybook-spatial` `-g puzzle2d` | puzzle/2d | 2 passed |
| `🧪️tests/🧊️storybook-spatial` `-g puzzle3d` | puzzle/3d | 2 passed |
| `🧪️tests/🧊️storybook-spatial` `-g puzzle5d` | puzzle/5d | 4 passed |
| `🧪️tests/📚️storybook-index` (vitest, bun) | none | 6 passed |

The 2d result is 10 passed in the full run plus the brush-commit test re-run green after its fix (1 test), so 11 of 12 are green; only the lasso test is red.
The `-g` filters exist because `storybook-spatial` also holds infinite-scope tests (WasmCanvas, WorldR3f, ReferenceMedia) that belong to other Storybook scopes.

## WASM package
Not rebuilt. `repo run` was not attempted again: the precedent ticket (☀️18) showed the graph service queue never drains, and a native rebuild is about 100 minutes and 1056+ crates while `/home` has only 2.3 GB free.
The existing bindgen output is used: `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js` (+ `_bg.wasm` 290.7 MB, `.d.ts`), built 2026-11-19, and the Storybook stage copy at `.cache/b/p/m/browser/prototype/wasm-packages/d30e56a055ea2d5bde08382c/` (identical sizes).
Package/target: `@semio-tech/puzzle-wasm` is produced by project `@semio-tech/puzzle-plugin` target `wasm` (`🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`); the native equivalent is documented in `☀️18/.../wasm-native-fallback.md`.
The pkg is stale: more than 1100 Rust files under `🧰️framework` and `🌎️hub` and many under `✏️s/🔌️plugins/🧩️puzzle` are newer than it.

## Commands (all in this ticket folder)
- `l3-serve.sh <2d|3d|5d>`: foreground Storybook dispatcher on 6010 (`STORYBOOK_SCOPE=puzzle/<scope>`, `SEMIO_BUILD_MODE=prototype`, `SEMIO_VITE_CACHE_DIRECTORY=$root/.cache/b/p/m/a/l3-vite`; the latter is mandatory, otherwise `.storybook/main.ts` throws "Vite config requires its original bounded compiler cache").
- `l3-pw.sh <test-dir> <grep> <name>`: `bun node_modules/playwright/cli.js test -c l3-pw.config.ts -g <grep>` against a running server.
- `l3-storybook.sh <scope> <test-dir> <grep>`: server plus tests in one go (needs `SEMIO_VITE_CACHE_DIRECTORY`, now set).
- `l3-pw.config.ts`: Playwright config with WebGPU launch flags and a 60 s expect timeout.
- Index suite: `bun node_modules/vitest/vitest.mjs run -c ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️config/🟦️.ts storybook-index`.
- `l3-probe.mjs`, `l3-brush-probe.mjs`, `l3-lasso-probe.mjs`: Playwright probes used to read console/attributes/the refused snapshot (`globalThis.__semioBoard2dRefusedSnapshot`).

## Root causes and fixes
1. No WebGPU in headless Chromium ("no WebGPU adapter" thrown by Board2dHost). The precedent config only passed GL flags. Fixed in `l3-pw.config.ts`: `--enable-unsafe-webgpu --enable-features=Vulkan,WebGPU --use-angle=swiftshader --use-vulkan=swiftshader --use-webgpu-adapter=swiftshader --enable-unsafe-swiftshader --ignore-gpu-blocklist`. No repo-committed Playwright config exists for these suites; this one should be adopted as the canonical one (not yet moved out of the ticket).
2. Board canvas lookup was ambiguous: the 2d host now contains a second, `aria-hidden` tool-run trace canvas, so `host.locator("canvas")` violated strict mode. Fixed in `🧪️tests/◻️storybook-2d/🟦️.ts` and `🧪️tests/🧊️storybook-spatial/🟦️.ts` by selecting `canvas:not([aria-hidden="true"])`.
3. Stories collapsed to zero height: the 2d-board, 2d-fixtures and 3d-world stories used `height: "100%"` under `layout: "fullscreen"` (no parent height), so the canvas was hidden; 5d already used `100vh`. Fixed to `100vh` in `✏️s/🧑‍💻dev/🧩️puzzle/📖️stories/🎭️2d-board/🧪️.story.tsx`, `🎭️2d-fixtures/🧪️.story.tsx`, `🎭️3d-world/🧪️.story.tsx`.
4. Brush placement in the 2d-board story emulator produced handles without `id`/`radius`; the engine's `BoardHandleSnapshot` (`deny_unknown_fields`, `id` required) refused the snapshot. Fixed in `🎭️2d-board/🧪️.story.tsx` (`brushPlace`): handles get `id = <nodeId>:v<i>` and default `radius: 6`.
5. 2d-fixtures story fed the parsed puzzle document (`puzzle2dParseDslJson`) straight to the board engine. The document carries fields the engine's closed snapshot schema (`🧰️framework/.../♾️infinite/🎲️board/🧬️schema/📸️board-snapshot-records/🦀️.rs`, `deny_unknown_fields`) refuses (node `anchor`; edge `gap/shift/rise/rotation/turn/tilt/x/y`). Fixed in `🎭️2d-fixtures/🧪️.story.tsx` by projecting to the engine vocabulary (`boardEngineSnapshot`) when building the scene.
6. The default 5 s Playwright `expect` timeout is too short for WASM plus WebGPU on swiftshader on a loaded host; set to 60 s in `l3-pw.config.ts`.

## Remaining failure
`🧪️tests/◻️storybook-2d/🟦️.ts:146` "puzzle2d lasso select: polygon selects alpha while excluding beta": after the drag is released, `selection` stays `[]` (expected `["alpha"]`) for 60 s. Probe: during the drag the engine reports `preselectCount: 5` (selection mode), after release `mode: "none"`, `selectionCount: 0`. Click selection, Ctrl+A, delete, wheel, brush all pass. The engine code path is the resumable `step_selection_commit` in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/🪝️pointer-commit-steps/🦀️.rs`. Likely cause is the stale `pkg/semio_puzzle_bg.wasm` (2026-11-19) against current engine sources; it needs the WASM package rebuilt from current sources (about 100 minutes, needs several GB of free disk) before it can be judged as a real engine bug.

## Environment findings
- `/home` is 100% full (465 GB, 2.3 GB free). Largest consumers are other tickets' generated outputs (`☀️18/DEV-PUZZLE-5D...` 57 GB, `🌙️09/☀️06/REMODEL-PLUGIN-END-TO-END` 137 GB, `☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END` 28 GB). I did not delete any of them (not mine). A short `ENOSPC` hit my own `sed -i` once.
- Host memory was near exhausted (other lanes' rustc runs); a Chromium page crashed once ("Page crashed") and passed on retry.
- Servers I started were stopped; my Vite cache `.cache/b/p/m/a/l3-vite` and result directories were removed. Only the `.log` files and `serve-*` dirs remain under `🗑️generated/l3-run`.
