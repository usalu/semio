# W7 — Runtime plugin dependencies become a declared, schema-first concept

Implementer W7, 2026-09-07. Scope: stop deriving the plugin registry's `dependsOn` (the browser's
runtime load graph and the dev session's build closure) from Cargo `[dependencies]`; make it an
explicit declaration. TypeScript/JSON/Cargo-metadata only — no cargo build was run in this lane.

---

## 1. What a `dependsOn` edge actually does at runtime

Grepped `dependsOn` / `dependencies` / `dependency-missing` / `blockedDependency` across `🧰️framework`.
Three live consumers, all reading the SAME registry column, plus one unrelated surface that does not:

| Consumer | File | Effect of one `dependsOn` edge |
|---|---|---|
| Browser load set | `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:301-322` `expandPluginRegistry` | The primary's transitive `dependencies` closure is added to the set of plugin actors the session instantiates. `consumes`/`contributes` alone never pulls these, per the function's own comment. A pulled entry whose served `🔣️.json` 404s fails `fetchDescriptorManifest` (`:107-125`, `plugin.descriptor-unavailable`). |
| Load order + graph faults | `🎠️kernel/🟦️.ts:3110-3126` `validatePluginDependencyGraph`, `:3252` `orderPluginRegistryEntries`, called from `resolvePlaygroundBoot` `:2981` | A dependency id with **no node in the graph** yields `transaction.dependency-missing` and the DEPENDENT entry is dropped from the boot list entirely (`orderPluginRegistryEntries` retries on the non-blocked subset). Rendered as *"Plugin X needs Y, which is not installed."* (`:3281`). |
| Dev build closure | `📇️registry/📜️script.ts:632` `resolveRegistryPluginIdsForFilter` | Decides which crates a filtered `dev <plugin>` session builds at all. |
| (NOT this column) surface permission | `🎠️kernel/🟦️.ts:600-640` `AppRouter.build` | Reads `manifest.dependencies` off the **loaded descriptor**, i.e. the Rust builder's `.depends_on(...)`, not the registry row. A plugin registering a surface over an artifact kind another plugin owns must name that owner, or the whole plugin is excluded with `surface.contribution-not-permitted`. Untouched by this packet. |

Note on the report the coordinator handed me: `loadPluginModulesInDependencyOrder` /
`blockedDependency` **no longer exist anywhere in `🧰️framework`** (verified by grep today — the only
surviving mention is a prose reference in `🏛️ShellHost/🟦️.tsx:3088`). The shell now installs plugins
independently through a bounded worker pool (`🏛️ShellHost/🟦️.tsx:3074-3110`), and only the PRIMARY
failing is fatal. So the "cascade-skip" mechanism described in
`📓️explore-stdio-descriptor-from-existing-module.md` §3 is gone; what remains from a bogus `dependsOn`
edge is (a) a guaranteed `plugin.descriptor-unavailable` console failure for a plugin nobody needs,
(b) a dropped-from-boot dependent whenever the id does not resolve to a node at all, and (c) a dev
session that builds crates it never needed.

**So the genuine test is: does the dependency's own ACTOR have to be loaded beside this plugin?** That
is true for exactly three shapes — an extension's `extends` host; a plugin registering surfaces over
another plugin's artifact kinds (AppRouter/`register_contributions` gate); an `ArtifactContribution`
onto another plugin's artifact. It is false for a Rust rlib link, however deep.

### Per-plugin verdict on every pre-existing Cargo-derived edge

Evidence per row: the depending crate's own cross-crate `use` statements and call sites
(`grep -rhoE "use (cad|gis|…|semio_s_plugin_[a-z_]+)::"` over each plugin root), plus
`ArtifactContribution::builder` (repo-wide: exactly ONE file, `📐️cad/🧩️extensions/🏢️aec-building/🦀️.rs`).

