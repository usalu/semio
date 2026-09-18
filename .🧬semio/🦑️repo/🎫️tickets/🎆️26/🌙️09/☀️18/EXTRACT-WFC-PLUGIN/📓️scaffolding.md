# 🀄️ Slice P — plugin skeleton + repo registration

Plugin folder: `✏️s/🔌️plugins/🀄️wfc` (see §0 — it is NOT `🌊️wfc`). Crate `semio-s-plugin-wfc`,
package `semio:wfc`, nx projects `@semio-tech/wfc-plugin` / `@semio-tech/wfc-js`.

## 0. Two emoji faults found and fixed (both were plan defects, both blocked the whole repo)

### 0.1 `▦️grid2d` — taxonomy loader refused the artifact folder
`▦️` (U+25A6 SQUARE WITH ORTHOGONAL CROSSHATCH FILL) is not Extended_Pictographic, so
`canonicalTaxonomyEmoji` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:706`)
rejects it and `loadTaxonomy()` threw `semanticDirectoryMemberKinds[…] has invalid exact member`
(`:4217`) — killing `verify taxonomy`, every `bun ./📜️script.ts new …` scaffolder and every
taxonomy-driven gate repo-wide. Renamed to `🔲️grid2d` (U+1F532 + FE0F) per the coordinator's call;
`loadTaxonomy(repoRoot)` was then probed directly and returns cleanly.

### 0.2 `🌊️wfc` — the plugin folder emoji collides with the `🌊️flow` plugin
`parseModuleDirectories` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts:45`)
folds each `directoryName` to its first grapheme with FE0F stripped and **rejects a duplicate**:
`🌊️flow` and `🌊️wfc` both fold to `🌊`, so `plugin-registry:generate` died with
`Duplicate module identity or sibling emoji`. Leaving the row out is equally fatal — the
`@repo/emoji-project-json` nx plugin then fails the whole project graph with
`Component needs an authored deployment directory: …/📦️packages/🦀️rust/Cargo.toml` for any crate
carrying `role = "plugin"`. There is therefore no state in which `🌊️wfc` can host a plugin crate.

The folder was moved to `✏️s/🔌️plugins/🀄️wfc` (🀄 U+1F004 MAHJONG TILE RED DRAGON + FE0F: RGI
emoji, free among the 59 plugin sibling emojis, and the domain object of all five artifacts — the
engine already names its tiled module `🀄️tiled`). The coordinator adopted `🀄️wfc` as canonical and
ran its own replacement pass; the duplicate `🌊️wfc` tree that peer slices had recreated was proven
fully redundant (235 files, 0 unique, 0 newer than their `🀄️wfc` twins) and removed, which is what
finally made the nx project graph load again.

## 1. Files created inside the plugin

| path | what |
|---|---|
| `🀄️wfc/🦀️.rs` | plugin root: `WfcApps` (10 variants), `register_inference_factories()`, `plugin()` |
| `🀄️wfc/📦️packages/🦀️rust/Cargo.toml` | crate `semio-s-plugin-wfc`, 5 playground rows, 3 `[[test]]` blocks |
| `🀄️wfc/📦️packages/🦀️rust/📋️project.json` | `@semio-tech/wfc-plugin`: test/test-quick/test-long/test-exhaustive/describe |
| `🀄️wfc/📦️packages/🦀️rust/📜️script.ts` | `TestScript` + `DescribeScript` + `registerPlaygroundSiteBuildCommands` |
| `🀄️wfc/📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,🟦️.ts}` | `@semio-tech/wfc-js`, barrel of 5×11 facade re-exports |
| `🀄️wfc/🧪️tests/🎚️config/🟦️.ts` | vitest config the TS `test` target runs |
| `🀄️wfc/🧪️tests/{🔬️surface,🚪️close-ladder,😴️idle-turns,🔬️boot-deadline}/🦀️.rs` | plugin-level laws, adapted to the five apps |
| `🀄️wfc/🧫️fixtures/🚪️close-ladder/🔣️.json` | `semio.wfc.close-ladder/v1` cost fixture |
| `🀄️wfc/🎮️commands/📌️.empty.md`, `README.md`, `AGENTS.md` | owner docs |

