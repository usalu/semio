# 📓️ terra — ts-fleet vitest measurement + include/includeSource double-count fix

Packet: `ts-fleet` (the plugin-side TypeScript packages: `🌊️flow`, `🎞️animate`, `📐️cad` + its 4
extensions, `🔱️trinity/🔌️jack/🧠️lsp`, `🧩️puzzle`). Owned paths: `✏️s/🔌️plugins/*/📦️packages/🟦️typescript/**`,
`✏️s/🔌️plugins/*/🧩️extensions/*/📦️packages/🟦️typescript/**`, `✏️s/🔌️plugins/🔱️trinity/🔨️modules/**`.

## 1. `include`+`includeSource` double-count bug — ALREADY FIXED before this packet started

The brief named 4 cad extensions and `animate` as still carrying the bug (rule 18 in
`📌️important.md`). Re-reading all 9 `🧪️vitest.config.ts` files under my owned paths from disk showed
every one of them already has `include: []` alongside `includeSource`:

- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️vitest.config.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️vitest.config.ts`

`git log -p -1` on each shows the `include: [...] → include: []` edit already landed in commit
`e3881b1548` (HEAD at packet start, auto-committed, dated 2026-08-19 09:57:12 +0200 by `--date=iso`).
`git diff HEAD` and `git status --porcelain` on these files are empty — no uncommitted delta, the fix
is fully persisted. I made **no changes** to these config files; nothing left to do here.

The other 3 packages (`🌊️flow`, `🔱️trinity/🔌️jack/🧠️lsp`, `🧩️puzzle`) use `include` only (no
`includeSource` key at all), so the double-count precondition never applied to them.

### Current (post-fix) unique counts — measured fresh this packet, `--reporter=verbose`

| package | test files | tests | exit | notes |
|---|---|---|---|---|
| `@semio-tech/flow-js` | 1 passed | 0 (no test blocks) | 0 | smoke-import only |
| `@semio-tech/trinity-jack-lsp-worker` | 2 passed | 0 (no test blocks) | 0 | smoke-import only |
| `@semio-tech/puzzle-js` | 9 passed | **15 passed / 15 unique**, confirmed by name via `--reporter=verbose` | 0 | see `terra-ts-fleet-puzzle.txt` |
| `@semio-tech/animate-js` | 1 **failed** | 0 (collection error) | 1 | pre-existing, unrelated — §2 |
| `@semio-tech/cad-js` | 1 **failed** | 0 (collection error) | 1 | pre-existing, unrelated — §3 |
| `@semio-tech/cad-js-module-aec-building-structure` | 1 **failed** | 0 (collection error) | 1 | §3 |
| `@semio-tech/cad-js-module-aec-building` | 1 **failed** | 0 (collection error) | 1 | §3 |
| `@semio-tech/cad-js-module-spatial-shape` | 1 **failed** | 0 (collection error) | 1 | §3 |
| `@semio-tech/cad-js-module-aec-building-energy` | 1 **failed** | 0 (collection error) | 1 | §3 |

No doubling anywhere — every number above is the unique count from a single `nx test <project>` run.
"Before" (pre-fix, doubled) counts cannot be re-measured honestly since the fix predates this packet;
the git diff above is the evidence the bug existed and was corrected.

## 2. `@semio-tech/animate-js` — pre-existing collection failure, NOT the double-count bug

`bunx nx test @semio-tech/animate-js -- --reporter=verbose` → **exit 1**, `Test Files 1 failed (1)`,
`Tests no tests`. Full output: `terra-ts-fleet-animate.txt`.

```
Error: Failed to resolve import "@semio-tech/animate-present-core" from
"…/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️component.tsx"
```

The vitest config aliases `@semio-tech/animate-present-core` and
`@semio-tech/mit-bestand-praesentation-projektetage-spec` to
`./✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/⚡️implementations/🟦️typescript/📦️index.ts` and
`./♻️mit-bestand/🎤️präsentation/📅️33.projektetage/⚡️implementations/🟦️typescript/📦️index.ts` — **neither
path exists** (verified with `pathlib.Path(...).exists()` → `False` for both; no `🎛️apps` directory
exists under `🎞️animate` at all). Functions the component imports from that alias
(`splitFigureGrid`, `unionDispositionPositions`, …) are consumed only, never defined, anywhere in the
repo — the `animate-present-core` package appears to have been deleted/renamed (old `⚡️implementations`
naming convention, superseded by the current `🗿️artifacts/…/🏅️standards/🔖️1/…` layout) without the
alias being repointed. **Not touched** — out of scope for the double-count task and requires
domain knowledge of what the renderer should now import; flagging for the coordinator/whoever owns
`animate`'s renderer.

## 3. `📐️cad` + its 4 extensions — one real fix landed, one deeper pre-existing blocker found

### 3a. Fixed: missing `runtime`/`brepjs` exports on `@semio-tech/cad-js` (in my owned path)

All 4 extension in-source tests do `const { runtime, brepjs } = await import("@semio-tech/cad-js")`
and destructure `bootstrapCadModules` off `runtime`. `📦️index.ts` never exported either namespace
(confirmed by reading the file — only `core`, `cad_schema`, `cad_snapshot_*`, `cad_diff*`, `cad_dsl`,
`cad_pack`, `cad_op`, `cad_mutations`, `cad_spr`, `cad_io`). Every extension suite failed immediately
with `TypeError: Cannot destructure property 'bootstrapCadModules' from null or undefined value` /
`undefined is not an object (evaluating 'runtime.bootstrapCadModules')` — 100% collection failure,
0 tests, before I touched anything (`terra-ts-fleet-cad-ext-*.txt` capture the state after my fix, not
before; the before-state trace is quoted above verbatim from the first run).

Fixed by adding two lines to `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📦️index.ts`:

```ts
export * as runtime from "../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️component.ts";
export * as brepjs from "../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts";
```

`bootstrapCadModules` lives in the `🏃️runtime` component; `BrepjsKernel`/`preciseSpatialKernelMath`
(what the extensions destructure off `brepjs`) live in the `🧱️brepjs` engine component — both verified
present by grep before wiring the exports.

### 3b. Discovered after 3a: real circular-import blocker (NOT fixed, out of scope)

Re-running after the export fix, all 4 extensions now fail one hop deeper — e.g. structure:

```
TypeError: Cannot destructure property 'bboxSizesFromPositions' from null or undefined value
 ❯ ../📐️spatial-shape/🟦️component.ts:6:208