| Edge (before) | Plugins | Evidence | Verdict |
|---|---|---|---|
| `→ stdio` | animate, architect, block, cad, dag, demonstrator, draw, energy, fem, flow, flow-extension-brep, forms, gis, imperative, layout, lowpoly, mathematical, norm, note, playbook, procedural, process, puzzle, **raster**, reasoning, remodel, sequence, shooting, sourcing, space, trinity, vcs, writer (33 crates) | Every single cross-crate reference is `semio_s_plugin_stdio::artifacts::<format>::…` — snapshot structs, `encode_*`/`decode_*`/`parse_*`/`write_*` free functions, `STDIO_*_DOCUMENT_SCHEMA` constants. No App/editor/viewer type, no `ArtifactContribution`, no cross-actor message, no topic consumption of an `s.stdio.*` publication. Matches `📓️w3-raster-io.md` §1 for raster specifically ("composite → stdio's `encode_png`", all in-process). | **LIBRARY LINK — dropped** |
| `→ cad` | lowpoly | Single reference, `use cad_plugin::artifacts::cad::io::geometry_import::{objects_from_fixture_model, parse_geometry}` inside a schema test module. No cad surface, no contribution. | **LIBRARY LINK — dropped** |
| `→ fem` | norm | `use fem::core::{Dof, MemberUdl, Model, Node, Support}`, `fem::core::elements2d::BeamEb2` — solver value types, behind an `optional = true` Cargo feature. | **LIBRARY LINK — dropped** |
| `→ trinity` | writer | `use trinity::core::{example_graph, lint}`, `use trinity::lexer::{lex_spanned, SpannedToken, Token}`. | **LIBRARY LINK — dropped** |
| `→ imperative-control`, `→ imperative-effect`, `→ imperative-math`, `→ imperative-text` | sequence | The crates are `semio-s-plugin-imperative-*`; their PLUGIN ids are `imperative-extension-*`. The four derived ids **matched no registry node**, so `validatePluginDependencyGraph` would raise four `transaction.dependency-missing` and drop `sequence` itself from its own boot. Code use is `imperative_engine::{…}` node-evaluation types. | **PHANTOM + LIBRARY LINK — dropped** |
| `→ draw-fsm` | draw | `semio-s-plugin-draw-fsm` is a command-FSM sub-crate that lives INSIDE draw's own artifact tree (`🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm`). It is not a plugin and has no registry node — same drop-from-boot fault as `sequence`. | **PHANTOM — dropped** |
| `→ cad, gis, procedural, process, puzzle, sourcing` | demonstrator | `use cad::editor::cad::{create_cad_app, CadPlayApp}`, `gis::editor::gis2d`, `procedural::editor::generation3d`, `process::editor/viewer::process3d`, `puzzle::editor::puzzle3d`, `sourcing::editor/viewer::sourcing` — ten surfaces registered over SIX foreign plugins' artifact kinds. `AppRouter.build` excludes the whole plugin (`surface.contribution-not-permitted`) unless each owner is a declared dependency; demonstrator's own `assert_surface_dependencies_declared` test enforces it. | **GENUINE RUNTIME — kept, now declared** |
| `→ <host>` | 26 extensions (cad×4, flow×9, imperative×5, playbook×1, process×4, sourcing×3) | Contract freeze §4 rule 1: an extension's host IS `dependsOn[0]`. Already explicit via `[package.metadata.semio].extends`, never Cargo-derived. | **GENUINE RUNTIME — kept (via `extends`)** |

Two pre-existing bugs the blind regex caused, both fixed as a side effect:

1. **Phantom ids in the shipped registry.** `generatePluginRegistry` used the raw regex ids, while
   `auditPluginCatalogSources` re-derived a *different*, package-name-canonicalized set. The shipped
   `🔌️plugins.json` therefore carried `sequence → ["imperative-control", …]` and
   `draw → ["draw-fsm", …]` — ids no node ever provided — while the audit carried
   `["imperative-extension-control", …]`. Two sources of truth, one of them boot-breaking.
