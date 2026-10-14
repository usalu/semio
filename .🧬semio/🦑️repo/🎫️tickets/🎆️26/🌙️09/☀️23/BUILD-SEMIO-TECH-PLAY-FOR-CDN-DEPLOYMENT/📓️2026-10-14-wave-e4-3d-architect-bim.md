# 🧊️ Wave E4: framework-3d, architect program and BIM model (2026-10-13 / 2026-10-14)

Executor: Lane D / E4. Scope: `semio-framework-3d`, `semio-s-artifact-architect-program` and `semio-s-artifact-bim-model`.

The targets changed during the wave:

- **10-13 brief.** Laws L3/L5/L6/L8 and §10 E4 of `📓️2026-10-13-scalable-build-architecture.md`. Each library ≤ 15 k own lines and ≤ 3 GiB peak.
- **10-14 restart brief.** The global source admission ceiling of the Codex fleet (`🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/crate-production-boundaries.md`): **4,000 active lines, 262,144 full bytes and 32 files per unit**. The test closure of a unit counts too (`architecture-audit-current.md`, "Admission And Test Gaps").

Rules I kept:

- No git-modifying command, no worktree, no sub-agent, no ticket operation.
- I touched nothing in `🖱️ui` and nothing in `🗑️generated` outside `🗑️generated/e4/`.
- Every cargo call ran in the foreground through `🚦️cargo-slot.sh`, with private target and build dirs under `⚡️cache/e4/`. Those dirs are deleted at the end.
- Only per-test runs and one nx gate attempt ran in the background, and neither is cargo.

All tools are kept in `e4-3d-architect-bim/` (§8).

## 1. Result at a glance

| Crate | 10-13 | 10-14 | Ceiling status |
|---|---|---|---|
| framework-3d | Step 1–4 done: cut into 5 crates. Measured, tests identical. | Cut further into **14 crates**. 11 of them are admitted under the ceiling for both production and tests. All 14 compile native with tests and wasm32-wasip2. 733/733 tests are accounted for. | 11 admitted. Refused: `semio-framework-3d` (mesh), `-brep-operations`, `-brep-engine` (plan §5.1). |
| architect program | Step 2 map done. Step 3 decoupling done in source: 0 forbidden facet edges. | Step 3 is unchanged and **not compile-verified**: os-kernel and plugin are red upstream all day (§6). The 6-crate root split is scripted and dry-run (`✂️architect-split.json`) but not applied. | Refused. The ceiling needs a mutation-model change (§5.2). |
| BIM model | Baseline peak 4.75 GB (unit compile interrupted). | Step 2 map done. **Step 3 started**: root barrels removed and 2,082 paths canonicalized, schema identity moved into the model. **Not compile-verified**, upstream red. Remaining edges are listed name by name (§4). | Refused. Cut plan to ~40 units in §5.3. |

Upstream state on 10-14 (`📓️2026-10-14-compile-baseline.md` plus my own checks): `ui-theme`, `canvas`, `job`, `os-store-message-ledger`, `os-pack` and `os-command` all fail with working-tree edits from other lanes. Every crate depending on os-kernel or plugin therefore could not be checked: architect, BIM, their hub compositions, the 3d consumers above the kernel, the hub wasm workspace check and the manifest sweep.

I verified framework-3d against the HEAD copy of the one broken crate in its closure, `semio-framework-job` (§3.5).

## 2. Baselines (measured)

### 2.1 framework-3d

- **Unit compile (10-13).** wasm-release unit alone, private dirs: rebuild wall 24 s, peak RSS **1.643 GB** (`📏️unit-measure.sh`).
- **Lines.** 37,751 own non-test lines, 13,647 test lines.
- **Lib tests (HEAD 10-13, one process per test).** I built HEAD in a scratch copy (`git archive`; sibling modules symlinked; private build dir) and ran it with `🧪️each-test.sh`, 6 in parallel, 300 s alarm: **720 tests: 696 ok, 21 FAILED, 3 TIMEOUT.** The 21 failures were 18 mesh `original_*` laws and 3 engine laws, all pre-existing.

