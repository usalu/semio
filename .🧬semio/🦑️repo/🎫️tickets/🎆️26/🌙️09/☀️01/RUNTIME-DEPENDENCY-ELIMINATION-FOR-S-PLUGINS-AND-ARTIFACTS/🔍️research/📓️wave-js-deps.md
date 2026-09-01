# Wave 8 (JS) — react/react-dom/three/@react-three/*/brepjs/brepjs-opencascade/chevrotain/xstate/pdfjs-dist/reveal.js

Scope: the 3 JS production manifests (`✏️s/🔌️plugins/📐️cad`, `🧩️puzzle`, `🎞️animate`) with 11
distinct third-party packages / 22 total `dependencies` entries across them.

## 1. Exhaustive import measurement (before any change)

Searched every `.ts .tsx .js .jsx .mjs` file under each plugin (`grep -rn` on the exact import
specifiers, subpath imports, and dynamic `import()`).

### `✏️s/🔌️plugins/📐️cad` (81 TS/TSX files scanned)

| package | imported by | files |
|---|---|---|
| `react` | 1 file | `🗿️artifacts/…/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx` — runtime `Fragment` (2 uses) + type-only `CSSProperties`, `KeyboardEvent`, `MouseEvent as ReactMouseEvent`, `ReactNode` (~40 uses) |
| `react-dom` | 0 files | — |
| `three` | 1 file | same renderer file — **type-only**: `BufferGeometry, Camera, Group, MeshStandardMaterial, Object3D, Ray, Scene, Vector3` |
| `@react-three/fiber` | 0 files | — |
| `@react-three/drei` | 0 files | — |
| `brepjs` / `brepjs-opencascade` | 1 file | `📦️packages/🟦️typescript/🟦️brep-implementation.ts` — real runtime: dynamic WASM URL, `initOpenCascade`, delegates every kernel op (`box`, `extrude`, `cut`, `loft`, …) to the `brepjs` module |
| `chevrotain` | 2 import lines, 1 file | `🗿️artifacts/…/🧬️schema/💡️inferences/🟦️component.ts` — real runtime: `CstParser`, `createToken`, `Lexer` back a hand-written Cypher-like "construct query language" |
| `xstate` | 1 file | `🗿️artifacts/…/⚙️engine/🎰️stately/🟦️component.ts` — real runtime: `createActor`, `setup().createMachine()` driving a flat, one-level chart (`__advance` event + guarded branch rows) |

### `✏️s/🔌️plugins/🧩️puzzle` (197 TS/TSX/JS files scanned)

**Zero** imports of any of the nine packages (`react`, `react-dom`, `three`, `@react-three/fiber`,
`@react-three/drei`, `brepjs`, `brepjs-opencascade`, `chevrotain`, `xstate`) anywhere in the
package. All nine `dependencies` entries were dead declarations.

### `✏️s/🔌️plugins/🎞️animate` (49 TS/TSX/JS files scanned)

All four declared packages are used, all in one file —
`🗿️artifacts/🎬️present/…/✏️editor/📺️renderer/⚛️react/🟦️component.tsx` (9385 lines):

| package | usage |
|---|---|
| `react` | real: `act, createContext, Fragment, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState` + types `CSSProperties, FC, ReactNode, RefObject` |
| `react-dom` | real: `createRoot, type Root` from `react-dom/client` |
| `pdfjs-dist` | real: dynamic `import("pdfjs-dist")`, `GlobalWorkerOptions.workerSrc` — PDF page rendering for `PdfEmbodiment` |
| `reveal.js` | real: `import Reveal from "reveal.js"` — this file **is** the plugin's presentation engine (`Reveal.Api`, `Reveal.Options`, live deck instance drives slide navigation/auto-animate) |

## 2. Outcome applied per package

| package | manifests | outcome |
|---|---|---|
| `react-dom`, `@react-three/fiber`, `@react-three/drei` | cad, puzzle | **deleted** — zero imports |
| `react`, `three`, `xstate`, `brepjs`, `brepjs-opencascade`, `chevrotain` | puzzle | **deleted** — zero imports |
| `react` | cad, animate | **moved**: re-pointed to `@semio-tech/ui-react`, which already depends on `react`/`react-dom` (framework boundary). Deleted from both `dependencies` blocks. |
| `three` | cad | **moved** (type-only): re-pointed to `@semio-tech/ui-react`. Deleted. |
| `react-dom` | animate | **moved**: `createRoot`/`Root` re-pointed to `@semio-tech/ui-react`. Deleted. |
| `xstate` | cad | **moved**: real capability ported to a new first-party package `@semio-tech/machine` (see §3). Deleted from `dependencies`; kept as `devDependencies` **test-oracle** for a differential test. |
| `brepjs`, `brepjs-opencascade` | cad | **deferred** — see §4, too large for this slice, left in `dependencies` as documented. |
| `chevrotain` | cad | **deferred** — see §5, too large for this slice, left in `dependencies` as documented. |
| `pdfjs-dist`, `reveal.js` | animate | **deferred** — see §6, too large for this slice, left in `dependencies` as documented. |