2. **`stdio` unconditionally in every session's load set.** 33 of 59 crates named it, so every single-
   plugin dev boot tried to instantiate an actor whose descriptor has never existed in this repo's
   git history.

---

## 2. Where the declaration lives (decision)

Three candidate homes were examined:

- **`Plugin::builder(...).depends_on(id, VersionReq)`** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33627`
  (plus `ExtensionBundle::depends_on`, `:33627`-adjacent, with the `extends == dependencies[0]` panic at
  `:33636-33645`). Already real, already schema-first: it fills Rust `PluginManifest.dependencies`
  (`🛂️manifest/🦀️.rs:4247`), which travels on the wire and reaches the descriptor as
  `manifest.dependencies`. Verified on disk: of the 22 owner-root `🔣️.json` descriptors, **only
  demonstrator carries a `manifest.dependencies` array**; every other plugin's manifest has no such key,
  and only two source files in `✏️s` call `.depends_on` at all.
- **Descriptor `🔣️.json` → registry** — rejected as the generator's source. The descriptor is a BUILD
  OUTPUT: producing one requires `cargo rustc --target wasm32-wasip2` + the native `describe` emitter.
  The registry generator runs *before* any build on every dev boot (`ensurePluginRegistry` is a hard
  gate ahead of `buildPluginsStreaming`), so making the pre-build load graph depend on a post-build
  artifact is a chicken-and-egg: 19 of 59 crates have no descriptor pair at all today (including
  `stdio` itself), and the rest are stale. `parsePluginCargo` already documents exactly this reasoning
  for `consumes` — *"`consumes` is ALWAYS read from Cargo metadata regardless"*.
- **`[package.metadata.semio] depends-on = [...]`** — **chosen.** It sits beside `role`, `extends`,
  `consumes` and `contributes`, which the generator already reads from the same block; it needs no
  build; and it is the only declaration the registry can honour on a cold tree.

To keep this from becoming two disagreeing sources of truth, the repo-policy gate
`policyPluginDependencyParityBreaches` (root `📜️script.ts:28332`) was **re-pointed**: it now compares
the builder's `.depends_on(...)` against `[package.metadata.semio].depends-on` (+ `extends`), in both
directions, instead of against Cargo `[dependencies]`. The old gate encoded the exact false coupling
this packet removes — it demanded a `.depends_on` for **every** sibling crate link, i.e. it would have
pushed `stdio` straight back onto all 33 crates.

`PluginCatalogTarget`/`PluginBuildTarget` needed no shape change: `dependsOn` stays
`readonly string[]`; only its meaning, its source and its docs changed. The `catalog-complete`
language-agnostic schema (`🧪️tests/🧬️catalog-complete/🧬️schema/🔣️.json`) is already source-neutral
(`dependsOn` = array of plugin-id strings) and was left untouched.

---

## 3. Changes

| File | Line(s) | Change |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` | 50-58 | `PluginRegistryEntry.dependsOn` doc: declared runtime actor dependencies, explicitly not Cargo links. |
| ″ | 286-288 | `parsePluginCargo` now calls `parseSemioDependsOnIds(semioText, pluginId, manifestPath)`; `extends` still prepended (contract freeze §4 rule 1). |
| ″ | 401-425 | **`parseCargoPluginDependencyIds` deleted**, replaced by `parseSemioDependsOnIds` (reads `depends-on`, validates each id against `CATALOG_ID`, rejects self-reference and duplicates). No fallback, no compat path. |
| ″ | (was 420-427) | **`parseCargoPluginDependencyPackageNames` deleted** — its only caller was the audit's canonicalization. |
| ″ | 2634-2661 | `auditPluginCatalogSources`: `pluginByPackage`/`manifestByPlugin` maps removed. |
| ″ | 2684-2701 | Canonicalization replaced by **validation**: a declared id that no discovered crate provides is a `dependency-invalid` issue instead of being silently dropped; `orderCatalogNodes` now runs on the entries themselves. |
| ″ | 619-631 | `resolveRegistryPluginIdsForFilter` doc: a merely-linked crate is not in the session closure. |
| ″ | 752-756 | Emitted `PluginBuildTarget.dependsOn` doc in `🧩️plugins.ts`. |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | 307-312 | `expandPluginRegistry` comment rewritten to the new semantics (declared edges, never Cargo links). |
| ″ | 380-385 | `dependsOnToPluginDependencies` doc: no longer "build-time Cargo ground truth"/"ahead of the `.depends_on` rollout". |
| ″ | 1145-1155 | `PluginCatalogTarget.dependsOn` doc. |
| `📜️script.ts` (root policy) | 28313-28327 | **`policyCargoPluginDependencyIds` deleted**, replaced by `policySemioMetadataDependsOnIds` (reads `depends-on` + `extends`). |
| ″ | 28329-28377 | `policyPluginDependencyParityBreaches` re-pointed to builder ↔ Cargo-metadata parity; new breach id `plugin-dependency-undeclared-metadata-*`; both directions now `high` (the "held at medium while the API rolls out" carve-out is gone — there is nothing left to roll out). |
| `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` | 24-32 | `depends-on = ["cad", "gis", "procedural", "process", "puzzle", "sourcing"]` added to `[package.metadata.semio]`, with a comment recording why `stdio` is absent. **The only plugin in the repo with a genuine runtime need.** |
| `✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs` | 57-58 | `.depends_on("stdio", VersionReq::Any)` removed (its stdio use is `artifacts::csv/json/xlsx/zip` codecs only). |
| ″ | 120 | `bundle_keeps_its_plugin_identity` expectation updated to the six genuine dependencies. |
| `📇️registry/✅️catalog-complete.test.ts` | 426-441 | Cargo-derivation expectation replaced: `sequence`/`raster` → `[]`, `demonstrator` → the six, `cad-extension-aec-building` → `["cad"]`; explicit `120_000` timeout (see §4). |
| `📚️library/📦️packages/🟦️typescript/🔬️index.test.ts` | 4056-4115 | Parity tests updated to metadata semantics + one new test asserting a sibling Cargo link produces **no** breach while an undeclared-in-builder metadata entry does. |
| `.🧬semio/…/RASTER-PLUGIN-END-TO-END/🟦️w7-depends-on-probe.ts` | new | Ticket-local probe printing each filter's declared closure (kept as an input file). |
| `🤖️generated/🔌️plugins.json`, `🤖️generated/🧩️plugins.ts`, `.vscode/launch.json` | regenerated | See §4. |

