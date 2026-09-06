# 📓️ Explore — build gates and history for `✏️s/🔌️plugins/🔋️energy`

Read-only, no builds run, no code touched. All paths repo-root-relative unless noted.

## 1. Repo-wide gates the energy plugin must pass

### 1.1 Root `📜️script.ts` `verify` subcommands (`VerifyScript`, `📜️script.ts:10477`)

Dispatch table at `📜️script.ts:10478-10543`. Bare `bun ./📜️script.ts verify` runs `runGate()`
(`:11013-11162`) then, unless the first segment is `gate`, `nx run-many -t test --all --exclude
workspace` (`:10543-10544`) — i.e. `verify` alone is gate + full test suite; `verify gate` is gate only.

| subcommand | dispatch | what it checks |
|---|---|---|
| `verify` (bare) | `:10543-10544` | `runGate()` + full `nx run-many -t test --all` |
| `verify gate` | `:10543, returns at :10544` | `runGate()` only (below) |
| `verify taxonomy report\|enforce [--scope <path>]` | `:10479-10480`, impl `runTaxonomy` `:10547-10556` | calls `verifyTaxonomy()` (normalization module, see §1.2) via `--scope`; `report` logs only, `enforce` throws on any `error`-severity violation |
| `verify mutation-outcome-law` | `:10482-10485`, impl `:10565-10572` | the 7 mutation-outcome/merge-policy/no-CRDT/no-validate/derive-mirror rules (`policyMutationOutcomeMergePolicyBreaches`, `priority==="high"` only) — same block `runGate()` runs at `:11134-11143` |
| `verify rust-warnings --target <native\|wasm32-wasip2\|wasm32-unknown-unknown> [-p <crate>]` | `:10486-10488`, impl `runRustWarnings` `:10585-10602` | `cargo clippy -p <pkg> ... -- -D warnings` per crate for the target; deny-on-warnings via clippy's trailing flag, never `RUSTFLAGS` (would clobber `.cargo/config.toml`'s wasm cfg) |
| `verify interactivity tool-jobs [...]` / `apps` / `p1q-b1-b6` / `p1w`/`p1x`/`p1y`/`p1z`/`p5d`/`p5e`/`p3mn` / bare | `:10489-10516` | Phase-0 forbidden-call audit (`block_on`, sync-fs/net/clipboard/process/db, thread/Rayon/Tokio construction) — currently WARN severity (`INTERACTIVITY_AUDIT_SEVERITY`, see `:10653-10662`), reports only, exit 0 |
| `verify dependencies <parity js\|self-test\|summary\|literal-external\|list\|write-baseline\|(bare)>` | `:10517-10519`, impl `runDependencyFreeze` `:10918-10989` | `literal-external`/`summary`: `dependencyTruthReport` — target is **zero** literal third-party deps repo-wide, else oracle-conflicts/toolchain-owner-conflicts/toolchain-audit failures (`:10947-10959`); bare form: new-dependency ratchet vs `📓️dependencies-baseline` (`:10976-10988`) |
| `verify layering [write-baseline]` | `:10520-10523`, impl `runLayering` `:10996-11011` | dependency-direction: repo-wide/framework code must not grow references into an "implementation area" past a shrink-only baseline |

`runGate()` (`:11013-11162`) is the actual close-out bundle, in order:
1. `bunx dependency-cruiser 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs` (`:11019`)
2. `nx run @semio-tech/plugin-registry:check` — **the registry/catalog gate**, §1.3 (`:11022`)
3. region/host-contract lints (`framework-renderer-react:lint`, `framework-os-dev:plugin lint`, `ui-styling-tokens:check-no-px`) (`:11024-11026`)
4. framework owned-schema binding freshness (`framework-rs:check`) (`:11028`)
5. UI locale/terminology + chrome-i18n scans (`:11030-11032`)
6. `this.runLayering([])` (`:11034`)
7. **owner-root test taxonomy/feature contract** — `bun <testDomainPath>/📜️script.ts contract` (`:11036-11039`), see §1.4
8. Storybook/Tailwind scope freshness (`:11041`)
9. OS exclusive-state-authority + document-app-shape policies (`:11043-11054`)
10. standards/subsets vocabulary, handcrafted-grammar P3/M4, artifact-schema, app-schema, package-language-purity, dissolve-core/plugin-root, window/mode capability-taxonomy, mutation-outcome/merge-policy policies (`:11056-11143`, all `throw` on any `priority:"high"` breach)
11. dsl fixture laws quick-level tests (`:11144-11151`)
12. **`this.runRustWarnings(["--target", "wasm32-unknown-unknown"])`** (`:11160-11161`) — scoped only to the actor kernel, NOT energy; energy's own wasm32-wasip2/native cleanliness is only checked via the opt-in `verify rust-warnings --target <triple> -p semio-s-plugin-energy`, never by the default gate.

