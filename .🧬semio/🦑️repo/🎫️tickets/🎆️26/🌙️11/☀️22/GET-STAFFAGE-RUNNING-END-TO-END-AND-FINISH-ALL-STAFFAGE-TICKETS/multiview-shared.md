# 🔭️ Multiview Coverage: Shared-Code Repairs (2026-11-24)

Paths: `R` = repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `K` = the multiview ticket of 2026-11-02, `U` = this ticket folder. Plan: `U/multiview-diagnosis.md` section 8, steps 1 to 3. No asset under `S/🗂️catalog` was touched.

## 1. Result

- The four shared-code repairs (paper box, diagram cue, anchors on descendant bones, twin tier), the survivable generator, the authors' gate and the band-copy allowlist are landed, each with a vector in the fixture, a scenario in `T/🥒️.feature` and a case in `T/🟦️.ts`.
- `T/🟦️.ts`: 23 of 23 before, **37 of 37** after. `tsc` of the staffage package: 0 errors.
- The suites that cover `structural-view`: 23 files, 481 tests, 479 pass. No pinned oracle changed. The two that fail are in `🧊️chain-state` and do not reach the structural view (section 5).
- Whole catalog, one scan process: 375 assets, **206 with a hard finding, 169 clean** (diagnosis this morning: 249 and 126; predicted after the shared repairs alone: 227 and 148; the rest of the difference is the fixers' work of today) (section 6, table in `U/multiview-scan-after-shared.md`, data in `U/🧰️matrix/scan-after-shared.json`).

## 2. Decisions recorded

The coordinator announced two decisions as "D46" and "D47". Both numbers were already taken in `K/🗂️work-board.md` (D46 `lateralScale`, D47 `sweep`), so they are recorded under the next free numbers, each with a note of the announced number:

- **D48** (announced as D46): the hard per-bone tier of D41 holds for a shape and its twin of the same id; a bone whose single shapes in the two views carry different ids goes to the reported `group-drift` tier. The anchor test accepts the shapes of the anchor's bone and of its descendant bones.
- **D49** (announced as D47): `derived-view-band-copy` gets an allowlist per asset, view and pair of bands with a physical reason in English and German.

## 3. Changes per file

### `S/🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts`

- `drawDiagram`: the area of a member diagram (bending moment, shear, axial) is hatched by its sign when patterns are on and the member's invariant maximum is above 15 % of the ordinate maximum: diagonal hatch (`compression`) for negative, dots (`tension`) for positive. This is the rule the member fills have.
- The ring of the line-of-sight symbol of a strong member gets the same hatch. Without it a view that sees the member end-on showed a strong value by colour only (measured: `used-sun-lounger-bench` front and plan, `market-parasol` and `reclaimed-steel-stair` plan at 1:500).
- `PATTERN_FRACTION` and the new `signPattern` are exported; the member fills use `signPattern` too.
- Legends: the member legend now ends with "weaker values are drawn unhatched" / "schwächere Werte bleiben unschraffiert"; the diagram legend states the 15 % rule for the area and the ring in both languages.
- Found on the way and fixed: `drawDiagram` read `ctx.language` for the label text even when labels are off, so a textless moments scene with a diagram threw ("a textless depiction has no language"). The reading is now built only when labels are on. `🤐️textless-depiction` had two failing cases because of it; both pass now.

### `T/🧰️.ts` (engine)

- 5.1 paper box: `paperIds` and `paperBox` read the fill (`shape:<id>`) and every visible outline run (`outline:<id>:<n>`) of the shapes of the view. `paper-width` and `paper-height` use them (`paperFindings`). The `blank-shapes` check is unchanged.
- 5.2 cue rule: `cueFinding` is one rule for thermal, forces and moments. A scene without hatch pattern and dashed stroke passes only when its legend states, in English and German, the share below which a value is drawn unhatched (`UNHATCHED_PERCENT`: thermal 35, forces 15, moments 15). Otherwise it fails as `analysis-cue-legend`. The code `analysis-cue` no longer exists.
- The checks are exported as pure functions so that the gate runs exactly the checks of the matrix: `geometryFindings`, `paperFindings`, `cueFinding`, `derivedFindings`. The collector calls them in the old order; codes, messages and tolerances are unchanged apart from the decided rules.
- `failedAsset`: the cells of an asset whose run did not end, failing under a given name with the message.
- `runAsset` takes an optional `onEntry` callback (progress per scale entry).
- D49: `allowlists()` reads `derivedViewAllowlist` and `bandCopyAllowlist`; `derivedFindings` honours both. An entry without a reason in both languages allows nothing.

### `T/⚓️landmarks.ts`

- Cause A: `anchorLandmarks` measures the distance to the shapes of the anchor's bone and of its descendant bones (`subtrees`). The condition "in every view in which the anchor's own bone is drawn" is unchanged, so the change only removes findings.
- Cause H: `boneSync` puts a bone in the hard tier only when its single shapes in the two views have the same id; otherwise it is `group-drift` and the message names the two parts.

### `T/📊️matrix.ts` and new `T/🗃️cells.ts` (generator)

- Every asset runs in a process of its own and writes `<out>/cells/<asset>.json` when it ends (temporary file, then rename).
- Resume: a rerun skips every asset with a current record. A record carries the digest of the asset document; when the asset changed since, it is run again. `--retry-failed` reruns named failures, `--fresh` everything.
- Named failures: an error thrown inside an asset, a process that dies and an asset that exceeds `--timeout-s` (default 7200) are recorded as `asset-crashed` or `asset-timeout` with the message and the scale entry reached. Their cells fail under that name; only an asset no folder holds is `cell-missing`.
- Merge: `--merge-only` with `--from dirA,dirB` merges the records of separate invocations; `--shard i/n` runs a part and does not merge. `--categories` selects categories (humans and vegetation on their own shards).
- Progress on stderr per ended asset with the remaining time, and in `<out>/progress.json`. SIGINT and SIGTERM stop the running assets, keep the records and exit 130.
- `--language en|de` is required; progress and summary lines exist in both languages.

### New `T/🔎️scan.ts` (authors' gate)

Usage is documented at the top of the file. In short, from `T` with the pinned bun:

```
bun 🔎️scan.ts --language en <asset id> [<asset id> …]
bun 🔎️scan.ts --language en --all --out scan.jsonl --report scan.json
```

- It calls the engine's own functions: every pose of every authored scale through `geometryFindings` (synchrony, twin tier, anchors, dimensions), the default pose through `derivedFindings` (band copy and in-band, both allowlists), `paperFindings`, the resolver's availability and `cueFinding` for thermal, forces and moments.
- Each finding names code, family, tier, scale, pose, views and, where it has one, bone, anchor, anchor kind, the place the anchor lies instead (another bone or off every shape) and the delta in millimetres. The listing groups them per code, views, bone and anchor with poses, scales and the worst case.
- Exit code 0 clean, 1 hard finding, 2 usage, 130 cancelled. `--out` is resumable.
- It leaves clip instants, transitions, range ends, symbols and German renders to the matrix. A clean scan is necessary for a clean matrix, not sufficient.
- Cost: under a second to about 20 s for most assets; trees and some people take one to four minutes on this host under load. The time goes into the cold evaluation of the four scales and the line render that the paper check needs (measured: `dawn-redwood` 21 s per scale for the line render, `cyclist` 17 s for the first evaluation).

### `T/🟦️.ts`, `T/🥒️.feature`, `T/🐍️.py`, `S/🧫️fixtures/🔭️multiview-coverage/🔣️.json`

- 14 new cases (23 to 37), 7 new scenarios (14 to 21).
- Vectors: `paper.outlined`, `landmarks`, `boneSync`, `analysisCue`, `generator`, `scan`, `allowlists`, and the empty `bandCopyAllowlist`. `statuses` now lists the five statuses the engine has.
- The oracle `🐍️.py` gained: svgelements box of the paths whose id matches (paper box), svgelements boxes of pattern-filled paths (diagram hatch), shapely distance from a point to polygons (anchors), shapely bounds (bone extents), and a plain JSON read of a record file.
- The fixture file had CRLF line ends; it is LF now like the rest of the suite.

## 4. Allowlist entry shapes (for the coordinator's merge)

Both lists are top-level arrays of `S/🧫️fixtures/🔭️multiview-coverage/🔣️.json` and start empty.

```json
"derivedViewAllowlist": [
  { "asset": "<asset id>", "view": "front" | "plan", "reason": { "en": "…", "de": "…" } }
],
"bandCopyAllowlist": [
  { "asset": "<asset id>", "view": "front" | "plan", "bands": ["<scale key>", "<scale key>", …], "reason": { "en": "…", "de": "…" } }
]
```

- `derivedViewAllowlist`: allows `derived-view-in-band` of that view of the asset in every band (front or plan equal to the elevation, or plan equal to the front).
- `bandCopyAllowlist`: allows `derived-view-band-copy` of that view between any two of the named bands and no other. `bands` holds two or more different scale keys as strings ("50", "100", "200", "500" or a declared key) in any order. A pair per entry (as MB-1 writes) and a group of three per entry (as MB-2 writes) are both valid; the coordinator's wording was "band pair", the group form states the same pairs in one entry.
- An entry whose `reason.en` or `reason.de` is missing or blank allows nothing.
- A case of the suite checks every entry: catalog asset, view `front` or `plan`, both texts, two or more different authored bands.
- The fixers' files already have these shapes: `U/🧰️matrix/MB-1/allowlist.json` (`{ "bandCopyAllowlist": […] }`, pairs), `U/🧰️matrix/MB-2/allowlist.json` (both lists, groups of three bands) and `U/🧰️matrix/MS/allowlist.json` (a bare array of `derivedViewAllowlist` entries, to be put under that key). Checked without merging them (`U/🧰️matrix/allowlist-check.ts`): 15 and 48 entries for 34 assets; the derived-view findings of those assets go from 109 to 0. I did not judge the reasons.
- Note for the merge: the `reason` of `derivedViewAllowlist` was a plain string in the old type. The list was empty, so nothing had to be converted.

## 5. Proof

Each run to the end with the pinned bun from `S/📦️packages/🟦️typescript` (suites) or `T` (CLIs).

| Run | Result |
|---|---|
| `vitest run … 🔭️multiview-coverage/🟦️.ts` before | 23 passed, 171 s |
| the same after | 37 passed, 192 s (run again after the last allowlist change: 37 passed, 169 s) |
| `tsc --noEmit -p tsconfig.json` | 0 errors |
| 23 suites around the structural view, serial (`🏗️structure-projection`, `🎨️structure-legend`, `🎨️seven-modes`, `🎨️view-synchrony`, `🤐️textless-depiction`, `🧾️legend-units`, `📑️multiview-sheet`, `🧊️axonometric-depiction`, `🦴️lateral-rest`, `🧊️model-glyph-size`, `⚖️equilibrium`, `🌉️simply-supported`, `🏗️rigid-trees-3d`, `🧊️chain-state`, `🔤️caption-fit`, `🏗️cantilever`, `🏗️structure-oracle`, `📕️pdf-reopen`, `📐️dxf-reopen`, `🏷️revision`, `🧊️model-looks`, `🧊️model-exports`, `🧊️model-scales`) | 479 of 481 passed, 584 s |
| scan CLI on the 12 cue assets, wheel-loader, red-oak, tram, used-steel-oil-storage-tanks | exit 1 as expected: 12 clean, 4 with band-copy or in-band findings only |
| matrix on 3 small assets, cut and resumed | below |

**Pins.** No pinned oracle, digest or snapshot changed. The suites that pin legends or scenes of the moments view passed unchanged. Two cases of `🤐️textless-depiction` went from red to green (the label bug above).

**The two red cases** are `🧊️chain-state` "ranks the right of the asset nearer in the elevation…" (`person-with-umbrella`) and "leaves every view of person-on-crutches unchanged…". The file imports evaluation, rig, animation and the depth module, not the structural view, and both measure catalog assets that are being repaired today. They were red with the same numbers in both runs of today. Not caused here, not fixed here.

**Scan CLI, expected clearings** (before: `multiview-diagnosis.md` appendix A; after: this run):

| Asset | Before | After |
|---|---|---|
| wheel-loader | 170 anchor findings, all on a descendant bone | clean |
| red-oak | 12 anchor findings, all on a descendant bone | clean |
| tram | 27 per-bone findings, all "other shape id" | clean; 108 reported as group drift |
| used-steel-oil-storage-tanks | `paper-width` in 56 matrix cells | clean |
| finned-heat-exchanger, market-parasol, reclaimed-steel-stair, shopping-trolley, tilt-window, used-brine-water-heat-pump, used-sun-lounger-bench, hot-water-cylinder | no moments cue (and, for some, findings the fixers have since repaired) | clean |
| pendant-lamp, tealight, used-bollard-light | no moments cue, plus derived views | cue cleared; in-band front remains (asset or allowlist work) |
| storage-crate-stack | no moments cue, plus band copies | cue cleared; 4 band copies remain |

How the 12 cue assets clear (`U/🧰️matrix/cue-after.ts`): market-parasol, reclaimed-steel-stair, shopping-trolley, tilt-window and used-sun-lounger-bench have a strong diagram and get hatched areas or rings. The other seven have no strong coloured value on the sheet and pass by the legend rule. In hot-water-cylinder the only loaded bone is drawn neutral because it is not an eligible member, as the diagnosis said.

**Matrix, three small assets** (electric-fan-heater, finned-heat-exchanger, used-cast-iron-sectional-boiler; `U/🧰️matrix/interrupt-resume.py`):

- Cut by SIGINT after the first record: exit 130, one record left, no `matrix.json`.
- Second run: "2 assets to run, 1 finished ones skipped"; then 735 cells: 602 ok, 63 unavailable, 70 fail; 3 done, 0 failed by name, 0 missing. The 70 are `anchor-landmark` of electric-fan-heater (asset data, cause C). finned-heat-exchanger had 33 failing cells in the diagnosis sample and has none.
- `--timeout-s 3` on two assets: both recorded as `asset-timeout` with "the asset did not end within 3 s (scale entry 5/11 100)"; all 483 cells fail under that name, none is `cell-missing`. A plain rerun skips them; `--retry-failed` runs them to `done`.
- Two shards by separate invocations into two folders, then `--merge-only --from`: the same 735 cells as the single run. Merging one folder alone reports the third asset as missing.
- The scan itself was cancelled once during the catalog run and resumed from its output file.

## 6. Whole catalog after the shared repairs

One process of `🔎️scan.ts --language en --all`, 13:04 to 14:39, 5,505 s of scan time for 375 assets (14.7 s per asset; cancelled once at 13:19 and resumed at 13:23 to profile it). Per-asset table: `U/multiview-scan-after-shared.md`. Data: `U/🧰️matrix/scan-after-shared.json`. Both allowlists were empty during the run.

| | Diagnosis scan, this morning | Predicted after the shared repairs | Measured now |
|---|---|---|---|
| Assets with a hard finding | 249 | 227 | **206** |
| Clean assets | 126 | 148 | **169** |
| Assets with a synchrony finding | 168 | 158 | 151 |
| Assets with an anchor finding | 133 | 102 | 94 |
| Assets with a band copy | 37 | 37 | 15 |
| Assets with an in-band derived view | 20 | 20 | 17 |
| Assets without a moments cue | 12 | 0 | 0 |
| Per-bone synchrony findings (hard) | 10,151 | about 8,100 | 8,077 |
| Anchor findings | 6,866 | about 2,900 | 2,852 |

- The measured numbers are below the prediction because the asset fixers worked on the catalog between the two scans (band copies 37 to 15 is their work alone). The two scans are therefore not a clean before and after of the shared code; the per-code effects of the shared code are the four rows where the code vanished or changed kind: no cue finding, no anchor "on a descendant bone" (3,930 this morning), the "other shape id" per-bone findings moved to group drift (2,013 this morning), one paper asset cleared.
- Hard findings now: 11,660. By family: synchrony 8,701 in 151 assets (305 at rest in 61 assets, 8,396 in other poses), anchors 2,852 in 94 (2,144 on a shape of another bone, 708 off every shape; emitters 1,653, contacts 1,091, attachments 108), band copies 76 in 15, in-band 28 in 17, paper 3 in 2, cue 0.
- Reported only: 10,500 group-drift findings in 207 assets.
- Paper, now measured for the whole catalog (the diagnosis had left it open): two assets. `person-taking-selfie` front at 1:200 (2.35 mm drawn, 2.42 mm by the dimensions) and `used-harmonium` elevation at 1:50 and 1:100 (22.54 mm against 23.00 mm). Both are asset data: the drawing is smaller than the dimensions say by just over the tolerance.
- People stay the bulk: 71 of 75 with a hard finding, all with synchrony findings.
- With the fixers' three allowlist files merged, 20 more of the 206 would lose their derived-view findings (section 4); how many of them become clean depends on their other findings.

## 7. Open points

- A crash of the asset process itself (signal, out of memory) is handled by the same path as the timeout but was only exercised through the timeout and the unit case for a thrown error.
- The record digest covers the asset document, not the engine. After an engine change a full run needs `--fresh`.
- The scan is slower than the diagnosis script because it also renders the line view of every authored scale for the paper check.
- The catalog changed while the scan ran (other workers repair assets). Each row is a snapshot.
- The scan found no case of the allowlists in use, since both are empty; the fixers' entries wait for the coordinator's merge.
- Tool output of this work was under `U/🗑️generated/matrix-shared/` and is deleted. Kept inputs: `U/🧰️matrix/scan-table.py`, `cue-after.ts`, `interrupt-resume.py`, `allowlist-check.ts`, `scan-after-shared.json`.
