# 🧷️ Wave 0: Upstream Feature Drift (2026-10-13)

This is Wave 0 of `📓️2026-10-13-stdio-semio-split-plan.md`. It removes the per-consumer feature variants that made shared crates compile more than once per profile in the hub prebuild closure. It also adds a permanent gate for that rule.

## 1. Result

The census covers the hub prebuild plan: 44 component packages of `🌎️hub/Cargo.toml`, each resolved alone with `CARGO_RESOLVER_FEATURE_UNIFICATION=package`, at wasm32-wasip2.

| | Before | After |
|---|---:|---:|
| Compile unit keys (package, target, mode, platform) | 375 | 375 |
| **Compile units (distinct closure signatures), wasm-dev** | **708** | **388** |
| Keys with more than one feature set | 43 | 2 (both allow-listed) |
| Keys with more than one closure signature | 122 | 7 |
| `semio-s-artifact-stdio-semio` feature sets / units | 8 / 16 | 8 / 8 |

The plan's Wave 0 gate expected "semio = 9 units, or 8 with U5". The result is **8**. wasm-release gives the same 375 keys and 388 units.

### Variant counts per crate

Columns: feature sets / closure signatures, before → after. Every crate whose count changed is listed in `🗑️generated/wave0/census-{before,after}.json`. These are the drifting crates and the heavy ones:

| Crate (unit) | Before | After |
|---|---:|---:|
| `semio-s-artifact-stdio-semio` | 8 / 16 | 8 / 8 (allow-listed until the split) |
| `semio-hub-stdio` | 2 / 3 | 2 / 2 (allow-listed, §5) |
| `semio-s-artifact-stdio-dwg` | 2 / 8 | 1 / 1 |
| `semio-s-artifact-stdio-pdf` | 2 / 8 | 1 / 1 |
| `semio-framework-plugin` | 1 / 8 | 1 / 1 |
| `semio-framework-os-kernel` | 2 / 3 | 1 / 1 |
| `semio-framework-value` (target / host) | 1 / 2, 1 / 2 | 1 / 1, 1 / 1 |
| `semio-s-artifact-stdio-contract` | 1 / 8 | 1 / 1 |
| `semio-framework-os-infinite` | 1 / 7 | 1 / 1 |
| `semio-framework-2d` | 3 / 5 | 1 / 1 |
| `bytemuck` | 3 / 5 | 1 / 1 |
| `syn` (host) | 2 / 2 | 1 / 1 |
| `semio-framework-os` | 2 / 3 | 1 / 1 |
| `semio-s-artifact-trinity-jack` | 2 / 2 | 1 / 1 |
| `semio-s-artifact-stdio-{txt,xml,zip}` | 2 / 10 | 1 / 1 |
| `semio-s-artifact-stdio-{binary,deflate,dxf,json,png,svg}` | 2 / 9 | 1 / 1 |
| the other 26 stdio format crates | 2 / 3–8 | 1 / 1 |
| `semio-framework-ui`, `-pixels`, `-tool-run`, `semio-framework` | 1 / 5 | 1 / 1 |
| `serde` (target, host), `serde_derive`, `wit-bindgen*`, every framework derive crate | 1 / 2 | 1 / 1 |

Seven keys still have 2 or more signatures:

- `stdio-semio` and `semio-hub-stdio`, which are allow-listed.
- `forms`, `gismap`, `generation3d`, `process3d` and `curation`. These have one feature set each. They differ only because they sit above `stdio-semio`'s feature sets, so the split removes them.

## 2. Changes

### U1: `semio-framework-2d` (3 → 1)

- **Features deleted.** `booleans`, `trace` and `serde` are gone, and so is the optional `serde` dependency. The `booleans`, `regions` and `trace` modules are now unconditional.
- **`PathSegment`** keeps only its `ToValue`/`FromValue` wire derive.
- **105 manifest lines** dropped `default-features = false` and/or `features = [...]` on `semio-framework-2d`.
- **Serde moved to `semio-framework-os-flow`**, its only serde user. `🖍️drawing/🦀️.rs` now owns an explicit conversion type:
  - `PathSegmentJson`, with the same `kind` tag and camelCase/`largeArc` shape.
  - `From` conversions in both directions.
  - Two `#[serde(with)]` codecs, `path_segments` and `optional_path_segments`, used by `DrawingNode::Path`, `SceneNode.clip` and `StoredNode.clip`.
  - The JSON helpers `trace_bitmap_json` and `boolean_segments_json` go through the same type.
