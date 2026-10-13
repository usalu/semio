# 🧿️ Stdio Semio Crate Split Plan (2026-10-13)

This is a read-only analysis with an executable plan. No source was edited. Cold release build #10 was running during the analysis.

The plan restructures `semio-s-artifact-stdio-semio` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`) so that every crate compiles exactly once per profile, no single unit dominates memory, and runtime behaviour stays the same.

Ticket inputs I created (kept as input files):

- `stdio-semio-split/📏️measure.sh` measures the crate cold at wasm-dev. It uses a private target/build dir, runs under `🚦️cargo-slot.sh`, and records `/usr/bin/time -l`, `--timings`, and per-rustc peak RSS.
- `stdio-semio-split/🌳️closure-census.py` counts distinct compile units from `cargo tree` under the prebuild's per-package feature unification. It does not compile anything.

The private dir `.🧬semio/🦑️repo/⚡️cache/split` was deleted after the run.

---

## 1. Summary

**Root cause, measured.** In the current hub prebuild plan (`fChPzBn/p.json`, 44 packages), 34 packages reach `semio-s-artifact-stdio-semio`. Per profile and target they produce **16 distinct units** of the monolith. Two things drive that number:

- **8 distinct `conversion-*` / `component-app-assembly` feature sets.**
- **Upstream feature variance in three crates of its closure:**
  - `semio-framework-2d`: 3 feature sets.
  - `bytemuck`: 3 feature sets.
  - `syn` (host): 2 feature sets.

Removing the features alone, by splitting the crate, still leaves **7** units per new crate. Normalizing the upstream features alone leaves **9** units of the monolith. Only **both together give 1 unit per crate per profile.** Both are therefore part of this plan:

- **Wave 0:** upstream normalization.
- **Waves 1–4:** the split itself.

**Target shape.** The monolith becomes 47 feature-free crates:

| Group | Count | What it holds |
|---|---:|---|
| `semio-s-artifact-stdio-semio-base` | 1 | Foundation vocabulary and codec primitives |
| `semio-s-artifact-stdio-semio-<subset>` | 18 | Schema, io core, editor and viewer of one subset |
| `semio-s-artifact-stdio-semio-<subset>-import` / `-export` | 26 (13 subsets × 2) | Conversions, depending on the subset crate plus their format crates |
| `semio-s-artifact-stdio-semio` | 1 | Aggregator: envelope union, `SemioMembers`, declaration, register |
| `semio-s-artifact-stdio-semio-catalog` | 1 | Full-catalog contribution = aggregator + all conversions |

**Expected effect** (details in §5):

| Metric | Today | After |
|---|---|---|
| Semio-family CPU per wasm-release profile | ≈ 16 × 520 s ≈ **2.3 CPU-h** | ≈ **12–14 CPU-min** |
| Semio-family CPU per wasm-dev profile | ≈ 37 CPU-min | ≈ 4–5 CPU-min |
| Largest semio unit at wasm-dev | **11.8 GB** (all features) / 4.3 GB (default) | ≈ **1.1–1.5 GB** |
| Largest semio unit at release | 14–20 GiB | ≈ 2–2.5 GiB |
| Semio-family critical path, release | ≈ 520 s | ≈ 140–170 s |
| Prebuild memory budget | `bytesPerJob` 10 GiB / `heavyUnitBytes` 20 GiB | ≈ 4 / 8 GiB |
| Prebuild jobs on the 10-core / 32 GB machine | 2 | about 6 |

- The largest unit after the split is either `-table` or one of the conversion crates.
- The new memory budget is to be confirmed by re-measuring with `📈️rustc-peak-sampler.sh`. The heavy units then become `semio-s-artifact-stdio-dwg` (6.9 GB at wasm-dev) and the linked hub components (5–7 GiB).

---

## 2. Measurements

### 2.1 Cold wasm-dev builds of the semio crate

Setup:

- Workspace: the standalone semio workspace.
- Target and profile: `--target wasm32-wasip2 --profile wasm-dev`.
- Jobs: `--jobs 4`.
- Dirs: private `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR`.
- Machine state: shared with build #10, which ran about 3 rustc processes at ~340 % CPU the whole time.

| Run | Semio unit wall | Semio peak RSS (sampler / `time -l` max) | Cargo wall | Units built |
|---|---:|---:|---:|---:|
| default features | 157.7 s | 3.91 GB / 4.33 GB | 353 s (incl. 129 dependency units) | 130 |
| `--all-features` (incremental over the default deps) | 125.3 s | **10.70 GB / 11.76 GB** | 260 s | +44 (the 29 format crates + new semio) |

How to read these numbers:

- **Time does not grow with features. Memory does.** The conversion leaves are ≈ 8.7 k lines (+9 % code), but they add +7.4 GB (×2.7) of peak RSS.
- The two time samples are noisy because of the shared machine. I use ≈ 140 s as the wasm-dev cost per unit.
- Release reference (existing `⚡️cache/py/t/cargo-timings`, a `semio-hub-process` wasm-release build at jobs=10):

  | Unit | Release time |
  |---|---:|
  | `semio-s-artifact-stdio-semio` (default) | **519.5 s** (the longest unit, 2.2× the next) |
  | `stdio-dwg` | 240.9 s |
  | `stdio-gltf` | 159.9 s |
  | `os-kernel` | 115.4 s |

- Build #10's live peak sampler showed the semio unit at 6.34 GB. The lease docstrings already state 14–20 GiB at both dev and release.
- Other heavy units seen in the all-features run: `stdio-dwg` 6.90 GB, `stdio-pdf` 3.63 GB, `stdio-gltf` 3.53 GB, `semio-framework-plugin` 3.45 GB, `os-kernel` 2.86 GB.

### 2.2 Unit census

Method: `🌳️closure-census.py` runs `cargo tree -e normal --target wasm32-wasip2` per planned package with `CARGO_RESOLVER_FEATURE_UNIFICATION=package`, which matches the prebuild.

| Scenario | Distinct semio units per profile |
|---|---:|
| Today (features + upstream variance) | **16** |
| Normalize upstream only (2d, bytemuck, syn) | 9 |
| …plus `component-app-assembly` made unconditional on stdio format crates | 8 |
| Split only (no semio features, upstream unchanged) | **7 per new crate** (≈ 7× the whole family) |
| Split + normalize 2d + bytemuck | 2 |
| **Split + normalize 2d + bytemuck + syn** | **1** |

Today's 16 units by feature set:

| Units | Feature set |
|---:|---|
| 2 | none |
| 1 | none + apps |
| 4 | drawing |
| 2 | drawing + mesh |
| 2 | brep + drawing + mesh |
| 1 | drawing + image |
| 1 | drawing + image + mesh |
| 3 | all 13 groups (`semio-hub-stdio`, `-gis`, `-vcs`) |

A second sample from the shared build-dir agrees. The `wasm32-wasip2/debug` directory holds 21 semio units, with 7 feature sets but 13 distinct dependency closures.

Where the upstream variance comes from (who requests the extra features):

- **`semio-framework-2d`** has three feature sets: `''` (21 packages), `booleans,default,serde,trace` (12 packages) and `booleans,trace` (draw).
  - `serde` comes from `semio-framework-os-flow`.
  - `default` (= `booleans,trace`) comes from 2d's default features; the draw artifact enables `booleans,trace` explicitly.
- **`bytemuck`**: `extern_crate_alloc` comes from the external `image` crate (gis, demonstrator). `min_const_generics` comes from `font-types` (layout).
- **`syn`** (host proc-macro): `visit, visit-mut` come from `wasm-bindgen-macro-support` (demonstrator, puzzle).

This changes semio's unit hash because cargo's `-C metadata` hashes include every dependency unit's metadata. That includes proc-macro units, which is why `syn` matters.

---

## 3. The crate today

### 3.1 Shape

- **One lib.** `📦️packages/🦀️rust/Cargo.toml` has `lib.path = "../../🦀️.rs"`.
- **Root module file.** `🦀️.rs` is 4 805 lines. It contains:
  - 1 534 `#[path]` attributes forming the whole module tree.
  - The `retire_leaf!`/`retire_struct!` impls for all subset types.
  - The `member_owners!` impls (`SnapshotRetirementFactory`, `MemberStoreOwner`).
  - `space_members! SemioMembers`.
  - `declaration()` and `register()`.
  - `io_registry`.
  - `editor`/`viewer` module trees, gated by `component-app-assembly`.
