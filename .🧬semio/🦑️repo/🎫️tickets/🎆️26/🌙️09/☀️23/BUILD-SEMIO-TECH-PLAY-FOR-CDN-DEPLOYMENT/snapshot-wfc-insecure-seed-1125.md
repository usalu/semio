# Snapshot WFC `insecure-seed` During Actor Preparation (2026-11-25)

Tree: `/Users/ueli/.sinc/play` (snapshot). Nothing was compiled or run (no cargo/rustc); the running release build is the verifier.

## Symptom

`semio-plugin-describe describe --prepare-actor` on `semio-hub-wfc-component` (wasm32-wasip2) refused with
`actor preparation requested wasi:random/insecure-seed@0.2.9::insecure-seed` (phase `prepare`, ~13 s / 5.3e8 fuel).
On wasip2 that import is what `std::hash::RandomState::new()` uses the first time a default-hashed `std::collections::HashMap`/`HashSet` is built.

## Preparation Path (read-only framework)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs` `--prepare-actor` → `write_prepared_actor` → `compiled.prepare_actor(...)`.
- `🧠️interpreter/🔌️owned-semio-boundary/🌱️preparation/🦀️.rs` `prepare_core` runs the export `semio_owned_prepare_actor_v1` import-free; any `HostCall` is refused with the error above.
- `📣️macros/⚛️wp4-reactor/🦀️.rs` (`plugin_exports!(…, prepare)`): the export runs `__semio_ensure_plugin_runtime()` → `PluginRuntime::new` + `install_plugin_bundle_result(runtime, semio_hub_wfc::plugin())`.
- `🌎️hub/🧩️compositions/🀄️wfc/🦀️.rs` `plugin()`: `register_inference_factories()` (five `ActionBus::register_once`; wfc is the only composition doing this), `routed_inference` ×5, `declare_artifact(artifact::<WfcApps>())` ×5 (eagerly builds each subset: schema/inference descriptors, `io::io()`, `create_*_editor/viewer()` app definitions with tools, layouts, window kinds, examples), mutation rosters, `try_build()`.
- `🏗️builder/🦀️.rs` `try_build` → `project_artifact_declarations` → per surface `OriginalPluginAppFactory::registration` → `produce_declared_tool_catalog::<A>` (`🧵️instance/👷️jobs/📥️staging/🧱️components/🏛️declarations/🦀️.rs`), which calls each wfc editor/viewer `register_tool_job_factories` and bounded-proof factory registration on the production bus; then `into_runtime`, `publish_declared_catalogs`, `commit_original_producers`.
- The framework structures on this path (`ActionBus`, `Plugin`, `PluginRuntimeRegistry`, registration plan) all use `BTreeMap`/`BTreeSet`; the framework has no deterministic `BuildHasher` (`FixedState`/`BuildHasherDefault`/`DeterministicHasher` do not exist). Convention: ordered maps.
- Prior live note (`🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS/session-2026-11-23.md`): the wasm stack of `semio_owned_prepare_actor_v1` reached the import "through `std::sync::Once` and `alloc::string` while the grid2d inference factory is being registered" (ICF-folded symbols: the five `register_*_inference_factory` bodies are identical). Live resolved it in the HOST (fixed preparation seed), which is framework code and not portable here.

## Randomness Sources Found In `✏️s/🔌️plugins/🀄️wfc` (library code)

Every default-hashed collection in wfc library code, all replaced by ordered equivalents:

| Site | Was | Now | Iteration effect |
|---|---|---|---|
| `⚙️engine/🏗️model/🦀️.rs` `ModelBuilder.tag_ids` (built by `ModelBuilder::new()`/`Default`, reached by every `TiledModelBuilder::new()` incl. `Grid2dInferenceJob::new`, `Grid3dInferenceJob`, wfc2d/wfc3d job cores, every fill tool) | `HashMap<String,u32>` | `BTreeMap` | lookup only |
| same, `CompiledModel.tag_ids` and incremental-compiler `HashMap::new()` | `HashMap` | `BTreeMap` | lookup only |
| `⚙️engine/⛏️extract/🦀️.rs` `extract_2d` `window_freq` (bitmap compile / entropy) | `HashMap<Vec<TileId>,u64>` | `BTreeMap` | already sorted afterwards; now born sorted |
| `⚙️engine/🗺️topology/🦀️.rs` `from_graph_view` `index_of` | `HashMap` | `BTreeMap` | lookup only |
| `⚙️engine/🚫️nogood/🦀️.rs` `NogoodIndex.watchers` (every `search::solve*`) | `HashMap` | `BTreeMap` | `values_mut` index remap is order-insensitive |
| `⚙️engine/🕳️sparse-3d/🦀️.rs` `SparseVolume.index` | `HashMap` | `BTreeMap` | lookup only |
| `⚙️engine/🎼️motif/🦀️.rs` `canonicalize` | `HashMap` | `BTreeMap` | first-seen order via `len()`, unchanged |
| `🗿️artifacts/🖼️bitmap/…/🧬️mutations/✍️paint-input-stroke/🦀️.rs` `stroke_cells` `seen` | `HashSet` | `BTreeSet` | membership only; output Vec keeps drawing order |

No `ahash`, `rand`, `fastrand`, `getrandom`, `RandomState`, `DefaultHasher`, `indexmap` anywhere in wfc. Non-semio deps are only `serde`, `serde_json`, `base64_codec`, `machine`, `pack`. WFC pseudo-randomness already comes from an explicit document seed: `search::solve` uses `Rng::from_seed(seed)` and every job sets `operation.seed = snapshot.seed`.

## Files Changed

- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/⚙️engine/🏗️model/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/⚙️engine/⛏️extract/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/⚙️engine/🗺️topology/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/⚙️engine/🚫️nogood/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/⚙️engine/🕳️sparse-3d/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/⚙️engine/🎼️motif/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✍️paint-input-stroke/🦀️.rs`

Key types are `Ord`: engine ids derive `PartialOrd, Ord`; `VoxelCoord = (i32,i32,i32)`; graph-engine `NodeId` was already `sort_unstable`d. No API used is HashMap-only (no `with_capacity`/`reserve`/`drain`/`capacity`). No dependency or Cargo.lock change. Hub composition unchanged (no randomness there). Live repo has the same HashMaps; nothing to port.

## What Remains And Why

- `HashSet` in tests only: `🗿️artifacts/*/🏭️bridge/🧪️tests/🦀️.rs` (5) and `🗿️artifacts/🖼️bitmap/🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` — not in the component.
- `std::time::Instant::now()` in `🗿️artifacts/*/🏭️bridge/🦀️.rs` (native mutation-inventory tool binaries, own workspaces, not linked into the component) and in `⚙️engine/🔍️search/🦀️.rs` (`solve`, `solve_with_*`) / `⚙️engine/🔍️search/🏃️batch/🦀️.rs` — wall-clock deadlines of batch solves, only reached when a solve runs, never during plugin assembly. Not randomness, but on wasip2 they would request `wasi:clocks` if a solve ever ran during preparation.
- Risk: static tracing could not prove which wfc call constructed the first `RandomState` (no wfc HashMap is on an obvious assembly hook; the live stack points at inference-factory registration). With this change wfc library code contains no default-hashed collection at all, so if the release build still refuses, the remaining `RandomState` is in framework code reached only through a wfc-specific declaration shape (framework is out of scope here); the next step would be the guest wasm stack of `semio_owned_prepare_actor_v1` at the `insecure-seed` import.

## Second Pass

The rebuilt component (`component.wasm`, 229,794,556 B, 13:03) still refused. Nothing was compiled; the analysis below runs only the already built `semio_hub_wfc_component.core.wasm` (jco transpile stage, 14:06) under Node with every host import stubbed.

### 1. Closure Diff (`🌎️hub/Cargo.lock`)

`semio-hub-wfc` closure: 1158 packages. Union of `semio-hub-note`, `-puzzle`, `-trinity`, `-vcs`, `-stdio-pdf`, `-stdio-binary`, `-stdio-media`, `-stdio-mesh`, `-flow`: 1857. Suspects (wfc-only), all workspace crates, no third-party crate:

`semio-framework-job-headless`, `semio-hub-wfc`, `semio-s-plugin-wfc-engine`, `semio-s-plugin-wfc-engine-{foundation,job,kernel,lattice,solve}`, `semio-s-wfc-sqlite-bitmap`, and the 50 `semio-s-artifact-wfc-*` crates (`2d`, `2d-config`, `2d-diff`, `2d-inferences`, `2d-mut-any`, `2d-mutations`, `2d-pin-slot`, `2d-schema`, `2d-tests`, `2d-transient`, `2d-v1`, `3d`, `3d-config`, `3d-inferences`, `3d-mut-family`, `3d-mutations`, `3d-schema`, `3d-set-slot-positions`, `3d-snapshot`, `3d-tests`, `3d-transient`, `3d-wfc3d`, `bitmap`, `bitmap-bitmap`, `bitmap-inferences`, `bitmap-mut-family`, `bitmap-mut-family-2`, `bitmap-pin-solution`, `bitmap-schema`, `bitmap-snapshot`, `bitmap-tests`, `bitmap-tests-shrinks-output-pin`, `bitmap-transient`, `grid2d`, `grid2d-create-tile`, `grid2d-diff`, `grid2d-inferences`, `grid2d-schema`, `grid2d-snapshot`, `grid2d-tests-cuts-pinned-corner`, `grid2d-v1`, `grid3d`, `grid3d-editor`, `grid3d-inferences`, `grid3d-mut-family`, `grid3d-mutations`, `grid3d-schema`, `grid3d-snapshot`, `grid3d-tests`, `grid3d-transient`).

### 2. Suspect Sources

Following each lib `path` and every `#[path]`/`include!` mount: 889 files, all under `✏️s/🔌️plugins/🀄️wfc` or `🌎️hub/🧩️compositions/🀄️wfc`, plus `🧰️framework/🔨️modules/🧵️job/🏃️headless/**` and `🧰️framework/🔨️modules/🌱️value/📽️projection/👁️post/🧪️testing/🦀️.rs`. After the first pass, the only `HashMap`/`HashSet`/`RandomState`/`ahash`/`fxhash`/`rand`/`fastrand`/`getrandom`/glob hits are in the bitmap inference unit tests. No dependency of these crates turns on any extra feature. So the trigger is shared code that only wfc's declarations reach.

### 3. Dynamic Localisation (built core module, Node)

- `semio_owned_prepare_actor_v1` with every import throwing reproduces the error: `wasi:random/insecure-seed@0.2.9::insecure-seed`. Stack (wasm function indices, release build has no name section): `15854 > 41764 > 41818 > 41786 > 41563 > 231686 > 41566 > 41528 > 41525 ⇄ 41564 > 41557 > 41555 > 231061 > 130861 > 133108 > 159623 > 159725 > 175125 > 175825(export)`.
- Identified by the string literals each body references: 133108 = `semio_hub_wfc::plugin()` (`bitmap-inference-factory`, `semio:wfc`, …); 130861 = `PluginBuilder::try_build` (`plugin-assembly.*`); 231061 = `PluginRuntimeRegistry::extend_contributions` (`plugin-assembly.owner-mutation-roster`); 41555…41525/41564/41528 = the mutation-input JSON-Schema reader (`x-semio-ui`, `a mutation payload schema describes an object`, `$ref`); 41818 = `OnceLock` init (`Once instance has previously been poisoned`, `$ref$defs`); 41764 = `RandomState::new` (reads TLS keys at `global 614`+16, else calls 15854); 15854 = std `hashmap_random_keys` → the import.
- Completeness check: in an in-memory copy, the keys TLS state is never latched, so every `RandomState::new` asks the host again, and insecure-seed returns a fixed seed. Preparation then completes with readiness `1`, makes no other host call, and asks for the seed only on this one path (twice, from the same site). **This is the only randomness source during wfc preparation.**

