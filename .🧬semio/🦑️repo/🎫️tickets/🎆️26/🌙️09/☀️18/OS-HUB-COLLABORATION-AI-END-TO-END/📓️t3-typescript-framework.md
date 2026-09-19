# T3 — framework TypeScript debt, scoped tsconfig + `typecheck` target

Slice T3 (Opus). Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`.
Scope: `🧰️framework/🔨️modules/**` and `🧰️framework/📦️packages/🟦️typescript/**`.
Inputs: `📓️audit-typescript-debt.md` (clusters 2, 3, 6), `📓️t1-codegen-corruption.md` (root tsconfig
scope + `@types/bun`), `📓️t4-typescript-os-renderer.md` (pattern for a scoped tsconfig + nx target).

## 1. Deliverables

| thing | where | state |
|---|---|---|
| framework-scoped tsconfig | `🧰️framework/📦️packages/🟦️typescript/tsconfig.json` | inherited from the dead predecessor, kept, extended (`paths` for `fast-json-stable-stringify`) |
| ambient module decls | `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts` | inherited, extended (`*.wasm?url`) |
| real types for a mis-typed npm package | `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts` | **new** |
| `typecheck` script | `🧰️framework/📦️packages/🟦️typescript/📜️script.ts` (`TypecheckScript`) | inherited, kept |
| nx target | `…/📦️packages/🟦️typescript/📋️project.json` → `@semio-tech/framework:typecheck` | inherited, kept |
| launch row | `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` → `🛠️dev🧰️framework🪁️typecheck`, group `3_dev`, order 390.1 (next to `🛠️dev🖱️ui🪁️typecheck` at 390) | **added** (both files parse as JSONC; 305 configurations) |

Run it with `bun nx run @semio-tech/framework:typecheck`, or
`bun ./📜️script.ts typecheck` from `🧰️framework/📦️packages/🟦️typescript` (≈15 s wall, even under
fleet load). **Honest note:** every measurement below was taken through the `📜️script.ts` form —
which is exactly what the nx target invokes (`cwd` + `bun ./📜️script.ts typecheck`). I did not get a
clean `bun nx run …` of it: nx project-graph construction did not finish in 10 minutes under the
current fleet load (see §4), so the nx wrapper and the launch row are wired but unexercised.

## 2. Measurements

Command: `bun ./📜️script.ts typecheck --pretty false`. All captures in `🗑️generated/t3-run*.txt`.

| capture | whole program | **T3-owned (`🔨️modules/**`)** |
|---|---:|---:|
| `t3-before.txt` (inherited tree) | 545 | **420** |
| `t3-run2.txt` (typed test-dependency bags) | 398 | 273 |
| `t3-run3.txt` (storybook + design-system families) | 316 | 191 |
| `t3-run4.txt` (fixture types, THREE namespace, Select ids) | 260 | 135 |
| `t3-run5.txt` (shadowed class types, machine vocabulary) | 229 | 104 |
| `t3-run6.txt` (dead `test.mode`, tuple policy sources, manifest fixtures) | 207 | 85 |
| `t3-run7.txt` (assert predicates, effect cleanups, `Object.freeze` slot) | 196 | 74 |
| `t3-run8.txt` (`Script` protected state, `*.wasm?url`) | 193 | 71 |
| `t3-run9.txt` (**final**) | **175** | **65** |

**T3-owned: 420 → 65 (−355, −85 %).** Whole program: 545 → 175 (−370).

The program pulls non-T3 files in through the import graph; tsc reports them but they are other
slices' debt. Final split of the 175:

| owner | count |
|---|---:|
| **T3 — `🧰️framework/🔨️modules`** | **65** |
| T2 — `🛍️products/🦑️repo/📚️library` | 48 |
| T4 — `✏️s` plugins | 22 |
| T4 — `🛍️products/💻️os` | 21 |
| root `📜️script.ts` | 19 |

Nothing was excluded from the tsconfig to hide errors; no `any` blanket, `@ts-ignore` or
`@ts-expect-error` was introduced (the repo-wide count of those pragmas under my paths is still 0).

## 3. Fix families

### a. `dependencies: any` / broken `Pick<>` in extracted test suites (−147)
28 `registerTestsN` suites took their dependency bag as `any`. Worse, the dead predecessor had
already rewritten some to a `Pick<typeof import(caller), …>` whose key union included names the
caller does **not** export — TS2344, which degrades *every* key to `unknown` (that is where the
46-error `🧪️playgroundflowwasmdevstubplugin` wall came from).

`🐍️t3-typed-test-deps.py` (new, in this folder) derives the parameter type from the call site:

* a key the caller exports → `Pick<typeof import("<caller>"), "key">`
* a key the caller re-imports → `Pick<typeof import("<rebased specifier>"), "key">`
* an aliased / namespace / default import → an explicit member (`{ readonly k: (typeof import(s))["orig"] }`), so an `import { X as Y }` can never be picked under the wrong name
* a caller-local declaration → the declaration gains `export` and joins the caller's `Pick`
* anything unresolved → the signature is left alone and reported (one case: `kernelGeometry`)

Parts are intersected; nothing is widened. Applied to 28 signatures.

### b. Storybook CSF3 metas missing `args` (−~45)
`StoryObj<typeof meta>` demands `args` whenever the component has required props; ten render-only
story files had none. Fixed at the root by giving each `meta` the component's real required args:
`🎭️providers` (`children`), `🎭️panel-tab-bar` (`variant`/`tabs`/`activePath`/`onActivePathChange`),
`🎭️navbar-example-select`, `🎭️action-dropdown`, `🎭️mode`, `🎭️app`, `🎭️ui`, `🎭️uiintroduction`,
`🎭️unified-gumball` (a real `new Object3D()`). `🎭️selection-marquee`'s props are a discriminated
union, which meta-level args cannot satisfy, so each of its three stories carries its own `args`.

### c. Design-system required slots omitted at call sites (−~40)
`ModeWindowDescriptor.iconId` (13 + 3 + 4 sites), `EngagementOption.icon` (6 sites) and the
mandatory `icon` on `Button` / `ActionGroupItem` / `Toggle` — "Consumers MUST provide an icon for
each Button", `🧱️elements/🔘️Button/🟦️.tsx:17` — were missing in stories and component tests. Real
icon ids/elements were supplied; `<Button variant="ghost"><ChevronDown/>…` became
`<Button variant="ghost" icon={<ChevronDown/>}>…`.

### d. Drifted names against live types (~20)
* `createIconComponent("maximize2")` → `"maximize-2"` (`🎀️Ribbon`, `🪟️Window` stories);
  `("check-circle2")` → `"check-circle-2"` (`🔚️Footer`); `icon: "refresh-ccw"` → `"rotate-ccw"` (`🖱️ContextMenu`).
* `ResizablePanelGroup direction=` → `orientation=` (the component's real prop), 2 sites.
* `Window` story `loading: true` → `status: "loading"` (`UiStatus`).
* `IntroductionStepDefinition.advance` → `interactions: [], ordered: false` (+ the required
  `introduce: null`), 5 sites across the story and the ui test.
* `PanelDock.anchors` literals gained the `left-middle` / `right-middle` anchors the type requires.
* `TableSkeleton columns={3}` → a real `TableColumn[]`; `Navbar` story referenced an undefined
  `defaultItems`.
* **`TutorialSlice.document` → `.artifact`** — the schema renamed the tutorial track
  (`TutorialTracks.artifact`, `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:1148`) and the TS mirror in
  `🎯️targets/⚛️react/🟦️.tsx:6195` still said `document` while already holding
  `TutorialArtifactEvent[]`. Renamed in the mirror, in `tutorialSlice`, in `validateTutorial`'s sort
  table and in the 5 test sites. This is production code, not a test-only rename.
* Dead `test.mode` key removed from **8** vitest configs (`InlineConfig` has no `mode`;
  `🎭️actor`, `🧊️3d`, `🔄️machine`, `📡️replication`, `🎠️kernel`, `⏳️async`, `◻️2d`, `🖱️ui/🎨️styling`).
* `🖱️ui/🎨️styling/🟦️.ts:2` re-exported a `default` that `🌓️theme/🟦️.ts` does not have (no consumer
  imported it) — removed.

### e. Ajv-narrowed fixtures collapsing to `unknown` (−~30)
`await Bun.file(…).json()` is `any`, but the next line runs the binding through an Ajv
`ValidateFunction` whose default parameter is `unknown`, so `assert(validate(fixture))` *narrows the
binding down to `unknown`* and every later read fails (TS18046 / TS2698). `🐍️t3-annotate-bun-fixtures.py`
(new) derives a `readonly` structural type from the fixture document on disk and annotates the
binding, so the assertion keeps doing only its runtime job. Applied to 7 bindings in
`📡️replication/📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract` and `🌱️value/🗂️ordered/…/🧪️source-contract`.

### f. Names shadowed between value and type space (−~15)
A destructured `const` from the dependency bag shadows the *class* or *namespace* of the same name,
so every type annotation using it fails (TS2749 / TS2503). Fixed with type-only aliased imports:
`THREE` → `import type * as Three from "three"` (7 type positions in
`🧪️owned-locale-detector-retirement`), `OwnedActorTurnOutput(s)` → `…Handle`/`…Queue`,
`OwnedUiPayload` → `OwnedUiPayloadOf` (and `Profile` was exported from
`🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts`, where it was module-private).

### g. Local `type X = any;` stubs left by the suite extraction (−~20)
`🔄️machine/🧪️tests/🧪️semio-tech-machine` declared **seven** (`Machine`, `MachineSpec`, `GuardFn`,
`Command`, `NodeDef`, `TransitionDef`, `StatechartEvent`) and `🎨️styling/🧪️tests/🧪️levels-oklabmix`
one (`Rgba8`). Replaced with type-only imports of the real module vocabulary.

### h. Missing exports on the react barrel
`🧪️owned-locale-detector-retirement` imports `DockSkeleton`, `IntroductionStepDefinition`,
`TutorialCameraKeyframe` and `TutorialDefinition` from `🎯️targets/⚛️react/🟦️.tsx`, which imported
them from `@semio-tech/framework` without re-exporting. Added the re-export line beside the existing
`CanvasHoverFocus`/`CanvasPickRequest`/`CanvasPickTarget` one.

### i. Tuple arity lost by `.map()` (−7)
`interactivityMounted*Failures(...sources)` take 5–8 positional `string` parameters; the tests built
`sources` with `paths.map(read)`, which yields `string[]` and cannot be spread (TS2556). Each test
now declares a named tuple type (`LayoutTextPolicySources`, `SurfaceLanePolicySources`,
`PreparedRenderPolicySources`), builds `clean` positionally and types `mutated` with it.

### j. Wrong or incomplete third-party / ambient declarations
* `fast-json-stable-stringify` ships an `index.d.ts` declaring a **single-argument** `stringify(obj)`
  while its runtime (and every byte-order oracle in this repo) passes `{ cmp }`. Declared the real
  surface in `🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts` and routed it in through the framework
  tsconfig `paths` (same mechanism the file already used for `dom-accessibility-api`).
* `*.wasm?url` (Vite asset query, used by `🧊️3d`'s brep loader) had no declaration — added to
  `🌿️ambient/🟦️.d.ts`.
* The hand-written `bun:test` declaration
  (`🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts`, T2's file, explicitly
  included by my tsconfig) was missing `expect.any` — added one line beside `arrayContaining`.

### k. Miscellaneous real defects
* `🎭️actor/🧬️typegen/🏃️execution/🟦️.ts` read `script.repoRoot` / `script.root`, which are
  `protected` on `Script` (`📚️library/🏃️process/🧭️routing/🟦️.ts:7-8`). The helper now takes the two
  paths as parameters instead of reaching into another object's protected state.
* `assert.throws(fn, undefined, message)` (3 sites) — `undefined` is not an `AssertPredicate`;
  now `assert.throws(fn, Error, message)`, which also asserts more.
* `React.useEffect(… return () => lifecycle.push("cleanup"))` in `💬️Dialog`, `📑️Tabs` and
  `🗨️Popover` component tests returned `Array.push`'s number as a destructor — wrapped in a block.
* `Object.freeze = value => {…}` in `🌱️value/💾️resident/…/🧪️resident-oracle` (4 sites) gave `value`
  an implicit `any`; the replacement is now declared `const failingFreeze: typeof Object.freeze`
  and assigned, so the parameter is contextually typed.
* `🔽️Select` component tests: 9 `<Select>` elements and the `BasicSelect` helper's default `{}` were
  missing the required `id`; the helper's parameter is now `Omit<ComponentProps<typeof Select>, "id">`.
* `📨️UIDialog` component test passed a raw fixture `string` where `changeLanguage` wants
  `UiLocale`; added a `uiLocaleOf` boundary that runs the real `isShellLocale` guard (no cast).
* `🛂️manifest/🧪️tests/🔬️tool-run-actions` read `.run` off a JSON-union element; a real
  `if (!("run" in candidate)) throw` narrows it.
* `🪟️resolved-host-context` used `"remove" in row` (which does not remove `undefined`) — now
  `row.remove ?? []`.
* `🖱️ui/🎨️styling/🧪️tests/🧩️suite`: the `OwnedBuildServer` stubs (4 sites) were missing the required
  `ws` member.

## 4. Verification

**Typecheck** — run nine times end to end; final capture `🗑️generated/t3-run9.txt`. No new error
code appeared across the sequence; every per-file count is monotonically non-increasing except the
one regression I introduced and fixed within the same session (a codemod added a second `id` to a
`<Select>` that already had one, TS17001, gone in `t3-run7`).

**Framework vitest gate** — `bun ./📜️script.ts test --run` in `🧰️framework/📦️packages/🟦️typescript`,
the gate O1 repaired:

| capture | result |
|---|---|
| `🗑️generated/t3-vitest-1.txt` (mid-session) | exit 0 — **Test Files 2 passed (2), Tests 141 passed (141)** |
| `🗑️generated/t3-vitest-2.txt` (after every edit) | exit 0 — **Test Files 2 passed (2), Tests 141 passed (141)** |

Non-zero test count asserted in both runs (141, not 0), so the gate is really executing.

**`🌱️value/💾️resident` suite** — the only edit of mine that changes a runtime binding
(`Object.freeze` replacement) sits in that module's oracle, so I ran it directly:
`bun ./📜️script.ts test --run` in `🧰️framework/🔨️modules/🌱️value/💾️resident` → exit 0, capture
`🗑️generated/t3-vitest-resident.txt` (`admissionFailures=5 … strictTS=0 oracle=Ajv+Immer+Buffer+BigInt`).

**Not run:** `bun nx run @semio-tech/value-resident:test` never got past nx project-graph
construction in 10 minutes under the current fleet load (capture overwritten by the direct run); I
used the package's own `📜️script.ts` instead, which is what that target invokes.

**Verified by typecheck only, not by a test run:** the storybook `*.story.tsx` edits (no vitest
project collects them — they are Storybook CSF, and no storybook build was run), the
`🧱️elements/*/🧪️tests/🧩️component` React suites, and the `🖱️ui/🎨️styling` suite. The
`TutorialSlice.document → .artifact` rename is covered by assertions inside
`🧪️owned-locale-detector-retirement`, which is an in-source suite of `🎯️targets/⚛️react/🟦️.tsx` and
is **not** in the framework gate's `includeSource` — so that rename is typecheck-verified only.

## 5. Honest gaps — the 65 diagnostics I still own

| file | n | what it is |
|---|---:|---|
| `📡️replication/📦️packages/🦀️rust/📜️script.ts` | 12 | JSON-fixture union members (`row.offset`/`row.value`/`row.authority` present on only some `negative` rows) + `Buffer<ArrayBufferLike>` vs `Buffer<ArrayBuffer>`. Needs per-row discriminated fixture types; same treatment as §3e but the fixture is far more heterogeneous. |
| `◻️2d/🟦️.ts` | 9 | **Real defect, not type debt.** The module loads its drawing wasm with `import("./🟦️")` — it imports *itself*. The intended target is the flow-core wasm-pack bindings (the sibling `🧊️3d/🟦️.ts:340` shows the pattern), but `🌊️flow/🫀️core/🕸️bindings/flow_core.d.ts` exports **no** drawing functions (`render_drawing_scene`, `export_drawing_svg`, `export_drawing_pdf`, `dispose_drawing`, `trace_drawing_bitmap`, `boolean_drawing_segments` are all absent). I did not invent a target: the 2d drawing wasm is simply not wired into the flow core pack. Needs an owner decision, not a type annotation. |
| `🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component` | 6 | react-flow `Node` generic vs the test's own node literal; `DiagramLayoutDescriptor.kind` is a `string` where a union is required; one `DiagramForceNode.id` missing. |
| `🎭️actor/🤖️generated/🎭️actor/🟦️.ts` (+ `🎭️actor/🟦️.ts`, `⏳️async/🟦️.ts`) | 6 | TS2307 on `./🤖️generated/🟦️actor.js`, `../🚪️lifetime/🟦️component.js`, `./🤖️generated/🟦️async.js` — wasm-component bindings that are **not on disk**. Regenerating them is the wasm build, i.e. T1's pipeline, not a source edit. |
| `🎭️actor/📤️return/…/🧪️actorreturn-codecs…` + `🚪️lifetime/…/🧪️actor-instance-close-fault…` | 6 | TS7053: fixture `kind` strings indexing a closed record; needs the fixture's kind column typed as the record's `keyof`. |
| `🌱️value/💾️resident/🧪️tests/🧪️resident-oracle` | 4 | remaining TS7006 on other callbacks in the same oracle. |
| `🖱️ui/🎨️styling/🧪️tests/🧩️suite` | 3 | parse5's `exports` map has no `"types"` condition, so `ReturnType<typeof parse>` resolves to `unknown` under `moduleResolution: "bundler"`; needs a `paths` entry like the two in §3j. |
| `⏯️tool-run/🧪️tests/🧩️conformance` | 3 | `typeof ToolRunCodecError` rejected as an `AssertPredicate`. |
| `📡️replication/📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract` | 3 | one more fixture binding (line 237) not yet annotated. |
| 14 further files | 13 | one or two each: `import.meta.glob` (theme), `OwnedBuildServer.config`, duplicate `act` re-export, `SourceFile.parseDiagnostics`, `PluginManifest` cast in `🎠️kernel`, `kernelGeometry` namespace, `DslValue` union in `🗂️map`, … |

**This slice did not reach zero.** 85 % of the owned backlog is gone and every remaining item is
listed above with its cause; two of them (`◻️2d`, the missing wasm-component `.d.ts`) are not
type-annotation problems at all and cannot be closed from inside this slice.

Also outstanding and **not mine**: the 110 diagnostics from T2 (`📚️library`, 48), T4 (`✏️s` 22,
`💻️os` 21) and the root `📜️script.ts` (19) that the framework program reaches through imports. The
framework `typecheck` target will not go green until those slices land, even after my 65 are gone.

## 6. Files changed

**Config / infrastructure (5)**
`🧰️framework/📦️packages/🟦️typescript/tsconfig.json`,
`🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts`,
`🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🔤️fast-json-stable-stringify/🟦️.d.ts` (new),
`.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.