---

## 4. Regenerated registry + verification

### `🤖️generated/🔌️plugins.json` diff (59 rows, ids unchanged, only `dependsOn` moved)

Baseline = the state dumped at the start of this session (before the generator change).

| Row(s) | Before | After |
|---|---|---|
| **raster** | `["stdio"]` | `[]` |
| animate, architect, block, cad, dag, energy, fem, flow, forms, gis, imperative, layout, mathematical, note, playbook, procedural, process, puzzle, reasoning, remodel, shooting, sourcing, space, trinity, vcs (25 rows) | `["stdio"]` | `[]` |
| lowpoly | `["cad", "stdio"]` | `[]` |
| norm | `["fem", "stdio"]` | `[]` |
| writer | `["stdio", "trinity"]` | `[]` |
| draw | `["draw-fsm", "stdio"]` | `[]` |
| sequence | `["imperative-control", "imperative-effect", "imperative-math", "imperative-text", "stdio"]` | `[]` |
| flow-extension-brep | `["flow", "stdio"]` | `["flow"]` |
| demonstrator | `["cad", "gis", "procedural", "process", "puzzle", "sourcing", "stdio"]` | `["cad", "gis", "procedural", "process", "puzzle", "sourcing"]` |
| 25 other extensions (cad×4, flow×8 remaining, imperative×5, playbook×1, process×4, sourcing×3) | `[<host>]` | `[<host>]` — unchanged |
| stdio | `[]` | `[]` — unchanged |