### 4. Root Cause (outside scope, not edited)

`/Users/ueli/.sinc/play/🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs:137-138,141`. Live is identical:

```rust
struct Index { parsed: std::collections::HashSet<(usize, usize)>, documents: std::collections::HashMap<String, DslValue> }
let mut index = INDEX.get_or_init(Default::default)...
```

`registered_input_schema_document` builds a process-wide index with default std hashers the first time a mutation payload schema resolves a cross-document `$ref`.

Call chain: `semio_hub_wfc::plugin()` (`🌎️hub/🧩️compositions/🀄️wfc/🦀️.rs`, `.editor_mutation_roster::<…>()` / `.viewer_mutation_roster::<…>()`) → `PluginBuilder::try_build` → `runtime.extend_contributions(…, &owner_mutation_rosters, …)` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:723`) → `WireMutationRosterEntry::owner` → `WireMutationInputs::of` (`…/🔌️plugin/📜️declaration/📇️registration/🦀️.rs:286,654,673`) → `semio_framework::mutation_input_defs(schema, &registered_input_schema_document)` → `InputSchemaReader::target` (`🔡️mutation-inputs/🦀️.rs:461`, cross-document branch) → `registered_input_schema_document` → `Index::default()` → `RandomState::new`.

Why only wfc: the passing hubs that register rosters (trinity, flow) have no leaf payload schema with a cross-document `$ref`. In wfc exactly two roster leaves have one, because `WfcTileMedia2d::Image` carries an `ArtifactChild`:

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-tile/🧬️schema/🔣️.json:734`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/🧬️schema/🔣️.json:692`

Both point at `https://json.schemas.assets.semio-tech.com/framework/schema/artifact-reference/child/schema.json`, which itself points at `…/artifact-reference.json#/$defs/ArtifactRef` (→ local `#/$defs/ArtifactDialect`). All other wfc `$ref`s to `https://…` are in aggregate, transient, snapshot or tool-cache documents that the roster reader never resolves.

### Fix

- **Recommended (framework, one owner):** use `std::collections::BTreeSet` / `BTreeMap` for `Index.parsed` / `Index.documents` at `🔡️mutation-inputs/🦀️.rs:137-138`. It is only used for keyed lookups and insertion, so iteration order does not matter. Every plugin whose roster leaves use a cross-document `$ref` needs this.
- **Smallest wfc-side workaround (not applied):** make the two grid2d leaf schemas self-contained. Copy `ArtifactChild`, `ArtifactRef` and `ArtifactDialect` into each leaf's local `$defs`, and point `child` at `#/$defs/ArtifactChild`, so `InputSchemaReader::target` walks the root and never calls the resolver. Cost: it duplicates framework-owned schema text into two plugin files, which can drift, and it changes the referenced-documents set of those leaves. Deferring the roster is not possible from wfc (the framework assembles it in `try_build`), and removing the `Image` media arm would remove a feature.

### What Changed In The Second Pass

Nothing was edited in the second pass. The first-pass collection swaps stay; they were not on the preparation path, but they make wfc's library code deterministic. The probe scripts are in the session scratchpad only, and no `.cache` file was written.

## Workaround Applied

Decision (coordinator): the framework fix goes to LIVE only. The snapshot gets the wfc-side workaround. Nothing was compiled.

### Change

In both grid2d roster leaf payload schemas, `properties…media.properties.child.$ref` changed from `https://json.schemas.assets.semio-tech.com/framework/schema/artifact-reference/child/schema.json` to `#/$defs/ArtifactChild`. A new root `$defs` (neither file had one, so nothing collides) now holds, copied verbatim from the snapshot's framework schemas:

- `ArtifactChild`, from `🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔣️.json`. It is the same object without `$schema`/`$id`: a nested `$id` would rebase `#/…` resolution onto the framework URI. Its nested `$defs.ContentId` is kept, and `target.$ref` is rewritten to `#/$defs/ArtifactRef`.
- `ArtifactRef` and `ArtifactDialect`, from `🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json`. `ArtifactRef.dialect` already points at `#/$defs/ArtifactDialect`. `ArtifactKindId` is not referenced, so it was not copied.

