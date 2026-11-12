# Staffage LoD design (staffage owner, phase 1 analysis, no source edits yet)

`$T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️11/DECLARATIVE-TEACHING-MORPHS-AND-FAST-PREVIEWS`. Scripts: `$T/staffage-measure.py`, `staffage-motion-cost.py`, `staffage-catalog-census.py`, `staffage-composition.py`; raw output `$T/🗑️generated/staffage/{measure,motion-cost}.json` (delete when done).

## 1. Today's pipeline (traced)

```
clip .py  --literal cast id-->  staffage()/staffage_motion()/fitted_staffage()/staffage_height|width|parts()
   (🎨️visuals/🐍️.py, re-exports)          |
                                           v
🎨️visuals/🧍️staffage/🐍️.py  ensure_casts(ids)
   _manifest()  = 🎨️visuals/🧍️staffage/🔣️.json   (99 casts; schema 🎭️cast/🧬️schema/🔣️.schema.json, mirrored by entry_issues())
   cast_key(entry) = sha256(entry JSON + asset catalog-file revision + environment preset revisions + _source_digest())[:32]
   _source_digest  = every file under 🧍️staffage/🔨️modules/** and 🧬️schema/** + print-profiles (84 ms once per process)
   folder = <cache>/staffage/<castKey>/<castId>/{🔣️.json, NNNN.svg}   cache = .🧬semio/🎓️teaching/⚡️cache (SEMIO_TEACHING_CACHE_ROOT)
   missing -> per-cast OS lock -> ONE `repo tools exec --light bun -- 🎭️cast/🟦️.ts --manifest tmp --ids a,b --out tmp` (all missing casts in one batch)
              -> atomic rename into cache -> _prune (24 h) -> _provenance() check against manifest+catalog
TS 🎭️cast/🟦️.ts exportCast: loadCatalogFromFiles (ALL 152 assets, 40 MB JSON) + loadEnvironments
   per cast, per frame: evaluate(asset, query, {kind:"scale"}) -> solveThermalAt (session) | solveStructure | solveTransport
                         -> depict(textlessDepictionOptions, annotations off, transparent) -> writeSvg -> union viewBox/origin alignment
   frames: SHA-dedup; schedule = thinned(frameSchedule(clip, fps)) (cap 600 samples, fractional fps), index 🔣️.json
Python playback: _Motion(_FigureBounded VGroup): _slot_count() reads EVERY frame file, one slot VMobject per max path, _show(frame) re-parses (regex SVG, LRU 256) and rewrites all slots; playback_frame(t,fps,count,duration,loop) = floor(t*fps) / wrap / hold
```

Axes today (all fused into one manifest entry; the cast id is the only handle clips have):

| Axis | Where resolved today | Independent? |
|---|---|---|
| ID/revision | `entry.asset` + catalog file sha256 (`asset_revision`), env preset canonical-JSON sha256 | revision yes (content addressed) |
| view | `entry.view` (7 styles), `entry.viewpoint` (elevation/front/plan), `entry.theme`, `mirror` | yes, but only as manifest data; plan/front have 6 casts only |
| authored drawing scale | `entry.scale` in 500/200/100/50 -> `pickRepresentation` + `measureSuitability` (over ALL poses, per asset per process) AND `mmPerMetre = 1000/scale` AND pens | NOT independent of geometry source (see 4). 96/99 casts are 1:50, 3 are 1:100 |
| pose/animation | `entry.pose` xor `entry.clip`; clock = `fps` + `transportAgeS` offset | clock yes |
| scenario | thermal: `environment` xor `timeline`, `thermalSpanK`, `transport[]`; structural implied by view forces/moments | yes, recorded in provenance.scenario |
| frame count/rate | `entry.fps` (15 for all 50 clips) -> `frameSchedule`; >600 thinned; baked into cache key and files | NOT independent of cache key, no lower-rate variant exists |
| resolution | none: SVG vectors, Manim `unit` (Manim units per metre) applied at placement | n/a (raster only in React downloads) |

Output provenance (`index.provenance`, schema `Provenance`) lists asset id+category+revision, view, scale, state, clock, scenario. There is NO `lod` and NO profile dimension.

## 2. Measured cost (warm cache, this host; cold TS export not measured: heavy job, see Open 8)