```

Root cause, traced by reading the import graph: `🏃️runtime/🟦️component.ts` statically imports all 4
extension packages (`@semio-tech/cad-js-module-{spatial-shape,aec-building,aec-building-energy,
aec-building-structure}`) to bootstrap them. Every extension's `component.ts` in turn does
`import { core } from "@semio-tech/cad-js"` at its own top level. So the moment `cad-js/📦️index.ts`
exports `runtime` (3a), a genuine cycle exists: extension → `cad-js/index.ts` → `runtime.ts` → *the
same extension* (and its 3 siblings) → back to `cad-js/index.ts` for `core` — and depending on JS
module-init ordering through Vite/esbuild's ESM interop, the circularly-reentered `core` binding reads
as `undefined`. This is an **architectural layering defect** (the bootstrap/runtime module and the
extension modules import each other), not a config bug, and not something this packet's remit
(vitest include/includeSource) should attempt to restructure — fixing it means either lazy-loading the
extensions inside `bootstrapCadModules()` or breaking `core`'s re-export out of the cycle, which is a
design decision, not a mechanical fix. **Reported, not attempted.**

### 3c. Separate, unrelated pre-existing blocker on `@semio-tech/cad-js` itself

`@semio-tech/cad-js`'s own suite (`DOMAIN_FILES`, `includeSource`) doesn't even reach 3b — it fails
earlier, at module resolution, with an error that has nothing to do with `📐️cad`:

```
SyntaxError: Export named 'join' not found in module
'/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts'.
```

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/…` is **not** in my owned paths and nothing under
`📐️cad` imports `join` directly (grepped, no hits) — this is a transitive dependency on a shared
library package whose exports moved, almost certainly fallout of another session's concurrent work on
that shared module (matches the documented "Concurrent Cargo Workspace Churn" pattern, TS side).
**Not touched** — outside owned paths, and per the ticket's own guidance this class of failure needs
attribution against `git log` on the *actual* file, not guessed at from here.

## Net effect on cad ecosystem

Before this packet: 5/5 cad packages (main + 4 extensions) already failed to collect any test (0
tests, exit 1) — for the shallow "missing export" reason on the 4 extensions, and the unrelated `join`
reason on `cad-js` itself. After this packet: same 5/5 still fail to collect (0 tests, exit 1), but the
4 extensions now fail for the *real* underlying reason (3b) instead of the superficial one (3a is
fixed and correct on its own terms). **No regression** — the export fix is objectively correct and
necessary regardless of 3b; it just wasn't sufficient by itself to turn the suites green.

## Files changed

- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📦️index.ts` — added `runtime` and `brepjs` named
  namespace re-exports (2 lines). This is the only source edit this packet made.

## Logs (ticket folder, `.txt` per rule 5)

`terra-ts-fleet-animate.txt`, `terra-ts-fleet-cad-main.txt`,
`terra-ts-fleet-cad-ext-aec-building-structure.txt`, `terra-ts-fleet-cad-ext-aec-building.txt`,
`terra-ts-fleet-cad-ext-spatial-shape.txt`, `terra-ts-fleet-cad-ext-aec-building-energy.txt`,
`terra-ts-fleet-flow.txt`, `terra-ts-fleet-jack-lsp.txt`, `terra-ts-fleet-puzzle.txt` — all captured
with `--reporter=verbose`, commands and exit codes as pasted above.

## For the coordinator / next packet

- **lease-request-worthy, not filed as one**: §3c's `join` export break in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` blocks
  `@semio-tech/cad-js` outright — someone with write access there needs to check what removed/renamed
  `join`.
- §3b (cad-js ↔ extensions circular import) needs an owner decision on whether extensions should stop
  statically importing `core` from the package root (breaking the cycle) or `runtime.ts` should
  lazy-import the extensions it bootstraps.
- §2 (`animate`'s dangling `animate-present-core`/`mit-bestand …` aliases) needs whoever owns the
  `present` renderer to say where that code now lives, if anywhere.
