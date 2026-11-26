FINAL

# 🧊️ Lane VOL: the 28 Missing Volumes in the Catalog (2026-11-25, closed 22:04)

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, prepared scripts `U/🧰️linux/s4/` (see `linux-s4-volumes.md`), lane scripts `U/🧰️matrix/VOL/`. Every run went through `repo capacity run` (800 MB scripts, 1000 MB check CLI, 2500 MB vitest with a name filter), at most two queue requests at a time.

## 1. Result

All 28 scale entries (27 assets at 1:500, `used-microwave-oven` also at 1:200) are authored in the catalog files and proven. The 15 findings at other scales are cured in the assets. One further asset defect, found only by the complete case, is cured too. Nothing is open.

| Proof | Result |
|---|---|
| Volumes in the catalog | 27 of 27 documents hold `volumes["500"]` `authored` (the oven also `["200"]`); written once at 16:40, all 27 keep that mtime until now except the one document of section 4 |
| `🧪️tests/🧊️model-catalog` "ships all four digital scales" | passes for all 27 together (1 passed, the 27 ids as the name list); `authoredScales` is `[50,100,200,500]` for each |
| Per-asset case (`offers the 3D formats … valid file set in every pose, clip, scale and mode`) | 27 of 27 pass on the current document (`catalog/done.txt` with the sha of the document; 26 from the sweep of 17:30 to 17:50, `used-wall-hung-gas-boiler` after its repair, section 4) |
| Check CLI `--asset <id>` | 27 of 27 `✓ (1/1)` (`validateAsset`: no finding, none new); the wall-hung boiler run again after its repair |
| `checkVolumes` at every authored scale, `volumeIssues`, `exportIssues`, geometry findings | 0 for all 27 (`volumes.ts`; the wall-hung boiler again after its repair) |
| `measure.ts` before and after (silhouette, validate, sweep) | the same as before the write for 27 of 27; one cosmetic IoU digit at 1:50 on `used-moving-boxes` (0.9974 to 0.9973, from its repaired flaps) |
| Suites with the 27 ids as name filter | `🧊️catalog-silhouette` 27 passed, `🧊️three-views` 54, `🌡️catalog-physics` 54, `🧊️volume-schema` 115 of 115, `🧊️volumes-rules` 128 of 128, `🧬️schema-conformance` 44 of 44; `🧊️model-bounds` 42 passed, 40 failed, none of the 40 names one of the 27 (vacuum cleaner, shopping trolley, street lamp, other assets: owners are other lanes) |
| Bodies looked at | 28 pictures rendered at 1:500 (six shaded views, `r3-sheet.ts`) and opened; 13 reworked and looked at again (sheets `n1`, `n2`, re-opened for this report: bedside table, organ, harmonium, towel radiator, boiler, dryer, rocker, cooker, oven, vanity, WC, heat pump and trumpet each read as their object at 1:500) |

## 2. Adopted Chains

At 18:57 the batch had 14 processes: the two sweeps of `🧊️model-catalog` over every other asset (`catalog.sh 2500`, `all.sh`, slices of 8 and 9 assets, results `catalog/done.txt`, 140 verdicts). They were a by-product of the predecessor, not part of the 28 volumes; the queue was starved (13 to 20 waiting, up to 1800 s per grant), so they were stopped at 19:03 together with their parents, and the nine claim folders of unfinished slices were removed so a later sweep can retake them. Verdicts of the sweep stay in `catalog/done.txt` (111 pass); `owners.py` lists the others by batch. Nothing of the 27 was running any more: the proofs were all finished.

## 3. Catalog State per Asset

27 documents, hashes at the end (`shasum -a 256`, first 16 hex digits): equal to those of the verdicts in `done.txt`, so every case verdict belongs to the document now in the catalog. No other lane wrote any of the 27 documents since 16:40 (mtime). Only `asset.volumes.500` (the oven also `.200`) differ from the backups, plus the four repairs of section 5 and the rename of section 4 (`paths.py`).

