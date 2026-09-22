# TS1 — os product TypeScript zero + backbone-worker offline resilience

Slice TS1, session 8 (2026-09-22, started ~12:2x CEST). Inputs: `📓️g18-tree-compile-health.md`
(§TypeScript, §Vitest, §Ranked breaks), `📓️c7-hub-document-mount-and-scenario.md` §5,
`📓️status.md` tail. Foreground only, no sub-agents, no cargo.

## 0. Baseline (measured)

`cd 🧰️framework/🛍️products/💻️os && bun x tsc -p tsconfig.json --noEmit`
→ capture `🗑️generated/ts1-typecheck-000-baseline.txt`, **77 `error TS…` diagnostics, exit 2**
(G18 measured 76 at 12:08; one more arrived from peer churn in the 15 min between).

## 1. TypeScript: 77 → 4

**77 → 4** (`🗑️generated/ts1-typecheck-final.txt`, 2026-09-22 17:0x). Waves: 77 → 17 → 8 → 4.
The four that remain are §1.1; every one of them is a stale GENERATED artifact or a foreign contract,
named there with the exact command that clears it.

| file | was | root cause | fix | landed |
|---|---|---|---|---|
| `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts` | 2 | the injected `dependencies` bag and the local `DemonstratorIdleScheduler` were both `any`, so every callback param was an implicit any | `import type * as DemonstratorBrand` + a `Pick<>` dependency type; the local `= any` alias is gone | ✅ |
| `✏️s/…/🏛️architect/…/🧬️schema/🔺️diff/🟦️.ts` | 2 | the long `import type { … }` list had drifted: `KnowledgeRecord`/`BenchmarkRecord` were used but never imported (both ARE exported by `../🟦️.ts`) | added both to the import list | ✅ |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎡️ordered-scroll/🧪️tests/🔬️ordered-scroll/🟦️.ts` | 1 | `flatMap` narrowed the element type to the non-`pointer-move` members, so pushing the retained pointer sample back was rejected | annotated the `retained` array with the full event type | ✅ |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx` | 1 | `filter((v): v is string …)` over branded `UiLabel` values — a predicate type must be assignable to its parameter | predicate is `value is UiLabel`, type imported | ✅ |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | 2 | two `.catch()` fallbacks returned a bare error envelope literal, so the awaited union lost `result` | both fallbacks annotated `: McpEnvelope` | ✅ |
| `…/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts` | 1 | `Bun.build({ write })` — current Bun has no `write` key; a build writes exactly when it is given an `outdir` | `...(write ? { outdir } : {})`; verified against bun 1.3.14 that an outdir-less build writes nothing | ✅ |
| `…/🧪️tests/⌨️browser-keyboard-scope/🟦️.ts` (+ its schema and fixture) | 5 | fixture rows omitted the false modifiers, so the JSON-import union had no `alt`/`shift`; three `enqueueLossless` stubs returned `void` where the transport contract returns "admitted" | schema now REQUIRES all four modifiers, the 24 fixture rows spell them out, stubs return `true` | ✅ |
| `…/🧪️tests/⚙️settings-general-layout/🟦️.ts` | 5 | harness built `Select`/`Input` with no `id` and `Button` with no `icon` — all three are required props | supplied the ids the labels already name, and `save`/`trash-2` icons | ✅ |
| `…/🧪️tests/🎨️settings-theme-publication/🟦️.tsx` | 5 | same drift (Select `id`, four Buttons' `icon`) | supplied `id` + `save`/`rotate-ccw`/`import`/`trash-2` | ✅ |
| `…/🧪️tests/🎚️window-measure-controls/🟦️.tsx` | 1 | `onIntent` returned `Array.push`'s number where the contract returns `void` | braced the body | ✅ |
| `…/🧪️tests/🎥️tutorial-bridge/🟦️.ts` | 1 | helper param was `typeof fixture.snapshot`; `fixture.mutated` legitimately has a smaller `activeUtilityByWindowId` | widened to the union of the two fixture rows it is called with | ✅ |
| `…/🧪️tests/🎥️world3d-camera-framing/🟦️.ts` | 1 | the fixture's `projection` is a projection SPEC, spread straight into a camera state whose `projection` is the orbit mode | pass the `parallel` decision the scene camera is already built from | ✅ |
| `…/🧪️tests/🎨️world3d-glb-outline/🟦️.ts` | 1 | `setIndex(readonly number[])` | copy | ✅ |
| `…/🧪️tests/🎨️world3d-scene-shading/📜️script.ts` | 13 | four names (`React`, `createRoot`, `Canvas`, `Grid`) come from the browser bundle's own prelude like the file's existing `declare const THREE`; Ajv's `compile()` guard narrowed two `JSON.parse` fixtures to `unknown` | added the four ambient declarations with the reason, and a `PixelOracleFixture` type the validator now guards to | ✅ |
| `…/🧪️tests/🔬️engine-contract/🟦️.ts` | 2 | board session mock missing `pointerCancelScreen`; a staged `string` arg schema whose `options` are now `{value,label}` | added the mock method and the option objects | ✅ |
| `…/🧪️tests/🚗️driver-editor/🟦️.ts`, `…/🔎️ShellSearch/🧪️tests/🧩️component/🟦️.tsx` | 2 | `uiI18n.language` is a raw `string \| undefined`; `changeLanguage` takes `ShellLocale` | restore through the existing `isShellLocale` guard | ✅ |
| `…/🧪️tests/🧩️package-integration/🟦️.ts` | 1 | inferred union of env literals is not a `Record<string,string>` | annotated `contexts` | ✅ |
| `…/🧪️tests/🪟️spawned-program-session/🟦️.tsx` | 1 | `Extract<…,{kind:"row"}>` is `never` because the node declares `kind: "row" \| "column"` | Extract on both | ✅ |
| `…/🎓️HubFirstRun/🧪️tests/🧩️component/🟦️.tsx` | 1 | test port predates the three agent-delegation methods and hid it behind an `as` cast | implemented the three stubs, cast dropped | ✅ |
| `…/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx` + `🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts` | 10 | `"left" in surface` gave `unknown`; two `TreeDataItem` doubles lacked `label`; the owned `fireEvent` boundary typed every target as `Element` while the host installs its pointer listeners on `window`; a resolver held in a `let` stayed narrowed to its `null` initialiser | declared `MountedSurface`, added the labels, widened the owned boundary to `Element \| Document \| Window` (17 methods), held the resolver on an object | ✅ |
| `…/📌️ChromePanels/🟦️.tsx` + its component test | 2 | `createFrameworkSettingsPanelTab` always returns a branch but declared `PanelTabNode` | return type narrowed to `PanelTabBranch`; the test narrows its child to `PanelTabLeaf` | ✅ |
| `…/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts` | 2 | `assert(x?.includes(…))` asserts the boolean, not `x` | `assert(x !== undefined && x.includes(…))` | ✅ |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree`, `🛂️manifest`, `🗣️Interpreter` (tree windows) | 6 | the Tree element moved to a per-container `TreeWindowRowExtent` and dropped the global `rowHeightPx` param; the generated ui-contract mirror still exported `TreeWindow = {total, offset}` and no `TreeWindowRowExtent` | see §1.2 | ✅ |
| `…/🏛️ShellHost/🟦️.tsx` (2, outcome 1's critical path) + `🛠️ShellHelpers`, `👥️scoped-presence` | 3 | the props declared `plugins: { pluginId; moduleUrl }[]`, a shape narrower than the `PluginRegistryEntry[]` `expandPluginRegistry` already takes, so the registry union had no `consumes`; `presenceEphemeralSnapshotWithinBoundV1` declared `(instanceId: string) => …{presence?}\|undefined` while the real `PluginWasmHandle.ephemeralSnapshot` is `(instanceId: number) => …\|null` | props widened to `PluginRegistryEntry[]` (a pure widening — every caller's literal still satisfies it); the helper now states the handle's own signature and normalises the ABI's `null` to the beat's `undefined`, with the test carrying a `null` case | ✅ |
| `…/🗣️Interpreter/📖️stories/🧪️.story.tsx` | 2 | the story was written against a removed declarative `UiNode` JSON shape and called `interpretUiNode(node, ctx)`; the interpreter is store-driven (`interpretUiNode(store, ctx)`) and `UiNode` is gone | rewritten against the retained contract — three `BuiltNode` fixtures through `builtNodeToSnapshot` + `UiDocumentStore` + `InterpretedUiNode`, following the migrated `📸️remodel` story; typechecks, **not rendered** (§5) | ✅ |
| `…/🔌️plugin/⚡️caching/🔒️leases` + `🧑‍💻dev/🔌️vite-plugins` | 0 (vitest) | see §3.2 | `bun:sqlite` named through a `const` + `@vite-ignore` | ✅ |

### 1.1 The four that remain (all stale generated artifacts or a foreign contract)

| error | why it is not a source defect | what clears it |
|---|---|---|
| `✏️s/…/🧩️puzzle/…/✏️editor/🌉️wasm/🟦️.ts(14,3)`: `BoardSession` missing `pointerCancelScreen` | the Rust HAS it (`…/🌉️wasm/🦀️.rs:255`, `#[wasm_bindgen(js_name = pointerCancelScreen)]`); `📦️packages/🦀️rust/pkg/semio_puzzle.d.ts` is a **gitignored wasm-pack artifact last built 2026-09-17**, before that export existed | `bun ./📜️script.ts wasm` in `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`, through the fleet wasm mutex |
| `…/🪪️WasmSessionLoader/🟦️.tsx(180,3)`: `EditorSession` not assignable to `EditorWasmSession` | same shape: `🧰️framework/🔨️modules/✍️editor/🦀️.rs:1725` declares `pointerCancelScreen`; `📦️packages/🦀️rust/pkg/framework_editor.d.ts` is from **2026-09-13** | `bun ./📜️script.ts wasm` in `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust`, through the mutex |
| `🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts(24,29)`: no `defineConfig` overload matches | the config is written against the repo's OWN build contract (`🖱️ui/🎯️targets/⚛️react/🛠️build-tooling`'s `OwnedBuildConfig`/`OwnedBuildPlugin`, with `defineOwnedBuildConfig`), but imports vite's `defineConfig` and vite's own `react()`/`tailwindcss()` directly. Annotating the return `Promise<UserConfig>` turns the one diagnostic into **16** — every owned plugin factory against vite's `PluginOption`, plus `BuildEnvironmentOptions` and `DepOptimizationOptions`. TS1 measured that, then reverted: the honest fix is to move this config onto the owned boundary (and onto `uiReactBuildPlugin()`/`uiTailwindBuildPlugins()`), which is a design change to the live `dev s` config, not a type patch | moving it to `defineOwnedBuildConfig` + the owned plugin factories |
| `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts(3530,7)`: the inert cold-pair double | `VerifiedColdDocumentPair` is a **nominal** class (private `state`/`lease`/`runtimeKey`/… + a private mint token), so no structural stand-in can ever satisfy it. The six laws that install one are about backbone byte retention and the outbox, on `"demo/v1"` states with no execution-target lease at all; a REAL pair there would need a full manifest-backed lease and would then fail its own `assertCurrent()` (it checks `state.executionTargetLease === this.lease`), flipping `documentBackboneAdmissionReady` to false and changing what all six measure | either minting a real pair (and rewriting those six laws around a lease) or declaring the field by the surface production reads — the latter weakens a deliberate mint invariant, so TS1 left it and named it |