- **Internal paths.** 5 452 internal `crate::standards::v1::subsets::…` references.

### 3.2 Size per subset (Rust lines, excluding `🏭️generator`, `🔬️probes` and nested packages)

| Subset | Schema | io core | Conversions (import / export) | Editor + viewer | Tests |
|---|---:|---:|---:|---:|---:|
| table | 12 202 | 1 362 | 0 | 700 | 6 589 |
| graph | 5 676 | 2 053 | 0 | 710 | 5 216 |
| value | 4 149 | 1 922 | 253 / 489 | 778 | 2 664 |
| drawing | 3 495 | 2 582 | 657 / 1 049 | 684 | 3 947 |
| brep | 3 149 | 2 961 | 418 / 292 | 917 | 4 538 |
| document | 3 086 | 2 639 | 282 / 326 | 820 | 1 785 |
| mesh | 2 860 | 2 314 | 651 / 881 | 980 | 3 958 |
| ✉️base | 2 836 | 1 964 | 0 | 1 073 | 2 125 |
| presentation | 2 691 | 2 313 | 93 / 101 | 701 | 1 795 |
| model | 2 246 | 2 531 | 366 / 437 | 689 | 1 638 |
| kit | 2 004 | 1 906 | 0 | 727 | 2 695 |
| cad | 1 982 | 2 066 | 349 / 272 | 688 | 1 425 |
| flow | 1 980 | 2 136 | 105 / 81 | 694 | 1 949 |
| image | 1 904 | 1 760 | 297 / 315 | 695 | 2 555 |
| animation | 1 875 | 1 916 | 218 / 273 | 696 | 1 434 |
| video | 1 525 | 1 496 | 119 / 160 | 704 | 1 501 |
| audio | 1 457 | 1 573 | 136 / 98 | 669 | 1 310 |
| object | 1 304 | 1 352 | 0 | 684 | 1 969 |
| text | 1 170 | 1 363 | 0 | 684 | 1 615 |
| **Total** | **57 591** | **38 209** | **8 718** | **14 293** | |

The default build compiles ≈ 101 k lines: schema + io core + root. All features plus apps compiles ≈ 124 k.

### 3.3 Dependencies between subsets

Method: `rg` over every `subsets::<x>::` path, classified by file role. Most cross-subset edges are **misplaced generic helpers**, not domain dependencies:

- **`audio::io::text::diff::{strip_brackets, split_top_level}`** (113 uses). Duplicates of ✉️base's own copies.
- **`drawing::io::text::snapshot`** (162 uses):
  - `enc_str`/`dec_str`, `encode_option`/`decode_option`, `enc_list`/`dec_list`, `hex_encode`/`hex_decode`, `enc_point2/3`, `enc_bool`, `parse_f64`, `enc_transform`.
  - Also `drawing::io::binary::snapshot::{write_str_lp, read_str_lp, read_transform}`.
- **Small codec helpers in other subsets:**
  - `document::io::text::mutations::parse_usize`
  - `mesh::io::text::snapshot::parse_u32`
  - `flow::…::{enc_f64, dec_f64}`
  - `video::…::dec_bool`
  - `model::io::text::diff::{enc_transform, dec_transform, enc_list}`
  - `mesh::…::{enc_rgba, dec_rgba}`
  - `cad::…::{enc_point2, dec_point2}`
- **`ArtifactChild` codecs, duplicated in object and kit.** They form an object↔kit **cycle**:
  - text: `object|kit::io::text::snapshot::{enc_child, dec_child, enc_child_opt}`
  - binary: `object|kit::io::binary::snapshot::{write_child, read_child}`
  - sqlite: `object::io::sqlite::snapshot::{native_child, native_reference}`
- **Dead import.** `audio → animation::io::text::snapshot::{enc_channel, dec_channel}` sits under `#[allow(unused_imports)]`. Audio has its own `enc_channel`.

**Real domain edges that remain** after the helpers move to `-base`:

- `value ← table`
- `value ← graph`
- `{brep, mesh, value, model} ← object`
- `{object, model, value} ← kit`
- `document ← presentation` (`enc_block`, `native_blocks`, `DocBlock`)
- Conversion-level: `drawing-export ← mesh-export` (mesh → png uses the drawing png serializer).
- Union: `✉️base` (the envelope union `SemioSnapshot` / `SemioDiff` / `SemioMutation`, its io, its sqlite and `SemioAny*` apps) depends on **all 18 subsets**.

✉️base also holds the foundation vocabulary that every subset uses:

- `🧬️schema/{🧮️geometry, 🧰️triples, 🪆️child}`.
- `semio_mutation_refusals`/`_codes`.
- `🚪️io/🪶️sqlite/📸️snapshot/{🛬️/🛫️/📏️native, 🫙️projection, 🧮️semantic, 💰️reconstruction}`, used as `native_decoding` (82 uses) and `native_encoding` (16).
- The text triple codecs in `🚪️io/📝️text/📸️snapshot` (`split_top_level` … `dec_named_added`, lines 216–331).
- `✏️editor/🧭️edit-plumbing`, `🧭️edit-rules`, and `📚️examples` (used by every editor).

So ✉️base must be split into a **foundation** (`-base` crate) and the **union** (aggregator).

### 3.4 What `conversion-*` gates

There are 179 sites in 25 files.

- **Module trees.** The root `🦀️.rs` gates the `pub mod import { deserializers … }` and `pub mod export { serializers … }` trees for brep, cad, document, drawing, image, mesh, model, value, presentation and flow. Video, audio and animation declare their conversion `mod`s inside their own `🚪️io/🦀️.rs`.
- **Inside each subset's `🚪️io/🦀️.rs`:**
  - The `IoBridgeImports` and `io_bridge_entries()` vectors (`deserializer_entry_of` / `serializer_entry_of`).
  - The `#[cfg] register_composer_entries(...)` call in `register()`.
  - The `#[cfg] builder.composers(crate::semio_written(...))` call in `declare()`.
  - The public `SemioMeshFormat` + `encode_mesh` / `decode_mesh` and `SemioDrawingFormat` + `encode_drawing` / `decode_drawing` APIs.
  - The inline `brep::io::dwg::{import, export}`.
  - The `derived-composition-unit` tests.
