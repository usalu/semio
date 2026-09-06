# Lane M — plugin-identity conformance: `reasoning` and `imperative-extension-effect`

Ticket `26/09/05/S-END-TO-END` · Opus 5 · lane M `plugin-identity-conformance`. Spec:
`📓️explore-plugin-identity-conformance.md` (2 FAIL rows), recipe: `📓️opus-space-plugin-identity.md`
(lane K), decision: `📋️plan.md` 21:55 — **the plugin id follows the artifact-kind grammar
`s.<plugin>.<artifact>`**.

## Headline

1. **`reasoning-mindmap` → `reasoning`.** The grammar decides the direction: the artifact kinds are
   already `s.reasoning.wires*` and the root already said `builder("reasoning")` /
   `.package_id("semio:reasoning")`, so the identity is **`reasoning`** and every *other* authority was
   the drifted one. The Cargo component package, the deployment catalog row, the dev cache directory,
   the taxonomy path patterns/registry keys, the framework's descriptor-migrated roster, the storybook
   story and all generated projections now say `reasoning`. **Zero artifact-kind literals changed** —
   the explorer's ranked fix #2 (rename the 10+ `s.reasoning.wires*` ids to `reasoning-mindmap`) is the
   *opposite* of the 21:55 decision and was deliberately not done.
2. **`imperative-extension-effect`.** `EXTENSION_ID` and the bundle label are fixed — **and two live
   call sites the explorer census missed** were fixed with them: both
   `register_native_imperative_module("imperative-extension-core", …)` registrations. That map is keyed
   by the contribution entry's `plugin_id`, which *is* `EXTENSION_ID`
   (`✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️.rs:96-98` → `ProgramContributionEntry { plugin_id: extension_id.into(), … }`,
   consumed at `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️.rs:125` `registrars.get(&entry.plugin_id)`).
   Fixing only the constant would have silently demoted the native effect module to a
   `ContributedExtensionStub` — every `log-print`/`state-set`/`state-increment` operator failing with
   `EvalError::PendingExtension` in the sequence editor and the imperative procedure host.
3. **Two new tests.** A `reasoning` Rust identity law driven by a language-agnostic fixture (lane K's
   shape), plus a **repo-wide** TypeScript law that runs the same join for **all 59 rows** — the
   extension requirement of task 2 is met by that one gate rather than 26 copies, and it also pins every
   hand-authored identity fixture (space's and reasoning's) against its own JSON-Schema with ajv 2020.
4. **Task 3 (cosmetic drift): confirmed unreachable by any gate — follow-ups only**, with the real site
   counts (larger than the census said: remodel has ~20 sites, not 4).

## 1. `reasoning` — file:line changes

### The identity authorities

