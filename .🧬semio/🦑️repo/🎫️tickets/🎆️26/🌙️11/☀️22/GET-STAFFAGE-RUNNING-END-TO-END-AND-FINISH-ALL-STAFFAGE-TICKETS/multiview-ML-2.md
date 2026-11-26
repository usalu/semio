FINAL

# 🔭️ Multiview Round ML-2: Open Non-Plant Leftovers (2026-11-25)

Paths: `R` repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder. Batch: `U/🧰️matrix/batches/ML-2.tsv` (26 assets). Scripts and proof records: `U/🧰️matrix/ML-2/`. Every run went through `repo capacity run`.

## Result

- **26 of 26 clean with all proofs; 0 open.** No catalog document was changed by this round: measured now, none of the 26 has a hard finding or a `fail` cell, so there was nothing to repair in an asset.
- Why they are clean now although the audit lists them as open: (1) every allowlist entry the earlier batches proposed for these assets was merged into the fixture by the engine owners (D49, D51, D58), each with a physical reason; (2) the `paper-height` tie of `used-harmonium` is decided in micrometres (D50); (3) the heater case of `🧊️model-bounds` states per band what the body resolves (D52); (4) the 1:20 print gap of `used-hinged-wardrobe` was a floating-point tie in the print measure, found here, filed, and repaired by MX-2 at 19:50.
- No new allowlist entry is proposed (`🧰️matrix/ML-2/allowlist.json` holds two empty lists).
- The catalog documents move under the batch: most of the 26 were rewritten by other lanes after the audit (the hashes differ from its table; writes seen up to 21:04, among them nine documents at 20:35 and `used-wall-hung-gas-boiler` at 21:04). Every proof below ran after the last of these writes; the hashes in the table are those of the proven files, compared again after the runs (all 26 equal).

## Proof (each run to its end; records in `🧰️matrix/ML-2/proof-*.txt`)

- **Scan** `T/🔎️scan.ts --language en` over the 26, 20:25 and again 21:29 at the head of the matrix chain: `[scan] 26 assets: 26 clean, 0 with a hard finding, 0 hard findings` (`proof-scan.txt`).
- **Matrix** `T/📊️matrix.ts --language en --jobs 1 --fresh --assets <id>`, one asset after the other, 21:29 to 21:49 (`measure.sh`; `proof-matrix.txt`): 26 runs, each `exit 0`, 6,531 cells, **0 fail**.
- **Rest of the proof list** (`rest.sh`, 21:29 to 21:48; `proof-rest.txt`), per asset: fixer's gate over poses, clip instants and transitions (`MR/gate.ts --twins`) `hard findings: 0` ×26; silhouette guard `OK` ×26; `validateAsset` `0 issues` ×26; three-views sweep `0 findings` ×26; `every authored scale prints` ×26 (keys and both range ends); check CLI `✓` ×26.
- **Suites with the name filter over the 26**: `🧊️catalog-silhouette` 26 passed; `🧊️three-views` 56 passed; `🌡️catalog-physics` 52 passed; `✅️asset-validation` 10 passed; `⚡️asset-power` 2 passed; `🫁️breathing-catalog` names none of the 26 (290 skipped); `🧊️model-bounds` filter `heater` 8 passed. No failing line in any log.

