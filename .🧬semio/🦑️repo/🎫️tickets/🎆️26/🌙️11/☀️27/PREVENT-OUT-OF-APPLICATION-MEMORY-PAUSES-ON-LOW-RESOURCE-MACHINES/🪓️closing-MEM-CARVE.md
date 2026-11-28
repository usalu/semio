FINAL (2026-11-27 23:30: nine staffage test units carved from 2.2-3.9 GB to 0.26-1.3 GB peak RSS, model-catalog audit lowered 16-28 % RSS and 38-50 % footprint, 21 carved cases green; hub component finals, parallel cargo trees and typecheck NOT carved, reasons below)

# 🪓️ Closing Report, Lane MEM-CARVE

Measurements, the definition of a low resource machine and the ranked list: `📊️unit-peaks.md` (same folder). Tools kept beside it: `📏️process-peaks.py`, `📏️staffage-files.sh`, `📏️catalog-memory.ts`. Raw logs: `🗑️generated/` (delete with the ticket). Every measuring run went through `repo capacity run`, at most two requests at a time; one extra `repo show projects` that sat in the queue was cancelled by pid. Only own pids were killed.

## 1. Low resource machine and ceiling

8 GB RAM, 4 cores. From `hostCapacity.memory` of the build policy: floor 1536 MB (12.5 % clamped), interactive reserve 2048 MB, 4.6 GB left for builds, session cap 3.3 GB. **Ceiling per unit: 2048 MB of kernel footprint** (what the warden charges since MEM-GUARD). RSS is only an upper bound (up to 3x footprint on macOS for the allocator-heavy units).

## 2. What was carved and the proof

All numbers: peak resident memory of the largest process of one vitest run of the test case (`/usr/bin/time -l`), one case per process, `--maxWorkers=1`, inside `repo capacity run --memory=2200`. Before = `staffage-files.tsv`, after = `staffage-files-after.tsv`, exit code 0 in both unless noted.

| Unit (test case of `@teaching/staffage`) | Before MB | After MB | Fix |
| --- | --- | --- | --- |
| `🌤️reference-environment` | 3946 | 431 | load 2 named assets (`only`) |
| `🌊️flow-exports` | 3386 | 657 | export CLI reads its one asset |
| `🎓️course-coverage` | 2953 | 263 | entries parse their document on access, one at a time |
| `🎭️cast-export` | 2808 | 713 | load the casts' assets only; CLI as above |
| `✂️section-views` | 2562 | 366 | `catalogAssetsById` |
| `🎞️frame-timing` | 2478 | 243 | export CLI |
| `🌐️thermal-multiview` | 2469 | 736 | `catalog([id])` per asset |
| `⚡️asset-power` | 2465 | 1295 | catalog without volumes (`catalogAsset(id, false)`) |
| `🏷️tag-titles` | 2247 | 632 | keep `{id, tags}` of each document |
| `🏗️structure-projection` | 1181* | 923 | per-asset `catalog([id])` with a 4-asset cache |
| `🧊️axonometric-exports`, `🫁️breathing-catalog`, `🧊️chain-state`, `🧊️model-glyph-size`, `🧊️model-glyph-edges`, `🧊️model-frame`, `📑️multiview-sheet`, `📐️dimension-leader`, `📐️dimension-clearance`, `🏷️cast-provenance`, `🔤️caption-fit` | 239-1163* | 212-1008 | same helpers; `📑️multiview-sheet` was red (1 test, a missing asset id from my first edit) and is green now (340 MB) |

\* measured after the edit of that file had already landed (the shared worktree let the first pass see later edits); only the first nine rows have a clean before.

Result: 9 of 131 cases were above 2048 MB; none is after (largest 1295 MB); the whole-catalog parse (about 4 GB) is gone from every case. The suite as a whole no longer sums nine such workers.

### Code changes (all under `🎓️teaching/🏛️architecture/🧍️staffage`)

- `🔨️modules/📤️export/🎬️cli/🟦️.ts`: `catalogOf` (whole catalog, cached forever) became `assetOf` (one asset, last 2 kept). This is product code: every `repo ... export` call and the cast export read one document, not 774 MB.
- `🔨️modules/🗂️catalog/📥️files/🟦️.ts`: `streamAssetsFromFiles` (one asset at a time in catalog order).
- `🧪️tests/🧰️support/🧊️.ts`: `catalogAssetsById`, `catalogAssetsWithVolume(only)`. `🧪️tests/🧊️three-views/🧰️.ts`: `catalog(only)` with a 4-asset cache, `catalogIds()`. `🧫️fixtures/🌡️thermal/🟦️.ts`: `catalogAsset(id, withVolumes)`.
- 25 test files switched from "load all, pick some" to "load what is named" (list in the git status of the worktree); `🧊️thermal-scenarios` and `🧊️three-views` iterate the catalog asset by asset.
- `🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`: collect after every FBX (assimp heap and three scene), and compare ZIP members by length and SHA-256 per member instead of holding every file and the whole unzipped archive.
- New test case `🧪️tests/🎯️bounded-catalog-load` (feature file and three tests, reference catalog folder whose other document is not JSON): 3 pass; the loader without `only` rejects on that folder, so the pass proves the other document was never opened.