The leaf `$id`, title, top-level keys and every `x-semio-ui` are unchanged.

Files:
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-tile/🧬️schema/🔣️.json`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/🧬️schema/🔣️.json`

### Equivalence

- **Fully dereferenced trees:** the old file plus the two framework documents, and the new self-contained file, were each expanded by following every `$ref` and dropping only `$defs`/`$schema`/`$id`. The results are byte-identical, `x-semio-ui` included, for both files.
- **Third-party oracle (Ajv from the snapshot's `node_modules`):** 7 instances per schema gave identical verdicts for old and new: valid, missing `childId`, extra property, missing `dialect`, extra dialect key, wrong `childId` type, missing `subset`. Results: `true,false,false,false,false,false,false`.
- **Input reader (framework `🔡️mutation-inputs`):** `child` carries `widget: hidden` on an object, so it yields `ArgSchema::Any` (line ~715) either way. The only other difference is the cycle-guard key in `ResolvedInput.refs`. The derived inputs are identical, and the cross-document branch of `InputSchemaReader::target` (line 461) is no longer taken.

### Other Consumers / Derived Files

- **No committed mirror encodes the old `$ref` for these two leaves.** They declare `requiredLanguageSurfaces: ["rust","json-schema"]` only, so there are no per-leaf `🟦️.ts`/`🔗️.graphql`/`🛰️.proto`. The aggregate `…/🔲️grid2d/…/🧬️mutations/🔣️.json`, the TS id lists (`…/🧬️schema/🧬️mutations/🟦️.ts`, `…/🚪️io/📝️text/🧬️mutations/🟦️.ts`), `…/🔮️oracles/🔣️.json` and the leaf tests reference the leaves only by kind or `$id`, which is unchanged. No committed digest of the schema text exists.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json`** maps the two unchanged `$id`s to unchanged paths. No regeneration is needed; the new inline defs have no `$id`.
- **Rebuild-derived values (not committed):** `MutationLeaf::PAYLOAD_SCHEMA` (via `include_str!`), and `COMPILED_INPUTS.schema_sha256` / `framework_document_sha256` (computed by the derive at compile time; the framework-document list becomes empty for these two leaves). `PAYLOAD_SCHEMA_DOCUMENTS` stays empty: the framework child schema was never in wfc's index.
- **Still holding the framework child `$ref`, intentionally:** two document snapshot schemas that the roster input reader never reads, so they are not on the preparation path:
  - `…/🔲️grid2d/…/🧬️schema/📸️snapshot/🔣️.json:63`
  - `…/🧱️grid3d/…/🧬️schema/📸️snapshot/🔣️.json`

  A later framework fix makes all of these harmless. A later regeneration of the leaf schemas from a shared template would bring the external `$ref` back.

### Other Cross-Document `$ref`s (Step 3)

I enumerated all 95 leaf descriptors (`🔣️.json` with `payloadSchema`) across bitmap, grid2d, wfc2d, grid3d and wfc3d, which are the only documents the roster passes to `mutation_input_defs`. After the edit, none of their payload schemas contains a `$ref` that does not start with `#`. The remaining `https://…` refs in wfc are in aggregates, config/transient aggregates, root/snapshot/inference schemas and the grid2d fill cache. None of these is a roster input, so none was changed.

### Harness

The existing core wasm (14:06) still embeds the old schema text: both embedded copies of the create-tile leaf contain the external `$ref` and no `#/$defs/ArtifactChild`. So it still fails exactly as before and cannot show the effect of this change until the component is rebuilt and re-transpiled. What it does prove (second pass) is that this cross-document resolution is the only randomness request during preparation. With it gone from every roster leaf, the next build's preparation should be import-free. Re-run `probe.mjs` (throwing stubs) on the rebuilt `.core.wasm` to confirm `result 1`.