| # | File:line | Before → after |
|---|---|---|
| 1 | `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml:14` | `package = "semio:reasoning-mindmap"` → `package = "semio:reasoning"` — the ROOT authority: `parsePluginCargo` derives `pluginId = packageId.slice("semio:".length)` (`📇️registry/📜️script.ts:269-270`), i.e. **from the component package, never from the crate name** (verified for task 1's explicit question) |
| 2 | `…/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json:50` | `{ "pluginId": "reasoning-mindmap", "directoryName": "💡️reasoning-mindmap" }` → `{ "pluginId": "reasoning", "directoryName": "💡️reasoning" }`. The row's sorted position is unchanged (`raster` < `reasoning` < `remodel`), so `✅️catalog-complete.test.ts:78-82`'s order-equality against the generated registry still holds without moving it — unlike lane K's `s`→`space`. The `💡️` emoji is preserved (`parseModuleDirectories` rejects duplicate sibling emoji) |
| 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32902` | `DESCRIPTOR_MIGRATED_PLUGINS` roster `"reasoning-mindmap"` → `"reasoning"` — the descriptor-staleness assertion inside `plugin_exports!` keys on the plugin id |
| 4 | `.storybook/stories/framework/os/plugins.stories.tsx:52` and `:97` | `export const ReasoningMindmap … plugin: "reasoning-mindmap"` → `export const Reasoning … plugin: "reasoning"`, sorted position unchanged, **and** the `EXPORTED_STORIES` barrel that `assertPluginMatrixCoverage` reads. **That barrel was already broken at HEAD by lane K**: it still listed `S`, a binding lane K had renamed to `Space`, so the module referenced an undefined identifier — fixed here too (`S` → `Space`, moved to its sorted slot after `Sourcing`). Re-verified mechanically: 33 exports, 33 barrel names, zero one-sided entries, `toPascalCase(args.plugin) === exportName` for all 33, barrel sorted |
| 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/💡️reasoning-mindmap` → `💡️reasoning` | `mv` (not delete/rebuild): the core wasm, component JS/d.ts, `interfaces/`, `🌉️bridge.js`, `🔣️.json` and `🛂️.descriptor.semio` all survive under the new name, which is what `moduleDirectoryName("reasoning")` now resolves to. `git mv` refused (the directory's contents are gitignored build outputs), plain `mv` used |

### Taxonomy (schema authority) — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`

Three token substitutions, re-parsed with `json.loads` after writing; **16 occurrences total**:

- `💡️reasoning-mindmap` → `💡️reasoning` (6): the `deployed-module-members` member name (line 5441,
  sorted position unchanged) and the five `🔌️plugin-modules/💡️reasoning-mindmap/…` path patterns.
- `dev-plugin-component-reasoning-mindmap-{js,declaration,wasm}` → `dev-plugin-component-reasoning-…`
  (5: 2 + 2 + 1 — the `-wasm` key has no fixed-contract-map row, unlike its two siblings).
- `dev-plugin-interfaces-reasoning-mindmap` → `dev-plugin-interfaces-reasoning` (5: its own row plus the
  four `dev-jco-{all,plugin,contributor,wall-clock}-interfaces` group lists; every list stays sorted
  because `reasoning` occupies the same slot between `raster` and `remodel`).

The **emitted filenames inside those patterns are untouched** — `semio_s_plugin_reasoning_mindmap_component.{js,d.ts,core.wasm}`
derive from the Cargo **crate name**, which is not an identity (§3).

### Deliberately NOT changed, with the reason

- **Crate name `semio-s-plugin-reasoning-mindmap`** and the wasm out `semio_s_plugin_reasoning_mindmap.wasm`.
  Verified against the generator: `parsePluginCargo` reads `name` only for `packageName`/`wasmOut`
  (`📜️script.ts:267,273`) and derives `pluginId` purely from the component package — so the crate name is
  a third, independent name. Kept, and **pinned** by the new fixture so nobody "fixes" it later.
- **Playground variant `reasoning-wires`** (+ alias `wires`, ports 6015/6115). Names the shell variant,
  not the plugin; the generated map now reads `("reasoning-wires", "reasoning")` and `("wires", "reasoning")`.
- **Nx project name `@semio-tech/reasoning-mindmap-plugin`** (`📋️project.json:2`) and the router docstring
  in `📦️packages/🦀️rust/📜️script.ts` — these track the **crate** name, which is unchanged.
- **`✏️s/🔌️plugins/💡️reasoning/🔣️.json:5` (`"pluginId": "reasoning-mindmap"`) and its packed twin
  `🛂️.descriptor.semio`.** Build outputs of `describeBuiltPlugin`, hash-bound to each other; hand-editing
  desyncs the pack hash. Left exactly as lane K left space's — the next rebuild re-emits them. Until then
  `plugin-registry:check` will report this row's descriptor identity as divergent: **intended, visible
  drift, not a regression.** (The same file also still carries the stale
  `activationEvents: ["on-artifact-kind:graph.wires"]`, which the live source has said `s.reasoning.wires`
  for some time — an independent pre-existing staleness the same rebuild clears.)
- **`🧶️bundles/🚀️bootstrap-B__lgXdC.js` and `🖥️runtime-Dc0wwVhF.js`** — committed, content-hash-named
  production Vite build outputs that embed the old catalog row. They need a full `framework-os-dev:build`
  (wave 3 / coordinator), exactly as lane K recorded for `🪐️s`.
- **`🗣️dsl/🧹️fixture-sweep`'s `Cargo.toml:43` and `🧫️fixture/🔣️.json:210`** — both name the crate
  (`package = "semio-s-plugin-reasoning-mindmap"`), not the identity.

### Regenerated (not hand-edited)

`bun ./📜️script.ts generate` in `🔌️plugin/📇️registry` → `🤖️generated/{🔌️plugins.json,🧩️plugins.ts,🖥️hosts.rs,🗿️artifacts.rs,🎠️playgrounds.json,🎮️playgrounds.ts,…}`
and `.vscode/launch.json`; then `generate-frame-worker` for the wgpu bundle that embeds the deployment
catalog. (`🤖️generated/` is **gitignored** — it shows no diff on purpose.)

## 2. `imperative-extension-effect` — file:line changes

| # | File:line | Before → after |
|---|---|---|
| 1 | `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🦀️.rs:116` | `const EXTENSION_ID: &str = "imperative-extension-core"` → `"imperative-extension-effect"` (matches Cargo `semio:imperative-extension-effect`, `…/📦️packages/🦀️rust/Cargo.toml:11`, and the registry row) |
| 2 | `…/📣️effect/🦀️.rs:137` | `ExtensionBundle::new(EXTENSION_ID, "Imperative Core", …)` → `"Imperative Effect"` |
| 3 | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:357` | `register_native_imperative_module("imperative-extension-core", semio_s_plugin_imperative_effect::register)` → `"imperative-extension-effect"` — **missed by the census**, see below |
| 4 | `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:40` | same rename, the in-plugin registration |
| 5 | `…/📣️effect/📦️packages/🦀️rust/📋️project.json:2` | nx project `@semio-tech/imperative-extension-core-rust` → `…-effect-rust`, matching all four siblings (`control`/`logic`/`math`/`text`). No other file references the old project name |

**Why #3/#4 were mandatory, not cosmetic.** The census said the constant had "zero other files
referencing it outside the file". That is true of the *constant*, but not of its *value*: the native
registrar map is keyed by the string. `imperative_module_contribution(EXTENSION_ID, …)` stamps
`ProgramContributionEntry.plugin_id = EXTENSION_ID` (`🧩️extension_sdk/🦀️.rs:96-98`), and
`compose_registry` looks the registrar up with `registrars.get(&entry.plugin_id)`
(`📇️registry/🦀️.rs:114-126`) — falling back to `ContributedExtensionStub`, whose `evaluate` returns
`EvalError::PendingExtension`, when the key misses. Renaming the constant alone would have turned every
effect operator into a runtime pending-extension fault in both hosts, with no compile error.

**Deliberately not changed**: the *module* slug `"core"` still passed as `module_id`/`manifest_id`
(`📣️effect/🦀️.rs:123,131` and its own test at `:162`). That is a within-`imperative` module namespace, a
different axis from the plugin identity, self-referenced only; listed as a follow-up (§5).

## 3. Tests added

### `reasoning` — language-agnostic fixture + Rust law

- **New** `✏️s/🔌️plugins/💡️reasoning/🧪️fixtures/🧫️plugin-identity/🔣️.json` and its pinned
  `🧬️.schema.json` (2020-12, `additionalProperties:false`, a pattern per id shape):

  ```json
  { "schema": "semio.reasoning.plugin-identity/v1", "pluginId": "reasoning",
    "packageId": "semio:reasoning", "packageName": "semio-s-plugin-reasoning-mindmap",
    "moduleDirectoryName": "💡️reasoning", "artifactKindPrefix": "s.reasoning.",
    "playgroundVariant": "reasoning-wires" }
  ```

  `packageName` and `playgroundVariant` are in the tuple **on purpose**: this is the one place that
  states, in data, that the crate name and the launch variant are two *other* names — the exact
  conflation that produced this bug.
- `✏️s/🔌️plugins/💡️reasoning/🦀️.rs:53-130` — new `#[cfg(test)] mod identity_tests` with
  `plugin_identity_is_the_same_in_every_authority`, joining five authorities in one law: the assembled
  manifest's `plugin_id` (covering `builder(…)` **and**, through `try_build`'s own `semio:<plugin_id>`
  rule, `.package_id(…)`), the Cargo `[package.metadata.component] package` + `[package] name` + the
  playground `variant` (asserted to be the *other* name), the canonical grammar (`artifactKindPrefix ==
  "s.<pluginId>."` and every app id starts with it — the predicate `preflight_artifact_identity` derives
  via `ArtifactKindId::plugin()`), the deployment catalog row (id + `directoryName`), and the generated
  registry row (`packageId` + `packageName`). No second literal of the identity exists in the crate.

### Repo-wide — **new** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🪪️plugin-identity.test.ts`

Three laws, run by the registry's registered `test` target (vitest picks the file up automatically —
no project.json or launch-seed change needed):