Cache: 97 of 99 casts present (`shared-person-relaxed-line`, `bim-consistency-decider` not rendered yet), 269.5 MB, 15,294 frames (no dedupe win: all unique). 50 clips, median 106 frames, **22 clips have 600 frames = 13,200 of 15,247 clip frames (87%) and almost all bytes** (wheel 26.9 MB, vacuum 19.6 MB, shared-radiator-thermostat-front 12.1 MB). Poses: 49 casts, 1 MB total, median 24 paths. Paths in frame 0: median 28, max 752 (transport pathline casts 376-465; `outline-honeycomb` 156 on the sorption wheel).

Per-call bridge cost (`staffage-motion-cost.py`):

| cast | frames | ensure_casts warm | _slot_count (reads all frames) | staffage_motion ctor cold slots / warm | fitted_staffage (probe + real) | frame swap cold / warm |
|---|---|---|---|---|---|---|
| walker (thermal clip) | 18 | 3.9 ms | 8 ms | 27 / 23 ms | 50 ms | 21 / 5 ms |
| laptop (pose) | 1 | 0.6 ms | 0.2 ms | 7 / 7 ms | 13 ms | 5 / 1.2 ms |
| summer-strategy-wheel | 600 | 0.6 ms | **126 ms** | **154** / 40 ms | 84 ms | 37 / 8 ms |
| vacuum | 600 | 0.8 ms | 118 ms | 141 / 23 ms | 46 ms | 20 / 5 ms |
| ventilation-volume-cook (465 paths) | 60 | 1.0 ms | 19 ms | 111 / 109 ms | 196 ms | 104 / 25 ms |

So warm decode is cheap per cast but wasteful: `fitted_staffage` builds a complete `_Motion` (slots, frame 0 parse) only to read `.width/.height` and throws it away (build-high-detail-to-discard), and `_slot_count` byte-reads every frame at construction. The dominant cost is cold export (frames x thermal history integration + depict + SVG). Because `_source_digest` hashes ALL of `🔨️modules/**` (including dxf, pdf, dwg, zip, video, sheet, cli), any edit of any module re-keys and re-exports all 99 casts (about 15k frames) once; the digest should cover the cast dependency closure only.

Lower-detail representations that exist: none for casts (one full SVG set per cast). The archive does hold authored simplified geometry per scale (1:500 present in 108/152 assets, 1:200 in 146, 1:100 in 152) with coarser pens (e.g. standing-adult elevation shapes 58 at 1:50, 31 at 1:100, 20 at 1:200, 13 at 1:500; `PenSet` null = role not drawn), plus `symbols` (46 assets) which are paper-mm conventional pictograms (`fidelity: conventional-symbol`, not true extent): symbols are NOT acceptable LoDs (they change dimensions).

## 3. Where zoom, print scale and render detail are conflated today

1. `resolveDepiction` derives the geometry/pen source from `options.scale.denominator`, the same field that sets `mmPerMetre`; there is no separate "geometry detail" input. A cheaper geometry therefore cannot be chosen without changing paper units (`depict()` L~150, `🖨️representation/🧭️pick`).
2. `entry.scale` (print detail) is the only detail knob for clips, and all clips use 1:50 (hatches, texture pens 0.1 mm) even when the figure is drawn 0.8 Manim units tall (`watt_anchor`, `fitted_staffage(width=0.8)`).
3. Viewport zoom leaks into pens: `_style` = `max(stroke_mm * unit/mmPerMetre / 0.01, 1.0)`: the Manim `unit` (zoom) rescales the authored print pen and the hard floor `_MIN_STROKE = 1.0` overrides it; this legibility rule is hidden in code instead of being data.
4. `fps` is simultaneously clip sampling rate, cache key part, file count and playback map; a 15 fps cache cannot serve a 5 fps preview and a preview cannot be a strict subset by name. `thinned()` changes fps fractionally for >600 frames, so "frame i" is not a stable canonical index.
5. `measureSuitability` measures every named pose of an asset at export start (print suitability), irrelevant for a figure rendered tiny on screen.
6. Dimensions (`staffage_height/width`) and anchor polygons (`staffage_parts`, straight-line regex only) are read from the same heavy frames; there is no LoD-independent physical descriptor.

## 4. Design: one shared resolver

### 4.1 Concepts kept independent (and where each is data)

| Concept | Field | Owner | Changes |
|---|---|---|---|
| canonical reference | `cast` id (+ asset `revision`) | cast manifest (unchanged, no LoD in entries) | identity, view, authored scale, state, scenario, clock |
| viewport zoom | Manim `unit` / React CSS size | placement only | never selects data |
| authored print scale | `entry.scale` | manifest | mm/metre, authored geometry+pens |
| render LoD | `lod` id from the render profile | NEW levels file | frames sampled, shape roles kept, transport density, thermal discretization, stroke floor |
| profile | `preview` / `final` | profiles owner (`🎚️profile`) | maps to `lod` id only; staffage never reads other profile fields |

