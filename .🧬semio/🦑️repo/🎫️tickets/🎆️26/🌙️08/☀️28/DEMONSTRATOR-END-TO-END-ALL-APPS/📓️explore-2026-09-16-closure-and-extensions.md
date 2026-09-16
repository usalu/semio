# Demonstrator runtime closure, materialization and extension-loading audit — 2026-09-16

Read-only exploration. All paths relative to repo root `/Users/ueli/Documents/semio`. Times from `stat` (local) / `git log --date=iso` (per memory: commit-message dates are frozen/fake, `--date=iso` is the true author date).

## 1. Exact runtime component closure

**Algorithm** — `runtimeComponentClosure(components, roots)` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs` (26 lines, whole file):
- Seeds `pending` with `roots`.
- For each popped id: selects it, then pushes `component.dependsOn` (direct edge), then for each topic in `component.consumes` pushes every component whose `contributes` array names that topic (topic-based pull — the only way an `extension`/`extends` component enters a closure it isn't `dependsOn`'d by name), and **if the component itself declares a `host` metadata object, pushes every known component id** (e.g. `space`, `PLUGIN_HOST_CONFIGS` in `🤖️generated/🧩️plugins/🟦️.ts:39`) — a "pull the whole universe" edge that does **not** apply to `demonstrator` (it has no `host` field).

**Roots** — `demonstratorRuntimeComponentIds()` in `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts:10-12` calls `runtimeComponentClosure([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS], DEMONSTRATOR_RUNTIME_TARGETS.map(row => row.pluginId))`. `DEMONSTRATOR_RUNTIME_TARGETS` (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🟦️.ts:44-46`) is `PLAYGROUND_BUILD_TARGETS` filtered to the pane `variant`/`runtimeVariant` names in the demonstrator's own pane catalog (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json`: generator→generation3d, koordinator→koordinator, aggregator→aggregator, aussuchen→aussuchen, bearbeiten→bearbeiten, verfolgen→verfolgen). Verified by running the actual modules through `bun` (importing, not building — per task instructions plain-data modules are import-safe): roots resolve to **`["demonstrator" ×6, "procedural" ×1]`** — every pane row except `generator` maps to `pluginId: "demonstrator"` in `🤖️generated/🎮️playgrounds/🟦️.ts` (`generation3d` is a *separate* row, `pluginId: "procedural"`, line 54; `generator` itself is also `pluginId: "demonstrator"`, line 55 — the six panes are one shared shell crate, `app:` picks which foreign artifact kind it renders).

**Computed closure — 21 components**, confirmed by script (dedup via `Set`):

| id | role | cratePath | dependsOn | contributes/consumes edge |
|---|---|---|---|---|
| cad | plugin | `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust` | [] | direct (demonstrator dependsOn) |
| demonstrator | plugin | `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust` | cad,gis,procedural,process,puzzle,sourcing | root |
| flow | plugin | `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust` | [] | pulled by flow-extension-*'s `dependsOn:["flow"]` |
| flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text} (9) | extension | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/*/📦️packages/🦀️rust` | flow | pulled via `demonstrator.consumes:["flow.extension"]` (and `procedural.consumes` too) → `contributes:["flow.extension"]` |
| gis | plugin | `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust` | [] | direct |
| procedural | plugin | `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust` | [] | direct (also a root) |
| process | plugin | `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust` | [] | direct |
| process-extension-{concrete,metal,robotic,wood} (4) | extension | `✏️s/🔌️plugins/🏭️process/🧩️extensions/*/📦️packages/🦀️rust` | process | pulled via `demonstrator.consumes:["process.machines"]` → `contributes:["process.machines"]` |
| puzzle | plugin | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust` | [] | direct |
| sourcing | plugin | `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust` | [] | direct |