### Plugin root contract (`🀄️wfc/🦀️.rs`)
- `dyn_enum_close!` `WfcApps`: `BitmapEditor/BitmapViewer/Grid2dEditor/Grid2dViewer/Wfc2dEditor/`
  `Wfc2dViewer/Grid3dEditor/Grid3dViewer/Wfc3dEditor/Wfc3dViewer`, each
  `VcsArtifactApp<EditorApp<…>>` / `VcsArtifactApp<ViewerApp<…>>`.
- `register_inference_factories()` installs all five `register_<ident>_inference_factory(&ActionBus::production())`
  before the builder (procedural/assembly precedent), each mapped to a named `PluginAssemblyError`.
- `plugin()`: `builder("wfc").label("WFC").version("0.1.0").package_id("semio:wfc")`,
  5× `.routed_inference(<crate>::standards::v1::subsets::any::schema::inferences::<ident>_inference_metadata())`,
  5× `.declare_artifact(<crate>::artifact::<WfcApps>())`, 5× `.editor_mutation_roster` +
  5× `.viewer_mutation_roster`, 5× `.activation(OnArtifactKind { kind: <crate>::artifact_kind().id })`,
  `.execution(ExecutionMode::Isolated)`, one `.requests(CapabilityRequest { id: "artifacts.write", … })`,
  `.try_build()`.
- `#[cfg(test)] #[path = "🧪️tests/🔬️surface/🦀️.rs"] mod surface_tests;` and
  `#[cfg(feature = "plugin-entry")] plugin_exports!(plugin, WfcApps);`.

### Deviations from the slice brief (deliberate)
- **No `fixtures-lint` nx target / `FixturesScript`.** Puzzle's `fixtures lint` is a **repo-wide**
  walk (`discoverArtifacts(this.repoRoot)` finds every `🧬️mutations` tree in the monorepo); it is a
  singleton that happens to live in puzzle — no other of the 36 plugin manifests carries it. Copying
  its ~200 lines into wfc would create a second identical repo-wide lint, which the repo's
  no-duplication rule forbids. wfc's `📋️project.json` therefore carries the standard
  test/test-quick/test-long/test-exhaustive/describe set (procedural/remodel/note shape) and wfc's
  mutation trees are already covered by puzzle's existing lint.
- **No wasm-pack `WasmScript`** (as instructed) and no `[package.metadata.wasm-pack]` block — those
  are puzzle-5d's board-session package, not plugin boilerplate.
- **Plugin-level tests use only framework API.** The procedural originals arm a preview by loading a
  named example DSL and rendering a known window body key (`procedural.play.preview`). wfc's window
  kind ids and body keys are owned by the artifact slices, so the ported laws drive the close ladder
  and the idle window with document-text round-trips and empty reactor turns instead of invented
  window keys. The close-cost fixture's `maximumCloseTurns: 768` / `minimumRetainedWorkDilutionPercent: 140`
  are carried over from the procedural twin and are marked in the fixture's `lawNote` as needing a
  re-measurement against wfc's own runs once the artifacts' preview windows retain real state.
- The surface test's `examples` law calls `<crate>::examples::example_source_slice()` (plan §1's
  stated export) and the inference law asserts `metadata.owner == "wfc"` and
  `metadata.artifact_kind == artifact_kind().id` — `Plugin::runtime` is private, so the routed rows
  cannot be read back off the built manifest.

## 2. Artifact crate manifests + stubs (created only where absent)

`test -e` was checked immediately before every write; artifact slices own these files.

| folder | crate | Cargo.toml | `🦀️.rs` stub |
|---|---|---|---|
| `🗿️artifacts/🖼️bitmap` | `semio-s-artifact-wfc-bitmap` | **written by P** | written by P |
| `🗿️artifacts/🔲️grid2d` | `semio-s-artifact-wfc-grid2d` | already present (slice A2) | written by P |
| `🗿️artifacts/◻️2d` | `semio-s-artifact-wfc-2d` | already present (slice A3) | written by P |
| `🗿️artifacts/🧱️grid3d` | `semio-s-artifact-wfc-grid3d` | **written by P** | written by P |
| `🗿️artifacts/🧊️3d` | `semio-s-artifact-wfc-3d` | already present (slice A5) | written by P |

