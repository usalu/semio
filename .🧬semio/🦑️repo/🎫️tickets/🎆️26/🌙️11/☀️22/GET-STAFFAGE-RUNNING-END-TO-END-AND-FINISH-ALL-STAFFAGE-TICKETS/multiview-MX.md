FINAL

# 🔭️ Multiview: Shared-Code Owner MX (2026-11-25)

Paths: `R` repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder, `K` the multiview ticket of 2026-11-02. Scripts: `U/🧰️matrix/MX/`. Every run went through `repo capacity run` with the pinned bun.

## 1. Result

- All 18 `## multiview …` requests of `U/requests.md` are answered in place (17 found at the start, one filed by MH-4 during the work), three of them twice after the coordinator's message of 11:30.
- Six defects of shared code repaired, each test first: the `paper-height` tie, `blank-shapes` on unfilled outlines and symbols, the in-band allowance per band, `shapeDepth` of swept solids, `PolygonUnionError` on prisms, and a new validator refusal for section labels that differ per language. One test made precise (the heater case of `🧊️model-bounds`).
- Four decisions for the dev: D50, D51, D52, D53 in `K/🗂️work-board.md` (section 4).
- One asset changed: `glulam-beam` (the sub-print steps of batch MC removed on the coordinator's word).
- **Still failing, not hidden:** `steel-column` 21 cells and `glulam-beam` 7 cells until the coordinator merges the two entries of `🧰️matrix/MX/allowlist.json` (the merge script refuses `steel-column` front so far). The people of MH-2, MH-4 and MH-6 named in section 5 keep their hard findings; nothing in this pass removes them.
- Not done: the pocket rule of `🧊️catalog-silhouette` (section 5, first item) and `lateralScale` of revolved drawing solids. No sheet was opened by eye in this pass.

## 2. Per request

| Request | Verdict | Measurement | Change | Proof |
|---|---|---|---|---|
| ML-1 `paper-height` tie; MO `paper-height` on a section | defect of the matrix | `used-harmonium` 1:50 `0.46000000000000085 > 0.45999999999999996`, 1:100 `0.23000000000000043 > 0.22999999999999998`; in micrometres drawn 1,127,000, nominal 1,150,000, tolerance 23,000 | D50: `micrometres`, `withinD18` in `T/🧰️.ts`, used by `geometryFindings` and `paperFindings` | suite 41 of 41; scan of `used-harmonium` clean; fresh matrix 231 ok, 21 unavailable, 0 fail (21 fail before). Section against the whole body: not changed |
| MH-2 `blank-shapes` | defect of the matrix | `kneeling-gardener` symbol `gardener-block` 1:200, three modes, three views: old rule `blank-shapes` in 9 of 9, new rule drawn in 9 of 9 | `blankFinding` (fill or outline run), `blankReason` reads the symbol's pens | suite 41 of 41. The matrix of `kneeling-gardener` was not run here (batch MH-2) |
| MC `steel-column`; MB-1 band copy and in-band | rule made precise | front against side 0.02522 (1:100), 0.00667 (1:200, 1:500), none at 1:50; `glulam-beam` plan 1:500 0.02875 | D51: `DerivedAllowance.bands`; two entries proposed | with the entries no finding in any band of both assets; without them matrix 21 and 7 fail cells, only `derived-view-in-band` |
| MO `shapeDepth` | defect of the engine | `used-classical-guitar` 1:50 stored plan: `lid-panel` 0.3528 to 0.1420, `pocket` 0.2680 to 0.1630, `back-panel` 0.2048 to 0.0140, `strap-l` −0.0500 to 0.0040 | `shapeDepthIn` places the profile in the plane of its sweep | `🧊️solid-sweep` 21 of 21, `🧊️depth-order` 5 of 5, `🧊️chain-state` 18 of 20 (the two reds the previous owner recorded). The red run of the new cases was not taken before the fix; `depth-old.ts` computes what the removed line gave (front 0.30 against 0.45 and 0.01) |
| MH-4 union along v; MH-6 union along w | defect of `unionPolygons` | `pregnant-person` 2 of 150 subjects, `tour-guide` 1 of 117 throw; sweep edges 0.5 to 23 nm off the vertical; the slab union closes with the exact area when turned | `SLAB_TURNS`, `turnedSlabUnion` in `S/🔨️modules/🔺️polygon/🟦️.ts` | `🔺️polygon-union` 23 of 27 before, 27 of 27 after (shapely and polygon-clipping) |
| ML-1 heater `🧊️model-bounds` | the case, not the asset | 1:50 open grip, 8 and 8 bands; 1:100 open, 4 and 6; 1:200 and 1:500 closed, 1 and 1; dial out of the 1:500 body | D52: `HEATER_RESOLVES` per band | filter `heater` 1 of 4 before, 4 of 4 after; bench cases 4 of 4 |
| MB-2 `storage-crate-stack` | asset, closed by MC; engine trap closed | scan clean | D53: validator `section-label-language` | `✂️section-views` 27 of 28 before the rule, 28 of 28 after; `✅️asset-validation` filter `section` 3 of 3 |
| MO half cone (`used-concert-harp`) | closed in the asset by ML-1 | scan clean, 14 reported group drift | none | none needed |
| MH-4 small-hole rule (new) | defect of the silhouette guard, not changed | MH-4's own table: 80.2 to 167.0 mm under the guard, 80 mm on a plain raster | none | open, section 5 |
| MH-6 plates; MH-4 opposite gates; MH-2 walkers | notes; no existing function is wrong | not measured again here | none | open, section 5 |
| MB-2 rewritten documents; MB-2 saguaro; MM queue | for the coordinator and MM | queue at 11:00: 20 grants, the core ceiling was the limit | none | none |

## 3. Coordinator's message of 11:30

1. `steel-column` and the slender bodies as one rule: D51 (above).
2. `glulam-beam` without sub-print steps: `S/README.md` asks no band to show what it cannot print (line 41 only says the scale selects line weights and level of detail). `🧰️matrix/MC/make/glulam-beam.py` now puts the merged bar and its thin-element text back; MC's `write.sh` applied 2 ops and the second run changed nothing; the document equals the committed one as JSON values. Proofs after: `validateAsset` 0 issues; prints at every key and range end from 1:20 to 1:1000; check CLI `✓`; `🧊️catalog-silhouette` 1 of 1, `🧊️three-views` 2 of 2, `🌡️catalog-physics` 2 of 2, `✅️asset-validation` 10 of 10 (breathing and power name no beam); scan 1 hard finding and matrix 245 ok, 7 fail, both only the plan at 1:500, cleared by the proposed entry.
3. `storage-crate-stack`: answered as solved by the asset; the engine refuses a differing label now (D53) instead of normalising it.

## 4. Decisions for the dev

- **D50** tolerance comparisons in whole micrometres. A sub-micrometre case can change sides in either direction; the tolerance itself is unchanged.
- **D51** an in-band allowlist entry may name its bands. Needs the coordinator's merge of two entries and the lifting of `REFUSED` for `steel-column` front.
- **D52** the heater case states per band what the body resolves. The alternative (1:50 detail in every band) needs asset work and an exemption in `volumes-feature`.
- **D53** a section label is the same in both languages.

## 5. Open, for the dev and the coordinator

- **Pocket rule of `🧊️catalog-silhouette`** (`fillSmallHoles` and the seal decide the drawn mask and the body mask differently). MH-4 located it on `person-with-white-cane` (56 cells stay). Not changed: it decides all 385 assets while twelve fixers measure against it; it needs the whole suite before and after with nobody writing assets. Proposal in the answer under the request.
- **A drawing solid with a free cross-section along the bone**, or the plans remade from the volume solids: `tennis-player`, `tour-guide`, `waving-person`, `worker-with-jackhammer`, `worker-carrying-box`, `traveller-with-rolling-suitcase` (MH-6); `person-with-arm-sling`, `postal-carrier`, `person-with-rollator`, `person-with-guide-dog`, `person-stretching` (MH-4); `nurse-walking`, `office-worker-carrying-laptop`, `patient-with-iv-pole` (MH-2). A schema decision; `lateralScale` of a revolved drawing solid is still not read by `solid3` and belongs to it.
- A declared section is compared with the dimensions of the whole body (no asset is flagged after D50).

## 6. To re-measure

- Coordinator: merge `🧰️matrix/MX/allowlist.json`, then `steel-column` and `glulam-beam` (matrix `--fresh`).
- MH-2: `kneeling-gardener` (matrix `--fresh`; 30 `blank-shapes` cells should go), and any asset with an unfilled symbol or view (`potted-monstera`, `hosta-clump`).
- MH-4: `pregnant-person` (the `unlevel` workaround is no longer needed); MH-6: `tour-guide` plan hand.
- Anyone: records older than 11:10 are older than the engine; the record digest covers the asset only, so `--fresh`.
- `used-harmonium`: done here, 0 fail.

## 7. Files (repo-relative)

Updated, shared code under `🎓️teaching/🏛️architecture/🧍️staffage/`: `🧪️tests/🔭️multiview-coverage/🧰️.ts`, `🟦️.ts`, `🐍️.py`, `🥒️.feature`; `🧫️fixtures/🔭️multiview-coverage/🔣️.json` (vectors `paper.tie`, `blank`, `allowlists.inBandBands`; the two allowlists untouched); `🔨️modules/🧮️evaluation/🟦️.ts`; `🧪️tests/🧊️solid-sweep/🟦️.ts`, `🥒️.feature`; `🧫️fixtures/🧊️solid-sweep/🔣️.json`; `🔨️modules/🔺️polygon/🟦️.ts`; `🧪️tests/🔺️polygon-union/🥒️.feature`; `🧫️fixtures/🔺️polygon-union/🔣️.json`; `🧪️tests/🧊️model-bounds/🟦️.ts`, `🥒️.feature`; `🔨️modules/🗂️catalog/🟦️.ts`; `🧪️tests/✂️section-views/🟦️.ts`, `🥒️.feature`; `🧫️fixtures/✅️asset-validation/🔣️.json`.

Updated, catalog: `🗂️catalog/🧱️components/glulam-beam/🔣️.json`.

Updated, tickets: `K/🗂️work-board.md` (D50 to D53); `U/requests.md` (21 answers); `U/🧰️matrix/MC/make/glulam-beam.py`, `U/🧰️matrix/MC/ops/glulam-beam.json`.

Created: `U/multiview-MX.md`; `U/🧰️matrix/MX/` (`patch.py`, `answer.py`, `suite.sh`, `after.sh`, `glulam.sh`, `probe-shared.ts`, `derived-with.ts`, `depth-old.ts`, `union-repro.ts`, `union-diagnose.ts`, `union-try.ts`, `union-vectors.py`, `allowlist.json`, `edits/*.json`, `answers/*.json`).

Removed: `U/🗑️generated/matrix/MX` and the `U/🗑️generated/matrix/MC` that MC's scripts wrote during the glulam proof (tool output).

Type check of the staffage package after the last source change: `tsc --noEmit` exit 0. Suite `🔭️multiview-coverage` after every shared change: 41 of 41 (375 s); it held 37 cases when the previous owner left and 38 at the start of this pass, three were added here.