**Energy-relevant conclusion:** the default `verify`/`verify gate` bundle does *not* directly compile
energy. The only paths that will actually invoke `cargo check`/`clippy` on
`semio-s-plugin-energy` are (a) `nx run @semio-tech/energy-plugin:test*` (which needs the crate to
build to run its `#[cfg(test)]` modules), (b) an explicit `verify rust-warnings --target native -p
semio-s-plugin-energy` (and the `wasm32-wasip2` variant), and (c) `nx run
@semio-tech/plugin-registry:check`'s descriptor gate indirectly requiring a built wasm (§1.3). None of
these run automatically inside `verify gate`.

### 1.2 Taxonomy validator

Two distinct layers exist under this name:

- **Internal vocabulary self-consistency** — `validateTaxonomy()` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:2906-2921+`): asserts `🔣️taxonomy.json` itself is well-formed (`schemaVersion === 7`, no removed keys, ids referenced by every `*Ids` array actually exist in their registry, patterns are valid anchored regexes, etc.). `fileKinds` (`🔣️taxonomy.json:1460+`) is one of the registries this cross-checks — e.g. `semanticDirectoryMemberKinds[id].memberNames` (`🔍️discovery/🟦️.ts:3116-3117`) must be a non-empty array, and every consumer (`🔍️discovery/🟦️.ts:1702,1949,2001,2014,2173,2184,2192,3340,3359,4435`) requires a directory name on disk to be **literally present** in its owner kind's declared `memberNames` array — this is the "memberNames vs disk" rule: an emoji-prefixed directory that exists on disk but isn't listed in `memberNames` for its parent kind fails every one of those call sites.
- **Path-vs-taxonomy drift** — `verifyTaxonomy()` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:8283-8301`, invoked by `verify taxonomy report|enforce`): builds a `plan` (`planTaxonomy`) of required moves/renames/reference-edits against the real filesystem and reports every unresolved item as a `TaxonomyViolation` (`error`/`warning` severity); `enforce` throws if any `error`-severity finding remains (`📜️script.ts:10556`).

No energy-specific taxonomy findings were pre-computed in this session (no cached run inspected); the plugin's directory tree (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/...`) already exists on disk and matches the shape used by `🏗️fem`/`🧩️puzzle` (see §1.5), so no drift is expected here without running it.

### 1.3 Registry `check`/`generate` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`)

`CheckScript.run` (`:3088-3161`) never writes; `GenerateScript` (`:1909`) writes the same artifacts.
Gate order inside `check`:
1. `validateGeneratorContractsAgainstWorkspace` (`:3091-3092`) — throws on any contract-authority problem.
2. Byte-compare every rendered catalog file (`🤖️generated/*`) and `.vscode/launch.json` (`generateLaunchJson`) against what's on disk; any mismatch → `process.exit(1)` telling the dev to run `generate` (`:3093-3112`).
3. `validatePlaygroundRegistry` + `validatePlaygroundSessions` (`:3114-3119`).
4. Taxonomy-tree audit for newly-contracted plugin roots (`:3120-3135`) — warn while `PLUGIN_AREAS_STATE` is `legacy`/`mixed`, hard-fail once `clean`.
5. `discoverPackageProblems` (warn-only) (`:3140-3144`).
6. **`validateDescriptors(entries, repoRoot)`** (`:3147, def :2063-2100+`) — the §3-of-`📓️design-abi.md` descriptor gate: for every plugin it requires BOTH `🔣️.json` and `🛂️.descriptor.semio` to exist under the owner root (else `errors.push(...)`, `:2072-2074`), then `validateCatalogDescriptorPair` (hash-verifies the descriptor's `wasmSha256`/`coreWasmSha256`/`descriptorSha256` against the actually-built component) and `auditInteractiveJobClassificationDrift`; any thrown/returned error → `process.exit(1)` (`:3152-3156`).

Energy's descriptor pair **does exist on disk** (`✏️s/🔌️plugins/🔋️energy/🔣️.json`,
`✏️s/🔌️plugins/🔋️energy/🛂️.descriptor.semio`, confirmed present via `find`), so step 6's existence
check passes — but the sibling `S-END-TO-END` catalog audit
(`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️explore-catalog-build-state.md:22,64,95`)
records energy as **"a staged descriptor but no wasm/JS"** and flags a **"DIVERGENT (`data.model` vs
`data.🔋️model`)"** mismatch — meaning the hash-verification half of step 6 (`validateCatalogDescriptorPair`)
is expected to fail once run, because there is no current `.core.wasm` for the hashes to match (the
crate doesn't compile, so it was never built). `describe` (energy's own nx target, §1.5) must be re-run
after the crate compiles and its wasm32-wasip2 component is rebuilt.