1. **All 59 crates, plugins and extensions alike.** For each generated row: `parseComponentPackageId`
   over the crate's real `Cargo.toml` must equal `semio:<pluginId>` and the row's `packageId`; the crate's
   root source must **declare** that same identity — `Plugin::<…>::builder(<literal|CONST>)` for a plugin,
   `ExtensionBundle::new(<literal|CONST>, …)` for an extension, with `SCREAMING_SNAKE` constants resolved
   from `const NAME: &str = "…"` in the same file; any `.package_id("…")` literal must equal the component
   package; and the deployment catalog must carry a row for the id. This is task 2's "same identity test
   for extensions (all 26)" as **one** gate instead of 26 copies, and it now also guards the 33 plugins.
2. **Every hand-authored `🧪️fixtures/🧫️plugin-identity/` tuple** (space's and reasoning's today, any
   future one automatically) is validated against its own sibling schema by **ajv 2020** (third-party
   oracle) and then joined to the derived tuple: `pluginId`/`packageId`/`packageName`/
   `artifactKindPrefix`/`moduleDirectoryName` and the Cargo playground variant.
3. **A non-vacuity law.** Synthetic sources prove the extractor reads the *real* call and not a
   doc-comment decoy (`builder(…)`/`builder(...)` — the `energy`/`note` trap the census flagged), that it
   resolves a constant, that it *reports* drift rather than passing (a `"imperative-extension-core"`
   constant is read as exactly that), and that it throws on an undefined constant or a missing call.