The two P-written manifests carry `role = "s-module"`, `[lib] path = "../../🦀️.rs"`,
`default = []`, and `component-app-assembly = ["semio-framework-plugin/component-guest",
"dep:semio-framework-async", "dep:semio-framework-geometry", "dep:semio-framework-os-infinite",
"dep:semio-framework-ui", "dep:semio-framework-ui-contract", "dep:semio-framework-ui-scene",
"dep:semio-framework-ui-styling"]` — puzzle-2d's list minus `dep:wasm-bindgen` (its `web-sys`
canvas bridge is puzzle-specific) plus assembly's `ui`/`ui-styling`. Deps are puzzle-2d's framework
set + `semio-s-plugin-wfc-engine` + `semio-s-artifact-stdio-{semio,txt,json}` (the 2D artifacts also
get `-png` and `-svg` for `TileMedia2d`).

## 3. Registration outside the plugin — exact lines

### `Cargo.toml` (root)
`members`, plugin block (next to the engine member slice E added):
```
    "✏️s/🔌️plugins/🀄️wfc/📦️packages/🦀️rust",
```
`members`, artifact block (after the puzzle trio):
```
    "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust",
```
`[workspace.dependencies]` (artifact aliases after `semio-s-artifact-puzzle-5d`, plugin alias before
`semio-s-plugin-wfc-engine`):
```
semio-s-artifact-wfc-bitmap = { path = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust" }
semio-s-artifact-wfc-grid2d = { path = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust" }
semio-s-artifact-wfc-2d = { path = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust" }
semio-s-artifact-wfc-grid3d = { path = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust" }
semio-s-artifact-wfc-3d = { path = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust" }
semio-s-plugin-wfc = { path = "✏️s/🔌️plugins/🀄️wfc/📦️packages/🦀️rust" }
```
The engine member + alias were already there (slice E); untouched.

### `package.json` (root)
```
    "✏️s/🔌️plugins/🀄️wfc/📦️packages/🟦️typescript",          → workspaces
    "dev:wfc": "bun nx run workspace:dev -- wfc",              → scripts
```

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`
```
    { "pluginId": "wfc", "directoryName": "🀄️wfc" },           (between "vcs" and "writer")
```

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `semanticDirectoryMemberKinds["members-of-plugins"].memberNames` += `"🀄️wfc"`.
- `semanticDirectoryMemberKinds["members-of-artifacts"].memberNames` += `"🔲️grid2d"`, `"🧱️grid3d"`,
  `"🖼️bitmap"` (`◻️2d` and `🧊️3d` were already listed for puzzle/fem/block).
- `fileKinds`: new `dev-plugin-component-wfc-{js,declaration,wasm}` (pathPattern
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/🀄️wfc/semio_s_plugin_wfc_component.{js,d.ts,core.wasm}`)
  and `dev-plugin-interfaces-wfc` (exact path `…/🔌️plugin-modules/🀄️wfc/interfaces`), each mirroring
  the writer/puzzle rows verbatim.
- `fixedDirectoryContractSets`: `"dev-plugin-interfaces-wfc"` appended to the **four** sets that
  carry every plugin — `dev-jco-all-interfaces`, `dev-jco-plugin-interfaces`,
  `dev-jco-contributor-interfaces`, `dev-jco-wall-clock-interfaces`.
- authority rows: `dev-plugin-component-wfc-js` ("JCO WFC companion without purity exemption") and
  `dev-plugin-component-wfc-declaration` ("JCO WFC declaration pairing").
- **Deliberately NOT added**: `dev-jco-{filesystem,actor,pure,resident}-interfaces`. Those four sets
  are behaviour subsets keyed on which host interfaces a component actually imports (procedural is in
  actor/pure/resident, note is in filesystem/actor but not pure). wfc's component has never been
  built, so its real interface roster is unknown; adding it on structural analogy would be a guess.
  **W2 must re-check these four after the first `describe`/jco materialization.**