### 1.4 Test-domain `contract` phase and its breach cache

Router: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` (`bun ./📜️script.ts
<discover|contract|oracle|subject|parity|run|report|clean|...>`, header at `:1-9`). `ContractScript.run`
(`:829-838`): `validateAllContracts(repoRoot, cases)` → writes the full breach list to
`.🧬semio/⚡️cache/breaches/testing.json` (`getRepoMetaDir(repoRoot)/⚡️cache/breaches/testing.json`,
`:833-835`) unconditionally (even zero breaches — an empty-array file), then `console.log`s
`formatBreachReport`, then `process.exit(1)` if any breach exists (`:837`). `RunScript` (`:863-873`)
runs the same `validateAllContracts` inline before the oracle/subject phases but does **not** cache it
(`:868`, "not cached — contract phase failed inside run"). This is the gate `runGate()` step 7 invokes
repo-wide via `bun <testDomainPath>/📜️script.ts contract` (`📜️script.ts:11036-11039`,
`testDomainPath` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` per
`🔣️taxonomy.json:17948`). **The cache file does not currently exist on disk**
(`.🧬semio/⚡️cache/breaches/breaches/testing.json` absent at the time of this exploration) — no
`contract` run has been cached in this ticket yet, so no committed energy-specific breach exists to
report; running `bun ./📜️script.ts contract` from the test domain (or `verify gate` step 7) is how one
would populate it.

### 1.5 `launch.json` conventions for a plugin

