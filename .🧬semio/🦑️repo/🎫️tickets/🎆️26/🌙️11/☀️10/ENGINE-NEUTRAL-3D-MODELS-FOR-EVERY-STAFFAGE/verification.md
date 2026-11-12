# Verification: all 152 staffage 3D models end to end, batch export target

Ticket `ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE`. `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `K` = `🧰️framework/🔨️modules/🧊️3d/🗿️model`, `T` = this folder. Tool output under `T/🗑️generated/verification/` (batch logs, audit jsonl, guard output). Silhouette table: `volume-agreement.md`. The `repo` MCP server did not connect, so the ticket is not closed with `ticket_close`.

## 1. Result in one table (152 assets, 556 poses, 359 clips, 915 model documents, 152 ZIPs)

| Check (oracle) | Pass | Fail |
|---|---|---|
| authored `volume`, `volumeIssues` | 152 | 0 |
| `exports` lists `glb`, `fbx`, `model`, `model-zip`; `exportIssues` empty (every asset has a named pose and at least two clips) | 152 | 0 |
| `modelOf` (all views, all poses) then `validateModel`: 0 errors; `tessellate`; per-solid tessellation equals whole tessellation | 152 | 0 |
| non-finite numbers / degenerate triangles (area under 1 mm2 or collapsed after welding) in 11 713 solids | 0 / 0 | 0 |
| watertight and outward (independent weld at 0.1 micrometre, directed-edge pairing, signed volume): box 873, ellipsoid 688, capsule 1125, extrude 368, loft 1373 (all capped), revolve 7286 | all closed | 0 (6 extrudes failed before, see fixes) |
| bounds in `presentation.defaultPose` against `dimensions` (A6), gate 8 % and at least 15 mm | 152 | 0 (largest misses: hedgehog height +11.4 % = 14.8 mm of 0.13 m, inside the floor; date-palm 3.7 / -0.1 / -6.4 % after the fix; red-squirrel, agave, lombardy-poplar, hosta-clump, bamboo-clump, scots-pine inside 6.4 %); the rest pose is never compared; no asset needed a different pose |
| every pose GLB: Khronos `gltf-validator` 0 errors and 0 warnings, 7 variants in order, three `GLTFLoader` loads, finite extent, no animation | 556 | 0 |
| every clip FBX: three `FBXLoader` and `assimpjs` both load, one animation, duration equals the clip (1 ms or 1e-6 relative), non-empty, finite keys, all rig bones present as nodes in both readers | 359 | 0 |
| every model JSON (pose and clip): Ajv against `semio.model/v1`, `validateModel` 0 errors, 7 variants, rig bones present, clip animation present | 915 | 0 |
| ZIP: member names equal the planned files, no duplicate member, each member byte-equal to its file | 152 | 0 (bundles with a pose and a clip of one id, herring-gull, solar-thermal-collector, seated-office-worker, person-cooking, were silently losing one JSON before, see fixes) |
| manifest: bytes, SHA-256, triangle count of every file equal the file | 1982 | 0 |
| path length (absolute, default output dir) at most 256 | 1982 | 0 (longest file name 78 characters) |
| budgets: no GLB above 8 MB (largest `bookshelf` 3.6 MB, then `cyclist` 2.2 MB, `seated-office-worker` 2.0 MB); no asset above 400 000 triangles at 1:50 (largest `tower-crane` 48 832, `cyclist` 42 954, `cargo-bike` 36 456, `city-bicycle` 34 344, `date-palm` 31 668; median 6 568; sum 1 304 522) | 152 | 0 outliers |
| coarser scales via `maxDenominator` reduce the count: 124 assets carry `maxDenominator` solids and every one of them has fewer triangles at 1:500 than at 1:50; 28 assets have none (constant). Totals of volume-solid triangles: 1:50 1 304 522, 1:100 1 130 912 (108 assets reduced), 1:200 1 000 306 (124), 1:500 837 536 (124); never growing with the scale | 152 | 0 |

The guard is `S/🧪️tests/🧊️model-catalog` (153 tests, all green; with `SEMIO_STAFFAGE_MODELS_DIR=<output dir>` it reads the batch output, otherwise it exports each asset into a temporary directory; 48 s on the existing output, 2.6 GB peak resident because it holds three, the validator and assimp). Its oracle code is `🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts` and is the same code that validates a whole output directory (task 4 was not done twice). `LIMITATIONS` in the guard is an empty, typed table (asset, check, item, reason): nothing is excused. Assertions were not weakened: the old guard compared the extent in the presentation pose or the best pose with a printed note; the new one requires the presentation pose to pass.

## 2. Fixes made

Kernel (`K`, precise minimal fixes, each with a test and a vector):
- `K/🔺️tessellation/🔺️polygon/🟦️.ts`, cap triangulation with holes. Root cause of 6 non-watertight extrudes (`kitchen-extractor-hood/filter`, `chalkboard/frame-ring`, `plate-heat-exchanger/end-wall-left|right`, `tied-canopy/wall-insulation|wall-render`): two rectangular holes that share a row (the corners at the same y) bridged along the collinear edge of the other hole, and a vertex id that appears twice after an earlier bridge was entered at the wrong occurrence; the cap lost 6 triangles per face and covered the holes. Fix: collinear candidates beyond the ray hit are skipped, and the new bridge enters the occurrence whose interior wedge faces the hole (`wedgeContains`). Vectors: `K/🧫️fixtures/🔺️polygon` cases `two-aligned-rectangular-holes`, `three-aligned-holes-one-row`, `aligned-holes-two-rows`; `K/🧫️fixtures/🏗️extrude` case `two-aligned-holes` (generator `T/generate-model-fixtures.py`, regenerated: only those two fixture files changed); test `K/🧪️tests/🔺️polygon` grid of aligned holes 1x1 to 4x4 (boundary once, interior paired, area). The 3000-polygon stress test still passes (a first attempt that skipped all candidates beyond the hit failed it once and was narrowed to the collinear case).
- `K/📏️edges/🟦️.ts`: `featureEdges` option `material` may be a function of the source primitive; test `K/🧪️tests/📏️feature-edges`.
- `K/📤️export/🎬️fbx/🟦️.ts`: a node that is a skin joint and also holds rigid triangles was written as one `Mesh` model; three's `FBXLoader` turns a skin joint into a `Bone` and dropped its geometry (shower-head, english-oak, london-plane-pollarded, small-leaved-lime, weeping-willow lost 40 to 55 % of the triangles in three; assimp saw all of them). Now a `LimbNode` with a child `Mesh` model `<id>-geometry` (the solid ranges move with the geometry). Test `K/🧪️tests/...three-roundtrip` ("keeps the geometry of a joint that also holds a mesh", every triangle seen by three), the structure test reads `SolidRanges` from either model. FBX suite 43 tests, core suite 329 tests, `tsc` 0 errors.

Staffage (`S`):
- Duplicate file names: a pose and a clip may share an id (herring-gull `landing`, solar-thermal-collector `stagnation` and `operating`, seated-office-worker `typing`, person-cooking `stirring`): their model JSON had the same name, so the bundle ZIP held a duplicate member (an unzip into a map hides it) and the CLI overwrote one with the other. `🔨️modules/📤️export/🗿️model` `modelExtension`: `<stem>.pose.json` and `<stem>.clip.json` in the CLI, the bundle and the batch; `🧪️tests/🧊️model-exports` follows.
- `writeGlbDetailed`, `FbxFileResult.triangles`, `trianglesOf` for the manifest; `streamAssetsFromFiles`-style `assetPathsFromFiles` and `readStreamedAsset` in `🔨️modules/🗂️catalog/📥️files/🟦️.ts`.
- Glyph outlines (coordinator item): `🔨️modules/🧊️volume/🪧️glyphs` `glyphEdgeOf`, `edgeMaterialOf`; `modelOf` adds a hidden `<glyph material>-edge` material per glyph slot and every variant maps it exactly as it maps the glyph surface (the edge ink in the variant of the glyph, hidden elsewhere); the GLB and the resolved model JSON use `edgeMaterialOf(procedural)` for the edge pass. Test `🧪️tests/🧊️model-glyph-edges` (Khronos validator, parsed glTF, solid provenance): per variant no visible glyph outline without its visible surface (cantilever-balcony forces 5 surfaces / 5 outlines, moments 6 / 6, all other variants 0 / 0). The React `withoutGlyphEdges` workaround in `🎯️targets/⚛️react/.../🏭️prepare` is redundant now and was left to its owner.
- Exports offered (coordinator item): `glb`, `fbx`, `model`, `model-zip` appended to `exports` of all 152 assets (exact-string insertion into the member, per-file indentation kept, files re-read before the write), to the 152 `entries` of `🗂️catalog/🔣️.json` (the manifest mirrors them; `validateManifest` demands equality), to the sample-figure, sparse-smooth-walk and reference-walker fixtures and to the manifest in `🧫️fixtures/✅️asset-validation`; `modelFormats` now includes `model-zip`, `exportIssues` judges it; the guard asserts every asset offers every producible format and that `fbx` is only offered with a clip (all 152 have at least 2).
- Audit items: `model-zip` in `ExportFormat` (schema `🧬️schema/🔣️.json`, twin, description; the Python consumer has no list); cancellation checks in every emit loop of the export CLI (svg, dxf, dwg per frame and in `emit`), test "stops the emit loops of the export command when the signal aborts"; bilingual CLI usage and refusals (`CliUsageError`, `refuse`, both languages in message and `reason`), bilingual batch command line (`BatchUsageError`), bilingual refusal codes `model-unknown-bone` and `model-batch-path`; the bone-title fallback in `titleOfSurface` now throws `ModelRequestError("model-unknown-bone")`; `robust-predicates` 3.0.3 declared (root `package.json` and the workspace line of `bun.lock`, oracle registry `S/🔮️oracles/🔣️.json`; the package was already locked as a transitive dependency; `🔒️dependencies.json` was not regenerated); design-required artefacts: `🧪️tests/🧊️model-digests` with fixture `🧫️fixtures/🧊️model-digests` (GLB digests per pose and FBX digests per clip of the sample figure: byte length, SHA-256, glTF counts, variants, FBX object counts, notices; oracles Khronos validator, three loaders, assimp; generator `T/gen-model-digests.ts`) and `🧪️tests/🧊️model-silhouette` (sample figure and dining-table, single-bed, glulam-beam: exact IoU of polygon-clipping at least 0.80 in all three views, an owned scanline raster within 0.03 of it, bounding box within 8 %).
- Left undone from the audit: emoji reuse across docstrings (204 distinct in 1 122; no rule-conforming scheme exists to apply mechanically), `generator: "staffage volume/v1"` (a label of the volume generator, not the schema version; changing it changes every model byte), the `staffage/v1` namespace in the React downloads, English-only messages in the depiction views, preview and cast modules, the dead guards for `volume === undefined`, `VolumeUnavailableError` and its React catch sites (other agents' files).

Asset data (catalog files, mirrored in `T/volumes/<category>/<slug>.json`; scripts `verification-fix-date-palm.py`, `verification_textedit.py`):
- `🌳️vegetation/date-palm`: `fr-r1b-blade`, `fr-l1b-blade` sections `at[0]` 0.26393, 0.87975, 1.49557, 1.7595 to 0.22434, 0.74779, 1.27123, 1.49558 (and the `fr-l1b` blade's mirrored values likewise, 0.85 times), `fr-r1b-rachis`, `fr-l1b-rachis` `p1[0]` 1.7595 to 1.49558. Reason: the six-copy fan made the crown round (x 7.013 m, +11.3 % against 6.3); now x 6.53 (+3.7 %), y 12.49, z 6.55 (-6.4 %).
- `exports` of all 152 assets: `[..., "component"]` to `[..., "component", "glb", "fbx", "model", "model-zip"]`.

red-fox `curled-sleeping`: decided with evidence in `volume-agreement.md`: the authored abduction (lumbar 22, neck 40, head 25 degrees), not a binding error; nothing changed.

## 3. Batch target

Library: `S/🔨️modules/📤️export/📦️batch/🟦️.ts` (`exportModels`, `plannedNames`, `parseBatchArguments`, `BatchManifest`; also the command line). Project target in `S/📦️packages/🟦️typescript/📋️project.json` (`export-models`, script class in `📜️script.ts`, `cache: false`, output `{projectRoot}/📤️dist/🧊️models`; `📤️dist` is in `.gitignore`, as the site build's `.🧾️store`).

```
repo run @teaching/staffage:export-models                       # --lang en is the target's argument, forwardAllArgs adds yours
repo run @teaching/staffage:export-models -- --lang de --asset tealight,horse --scale 100 --view thermal --force
repo tools exec bun -- S/🔨️modules/📤️export/📦️batch/🟦️.ts --lang en [--out <dir>] [--scale 50] [--asset a,b] [--view <view>] [--environment <preset>] [--span-k <K>] [--catalog <dir>] [--parent <pid>] [--force]
```
Output (default `S/📦️packages/🟦️typescript/📤️dist/🧊️models`):
```
manifest.json                                   settings {generator, scale, view, environment, presets digest, thermalSpanK}, assets [{id, source sha256 of the catalog document}], files sorted by path
<asset>/<asset>-<pose>-all-model-1to50.glb      one per named pose, seven variants inside
<asset>/<asset>-<clip>-light-solid-model-1to50.fbx   one per clip, default view, --view selects another
<asset>/<asset>-<pose>-all-model-1to50.pose.json   <asset>/<asset>-<clip>-all-model-1to50.clip.json   neutral model
<asset>/<asset>-all-model-1to50.zip             glb/ fbx/ model/ members, byte-identical to writeModelBundle
```
Manifest file row: `asset, subject (pose|clip|asset), item, view, kind (glb|fbx|model|zip), file (relative, `/`), bytes, sha256, triangles` (triangles of the geometry the file holds or describes; null for the ZIP). Properties, each with a test in `🧪️tests/🧊️model-batch` (7 tests, 14 s): assets streamed one at a time (peak 686 to 856 MB resident for the whole catalog, one parsed asset and its files at a time), temp-and-rename writes, manifest rewritten after every asset, no time stamp in any byte (two runs identical), resume (second run writes 0 files; a damaged file rewrites itself and the ZIP; another scale or `--force` rewrites everything; a code change needs `--force`, the manifest records only the asset document, the settings and the generator version), orphans of an asset folder removed, every path checked against 256 characters (`model-batch-path`), progress `[asset i/n] id` and `[percent] phase` in the chosen language on stderr, cancellation by SIGINT, SIGTERM, SIGHUP/SIGBREAK, `--parent`, and the `JobControl` signal (exit 130, no temporary file, manifest of the finished assets, resumable).

`repo run @teaching/staffage:export-models` was tried once at the end: `Reading the project graph slice` ran for 100 s and was cancelled (the known defective graph read), so the target wiring is checked only as valid JSON and by the direct command above (the script body was not executed through `repo`).

Site: the site serves its exports from the browser (lazy exporters, blobs; no generated static exports), so no site hook was added. For the site agent: the layout above is the contract; a static download folder can serve `<asset>/<file>` with `manifest.json` as the index.

## 4. Whole-catalog run

| | |
|---|---|
| command | `bun S/🔨️modules/📤️export/📦️batch/🟦️.ts --lang en --force` (host shared, other agents running) |
| files | 1982 = 556 GLB + 359 FBX + 915 model JSON + 152 ZIP, plus `manifest.json` |
| bytes | 1 725 581 196 (1.61 GiB; ZIPs are stored copies of their files, about half of it) |
| time | 445 s wall (224 s user, 14 s system), 152 assets; the last 5 assets re-run after the FBX fix 16 s |
| peak memory | 856 MB resident (the validating guard: 2.6 GB) |
| validation of the whole output | guard run with `SEMIO_STAFFAGE_MODELS_DIR`: 153 tests green in 48 s; `T/verification-run.ts` audit over the same files: 0 findings in 152 assets (`🗑️generated/verification/audit2.jsonl`) |

Largest files: GLB `bookshelf` 3.6 MB; FBX `date-palm` 1.1 MB; ZIP `dairy-cow` 18.3 MB. Model JSON per asset is dominated by the clip documents (all seven variants and every frame).

## 5. Other test state

Staffage library suite once in full (100 files, 2488 tests): the failures that remain are outside this scope: `thermal-span` (cast schema requires `scale`), `course-coverage`, `plan-projection` (2), `multiview-sheet` (projection symbol), `dwg-reopen` (wasm), `export-performance` (pinned digests), `clip-sampling`, `continuous-rotation`, `representation-copied`, `cast-export` (4), `representation-resolution` (2), and `three-views` (3: herring-gull clip `landing` @ 3.20 s at 1:100 and 1:200 and seated-office-worker `settle-back` @ 6.99 s at 1:50 fail in `evaluate` with the 2D polygon union ("neither the edge-splitting nor the slab union closed a boundary"), seated-office-worker also an elevation/plan x spread, hosta-clump height 0.585 m against `heightM` 0.62). The 3D export does not use those 2D scale representations and is unaffected. The model suites (`model-catalog`, `-batch`, `-digests`, `-silhouette`, `-glyph-edges`, `-exports`, `-frame`, `-skeleton`, `-looks`, `-bounds`), `asset-validation` (73), `schema-conformance`, `schema-twin`, `volume-schema`, `solid-sweep` pass (the digest fixture was regenerated after the FBX fix: only the FBX object counts and byte lengths changed). Library `tsc`: 1 pre-existing error (`cast-provenance`), kernel `tsc` 0.

## 6. Remaining limitations (plain)

- Silhouette agreement: 15 of 152 volumes miss 0.80 IoU or 8 % in a view in the default pose (`volume-agreement.md`); 14 are 2D drawing simplifications (sections, lattices or foliage as filled area, inconsistent views) and were left, the date-palm bounding box was fixed. The reason categories are inferred from the worst shapes, not verified against the artists' intent.
- Watertightness is asserted per solid, not for the union (solids overlap by design); the check welds at 0.1 micrometre.
- FBX: lines (feature edges) are not written, unlit is approximated, events and loop are not representable (kernel notices); three's loader logs two benign warnings for the colour curves. Blender, ufbx and the Autodesk SDK were not available as oracles.
- Resume does not detect code changes (use `--force`); the manifest `triangles` of a JSON row is that of its sibling GLB or FBX.
- `repo run` could not be completed (graph read); the guard run in the package needs about 2.6 GB.
- The guard exports each asset itself when no output directory is given (about 8 minutes for the catalog); the default `repo run test` therefore got slower by that amount.
- `hedgehog` height is +11.4 % relative (14.8 mm), accepted by the 15 mm floor.