Root-file resolution needs no per-plugin literal: `<cratePath>/../../🦀️.rs` for 58 crates, with a
shallowest-match walk that finds `demonstrator`'s bespoke `🪪️manifest/🎪️demonstrator/🦀️.rs`.
`stdio`'s missing `.package_id(…)` literal (it derives the package id at runtime from its embedded
Cargo.toml) and every extension's missing one (`ExtensionBundle::package_id` is unused repo-wide — the
census's open question #4) are **asserted as the two documented exceptions**, not silently skipped, so
the day either grows a literal the test starts checking it.

## 4. Commands and real outputs

### Regeneration

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry && bun ./📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 45 framework packages) -> …/📇️registry/🤖️generated
.vscode/launch.json regenerated -> /Users/ueli/Documents/semio/.vscode/launch.json

$ grep -rn reasoning 🤖️generated/ 📦️deployment/🗺️catalog.json
🤖️generated/🔌️plugins.json:1155:    "pluginId": "reasoning",
🤖️generated/🔌️plugins.json:1156:    "packageId": "semio:reasoning",
🤖️generated/🔌️plugins.json:1158:    "packageName": "semio-s-plugin-reasoning-mindmap",
🤖️generated/🔌️plugins.json:1159:    "wasmOut": "semio_s_plugin_reasoning_mindmap.wasm",
📦️deployment/🗺️catalog.json:50:    { "pluginId": "reasoning", "directoryName": "💡️reasoning" },
🤖️generated/🖥️hosts.rs:62:    ("reasoning-wires", "reasoning"),
🤖️generated/🖥️hosts.rs:98:    ("wires", "reasoning"),
🤖️generated/🎮️playgrounds.ts:70: { variant: "reasoning-wires", pluginId: "reasoning", … ports: { react: 6015, wgpu: 6115 }, … },
🤖️generated/🗿️artifacts.rs:52:    ("reasoning", "semio_s_plugin_reasoning_mindmap.wasm"),
🤖️generated/🧩️plugins.ts:69: { pluginId: "reasoning", packageId: "semio:reasoning", … }
```

```
$ cd …/🎯️targets/🧊️wgpu/📦️packages/🦀️rust && bun ./📜️script.ts generate-frame-worker
framework-renderer-wgpu: generated 🎞️frame-worker.js
$ bun ./📜️script.ts check-frame-worker
framework-renderer-wgpu: 🎞️frame-worker.js is fresh
```

### The new identity gate, standalone

```
$ cd …/🔌️plugin/📇️registry && bun …/node_modules/vitest/vitest.mjs run --config 🧪️tests/🟦️.ts \
    --testTimeout 600000 🪪️plugin-identity.test.ts
 RUN  v4.1.10 …/🔌️plugin/📇️registry
 Test Files  1 passed (1)
      Tests  3 passed (3)
   Duration  8.41s
