# 🔭️ Multiview Batch MO (2026-11-24)

17 objects of `🧰️matrix/batches/MO.tsv`. Tools, ops writers and ops: `🧰️matrix/MO/` (`make/<id>.py` writes `ops/<id>.json`, `apply.ts` applies it as the last step of the asset's chain; `app.sh` runs both twice and checks that the second run changes nothing). Measured with `T/🔎️scan.ts` and `T/📊️matrix.ts --fresh --jobs 1 --assets <id>`.

## Result

| | Assets |
|---|---|
| Clean in scan and matrix (no `fail` cell) | 15 |
| Open | 2: `used-concert-harp` (84 cells, contact `shoulder` in the plan), `used-harmonium` (21 cells, `paper-height` on a section view; asset not changed) |

Hard findings of the scan: 232 before, 10 after (8 harp, 2 harmonium).

## Per asset

Matrix cells of the asset after the repair; "drift" is the reported tier, not a failure.

| Asset | Hard findings before (scan) | After | Matrix after | What was changed |
|---|---|---|---|---|
| charcoal-grill | sync-forward ×3 (95 mm), anchor ×3 (95 mm) | 0 | 14 ok, 238 drift, 0 fail (283 s) | Plan, 4 bands: the three legs are inclined tubes (solids); a leg drawn as one stroke is swept over the 25 mm of the tube |
| stepladder | sync-bone-height ×24 (117 mm) | 0 | 70 ok, 182 drift, 0 fail (37 s) | Front and plan, 4 bands: the platform is its side outline swept over 0.29 m |
| used-basketball-hoop | sync-bone-forward ×5 (345 mm), sync-bone-height ×4 (179 mm), sync-forward ×2 (200 mm) | 0 | 84 ok, 168 drift, 0 fail (106 s) | Plan: every board part a prism. Front and plan: pole and arm tubes bodies of revolution, flange, end plate and strut prisms. Net: at 1:50 to 1:200 the one path of strokes becomes one ribbon per chain (69, 69, 12; as wide as the chain solid, 7 mm) in front and plan; at 1:500 the cone is a body of revolution |
| used-bicycle-shelter | anchor ×12 (143 mm) | 0 | 231 ok, 21 range-end, 0 fail (110 s) | 1:500 gets the side stand of the bike as a 75 mm band in the three views and as a body solid with thermal and structural reference; two decision texts follow |
| used-cello | sync-forward ×4 (42 mm) | 0 | 35 ok, 217 drift, 0 fail (184 s) | Plan, 4 bands: the neck end of the lid as a solid under the lid outline |
| used-classical-guitar | 83: sync-height ×20 (507 mm), sync-bone ×14 (520 mm), sync-forward ×4, anchor ×45 (520 mm) | 0 | 238 ok, 14 drift, 0 fail (555 s) | Front and plan, 4 bands: every panel of the gig bag and its lid is its plan outline swept over its thickness |
| used-concert-harp | sync-bone ×6 (250 mm), anchor ×16 (123 mm) | **anchor ×8 (123 mm)** | 168 drift, **84 fail** (254 s) | Front and plan, 4 bands: column a body of revolution; base, moulding, pedal blocks, neck and action plates prisms. The soundbox stays flat (see Open) |
| used-djembe-and-cajon | sync-forward ×3 (59 mm) | 0 | 189 ok, 42 drift, 21 unavailable, 0 fail (522 s) | Plan, 3 bands: the shell of the djembe as the body of revolution of the other views, under the head collar |
| used-double-bass | anchor ×16 (66 mm) | 0 | 252 ok (511 s) | 1:500: the merged stool base is a square plate with the glides at its corners (plan square, body box) instead of a disc; three decision texts follow |
| used-double-swing | sync-bone-forward ×4 (472 mm) | 0 | 252 ok (123 s) | 1:200 and 1:500, front and plan: the eight chain strokes are solids as wide as the chain solid (21 mm) |
| used-drum-kit | sync-bone ×5 (297 mm), anchor ×4 (297 mm) | 0 | 210 ok, 42 drift, 0 fail (547 s) | 1:500: bass drum, boom tubes and cymbals bodies of revolution; 1:200: boom tubes and joints |
| used-football-goal | anchor ×10 (880 mm), sync-bone ×2 (1,000 mm) | 0 | 28 ok, 224 drift, 0 fail (57 s) | 1:500 front: the portal becomes right post, left post and crossbar on their own bones, as solids; plan crossbar a solid. Net cage, back net and top net prisms in all bands |
| used-harmonium | paper-height ×2 (23 mm) | **paper-height ×2** | 210 ok, 21 unavailable, **21 fail** (72 s) | Nothing (see Open) |
| used-spring-rocker | sync-height ×6 (51 mm) | 0 | 210 ok, 21 unavailable, 0 fail (243 s) | Front and plan, 3 bands: every board and tube of the body, the spring plate and clamps are prisms; the coils of 1:200 bodies of revolution |
| used-viola | anchor ×2 (19 mm) | 0 | 119 ok, 112 drift, 21 unavailable, 0 fail (274 s) | Plan, 3 bands: ribs and back of the viola as solids under the top (layers z 0.8 and 0.9) |
| used-water-play-table | sync-forward ×4 (95 mm) | 0 | 252 ok (244 s) | Front and plan, 4 bands: the foot block of the screw mount is a prism; new in the plan, under the tube |
| used-wc-pan-cistern | sync-height ×9 (119 mm), sync-bone-height ×3 (239 mm) | 0 | 140 ok, 70 drift, 21 unavailable, 0 fail (133 s) | Front and plan, 3 bands: lid, seat ring and the two skirts of 1:50 are the plan outline swept over their thickness |

## Proof (each run to the end, after the last change)

- Scan `T/🔎️scan.ts`, 17 assets in one process: 15 clean, 2 with a hard finding, 10 hard findings.
- Matrix per asset: table above; every run exited 0, no asset crashed or timed out.
- `validateAsset`: 0 issues for all 17, before and after.
- Check CLI `--asset <id>`: `✓` for all 17.
- Real suites with the name filter over the 17 ids, one process per suite, verbose lines compared before and after (`suites.sh`, `results.py`): `🧊️catalog-silhouette` 17 passed / 17 passed; `🧊️three-views` 34 / 34; `🌡️catalog-physics` 34 / 34; `✅️asset-validation` 10 / 10; `🫁️breathing-catalog` and `⚡️asset-power` have no case for these assets (0 selected in both runs). No test line changed.
- Print: every authored scale and both range ends resolve as before under the default profile (`try.ts`); the only non-authored end is 1:1000 of `used-bicycle-shelter` (gutter, purlin; unchanged, 21 `range-end` cells).
- Idempotency: second run of writer and apply leaves all 16 documents unchanged and the ops equal as JSON. Verified again at the end that the 16 catalog documents still hold the repair and `used-harmonium` equals its backup.
- Sheets rendered for all 16 changed assets (one changed band, mostly in the turned pose). Opened and read: hoop 1:50 `in-service` and `maintenance`, guitar 1:50 `carried`, football goal 1:500 `maintenance`. The other 16 rendered sheets were not opened.

## Open

- `used-concert-harp`: `anchor-landmark` ×8 in the scan, 84 matrix cells, all contact `shoulder` in the plan in the played poses (117 to 123 mm, tolerance 39 mm). The plan shows the soundbox back from −0.752 to 0.072 m where the side elevation shows −0.947 to 0.242 m. No solid hint describes the tapering half cone; a solid soundboard alone was tried and withdrawn because it broke printing in the played pose (0.198 mm at 1:200). Request in `requests.md`.
- `used-harmonium`: the side elevation of 1:50 and 1:100 is a declared section; the cut removes the 1.15 m middle of the crest and leaves 1.127 m, which is drawn. The difference is exactly the tolerance and fails by rounding. Raising the crest was tried and withdrawn (silhouette 22.5 mm). Request in `requests.md`.
- `shapeDepth` ranks solids swept along v or u by a wrong depth (measured on the guitar bag). Rest drawings are unaffected; a paint order in a pose can be. In the guitar sheet `carried` the guitar is painted over the upright bag in the front; it was painted over the bag before the repair too. Request in `requests.md`.
- A solid is always a closed outline, so a part a coarse band draws as one stroke becomes a sliver. It must be as wide as its body solid, otherwise the silhouette measure fails (hoop 36.9 mm at 1:100 and swing 157 mm at 1:500 with 3 mm and 6 mm slivers; 7.5 mm and 13.0 mm with the true widths). On paper the two edges merge into one stroke that is heavier than the stroke of the side elevation.
- `used-double-swing`: the cradle chain stroke runs past the bone length, so its last 0.28 m keep the lateral position of the bone tip (5 mm off the old stroke end).
- Reported tier left as it is: 238 drift cells for `charcoal-grill` (bowl), 224 `used-football-goal` (posts and side frames of the finer bands are still flat), 217 `used-cello`, and the others in the table.
- No allowlist entry: the batch has no band-copy or in-band finding, before or after.

## Files

Updated (16 catalog documents, `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/<id>/🔣️.json`): charcoal-grill, stepladder, used-basketball-hoop, used-bicycle-shelter, used-cello, used-classical-guitar, used-concert-harp, used-djembe-and-cajon, used-double-bass, used-double-swing, used-drum-kit, used-football-goal, used-spring-rocker, used-viola, used-water-play-table, used-wc-pan-cistern.

Updated: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (three sections `## multiview MO — …`).

Created under the same ticket folder: `multiview-MO.md`; `🧰️matrix/MO/` with `env.sh`, `lib.py`, `apply.ts`, `files.ts`, `app.sh`, `try.ts`, `probe.ts`, `ext.ts`, `rings.ts`, `solids.ts`, `print.ts`, `depth.ts`, `validate.ts`, `suites.sh`, `results.py`, `confirm.sh`, `restdiff.sh`, `groups.py`, `codes.py`, `rig.py`, `boxes.py`, `twins.py`, `dump.py`, `view.py`, 16 `make/<id>.py` and 16 `ops/<id>.json` (`apply.ts`, `files.ts`, `try.ts`, `validate.ts`, `rings.ts`, `solids.ts`, `dump.py`, `view.py`, `lib.py` started as copies of MB-2).

Removed: `🗑️generated/matrix/MO` (backups, scans, logs, matrix folders, sheets).