### `.vscode/🧩️launch.seed.jsonc`
- `configurations`: five `"@generated:<variant>:react"` + `"@generated:<variant>:wgpu"` pairs and
  five hand-written `🖥️native` entries (`bun nx run @semio-tech/framework-renderer-wgpu:native -- <variant>`),
  presentation orders 201.2 / 202.2 / 203.2 / 204.2 / 205.2.
- `devLaunchers`: five entries, `S_OS_PORT` + `SEMIO_RENDERER` env (procedural's modern shape; the
  `<PLUGIN>_PLAY_PORT` var of the older entries is dead code), orders 201–205 / wgpuOrders 201.1–205.1.

| variant | namePrefix | dev command |
|---|---|---|
| bitmap | `🀄️wfc🖼️bitmap` | `bun nx run workspace:dev -- wfc bitmap` |
| grid2d | `🀄️wfc🔲️grid2d` | `bun nx run workspace:dev -- wfc 2d grid` |
| wfc2d | `🀄️wfc◻️2d` | `bun nx run workspace:dev -- wfc 2d` |
| grid3d | `🀄️wfc🧱️grid3d` | `bun nx run workspace:dev -- wfc 3d grid` |
| wfc3d | `🀄️wfc🧊️3d` | `bun nx run workspace:dev -- wfc 3d` |

The dev command words are matched against the playground rows' `aliases` **exactly**
(`🎮️playground/🧭️session/🟦️.ts:46` — `variant === v || aliases.includes(v)`), so `"wfc 2d"` and
`"wfc 2d grid"` do not shadow each other.

### `.vscode/launch.json`
Not hand-edited. `bun nx run @semio-tech/plugin-registry:generate` rendered all **15** wfc entries
(`🛠️dev🀄️wfc<artifact>{⚛️react,🧊️wgpu🌐️wasm,🧊️wgpu🖥️native}`) from the seed + the registry ports.

### `.claude/launch.json`
Five attach rows appended: `wfc-bitmap-react-attach` 6041, `wfc-grid2d-react-attach` 6042,
`wfc-wfc2d-react-attach` 6043, `wfc-grid3d-react-attach` 6044, `wfc-wfc3d-react-attach` 6045.

## 4. Port table (re-verified free repo-wide before use)

`grep -rn "604[1-5]\|614[1-5]" --include=Cargo.toml` plus `.vscode/launch.json`, `.claude/launch.json`
and the seed returned **zero** hits before the rows were written.

| variant | app id | react | wgpu |
|---|---|---|---|
| bitmap | `s.wfc.bitmap@1/*#editor` | 6041 | 6141 |
| grid2d | `s.wfc.grid2d@1/*#editor` | 6042 | 6142 |
| wfc2d | `s.wfc.wfc2d@1/*#editor` | 6043 | 6143 |
| grid3d | `s.wfc.grid3d@1/*#editor` | 6044 | 6144 |
| wfc3d | `s.wfc.wfc3d@1/*#editor` | 6045 | 6145 |

## 5. Gate results

Logs under `🗑️generated/P/`.

| gate | result |
|---|---|
| `loadTaxonomy(repoRoot)` (direct probe) | ✅ returns cleanly — the `invalid exact member` throw is gone |
| `bun nx run @semio-tech/plugin-registry:generate` | ✅ `60 plugin crates, 65 playgrounds`; `.vscode/launch.json` regenerated with all 15 wfc entries; the 5 wfc rows in `🤖️generated/🎠️playgrounds.json` carry the right app ids, aliases and ports |
| `cargo metadata --format-version 1 -q` | ✅ exit 0 — the workspace resolves with all six wfc crates |
| `bun ./📜️script.ts verify taxonomy report --scope "✏️s/🔌️plugins/🀄️wfc"` | see §5.1 |
| `cargo check -p semio-s-plugin-wfc -j 4` | see §5.2 — expected red until all five artifact crates export their surfaces |
| `bun install` | not run — the TS workspace addition needs no new third-party package (`@semio-tech/wfc-js` has an empty `dependencies` and only `typescript`/`vitest` dev deps, both already hoisted) |

### 5.1 `verify taxonomy report --scope "✏️s/🔌️plugins/🀄️wfc"` — blocked by two PRE-EXISTING, non-wfc faults