**Production / module sources (9)**
`🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (type re-exports + `TutorialSlice.artifact`),
`🖱️ui/🎨️styling/🟦️.ts`,
`🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts` (`export type Profile`),
`🎭️actor/🧬️typegen/🏃️execution/🟦️.ts`,
plus `export` added to caller-local test seams by the dependency codemod in
`🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, `🖱️ui/🎨️styling/🌓️theme/🟦️.ts`,
`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, `🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts`,
`🎭️actor/📤️return/📨️response/🟦️.ts`.

**Shared ambient declaration (1, T2's tree)**
`🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts` (+1 line: `expect.any`).

**Vitest configs (8)** — dead `test.mode` removed:
`🎭️actor`, `🧊️3d`, `🔄️machine`, `📡️replication`, `🎠️kernel`, `⏳️async`, `◻️2d`,
`🖱️ui/🎨️styling` → `🧪️tests/🎚️config/🟦️.ts`.

**Stories (19)** `🖱️ui/📖️stories/{🎭️mode,🎭️app,🎭️ui,🎭️engagement,🎭️providers,🎭️uiintroduction,🎭️action-dropdown,🎭️navbar-example-select,🎭️panel-tab-bar,🎭️selection-marquee,🎭️sortable-tree-items,🎭️unified-gumball,🎭️label}` and
`🖱️ui/🧱️elements/{↕️Collapsible,⚡️ActionGroup,🌳️Tree,💬️Dialog,🖱️ContextMenu,🗨️Popover,🪟️Window,📊️Table,🔚️Footer,🔝️Navbar,🎀️Ribbon,↔️Resizable}/📖️stories`.

**Tests (≈35)** — the 28 suites the dependency codemod retyped, plus
`🔽️Select`, `📨️UIDialog`, `📑️Tabs`, `💬️Dialog`, `🗨️Popover` component tests,
`🔬️interactivity-mounted-{layout-text,surface-lane,prepared-render}`,
`🪟️resolved-host-context`, `🔬️tool-run-actions`, `🔣️json-projection`,
`🧪️resident-oracle`, `🧪️semio-tech-machine`, `🧪️source-contract` (×2),
`🧪️levels-oklabmix`, `🧪️owned-locale-detector-retirement`,
`🧪️typednodefields-…`, `🧪️ownedactorturnoutput`.

**Tools left in this folder (new, keep)**
`🐍️t3-typed-test-deps.py`, `🐍️t3-annotate-bun-fixtures.py`
(both reusable; the latter builds on the predecessor's `🐍️t3-fixture-types.py`).
The predecessor's `🐍️t3-type-dependencies.py` is superseded by `🐍️t3-typed-test-deps.py` — it is the
script that produced the broken `Pick<>` unions described in §3a.

---

# T3b — continuation: the 65 of §5 taken to ZERO

Slice T3b (Opus), same ticket, same scope, same rules. Picks up exactly the §5 backlog above.
All captures in `🗑️generated/t3b-*.txt`. Tools added to the ticket folder: `🐍️t3b-scope.py`,
`🌿️t3b-ambient.d.ts`, `🔣️t3b-one.json`, `🔣️t3b-scope.json`, `🐍️t3b-render-actor-typegen.py`.

## 7. Result

Command (unchanged): `bun ./📜️script.ts typecheck --pretty false` in
`🧰️framework/📦️packages/🟦️typescript` — i.e. exactly what `@semio-tech/framework:typecheck` runs.

| capture | whole program | **T3-owned (`🔨️modules/**`)** | root `📜️script.ts` |
|---|---:|---:|---:|
| `t3b-base.txt` (T3's §5 backlog, re-measured at T3b start) | 148 | **65** | 0 |
| `t3b-run1.txt` | 83 | 1 | 1 (a peer's, gone by the next run) |
| `t3b-run2.txt` | 81 | **0** | **0** |
| `t3b-run3.txt` | 45 | **0** | **0** |
| `t3b-run4-final.txt` (**final**) | **45** | **0** | **0** |

**T3-owned: 65 → 0.** The whole-program number keeps falling because T2/T4 are landing at the same
time; the T3 column is the one this slice owns. Final split of the 45 that remain, none of them mine:

| owner | count |
|---|---:|
| **T3 — `🧰️framework/🔨️modules`** | **0** |
| T2 — `🛍️products/🦑️repo/📚️library` | 12 |
| T4 — `🛍️products/💻️os` | 11 |
| T4 — `✏️s` plugins | 22 |
| root `📜️script.ts` | 0 |

No `any` blanket, no `@ts-ignore`, no `@ts-expect-error`, no `skipLibCheck` flip, nothing removed
from the tsconfig `include`; `exclude` is byte-identical to what T3 left. (A working-tree grep for
those pragmas under `🧰️framework` does show four `any` lines — `ref={wrapperRef as any}` in
`🕸️Diagram/🟦️.tsx`, which T3b never opened, and three in `🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx`
around a `changes: any[][]` binding T3b did not write. They belong to another agent's uncommitted
work in the same files; §12 lists exactly what T3b changed there.) The only tsconfig change
in this slice is one `paths` entry that was added and then **removed again** once measurement showed
it unnecessary (see §9c) — net zero.

## 8. Method: the per-file scoped tsconfig loop

`🐍️t3b-scope.py <out.json> <repo-relative source>…` writes a tsconfig that `extends` the real
framework tsconfig (so every `compilerOptions`, `paths` and ambient declaration is identical),
replaces `include` with `[]`, and lists the named sources plus three always-present declaration
files in `files`. Measured cost: **1.4–1.8 s for one file's graph** against **27–32 s** for the whole
program. Diagnostics are identical for the named files.

Two things had to be added before the loop was faithful, both because `include` is what normally
drags the ambient declarations into the program:

* `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts` and
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts` are appended to
  every scope (the second is the hand-written `bun:test` declaration).
* `🌿️t3b-ambient.d.ts` (ticket folder) carries a single
  `/// <reference types="vitest/importMeta" />`. Without it every scoped run invented a phantom
  `TS2339: Property 'vitest' does not exist on type 'ImportMeta'`, because in the full program that
  reference is pulled in by whichever module file happens to be in the graph.

`🔣️t3b-scope.json` (all 22 owned files at once) reproduces all 65 owned diagnostics plus 9 that
belong to T2/T4 — it is the cross-check that the loop is not hiding anything; `🔣️t3b-one.json` is
the working file the loop rewrites.

## 9. The two decisions the brief asked for

### a. `◻️2d/🟦️.ts` imports itself — the drawing-wasm surface is dead, and it is now gone (−9)

`git log -p --follow` on the module shows the specifier's whole life:

```
../../../../framework/product/os/module/flow/core/rs/pkg/flow_core.js      (pre-emoji)
…/🌊️flow/🫀️core/pkg/⚡️implementations/🦀️rust/flow_core.js
…/🌊️flow/🫀️core/pkg/flow_core.js                                          (commit …589)
./🟦️                                                                       (commit 025ec86a42 / b0dfa0f09b)
```

The last hop is the corruption: a path-rewrite pass collapsed the flow-core wasm-pack specifier
(`pkg/flow_core.js`, plus its `flow_core_bg.wasm?url` sibling) onto the module's own path. The
rename target it was chasing does exist — `🌊️flow/🫀️core/🕸️bindings/flow_core.{js,d.ts}` — but
**that pack exports no drawing functions**: its surface is `brep_invoke`, `dispose`, `tessellate`,
`flowAttachSurfaceCanvas`, `initialize_browser_clock`, `initSync`. `render_drawing_scene`,
`export_drawing_svg`, `export_drawing_pdf`, `dispose_drawing`, `trace_drawing_bitmap` and
`boolean_drawing_segments` are nowhere in the repo as wasm-bindgen exports.

Where the drawing code actually lives now: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs`,
reached through the **plugin action ABI** (`🌊️flow/🕸️wasm/🦀️.rs` dispatches operations 2606/2607/2609…
which call `dispose_drawing(text(args, "handle")?)` &c.), and through the `🖍️draw` flow extension
(`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw`, whose Rust tests use
`flow_extension_sdk::{boolean_segments_json, render_scene_json, export_svg_json, export_pdf_json,
trace_bitmap_json, dispose_drawing}`). There is no free-function wasm surface to bind, and there has
not been one since the drawing moved behind the ABI.

Consumers: **none.** A repo-wide grep (`*.ts`, `*.tsx`, excluding `node_modules`) for
`ensureDrawingWasmLoaded`, `createDrawingWasmBridge`, `createDefaultDrawingWasmBridge`,
`DrawingWasmModule`, `DrawingWasmBridge`, `DrawingExportBridge`, `booleanPathsClient`,
`parseSceneJson`, `parseSegmentsJson`, `encodeSegmentsForWasm`, `parseExportPayload` returns hits
**only inside `◻️2d/🟦️.ts` itself**. The module's own in-source suite depends on
`canvasDrawingPngExportPort`, `drawingSceneFromPreviewPayload` and `isDrawingRef` — none of them in
that closure.

Greenfield, so: **the dead surface is removed**, not re-pointed at a stub. Deleted from
`🧰️framework/🔨️modules/◻️2d/🟦️.ts` — `DrawingWasmBridge`, `DrawingExportBridge`, the whole
`#region 🔌️WasmBridge` (`DrawingWasmModule`, the `drawingWasm` ephemeral box, `parseSceneJson`,
`parseSegmentsJson`, `encodeSegmentsForWasm`, `booleanPathsClient`, `parseExportPayload`,
`ensureDrawingWasmLoaded`, `traceBitmapViaWasm`, `booleanPathsViaWasm`, `createDrawingWasmBridge`,
`createDefaultDrawingWasmBridge`) and the now-unused `ephemeralBox` import. `drawingSceneFromPreviewPayload`
survives in a new `#region 🎬️PreviewPayload`; the contracts (`DrawingScene`, `PathSegment`,
`DRAW_BOOLEAN_OPERATIONS`, the three export ports) and the whole canvas rasteriser are untouched.
The header docstring now says "canvas raster" instead of "WASM bridge". `◻️2d`'s own gate still
passes (§10).

**Follow-up that is not mine:** the dev stub
`🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` (`PLAYGROUND_WASM_JS_STUB`) still exports six
`render_drawing_scene`/`export_drawing_svg`/… stubs for a wasm pack that no longer declares them.
They are inert strings in a Vite `load()` hook, they cost no diagnostic, and the playground's wasm
stub surface is not this slice's contract — flagged, not touched.

### b. The 6 × TS2307 on wasm-component declarations "not on disk" (−6)

They were three different things, none of them a missing build artefact, and **no hand-written
stand-in `.d.ts` was committed**.

1. `🎭️actor/🟦️.ts:9` re-exported `./🤖️generated/🟦️actor.js`. The generator's declared output
   location is `🤖️generated/🎭️actor/🟦️.ts` — `🧬️typegen/📋️plan/🟦️.ts:6` (`actorTypegenTarget`) and the
   nx target's own `"outputs": ["{projectRoot}/../../🤖️generated/🎭️actor"]` both say so, the file is
   on disk, and every other module in the repo spells this `./🤖️generated/<name>/🟦️.ts`
   (`🛂️manifest`, `🔌️plugin/📇️registry`, …). Repointed at the generator's real output.
2. `⏳️async/🟦️.ts:11` — identical defect, identical fix (`./🤖️generated/⏳️async/🟦️.ts`, which exists).
3. The remaining 4 are **inside** the generated mirror and are emitted by the generator:
   `🎭️actor/🦀️.rs` holds the TypeScript projection as string literals, three of which spell
   `import("../🚪️lifetime/🟦️component.js")` / `import("../🚪️lifetime/🩹️patch/🟦️component.js")`.
   Relative to `🤖️generated/🎭️actor/` that resolves to `🤖️generated/🚪️lifetime/…`, which never
   existed; the real modules are `🚪️lifetime/🟦️.ts` and `🚪️lifetime/🩹️patch/🟦️.ts`, two levels up.
   **Fixed at the source**, in `🧰️framework/🔨️modules/🎭️actor/🦀️.rs` (lines 156/166/176), to
   `import("../../🚪️lifetime/🟦️.ts")` and `import("../../🚪️lifetime/🩹️patch/🟦️.ts")`.

   Regenerating the mirror needs `cargo test --locked --features typegen` (`🧬️typegen/🏃️execution/🟦️.ts`),
   which this slice must not run. `SchemaMetadata::render_typescript` (`🦀️.rs:206`) is a pure
   concatenation of those literals, so the identical textual substitution was applied to the
   committed mirror `🤖️generated/🎭️actor/🟦️.ts`. `🐍️t3b-render-actor-typegen.py` (ticket folder)
   proves the equivalence: it extracts every `typescript: "…"` literal from `🦀️.rs` and asserts each
   occurs byte-for-byte in the mirror — **9/9 present**, including all three edited ones. A clean
   checkout that runs `bun nx run @semio-tech/framework-actor-rs:typegen` therefore reproduces
   exactly the bytes now on disk. Re-run that script after any future edit to the `TYPES` table.

   **Stale comment, not mine to fix:**
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/next.config.ts:12-19`
   turns on `typescript.ignoreBuildErrors` and justifies it with exactly this generator defect
   ("a specifier its generator emits for a file that does not exist"). The defect is fixed; that
   escape hatch can now be removed by whoever owns the coordinator app — it needs a Next build to
   verify, which this slice cannot run.

### c. Root `📜️script.ts`

T4 reported 0 and T3b confirms 0 in `t3b-run2.txt` and `t3b-run3.txt`. One transient TS2304
(`AGENT_INSTRUCTION_ALIASES`) appeared in `t3b-run1.txt` from a peer's in-flight edit to that file
and was gone — fixed by the peer — before the next run. Nothing owed here.

## 10. Fix families (the other 50)

### a. Fixture unions in `📡️replication/📦️packages/🦀️rust/📜️script.ts` (−12)
`RetainedVerificationFixture.negative` was a single row type with `offset?`/`value?`/`repairCrc?`/`hex?`
optional, so every branch of the mutation switch saw `number | undefined`. Replaced by a real
discriminated union on `operation`, derived from the fixture on disk
(`📐️format/🔎️verification/🧫️fixtures/🔣️.json`): `replace-first-length` carries `hex`; `record-limit`
and `file-limit` carry `value` (declared as two separate members — a shared
`"record-limit" | "file-limit"` discriminant does **not** narrow away in the final `else`);
the four `*-xor` operations carry `offset`+`value`+`repairCrc`. `compressed` became a union on
`error: null` (the only row with `error: null` is the only row carrying `rawHex`, verified against
all ten fixture rows). `const parts = [header]` → `const parts: Buffer[]`, so
`Buffer<ArrayBufferLike>` frames can be pushed next to the `Buffer<ArrayBuffer>` header.
Two `structuredClone(fixture)` schema-rejection probes mutate a property the fixture type does not
have (on purpose — they prove `additionalProperties: false`); they now go through
`JSON.parse(JSON.stringify(fixture)) as { … Record<string, unknown>[] }`, which is the annotation of
a `JSON.parse` result, not a cast between incompatible types. `assert.throws(fn, undefined, id)` →
`assert.throws(fn, Error, id)`.

### b. Local `type X = any` stubs left by the suite extraction (−6, and 10 stubs gone)
`🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-…` declared four
(`ActorInstanceCloseRequest`, `ActorInstanceLifecycleReceipt`, `ActorInstanceLifecycleWire`,
`ActorInstanceOpenRequest`) and `🎭️actor/📤️return/🧪️tests/🧪️actorreturn-codecs-…` six
(`ActorReturnControl`, `ActorReturnDrive`, `ActorReturnIdentity`, `ActorReturnOrigin`,
`ActorReturnPageReceipt`, `ActorReturnResult`). All ten replaced by `import type { … } from "../../🟦️.ts"`,
which is what makes the fixture `kind` columns index the closed tag records (the TS7053 family).

### c. `typeof Object.freeze` does not contextually type its parameter (−4)
`🌱️value/💾️resident/🧪️tests/🧪️resident-oracle` replaces `Object.freeze` four times to fault the
freeze of a specific class. `const failingFreeze: typeof Object.freeze = value => …` looks typed but
`Object.freeze` is **overloaded**, and TS does not contextually type a parameter from an overloaded
target — every `value` stayed implicitly `any`. Written as
`const failingFreeze: typeof Object.freeze = <T>(value: T) => …`, so the parameter is explicit and
the declared variable type is unchanged.

### d. Declarations that under-described a real API (−7, all in shared ambient/contract types)
* `bun:test`'s hand-written declaration
  (`🛍️products/🦑️repo/📚️library/🏃️process/🌿️environment/🟦️.d.ts`, T2's file, explicitly included by
  the framework tsconfig): `toThrow(expected?: string | RegExp | Error)` rejected the error
  **constructor** bun:test really accepts — 3 × TS2345 in `⏯️tool-run/🧪️tests/🧩️conformance` on
  `.toThrow(M.ToolRunCodecError)`. Widened with `| (new (...args: never[]) => Error)`.
  `toBeCloseTo(expected: number, precision?: number)` was missing entirely (5 sites in
  `🖱️ui/🎨️styling/🧪️tests/🧩️suite` that a peer added mid-session) — added.
* `OwnedBuildServer` (`🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`) declared only `middlewares`
  and `ws`, while `playgroundStaleOptimizeDepPlugin` reads `server.config.root` /
  `server.config.cacheDir` — Vite's `ResolvedConfig`. Added
  `readonly config: { readonly root: string; readonly cacheDir: string }`, and gave the four
  `configureServer` stubs in `🧩️suite` a real `config`.
* `🖱️ui/🎨️styling/🌓️theme/🟦️.ts` uses `import.meta.glob` with no `/// <reference types="vite/client" />`
  in the file — added, matching `◻️2d`'s header.

### e. Types that were lying about production behaviour (−3, production code)
* `PluginSourceEvent`'s snapshot arm required `rebuiltAt: number`, but `createBundledPluginSource`
  (`🎠️kernel/🟦️.ts:2918`) deliberately emits `rebuiltAt: undefined` — its own comment says so
  ("No per-plugin `rebuiltAt` on the connect-time snapshot"), and every reader downstream already
  takes `number | undefined` (`PluginSource.moduleUrl(pluginId, rebuiltAt?)`,
  `pluginAvailabilityRouteV1(…, eventRebuiltAt: number | undefined)`, which returns `"drop"`
  precisely for `undefined`). Made `readonly rebuiltAt?: number` and widened the two ShellHost
  bindings that were typed `number` while receiving `undefined` at runtime
  (`🏛️ShellHost/🟦️.tsx` `pending` and `handlePluginAvailable`).
* `scopeContributionsJson` (`🎠️kernel/🟦️.ts:379`) demanded a whole `PluginManifest` per entry but
  reads only `manifest.topicContributions`. Narrowed to
  `Pick<PluginManifest, "topicContributions">`, which deletes the `as PluginManifest` cast the
  scope-contributions test needed (and all real callers still satisfy it).
* `🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.ts` star-exports both `../../🖌️render/🟦️.ts`
  and `../../🟦️.tsx`, and **both** exported `act` — the render adapter's own
  `act(update) { testingAct(update) }` and a bare re-export of React's `act` sitting in the react
  barrel's hook line. No file in the repo imports `act` from either barrel (they take it from
  `react` or `@testing-library/react`), so the duplicate came out of the production barrel
  (`⚛️react/🟦️.tsx:116`) and the test adapter's wrapper stays the single `act`.

### f. Literals that widened away from the type they were built for (−6)
`IconRenderRequest`'s `camera.position`/`target` are 3-tuples; two request literals in the react
target's in-source suite inferred `number[]`. Annotated the bindings (`const request: IconRenderRequest`)
instead of asserting the tuples. `DiagramLayoutDescriptor.kind` is `typeof DIAGRAM_LAYOUT_CODEC_KIND`;
the malformed-descriptor array widened it to `string`, so the array got a name and a type
(`const malformed: DiagramLayoutDescriptor[]`). `largeDiagramNodes()` returned
`{ …; data: Record<string, never> }[]`, which is not react-flow's `Node` — the two `onNodesChange`
mocks and the `onNodeDrag` mock derived their parameter types from it and could not be handed to
`Diagram`. The factory now returns `Node[]` (re-exported by `🕸️Diagram/🟦️.tsx`), which fixes all
three mocks at once. `onNodesChange.mock.calls[0]![0].at(-1)` gained its `!`.

### g. Narrowing instead of `!` on real unions (−4)
* `playgroundPlayBootHtmlPlugin().transformIndexHtml!.handler!({} as never).tags` asserted its way
  through `OwnedBuildHtmlHook | { order?; handler }` **and** `OwnedBuildHtmlResult`. Replaced by two
  real guards (`typeof hook !== "object"` → throw; result must be a non-array object with `"tags" in`
  it → throw) and a real call `hook.handler("", { path, filename })`; the test became `async`.
* `📡️replication/📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract` line 237: the last
  un-annotated `await Bun.file(…).json()` binding, narrowed to `unknown` by its Ajv assertion (the
  §3e family). Annotated with a `readonly` structural type derived from the interaction set-state
  fixture on disk.
* `📡️replication/🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-source`: the base documents of the
  associativity triple-loop (`[{}, { key: "old" }, { key: null }]`) inferred a three-member union
  that is not a `Record<string, DslValue>`. A `freshBases(): Record<string, DslValue>[]` factory
  gives the loop a typed, still-fresh-per-iteration array and removes the `as Record<…>` cast.
* `🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js` took `dependencies: any`, aliased
  `type MeshTransfer = any` and destructured `kernelGeometry` as a **value** so it could write
  `as kernelGeometry.FaceRef` in type position (TS2503 — the §3f shadowing family; this was the one
  case T3's codemod could not resolve). Now `import type * as brep from "../../🟦️.ts"` with
  `brep.MeshTransfer` / `brep.kernelGeometry.FaceRef`, the dependency bag is a real `Pick<>`, and the
  caller (`🧊️3d/🟦️.ts:531`) stops passing a namespace that only the type system needs.

### h. Internal compiler API and third-party resolution (−4)
* `🖱️ui/🧪️tests/🎚️axes` read `SourceFile.parseDiagnostics`, which is not on the public type.
  Replaced by the public `ts.transpileModule(source, { …, reportDiagnostics: true }).diagnostics`,
  which asserts the same property (the emitted axes projection parses clean) through a supported API.
* `🧩️suite`'s parse5 visitor used `ReturnType<typeof parse>["childNodes"][number]`. `parse` is
  **generic** (`parse<T extends TreeAdapterTypeMap = DefaultTreeAdapterMap>(…): T["document"]`), so
  `ReturnType` instantiates it at the constraint and yields `unknown` — the three diagnostics were
  never a resolution failure. Named the real type instead:
  `import("parse5").DefaultTreeAdapterTypes.ChildNode`. I first added a `paths` entry for parse5,
  measured that it changed nothing, and **removed it again**; the framework tsconfig's `paths` is
  back to T3's two entries.

### i. One peer-owned drift repaired in passing
`ui.settings.theme.contrast` (a peer's live WCAG verdict feature, `📚️I18n/🟦️.tsx` modified 9 minutes
before the run) declares `readonly label: UiLabelValue` and `📌️ChromePanels/🟦️.tsx:716` calls
`shellLabel("ui.settings.theme.contrast.label")` — which only type-checks if that node is a
`UiLabelValue`. Both locale literals in `⚛️react/🟦️.tsx` (2740 de, 3615 en) supplied a bare
`UiLabelPair`. Wrapped to `label: { label: { normal, beginner } }`, matching the schema and the
consumer. Flagged here because the peer may still be moving that feature.

## 11. Verification

**Typecheck** — run end to end four times (`t3b-base`, `t3b-run1`, `t3b-run2`, `t3b-run3`). Owned
count is monotonically non-increasing; no new error code was introduced at any step. Every
individual fix was additionally confirmed against the scoped loop before the next one started.

**The nx target itself, end to end** — `bun nx run @semio-tech/framework:typecheck`,
capture `🗑️generated/t3b-nx-typecheck.txt`. This is the run T3 never got: nx built the project graph,
executed the target (`typecheck 41.2s`, run duration `2m 0s`) and printed real `tsc` output.
**Owned diagnostics in the nx capture: 0.** The task is reported as failed and the command exits 1,
because the target does not filter by owner and T2/T4 diagnostics are still in the program — that is
the expected state until those slices land, and it is the same non-zero T3 documented.

**Framework vitest gate** — `bun ./📜️script.ts test --run` in `🧰️framework/📦️packages/🟦️typescript`,
capture `🗑️generated/t3b-vitest-1.txt`: **exit 0, Test Files 3 passed (3), Tests 163 passed (163)**.
Non-zero asserted. (T3 recorded 141/2 files; peers have since added a third file and 22 tests — the
count is higher, never zero.)

**Per-module suites for every module whose runtime I touched** (captures `t3b-vitest-*.txt`):

| module / target | command | result |
|---|---|---|
| `◻️2d` | `📦️packages/🟦️typescript/📜️script.ts test --run` | exit 0 — 3/3 |
| `⏳️async` | same | exit 0 — 24/24 (exercises the repointed `🤖️generated/⏳️async` re-export) |
| `🧊️3d` | same | exit 0 — 1/1 (exercises the new `brep.*` dependency bag) |
| `📡️replication` | same | exit 0 — 6/6 |
| `📡️replication` `test-source` | `📦️packages/🦀️rust/📜️script.ts test-source` | exit 0 (local-interaction + ordered source contracts) |
| `📡️replication` `presence-peer-codec-check --oracle-only` | same script | exit 0 — 27 vectors, 23 hostile inputs (exercises both runtime edits in that script: the `JSON.parse` schema-rejection probe and `assert.throws(…, Error, …)`) |
| `🌱️value/💾️resident` | `💾️resident/📜️script.ts test --run` | exit 0 (`admissionFailures=5 resourceWrapper=5 finalizerFrontiers=8` — every `failingFreeze` path) |
| `🖱️ui/🎨️styling` | `🎨️styling/📦️packages/🟦️typescript/📜️script.ts test --run` | 61 pass / 1 fail — see below |
| `🖱️ui/🎯️targets/⚛️react` | `…/📦️packages/🟦️typescript/📜️script.ts test --run` | 778 pass / 18 fail — see below |
| `🕸️Diagram` alone | same, filtered `🕸️Diagram` | **exit 0 — 51/51** |
| `🎠️kernel` | `📦️packages/🟦️typescript/📜️script.ts test --run` | 70 pass / 1 fail — see below |
| `🎭️actor` | same | 265 pass / 15 fail — see below |

**Every one of those failures is pre-existing repo debt, not this slice.** Evidence, per cluster:

* `🎭️actor` (8 of 15) and `🎠️kernel` (1 of 1) fail with
  `Error: can't resolve reference …/framework/value/schema.json#/$defs/NonZeroU64`. `NonZeroU64` is
  absent from `🌱️value/🧬️schema/🔣️.json` (grep count 0), which was last committed 2026-09-15 and is
  **unmodified in the working tree**. Pure JSON-schema drift; my edits in those two modules are
  type-level only.
* The other 7 `🎭️actor` failures are source-inventory oracles
  (`expected ['residentLedger', …43] to deeply equal […40]`, `PendingEntry […8] vs […5]`) in
  `📤️return/📨️response/🟦️.ts`, `📮️shard-client/🟦️.ts` and `🪪️activation/🚪️instance/📥️output/🟦️.ts` —
  files with uncommitted peer edits, none of which I touched. My two changes in that module's test
  tree are `import type` + removed `= any` aliases, which erase at runtime (the diff is in §12).
* `🖱️ui/🎨️styling`'s single failure is a CSS regex
  (`[data-slot="panel-tabs"] > [data-slot="panel-tab-button"] { border-inline-end-color: … }`)
  against `uiCss`. I changed no CSS.
* 16 of the 18 `⚛️react` failures are
  `ENOENT: … 🧰️framework/🎨️styling/🖌️ui/🎨️.css` — a peer is mid-move of that stylesheet.
  `📨️UIDialog` fails on the same ENOENT path.
* The one `🕸️Diagram` failure in the full react run
  ("keeps controlled input positions stable while emitting cooperative proposals") is a timing law
  (`expect(setupElapsed).toBeLessThan(8)`, `Math.max(...elapsed) <= 6.1`) starving under the cargo
  fleet. Re-run in isolation: **51/51 pass, exit 0**, which also exercises the one real runtime edit
  in that file (the `DiagramForceNode` identity accessor moved from `Object.defineProperty` into the
  object literal, in the oversized-identifier test).

**Verified by typecheck only, not by a test run** — and honestly so:

* `🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin` (§10g): its caller's `import.meta.vitest`
  block lives in `🏗️builder/🌐️vite/🟦️.ts`, and the styling vitest config's `includeSource` is
  `["📽️projection/🟦️.ts"]` only — no project collects it. The narrowing edit there is unexercised.
* `📡️replication`'s `retained-verification-check` and `retained-record-observation-check` both abort
  at `ajv.compile(schema)` (line 189 / 296) with
  `no schema with key or ref "http://json-schema.org/draft-07/schema#"` — those two scripts import
  `ajv/dist/2020.js` while their schema files declare draft-07. Pre-existing (both schema files are
  committed and unmodified) and it happens **before** any of my edited lines run, so the fixture-union
  rewrite in §10a is typecheck-verified only for those two oracles. The third oracle in the same file
  (`presence-peer-codec-check`) does run and does cover my two runtime edits there.
* The `🎭️actor` generator repair (§9b.3) is proven byte-equivalent by `🐍️t3b-render-actor-typegen.py`,
  not by an actual `cargo test --features typegen` run — this slice must not run cargo.
* `🎠️kernel`'s `PluginSourceEvent`/ShellHost widening and the `act` de-duplication are typecheck- and
  grep-verified (no importer of `act` from either barrel); no shell boot was run.

## 12. Files changed by T3b

**Framework config (1)** — `🧰️framework/📦️packages/🟦️typescript/tsconfig.json` (a parse5 `paths`
entry added and removed again; net unchanged from T3's state).

**Production / module sources (9)**
`🔨️modules/◻️2d/🟦️.ts` (dead drawing-wasm surface removed),
`🔨️modules/🎭️actor/🟦️.ts`, `🔨️modules/⏳️async/🟦️.ts` (generated-mirror specifiers),
`🔨️modules/🎭️actor/🦀️.rs` + `🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts` (generator specifier, in lockstep),
`🔨️modules/🎠️kernel/🟦️.ts` (`PluginSourceEvent.rebuiltAt` optional, `scopeContributionsJson` `Pick`),
`🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` (duplicate `act` export removed, 2 `IconRenderRequest`
annotations, contrast label shape),
`🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts` (`OwnedBuildServer.config`),
`🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts` (`vite/client` reference),
`🔨️modules/🧊️3d/🟦️.ts` (test dependency bag).

**Other slices' trees, minimal and root-cause (2)**
`🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (two bindings widened
to `number | undefined` to follow the kernel type),
`🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts` (`toThrow` constructor
overload, `toBeCloseTo`).

**Build/oracle scripts (1)** `🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts`.

**Tests (11)** `⏯️tool-run/🧪️tests/🧩️conformance`,
`🌱️value/💾️resident/🧪️tests/🧪️resident-oracle`,
`🎠️kernel/🧪️tests/🔬️scope-contributions`,
`🎭️actor/📤️return/🧪️tests/🧪️actorreturn-codecs-…`,
`🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-…`,
`📡️replication/🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-source`,
`📡️replication/📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract`,
`🖱️ui/🎨️styling/🧪️tests/🧩️suite`,
`🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin`,
`🖱️ui/🧪️tests/🎚️axes`,
`🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component`,
`🧊️3d/🧪️tests/🧪️semio-tech-geometry-brep-js`.

**Ticket folder, new and worth keeping**
`🐍️t3b-scope.py` (scoped-tsconfig generator, ~1.5 s per file),
`🌿️t3b-ambient.d.ts` (the `vitest/importMeta` reference the scoped loop needs),
`🐍️t3b-render-actor-typegen.py` (mirror ↔ `🦀️.rs` verbatim-projection check),
`🔣️t3b-one.json` / `🔣️t3b-scope.json` (working scopes).

## 13. Left for others

* T2 (12, `📚️library`) and T4 (33, `💻️os` 11 + `✏️s` 22) still owe their diagnostics; the framework
  `typecheck` target stays red until they land, exactly as T3 predicted.
* `next.config.ts`'s `typescript: { ignoreBuildErrors: true }` in the coordinator app is now
  unjustified (§9b) — removable by the app's owner, needs a Next build to confirm.
* `PLAYGROUND_WASM_JS_STUB`'s six drawing exports in `🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` are dead
  along with the surface removed in §9a.
* Repo-wide, unrelated to types: `$defs/NonZeroU64` is missing from
  `🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json` (9 failing tests across `🎭️actor`/`🎠️kernel`), and
  `📡️replication`'s two retained oracles compile draft-07 schemas with `ajv/dist/2020.js`.