### 1.2 The tree-window contract, end to end

The Tree element moved each windowed container to its own closed row geometry
(`TreeWindowRowExtent`) and dropped the global `rowHeightPx` parameter from
`treeWindowVisibleRowsForViewport`/`treeWindowRequestsForViewport`. Three things had not followed:

- **the Interpreter's DOM observer** built its `TreeWindowContainerMeasure` without `rowExtent` while
  `treeWindowVisibleRowsForViewport` prices every row through `treeWindowRowExtentPx(container.rowExtent)`
  — an absent token returns `undefined`, so the whole container's geometry was `NaN`. It now reads the
  `data-tree-window-row-extent` the Tree element already stamps, validated against a new runtime list
  `TREE_WINDOW_ROW_EXTENTS` (`🌳️Tree/🟦️.tsx`, re-exported by `@semio-tech/ui-react`), defaulting to the
  contract's own `standard`. **This is a live rendering fix, not a type fix.**
- **three call sites** still passed the removed `rowHeightPx` (`treeWindowBodyRequestsV1`'s signature, its
  two inner calls, and the shell's report loop).
- **the generated TypeScript mirror** of the UI contract still said `TreeWindow = { total, offset }` with no
  `TreeWindowRowExtent` at all, while the Rust has carried `row_extent` (with a `#[default] Standard`) for a
  while. `bun nx run @semio-tech/ui-contract-rs:generate` could not run: its own gate
  (`🧬️typegen-export/🦀️.rs`) asserts `TYPES.len() == 81` and the contract now declares **84**, so the
  export aborted before writing and the mirror had silently frozen. Count bumped to 84, mirror regenerated
  (`🗑️generated/ts1-ui-contract-generate.txt`) — that alone cleared **5** of the remaining diagnostics.

## 2. Vitest — `@semio-tech/framework-os` offline resilience — **363 / 363 green**

`bun ./📜️script.ts test quick` from `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`.

| run | capture | result |
|---|---|---|
| baseline | `ts1-vitest-framework-os-000.txt` | 6 files, 363 tests — **3 failed**, 360 passed (reproduces G18 exactly) |
| after the capability fix | `ts1-vitest-framework-os-001.txt` | **1 failed**, 362 passed |
| after the pair-availability fix | `ts1-vitest-framework-os-002.txt` | **0 failed, 363 passed, 6/6 files, exit 0** |

### 2.1 Two of the three: the laws installed a session capability the product no longer accepts

`HUB_SESSION_CAPABILITY_PATTERN_V1` (`🧰️framework/🛍️products/💻️os/🟦️.ts`) is
`/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u`, pinned so "a proxy's error page can never be
installed into the credential-owning worker as a session". Three laws still installed
`"a".repeat(64)` / `"d".repeat(64)` (lines 4031, 4714, 5183 of
`🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`).
`installHubSessionCapability` returns `false` for those and installs nothing — its result was
ignored — so `hubSessionCapability` stayed `undefined` and **every** hub call in those laws threw
`hub session rebootstrap required` from `hubSessionFetch` (`🏪️store/👷️worker/🟦️.ts:772`). That is
exactly the two symptoms G18 recorded: `foreign-plan-scope: expected [] to deeply equal
['open-plan','manifest']` (no fetch ever happened) and `hub session rebootstrap required`.
**The product is right; the laws were stale.** They now install the suite's own well-formed
`WORKER_LAW_CAPABILITY` / `WORKER_LAW_CAPABILITY_TWO` and **assert `toBe(true)`**, so a future
tightening cannot silently disable a law again. That fixed reds 1 and 3.

### 2.2 The third: a missing canonical-pair route destroyed the lease

`browser document actor reservation activates only after an exact current socket Session` still
timed out, and the bare `"session activation test deadline"` named neither the row nor the stage.
The `wait` helper now carries a census (row name, bodies/loads/describes/activated, the status
trail), which printed:

```
valid-session {"bodies":0,"loads":0,"describes":0,"activated":0,
 "statuses":[…"verify"×4, "renderer-unavailable", "canonical-pair",
             "canonical checkpoint pair: media type mismatch"]}
```

Root cause: `activateDocumentBrowserActorAfterSession` calls C7's
`seedColdPairFromCanonicalCheckpoint` between the reservation and `owner.activate(socket)`
(`🏪️store/👷️worker/🟦️.ts:2634`). That seed threw on **transport** conditions —
`if (!response.ok) throw new Error(\`canonical checkpoint pair: unavailable (${response.status})\`)`
and `media type mismatch` — and its caller's `catch` drops the lease, emits `integrity-failed` and
closes the socket with `1008`. This law's hub double never served
`/spaces/{}/documents/{}/active-checkpoint/pair` (it predates the route), so the fall-through JSON
body tripped the media-type check and killed the whole activation.

**Product fixed, not the law.** Those two lines now `return false`. The seed's own contract already
distinguishes the two cases — line 4064 returns `false` for "there is nothing to seed" — and a hub
that does not HAND OVER a pair is that case, not an integrity violation: it is the exact state every
client was in before the route existed (no pack, no cold pair, a live socket). Any hub binary
predating C7's route answers 404, so keeping it fatal made **every cold document unopenable against
such a hub**, which is the freeze outcome 3 forbids. Every check on a pair the hub DID hand over
(scope mismatch, checkpoint mismatch, the decoder, the assembler's digests) is untouched and still
fatal.


## 3. The other six os-product vitest projects

Every number below is from a run TS1 executed. The machine was thrashing for part of this (1-min load
187–267, swap full), so the runs were serialised and bounded (`--pool=forks --maxWorkers=2`).

| project | verb | result |
|---|---|---|
| `@semio-tech/framework-os-shell` | `test quick` | **2 files / 7 tests, all green**, exit 0 |
| `@semio-tech/plugin-registry` | `test long` | **8 files / 60 tests, all green**, exit 0 (`test quick` is killed by its own 15 s budget before it finishes — §3.3) |
| `@semio-tech/framework-renderer-react` | `test long`, direct vitest | **98 files / 1955 tests — 1949 green, 6 red**, all six one cause (§3.1) |
| `@semio-tech/framework-os-dev` | `test long`, direct vitest | one red FIXED, one load fault FIXED (§3.2); the project still has named reds it never showed before |
| `@semio-tech/framework-os-mcp` | `test quick` | **11 files / 50 tests — 45 green, 5 red, 8 files red**; six distinct causes, none TS1's (`🗑️generated/ts1-vitest-os-mcp.txt`) |
| `@semio-tech/framework-renderer-wgpu` | `test quick` | **no test ran** — the verb dies in `loadTaxonomy` (§3.1) |

### 3.1 One deleted directory blocks two projects

`.ralph-tui/**` is **deleted from the working tree** (`git status --short` shows six staged ` D ` rows) —
a disk cleanup removed a TRACKED directory that the repo taxonomy
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`) declares as the required tracked
output of `generatorContracts["setup-wizard-config"]`. `loadTaxonomy()` therefore throws for every caller:

- `framework-renderer-wgpu`'s whole `test` verb (it routes through `runCargoTestBudgeted` →
  `resolveCargoPackageName` → `getCargoWorkspaceIndex` → `loadTaxonomy`), so **zero** of its laws ran;
- the six `framework-renderer-react` reds, all in `🧪️tests/🧩️package-integration/🟦️.ts`.

Recovery is a single `git restore` of `.ralph-tui` — a git-modifying command TS1 is forbidden to run.
**This is the one item that needs the coordinator.**

### 3.2 `framework-os-dev`: a red fixed at its root, and a whole file that had never loaded

- **`PluginReturnWit matches the shared fixed result vectors and exact enum subset` — FIXED.** Ajv could
  not compile the actor-return schema: `can't resolve reference …/framework/value/schema.json#/$defs/NonZeroU64`.
  Six schemas reference `NonZeroU64` in the value schema and **none defines it** — the value schema declares
  only `DslValue`. The fields it guards (`activationGeneration`, `returnSequence`, `pageSequence`) are
  carried as decimal strings, and the repo already pins that shape elsewhere
  (`🎭️actor/🪪️activation/📤️return`: `{"type":"string","pattern":"^[1-9][0-9]*$"}`). `NonZeroU64` is now
  defined in the value schema with the exhaustive `[1, u64::MAX]` alternation the actor page schema's `Word`
  already pins, minus its `0` branch. The law passes.
- **`🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` never loaded at all** (`(0 test)`): Vite refused
  `Cannot bundle built-in module "bun:sqlite"` reached from `⚡️caching/🔒️leases` and `🧑‍💻dev/🔌️vite-plugins`.
  The project's `long` level runs under jsdom (its Canvas pixel-parity cases need it), which makes Vite treat
  that graph as a CLIENT one. Both modules now name the specifier through a `const` (whose literal type still
  gives the import its real module type — no cast) and mark the import `@vite-ignore`, leaving resolution to
  the runtime that actually has Bun. **88 laws now run, 68 green.** The 18 reds they expose
  (jco bridge, vendor transport, Canvas PNG parity, and two `backboneDbHandleFor` cases that still hit the
  bundler) were hidden behind the load failure and are **not** TS1's: they are named here so they stop being
  invisible.
- Two `🧹️config` reds are pre-existing budget drift, not TS1's: `vite config module graph > stays inside the
  declared module and source-byte bounds` measures **58 modules against a declared bound of 40**, and the
  Bun-bundler cross-check disagrees by 5 modules. TS1 added no import to that config (its only change there is
  `format: "es" as const`). A third, `dev server transform freshness`, failed with `ECONNRESET` under load.

### 3.3 Two projects cannot finish inside their own budgets

`plugin-registry`'s `quick` (15 s) and `framework-os-dev`'s `quick` (30 s) and `long` (300 s) budgets, and
`framework-renderer-react`'s `long` (300 s), all kill the run before it ends on this machine. The corpora ran
only by invoking `bunx vitest run --config …` directly, which is how every number above for those projects was
obtained. Either the budgets or the corpora need revisiting; that is a decision for whoever owns the levels.

## 4. Runtime proof on a live `s` serve (6199)

Three of TS1's edits change RUNTIME behaviour, so they were taken to a live shell rather than left at
tests. Serve started by TS1 alone, on its own port, detached, `SEMIO_VITE_HMR=0`:

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
nohup env S_OS_PORT=6199 SEMIO_VITE_HMR=0 bun ./📜️script.ts serve s react dev \
  > 🗑️generated/ts1-serve-s-6199.txt 2>&1 & disown        # pid in 🗑️generated/ts1-serve-pid.txt
```

- **`VITE v7.3.6 ready in 2320 ms`, `http://127.0.0.1:6199/` answers `200`.** This is the load-bearing
  one: the dev server's own plugin module (`🧑‍💻dev/🔌️vite-plugins/🟦️.ts`) is a file TS1 edited, and
  Vite loads it while starting. It starts.
- **The shell boots and paints its whole chrome** (`🗑️generated/ts1-boot-diagnose.json`, probe
  `🐍️ts1-boot-diagnose.mjs`): `document.title` = `semio · s · home`, and the body carries
  `Artifact / Chat / Fullscreen / Editor ⌘️⌥️E / Viewer ⌘️⌥️V / Studios / Create Space / Actions /
  Utilities / Display / Remote: detached / **No one else is here** / signed out / Sign in / Settings /
  Marketplace / History / Tasks / Command`. `ShellHost` (props widened to `PluginRegistryEntry[]`) and
  `PresenceBar` (the `UiLabel` type predicate) are both on that path, and both render.
- **Not proven live: the tree-window row extent.** The home view opens no document, so the shell renders
  no `[data-tree-window-key]` container at all (`windows: 0`, `trees: 0`) — `🐍️ts1-tree-window-probe.mjs`
  is written and waits for one, and it timed out twice. Reaching one needs a plugin activation
  (`activate-s-react-dev`), and the serve's own freshness report names **seven** plugins as stale
  (`cad, playbook, process, puzzle, reasoning, sequence, writer` — their Rust is newer than the staged
  build), which is also what the six console `404`s are. TS1 did not re-activate: a wasm re-activation is
  a fleet-mutex job and would have taken the machine for the rest of the session. The fix is covered by a
  law instead (`🪟️tree-windows`, green in the renderer-react corpus).
- **Not proven live: the canonical-pair change.** Showing it needs a hub that does NOT serve
  `/spaces/{}/documents/{}/active-checkpoint/pair`; every hub binary on this machine post-dates C7's
  route. It is covered by the offline-resilience law that now passes.

The serve was killed by its recorded pid when this section was written; nothing else of TS1's is running.

## 5. Honest gaps

1. **TypeScript is at 4, not 0.** Every one is named in §1.1 with the command that clears it. Two are
   stale gitignored `wasm-pack` `pkg/` artifacts (puzzle 2026-09-17, editor 2026-09-13) whose Rust
   already declares `pointerCancelScreen`; rebuilding them is two `wasm-release` builds through the
   fleet mutex, which TS1 did not take (the machine ran at 1-min load 187–267 with swap full for most of
   the window, and 22 GiB free at the low point). **These two are also a live defect, not only a type
   one**: the shell calls `session.pointerCancelScreen()` on pointer cancel, and the loaded module does
   not have it. The other two are a foreign contract (the vite config against vite's own types, which
   expands to 16 real diagnostics if it is honestly annotated) and a nominal-class test double.
2. **The tree-window fix is proven by a law, not at runtime** (§4): no windowed container exists on the
   home view, and reaching one needs a plugin re-activation TS1 did not take.
3. **The canonical-pair change is proven by a law, not at runtime** (§4): no hub on this machine lacks
   the route.
4. **`framework-os-mcp` has 5 red tests / 8 red files that TS1 did not fix** (§3), and
   `framework-os-dev`'s staging suite exposes 18 reds that had been invisible behind a load failure
   (§3.2). Both are contract drift owned by other slices; TS1 named every cause rather than papering
   over any of them.
5. **`framework-renderer-wgpu` ran zero laws** and six `framework-renderer-react` laws fail, both from a
   deleted `.ralph-tui` directory (§3.1). TS1 cannot restore it — `git restore` is forbidden here.
6. **Captures were lost once.** `🗑️generated/` was emptied at ~16:30 (disk went 13 → 112 GiB free) while
   TS1 was mid-sweep; the pre-16:30 typecheck and vitest captures are gone. Everything cited above was
   re-measured after that and re-captured, except the intermediate counts 77 → 17 → 8, which are quoted
   from this report's own earlier revision rather than from a surviving file.
7. **`@semio-tech/framework-os` re-confirmed at 17:10, after the last product edit**: 6 files,
   **363 / 363**, exit 0 (`🗑️generated/ts1-vitest-framework-os-002.txt`).

## 6. Files changed

**Product (runtime behaviour):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` — a canonical-pair the hub does not
  hand over is no longer an integrity failure (§2.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — the
  tree-window observer reads the row-extent token; three stale call sites (§1.2).
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` — `TREE_WINDOW_ROW_EXTENTS`, `TreeWindowRowExtent`
  re-export.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — those two through the barrel.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts` — the owned `fireEvent` boundary accepts
  `Element | Document | Window` (17 methods).
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx` — `UiLabel` type predicate.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — props
  widened to `PluginRegistryEntry[]` (two declarations, one import).
- `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `presenceEphemeralSnapshotWithinBoundV1` states the real handle
  signature and normalises `null`.
- `…/🧱️elements/📌️ChromePanels/🟦️.tsx` — settings tab returns `PanelTabBranch`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` — two `.catch` fallbacks typed `McpEnvelope`.
- `…/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/📦️publication/🟦️.ts` — `Bun.build` writes by `outdir`, not `write`.
- `…/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` — `worker.format` literal.
- `…/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts` and
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🟦️.ts` — `bun:sqlite` resolved at
  runtime (§3.2).

**Contracts / generated:**
- `🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json` — `NonZeroU64` defined (§3.2).
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs` — declared type count 81 → 84.
- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` — regenerated (§1.2).
- `…/🧑‍🎨engine/🧬️schema/⌨️browser-keyboard-scope/🔣️.json` + its fixture — all four modifiers required.

**Tests / stories (no law weakened, none deleted):** `🧪️space-artifact-creation-owner` (capabilities +
census), `👥️scoped-presence`, `🔬️engine-contract`, `🖱️input-contract`, `⌨️browser-keyboard-scope`,
`⚙️settings-general-layout`, `🎨️settings-theme-publication`, `🎚️window-measure-controls`,
`🎥️tutorial-bridge`, `🎥️world3d-camera-framing`, `🎨️world3d-glb-outline`, `🎨️world3d-scene-shading`,
`🚗️driver-editor`, `🧩️package-integration`, `🪟️spawned-program-session`, `🎓️HubFirstRun`, `🔎️ShellSearch`,
`📌️ChromePanels`, `🌐️wasi-activation`, `🔬️ordered-scroll`, `🪟️tree-windows`,
`♻️mit-bestand/🧺️demonstrator/…/🧪️scheduledemonstratoridle`, `✏️s/…/🏛️architect/…/🔺️diff`, and the
`🗣️Interpreter` story (rewritten against the retained contract).

## 7. Captures and probes

`🗑️generated/ts1-typecheck-final.txt` (the 4), `ts1-ui-contract-generate.txt`,
`ts1-vitest-framework-os-00{0,1,2}.txt` (3 red → 1 red → 363/363),
`ts1-vitest-{os-shell,plugin-registry,renderer-react,os-dev,os-mcp,renderer-wgpu}.txt`,
`ts1-serve-s-6199.txt`, `ts1-serve-pid.txt`, `ts1-boot-diagnose.json`.
Probes: `🐍️ts1-boot-diagnose.mjs`, `🐍️ts1-tree-window-probe.mjs`.