### Model-catalog

Measured through `📏️catalog-memory.ts` (one process, several assets, RSS and kernel footprint):

| Run | RSS max MB | Footprint |
| --- | --- | --- |
| `barista`, before | 2489 | 1185 during the file audit |
| `barista`, collector after each FBX | 1787 | 642 |
| `english-oak`, `leaning-person`, `barista`, `excavator`, collector only | 3318 | 1101 |
| `english-oak`, `excavator`, collector and per-member ZIP hash | 2779 | 734 after, 1075 while reading |

Per asset the peak is that of the largest asset, there is no growth from asset to asset (three assets: 2068, 2085, 1892 MB). The 11.3 GB reported earlier was not reproduced and I do not claim a cause for it. `english-oak` remains above 2 GB RSS (2.78 GB) but below the ceiling in footprint (0.73 to 1.08 GB), which is what the warden charges. The vitest run on `barista`, `english-oak`, `excavator`, `leaning-person`: 5 of 6 tests pass; `english-oak` fails `budget pose:calm/windy/storm: GLB of 8.1 MB (budget 8 MB)`, a content finding independent of memory (not touched).

## 3. What was NOT carved, and why

- **Hub component finals** (`cad` 6.5 to 8.0 GB, `puzzle` 6.1 GB, `dag` 3.3 GB): thin LTO holds the bitcode of the whole closure (6259 MB for cad, measured in ticket 26/11/24 `carve-C3`). Only a smaller closure helps; the other lane `SPLIT-PLUGIN-WASM-INTO-SHARED-FRAMEWORK-COMPONENT-AND-THIN-PLUGIN-COMPONENTS` owns that.
- **Rust crates**: nothing to carve by size. Of 131 rustc units seen, the largest are `kernel_event_receiving` (test, 2.1 GB RSS) and `semio_framework_kernel` (1.4 GB); the workspace is 4707 crates, the largest 2045 lines. The 8 to 12 GB cargo classes (`shooting`, `raster-editor`, `procedural-3d-editor`) are sums of parallel rustc, bounded by the jobs of the grant (`capCargoGrantJobsV1`), not by one crate. I could not prove a new job cap on this loaded host (each cargo run waited behind the queue).
- **Nested trees** (`mit-bestand-demonstrator:activate-prototype` 20.5 GB, `framework-os-dev:activate-*-react-prototype` 11 GB, `workspace:setup` 12 GB, `test-cargo-units` 11 GB): classes of whole `repo run` trees; their members were not measured separately.
- **Typecheck** of staffage (class value 5.8 GB): not measured alone, not carved.
- **`🧊️catalog-silhouette` and `🧊️model-batch`**: ran longer than the 25 minute timeout of the first pass, so their peaks are not in the table. A second pass with a 55 minute timeout waited 40 minutes in the queue and was cancelled by pid; rerun `PASS=long TIMEOUT=3300 📏️staffage-files.sh catalog-silhouette model-batch` inside a capacity grant.
- Whole-suite run `repo run @teaching/staffage:test`: not run (hours; 152 assets in the model-catalog case). Green evidence is per case.

## 4. Findings for the other lanes

1. Capacity requests of 1500 to 2500 MB waited 40 to 100 minutes behind 2500 to 3600 MB requests because 4 to 5 GB were "reserved but not yet resident" while the holders observed 300 to 700 MB. This slowed every measurement of this lane.
2. `repo test-file` runs `bun test` on a fleet machine; three of my cases need vitest (`vi.setConfig`) and fail there for that reason only (`section-views`, `caption-fit`, `dimension-clearance`), and `multiview-sheet` needs the DWG wasm module that failed to build on `g1-m12` (`Undeclared original Repo build selection`). All pass under vitest in the pass above.
3. `footprint -f bytes --noCategories -p` fails; plain `footprint -p <pid>` works and was used here.

## 5. Proof of green

- Typecheck of the 30 touched files with `repo typecheck`: 0 errors of mine (the one remaining, `TS7016` for `gltf-validator` types, is outside the change).
- `repo test-file`: 11 of 15 changed cases green on the fleet, the 4 others green under vitest in the after pass.
- After pass (vitest, one process per case): 21 cases, all exit 0, largest 1295 MB.