Rows carrying any runtime dependency: **27 / 59** (26 extension→host edges + demonstrator).
Rows naming `stdio`: **33 → 0**. No non-`dependsOn` field on any row changed.

### Commands (all foreground, exit codes shown verbatim)

```
$ bun ./📜️script.ts generate            # 📇️registry
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages) -> …/🤖️generated
.vscode/launch.json regenerated -> /Users/ueli/Documents/semio/.vscode/launch.json

$ bun ./📜️script.ts check-generated
plugin registry generated catalog and launch bytes are fresh.
EXIT=0

$ bun ./📜️script.ts check
… (taxonomy-tree warnings for 🪐️space / 🪵️sourcing only — pre-existing, unrelated to dependencies;
    zero lines mentioning dependsOn / depends-on / dependency)
[exited with code 0]
```

### Declared-closure probe (`🟦️w7-depends-on-probe.ts`) — verbatim

```
raster: dependsOn=[] sessionClosure=["raster"]
demonstrator: dependsOn=["cad","gis","procedural","process","puzzle","sourcing"] sessionClosure=["cad","demonstrator","flow","flow-extension-bim","flow-extension-brep","flow-extension-dictionary","flow-extension-draw","flow-extension-list","flow-extension-logic","flow-extension-math","flow-extension-primitive","flow-extension-text","gis","procedural","process","process-extension-concrete","process-extension-metal","process-extension-robotic","process-extension-wood","puzzle","sourcing"]
sequence: dependsOn=[] sessionClosure=["sequence"]
draw: dependsOn=[] sessionClosure=["draw"]
lowpoly: dependsOn=[] sessionClosure=["lowpoly"]
block: dependsOn=[] sessionClosure=["block"]
writer: dependsOn=[] sessionClosure=["writer"]
norm: dependsOn=[] sessionClosure=["norm"]
rows with a declared runtime dependency: 27/59
rows still naming stdio: []
```

A `dev raster` session's plugin set is now exactly `["raster"]` — `stdio` is neither built nor
instantiated for it.

### Unit tests

Registry suite (`bun x vitest run --config 🧪️tests/🟦️.ts`, from `📇️registry`):

```
 ❯ |@semio-tech/plugin-registry| 🚀️launch.test.ts (5 tests | 4 failed) 152ms
 Test Files  1 failed | 3 passed (4)
      Tests  4 failed | 19 passed (23)
```

`✅️catalog-complete.test.ts` (11), `📖️generated-projection.test.ts`, `🪪️plugin-identity.test.ts` all
pass. The 4 `🚀️launch.test.ts` failures are **pre-existing and unrelated** — verbatim causes:
`ReferenceError: Bun is not defined` (×2, the test calls `Bun.*` under vitest/node);
`expected { inherits: 'dev', … } to deeply equal { inherits: 'dev', 'codegen-units': 1 }` with the
extra `package: { semio-s-plugin-lowpoly: { opt-level: 2 }, semio-s-plugin-puzzle: { opt-level: 2 } }`
(root `Cargo.toml` profile overrides added by the lowpoly/puzzle tickets); and a describe-script text
drift. None touch `dependsOn`, and I edited none of those files.

`✅️catalog-complete.test.ts` alone, after the change:

```
 Test Files  1 passed (1)
      Tests  11 passed (11)
   Duration  19.30s
```

The one assertion that first failed did so as `Error: Test timed out in 5000ms` (the audit pack-verifies
40 descriptor pairs and takes ~13 s on this loaded box); it passes in full at `--testTimeout=120000`, so
an explicit `120_000` was added to that `it(...)`, matching the `60_000` style used elsewhere in the repo.

Policy parity tests (`bun test 🔬️index.test.ts -t "policyPluginDependencyParityBreaches"`):