| Asset | sha256 (12) of the proven file | Matrix cells | Note |
|---|---|---|---|
| `etics-brick-wall-12` | `0b56beac8707` | 252: ok: 252; 0 fail | front band copies (six pairs) and plan in-band at 1:500 by merged entries: rendered face of a 1 m sample; one block at 1:500 |
| `etics-brick-wall-30` | `da0a91545e98` | 252: ok: 252; 0 fail | as above |
| `solid-brick-wall-1960` | `e98dec330257` | 252: ok: 252; 0 fail | as above, and plan 200/500 |
| `spiral-stair` | `d0fb7b8d0250` | 252: group-drift: 252; 0 fail | front and plan 100/200 by merged entries; every cell reported `group-drift` (bone `column`: elevation and front draw `stair-mass`, the plan `column`; 1047 mm, different parts, not a failure) |
| `stair-flight` | `28cfba82a92a` | 252: ok: 252; 0 fail | front 100/200 by a merged entry (0.00376; the bands differ by stringer flanges not seen from ahead) |
| `bicycle-stand` | `ca6da3cec2d7` | 252: ok: 252; 0 fail | front and plan 50/100/200 by merged entries (one bent tube) |
| `heat-pump-outdoor-unit` | `d392cea879c2` | 252: ok: 252; 0 fail | plan 200/500 by a merged entry (hinge knuckles of 9 mm) |
| `plate-heat-exchanger` | `c34554c7afa2` | 252: unavailable: 21, ok: 231; 0 fail | plan 100/200 by a merged entry |
| `solar-thermal-collector` | `db26d9b11427` | 252: group-drift: 168, ok: 84; 0 fail | front and plan 50/100 by merged entries; reported drift of `stand` and `headers` |
| `used-coffee-table` | `cc90174e3176` | 252: ok: 252; 0 fail | plan 200/500 by a merged entry |
| `used-folding-bookcase-ladder` | `425d3daa2189` | 252: ok: 224, group-drift: 28; 0 fail | front 200/500 by a merged entry |
| `used-shoe-cabinet` | `1d03d8b7d5ad` | 252: ok: 147, group-drift: 105; 0 fail | plan 200/500 by a merged entry; reported drift of the flaps |
| `used-tv-sideboard` | `0a64b3ccfd09` | 252: group-drift: 210, ok: 42; 0 fail | plan 200/500 by a merged entry; reported drift of `drawer` |
| `used-wall-unit` | `d9d98d181720` | 252: group-drift: 182, ok: 70; 0 fail | plan 100/200/500 by a merged entry; reported drift of drawers and doors |
| `tealight` | `927c85f8848d` | 252: unavailable: 21, ok: 189, group-drift: 42; 0 fail | plan 50/100/200 and front in-band by merged entries (body of revolution) |
| `used-bollard-light` | `252e3d677d96` | 252: ok: 210, group-drift: 42; 0 fail | front in-band at 200 and 500 by a merged entry (D58) |
| `pendant-lamp` | `09ebd0cab7dd` | 252: unavailable: 21, ok: 231; 0 fail | front in-band by a merged entry (bodies of revolution about the cord) |
| `ceiling-diffuser` | `3f0ca95d17a1` | 252: unavailable: 21, ok: 231; 0 fail | front in-band by a merged entry (four-fold symmetry) |
| `used-park-lantern` | `0cf5e48273fa` | 252: ok: 252; 0 fail | front in-band at 500 by a merged entry (D58) |
| `used-playground-roundabout` | `ab643beebfde` | 252: ok: 252; 0 fail | front in-band at 200 and 500 by a merged entry (D58) |
| `waste-bin` | `b6abaea081ea` | 252: ok: 252; 0 fail | front in-band at 500 by a merged entry (D58) |
| `electric-fan-heater` | `cbe66312d30a` | 231: unavailable: 21, ok: 210; 0 fail | `🧊️model-bounds` filter `heater`: 8 passed (D52, test made precise by MX) |
| `used-hinged-wardrobe` | `c25ecaab3de6` | 252: ok: 252; 0 fail | 1:20 prints since the engine repair of 19:50 (below) |
| `used-harmonium` | `808ac0c9bebf` | 252: unavailable: 21, ok: 231; 0 fail | the 21 `paper-height` cells of ML-1 are gone (D50); the MX claim now has a log |
| `used-towel-radiator` | `9f730d163dbc` | 252: unavailable: 21, ok: 231; 0 fail | file of 19:09 (after the MC record) proven |
| `used-wall-hung-gas-boiler` | `5ce3d2b79f56` | 252: unavailable: 21, ok: 231; 0 fail | file of 21:04 (after the MC record) proven |

## Allowlist entries: accepted as physical, none hides a defect found here