### 2.2 architect program

- **Unit compile (10-13).** wasm-release unit peak **5.80 GB**.
  - The clean-and-rebuild pass recompiled 33 units, because os-kernel and plugin changed under the run.
  - So I do not quote a solo wall time.
- **Lines and layers.** 44.3 k production lines.

  | Layer | Lines | Note |
  |---|---:|---|
  | mutations | 20.2 k | 1,140 files: 260 leaves × triad |
  | model | 10.3 k | |
  | editor | 6.0 k | editor + viewer + examples |
  | io | 2.4 k | |
  | inferences | 2.2 k | |
  | root | 3.1 k | mostly the module tree |

### 2.3 BIM model

- **Unit compile (10-13).** The wasm-release unit reached **4.75 GB** before upstream plugin breakage interrupted it.
- **Lines and layers.** 2,905 files, 92 k production lines, 98 k test lines.

  | Layer | Lines | Files | Note |
  |---|---:|---:|---|
  | inferences | 20.3 k | 158 | |
  | editor | 20.5 k | 256 | |
  | mutations | 18.4 k | 2,253 | most files are tests |
  | write | 9.4 k | | |
  | root | 7.9 k | | module tree |
  | model | 4.9 k | | |
  | diff | 4.2 k | | |
  | read | 2.8 k | | |
  | io | 1.3 k | | |

  Full before-census: `🗑️generated/e4/bim-census-before.txt`.

## 3. framework-3d: 14 crates

### 3.1 10-13: step 3 (decoupling) and the first cut

Path-resolved census (`🕸️3d-layers.json`): 4 forbidden facet edges became 0. The moves:

- **Kernel contract.** `⚙️engine/🔖️contract` (neutral types plus operation quality) moved to `📸️representation/🔖️contract`. Engine imports it privately, and the engine-root `pub use contract::*` is gone.
- **Intersection.** `🛠️operations/✂️intersect` moved to `💡️queries/✂️intersect`. It depends on representation only.
- **Wire, `make_loop`, `dummy_coedge`.** These moved from primitives and euler into `representation::topology`. `Body::from_seed` called `euler::make_loop`, and an inherent impl must stay with its type.
- **Allocation observer.** The test-only allocation observer (the global allocator plus the tessellation pointer witnesses) moved out of the tessellation tests into `🧊️3d/🧪️tests/🧰️support/🦀️.rs`. Every crate mounts it under `cfg(test)`.
- **Topology tests needing operations.** Seed, reachability and compaction tests need operation fixtures. They moved to `🛠️operations/🧱️primitives/🧪️tests/🔬️topology-{seed,reachability,compaction}`. A dev-dependency cycle would mismatch the crate's own test-instance `Body`.
- **Edge-use probe.** It moved from topology to `validation`, which is the only code that reads it.

Then the cut into `representation / queries / operations / engine` plus the base crate. Per-test verdicts were identical to HEAD (only the two load-sensitive timeouts passed). wasm-release peaks: 0.755 / 0.494 / 0.469 / 0.297 / 0.635 GB, against 1.643 GB for the uncut crate.

### 3.2 10-14: further cuts to the admission ceiling

- **queries** split into `intersect`, `tessellation`, `validation`, `analysis` and the measuring core (`bounding_volume`, `classification`, `mass_properties`, which close over each other).
  - The internal graph was already a DAG apart from that core cycle.
  - Analysis needed `faces_moments`, `area_from_moments`, `area_centroid_from_moments`, `mass_from_moments`, `solids_overlap`, `surface_uv` and `is_point_edge` made `pub`.