- **New language-neutral fixture** `🖍️drawing/🧫️fixtures/🔣️.json`, with all six variants, a clipped and an unclipped `SceneNode`, and three tests in `🖍️drawing/🧪️tests/🔬️path-segment-json`.

**Runtime cost of an unconditional `trace`: none, measured.**

- **Static check.** The trace module has no statics, `#[used]`, exports or constructors, and the features enabled no dependencies.
- **Probe.** I built two `wasm32-wasip2` release cdylib probes against the new 2d, in scratchpad with a private target and build dir under `⚡️cache/w0`:
  - Probe *a* uses only `PathSegment`.
  - Probe *b* also calls into `trace`.
  - The 2d rlib defines 44 trace symbols.
  - Neither linked component contains any trace string. Probe *b*'s calls were inlined to constants.
  - Sizes: 14 381 B and 14 389 B stripped; 2 683 205 B and 2 683 213 B unstripped.
- **Conclusion.** Unused trace code costs one compile of about 200 lines in the single 2d unit, and nothing in linked components.

### U2: `bytemuck` (3 → 1)

`semio-framework-ui` pins `["derive", "extern_crate_alloc", "min_const_generics"]`. The extra two features are requested by `image` and `font-types`.

### U3: `syn` (2 → 1)

The seven framework proc-macro crates pin `visit` and `visit-mut` next to `full` and `extra-traits`:

- os-kernel dsl-derive
- dsl-record-derive
- async macros
- schema-derive
- value-derive
- machine-derive
- dispatch

### U5: stdio format crates (all 35, not only the 7 of the plan) and `trinity-jack`

Each family extension enabled `component-app-assembly` on its own format crates, so every format crate compiled twice. The changes:

- **Feature deleted** in the 35 stdio format crates (every one except `🧿️semio`) and in `semio-s-artifact-trinity-jack`. Its 114 + 11 `cfg` gates were removed, and the editor and viewer apps are now unconditional.
- **Formerly optional dependencies are now required:**
  - pdf: `semio-framework-ui-contract`.
  - jack: seven crates, plus `semio-framework-plugin` with `component-guest`.
- **`required-features` removed** from the png and bmp `[[test]]` targets.
- **Hub stdio generator** (`🧩️composition/🟦️.ts`): it requests `component-app-assembly` on a row only when that artifact's own manifest declares the feature. This is schema-driven, so only `stdio-semio` keeps it, and the flag disappears with Wave 1h. I regenerated through `@semio-tech/stdio-plugin:composition-prepare`. The only output change was the 10 family manifests and the `component-app-assembly = ["full-artifact-catalog"]` region.
- **trinity hub and trinity bridge** no longer request the feature on jack. The `rewriting` feature no longer forwards it.
- **Scripts:** removed `testFeatures`, `snapshotSqliteTestFeatures` and `--features component-app-assembly` from 14 stdio artifact `📜️script.ts` files and jack's.
- **Launch entries:** 17 entries in each of `launch.json` and `🧩️launch.seed.jsonc` dropped `--features component-app-assembly` for non-semio stdio projects. That includes the `stdio-*-rs` run-many, which now builds semio without its app feature.
- **Laws in the hub stdio `📜️script.ts`:**
  - The home-io-surface law no longer requires the apps to be gated. It now requires that the Home I/O artifacts declare no `component-app-assembly` feature.
  - The editor-catalogue law reads which formats each package ships from the generated plugin assembly (`builder.editor::<semio_s_artifact_stdio_<format>::…>`) instead of from feature reachability.

### Other drift: `semio-framework-os` (2 → 1) and `semio-framework-os-kernel` (2 → 1)

`semio-hub-space` enabled `space-guest`, and through it `semio-framework-os-kernel/sync`. Five other hubs (layout, note, raster, remodel, shooting) used the featureless OS host. The changes:

- **Features deleted.** `space-guest` and `os-host-full` (which was an alias of `space-guest`) are gone.
- **Required dependencies.** `semio-framework-artifact-space-{space,collection}` are always present. `semio-framework-deflate` is always present in the non-p2 target table.
- **Kernel `sync` only off the WASI guest.** `semio-framework-os-kernel` gets `features = ["sync"]` through a `cfg(not(all(target_arch = "wasm32", target_env = "p2")))` target table. That is exactly the guard on the `store_sync` re-export and the backbone folder storage. So `sync` reaches native and browser builds but no wasm32-wasip2 component.
- **Source.** 28 `cfg` gates were dropped. The 14 stub items for the featureless build were deleted, including the local media registry, `OsMediaExportResult`, `create_os_id`, `OsArtifactDescriptor` and `os_resource_media_capability`.
- **Behaviour for the five hubs is unchanged.** They call only `rasterize_svg_to_png_base64` and `svg_to_polylines`, which have identical bodies, and build `OsMediaExportResult` literals, which have the same four fields. The space modules have no constructors or exports, so they are dead code there.
- **Consumers:**
  - The hub space, space-space, home and space-core manifests collapsed their two target tables into one dependency without features.
  - The os host `📜️script.ts`, the hub space `📜️script.ts`, the space testing script and one launch entry (in both launch files) dropped `--features os-host-full`.
- **Laws:**
  - The directory-home-browser-process law (hub `📜️script.ts` plus its fixture, `nativeFeature: "sync"`) now checks `semio-framework-os-kernel feature "sync"`: absent from the wasip2 guest graph, present in the native graph. It now runs `cargo tree` against `🌎️hub/Cargo.toml`. It used the repo root before, where `semio-hub-space` does not resolve.
  - I checked both graphs directly. The guest graph has no `sync`, ring, cc or tokio. The native graph has `sync`.
  - The home-io-surface law now requires that the OS host has no features and that the native target table carries `sync`.

## 3. Permanent gate: `os-hub:unit-census-check`

- **`🌎️hub/🧪️tests/🌳️unit-census/🟦️.ts`** (bun test):
  - `cargoUnitCensusV1` is a pure function over per-package `cargo build --unit-graph` output. It counts, per unit key, the feature sets (and which packages select each) and the closure signatures. A build-script compile unit is keyed by the platform of the run that executes it.
  - `cargoUnitFeatureDriftV1` returns the keys with more than one feature set that are outside the allow list.
- **Language-neutral fixture** `🌎️hub/🧫️fixtures/🌳️unit-census/🔣️.json`. Three synthetic cases:
  - one shared dependency with one feature set;
  - a feature split that propagates signatures upward;
  - host and target build scripts.

  The fixture also holds `featureVarianceAllowed` (package, reason, `until` link), the plan workspaces (`["🌎️hub"]`, read through `componentPrebuildPlanV1`, the same planner the fresh build uses), the target and both profiles.