The scoped run never reaches its findings: `verifyTaxonomy` → `planTaxonomy` throws before any wfc
path is classified. Two distinct faults were hit, in order, neither of them wfc's:

1. `Current compiler input manifest compiler inputs are not path-sorted`
   (`🧹️normalization/🟦️.ts` `compilerInputManifestRows`). Source:
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json` — gitignored, last
   written 2026-09-15, 3 030 `inputs[]` rows of which **10 consecutive pairs** are byte-unsorted
   (e.g. `…🎭️actor/🖼️wire-turn/🟦️.ts` before `…🎭️actor/📦️packages/🟦️typescript/🟦️.ts`). It contains
   **zero** wfc rows. Regenerating it is a `framework-os-dev` distribution build; not touched here.
2. `frozen-coordinate-evidence-invalid: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json: document digest does not match registered bytes`
   (`frozenCoordinateEvidenceCoordinates` ← `planMoveReferenceAuthority`). A concurrently edited cad
   fixture whose registered digest is stale. Also not wfc.

What **was** proven for this slice: `loadTaxonomy(repoRoot)` — the step that the `▦️` fault broke —
now returns cleanly when probed directly, so the taxonomy vocabulary wfc added is admissible. The
scoped findings pass still has to be re-run by W2 once the two faults above are cleared by their
owners.

### 5.2 `cargo check -p semio-s-plugin-wfc -j 4` — red, exactly one error, in a peer slice's file

```
error: couldn't read `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: No such file or directory (os error 2)
error: could not compile `semio-s-artifact-wfc-grid3d` (lib) due to 1 previous error
```
`semio-s-plugin-wfc-engine` and `semio-s-artifact-wfc-bitmap` compiled. The build stops at
`semio-s-artifact-wfc-grid3d`, whose crate root (slice A4, 538 lines) `#[path]`-mounts an
`✏️editor/🦀️.rs` that has not landed yet. **No error originates in a P-owned file, and no artifact
API was stubbed.**

Artifact crate-root state at the time of the run:

| artifact | root `🦀️.rs` |
|---|---|
| `🖼️bitmap` | still P's 1-line stub — slice A1's root has not landed |
| `🔲️grid2d` | 553 lines (A2) |
| `◻️2d` | still P's 1-line stub — slice A3's root has not landed |
| `🧱️grid3d` | 538 lines (A4), mounts a missing `✏️editor/🦀️.rs` |
| `🧊️3d` | 606 lines (A5) |

## 6. What remains red / open for W2

1. Rerun `cargo check -p semio-s-plugin-wfc --features …` once A1/A3 land their crate roots and A4
   lands `🧱️grid3d/…/✳️any/✏️editor/🦀️.rs`. The plugin root additionally requires, per artifact:
   `artifact_kind()`, `artifact::<PA>()`, `editor::<ident>::<Ident>Editor` +
   `create_<ident>_editor()`, `viewer::<ident>::<Ident>Viewer` + `create_<ident>_viewer()`,
   `examples::example_source_slice()`, and
   `standards::v1::subsets::any::schema::inferences::{register_<ident>_inference_factory, <ident>_inference_metadata}`.
2. `bun nx run @semio-tech/wfc-plugin:describe` → `plugin-registry:generate` → `:check` (describe
   needs a compiling `wasm32-wasip2` component, so it waits on 1).
3. `verify taxonomy report/enforce --scope` — waits on the two foreign faults in §5.1.
4. The four behaviour-keyed `dev-jco-{filesystem,actor,pure,resident}-interfaces` taxonomy sets —
   add wfc to whichever the first real jco materialization proves it imports (§3).
5. `verify dependencies write-baseline` → `literal-external`, and `bun ./📜️script.ts policy` — both
   need the artifact crates' final dependency sets, so they belong to W2, not to P.
6. `@semio-tech/wfc-js:test` runs `passWithNoTests: false` over
   `🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts` and
   `🗿️artifacts/**/🧬️schema/🧪️tests/🧩️suite/🟦️.ts` — red until the artifact slices author those.
7. The close-ladder fixture's ceiling/dilution numbers are inherited from procedural and must be
   re-measured against wfc's own runs (see the fixture's `lawNote`).