- **representation** split into `polynomial`, `vector`, `tolerance`, `curve`, `surface` and topology (`arena`, `contract`, `error`, `topology`).
  - The only real cycle was curve → surface. It is broken by moving the p-curve region (`impl Surface { project_curve… }`, `fit_pcurve`, `seam_crossings`) and the surface-fit region (`interpolate_surface_grid`, `coons_patch_nurbs`) from `curve`/`curve_ops` into `surface`/`surface_ops`, together with their tests (`🏄️surface/🧪️tests/🔬️pcurve`, `🪡️surface-ops/🧪️tests/🔬️fit`).
  - `averaged_knot_vector`, `solve_interpolation_1d` and `eval_nurbs_curve3` became `pub`.
  - `retire_geometry_variants!` is now exported from `vector`, the lowest geometry crate.
- **Base crate.** It now depends on `vector` only, not on representation. The peer feature lane (PROCEDURAL geometry-production) is editing `🥽️mesh` and `⚙️engine/🪅️shape-value` concurrently. I only retargeted paths there; two engine imports had to follow the cut (`analysis` alias, `vector::matrix::Frame3`).

Crate DAG:

```
polynomial ← vector ← curve ← surface ← representation ← intersect ← queries ← validation ← tessellation ← analysis
tolerance ←───────────────────────────────────────────── (representation, queries, operations, engine)
vector ← semio-framework-3d (mesh, rigid, inertia, collision)
queries/intersect/tessellation/validation ← operations ← engine
```

### 3.3 Admission measurement

Tool: `📏️source-admission.ts`, which drives the canonical `readRustSourceClosureV1` with the native `rustc --print cfg`, plus `test` for the test closure.

| Unit | Prod files / lines / bytes | Test files / lines / bytes | Verdict |
|---|---|---|---|
| brep-polynomial | 1 / 299 / 13,056 | 2 / 495 / 20,859 | admitted |
| brep-vector | 3 / 1,316 / 62,909 | 6 / 1,894 / 89,552 | admitted |
| brep-tolerance | 1 / 180 / 8,140 | 2 / 263 / 10,891 | admitted |
| brep-curve | 4 / 2,251 / 113,339 | 9 / 3,359 / 167,724 | admitted |
| brep-surface | 2 / 1,392 / 77,195 | 8 / 2,145 / 115,507 | admitted |
| brep-representation | 6 / 1,930 / 107,674 | 14 / 2,443 / 142,709 | admitted |
| brep-intersect | 5 / 2,418 / 114,423 | 9 / 2,941 / 137,988 | admitted |
| brep-queries | 4 / 2,934 / 142,598 | 11 / 3,726 / 183,728 | admitted |
| brep-tessellation | 1 / 1,346 / 68,533 | 4 / 2,020 / 120,570 | admitted |
| brep-validation | 4 / 1,084 / 74,640 | 7 / 1,685 / 127,651 | admitted |
| brep-analysis | 8 / 1,164 / 52,139 | 10 / 1,552 / 80,835 | admitted |
| brep-engine | 8 / 3,934 / 272,011 | 23 / 6,142 / 465,914 | REFUSED (bytes, test lines) |
| brep-operations | 17 / 11,021 / 607,364 | 35 / 13,429 / 878,096 | REFUSED |
| semio-framework-3d | 20 / 6,932 / 514,364 | 46 / 10,460 / 1,068,190 | REFUSED |

### 3.4 Unit peaks (wasm-release, each unit rebuilt alone, 2 s ps sampler)

- **New crates.** representation 0.590 GB, analysis 0.335 GB, operations 0.344 GB, engine 0.409 GB, base 0.715 GB. Curve, vector, surface, polynomial and queries were all ≤ 0.22 GB. Intersect, tolerance, tessellation and validation finished under the sampler's 2 s resolution.
- **Before the cuts:** 1.643 GB for the uncut crate.

### 3.5 Verification (run)

