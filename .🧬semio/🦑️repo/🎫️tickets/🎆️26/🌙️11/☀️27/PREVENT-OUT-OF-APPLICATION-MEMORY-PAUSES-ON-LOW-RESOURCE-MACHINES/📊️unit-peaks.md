# Unit Peaks: Peak Memory per Compile and Test Unit (MEM-CARVE, 2026-11-27)

Scripts and raw data of this report live beside it: `📏️process-peaks.py` (passive `ps` sampler of every build on the machine, raw `🗑️generated/process-peaks.json`), `📏️staffage-files.sh` (one vitest process per staffage test case under `/usr/bin/time -l`), `📏️catalog-memory.ts` (one process over several catalog assets, resident size and `footprint`).

## 1. The low resource machine

Derived from the capacity policy (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`, `hostCapacity.memory`), not chosen:

| Quantity | Rule in the policy | 16 GB (this Mac) | **8 GB, 4 cores (the target)** |
| --- | --- | --- | --- |
| Floor kept free | 12.5 % of RAM, clamped to 1536 to 4096 MB | 2048 MB | **1536 MB** |
| Interactive reserve | 25 % of RAM, at least 2048 MB | 4096 MB | **2048 MB** |
| Left for builds | RAM minus floor minus reserve | 10.2 GB | **4.6 GB** |
| Session cap | 40 % of RAM | 6.5 GB | **3.3 GB** |
| Load ceiling | 2 per core | 20 | **8** |

**Per-unit ceiling: 2048 MB of real footprint, so that one unit and one smaller unit fit the session cap and two units fit the machine.** The older ticket 26/11/24 set 4096 MB for the 16 GB class (`Policy.unitCeiling`); 2048 MB is the same rule for the 8 GB class (half the RAM per two units).

Two ways of reading "peak" differ by up to 3 times on macOS and this report names which one it uses:

- **RSS** (`ps`, `/usr/bin/time -l` "maximum resident set size", what the guard falls back to): includes pages the process freed but the allocator did not return; they are reclaimable and are not swapped.
- **Footprint** (`footprint -p <pid>`, the plain form works; the form with `-f bytes --noCategories` fails on this macOS): what jetsam charges.

Example, one catalog asset (`barista`, `📏️catalog-memory.ts --trace`): RSS 2139 MB while the footprint is 450 to 770 MB; the largest footprint is 1185 MB (while the files are audited), the largest RSS 2488 MB.

## 2. Measured units, ranked

Peak RSS in MB unless said otherwise. "Source" names where the number comes from.

### 2.1 Test units (staffage, one vitest process per test case)

Source: `📏️staffage-files.sh`, `🗑️generated/staffage-files.tsv` (all 131 cases except `🧊️model-catalog`, run through one capacity grant, `time -l` of the largest process). Eight cases were above the ceiling, a ninth (`🏷️tag-titles`) at 2247 MB; the other 122 are between 136 and 1405 MB. The measured cause of all nine is the same: a whole-catalog parse.

| Rank | Test case | Peak RSS MB | Wall s | Cause |
| --- | --- | --- | --- | --- |
| 1 | `🌤️reference-environment` | 3946 | 9 | `loadCatalogFromFiles` of all 152 assets to use two of them |
| 2 | `🌊️flow-exports` | 3386 | 34 | `drawingOf` of the export CLI kept the whole catalog for the life of the process |
| 3 | `🎓️course-coverage` | 2953 | 2 | parses every asset document to resolve a handful of casts |
| 4 | `🎭️cast-export` | 2808 | 42 | all assets loaded for the casts of one course, plus the CLI cache |
| 5 | `✂️section-views` | 2562 | 9 | all assets loaded to use 6 |
| 6 | `🎞️frame-timing` | 2478 | 14 | CLI cache |
| 7 | `🌐️thermal-multiview` | 2469 | 13 | `catalog()` of `🧊️three-views` reads every asset |
| 8 | `⚡️asset-power` | 2465 | 4 | every asset with its volumes for thermal facts |
| 9 | `🏷️tag-titles` | 2247 | 5 | parses every document to read two fields |
| 10 | `🧊️model-digests`, `🧊️model-exports`, `🧊️three-views` | 1405, 1327, 1219 | 144, 385, 952 | per-asset loops; the `three-views` row was measured after its fix |
| later | 118 cases | 136 to 1181 | 0.3 to 140 | healthy |
| not finished | `🧊️catalog-silhouette`, `🧊️model-batch` | not measured in the first pass (25 min timeout killed them, the largest process read only 213 and 242 MB) | > 1500 | see section 2.6 |

The catalog holds 774 MB of JSON (volumes are about half of the 23 MB of the largest asset); parsed it is about 4 GB. The suite as a whole (`run:test:@teaching/staffage:test`, 14.3 GB in `estimates.json`) is the sum of several such workers at once.

### 2.2 `🧊️model-catalog` (the guard over all 152 assets)

Source: `📏️catalog-memory.ts` through `repo capacity run`.

- Per asset, one process, three assets in sequence (`barista`, `city-bicycle`, `excavator`, no fix): RSS 2068 MB after the first asset and flat afterwards (2118 MB maximum); the JS heap stays at 287 MB or less. **There is no growth from asset to asset in these three; the peak is the largest asset.** The 11.3 GB of the earlier run was not reproduced and is not explained by this measurement; the measured retentions are the ones in section 2.6.
- Footprint of `barista`: 188 MB after the geometry, 450 to 770 MB during the export, 1185 MB while the files are audited (assimp WebAssembly heaps and three scenes of up to 100 files alive at once).

### 2.3 Rust compile units

Source: passive sampler over the builds of all lanes (RSS of the largest process per `--crate-name`), plus the ticket 26/11/24 telemetry for the hub component finals.

| Unit | Peak RSS | Note |
| --- | --- | --- |
| `semio_hub_cad_component` (wasip2 final) | 8036 charged (6550 resident, 2198 in a loaded run) | thin LTO holds the bitcode of the 1041-crate closure (6259 MB); carved by closure size only (ticket 26/11/24 `carve-C3`), other lane `SPLIT-PLUGIN-WASM-INTO-SHARED-FRAMEWORK-COMPONENT-AND-THIN-PLUGIN-COMPONENTS` |
| `semio_hub_puzzle_component` | 6148 | same |
| `semio_hub_dag_component` | 3291 | same |
| `semio_os_flow_component`, `node_graph`, `surface`, `editor` | 1857, 1232, 548, 385 | within the ceiling |
| `rustc-test kernel_event_receiving` | 2006 | the one test crate seen above 2 GB |
| `rustc semio_framework_kernel` | 1445 | within |
| every other crate seen (129 of 131 units) | 33 to 1064 | within |

The workspace is already carved to 4707 crates, the largest of 2045 lines (`staffage-dwg`): a single compile is not what exceeds the ceiling. The large cargo numbers of `estimates.json` (`cargo:test:semio-s-artifact-shooting-shooting` 11.7 GB, `raster-editor` 8.0 GB, `procedural-3d-editor` 8.2 GB) are the sum of rustc processes that run in parallel for one invocation (`-Z threads=0` and the jobs of the grant), not one crate.

### 2.4 Other classes (`~/Library/Caches/semio/host-capacity/estimates.json`, largest sample)

| Class | Peak MB | What it is |
| --- | --- | --- |
| `run:build:@semio-tech/mit-bestand-demonstrator:activate-prototype` | 20553 | the whole nested tree of one task (nine React prototypes, four wasm components) |
| `run:test:@teaching/staffage:test` | 14336 | the whole suite with all workers (section 2.1) |
| `run:test:repo:test-cargo-units` | 11223 | cargo test tree |
| `cargo:test:semio-s-artifact-shooting-shooting` | 11738 | parallel rustc tree |
| `run:build:@semio-tech/framework-os-dev:activate-shooting-react-prototype` | 11046 | vite tree |
| `run:build:workspace:setup` | 12143 | setup tree |
| `slice:graph` | 4766 | graph read |
| `run:check:@teaching/staffage:typecheck` | 5817 | tsc over `🔨️modules`, `🧪️tests`, 3D libraries |
| `dependencies:sync` | 2295 | |
| `tools:setup` | 2487 | |
| TS typecheck (`library.tsconfig.json`, sampler) | 2307 | one tsc |

### 2.5 Graph planner per owner (`.cache/f/graph-costs/owner-costs.json`)

127 of 639 owners have a sample above 2048 MB (36 `language:rust`, 35 `language:typescript`, 29 `language:cargo`, 27 `project`); the largest samples are `value-rs` 6516, `forms-plugin` 6168, `dag-js` 5673. These are samples of the graph reader process while it analysed an owner under load (each owner has one sample, taken with other owners in flight), not the memory of the owner's own build; they are the input of the graph lane (`slice:graph`), not of a carve.

### 2.6 Retentions found by the measurement

1. **Whole-catalog parse** (nine test cases, 2.2 to 3.9 GB): `loadCatalogFromFiles` without `only`, the export CLI's `catalogOf` (cached forever, all 152 assets), `catalog()` of `🧊️three-views`, `catalogAssetsWithVolume()`, `catalogIds().map(catalogAsset)`, a module-level array of every parsed document in `🏷️tag-titles` and `🎓️course-coverage`.
2. **Dead assimp WebAssembly heaps and three scenes** kept until the next collection while a file audit read up to 100 files of one asset (`barista`: footprint 1185 MB, RSS 2489 MB).
3. **Every file of an asset held by the audit** to compare it with its ZIP member at the end, plus the whole unzipped archive (`english-oak`: 656 MB external at the end of the audit).
4. **RSS above footprint**: the allocator keeps freed pages resident (`english-oak` export: RSS 2482 MB, footprint 440 MB). Not a leak; the warden reads the kernel footprint (`🧯️closing-MEM-GUARD.md`), so only footprint counts for admission.

## 3. What this ranks, in one list

1. Hub component finals (cad, puzzle, dag): closure size, other lane.
2. Whole-catalog loads of staffage test cases (eight cases at 2.5 to 3.9 GB): carved by this lane (section 4 of the report).
3. Parallel rustc trees (`-Z threads=0`): bounded by the jobs of the grant, not by a crate.
4. `🧊️model-catalog` per asset: 1.2 GB footprint, 2.5 GB RSS.
5. Typecheck of staffage and library: 2.3 to 5.8 GB.

## 4. Model-catalog, measured per asset

| Process | RSS max MB | Footprint at its largest MB | Note |
| --- | --- | --- | --- |
| `barista`, `city-bicycle`, `excavator` (no change) | 2489 (`barista` alone) | 1185 (file audit) | `mem-b.log` |
| `english-oak`, `leaning-person`, `barista`, `excavator` (collector after each FBX) | 3318 | 1101 | `mem-c.log` |
| `english-oak`, `excavator` (collector after each FBX, zip members compared by hash) | 2779 | 734 after the audit, 1075 while reading | `mem-d.log` |
| four assets through vitest (`catalog-after.log`) | 3269 | n/a | 5 of 6 tests pass; `english-oak` fails `budget pose:calm: GLB of 8.1 MB (budget 8 MB)`, a content finding that does not depend on memory |

## 5. Not measured

Peaks of `🧊️catalog-silhouette` and `🧊️model-batch` (section 2.6), the staffage typecheck alone (5.8 GB class value, one process was seen at 2.3 GB in the sampler), the 45 `semio_hub_*` finals, `cargo` trees per crate.