`.vscode/launch.json` is 100% generated (`generateLaunchJson`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts`) from the playground catalog —
never hand-edited; `registry:check` byte-compares it (§1.3 step 2).

**Energy's existing entries (verbatim, grep of `.vscode/launch.json`):**
- `:7901-7904` — `"🔋️ Energy JS Tests"` → `bun nx run @semio-tech/energy-js:test`
- `:9717-9726` — `"🛠️dev🧩️energy⚛️react"` → `bun ./📜️script.ts dev energy`, env `SEMIO_PLUGIN=energy`, `SEMIO_APP=s.energy.model@1/*#editor`
- `:9739-9748` — `"🛠️dev🧩️energy🧊️wgpu🌐️wasm"` → same command/app, wgpu+wasm variant
- `:9761-9768` — `"🛠️dev🧩️energy🧊️wgpu🖥️native"` → `bun ./🧰️framework/.../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native energy`

That is 4 launch entries total for energy: one JS-test entry and one dev entry per (react / wgpu-wasm /
wgpu-native) renderer target — the same *shape* every plugin with one artifact and one playground
variant gets (energy declares exactly one `[[package.metadata.semio.playground]]` block in its
Cargo.toml, §3). No energy-specific `📦️build`/`📦️audit`/Storybook entries exist, and none are expected:
those extra entries on siblings below come from those plugins having multiple artifacts (puzzle: 2d/3d/5d)
or extra audited surfaces (puzzle's publication-authority), not from anything energy is missing.

**Neighbouring plugin's full set** (`🧩️puzzle`, which has 3 artifacts × react/wasm/native × sometimes ×2
variants, plus Storybook and build/audit entries) — every `"name"` match in `.vscode/launch.json`:
`🛠️dev🧩️puzzle🩻️2d⚛️react` (`:1253`), `🛠️dev🧩️puzzle🩻️2d🧊️wgpu🌐️wasm` (`:1273`),
`🛠️dev🧩️puzzle🩻️2d🧊️wgpu🖥️native` (`:1293`), `🛠️dev🧩️puzzle🏙️3d⚛️react` (`:1304`),
`🛠️dev🧩️puzzle🏙️3d🧊️wgpu🌐️wasm` (`:1324`), `🛠️dev🧩️puzzle🏙️3d🧊️wgpu🖥️native` (`:1344`),
`🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest⚛️react` (`:1355`) + wasm/native (`:1375,1395`),
`🛠️dev🧩️puzzle👯️5d⚛️react` (`:1406`) + wasm/native (`:1426,1446`),
`🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest⚛️react` (`:1457`) + wasm/native (`:1477,1497`),
`🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream⚛️react` (`:1508`) + wasm (`:1530`),
`🛠️dev📖️storybook🧩️puzzle🩻️2d` (`:1922`), `🛠️dev📖️storybook🧩️puzzle🧊️3d` (`:1979`),
`🛠️dev📖️storybook🧩️puzzle🕐️5d` (`:1998`), `📦️build🧩️puzzle🌉️board` (`:7801`),
`📦️audit🧩️puzzle🛂️publication-authority` (`:7979`), `📦️build🧩️puzzle🏙️3d🎛️concrete` (`:8056`),
`📦️build🧩️puzzle👯️5d🎛️concrete` (`:8067`), `📦️build🧩️puzzle👯️5d🎛️capsule` (`:8078`).

The single-artifact sibling with the **closest shape to energy** is `🏗️fem` (2 artifacts, no
Storybook/build/audit extras): `🛠️dev🏗️fem🩻️2d⚛️react` (`:7655`) + wasm/native (`:7675,7695`),
`🛠️dev🏗️fem🏙️3d⚛️react` (`:7706`) + wasm/native (`:7726,7746`) — i.e. fem gets 2×(react+wasm+native) = 6
dev entries for its 2 artifacts, exactly the "3 dev entries per artifact" pattern energy's single
`🔋️model` artifact already gets (3 entries) plus its 1 JS-test entry. **Energy's launch.json footprint
is already complete for its declared shape** — nothing is missing here.

### 1.6 `📋️project.json` — energy vs a complete plugin

Energy's Rust project.json (`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📋️project.json`) declares
targets `test`, `test-quick`, `test-long`, `test-exhaustive`, `describe` — **byte-identical target set**
to `🏗️fem`'s (`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📋️project.json`), which is the other
same-generation "END-TO-END" plugin. `🧩️puzzle`'s (more mature, multi-artifact) project.json adds two
targets energy/fem don't have: `wasm` (`bun ./📜️script.ts wasm`) and `fixtures-lint` (`bun
./📜️script.ts fixtures lint`) — both because puzzle's own `📜️script.ts` implements those subcommands;
energy's `📜️script.ts` (`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📜️script.ts`, 1360 bytes) does not.
Energy's TS project.json (`📦️packages/🟦️typescript/📋️project.json`) has only `test` — identical to
fem's.

**The one structural gap vs `🏗️fem`:** fem has a per-(artifact/standard/subset) `🏭️generator`
project.json under `🗿️artifacts/◻️2d/.../🌐️any/🏭️generator/📋️project.json` and
`🗿️artifacts/🧊️3d/.../🌐️any/🏭️generator/📋️project.json` (targets `🏭️generator`,
`🏭️generator-manifests`, `🏭️generator-carrier`, `🏭️generator-carrier-manifests` — fixture-generation
infra). **Energy has no `🏭️generator` directory anywhere under its `🗿️artifacts/🔋️model` tree**
(confirmed via `find`) — there is no scaffolded fixture-generator project for the `✳️any` subset, which
lines up with the definition-of-done's item 2 (exhaustive per-mutation fixture tests) being unstarted
work, not merely unwired nx targets.

## 2. History — how energy got into its current non-compiling state

`git log --date=iso` for `✏️s/🔌️plugins/🔋️energy` returns 82 commits; every commit message on this repo
embeds a frozen template date (`🎆️26🌙️06☀️04`) per prior-session findings — `%ad` (author date) is used
throughout below, confirmed via `--date=iso`.

- **First commit touching the path:** `fa51b5c82f` 2026-08-04 17:28:01 — plugin scaffolded.
- **Plugin declared in the Cargo workspace** since creation: root `Cargo.toml:124` lists
  `"✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust"` as a member (confirmed present today).
- **`semio-framework` as its own crate has existed since `a5cc4dd9ab`, 2026-08-07 15:56:49** (`git log
  -S'name = "semio-framework"' -- '*Cargo.toml'` returns exactly this one commit) — root
  `Cargo.toml:181` declares `semio-framework = { path = "🧰️framework/📦️packages/🦀️rust" }  # 58 refs`.
  **The crate is real and has been real the whole time** — the compile blocker is not "the crate
  doesn't exist," it's that energy's own `Cargo.toml` never lists it as a dependency (confirmed by
  reading the current file: only `semio-s-plugin-stdio`, `geometry`, `semio-framework-os-kernel`,
  `semio-framework-job`, `semio-framework-plugin`, `semio-framework-dispatch-macros`,
  `semio-framework-schema`, `semio-framework-value-derive`, `pack`, `serde`, `serde_json` are
  dependencies — no plain `semio-framework`).
- **The `semio_framework::` import path was introduced exactly once** (`git log -S'semio_framework::'
  -- ✏️s/🔌️plugins/🔋️energy` → single hit `9ed590cd87`, 2026-08-25 09:16:05) and never removed by a
  committed change since — it sat broken in the tree until this ticket's W-A worker patched the
  working copy (uncommitted, see §3). That commit's message
  (`⚡️Ship energy model mounted retained simulation session with numerical microcursor and
  retained-wire channel`) is the same day as
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/📓️terra-p7-energy-current-gap-caller-ownership-census-2026-08-25.md`
  — the "P7c" packet that first authored energy's editor/simulation-session surfaces (`✏️editor/🦀️.rs`,
  `🧵️simulation-session/🦀️.rs`) had zero prior history — brand-new files — and both used `use
  semio_framework::kernel::{Effect, JobPlacement}` / `semio_framework::InteractiveJobClassification`
  from the moment they were authored, never a path that had ever resolved for this crate. This reads as
  a copy/adaptation mistake (most plugin code reaches the kernel through its own
  crate's re-export, `semio_framework_plugin::kernel::…`, not the bare framework crate) rather than a
  later external rename breaking a previously-working import.
- **This went uncaught because no gate ever re-ran `cargo check -p semio-s-plugin-energy` after
  2026-08-25.** `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📓️draw-energy-lowpoly-space-sourcing-writer-report.md`
  records energy as verified **0 warnings, 0 errors** on `cargo check -p semio-s-plugin-energy` (lib
  target) — but that ticket's own directory date (`🌙️08☀️17`) predates the 08-25 commit that broke the
  crate, i.e. the "clean" snapshot was taken before the offending files existed. §1.1's gate audit
  confirms `verify gate`'s only wasm/native compile step is scoped to the actor kernel
  (`:11160-11161`), not energy, and the two other gates that *would* have caught it
  (`verify rust-warnings --target … -p semio-s-plugin-energy`, and `energy-plugin:test*` actually
  building the crate) are both opt-in/on-demand, never invoked by `verify gate` automatically. The
  first automated signal was the sibling `S-END-TO-END` catalog audit
  (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md:47`):
  *"Does not compile"* — `semio_framework` import unresolved, **279 errors**
  (`🔋️model/…/🧵️simulation-session/🦀️.rs:7`); no core wasm — recorded 2026-09-05, ten days after the
  break, and the same audit's catalog-build-state companion
  (`.../📓️explore-catalog-build-state.md:22,64,95`) shows energy as "descriptor staged, no wasm/JS,"
  i.e. the crate has not produced a wasm artifact since before that break.
- **A second, independent compile blocker layered on top**, discovered by this ticket's W-A worker
  (`📓️w1-compile.md`, §2): the `ArtifactEditor` trait
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26623-26965`) is sync except
  `command_from_intent`, but energy's editor implemented `render`/`pending_effects`/
  `initial_snapshot`/`handle`/`command_id` all as `async fn` — a framework-wide API change (sync
  `ArtifactEditor`) that energy's editor was never updated to match, confirmed against the compiling
  `🧩️puzzle` 2D editor (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs:2089`, sync `fn
  render`) as the oracle. This is consistent with energy's editor/simulation-session code having been
  authored once (2026-08-25/2026-09-02 per §2's commit list) and never revisited as the framework
  trait signature moved underneath it — the same "no gate re-checks this crate" gap explains why this
  drift also went unnoticed.
- **`git log` shows energy's `✏️editor/🦦️.rs` and `🧵️simulation-session/🦀️.rs` were both last committed
  2026-09-02** (`e5465a2c1c` 13:31:56 and `21fbcd3538` 12:19:02) — these are two of the repo's periodic
  giant taxonomy-migration commits (kind-only filenames, package renames; commit bodies confirm scope:
  *"Migrate all plugin and artifact source, spec, and example trees from component-named files to
  kind-only filenames"*), not energy-specific edits — energy's files moved/were touched as a side
  effect of a repo-wide rename, not because anyone was working the plugin itself, between 2026-08-25
  and the start of this ticket.
- Root `Cargo.toml` and energy's own `Cargo.toml` have each been touched repeatedly since (root:
  `git log -n1 -- Cargo.toml` unresolved due to volume, energy's own last at `b0dfa0f09b` 2026-09-05
  19:04:38) by the same wave of repo-wide migrations — none of the visible history suggests anyone ran
  a scoped `cargo check -p semio-s-plugin-energy` between 2026-08-17 and this ticket's start on
  2026-09-06.

### 2.1 Other tickets that mention energy (grep for `🔋️`/`plugin-energy`/`semio-s-plugin-energy` across
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/**/*.md`, excluding `🗑️generated`, ~180 files hit on a broad "energy"
grep — the ones below are the substantive, energy-specific reports, not incidental mentions):

- `🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w3-semio-s-plugin-energy-report.md` — energy's
  plugin-architecture migration (crate/taxonomy consolidation) was explicitly marked RELEASED despite
  a self-contradicting cross-ticket status; i.e. energy was treated as "done" on the architecture axis
  even without its own completion report.
- `🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/📓️energy-simulation-relocation-report.md` —
  moved the simulation engine from `🧬️schema/💡️inferences/` to a new top-level
  `🔨️modules/⚡️simulation/⚙️engine/` location (the location the engine still lives at today).
- `🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/📓️packet-energy-report.md` — deleted the old
  `🗿️artifacts/🔋️model/…/⚙️engine/` tree (50 subdirs, ~11.9k LOC) after the relocation above — this is
  where the engine's current ~19k-line, 50-module footprint (per this ticket's status.md) was
  established.
- `🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️energy-model-direct-leaf-cutover.md` — cut the
  `♻️replace-model` mutation over to the direct-leaf mutation architecture; this is the same
  `♻️replace-model` mutation flagged elsewhere in this ticket's own exploration as violating the
  mutation-derivation rules (§ mutation-vocabulary report).
- `🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/📓️energy-remodel-note-serde-to-value.md`
  — records energy as "mostly already converted" from serde to first-party `ToValue`/`FromValue`
  *except* the engine bridge and 9 non-engine files, and explicitly flags
  `energy_structure_from_model`/`energy_model_from_structure` (in `🗿️artifacts/🔋️model/🦀️.rs`) as
  architecturally pinned to `serde_json::Value` — the same docstring visible in energy's current
  `Cargo.toml` comment (confirmed in §3).
- `🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📓️draw-energy-lowpoly-space-sourcing-writer-report.md`
  — the "energy was clean" snapshot discussed in §2 above; dated *before* the 08-25 break.
- `🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️v-energy-component-pilot-readiness.md` — an early,
  explicitly bounded/non-mutating read-only taxonomy-normalization dry run scoped to
  `🗿️artifacts/🔋️model`.
- `🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/` (whole
  subfolder, ~11 files, `terra-p7*`/`sol-p7c*`/`codex-p7c3*`) — the packet that authored energy's
  mounted retained simulation session (numerical microcursor, retained-wire channel) — this is the
  same work whose 2026-08-25 commit introduced the unresolved `semio_framework::` import (§2).
- `🌙️09/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md` and
  `.../📓️explore-catalog-build-state.md` — the sibling ticket's repo-wide plugin-catalog audit that
  first surfaced energy's current "does not compile, 279 errors, no core wasm, descriptor staged but
  divergent (`data.model` vs `data.🔋️model`)" state, one day before this ticket opened.

## 3. Current uncommitted state and `Cargo.toml`

`git status --porcelain -- ✏️s/🔌️plugins/🔋️energy` (at time of this exploration) shows exactly 4
modified, untracked-nothing:
```
 M ✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript/package.json
 M ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs
 M ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
 M ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs
```
These are exactly this ticket's own W-A worker's uncommitted fixes (three `semio_framework::` →
`semio_framework_plugin::` import-path edits + the TS `package.json` rewrite), per
`📓️w1-compile.md` §§1,3 — not yet a clean compile, since W-A's own report (§2) records a second,
still-unfixed blocker (async `ArtifactEditor` methods vs the now-sync trait).

`Cargo.toml` (`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/Cargo.toml`, read in full):
- `[package]` `name = "semio-s-plugin-energy"`, component package `semio:energy`
  (`[package.metadata.component]`), `role = "plugin"`, one playground variant
  (`[[package.metadata.semio.playground]] variant = "energy", app = "s.energy.model@1/*#editor", ports
  = { react = 6106, wgpu = 6206 }`) — this single playground declaration is why energy gets exactly
  3 dev launch entries (§1.5), not more.
- `[lib]` `crate-type = ["cdylib", "rlib"]`, `path = "🦀️.rs"`.
- `[dependencies]`: `semio-s-plugin-stdio` (features `["full-artifact-catalog"]`,
  `default-features = false`), `geometry` (package `semio-framework-geometry`),
  `semio-framework-os-kernel`, `semio-framework-job` (workspace), `semio-framework-plugin` (workspace,
  feature `component-guest`), `semio-framework-dispatch-macros`, `semio-framework-schema`,
  `semio-framework-value-derive`, `pack` (package `semio-framework-pack`), `serde`, `serde_json`
  (both `workspace = true`). **No plain `semio-framework` dependency** — confirms §2's root-cause
  finding.
- `[dev-dependencies]`: `semio-framework-async-macros` only.
- **No `[[test]]` entries** — no dedicated integration-test targets are declared; this matches the
  absence of any `🧪️tests/` directory under `📦️packages/🦀️rust` (`find` confirms only `Cargo.toml`,
  `📋️project.json`, `📜️script.ts`, `🦀️.rs` at that level) — all Rust tests are inline `#[cfg(test)]`
  modules, discovered by `cargo test`/`cargo check --tests` without any manifest wiring.
- Root `Cargo.toml:124` still lists `"✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust"` as a workspace
  member — confirmed unchanged.

## 4. Gate checklist — run in this order at the end of the ticket

1. `cargo check -p semio-s-plugin-energy --lib --tests` (native) — not a `verify` subcommand, but the
   prerequisite everything else assumes; use a private `CARGO_TARGET_DIR` per prior-session guidance to
   avoid contention with concurrent peer builds.
2. `cargo check -p semio-s-plugin-energy --target wasm32-wasip2` (or the crate's actual wasm build
   entry point) — DoD item 1's second half.
3. `bun ./📜️script.ts verify rust-warnings --target native -p semio-s-plugin-energy`
4. `bun ./📜️script.ts verify rust-warnings --target wasm32-wasip2 -p semio-s-plugin-energy`
5. `bun nx run @semio-tech/energy-plugin:test` (then `test-quick`/`test-long`/`test-exhaustive` as
   appropriate)
6. `bun nx run @semio-tech/energy-js:test`
7. `bun nx run @semio-tech/energy-plugin:describe` — regenerate `🔣️.json`/`🛂️.descriptor.semio` once
   the wasm32-wasip2 component builds cleanly (DoD item 3).
8. `bun nx run @semio-tech/plugin-registry:check` — registry/descriptor/catalog gate (§1.3); run
   `generate` first if it reports staleness.
9. `bun ./📜️script.ts verify taxonomy report --scope ✏️s/🔌️plugins/🔋️energy` (then `enforce` once clean)
10. `bun ./📜️script.ts contract` (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) — populates/
    refreshes `.🧬semio/⚡️cache/breaches/testing.json` (§1.4).
11. `bun ./📜️script.ts verify mutation-outcome-law` — energy's `♻️replace-model` mutation was flagged
    elsewhere in this ticket as a rule-6 violation; confirm before/after any mutation-vocabulary rework.
12. `bun ./📜️script.ts verify dependencies literal-external` — the crate adds no new third-party deps
    (per its own `Cargo.toml`), so this should already pass; re-check if any get added incidentally.
13. `bun ./📜️script.ts verify layering`
14. `bun ./📜️script.ts verify gate` — the full close-out bundle (§1.1); run last since it also
    re-invokes the registry check, layering, and contract-phase steps above.
15. `bun ./📜️script.ts verify` (bare, gate + `nx run-many -t test --all --exclude workspace`) — final
    repo-wide confirmation.