- **Live test**, for wasm-dev and wasm-release. It fails on any drift outside the allow list. It also fails when an allow-listed package stops varying, so the list ratchets down.
- **Independent oracle.** For every planned package, the library feature sets from `cargo tree -e normal,build` must equal the census's library units. They do, for all 44 packages (58 `expect` calls).
- **Registration:**
  - `.register("unit-census-check", HubUnitCensusScript)` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`.
  - Target `unit-census-check` (cache off) in its `📋️project.json`.
  - `⚖️gate🌳️hub-prebuild🧮️unit-census` (order 411.107555, right after `⚖️gate🧱️hub-foundations📐️source`) in `launch.json` and `🧩️launch.seed.jsonc`.
- **Verified** with `bun nx run os-hub:unit-census-check`: 2 pass, 0 fail, 39 s.

## 4. Verification (all run in this session, private `CARGO_TARGET_DIR=⚡️cache/w0/target`, through `🚦️cargo-slot.sh`)

| Command | Result |
|---|---|
| `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2` | **green**, 164 units checked including 2d, os-kernel, os, os-flow, plugin, every family extension, stdio-semio, dwg, pdf, txt, binary, jack, hub-stdio, hub-space and hub-writer; 895 warning lines as proof of type-checking |
| Same command, native | **green**, 176 units |
| Native `--lib` checks outside the hub graph: `semio-s-plugin-playbook-procedural`, `semio-trinity-mutation-bridge` (bin), `semio-s-plugin-flow-extension-draw` | green |
| `cargo test -p semio-framework-2d --lib` (🧰️framework workspace) | **90 passed** |
| `cargo test -p semio-framework-ui --lib`, default and `--features wgpu` | **1 passed**, **199 passed** |
| `cargo test -p semio-framework-os-flow --lib drawing::` | **43 passed** (40 kernel + 3 new PathSegment JSON tests) |
| `cargo test -p semio-framework-os-flow --lib` (whole suite) | aborts on a destructor panic after 53 ok / 5 failed (`extensions::wasm`), or 65 ok / 12 failed with `--skip extensions::wasm` (`host::*`, `geometry::ownership_tests`). Same families and abort as recorded before this wave in `📓️2026-10-10-migrate-os-domains.md` (PARK); none touch drawing |
| `bun nx run @semio-tech/stdio-plugin:home-io-surface-check` | pass |
| `bun nx run @semio-tech/stdio-plugin:test -- long editor-catalog-contract` | pass, 88 editors in 10 packages |
| `bun nx run os-hub:unit-census-check` | pass |
| `bun build --no-bundle` of every edited TS file | pass |

Lockfiles:

- **How refreshed.** I ran `cargo metadata --offline` without `--locked` (`wave0-feature-drift/🔒️refresh-locks.sh`) over the 173 non-ticket workspaces whose lock names a touched crate. All 173 resolved and 87 locks changed.
- **41 changed by six lines or fewer.** These are the expected edits: the `serde` line in the 2d entry, the jack and pdf optional dependencies, and the OS host space dependencies.
- **46 were already far behind their manifests** before this wave. They are mostly `🏭️bridge` workspaces, with up to 5 423 changed lines. That staleness is absorbed now. Existing entries keep their versions; offline resolution adds no upgrades.
- **The hub lock** changed by one line (`serde` removed from 2d). The census and gate run `--locked` against it.

## 5. Remaining drift

`semio-hub-stdio` (2 units) is allow-listed:

- **Why it splits.** `semio-hub-gis` and `semio-hub-vcs` depend on the stdio *component* crate as a library (`default-features = false`, `full-artifact-catalog`) for `catalog::native_artifact_catalog_{dependency,contribution}`. The root component build adds `plugin-root` (the exports) and `component-app-assembly`.
- **The proper fix** is to extract `🔌️plugin/📇️catalog` (583 lines) and `🛂️manifest` into a featureless catalog crate that gis, vcs, the stdio component and the native hub depend on. That touches the generator, the `[[test]]` targets and the home-io-surface layout assertions.
- **Why not now.** It overlaps the plan's Wave 3 rewrite of this generator (`catalogPackage`). The cost of leaving it is one small extra unit per profile (the hub-stdio crate itself). Its dependencies are already shared.

## 6. Noticed, not caused by this wave

- **`semio-framework-plugin-host-test-component` (`🔌️plugin/🖥️host/🧪️testing/🧩️component`) does not compile.** It has 18 errors: `semio_framework_plugin::os_vcs` does not exist, and `RetireOwned` is unsatisfied.
- **`tsc` type-checking of the edited TS is blocked.** Two files with syntax errors, `🧩️puzzle/🗿️artifacts/{🧊️3d,🖐️5d}/…/🧬️schema/🔺️diff/🟦️.ts`, produce 904 errors, so tsc skips semantic checking. Bun transpiles and runs every edited file.
- **Two existing `os-host` warnings** in `🖥️host` are now visible in all builds: the unused `extern crate … as dsl` and the unused `ErasedSnapshotRetirement` import. Before this wave they appeared only in the `space-guest` build.

## 7. Ticket inputs and outputs

- **Inputs:** `wave0-feature-drift/🌳️unit-census.py`, `🔓️ungate-features.py`, `⏱️bounded.sh`, `🔒️refresh-locks.sh` and `tsconfig.wave0.json`.
- **Outputs:** logs and census JSON in `🗑️generated/wave0/`. I created that folder and changed nothing else in `🗑️generated`, so the coordinator can delete it.
- **Cleanup:** I deleted the private `⚡️cache/w0` directory, including the probe build dirs.