```
bun test v1.3.14 (0d9b296a)

 3 pass
 580 filtered out
 0 fail
 8 expect() calls
Ran 3 tests across 1 file. [4.44s]
```

Parity gate against the real tree (the `⚖️gate🔗️plugin-dependency-parity` launcher's own command):

```
$ bun -e 'const m = await import("./📜️script.ts"); const b = m.policyPluginDependencyParityBreaches(process.cwd()); …'
breaches: 0
```

Replaying the OLD gate logic over today's tree for comparison: `missing-cargo(high)=0
undeclared-runtime(medium)=48 total=48`. Those 48 breaches were the institutionalised false coupling
(one per sibling-crate link lacking a `.depends_on`); the new gate reports **0** because the two
declarations now agree everywhere.

Type check of the two edited TS files (`tsc --noEmit --strict`, ad-hoc flags so pre-existing
repo-wide errors appear): the error line set for `📇️registry/📜️script.ts` is
`135 136 559 2922-3187` — all pre-existing, **none at any line this packet touched** (50-58, 286-288,
401-425, 619-631, 752-756, 2684-2701). The root `📜️script.ts` reports **zero** errors in the
`283xx` range that contains the rewritten gate.

---

## 5. What is NOT verified

- **No shell boot.** This lane is TypeScript/JSON/Cargo-metadata only; the coordinator owns raster's
  cargo test. The claim "raster now boots without `stdio`" is verified only to the level of "the
  registry no longer places `stdio` in raster's load set or build closure" (probe above) — the
  remaining boot behaviour was not observed live.
- **No cargo build / no Rust compile.** The demonstrator edits (`.depends_on("stdio")` removal and its
  `bundle_keeps_its_plugin_identity` expectation) are **not compiled or tested** here. They are
  source-consistent by inspection but must be picked up by demonstrator's own cargo test run.
- **Descriptors remain stale.** Demonstrator's owner-root `🔣️.json` still carries
  `manifest.dependencies` including `stdio`; it will only lose it when demonstrator is rebuilt and
  re-`describe`d. The registry no longer reads that field, so this does not affect boot, but
  `AppRouter`'s surface-permission gate reads the loaded manifest and will keep seeing the stale
  `stdio` entry (harmless — an extra declared dependency never causes a fault there, only a missing
  one does).
- **`🔬️index.test.ts` as a whole is red on this tree** — `494 pass / 89 fail` of 583, 384 s. I verified
  the `policyPluginDependencyParityBreaches` block is 3/3 green in isolation, and spot-checked every
  failure whose name matches `depend|parity|registry|catalog`: all five are missing fixtures or a
  missing env var (`ENOENT … 🔣️ui-host-package.json`, `SEMIO_TEST_ARTIFACT_DIR is required for cargo
  provider binding artifacts`, `Provider manifest locator must name Cargo.toml`) plus a layering
  baseline overflow — none is a `dependsOn`/parity assertion. I did not attribute the remaining ~84
  (mutation-metadata / source-provider / layering areas, concurrent refactors).
- **`stdio`'s own problem is untouched.** It still has no descriptor anywhere in git history and its
  standalone component still cannot be linked (wasmparser's 1,000,000-function ceiling). This packet
  only stops 33 innocent plugins from being coupled to that blocker; it does not fix it. Booting
  `stdio` itself, or any plugin that genuinely needs stdio's actor (none today), still fails.
- **Extensions still do not call `.depends_on(<host>)`** in Rust (only `cad-extension-aec-building`
  does). That is legal under the new gate because `extends` declares the host on the metadata side,
  and `ExtensionBundle::extends` asserts `extends == dependencies[0]` at build time — but it means an
  extension's descriptor `manifest.dependencies` is empty where the registry row says `[<host>]`. Not
  changed here; flagged as the next consistency step if someone wants the two to be literally equal.

## Files referenced

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-complete.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts`
- `📜️script.ts` (root policy gates)
- `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs`