- **Geometry crates.** Native `--lib --tests` and lib tests ran in the real workspace: polynomial, vector, tolerance, curve, surface. Their closure has no `job`.
- **All 14 crates.** Native `--lib --tests`: 0 errors. `--lib --target wasm32-wasip2`: 0 errors.
  - These ran in a scratch workspace (`⚡️cache/e4` build dirs) that symlinks the real framework tree and swaps only the HEAD copy of `semio-framework-job`. The working-tree `job` has a borrow error (`👶️child/🦀️.rs:13`, another lane, unchanged since 16:01).
  - 3d-owned warnings are the same set as HEAD, plus nothing new. Two new warnings were fixed: an unused extern crate, and the test support in queries.
- **Lib tests, one process per test** (8 in parallel, 300 s alarm): **733 tests: 725 ok, 4 FAILED, 4 TIMEOUT.**
  - Against the 10-13 run: the same 720 tests with the same verdicts, except that peer fixes turned 17 mesh failures to ok.
  - 6 tests are renamed (the p-curve and fit tests moved to surface; all ok), and the peer added 13 new tests.
  - The 3 engine timeouts pass alone: 154 s, 212 s and 227 s.
- **Consumers (10-13, real workspace).** `--lib` was green for generation3d, the flow brep extension, process3d, cad, step, the spatial session and the stdio semio brep crates. Their test trees were already red for other reasons, with 0 errors mentioning 3d crates.
  - 10-14 consumer manifests are synced (`🔗️sync-crate-deps.py`), but the consumers could not be compiled: plugin and os-kernel are red.
- **Language-neutral law.** `✏️s/🧑‍💻dev/📐️cad` `geometryOwnershipLaws()` passes. Its fixture follows the moves (`intersect` under queries, `kernelPackage` = brep-engine).

## 4. BIM model: step 3 so far (source only)

Done:

- **Root barrels removed.** These were `pub use …snapshot::*`, `…diff::patches::*`, the `diff` names, `ModelInference`, `ModelMutation`, and the `pub mod mutations` shim. **2,082 path occurrences** were rewritten in place to the canonical module paths (`🚚️bim-step3-moves.json`, `🏷️module-exports.py` for the snapshot and patches name sets), covering the crate, its 36 test hosts, the hub composition and the bridge.
- **Schema identity moved into the model.** `BIM_MODEL_DOCUMENT_SCHEMA`, `BIM_MODEL_DIALECT` and `artifact_kind()` moved from the crate root to the schema root (`🧬️schema/🦀️.rs`). The hub composition calls `…::schema::artifact_kind()`.

Remaining facet edges against the target DAG, from the path-resolved census (`🕸️bim-layers.json`). These need moves before the cut:

| Edge | Exact source → target | Fix |
|---|---|---|
| read → io (9), write → io (5) | `📥️import/🏗️ifc/*` and `📤️export/{csv,solver,energy,sheets}` → `🚪️io/🦀️.rs` `with_inferred`, `io::text` | Move `with_inferred` and the native text/binary codecs into a model-level `io-base` crate. The `io()` declaration (which composes read and write) moves to the subset aggregator. |
| inferences → mutations (2) | `💡️inferences/🧭️option-scope` → `mutations::option_rules::{element_ids, host}`; `🕸️model-graph/🧳️instance` → mutations | Move `option_rules` (pure snapshot queries) into the model. Split the model-graph instance session (it is a mutating session) into the mutations side. |
| inferences → diff (2) | `🕸️model-graph/{📡️session,🧳️instance}` → `🔺️diff` | Same as the line above. |
| mutations → inferences (1 prod) | `🦉️wall-geometry` line 255 → `inferences::wall_layout` | Invert: the wall-layout primitive it needs moves to the model. |
| render (`🖌️render`) → inferences, root; editor → render | render is a viewer-side projection | Put render in the editor/viewer crates. |
| editor → root (4) | `bim_window_config!` (a `macro_export`) and `BimApplication` | They go with the editor crate. |