All entries concerned were merged before this round. Read again against the documents and the measurements of their batches: each names a view that is the same drawing by the body itself (a body of revolution or of four-fold symmetry; a featureless rendered face; one bent tube; a plan in which the parts that differ lie under a top or inside a casing), and D58 limits the in-band entries to the bands where the measure needs them. Two things stay as the earlier reports state them and are not failures: seven views of MB-1 and MB-2 assets also carry covered shapes in the finer band, and the reported `group-drift` tier remains in `spiral-stair`, `solar-thermal-collector`, `used-folding-bookcase-ladder`, `used-shoe-cabinet`, `used-tv-sideboard`, `used-wall-unit`, `tealight`, `used-bollard-light` (worst: `spiral-stair` bone `column`, 1047 mm between the elevation shape `stair-mass` and the plan shape `column`, two different parts on one bone).

## used-hinged-wardrobe: the 1:20 gap

- 19:40, document `4114f0f95959`: `1:20 (band 50) unsuitable … "check":"gap","shapes":["mirror","handle-2-bar"],"measuredMm":1.7763568394002505e-15,"requiredMm":0.15`.
- Cause: mirror face and handle back are 10 mm apart, 0.5 mm at 1:20, both pens 0.5 mm: the ink gap is exactly zero, a join by the rule's own text; `gapsBetween` decided `ink <= 0` in floating point. Drawing and volume agree, so the asset was not nudged out of the tie.
- Request `## multiview ML-2 — print suitability: an ink gap of exactly zero …` in `requests.md`; answered by MX-2 at 19:55: `if (ink <= TOUCH_MM) continue;`, two new cases in `🧪️tests/📏️print-suitability` (red before, 9 of 9 after).
- 21:41, document `c25ecaab3de6`: `every authored scale prints`; matrix 252 ok.

## Sheets

No view was changed by this round, so there is no before and after. 16 product sheets (side, front, plan; line and solid) were rendered from the catalog documents at 22:00 with `sheets.sh` for the assets other lanes wrote after their last proof, and four were opened: `used-towel-radiator` 1:50 `lifted` (radiator upright beside the A-frame, valve and brackets in place in all three views), `used-wall-hung-gas-boiler` 1:50 `lifted` (boiler tilted above the pallet in the side view, upright outline in the front, straps laid out in the plan), `used-hinged-wardrobe` 1:50 `doors-open` (open doors edge-on in the front, swung out in the plan, shelves visible in the side; mirror and handle read apart), `used-harmonium` 1:50 `played` (section A-A with the cut-line marker in the plan). Nothing wrong seen. One remark, not a finding: the line front of the wardrobe with open doors does not show the handle that the solid front shows. Rendered but not opened: the other twelve (`used-towel-radiator` 1:50 `stored`, 1:200 `stored`; boiler 1:50 `stored`, `straps-down`, 1:200 `lifted`; wardrobe 1:50 `stored`, `carried`; harmonium 1:50 `stored`; `electric-fan-heater` 1:50 `off`; `used-tv-sideboard` 1:100 `stored`; `used-wall-unit` 1:100 `carried`; `spiral-stair` 1:100 `self-weight`). The sheets of the 21 allowlisted assets were opened by their batches (MB-1, MB-2, MS) when the entries were decided; not opened again here.

## Incidents

- The queue of this session was full from 19:14 to 21:29 (session share, then a restart of the repo runtime at 21:02 that cancelled both waiting requests; resubmitted). One request was lost to a path error of my own script (`${0:A}` read after `cd`; fixed in `measure.sh`, `rest.sh`, `prove.sh`). The matrix request was resubmitted at 800 MB on the coordinator's word (figures of MM).
- Processes stopped: only my own queue requests, by PID (a duplicate of `measure.sh`, the first heater request, the 1200 MB request).

## Files

- Catalog: none changed.
- Created: `multiview-ML-2.md`; `🧰️matrix/batches/ML-2.tsv`; `🧰️matrix/ML-2/` with `measure.sh` (scan and matrices in one grant), `rest.sh` (gate, guard, prints, check CLI, suites), `prove.sh` (copy of the MX-2 proof for single assets), `chain.sh`, `q.sh`, `sheets.sh`, `allowlist.json` (empty), `proof-scan.txt`, `proof-matrix.txt`, `proof-rest.txt`.
- Updated: `requests.md` (one section appended, answered by MX-2).
- Removed: `🗑️generated/matrix/ML-2` (tool output).

## Open

Nothing in the 26. For the coordinator: the documents are still being rewritten by other lanes, so the final full matrix on the frozen catalog decides; the hashes above tell which state was proven.