Module directory names all via `moduleDirectoryName()` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts:65-68`, backed by the hand-authored `🗺️catalog.json` (each of the 21 ids has a row there, including `{ "pluginId": "demonstrator", "directoryName": "🎪️demonstrator" }` at line 13).

**Registry source rows**: `demonstrator` row is `🤖️generated/🧩️plugins/🟦️.ts:50`; `sourcing` row line 72; `flow` row line 54; `sourcing-module-beams` row line 104; `cad-extension-aec-building` row line 81.

## 2. Materialization status of every closure component (`dist/dev/🔌️plugin-modules/`)

20 of 21 present, **`demonstrator` itself is entirely MISSING** (checked both `dist/dev/🔌️plugin-modules/` and `dist/release/🔌️plugin-modules/` — no `🎪️demonstrator` directory in either, despite the deployment catalog declaring the mapping). All other rows below are `*.core.wasm` / `🔣️.json` mtimes (local) vs. `git log -1 --date=iso -- <cratePath>`:

| component | wasm mtime | 🔣️.json mtime | source last commit (`--date=iso`) | status |
|---|---|---|---|---|
| **demonstrator** | — | — | `8add1df147` 2026-09-12 21:17:59 | **MISSING** — no plugin-module dir at all |
| cad | 2026-09-16 02:41:58 | 02:41:59 | `3250e6cb90` 2026-09-15 20:01:24 | OK (wasm newer) |
| flow | 2026-09-16 01:20:00 | 01:20:01 | `39fbe1b9bf` 2026-09-14 01:26:52 | OK |
| flow-extension-bim | 01:20:04 | 01:20:05 | `8add1df147` 2026-09-12 21:17:59 | OK |
| flow-extension-brep | 01:20:07 | 01:20:08 | same | OK |
| flow-extension-dictionary | 01:20:11 | 01:20:12 | same | OK |
| flow-extension-draw | 01:20:14 | 01:20:15 | same | OK |
| flow-extension-list | 01:20:18 | 01:20:18 | same | OK |
| flow-extension-logic | 01:20:21 | 01:20:21 | same | OK |
| flow-extension-math | 01:20:26 | 01:20:26 | same | OK |
| flow-extension-primitive | 01:20:30 | 01:20:30 | same | OK |
| flow-extension-text | 01:20:38 | 01:20:39 | same | OK |
| **gis** | 2026-09-16 **02:38:14** | 02:38:16 | `5a0b1ace28` 2026-09-16 **09:56:09** (current HEAD, per gitStatus) | **STALE** — source committed ~7h20m after the wasm was built |
| procedural | 2026-09-16 01:20:46 | 01:20:48 | `39fbe1b9bf` 2026-09-14 01:26:52 | OK |
| process | 2026-09-16 02:53:45 | 02:53:45 | `8add1df147` 2026-09-12 21:17:59 | OK |
| process-extension-concrete | 02:53:48 | 02:53:48 | same | OK |
| process-extension-metal | 02:53:50 | 02:53:51 | same | OK |
| process-extension-robotic | 02:53:53 | 02:53:54 | same | OK |
| process-extension-wood | 02:53:56 | 02:53:56 | same | OK |
| puzzle | 2026-09-16 01:08:12 | 01:08:19 | `39fbe1b9bf` 2026-09-14 01:26:52 | OK |
| sourcing | 2026-09-16 10:04:36 | 10:04:36 | `3250e6cb90` 2026-09-15 20:01:24 | OK (newest build of the set) |

`.nx-artifact.json` exists per materialized dir (e.g. `.../📐️cad/.nx-artifact.json`, 1384 bytes, mtime matching the wasm) but wasn't inspected field-by-field beyond mtime — its presence/mtime tracks the wasm build 1:1 for every present dir. `♻️hot-swap.json` at `dist/dev/🔌️plugin-modules/♻️hot-swap.json` is a single rolling record, currently `{"pluginId":"flow-extension-text","rebuiltAt":1789406081925}` — just the most recent rebuild event, not a full manifest.

**Two flags**: (a) `demonstrator` — MISSING, blocks every one of the five non-generator panes (see §3); (b) `gis` — STALE, the `verfolgen` pane is serving a wasm built before the latest `gis` source commit.

## 3. How the demonstrator learns which extensions to load per pane

There is **one shared activation**, not six. `readDemonstratorActivation()` in `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/♻️activation/🟦️.ts:7-12` calls `developmentRuntimeRoot(packageRoot, "generator", "dev", "react")` — hardcoded to the `"generator"` pane variant regardless of which pane is booting — then asserts `receipt.plugins.map(pluginId).sort().join() === demonstratorRuntimeComponentIds().join()`, i.e. the receipt must contain **exactly** the 21-component union computed in §1, or it throws `"Demonstrator activation does not contain its exact runtime union"`.

On disk, `developmentRuntimeRoot` resolves to `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/<variant>`. The actual materialized variants there are `{generation3d, fem3d, cad, process3d, puzzle3d, fem2d, sourcing}` — **there is no `generator` directory**, so `dist/runtime/react/dev/generator/activation/🔣️receipt.json` does not exist. This is the same gap as §2's missing `demonstrator` wasm: the demonstrator's own dev activation has never been produced, consistent with (and explaining) why none of the six panes currently have a shared receipt/extensions directory to read from (`extensionsDirectory: join(root, "extensions")` at line 11 would likewise not exist).

**Expected extension membership per pane**, from the §1 closure — only extensions whose host `contributes` a topic that `demonstrator`/`procedural` `consumes` actually enter the runtime union:
- **generator** (procedural): `flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text}` (procedural itself `consumes:["forms.questionKind","flow.extension"]`, `🤖️generated/🧩️plugins/🟦️.ts` procedural row).
- **bearbeiten** (process): `process-extension-{concrete,metal,robotic,wood}` (via `demonstrator.consumes:["process.machines"]`).
- **koordinator** (cad), **aussuchen** (sourcing), **aggregator** (puzzle), **verfolgen** (gis): **none** — see below.

**Finding — closure omission, not registry drift.** Source-crate `🧩️extensions/` directories match `EXTENSION_TARGETS` 1:1 (26 entries: cad 4, flow 9, imperative 5, playbook 1, process 4, sourcing 3 — verified by listing every `✏️s/🔌️plugins/*/🧩️extensions/*` dir and diffing against the registry; `flow/🧩️extensions/🧫️fixtures` is test fixtures, not a crate, correctly excluded). So there is **no registry drift**. But `cad-extension-{aec-building,aec-building-energy,aec-building-structure,spatial-shape}` (contribute `"cad.computer"`) and `sourcing-module-{beams,slabs,windows}` (contribute `"sourcing.module"`) are **never pulled into the demonstrator's runtime union**, because neither `demonstrator` nor `cad`/`sourcing` themselves `consumes` those topics anywhere in the generated registry (`cad` row: `consumes: []`; `sourcing` row: `consumes: []`; `demonstrator` row: `consumes: ["forms.questionKind","flow.extension","process.machines"]` — no `"cad.computer"`/`"sourcing.module"`). Concretely: **`koordinator` never gets the cad AEC/spatial-shape extensions, and `aussuchen` never gets the sourcing beams/slabs/windows catalog modules**, regardless of build freshness — this is a closure-graph gap, worth confirming against product intent (maybe those extensions are meant for a different host / not yet wired to the demonstrator, but as authored today they're structurally unreachable from any of the six panes).

`demonstratorRuntimeModuleLayout()` (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🟦️.ts:38-45`) is the consumer that turns the closure into `pluginModuleDirNames`/`extensionModuleDirNames` for the dev Vite config (`♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts`) and the runtime script (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts`) — both downstream of the same 21-id union, so they'd inherit both the missing-`demonstrator` and the missing-cad/sourcing-extension gaps.

## 4. Demonstrator plugin crate contents & dependency consistency

`✏️s/🔌️plugins/🎪️demonstrator/` contains: `🔣️.json` (descriptor: `composerEntries`/`ioEntries` over `s.demonstrator.playground` + the four `s.stdio.*` codecs), `🪪️manifest/🎪️demonstrator/🦀️.rs` (+ tests), `🛂️.descriptor.semio`, `🔮️oracles/🔣️.json`, `🗿️artifacts/🎪️playground/` (its own owned artifact kind + Rust + standards), `🎮️commands/` (empty), `📦️packages/🦀️rust` (the wasm crate) and `📦️packages/🟦️typescript`.

**Cargo.toml** (`📦️packages/🦀️rust/Cargo.toml`): `[package.metadata.semio] depends-on = ["cad", "gis", "procedural", "process", "puzzle", "sourcing"]`, `consumes = ["forms.questionKind", "flow.extension", "process.machines"]`. Actual path deps in `[dependencies]`: `procedural`, `cad`, `puzzle`, `sourcing`, `process`, `gis` (all `default-features = false`, deliberately excluding each pane plugin's own `plugin-entry` export to avoid duplicate-symbol link errors — six panes bundle into **one** wasm component, per the crate's own header comment). The crate deliberately does **not** depend on `semio-framework-os`/`semio-framework-3d` (documented regression guard referencing ticket `26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION`).

**Manifest** (`🪪️manifest/🎪️demonstrator/🦀️.rs`): `.depends_on("cad", …).depends_on("gis", …).depends_on("procedural", …).depends_on("process", …).depends_on("puzzle", …).depends_on("sourcing", …)` — same six, same order.

**Registry row** (`🤖️generated/🧩️plugins/🟦️.ts:50`): `dependsOn: ["cad","gis","procedural","process","puzzle","sourcing"]`, `consumes: ["forms.questionKind","flow.extension","process.machines"]`.

**Result: fully consistent, no mismatch** across Cargo.toml `depends-on`, the Rust manifest's `.depends_on(...)` calls, and the generated registry row — all three list the same six ids in the same order, matching the six pane apps (cad/koordinator, puzzle/aggregator, sourcing/aussuchen, process/bearbeiten, gis/verfolgen, procedural/generator). One extra detail: Cargo.toml also declares a 7th `[[package.metadata.semio.playground]]` row, `variant = "demonstrator"` (`app = "s.demonstrator.playground@1/*#editor"`, ports react=6107/wgpu=6207) — present in the generated `🤖️generated/🎮️playgrounds/🟦️.ts:33` too, but **not** one of the demonstrator site's six panes (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json` lists only generator/koordinator/aggregator/aussuchen/bearbeiten/verfolgen) — it's a standalone dev-harness entry for the crate's own owned artifact, not a bug.

## 5. Generated registry freshness

`🤖️generated/🧩️plugins/🟦️.ts` and `🤖️generated/🎮️playgrounds/🟦️.ts` both have mtime **2026-09-16 10:07** (`ls -la`), which is *after* the newest source commit touching any closure crate (`gis`, `5a0b1ace28`, 2026-09-16 09:56:09) — **the generated registry itself is fresh relative to source**, it is not the cause of the §2/§3 gaps.

Regeneration command: `ensurePluginRegistry()` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔄️refresh/🟦️.ts:106-110`) runs `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate` (the `ScriptRouter` in `📇️registry/📜️script.ts:9` registers `"generate" → GenerateScript`, the default command), then syncs built descriptors via `syncBuiltPluginDescriptors(...)`.

## Summary of flags

1. **MISSING** — `🎪️demonstrator` plugin module is not materialized in `dist/dev/🔌️plugin-modules/` (nor `dist/release/…`) despite a valid deployment-catalog directory mapping; the dev activation root `dist/runtime/react/dev/generator/` (and its `activation/🔣️receipt.json`) likewise doesn't exist. This blocks all six demonstrator panes, since every pane's dev activation is validated against the one shared `"generator"`-keyed receipt.
2. **STALE** — `gis` wasm (built 2026-09-16 02:38) predates its own latest source commit (2026-09-16 09:56, current HEAD `5a0b1ace28`) by ~7h20m; the `verfolgen` pane is serving an outdated gis build.
3. **Closure omission** (not registry drift) — `cad-extension-*` and `sourcing-module-{beams,slabs,windows}` are correctly registered in `EXTENSION_TARGETS` (no source/registry mismatch) but are structurally unreachable from the demonstrator's runtime union because nothing in the graph consumes their `"cad.computer"`/`"sourcing.module"` contributed topics — `koordinator` and `aussuchen` never receive their respective extensions.
4. Everything else checked out clean: registry is fresh vs. source; demonstrator's `dependsOn`/Cargo deps/manifest `.depends_on(...)` are all mutually consistent; 20/21 closure components are present and non-stale.