## 5. Cut plans against the 4,000 / 262,144 / 32 ceiling

### 5.1 framework-3d: the three remaining units

| Unit | Plan |
|---|---|
| **brep-operations** (11.0 k) | Internal cycles: `staged ↔ boolean`, `euler → boolean`, `euler/primitives/sew → validation` (allowed) and `offset ↔ blend`. Cut `construction` (euler, primitives, staged, transform, sew: about 2.9 k + tests) after moving the boolean-backed helpers euler and staged use into `boolean`. The other units are `boolean` (3.3 k), `sweep` (1.6 k) and `offset+blend` (3.2 k). Tests move with their subject; fixtures come through dev-dependencies on construction. |
| **brep-engine** (3.9 k lines, but 272 KB and 6.1 k test lines) | Split `operation-jobs` + `retention` + `shape-value` (one unit) from the `Brep` session root. Move the 2.2 k-line engine test tree with them. This waits for the peer lane's `shape-value` edits. |
| **semio-framework-3d** (6.9 k / 514 KB) | `rigid` + `inertia` + `collision` (1.1 k) as one unit. Mesh core, `mesh::modeling` (2.3 k) and `mesh::tessellation` become separate units once the peer lane's in-flight mesh edits land. Do not cut under it. |

### 5.2 architect program

- **What is ready.** The 6-crate L6 split is ready to apply once upstream is green: model 10.3 k, mutations 20.2 k, inferences 2.2 k, io 2.4 k, editor 6.0 k, aggregator. Step 3 left 0 forbidden edges. `✂️split-root.py` writes each crate root with rebased `#[path]`s, and the module paths stay `standards::v1::subsets::any::…`.
- **What blocks the ceiling.** The ceiling (and the 32-file limit in particular) cannot be met by a cut alone.
  - Each of the 260 mutation leaves implements `MutationKind<ProgramSnapshot, ProgramMutation>`. Its `inverse` builds `ProgramMutation`, so the enum and its leaves form one SCC of 1,140 files and 20 k lines.
  - Two options exist:
    - (a) Leaves keep only their structs and `diff`. The `MutationKind` impls and inverses live in the enum crate. This is orphan-legal, because `ProgramMutation` is a local type parameter. That gives per-register-group leaf crates of at most 32 files. The enum crate still holds 260 impls (about 9 k lines), which is over the ceiling.
    - (b) One generic register mutation, `Create/Delete/Rename/Replace<R: Register>`, generated from schema. It replaces 260 hand-written triads, and the enum crate shrinks below 1 k.
  - Option (b) is the only one that fits the ceiling. It is a framework and wire decision (the mutation kind ids must stay stable). I did not do it in E4.
- **Further splits needed.** The model's `registers` file (7.7 k) must be split by register group. The editor needs about 3 units.

### 5.3 BIM model

After §4, the target is about 40 units:

| Facet | Units | Basis |
|---|---|---|
| model | 2 | snapshot (entities, values, anchors, views, sheets, conditions, coordination, structure), authored |
| diff | 2 | patches, root |
| io-base | 1 | |
| read | 1 | ifc import |
| write | 3 | ifc, energy/gbxml, sheets/svg/pdf/gltf |
| inferences | 6 | by topic: element-solids, model-graph, diagnostics, layout/linework, energy, quantities/schedules |
| editor + viewer + render | 6 | |
| mutations | | Same SCC problem as architect: `ModelMutation` ↔ leaves, 2,253 files. Needs the generic or generated mutation decision of §5.2. |
| aggregator | 1 | |

## 6. Gates I could not run, and why