The bodies, one line each (judgement after looking): oven 1:200 and 1:500 with framed door, dark window, fascia; dishwasher casing with the door in its notch; djembe goblet and cajon; cooker with true 30 mm door, window, four plates; organ with 75 mm cheeks and four-legged bench; oil radiator envelope with control end; sewing machine on table; dryer with porthole ring; bedside table open shelf, four legs; high chair frames, seat, tray; heat pump open cabinet with compressor and exchangers; boiler block with panel and doors; inline pump with both flanges; copper coil ring in its cradle; panel radiator stack between end frames; towel radiator five bars between uprights; wall-hung boiler with flue and typified strap; accordion open case; harmonium stepped outline, gabled crest; moving boxes three stacked cartons; rocker with round handle bars; guitar closed case; trumpet case at true depth with leadpipe, tubes, valve block, flared bell; viola and violin open cases with instrument and bow; vanity with oval bowl mouth; WC turned oval bowl, tapered foot, rear body, lid.

## 4. The One New Defect: `used-wall-hung-gas-boiler`

The case failed at 17:38 with `expected [ …(36) ] to deeply equal []`: 36 `json … ajv /nodes/N/id must NOT have more than 64 characters` findings in the model files (6 per pose and clip). Cause, measured with `ids.ts` (the node ids of the resolved multi-scene model): the force arrows of the four pallet supports are named `<scale>-structure-arrow-force-reaction-<anchor>-<head|shaft>`, the anchors were `pallet-contact-<back|front>-<left|right>`, so 31 ids of 65 to 68 characters existed at 1:50, 1:100, 1:200 and 1:500 alike (7, 8, 8, 8). The defect predates the 1:500 volume (the backup of the document has the same anchors at the other three scales; `auditGeometry` of the s4 proof does not read model files, so it was invisible there); the 1:500 volume added the eighth only.

Repair, at the asset: the anchor and the four support ids lose `contact-` (`pallet-back-right`, `support-pallet-back-right`, 76 exact replacements in one write, the document read right before, JSON re-parsed; the longest node id drops by 8 characters and the schema accepts all). `U/🧰️linux/s4/used-wall-hung-gas-boiler.ts` carries the same four names, so it stays idempotent (`unchanged`). After: file audit 0 findings (`files.ts`: model JSON, GLB, FBX in every pose, clip, scale and mode, bounds `true stored`), `volumes.ts` 0, check CLI `✓`, and the full case passes (3 passed of 3: finds every asset, ships all four, the asset's case). The document now has the hash `5ce3d2b79f560df3`. Note for the batches `MC` and `ML-2`, which list this asset: its anchors and support ids were renamed; a matrix record of the asset is stale, and nothing else in `S` names them (grep over `🎓️teaching`).

## 5. The 15 Findings at Other Scales (cured in the assets)

| Asset | Findings | Cause and repair | Now |
|---|---|---|---|
| `used-cast-iron-sectional-boiler` | 4 `closed` (1:50, 1:100, 1:200) | the four middle sections were 120 mm on a 120 mm pitch: copies touched (overlapped edges); now 119 mm (the casting joint, `joints.py`) | case passes, 0 |
| `used-panel-radiator-pallet` | 4 `closed` (1:50) | radiator covers 100 mm on a 100 mm pitch; now 99 mm | case passes, 0 |
| `used-moving-boxes` | 6 `closed` (1:50) | flaps touching copies; now 164 mm, the carton seam | case passes, 0 |
| `used-trumpet` | 1 `budget` (structure triangles 10638 at 1:100, 12158 at 1:200) | joints of 60 mm and mass markers of 36 mm at 1:200 (`trumpet.py`): 10462 at 1:200 | case passes, 0 |

These are asset defects, none belongs to the engine; the repairs touch only these four documents at those scales. `volumes.ts` at 1:50, 1:100 and 1:200 for the four: `volumeIssues`, `exportIssues`, geometry and `checkVolumes` all 0.

## 6. Housekeeping

Scripts kept in `U/🧰️matrix/VOL/` (`apply.sh`, `proof.sh`, `volumes.ts`, `paths.py`, `joints.py`, `trumpet.py`, `catalog.sh`, `case.sh`, `cli.sh`, `diag`-scripts `files.ts`, `ids.ts`, `run1.sh`, `sheets.sh`, `look.sh`). Tool output under `U/🗑️generated/matrix/VOL/` removed except `catalog/done.txt` and `catalog/shas.txt` (the verdicts of the sweep, read by `owners.py`). No test, fixture, `T/📊️matrix.ts`, `T/🗃️cells.ts` or engine file was edited. No request is needed: nothing of this lane is open.