```

### Registry vitest as registered

```
$ bun nx run @semio-tech/plugin-registry:test        # (fails; see below)
$ node node_modules/nx/bin/nx.js run @semio-tech/plugin-registry:test --verbose
 ❯ |@semio-tech/plugin-registry| 🚀️launch.test.ts (5 tests | 2 failed) 144ms
⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯
 FAIL  … 🚀️launch.test.ts > plugin registry generated preview launchers > exposes every owned generator preview exactly once in contract order
 FAIL  … 🚀️launch.test.ts > WASI codegen profile policy > keeps generated native, root preflight, and MCP runtime profiles identical without debug
 Test Files  1 failed | 3 passed (4)
      Tests  2 failed | 21 passed (23)
   Duration  7.26s
```

The 3 passing files are `🪪️plugin-identity.test.ts` (3/3, new), `📖️generated-projection.test.ts` (4/4)
and `✅️catalog-complete.test.ts`'s quick level. **Both failures are the two peer-owned, pre-existing ones
lane K already documented**, re-verified here against HEAD rather than taken on trust:

- `dev-distribution-bundle` is missing from `🚀️launch.test.ts:26-34`'s `previewOrder` —
  `git show HEAD:…🔣️taxonomy.json | grep -c dev-distribution-bundle` → **2**, i.e. present before my diff;
  my taxonomy edit adds zero occurrences of it.
- the `🖨️describe` script's changed cargo-argument style — `git status --porcelain` on
  `🔌️plugin/🖨️describe` and on `🚀️launch.test.ts` is **empty**: neither file is touched by this lane.

### Cargo

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-m \
    cargo check -p semio-s-plugin-reasoning-mindmap --lib --tests --keep-going --message-format=short
warning: `semio-s-plugin-reasoning-mindmap` (lib) generated 44 warnings
error: could not compile `semio-s-plugin-reasoning-mindmap` (lib) due to 426 previous errors
```

**RED, and peer-owned — see §5.1.** Every one of the 426 is the in-flight repo-wide async-convention
sweep (`expected X, found future`, `no method named iter found for opaque type impl Future<…>`,
`E0433 unresolved crate semio_framework_job`), spread across `🗿️artifacts/🔌️wires/**`,
`👁️viewer/**`, `✏️editor/**` and `🧬️schema/**` — files this lane never touched
(`git status --porcelain ✏️s/🔌️plugins/💡️reasoning` lists **only** `Cargo.toml`, the root `🦀️.rs` and the
new `🧪️fixtures/`). The 44 warnings are proof the crate reached codegen rather than aborting at module
expansion.

An **untouched sibling** was measured to size the blast radius, so "peer-owned" is evidence, not a claim:

```
$ … cargo check -p semio-s-plugin-note --lib --keep-going --message-format=short
✏️s/🔌️plugins/🗒️note/…/✏️editor/🧵️retained/🦀️.rs:2372:17: error[E0433]: cannot find module or crate `semio_framework_job` …
warning: `semio-s-plugin-note` (lib) generated 76 warnings
error: could not compile `semio-s-plugin-note` (lib) due to 807 previous errors; 76 warnings emitted
```

`note` — a crate this lane never opened — is **807 errors** of the identical class. The sweep is
repo-wide and in flight.

```
$ … cargo check -p semio-s-plugin-imperative-effect --lib --keep-going --message-format=short
    Checking semio-s-plugin-imperative-effect v0.1.0 (…/📣️effect/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 30.37s
```

**GREEN** — task 2's crate compiles clean with the renamed `EXTENSION_ID` and label.

```
$ … cargo check -p semio-s-plugin-imperative --lib --keep-going --message-format=short
warning: `semio-s-plugin-imperative` (lib) generated 14 warnings
error: could not compile `semio-s-plugin-imperative` (lib) due to 42 previous errors; 14 warnings emitted
```

RED for the same peer sweep (`semio_framework_job` unresolved, `Label: From<&str>`,
`JsonValue: FromValue`) — **none of the 42 is in `🗿️artifacts/📜️procedure/…/🚪️io/🦀️.rs`**, the one file
this lane edited in that crate. `semio-s-plugin-sequence` (the other registrar call site, a one-line
string change) was not measured separately.