- **architect and BIM.** The native and wasm `--lib` checks, the lib tests, and the hub wasm workspace check were blocked: os-kernel and plugin were red all of 10-14, and the HEAD-tree substitution failed because HEAD's own `ui-styling` is broken.
- **`🔎️manifest-sweep.sh`.** It compiles every plugin manifest, so it would fail on the same upstream crates.
- **`bun repo run os-hub:unit-census-check`** (the plain `bun nx run` now refuses: "Original immutable build graph path and digest required"). The repository runner failed in `build:dependencies` with `bun install --frozen-lockfile`: "lockfile had changes, but lockfile is frozen". That is infrastructure owned by other lanes. The log is in `🗑️generated/e4/unit-census.log`.

## 7. Files touched

- **framework-3d.**
  - New manifests: `📐️brep/{📸️representation,💡️queries,🛠️operations,⚙️engine}/📦️packages/🦀️rust/Cargo.toml`, `📐️brep/📸️representation/{〰️polynomial,➡️vector,📏️tolerance,➰️curve,🏄️surface}/📦️packages/🦀️rust/Cargo.toml` and `📐️brep/💡️queries/{✂️intersect,🧩tessellation,✅validation,🔎️analysis}/📦️packages/🦀️rust/Cargo.toml`.
  - Crate roots: the facet and module `🦀️.rs` files of the above.
  - Base crate: `📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📜️script.ts}`. `📐️brep/🦀️.rs` was deleted.
  - Moves: `🔖️contract` (into representation), `✂️intersect` (into queries), the p-curve and surface-fit regions, and the tests named in §3.
  - New test support: `🧪️tests/🧰️support/🦀️.rs`.
  - Path retargets across `📐️brep/**` and `🥽️mesh/🛠️modeling/🦀️.rs`.
- **Workspaces.** `🧰️framework/Cargo.toml` (13 members and rows). Rows in the generation3d, step and stdio semio workspace roots.
- **Consumers (manifests and paths).** generation3d, cad, process3d, step, the stdio semio brep crates and their io crates, the flow brep extension, the spatial session, the hub stdio bench file, and `✏️s/🧑‍💻dev/📐️cad/🧫️fixtures/🌐️geometry-ownership/🔣️.json`.
- **Root `📜️script.ts`.** The plugin framework dependency allowlist lists the new crates. In `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`, the deletion-proof package list now names the new crates.
- **architect.**
  - Root `🦀️.rs`: shims and barrels removed. Composition, constructors and identity moved into the model (`🧬️schema/🦀️.rs` and `📸️snapshot/🦀️.rs`). Register exchange import moved into `🚪️io/📊️tables/🦀️.rs`.
  - Canonical paths across the crate, its test hosts, the hub composition (`🌎️hub/🧩️compositions/🏛️architect/🦀️.rs` and the package root) and the bridge.
- **BIM.** Root `🦀️.rs`, `🧬️schema/🦀️.rs`, and about 1,100 files with canonical paths (crate, test hosts, `🌎️hub/🧩️compositions/🏙️bim/🦀️.rs`).
- **Ticket inputs (kept), `e4-3d-architect-bim/`.**
  - Build and measurement: `🏗️cargo.sh`, `📏️unit-measure.sh`, `📏️scratch-unit-measure.sh`, `📏️source-admission.ts`, `🧪️lib-tests.sh`, `🧪️each-test.sh`, `✅️check-manifests.sh`, `⏳️await-upstream.sh`.
  - Graphs and codemods: `🕸️item-graph.py`, `🧲️impl-homes.py`, `🚚️retarget-paths.py`, `🏷️module-exports.py`, `🔗️sync-crate-deps.py`, `✂️split-root.py`, `🔎️taxonomy-kinds.ts`.
  - Specs and layer rules: `🗺️*-facets.json`, `🕸️*-layers.json`, `🚚️*-moves.json`, `🔗️3d-family.json`, `✂️architect-split.json`.
- **Generated (mine only):** `🗑️generated/e4/`. **Private cache:** `⚡️cache/e4/` is deleted.

## 8. Corrections entry

Appended to `📓️2026-10-10-migration-corrections.md` as `wave-e4`.