Resolution of the physical descriptor never depends on LoD: `heightM`, rig bounds, `figureBoundsM`, `originMm`, `mmPerMetre`, ground contact (origin on feet), clock (`durationS`, `loop`, `offsetS`) and `partsM` (named polygons used as ray starts/labels) are identical for every LoD (test-enforced to 1e-6 m).

### 4.2 Resolver signature (Python, twin TS `resolveCast`)

```python
resolve(ref: CastRef, *, lod: str, profile: str, view=None, viewpoint=None, scale=None, pose=None, scenario=None) -> ResolvedCast
```

- `CastRef = {cast: id, revision: "sha256:..."|None}`. The cast id fixes (asset, view, viewpoint, theme, mirror, scale, state, scenario, clock). The optional `view/viewpoint/scale/pose/scenario` arguments are DECLARED expectations (from a transition document `source`): the resolver checks them against the manifest entry and raises an actionable error naming the nearest casts of the same asset that do match (no silent fallback, no free combinations; the manifest stays the enumeration of supported combos, which is what "missing combinations produce validation errors" needs).
- `lod` must be a level id of `🧊️lod/🔣️.json`; `profile` is recorded in the result and counters (`instrument.count("asset.load", lod)`), not part of the cache key; `final` profile requests `full` only, a folder whose `index.lod.id != requested` is rejected.
- Returns `ResolvedCast(cast, key, lod, folder (path only, nothing decoded), descriptor (heightM, boundsM, figureBoundsM, partsM, originMm, mmPerMetre, clock, maxPaths), frameCount, provenance)`. Reading the descriptor is one small JSON read; frames are decoded lazily by `_Motion` after construction. `_slot_count` becomes `index.maxPaths`; `fitted_staffage` sizes from the descriptor (no probe mobject).
- Selection of the cheap representation happens BEFORE any decode: `resolve` computes `lodKey`, looks up `<cache>/<castKey>/<castId>/<lodId>-<lodRev8>/🔣️.json`; only on a miss it runs the exporter for that LoD alone (never `full` first).

### 4.3 LoD levels (data, schema-first): `🧍️staffage/🔨️modules/📤️export/🎭️cast/🧊️lod/🔣️.json` + `🔣️.schema.json`

| id | frames | roles drawn | patterns | transport | thermal discretization | use |
|---|---|---|---|---|---|---|
| `full` | canonical schedule, cap 600 (today) | all | yes | as authored | exact | final |
| `preview` | fps cap 5, frame cap 90, last sample = canonical end, loop wraps whole periods | `body`, `glazing`, `ground` (drop `detail`, `texture`) | no | density 0.25 of parcels/pathlines (deterministic stride by id) | `thermalMaxSteps` reduced (tolerance-tested, see 5) | preview/scrub |
| `poster` | 1 frame at canonical `posterTimeS` (default 0, clips may name a time) | as `preview` | no | off, except it keeps source marker if the cast defines it | as preview | thumbnails, posters, drag proxies |