## 5. Blockers and follow-ups

1. **`cargo check` on the two crates is RED for peer reasons, so the new Rust law is written but not yet
   executed.** The framework's SDK has already been de-async'd while the plugin bodies have not:
   `plugin_exports!` at `📦️packages/🦀️rust/🦀️.rs:608` reports
   `expected Result<Plugin<ReasoningApps>, …>, found future` against the crate's own
   `pub async fn plugin()`. My test therefore writes `crate::plugin().await`, matching the signature as it
   stands **today**; whoever sweeps this crate to a sync `plugin()` (as `space` already is — lane K's test
   has no `.await`) must drop that one `.await`. It is one more compile error inside the file they will
   already be editing. `imperative-extension-effect` itself is **GREEN**; the untouched sibling
   `semio-s-plugin-note` is **807 errors** of the same class, which is what makes the attribution solid.
2. **The committed owner descriptor pair for `reasoning` is now identity-stale by construction** —
   `✏️s/🔌️plugins/💡️reasoning/🔣️.json:5` says `reasoning-mindmap` and `🛂️.descriptor.semio` is
   hash-bound to it. Identical to lane K's blocker #1; only a rebuild clears it. **The next rebuild is the
   real proof** of both fixes: this lane deliberately built no wasm.
3. **Task 3 — the reported cosmetic drifts are NOT gate-reachable; do NOT fix them under this ticket.**
   Verified by reading the gate, not by inference: `preflight_artifact_identity`
   (`🔌️plugin/🦀️.rs:2118`) inspects only the string handed to it, and its three call sites
   (`:3517`, `:28090`, `:28094`) hand it only `ArtifactDeclaration.kind` and
   `subset.dialect.artifact_kind`. Schema descriptor ids reach only
   `preflight_artifact_schema_descriptors` (`🧬️schema/⚛️component.rs:324`), which checks
   **uniqueness and mutual consistency, never the owner segment**. Follow-up scope, with the corrected
   counts:
   - `remodel`: **~20** sites of `s.remodeling.*` (the census said 4) — the schema/snapshot/diff
     `#[artifact_schema(id = …)]` attributes, the inference descriptor and its two field specs, the four
     editor config schemas (`…remodelingworldcamera`, `…remodelinglayervisibility`,
     `…remodelingframecursor`, `…config`) and the presence schema, plus the capability descriptor strings
     at `🗿️artifacts/📸️remodeling/🦀️.rs:59-60`. Target `s.remodel.remodeling*`.
   - `procedural`: **8** sites of `s.generation.{2d,3d}[.config|.presence]` under
     `🗿️artifacts/{🧊️generation3d,🌀️generation2d}/…/✏️editor/{🎚️config,👥️presence}/🧬️schema/🦀️.rs`.
     Target `s.procedural.generation{2d,3d}.*`.
   - `imperative-extension-effect`'s module slug `"core"` (`📣️effect/🦀️.rs:123,131,162`) — a
     within-`imperative` module namespace, user-visible in the imperative catalogue, self-referenced only.
4. **Two stale production bundles** still embed `💡️reasoning-mindmap`:
   `🧑‍💻dev/📤️distribution/🧶️bundles/🚀️bootstrap-B__lgXdC.js` and `🖥️runtime-Dc0wwVhF.js`. Both are
   content-hash-named Vite outputs needing a full `framework-os-dev:build` (the same blocker lane K left
   for `🪐️s`).
5. **Lane K left `plugins.stories.tsx` referencing a deleted binding** (`S` in `EXPORTED_STORIES` after
   `export const S` became `export const Space`), which would throw at module-eval in Storybook's preview
   — i.e. the whole plugin-matrix scope was dead, not just the space story. Fixed here as a side effect
   of the same edit. Worth a look at whether any other lane-K rename has the same one-sided shape.
6. **The census's open question #4 is now pinned rather than answered.** No extension anywhere calls
   `ExtensionBundle::package_id(…)`, so every extension's `manifest.package_id` stays `""`. Law 1 asserts
   that exception explicitly, so the day the describe route starts requiring it, this gate is where it
   surfaces — but whether `describeBuiltPlugin` tolerates the empty string still needs one live build to
   settle (lane B).