- **`🧬️schema/🔺️diff`, 12 sites:** test-only demo snapshots (`#[cfg(all(test, feature = …))]`).
- **Format crates.** Every `conversion-<group>-<format>` sub-feature only enables an optional format crate.
  - Consumers use group features only.
  - `conversion-mesh` implies `conversion-drawing`.
  - The video sub-features `-avi` / `-mp4` are referenced in code.
- **Generated, not hand-written.** The `# 🧩️ Capability Features` / `Capability Dependencies` regions come from `🧩️composition/🔗️conversions/🔣️.json` through `🔗️conversions/🟦️.ts` → `prepareCargoCapabilityLinksV1`. The same generator writes `📜️artifact-definition.json` `dependencies` (all 29 format kinds, always).

### 3.5 How composers get registered

- **Per subset,** `🚪️io/🦀️.rs` has a `derived_composition` module containing:
  - `Semio<X>ComposerComposition` (an `ArtifactComposition` over the subset's own dialect).
  - `Semio<X>Validator`.
  - `register()`: schema descriptor, native document codec, subset validator, the conversion `register_composer_entries` (gated), and the inference descriptor.
  - `declare(builder)`: schemas, `document_codec_bare`, validators, inferences, and the gated `composers(semio_written(io_bridge_entries()))`.
  - `semio_written` keeps only the rows whose `writes.artifact_kind == "s.stdio.semio"`, i.e. the deserializers. Export rows (semio → X) are reachable only through `register()`.
- **At the root:**
  - `declaration(definition)` chains base + 18 `declare`.
  - `register()` chains 19 `register`.
  - `io_registry` wraps ✉️base's `entries()`.
- **Who calls them:**
  - **`declaration`:** `semio-hub-stdio` (through the generated `selected_contributions()` → `contribution()`, all features) and the semio extension (`host_artifact(declaration(definition()))`, feature set `component-app-assembly` only, so **no** conversion composers).
  - **`register()`:** raster (`ensure_stdio_semio_and_png_registered`, production) and remodel (production). gismap calls it only in tests. layout (test) and shooting (production) call `drawing::io::register`.

### 3.6 `component-app-assembly`

This feature gates the `editor`/`viewer` trees for the 38 apps (19 editor + viewer pairs, ≈ 14 k lines). Only `semio-hub-stdio-semio` enables it, which costs one extra unit per profile.

### 3.7 Who consumes the crate

Source paths, grouped by consumer:

| Consumers | What they use from semio | Features today |
|---|---|---|
| 30 hub compositions | `SemioMembers` | none |
| `semio-hub-stdio-semio` | editors, viewers, `declaration` | `component-app-assembly` |
| `semio-hub-stdio` | `contribution()` | all 13 groups (`full-artifact-catalog`) |
| `semio-hub-raster` | — | image, drawing |

Plugin artifacts, by subset and conversion (io) use:

| Plugin artifact | Subsets used | Conversion use and feature |
|---|---|---|
| ✒️writer | document | — |
| ➗️equation | table, text, value | — |
| 🀄️wfc 2d / grid2d | image | — |
| 🀄️wfc 3d / grid3d | mesh | — |
| 🌀️generation2d | base, drawing | drawing io (`conversion-drawing`) |
| 🌀️generation3d | base, mesh | mesh io (`conversion-mesh`) |
| 🌊️flow | base, flow, `apply_diff` | — |
| flow 📐️brep extension | — | brep io (`conversion-brep`) |
| 🏔️gisterrain | base, mesh | mesh io (`conversion-mesh`) |
| 🗺️gismap | base, drawing, image, value; `register` (test) | drawing io (`conversion-drawing`) |
| 🎬️presentation | animation, base, document, presentation | — |
| 🎥️shooting | base, drawing, image | drawing io (`conversion-drawing`) |
| 🎬️sequence | base, flow | — |
| 🏗️fem 2d / 3d | base, mesh | mesh io (`conversion-mesh`) |
| 🏛️program | table, value | — |
| 🧊️process3d | base, brep, flow | — |
| 💠️lowpoly | base, mesh | mesh io (`conversion-mesh`) |
| 🔌️wires | base, graph, value | — |
| 📋️forms | base, table, value | — |
| 📏️layout | base, drawing | drawing io (`conversion-drawing`) |
| 📐️cad | base, brep, drawing, mesh, model | brep + mesh io (`conversion-brep`, `-mesh`) |
| ⚡️din18599 | table, value | — |
| 📖️playbook | base, flow | — |
| 📜️procedure | base, flow, text | — |
| 📸️remodeling | image, mesh; `register` | mesh io (`conversion-image`, `-mesh`) |
| 🔋️model | table, value | — |
| ♻️rewriting | base, graph | graph io |
| 🔌️jack | base, graph, value | — |
| 🕸️dag | base, graph, value | — |
| 🖍️drawing | base, drawing | drawing io (`conversion-drawing`) |
| 🖨️raster | base, drawing, image; `register` | drawing io (`conversion-image`, `-drawing`) |
| 🗒️note | base, drawing, text | drawing io (`conversion-drawing`) |
| 🧩️puzzle 2d | drawing | drawing io (`conversion-drawing`) |
| 🧩️puzzle 5d | drawing, kit | drawing io (`conversion-drawing`) |
| 🧱️block 3d | kit | — |
| 🗂️curation | base, kit | — |
| `🌎️hub/💡️inference/{🏃️runtime, 🧾️wal}` | drawing, value | — |
| `✏️s/🔨️modules/🏗️fem` | base, mesh | — |
| `✏️s/🧑‍💻dev/🧩️composition` | `SemioMembers` | — |

Scale of the consumer rewrite:

- **Source:** 349 files, 1 003 occurrences of `semio_s_artifact_stdio_semio::`. 253 of those are `SemioMembers`, and that path stays unchanged.
- **Manifests:** 44 Cargo workspace roots, plus the package manifests, mention the crate.
- **Lockfiles:** 87 `Cargo.lock` files contain it.

---

## 4. Target design

### 4.1 Crate graph

Package directory convention (`📦️packages/🦀️rust/Cargo.toml` at a semantic node, lib root = that node's `🦀️.rs`):

| Crate | Package dir | Lib root (new file unless noted) | Contents | Deps (semio) |
|---|---|---|---|---|
| `semio-s-artifact-stdio-semio-base` | `🪆️subsets/✉️base/📦️packages/🦀️rust` | `✉️base/🦀️.rs` (new) holds only the foundation tree | geometry, triples, child, `semio_mutation_refusals`/`_codes`, `native_output_tests`; sqlite native helpers (as `#[path]` into `✉️base/🚪️io/🪶️sqlite/📸️snapshot/{🛬️,🛫️,📏️native, 🫙️projection, 🧮️semantic, 💰️reconstruction}`); the consolidated text/binary codec primitives (§4.3); `ArtifactChild` codecs; `edit-plumbing`/`edit-rules`; `📚️examples`; `semio_written`; `SemioConversion`; the `semio_member_owner!` macro; the generic `applied` helper | — |
| `semio-s-artifact-stdio-semio-<subset>` ×18 | `🪆️subsets/<subset>/📦️packages/🦀️rust` | `🪆️subsets/<subset>/🦀️.rs` | `schema`, `io` (text/binary/sqlite codecs + `derived_composition` without bridges + `Semio<X>Format` enums), `editor`, `viewer` (unconditional), the subset's `retire_*!` and `semio_member_owner!` invocations | base + domain edges from §3.3 |
| `semio-s-artifact-stdio-semio-<subset>-import` ×13 | `🪆️subsets/<subset>/🚪️io/📥️import/📦️packages/🦀️rust` | `…/📥️import/🦀️.rs` | `deserializers::artifacts::…` leaves, `entries()`, `pub const CONVERSION: SemioConversion`, `decode_<subset>`, inline import halves (brep `dwg::import`) | subset (+ base) + its format crates (§4.6) |
| `semio-s-artifact-stdio-semio-<subset>-export` ×13 | `🪆️subsets/<subset>/🚪️io/📤️export/📦️packages/🦀️rust` | `…/📤️export/🦀️.rs` | `serializers::artifacts::…` leaves, `entries()`, `CONVERSION`, `encode_<subset>`, inline export halves (brep `dwg::export`) | subset + format crates; mesh-export → drawing-export |
| `semio-s-artifact-stdio-semio` (aggregator, existing name) | `🧿️semio/📦️packages/🦀️rust` (existing) | `🧿️semio/🦀️.rs` (existing, reduced) | ✉️base **union**: `SemioArtifact`, `SemioSnapshot`/`SemioDiff`/`SemioMutation` + `apply-*`, union text/binary/sqlite io (`#[path]` into ✉️base), base `derived_composition`, `SemioAny{Editor,Viewer}`; `space_members! SemioMembers`; `create/open_semio_member`; `definition`, `formats`, `artifact_kind`, `declaration(definition, &[SemioConversion])`, `register(&[SemioConversion])`, `composable_subsets`, `io_registry`; the 19 aggregator-dependent test files (§4.5) | base + 18 subsets |
| `semio-s-artifact-stdio-semio-catalog` | `🧿️semio/🧩️composition/📦️packages/🦀️rust` | `🧿️semio/🧩️composition/🦀️.rs` | `contribution()` = aggregator definition/formats + `declaration(definition, &ALL)` over all 26 `CONVERSION`s; `ALL` is generated (§4.6) | aggregator + 26 conversion crates |

Rules the split follows:

- **No crate keeps a feature,** apart from optional test-only dev features, which affect native test units only.
- **No crate re-exports another.** Consumers name the crate that owns the symbol.
  - The aggregator does **not** re-export subsets under `standards::v1::subsets`. Re-exporting there would be a compatibility facade, and it would serialize every consumer behind the aggregator.
- **`apply_diff` re-export removed.** The 8 call sites use `semio_framework_os_kernel::apply_diff` directly.

Module paths inside the crates:

| Crate | Path today | Path after |
|---|---|---|
| Subset crates | `crate::standards::v1::subsets::<s>::…` | `crate::…` |
| Subset crates | another subset's `…` | `semio_s_artifact_stdio_semio_<other>::…` |
| Subset crates | foundation items | `semio_s_artifact_stdio_semio_base::…` |
| Conversion crates | `…::<s>::io::import::deserializers::…` | `semio_s_artifact_stdio_semio_<s>_import::deserializers::…` |

Each new crate root replicates the extern-crate aliases the derives and codecs rely on (`dsl`, `protocol`, `store`, `framework_schema`, `value_derive`, `graph_core`). It also replicates `#![allow(async_fn_in_trait, long_running_const_eval)]`.

✉️base is the envelope subset, and it also holds the shared vocabulary. Its files are therefore split by ownership: `-base` is rooted at `✉️base/🦀️.rs`, and the aggregator reaches the union files through `#[path]`.

- A crate rooted under `✉️base/🧬️schema` would break the taxonomy rule "schema never owns representations or codecs", because `-base` also holds io helpers. That is why `-base` is rooted at `✉️base/🦀️.rs` instead.
- **Aggregator owns:** `✉️base/🧬️schema/{🦀️.rs (SemioArtifact + descriptor + shared schema documents), 📸️snapshot, 🔺️diff, 🧬️mutations}`, `✉️base/🚪️io/{🦀️.rs, 📝️text/{📸️snapshot, 🔺️diff, 🧬️mutations}, 💾️binary/{…}, 🪶️sqlite/📸️snapshot/🦀️.rs}`, `✉️base/✏️editor/🦀️.rs` and `✉️base/👁️viewer`.
- **`-base` owns:** `✉️base/🧬️schema/{🧮️geometry, 🧰️triples, 🪆️child, 💡️inferences}`, the sqlite helper leaves listed above, the new io codec leaves (§4.3), `✉️base/✏️editor/{🧭️edit-plumbing, 🧭️edit-rules}` and `✉️base/📚️examples`.
- **Taxonomy check required.** Precedent exists for packages deep inside `🪆️subsets` (`🎵️mp3/…/🧪️tests/📦️packages/🦀️rust`, `…/🏭️generator/🧩️json/📦️packages/🦀️rust`). The taxonomy gate must still confirm this placement (Wave 2 gate).

### 4.2 Public API changes (clean, no shims)

- **`SemioConversion` in `-base`:**

  ```rust
  pub struct SemioConversion {
      pub declare: fn(ArtifactDeclarationBuilder<DeclarationReady>) -> ArtifactDeclarationBuilder<DeclarationReady>,
      pub register: fn(),
  }
  ```

  - Every import/export crate exports `pub const CONVERSION: SemioConversion`.
  - `declare` applies `semio_written`, so export crates contribute no declaration rows. That is today's rule, kept in one place.
- **Aggregator:**
  - `declaration(definition, conversions: &[SemioConversion])` replaces `declaration(definition)`.
  - `register(conversions: &[SemioConversion])` replaces `register()`.
  - `assembly()` / `contribution()` move to `-catalog`, the only user.
- **Moved APIs:**
  - `encode_mesh` → `-mesh-export`; `decode_mesh` → `-mesh-import`.
  - `encode_drawing` → `-drawing-export`; `decode_drawing` → `-drawing-import`.
  - `SemioMeshFormat` / `SemioDrawingFormat` → the subset crates, since both directions use them.
  - `brep::io::dwg::{import, export}` → `-brep-import::dwg::import` / `-brep-export::dwg::export`.
- **Subset `register()` / `declare()`** lose the bridge rows. Conversions are added only through `CONVERSION`.

### 4.3 Moves forced by the orphan rule and the cycles (done inside the monolith first, Wave 1)

1. **Codec primitives → `-base`.** Consolidate them into one foundation module, and delete the duplicates in audio, drawing, document, mesh, model, cad, flow and video. The module holds:
   - from ✉️base: `split_top_level`, `strip_brackets`, `enc_opt`/`dec_opt`, `enc_/dec_indexed_triple`, `enc_/dec_named_triple`, `enc_/dec_named_added`, `hex_*`;
   - from drawing: `enc_str`/`dec_str`, `encode_/decode_option`, `enc_/dec_list`, `enc_/dec_point2/3`, `enc_/dec_transform`, `enc_/dec_rgba`, `enc_bool`/`dec_bool`, `parse_u32`/`parse_usize`/`parse_f64`, `enc_/dec_f64`, `write_/read_str_lp`, `read_transform`.

   The proposed homes are io-owned, because schema never owns codecs: `✉️base/🚪️io/📝️text/🔤️codec/🦀️.rs` and `✉️base/🚪️io/💾️binary/🔤️codec/🦀️.rs`. The facet slug is to be admitted by the taxonomy gate. The text-triple half of `✉️base/🚪️io/📝️text/📸️snapshot/🦀️.rs` (lines 216–331) moves to the text codec leaf. The union DSL code stays.
2. **`ArtifactChild` codecs → `-base`** (the text, binary and sqlite variants from object and kit). This breaks the object↔kit cycle.
3. **Delete the unused `animation` import in audio's `mutations_codec`.**
4. **Snapshot and mutation retirement impls → each subset's lib root.** These are the `retire_leaf!`/`retire_struct!` lines and the `member_owners!` body (`impl dsl::SnapshotRetirementFactory<S> for …Factory<S>`, `impl dsl::MemberStoreOwner<M> for S`). A foreign trait cannot be implemented for a foreign type in the aggregator. `-base` provides `semio_member_owner!(snapshot, mutation[, open = <decoder>])`. Flow passes `SemioFlowSnapshotDecode`, which replaces `semio_snapshot_open!`. The `SemioSnapshotRetirementFactory` / `SemioMutationRetirementFactory` / `SemioOwnedValueRetirementFactory` structs move to `-base`.
5. **`retire_struct!(drawing::schema::diff::NodePath)` and `document_mutation::DocBlockPath`** move into drawing and document respectively.
6. **`crate::applied` (test helper) → `-base` as a plain `pub fn`** (3 lines, no feature). `snapshot_test_backing` is included via `#[cfg(test)] #[path]` only in the crates whose tests use it.

### 4.4 Editors and viewers

`component-app-assembly` is deleted.

- **Subset apps.** Each subset crate compiles its `✏️editor` / `👁️viewer` unconditionally. Unregistered apps are dead code in every component but `semio-hub-stdio-semio`. Today that crate already gets them as a separate unit.
- **Base apps.** `SemioAny{Editor,Viewer}` go to the aggregator.
- **Example dependency.** Editors' `crate::examples::demo` becomes `semio_s_artifact_stdio_semio_base::examples::demo`.

The same rule should be applied to the stdio format crates that still carry `component-app-assembly`: txt, xml, json, csv, md, tsv and html. Today they compile twice per profile, and they sit in the closure of the value, document, drawing and presentation conversion crates. This is Wave 0, U5.

### 4.5 Tests

- **Integration tests move** with their subset: the 9 `[[test]]` targets (brep ×6, drawing original prefix, value path frames, flow ×2) and `native_prefix` (✉️base sqlite, foundation helpers).
- **Dev-dependencies are distributed:**
  - brep: `parry3d`, `num-rational`.
  - drawing-export: `resvg`, `hayro`.
  - value/table: stdio `csv`, `json`.
  - Plus `os-kernel` `protocol-laws` / `mutation-testing`, `plugin` `artifact-app-testing`, `contract` `testing`, `trace`.
- **19 subset test files** call `crate::declaration(crate::definition())`. These are the per-subset `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests`. The files stay in place and are compiled by the aggregator's `#[cfg(test)]` tree (`#[path]`), because they test the assembled artifact.
- **Root tests.** `🧪️tests/🔬️unit` and `🧪️tests/🪶️sqlite/🛬️native` stay in the aggregator. `preparation_law` drops `required-features` and moves to `-catalog`.
- **Feature-gated tests.** The `#[cfg(all(test, feature = "conversion-*"))]` demo snapshots in `🔺️diff` and the `derived-composition-unit` includes move to the owning import/export crate as plain `#[cfg(test)]`.

### 4.6 Behaviour equivalence (per consumer)

Mapping rule: a consumer's group set G (closed under mesh ⇒ drawing) becomes direct deps on the subset crates it names. It also gets the import/export crates whose symbols it uses. Registration is explicit and equal to what the call site needs today.

| Call site | Today | After |
|---|---|---|
| semio extension `host_artifact` | `declaration(def)`, no conversions (closure has only `component-app-assembly`) | `semio::declaration(def, &[])` |
| hub stdio catalog | `contribution()`, all 13 groups → import rows | `semio_s_artifact_stdio_semio_catalog::contribution()` with `declaration(def, &ALL)` |
| raster `ensure_stdio_semio_and_png_registered` | `register()` under closure {drawing, image} | `semio::register(&[drawing_import, drawing_export, image_import, image_export])` |
| remodel, same helper | `register()` under {drawing, image, mesh} | `register(&[image_*, mesh_*, drawing_*])` |
| gismap (test) | `register()` | `register(&[drawing_*])` |
| shooting (prod), layout (test) | `drawing::io::register` (incl. drawing bridges) | `semio_s_artifact_stdio_semio_drawing::io::register()` + `drawing_import::CONVERSION.register` + `drawing_export::CONVERSION.register` |

- Nothing else calls `declaration` or `register` in production. The remaining conversion-feature users call `encode_*`, `decode_*` or leaf types directly. These are generation2d/3d, cad, brep-ext, lowpoly, fem, draw, note, puzzle, gis and the raster hub.
- `semio-hub-raster` enables conversion features but uses only `SemioMembers`, so it drops them.
- **Behaviour result:** registered and declared composer sets per component are identical in production.

### 4.7 Generators and contracts that enumerate crates

- **`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧩️composition/{🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts}`** (the `CompositionContributionV1` closed shape):
  - Replace `catalogFeatures: string[]` with `catalogPackage: string | null`. A non-null value names the crate whose `contribution()` the full catalog selects and depends on.
  - Add `symbolPackages: string[]`: the extra crates that app types and mutation descriptors may name.
  - All 36 `🗿️artifacts/*/🧩️composition/🔣️.json` change at once (35 get `catalogPackage: null, symbolPackages: []`).
- **`🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts`:**
  - Generalize the prefix check to `package` ∪ `symbolPackages`.
  - Hosted-extension deps become `package` + `symbolPackages` with **no** `component-app-assembly`. This assumes U5 is done; otherwise semio's row gets a per-row flag.
  - The full-catalog feature lists `dep:<catalogPackage>`.
  - `selected("full")` calls `<catalogPackage ?? package>::contribution()`.
  - Mutation descriptors are checked against `symbolPackages`.
- **Generator outputs to regenerate:** `🌎️hub/🧩️compositions/🗄️stdio/🤖️generated/{🧩️composition, 🧩️mutations, 🧪️editor-laws}/🦀️.rs`, `🧩️extensions/🧿️semio/🦀️.rs`, `🧩️extensions/🧿️semio/📦️packages/🦀️rust/Cargo.toml` (Composition Dependencies region) and `🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (Composition Features/Dependencies regions).
- **`🧿️semio/🧩️composition/🔣️.json`:** `package` stays the aggregator. Add `catalogPackage: "semio-s-artifact-stdio-semio-catalog"` and `symbolPackages` (base + 18 subsets). App `type`/`factory` become `semio_s_artifact_stdio_semio_<s>::editor::…`. Rewrite `mutationDescriptors` paths. `mutationCoordinates` (owner dirs) are unchanged.
- **`🧿️semio/🧩️composition/🔗️conversions/{🟦️.ts, 🔣️.json, 🧪️tests/🟦️.ts, 🧫️fixtures/🔣️.json}`:** retarget, keeping it schema-first.
  - The `groups[].targets` inventory stays the authority.
  - The generator writes three things:
    1. Each import/export crate's `# 🧩️ Capability Dependencies` region, as non-optional deps on the format crates that the leaves actually name (import / export lists in §3.2's source table).
    2. The `-catalog` crate's `CONVERSION` list and dependencies.
    3. `📜️artifact-definition.json` `dependencies` (unchanged set).
  - The framework `🧩️capabilities` feature mode stays for remodel.
- **Scripts that hard-code the package name:**
  - `🧿️semio/📦️packages/🦀️rust/📜️script.ts`: `cargo test -p semio-s-artifact-stdio-semio --lib subsets::object::` (and `kit`, `stdio_document_contract`) becomes `-p …-object` / `-p …-kit`. `runArtifactRustPackageMain` must test every semio workspace member.
  - `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1412`: `runRepositoryCargoTests(["semio-s-artifact-stdio-semio"], …, ["--lib","sqlite_snapshot_"])` becomes the aggregator plus the subset crates.
  - Unaffected: `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts` reads `🧿️semio/🦀️.rs` for the `SemioMembers` table, which stays. The hub `TRUSTED_BOOTSTRAP` lists plugin ids, not crates.
- **Fixtures naming the package:**
  - `📕️norm/🗿️artifacts/⚡️din18599/📜️artifact-definition.json` and `📕️norm/📇️registry/🧬️contract/🧫️fixtures/✅️valid/🔣️.json` (`dependencies`) → the crates din18599 uses (table, value).
  - `🔱️trinity/…/♻️rewriting/…/🔮️oracles/🔣️.json` `oracleHostPackages` → `-graph`.
  - `🌀️procedural/…/🧊️generation3d/…/🚪️io/🧫️fixtures/🛡️authority/🔣️.json` (`use …::subsets::mesh::io;`) → `-mesh-export` / `-mesh-import`.
  - `🌀️procedural/…/🧊️generation3d/…/🔮️oracles/🔣️.json` → re-check.
- **Workspace and repository metadata:**
  - `🧿️semio/Cargo.toml`: list the 46 new members explicitly.
  - Replace the blanket `exclude = ["**/🏅️standards/**", …]` and `exclude-patterns` with precise ones: `**/🏭️generator/**`, `**/🔬️probes/**`, `**/🏭️bridge/**`, `**/👽️guest/**`, `**/🔮️oracles/**`.
  - Add `[workspace.dependencies]` entries for the new crates.
- **nx:** nested crates carry no nx project files (precedent: generator and oracle crates), so no new `project.json` / `package.json` / `launch.json` entries. The `@semio-tech/stdio-semio-rs` inputs already cover `🏅️standards/**`.

### 4.8 Upstream normalization (required for "once per profile")

| ID | Crate | Change | Removes |
|---|---|---|---|
| U1 | `semio-framework-2d` | Delete features `booleans` and `trace` (always compiled; dead-stripped where unused). Delete `serde`: move `PathSegment`'s serde (de)serialization into `semio-framework-os-flow`, the only user (explicit conversion type, keeping the runtime-dependency-elimination rule). Drop `default-features = false` from the ≈ 20 manifests. | 3 → 1 |
| U2 | `bytemuck` | `semio-framework-ui`, which is in every semio closure, requests `["derive", "extern_crate_alloc", "min_const_generics"]`. | 3 → 1 |
| U3 | `syn` (host) | The always-present framework derive crates request `visit, visit-mut` with their existing `full, extra-traits`: `semio-framework-dsl-record-derive`, `-dispatch-macros`, `-schema-derive`, `-os-kernel-dsl-derive`, `-machine-derive`. | 2 → 1 |
| U4 | census law | A permanent `verify unit-census` command in the hub `📜️script.ts` runs the census over the prebuild plan. It fails if any `semio-s-artifact-stdio-semio*` crate (and, as a ratchet, any crate) has more than one closure signature per target. Language-agnostic fixture: the plan plus the expected count; oracle: `cargo tree` itself. | regression guard |
| U5 | stdio format crates | `component-app-assembly` → unconditional apps (txt, xml, json, csv, md, tsv, html). | 2 → 1 for those and their conversion dependents |

U1–U3 also collapse units of every other crate in those closures: `semio-framework-plugin`, `-value`, `-pack`, `stdio-contract`, `stdio-txt` and the rest. In the shared build-dir they currently show 9–19 units each. That is a bonus not counted in §5.

---

## 5. Expected effect

### 5.1 Per-crate estimates

These are linear in non-test lines and come from the §2.1 measurements:

- wasm-dev ≈ 1.56 s per k-line, release ≈ 5.14 s per k-line.
- Peak RSS ≈ 0.6 GB baseline plus 38 MB per k-line for core code.
- For conversions, the measured +7.4 GB for +8.7 k lines gives a pessimistic ≈ 0.87 GB per k-line.

| Crate | Lines | wasm-dev s | release s | wasm-dev peak GB |
|---|---:|---:|---:|---:|
| `-table` | 14 264 | 22 | 73 | 1.1 |
| `-graph` | 8 439 | 13 | 43 | 0.9 |
| `-brep` | 7 027 | 11 | 36 | 0.9 |
| `-value` | 6 849 | 11 | 35 | 0.9 |
| `-drawing` | 6 761 | 11 | 35 | 0.8 |
| `-document` | 6 545 | 10 | 34 | 0.8 |
| `-mesh` | 6 154 | 10 | 32 | 0.8 |
| `-presentation` | 5 705 | 9 | 29 | 0.8 |
| `-model` | 5 466 | 9 | 28 | 0.8 |
| `-flow`, `-cad`, `-kit`, `-animation`, `-image` | 4.4–4.8 k | 7–8 | 22–25 | 0.8 |
| `-video`, `-audio`, `-object`, `-text` | 3.2–3.7 k | 5–6 | 17–19 | 0.7 |
| `-drawing-export` / `-mesh-export` | 1 049 / 881 | ~3 | ~5–6 | 1.5 / 1.4 |
| `-drawing-import` / `-mesh-import` | 657 / 651 | ~3 | ~5 | 1.2 |
| other 22 import/export crates | 81–489 | ~3 | ~5 | 0.7–1.0 |
| `-base` (≈ 2.5 k with the moved helpers) | | ~4 | ~13 | 0.7 |
| aggregator (≈ 4.7 k: union + root) | | ~7 | ~24 | 0.8 |
| `-catalog` | tiny | ~2 | ~3 | 0.6 |

### 5.2 Totals per profile

| | Today | Split only | Split + U1–U3 |
|---|---:|---:|---:|
| Semio-family units | 16 (monolith) | ≈ 7 × 47 | **47 × 1** |
| wasm-dev CPU | ≈ 16 × 140 s = **37 min** | ≈ 7 × 260 s ≈ 30 min | **≈ 260 s ≈ 4–5 min** |
| wasm-release CPU | ≈ 16 × 520 s = **2.3 h** | ≈ 2 h | **≈ 750 s ≈ 12–14 min** (incl. ~47 × 3–5 s crate overhead) |
| Largest semio unit, wasm-dev | 11.8 GB (all) / 4.3 GB | ≈ 1.5 GB | ≈ 1.5 GB |
| Largest semio unit, release | 14–20 GiB | ≈ 2.5 GiB | ≈ 2.5 GiB |
| Semio critical path, release | ≈ 520 s (one unit) | | ≈ 140–170 s: base → value → table → aggregator (≈ 32 k lines) |

Without U1–U3 the split mostly buys memory headroom, not CPU. With them, the cold dev + release semio work drops by ≈ 2.6 CPU-h, from ≈ 3 h. That is the biggest single item in the build-9 timeline, which spent 88.9 % of its time in wasm component builds.

### 5.3 Prebuild memory cap

Today `cargoPrebuildJobsV1` gives `min(10, ⌊(32 − 6) / 10⌋) = 2` jobs, and `cargoPrebuildPassesV1` gives 1 pass. That is about 50 % of the machine idle.

After the split, the heaviest units become:

- `stdio-dwg`: 6.9 GB at wasm-dev; release to be measured.
- Linked hub components: 5–7 GiB.

Candidate values are `bytesPerJob` ≈ 4 GiB and `heavyUnitBytes` ≈ 8 GiB. That gives ⌊26 / 4⌋ = **6 jobs**, with 2 passes of 3 jobs or 1 pass of 6.

The values must come from a measured p95: run `📈️rustc-peak-sampler.sh` during the first cold build after the split (Wave 4). Do not guess.

### 5.4 Runtime

Runtime performance and wasm size are expected to stay the same:

- Release uses thin LTO, so cross-crate inlining is restored at the cdylib link.
- Generics are already instantiated in consumers.
- Every registered or declared composer set is unchanged (§4.6).

---

## 6. Execution plan (compile-atomic waves)

Every wave ends green on its gate before the next starts. All cargo commands are foreground, through `🚦️cargo-slot.sh`, with a private `CARGO_TARGET_DIR`.

### Wave 0: upstream normalization (independent, can land first)

Steps:

1. **U1:** edit `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/Cargo.toml` (features) and `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/🦀️.rs` (`cfg(feature = "trace" | "booleans" | "serde")`).
   - Move `PathSegment` serde into `semio-framework-os-flow`'s package (`Cargo.toml` + source).
   - Remove `default-features = false` / `features = […]` on `semio-framework-2d` in every manifest. Find them with `rg 'semio-framework-2d\s*=' --glob Cargo.toml`: ≈ 20 plugin and hub manifests, plus the draw artifact.
2. **U2:** edit the `bytemuck` line in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`. Make it non-optional if it is in every closure; confirm with `cargo tree -i bytemuck -e features` per hub.
3. **U3:** edit the `syn` lines in the derive crates listed in §4.8.
4. **U5:** edit the stdio format crates txt, xml, json, csv, md, tsv and html: `Cargo.toml` (drop `component-app-assembly`), their root `🦀️.rs` (`cfg` removal), and the hub stdio generator (no per-row feature).
5. **U4:** add the census command to `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (or the hub stdio script), with a fixture and a test. Register it in `.vscode/🧩️launch.seed.jsonc` → `launch.json` following the existing hub verify group.
6. Regenerate the affected `Cargo.lock` files with `cargo metadata --offline` per touched workspace. No version bumps.

Gate:

- The census shows semio = 9 units (or 8 with U5).
- `cargo check` for the hub workspace at wasm32-wasip2 wasm-dev passes.
- os-flow tests (PathSegment serde round-trip) pass.

### Wave 1: decouple inside the monolith (one crate, behaviour-identical, each step compile-checked)

Steps:

1. **1a (§4.3.1):** create the foundation codec module. Move the helpers out of the audio, drawing, document, mesh, model, cad, flow and video `🚪️io/{📝️text, 💾️binary}/{📸️snapshot, 🔺️diff, 🧬️mutations}/🦀️.rs` files and out of `✉️base/🚪️io/📝️text/📸️snapshot/🦀️.rs`. Rewrite their importers: all subsets' io codec files, ≈ 400 `use` lines.
2. **1b (§4.3.2):** move the `ArtifactChild` codecs out of `📦️object/🚪️io/{📝️text, 💾️binary, 🪶️sqlite}/📸️snapshot/🦀️.rs` and `🧰️kit/…`.
3. **1c (§4.3.3):** delete the dead import in `🔊️audio/🚪️io/💾️binary/🧬️mutations/🦀️.rs`.
4. **1d:** move `semio_mutation_refusals`/`_codes` out of `✉️base/🧬️schema/🧬️mutations/🦀️.rs` and `native_output_tests` out of `✉️base/🧬️schema/📸️snapshot/🦀️.rs` into foundation leaves. These go to a `🧰️triples`-adjacent schema leaf, or into `🧰️triples` itself if no new slug is admitted. Write `✉️base/🦀️.rs` as the foundation module tree, still included by the root as `pub mod base_foundation`.
5. **1e:** write the subset lib-root files `🪆️subsets/<s>/🦀️.rs` ×18, holding each subset's `#[path]` tree, its `retire_*!`, and a `semio_member_owner!` invocation. The root `🦀️.rs` keeps `mod standards { … #[path = "🏅️standards/🔖️v1/🪆️subsets/<s>/🦀️.rs"] pub mod <s>; … }`.
6. **1f:** carve each conversion out of `🪆️subsets/<s>/🚪️io/🦀️.rs` ×13 into new `🚪️io/📥️import/🦀️.rs` and `🚪️io/📤️export/🦀️.rs` (entries, `CONVERSION`, encode/decode, brep `dwg`, video/audio/animation conversion `mod`s). They stay behind `cfg(feature)` for this wave.
   - Introduce `SemioConversion`, `declaration(definition, &[…])` and `register(&[…])` in the root. `declaration` / `register` callers inside the crate pass the cfg-enabled list. External callers (extension, raster, remodel, gismap, layout, shooting, hub stdio) adopt the new signatures in the same step.
7. **1g:** move the conversion-gated tests (`🔺️diff` demo snapshots, `derived-composition-unit`) under `📥️import` / `📤️export`.
8. **1h:** remove the `component-app-assembly` gating. The editor/viewer trees move into the subset lib roots; `SemioAny*` stays in the root.

Gate per step:

- `cargo check -p semio-s-artifact-stdio-semio` with default features, with `--all-features`, and with `--features conversion-drawing`, for both native and wasm32-wasip2.
- `cargo test -p semio-s-artifact-stdio-semio --all-features`.
- `rg 'subsets::(audio|drawing|…)::io::(text|binary)::'` across subsets returns only same-subset hits.

### Wave 2: the crate cut (one compile-atomic wave)

Precheck: run the taxonomy and package-boundary gate on the planned paths. Add a semantic-directory slug for the codec leaf if it is required.

Steps:

1. **2a, manifests.** Create 46 new `Cargo.toml` files:
   - `✉️base/📦️packages/🦀️rust`
   - `<s>/📦️packages/🦀️rust` ×18
   - `<s>/🚪️io/📥️import/📦️packages/🦀️rust` ×13
   - `<s>/🚪️io/📤️export/📦️packages/🦀️rust` ×13
   - `🧩️composition/📦️packages/🦀️rust`

   Edit `🧿️semio/📦️packages/🦀️rust/Cargo.toml`: drop all features, optional deps, `[[test]]` entries and dev-deps that moved; depend on base + 18 subsets. Edit `🧿️semio/Cargo.toml` (members, excludes, `exclude-patterns`, `[workspace.dependencies]`).
2. **2b, lib roots.** `✉️base/🦀️.rs` (from 1d) and the `📥️import/🦀️.rs` / `📤️export/🦀️.rs` roots (from 1f) become crate roots. Create `🧩️composition/🦀️.rs`. Reduce `🧿️semio/🦀️.rs` to the aggregator.
3. **2c, codemod** over every `.rs` under `🧿️semio`:
   - own `crate::standards::v1::subsets::<s>::` → `crate::`
   - foreign → `semio_s_artifact_stdio_semio_<x>::`
   - foundation → `semio_s_artifact_stdio_semio_base::`
   - `crate::examples` → base
   - replicate the extern-crate aliases and crate attributes in every root.

   Two cautions apply:
   - **Codemod scope must fail closed:** use an explicit root and count the files before writing.
   - **Name-keyed edits need a region guard.**
4. **2d, `#[cfg(feature = "conversion…")]`:** delete all 179 sites. `rg -c 'feature = "conversion' 🧿️semio` must return 0.
5. **2e, aggregator test tree** for the 19 `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests` files.
6. **2f, lockfile:** run `cargo metadata --offline` for `🧿️semio/Cargo.lock`.

Gate:

- `cargo check --workspace` in `🧿️semio`, native and `--target wasm32-wasip2 --profile wasm-dev`.
- `cargo test --workspace`.
- The `#[path]` resolution check over all new roots.
- Warnings shown as proof of type-check.

### Wave 3: consumers and generators (one compile-atomic wave; the old feature names disappear)

Steps:

1. **3a, contract schema.** Edit `📇️registry/🧬️contract/🧩️composition/{🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🔣️.json, 🧪️tests/🟦️.ts}` and all 36 `🗿️artifacts/*/🧩️composition/🔣️.json`.
2. **3b, hub stdio generator.** Edit `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/{🟦️.ts, 🧪️tests, 🧫️fixtures}`. Run `composition prepare` to regenerate `🤖️generated/*`, `🧩️extensions/🧿️semio/{🦀️.rs, 📦️packages/🦀️rust/Cargo.toml}` and `🗄️stdio/📦️packages/🦀️rust/Cargo.toml`.
3. **3c, semio conversion generator.** Edit `🧿️semio/🧩️composition/🔗️conversions/{🟦️.ts, 🔣️.json, 🧪️tests/🟦️.ts, 🧫️fixtures/🔣️.json}`. Run `composition prepare` (semio-rs) to fill the 26 conversion manifests' dependency regions, `-catalog`, and `📜️artifact-definition.json`.
4. **3d, consumer manifests.** In the ≈ 44 `[workspace.dependencies]` roots and the package manifests listed in §3.7 (all of `rg -l semio-s-artifact-stdio-semio --glob Cargo.toml`), replace the single `semio-s-artifact-stdio-semio{features}` line with the exact subset and conversion crates used. Keep the aggregator only where `SemioMembers`, `declaration` or `register` is named. Also update `✉️base/🏭️bridge/Cargo.toml`, `🔮️oracles/📦️packages/🦀️rust/Cargo.toml` and `🌎️hub/Cargo.toml` + `🌎️hub/📦️packages/🦀️rust/Cargo.toml` (`native-artifact-execution`).
5. **3e, consumer sources.** Rewrite 349 files / 1 003 occurrences, per §3.7, including `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs` (`typed_snapshot_owner!` paths). The largest owners are ♻️rewriting (86), 📐️cad (64), 🌊️flow (63), 📏️layout (63), 🔌️jack (54) and 🗺️gismap (50). Apply the explicit registrations from §4.6 and the `apply_diff` call sites.
6. **3f, fixtures and scripts** from §4.7: din18599 ×2, trinity oracle, generation3d authority (+ oracle), the semio-rs `📜️script.ts`, and the hub stdio `📜️script.ts`.
7. **3g, lockfiles:** run `cargo metadata --offline` for all 87 touched lockfiles.

Gate:

- Hub `cargo check --workspace --target wasm32-wasip2 --profile wasm-dev`, and each touched plugin workspace natively.
- `composition prepare` is idempotent (second run: no diff).
- The hub stdio tests `editor-catalog` and `shipped-fleet` pass.
- The census shows exactly one unit per semio crate.
- `verify dependencies literal-external` passes.
- The taxonomy / artifact-name gates pass.

### Wave 4: build policy and measurement

Steps:

1. Run one cold release build with `📈️rustc-peak-sampler.sh`.
2. Update the prebuild constants and doc comments, which mention "the stdio semio artifact, measured 14 to 20 GiB":
   - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json`: `bytesPerJob`, `heavyUnitBytes`.
   - `🔒️lease/🟦️.ts` doc comments on `cargoPrebuildPassesV1` / `cargoPrebuildJobsV1`.
   - `🔒️lease/🧫️fixtures`, `🔒️lease/🧪️tests`.
3. Record the before/after timeline in a ticket note.

### Wave 5: residue sweep

These searches must all return nothing:

- `rg 'component-app-assembly|conversion-' --glob Cargo.toml` (expect 0, except remodel's own capability features).
- `rg 'standards::v1::subsets::' --glob '*.rs'` under `🧿️semio` and consumers.
- `rg 'catalogFeatures'`.
- `rg 'semio_s_artifact_stdio_semio::(standards|apply_diff|register\(\)|contribution|assembly)'`.

---

## 7. Risks and checks

- **Taxonomy admission** of `📦️packages` under `🪆️subsets/<s>`, `🚪️io/📥️import` and `🚪️io/📤️export`, and of the `🔤️codec` io facet slug. Run the gate before Wave 2. If the slug is refused, the fallback is a registered existing facet; never put codecs under `🧬️schema`.
- **Split ownership inside ✉️base.** Two crates compile files from one subset directory: `-base` from `✉️base/🦀️.rs`, and the aggregator through `#[path]` to the union files. The file-level ownership table in §4.1 is the authority, and the `#[path]` resolution check must show each file included by exactly one crate.
- **Repo tooling skips `**/🏅️standards/**`.** `[workspace.metadata.semio.repository].exclude-patterns` and the stdio workspace excludes do this today. Tightening them may surface `🏭️generator` / `🔬️probes` crates to tooling. List the exclusions explicitly.
- **Concurrent churn.** Wave 2 and Wave 3 are large codemods over shared files. Keep each wave compile-atomic, verify by diffstat, and use no git-modifying commands.
- **Lockfiles.** Prebuild and narrow builds run `--locked`, so every touched workspace's lockfile must be regenerated in the same wave.
- **Upstream normalization changes features of external crates** (bytemuck `extern_crate_alloc` / `min_const_generics`, syn `visit-mut`). These are additive. The 2d `trace` / `booleans` change makes code unconditional; check `trace`'s runtime cost before Wave 0 lands (`🦀️.rs:23` gate).
- **The conversion memory model is pessimistic.** The +7.4 GB for conversions may be one pathological monomorphization; the per-crate numbers in §5.1 are an upper bound. Wave 4 replaces them with measurements.