Numbers are initial; Open 2. Each level also carries `strokeMin` (today's hard-coded `_MIN_STROKE`) so the legibility floor is data, not zoom code.

Genuine, not substitute: a level only drops entities from the same authored geometry and samples the same analytical state at canonical times. Because the exporter evaluates at `t` analytically (thermal history is a pure function of `(asset, query, environment)`, `ThermalSession` is documented bit-identical to solving without one), any subset of canonical times gives bit-identical values at those frames; skipped frames cost nothing. Pose attachment, causal timing (source timing, `transportAgeS` warm offset), units, legend and span (`thermalSpanK`) are unchanged. Frame times: non-loop `times[k] = k*D/(m-1)` (first and last both exact, so the terminal pose/state is exact); loop `k*D/m`; `fps_eff = m/D` or `(m-1)/D` is written to the index, so `playback_frame(t, fps_eff, ...)` is unchanged. Variable-delivery cadence stays correct because playback is a pure function of canonical time.

Tier 2 (optional, Open 3): a `detailScale` field (new `DepictionOptions.detailScale`, default = `scale`) selecting a coarser AUTHORED geometry source (1:100/1:200) for far/thumbnail use while `scale` still sets mm/metre. It is only legal where the coverage test proves bounds (<=1e-3 m), part ids and thermal region coverage equal. Never `symbols`.

### 4.4 Coverage and validation (central)

- Vitest (existing runner, new file `🧪️tests/🧊️cast-lod/🟦️.ts` + feature): for EVERY cast in the manifest x EVERY level: schedule invariants (first/last time, loop seam, count caps, endpoint equal canonical), `resolveCast` returns the same descriptor, role mask keeps >=1 painted shape per thermal region (so heatmap values survive), preview bounds == full bounds (1e-6), named part ids present, `authoredRepresentation` exists. No frame rendering required (`evaluate` + `prepare` only), so it runs in seconds.
- Oracle: frame `k` of `preview` thermal equals frame `j` of `full` at the same canonical time to a stated tolerance (exact for same discretization); independent recomputation with `solveThermal` without session is already the repository oracle. Structural/forces: same via `solveStructure`.
- Python (`🎨️visuals/🧪️tests/🐍️staffage_lod.py`, added to `📜️script.ts` list): every literal cast id found by the `staffage_only` AST scan (all 99 in the course, literal-only is already a guard) x every level resolves and, for the exported ones, `_provenance` matches including `lod`; no clip names an `lod`/`profile` argument (guard rule added to `🐍️staffage_only.py` next to the `unit` literal rule); `bind_staffage` of the stage uses `resolve` only.
- Language-agnostic vectors: `🧍️staffage/🧫️fixtures/🧊️lod-resolve/🔣️.json` (valid levels with expected schedules for 5 representative clips/poses, invalid: unknown lod, unknown cast, declared view/scale mismatch, final profile with preview lod); both Vitest and unittest run them (twin pattern as `🎬️playback-clock`).

### 4.5 Cache keys

- `castKey` = today's formula but `_source_digest` narrowed to the cast closure (`🎭️cast`, `🧮️evaluation`, `🎨️depiction`, `🌡️thermal`, `💨️transport`, `🏗️structure`, `🏷️revision`, `🗂️catalog`, `🖨️representation`, `🎞️frames`, `🖼️svg`, schema) so editing dxf/pdf/zip/video/sheet/cli no longer re-keys the course. (Edits to lod/exporter DO re-key: one full cold rebuild, queue it.)
- `lodRev` = sha256 of canonical JSON of the resolved level object (all numeric params) + `lodSchema` version, 8 hex chars.
- Folder `<cache>/staffage/<castKey>/<castId>/<lodId>-<lodRev>/{🔣️.json, NNNN.svg}`; lock per `(castKey, castId, lodId)`; `index.lod = {id, revision, canonicalFrames, sampledFrames, canonicalFps}` and `provenance.lod`; `_provenance()` compares `expected_provenance(cast, lod)`.
- Preview folders are bounded: prune rule keeps the newest `castKey` per cast per lod; `preview`/`poster` budget (bytes) is asserted by the test (`<= full/8` as a gate to be tuned). Preview output never satisfies `final` because the folder name and `index.lod.id` are part of the lookup and the verification.
- Clip render keys (`🎬️clip/⚙️runtime` `casts_in`) get `lod` through the profile fingerprint (dependency on profiles owner): `casts_in(sources, lod)` returns the lodRev-qualified key.

### 4.6 Descriptor sharing (physical truth independent of LoD)

The exporter writes into every LoD index: `logical: {heightM, boundsM, figureBoundsM, originMm, mmPerMetre, partsM, maxPaths}` computed from the rig + `prepare()`d authored shapes over the CANONICAL time grid (`evaluate` only, no thermal solve, no depict: cheap) with the full role set, so it is LoD-invariant. `partsM` (named filled polygons of frame 0, world metres, mirror applied) replaces the regex `_parts` SVG parse in Python (that parser only supports straight `M/L/H/V/Z`), so anchors, ray starts and label points never depend on which SVG variant was loaded. `staffage_height/width/parts` read this.

## 5. Exact files to change

TS (`🎓️teaching/🏛️architecture/🧍️staffage/`):
1. NEW `🔨️modules/📤️export/🎭️cast/🧊️lod/🔣️.json`, `🔣️.schema.json`, `🟦️.ts` (levels, `resolveLod`, `lodSchedule`, `lodRevision`; pure).
2. `🔨️modules/📤️export/🎭️cast/🟦️.ts`: required `--lod`, `framesOf` through `lodSchedule`, role mask / patterns / transport density / thermal steps from the level, write `logical`, `index.lod`, `--parent` unchanged; drop eager catalog parse of unused assets if the catalog loader allows an id filter (measure first).
3. `🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json` + `🟦️.ts`: `LodId`, `Index.lod`, `Index.logical`, `Provenance.lod`.
4. `🔨️modules/🏷️revision/🟦️.ts`: `castProvenance(..., lod)`.
5. `🔨️modules/🎨️depiction/🟦️.ts`, `🧩️shapes/🟦️.ts`: `DepictionOptions.detail {roles, patterns}` (and `detailScale` if Open 3); `🔨️modules/💨️transport`: deterministic `density` option; `🌡️thermal`: expose `maxSteps` path already exists (`ThermalOptions.maxSteps`), only wire it.
6. Tests: `🧪️tests/🧊️cast-lod/🟦️.ts` + `🥒️.feature`, `🧫️fixtures/🧊️lod-resolve/🔣️.json`.

Python (`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/`):
7. `🧍️staffage/🐍️.py`: `resolve`, `ResolvedCast`, `lod` through `ensure_casts/_directory/cast_key/_lock/expected_provenance/_provenance/staffage*/_Motion`; remove `_slot_count` byte scan (use `index.logical.maxPaths`), remove `_parts` regex (use `partsM`), `_prune` per lod; `_STROKE_MIN` from the level; instrumentation counters.
8. NEW `🧍️staffage/🧊️lod/🐍️.py` (loads the shared levels JSON, schedule math twin of TS, validation) or kept inside 7 if it stays small; `🧍️staffage/🔣️.json` untouched.
9. `🐍️.py` (visuals): `fitted_staffage` sizes from `ResolvedCast.descriptor` (no probe), re-exports `resolve`.
10. `🧪️tests/🐍️staffage_lod.py` + `🔣️staffage_lod.json` (cases), `🧪️tests/🐍️staffage_only.py` (+ fixture rule: no clip passes `lod`/`profile` to staffage calls), `🧍️staffage/🧪️tests/🐍️.py` (lod provenance, folder isolation), `🏗️authoring/📜️script.ts` test list.
11. `🧍️staffage/README.md` (pipeline + axes + LoD table).

Not touching: profile JSON (`🎚️profile`, profiles owner, contract section 5) except to agree the `lod` ids; React `🎯️targets/⚛️react/.../🖼️raster` (raster resolution `rasterMaxPx` would live in the level; consumer is playback/exports owners).

## 6. Defects found (outside scope unless noted)

- `fitted_staffage` constructs a full `_Motion` just to measure it (build-to-discard); fixed by 4.2.
- `_slot_count` reads all frame bytes at construction (126 ms for a 600-frame cast); fixed by index field.
- `_source_digest` over all of `🔨️modules/**` couples unrelated exporters (dxf/pdf/video) to the cast cache: any edit re-exports everything. Fixed by 4.5.
- `staffage_parts` regex parser only handles straight `M/L/H/V/Z` (raises on curves); fixed by `partsM`.
- 2 manifest casts are not in the cache (`shared-person-relaxed-line`, `bim-consistency-decider`): rendered lazily on first use; the coverage test will force them in the validation run.
- Thermal frames never dedupe (15,294 unique of 15,294): the SHA dedupe saves nothing there; only fewer samples helps.

## 7. Open decisions for the coordinator

1. Approve a full cold re-export of all 99 casts for `full` (about 15k frames, 270 MB; exporter edit re-keys the cache) and of `preview`/`poster`, as heavy jobs via the validation queue (I will request `REQUEST staffage <exporter batch full+preview+poster> rekey`).
2. LoD numbers: preview fps cap 5 / frame cap 90 / roles body+glazing+ground / transport density 0.25 are my starting guess; confirm or let me tune after the first measurement (22 casts at 600 frames dominate).
3. Include tier 2 `detailScale` (coarser authored geometry) or restrict LoD to frame/role/transport/thermal discretization (simpler, my recommendation for the first iteration).
4. Confirm resolver contract: cast id + declared-axes check (no free axis combinations), and that profiles owner emits `lod` in {`full`,`preview`,`poster`}; `bind_staffage` calls `resolve` only.
5. Confirm the staffage descriptor (`logical`, `partsM`) is the authoritative source for `bind` bounds/anchors in the morph stage (contract section 3).
6. Who consumes `poster` (thumbnails/posters/export owners) and `rasterMaxPx` (React raster downloads).
7. Permission to narrow `_source_digest` (touches the clip-key/cast-key tests and `casts_in`).
8. Queue request for one cold TS timing baseline (not run: heavy): `repo tools exec --light bun -- 🎭️cast/🟦️.ts --manifest <3-cast manifest: walker, wheel, cook> --ids ... --out $T/🗑️generated/staffage/cold` to get seconds per frame for the rebase of budgets.