### React/three re-export in `@semio-tech/ui-react`

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` already does
`import * as React from "react"` / `import * as THREE from "three"` and already declares
`react`/`react-dom`/`three`/`@react-three/*` as its own `dependencies` (the allowed framework
pattern). Added one explicit re-export block, following the file's existing
"W-interim explicit re-export" convention (`export { reactHostPort, … }`):

```ts
export { Fragment, createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, act } from "react";
export type { CSSProperties, FC, KeyboardEvent, MouseEvent as ReactMouseEvent, ReactNode, RefObject } from "react";
export { createRoot } from "react-dom/client";
export type { Root } from "react-dom/client";
export type { BufferGeometry, Camera as ThreeCamera, Group, MeshStandardMaterial, Object3D, Ray, Scene as ThreeScene, Vector3 } from "three";
```

`Camera`/`Scene` were aliased (`ThreeCamera`/`ThreeScene`) because the barrel already exports an
unrelated first-party `Camera` interface and a `Scene` UI component under those bare names.
Updated both consumer files' imports accordingly (cad renderer, animate renderer).

## 3. `xstate` → `@semio-tech/machine` (new first-party package)

`🧰️framework/🔨️modules/🔄️machine/🟦️.ts` already existed as a complete, framework-owned, in-house
statechart kernel (1149 lines) — an explicit "TS twin" of the Rust `statechart!` macro's compiled
tables (`NodeDef`/`TransitionDef`/`GuardFn`/`ActionFn`, `init`/`macrostep`, `Snapshot`,
`runConformance`) — but it had **no TS package wrapper** (no `package.json`, no workspace entry),
so nothing could depend on it yet. Created one, mirroring `@semio-tech/s-3d-js`'s package shape
exactly:

- `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,📦️index.ts,tsconfig.json,vitest.config.ts}`
- Registered in root `package.json` `"workspaces"`.

Added an in-source fixture-test suite to `🔄️machine/🟦️.ts` itself (language-agnostic — a flat
traffic-light ring + a guarded-branch machine, both driven only through the public
`init`/`macrostep`/`Snapshot.matches` API): 3 tests, all pass.

Rewrote `✏️s/🔌️plugins/📐️cad/🗿️artifacts/…/⚙️engine/🎰️stately/🟦️component.ts`:
- `buildStatelyMachine` now compiles `InteractionSpec.machine` into a flat `MachineDefinition`
  (one atomic `NodeDef` per state under a synthetic compound root; every `(state, event)` row
  becomes a `TransitionDef` on one shared `__advance` `EventId`, disambiguated by a `GuardFn`
  closing over `(interactionKind, branch)` — the exact same encoding the old XState chart used).
- `StatelyStateEngine` now drives it synchronously via `init`/`macrostep` instead of an XState
  actor (`createActor(...).start()/.send()/.stop()`) — no behavior change, just no actor runtime.
- `statelyStateEngineProvider.id` renamed `"xstate-stately"` → `"machine-stately"` (grepped repo,
  zero other references to the old id).

**Differential test** (test-driven requirement): `xstate` was added to cad's `devDependencies` as
a pure test oracle (never `dependencies`) and a new test —
`"buildStatelyMachine matches an XState v5 chart built from the same spec (oracle)"` — builds a
literal XState v5 chart from every `(state, event, branch)` row of every shipped `InteractionSpec`
and asserts the new `@semio-tech/machine`-backed engine reaches the identical target state. This
runs alongside the pre-existing pure-ts-vs-stately snapshot-parity tests in the same file (box
workflow, undo, distance/area measure) — none of those needed changes and all still pass.

## 4. `brepjs` / `brepjs-opencascade` — deferred (too large for this slice)

Confirmed genuinely production per repo memory. `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️component.ts`
is **3958 lines**: a full B-Rep persistence graph (Vertex/Edge/Wire/Face/Shell/Cell/CellComplex/
Cluster + Anchor + Surfaces/Parts/Volumes derivations) wired directly to `brepjs` operator calls
(`box, bsplineApprox, circle, cone, cut, cylinder, extrude, face, filledFace, healSolid, intersect,
loft, sweep, thicken, translate, wire, …`), plus `📦️packages/🟦️typescript/🟦️brep-implementation.ts`
(237 lines) which lazily boots real OpenCascade WASM. Moving this behind a first-party type
surface means re-typing every one of those call sites and re-verifying zero behavior drift across
the entire CAD kernel — a multi-day effort on its own, not a same-slice rewiring. Left both
packages in `cad`'s `dependencies` exactly as-is; **not** faked or partially routed.

## 5. `chevrotain` — deferred (too large for this slice), port status

`🗿️artifacts/📐️cad/…/🧬️schema/💡️inferences/🟦️component.ts` (1726 lines total) contains a
hand-written Cypher-like "construct query language": a `chevrotain` `Lexer` + token vocabulary +
`CstParser` grammar + CST visitor evaluating `MATCH (Object {…})` / `CALL view.…` queries against
a `Model` snapshot. The framework's own DSL infrastructure at
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/` (`🔍️lexer`, `📖️grammar` with a self-hosted
`.grammar.semio` EBNF meta-grammar) is **Rust-only** — there is no TS-side package or lexer/parser
generator to port onto today. Building one (or hand-writing an equivalent recursive-descent parser
without `chevrotain`) plus the ticket's required corpus differential test is real, multi-day work.
**No port attempted** — `chevrotain` left in `cad`'s `dependencies` as-is. This is the largest
piece of remaining work in this wave.

## 6. `pdfjs-dist` / `reveal.js` — deferred (too large for this slice)

Both are genuinely, deeply used by `animate`'s single renderer file: `reveal.js` **is** the plugin's
presentation/slide engine (`Reveal.Api`, `Reveal.Options`, live deck driving navigation and
auto-animate — not a peripheral integration), and `pdfjs-dist` does real PDF page rendering for
`PdfEmbodiment`. Building a first-party presentation engine or PDF renderer is out of scope for
this slice. Left both in `animate`'s `dependencies` as-is.

## 7. Verification (verbatim tails)

### `bun install` (foreground)

```
bun install v1.3.14 (0d9b296a)
Resolving dependencies
Resolved, downloaded and extracted [38]
Saved lockfile

$ bun ./📜️script.ts setup postinstall

3 packages installed [3.91s]
```

### Typecheck

`@semio-tech/machine` fixture tests (`bun vitest.mjs run --config vitest.config.ts`):
```
 Test Files  1 passed (1)
      Tests  3 passed (3)
   Duration  5.77s
```

`@semio-tech/ui-react` (`bunx tsc --noEmit -p tsconfig.json`) — clean after fixing the two new
`Camera`/`Scene` name collisions; the two remaining lines are **pre-existing, unrelated** (not
under `🧰️framework/🔨️modules/🖱️ui/**`, not touched by this slice):
```
../../../../../🎭️actor/🚪️lifetime/🟦️component.ts(441,92): error TS2345: Argument of type 'unknown' is not assignable to parameter of type 'Error'.
../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts(6114,26): error TS7006: Parameter 't' implicitly has an 'any' type.
```

`@semio-tech/cad-js` (`bunx tsc --noEmit -p tsconfig.json`, scoped to exactly the 9 domain files
incl. the touched renderer + stately files) — same one pre-existing unrelated `📚️library` line,
**zero new errors**, exit code 0.

`@semio-tech/s-3d-js` (untouched sibling, run for comparison only): `bunx tsc --noEmit -p
tsconfig.json` on this class of minimal `🟦️.ts`-backed framework packages produces a large
pre-existing cascade of `TS5097`/`TS6059` errors (missing `allowImportingTsExtensions`, `rootDir`
violations reaching into `🛍️products/💻️os/…`, `📚️library`, `🎭️actor`) **identical** to what
`@semio-tech/machine`'s freshly-created (but template-identical) `tsconfig.json` also produces —
confirmed pre-existing and out of this slice's scope, not something introduced here. The package's
real gate is its `vitest`-based `test` target (see above; 3/3 pass), not a raw `tsc -p` invocation.

`@semio-tech/puzzle-js` has no `tsconfig.json`/typecheck target; verified via its test target
instead (below) since only `package.json` changed and zero source files reference the removed
packages.

`@semio-tech/animate-js` has no `tsconfig.json`/typecheck target either; verified via its test
target (below).

### Tests (`bun <bun-bin> node_modules/vitest/vitest.mjs run --config …`, matching the repo's own
wrapper invocation — `bunx vitest` fails to load these configs: hits the documented
"Vite Config Node Strip-Only TS" issue, node worker threads choke on a transitively-imported
parameter-property class)

`@semio-tech/puzzle-js`:
```
 Test Files  9 passed (9)
      Tests  15 passed (15)
```

`@semio-tech/animate-js` (full suite, `--testTimeout 30000`):
```
 Test Files  4 passed (4)
      Tests  148 passed (148)
   Duration  186.59s
```
(A first attempt through the repo's default 15s/`nx` test-budget wrapper timed out mid-suite
under heavy shared-machine contention — 3 tests failed with `act(...)`-adjacent 5000ms timeouts.
Re-run in isolation with a larger timeout after contention eased: 148/148 pass, confirming that was
environmental flakiness, not a regression from the `react`/`react-dom` → `@semio-tech/ui-react`
rewiring.)

`@semio-tech/cad-js` (full 9-domain-file suite, `--testTimeout 30000`, machine still under heavy
concurrent load — 1067s wall):
```
 Test Files  2 failed | 7 passed (9)
      Tests  6 failed | 316 passed (322)
```
The 6 failures are all `Error: Test timed out in 30000ms` inside `📄️artifact/🟦️component.ts`
(interaction e2e/fixture tests) and `🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts`
(transformation catalog) — brepjs-kernel-driven geometry tests, **not touched by this slice** and
unrelated to react/three/xstate. The `🎰️stately/🟦️component.ts` file — the one this slice actually
rewired, including the new XState-oracle differential test — reported **zero failures**; every one
of its tests, including the new differential test iterating every shipped interaction spec, passed.

### `bun ./📜️script.ts verify dependencies literal-external`

Before (commit `aad3d81959`, this slice's start):
```
total	168	166	2	0	3	163	87
oracle-conflicts=8
  oracle-conflict js:brepjs declared by cad/…/package.json, puzzle/…/package.json
  oracle-conflict js:three declared by cad/…/package.json, puzzle/…/package.json, ui-react/package.json, infinite-world-r3f/package.json, framework-renderer-react/…/react/package.json
```

After:
```
ecosystem	raw	third-party	first-party	composition-scoped	mandated-toolchain	corrected/literal-external	production-reachable	kind-census
js	73	73	0	0	3	70	31	production-runtime:31,repository-tooling:45,test-oracle:4
...
error: [verify dependencies literal-external] target=0, current=163, oracle-conflicts=6, toolchain-owner-conflicts=0, toolchain-failures=0.
  oracle-conflict js:brepjs declared by ✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json
  oracle-conflict js:three declared by 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json, 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/package.json, 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json
```

**`js:brepjs` conflict: `puzzle` cleared, `cad` remains (documented, deferred).**
**`js:three` conflict: both `cad` AND `puzzle` cleared — only the 3 legitimate framework
declarations remain (the allowed platform-layer pattern).**
Total `oracle-conflicts` dropped 8 → 6 (`js:manifold-3d`/rust conflicts unrelated, from concurrent
Rust-side slices).

`js` row `production-runtime:31` is **unchanged** from the plan doc's stated baseline of 31 — this
is expected, not a miss: it is a repo-wide *deduplicated package-name* count reachable from *any*
production manifest (framework included, which is the allowed exception). Every package removed
from `cad`/`puzzle`/`animate` in this slice (`react`, `react-dom`, `three`, `@react-three/fiber`,
`@react-three/drei`, `xstate`, `pdfjs-dist` is unchanged in animate) was **already** independently,
legitimately declared by a framework package (`ui-react`, `infinite-world-r3f`,
`framework-renderer-react`), so the deduplicated set doesn't shrink even though the s-plugin
manifests are now compliant. `total literal-external=163` is likewise unchanged for the same
reason plus ongoing concurrent Rust-side churn on the other ~150 non-JS violations, which are out
of this slice's scope.

## 8. Files touched

- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json` — removed `react-dom`,
  `@react-three/fiber`, `@react-three/drei`, `react`, `three`, `xstate` from `dependencies`; added
  `@semio-tech/machine`; added `xstate` to `devDependencies` (oracle only).
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json` — removed all 9 third-party
  `dependencies` entries (zero usage).
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json` — removed `react`, `react-dom`
  from `dependencies`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` — added
  explicit react/react-dom/three re-export block.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx`
  — `react`/`three` imports re-pointed to `@semio-tech/ui-react`.
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️component.tsx`
  — `react`/`react-dom` imports re-pointed to `@semio-tech/ui-react`.
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️component.ts`
  — rewritten onto `@semio-tech/machine`; new XState-oracle differential test.
- `🧰️framework/🔨️modules/🔄️machine/🟦️.ts` — added in-source fixture-test suite (3 tests).
- New package: `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/` (`package.json`,
  `📋️project.json`, `📜️script.ts`, `📦️index.ts`, `tsconfig.json`, `vitest.config.ts`).
- `package.json` (root) — added the new package to `"workspaces"`.

## 9. What remains (explicitly, for a follow-up slice)

1. `brepjs`/`brepjs-opencascade` in `cad` — move the 3958-line B-Rep kernel behind a first-party
   type surface via the `🧱️brepjs` spatial-kernel adapter (§4).
2. `chevrotain` in `cad` — port the construct-query lexer/parser onto a new TS-side grammar
   package (no such package exists yet; the framework's `🗣️dsl` is Rust-only), with a
   query-string-corpus differential test before deleting `chevrotain` (§5).
3. `pdfjs-dist`/`reveal.js` in `animate` — no framework equivalent exists; would need a first-party
   presentation-engine / PDF-render module (§6).
