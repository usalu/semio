IN WORK (not final). Fourth fixer of the batch (takeover 2026-11-25 19:00). Clean with all proofs: 10 of 13; written with the proof running: 1 (`horse`); open: 2 (`dairy-cow`, `pigeon`).

# 🐾️ Multiview Fixer MA: the 13 Animals

Batch `🧰️matrix/batches/MA.tsv`. Paths as in `📋️multiview-brief.md` (`R`, `S`, `T`, `U`); scripts in `U/🧰️matrix/MA/` (`W`), base tools of `U/🧰️matrix/MR/` and `U/🧰️matrix/MX-2/loft.ts`, `U/🧰️matrix/MC/prints.ts` (read only). Every number below is read from a log that ran to its end in `U/🗑️generated/matrix/MA/proof/` on 2026-11-25 and 26; that folder is deleted with this report, the scripts reproduce it.

## 1. Result

| Asset | State | Scan hard before → after | Gate hard (poses, clip instants, transitions) | Guard, validate, sweep | Check CLI | Prints | Matrix of the asset |
|---|---|---|---|---|---|---|---|
| `domestic-cat` | CLEAN, written 19:35 | 323 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 231 cells: 210 ok, 21 unavailable, 0 fail |
| `mallard-duck` | CLEAN, written 20:40 | 78 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 231 cells: 210 ok, 21 unavailable, 0 fail |
| `herring-gull` | CLEAN, written 21:39 | 204 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 231 cells: 182 ok, 21 unavailable, 28 group-drift, 0 fail |
| `labrador-dog` | CLEAN, written 23:40 | 569 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 252 cells: 252 ok, 0 fail |
| `hedgehog` | CLEAN, written 01:05 | 47 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 210 cells: 56 ok, 42 unavailable, 112 group-drift, 0 fail |
| `red-fox` | CLEAN, written 01:00 | 798 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 231 cells: 210 ok, 21 unavailable, 0 fail |
| `goat` | CLEAN, written 01:06 | 115 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 252 cells: 252 ok, 0 fail |
| `roe-deer` | CLEAN, written 01:49 (version 2; version 1 of 22:03 had 21 range-end cells) | 1012 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 252 cells: 252 ok, 0 fail (ended 02:28) |
| `pig` | CLEAN, written 02:30 (version 2; version 1 of 01:22 drew the hams over the face in the front) | 342 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 252 cells: 252 ok, 0 fail (ended 02:51) |
| `sheep` | CLEAN, written 02:07 (replaces the hybrid chain of 12:30) | 344 → 0 | 0 | OK, 0, 0 | ✓ | every authored scale | 252 cells: 252 ok, 0 fail |
| `horse` | trial copy clean (gate 0, guard OK, prints), write queued [horse] | 139 → not run yet [horse] | trial copy 0 [horse] | trial copy OK, 0, 0 [horse] | not run yet [horse] | trial copy: every authored scale [horse] | not run yet [horse] |
| `dairy-cow` | OPEN, catalog file unchanged | 342 (unchanged) | trial copy 752, see section 4 | | | | not run (file unchanged) |
| `pigeon` | OPEN, catalog file unchanged | 164 (unchanged) | trial copy 44, see section 4 | | | | not run (file unchanged) |

"Guard, validate, sweep" is `MR/measure.ts` of the catalog file against the backup: the silhouette guard of `🧊️catalog-silhouette` per scale and view, the issue count of `validateAsset`, the findings of the three-views sweep; "OK, 0, 0" was also the state before, except `horse`, whose backup failed the guard in one check (1:50 plan 18.5 mm of 15, the `lateralScale` of `tail-1` that MX-2 reported). `group-drift` is the reported tier of D41, not a failure. Vitest suites with the name filter (`stage.sh vt-*`, one grant of 2500 MB): `🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `✅️asset-validation` passed for `domestic-cat`, `mallard-duck`, `herring-gull`, `labrador-dog`, `hedgehog`, `red-fox`, `goat`, `sheep`, `roe-deer`, `pig` [suites]; `🫁️breathing-catalog` and `⚡️asset-power` have no case for an animal of this batch (all skipped by the filter, as before).

## 2. Method

The chain of an asset after its generators is `lfinish.sh <id>`: `loft.ts <id> --ops loft/<id>.json --write` twice (the second run changes nothing), then, where `loft/<id>.lofts.json` exists, `../MX-2/loft.ts <id> --ops loft/<id>.lofts.json --write` twice; the written file must equal the trial copy that passed the gate. The earlier chains of this batch (`fit.ts` with `ops/`, `nest.ts` with `nest/`: single solids and nested prisms, in the catalog for `pig`, `sheep`, `hedgehog` from 12:07 to 12:48) are replaced: over clips and transitions they left 4 to 1832 hard findings per asset.

What the repair does (D54, README "Loft shapes"):

- A part that a pose turns is ONE body in the three views: a `loft` shape. Limbs, neck, head, tail: the front and the plan take a loft with the v range of the drawn side elevation and the lateral range of the body solids of the bone at every station (`loft` op), so the side elevation keeps its drawn outline. A part that leaves the sagittal plane (wing, ear, antler) is the loft of its solid in the side elevation too (`"side": true`).
- A trunk is the body of its volume (`"width": "body"`, or the op `body` for a skinned or many-part trunk): a trunk left flat fails `sync-bone-*` (`labrador-dog` 156 mm in `sit-and-pant`, `red-fox` 151 mm curled), a trunk of fitted superellipses misses the silhouette guard (`red-fox` front 20.6 mm at 1:50). `sheep` is the exception: its trunk is a polyhedron of wool, stays as drawn and the gate has no finding for it.
- When the trunk is its own body, the parts the flat trunk outline drew with it need their own shape: hams, shoulder blades, upper arms, claws (`lbody.py` writes the entries for `../MX-2/loft.ts`, `llimb.py` the `body` ops with a paint order per view). Without them the guard fails (`red-fox` plan 17.7 mm, `pig` plan 43.8 mm at 1:50).
- A stroke leg of 1:200 (an open line that stood still in the front and the plan) is the loft of its solid.
- `volumes-feature` compares the thinnest solid of a bone with its thinnest drawing: `slim` (set by `thin.py`) keeps a loft no thicker than that solid. The print profile at the ends of a band: `swell` on a loft.
- Anchors (D45): `labrador-dog` `contact-rump`, a lumped contact on the centre line 33 mm off every drawn shape in the front at 1:50, is split into `contact-rump-n` and `contact-rump-f` at z = ±0.035 m (op `split-anchor`; the pose, the clip interval and the event that named it follow). No other anchor of the eleven written assets is reported by the gate after the drawing repair.

Tools of this takeover in `W`: `loft.ts` (third fixer; added: `slim`, `swell`, `"side": false`, the fallback of `"width": "body"`, the ops `body` and `split-anchor`), `lcycle.sh` (dry run: build, gate, measure, prints), `lfinish.sh` (write and prove), `lsum.sh`, `left.py`, `thin.py`, `edit.py`, `lsettle.py` with `lsettle.sh` (keeps the loft ops the guard accepts), `ltrunk.py`, `lbody.py`, `llimb.py`, `ltrunkbody.py`, `rebase.py`, `crop.sh`, `worker.sh` with `job.sh` (one lane per grant of the memory queue). The settings of `lbody.py` per asset are in `loft/<id>.env`.

## 3. Per asset

- `domestic-cat`: 73 loft ops. Sheets of 1:50, 1:100, 1:200 in `standing-alert`, `sitting-tail-wrapped`, `stretch-deep` opened before and after: side unchanged, front and plan the same outlines with smooth sections (the sitting front lost its lumps).
- `mallard-duck`: 67 loft ops; 10 parts the proposal left flat taken up (`left.py`: feet, scapulars), `slim` 9.9 mm on the shanks of 1:100. Sheets of 1:50 and 1:200 in `standing` and `upending`: the plan wings are smooth bodies instead of stacked rectangles.
- `herring-gull`: 121 loft ops; 29 parts taken up (coverts, secondaries, primaries, feet: the body of the solid that carries the id of the shape), the stroke legs of 1:200 as lofts. Sheets of 1:50 in `standing` and `gliding`: standing unchanged, the gliding plan shows the inner wing as bodies where it was two thin lines.
- `labrador-dog`: 103 loft ops, trunk (`croup`, `hindbody`, `chest`) as volume bodies, the split contact. Sheets rendered, not opened one by one; matrix 252 of 252 ok.
- `hedgehog`: 46 loft ops. Belly and chest are their volume lofts; the spiny coat (`dome`, `dome-front`, four `skirt-*`) is the volume loft below the spines swollen by 6 mm with `"side": false`: the side elevation keeps its drawn serrated outline, front and plan show the coat as a smooth body where the flat outlines were serrated (lofts that take the spine tips into their width miss the guard: plan IoU 0.9085 of 0.92). Sheets of `foraging` and `rolled` at 1:50 opened.
- `red-fox`: `loft/red-fox.json` (64 ops) and 22 body entries (thighs, shoulder blades, upper arms, jaw, far ear). Sheets of 1:50 in `standing-alert` and `curled-sleeping` opened: standing as before with haunch and shoulder as bodies, the curled plan is one coherent body.
- `goat`: croup and chest as volume bodies; 32 body entries: the belly as its two solids, thigh, shoulder blades, upper arms, pelvis of 1:200 and 1:500, the udder as one fitted loft (two halves at 1:50: four separate lofts leave a gap of 0.148 mm on paper at 1:75, one loft misses the front guard by 24.4 mm). Sheet of 1:50 `standing-alert` opened: as before.
- `roe-deer`: 105 loft ops; antlers of 1:50 and 1:500 taken up, `slim` 20.9 mm on the ears of 1:500. Version 1 printed every scale but 1:1000 (the end of band 500: `foot-fn` 0.1997 mm of 0.2 mm, 21 range-end cells); version 2 adds `swell` 0.5 mm on the four feet of 1:500 and prints every scale. Sheets of 1:50 and 1:500 `alert` opened: as before.
- `pig`: hindbody and chest as volume bodies, ears as body entries (the plan of a flat ear stood still when the head pitched: `sync-forward` 44 mm at 1:200), hams, shoulder blades and upper arms as `body` ops of `llimb.py`. Version 1 took their paint order from the side elevation and drew the hams over the face in the front (seen on the sheet, not by any check); version 2 orders them per view, sheet of 1:50 `standing-pig` opened: the face is back.
- `sheep`: 72 loft ops, trunk as drawn (wool polyhedron); at 1:50 the pasterns and the claws (drawn by one flat shape on the pastern bone) are the lofts of their solids, the claws 0.45 mm (fore) and 0.5 mm (hind) slimmer so that the cleft prints as a gap at 1:20 (0.15 mm) and the guard keeps the cleft (closing it costs 15.1 mm of 15 in the front). Sheets of 1:50 in `standing-resting` and `lying-ruminating` opened: the woolly outlines are kept.
- `horse`: the four tail bones as the lofts of their solids in the three views (18 body entries; the revolved tail with `lateralScale` failed the guard before and `sync-bone-lateral` up to 97 mm in the swish), neck and hooves of 1:50 left as drawn (`lsettle.py`: their lofts miss the guard by 17.3 and 18.8 mm). [horse-note]

## 4. Open

- `dairy-cow` (catalog file unchanged, scan 342 hard as on 2026-11-24). The trunk of its volume is skinned over `lumbar` and `thorax` (`barrel`, `back` at 1:50 and 1:100, `trunk` at 1:200 and 1:500). Trial of 02:06 (`ltrunkbody.py`: the trunk as `body` ops with the skin weights, jaw as body entry): every `sync-*` finding is gone; left are 752 `anchor-landmark` findings of two anchors bound to `thorax` while the skinned trunk is drawn on `lumbar` (`back-plume` 125 mm off in the front in `lie-down`, `contact-sternum` 103 mm in the side elevation and 79 mm in the plan in `graze-and-chew`), the guard fails in the front (62.6 mm at 1:50, 53.7 mm at 1:100: the hook and pin bones, ellipsoids with a transform, are not drawn yet) and the plan (20.3 mm at 1:50), and `region-undrawn` ×2 (`flank-rumen`: `ltrunkbody.py` replaced `rumen-flank`). Next steps, all in the asset: body entries for `far-hook`, `far-pin`, `near-hook`, `near-pin`; keep `rumen-flank`; bind `back-plume` and `contact-sternum` to `lumbar` with their coordinates in that frame (D45). Not done in this session; no request, nothing in shared code stands in the way.
- `pigeon` (catalog file unchanged, scan 164 hard). At 1:50 the volume carries the flight feathers on forty feather bones (`p1-*` to `p10-*`, `s1-*` to `s10-*`) that no view draws; the drawn `*-primaries` and `*-coverts` stood for them. Trial of 22:26 (wing parts as the lofts of `far-hand`, `far-coverts`, `far-shoulder`, the flat extra pieces of the front removed, legs of 1:200 as lofts): 44 hard findings left (1:50 `sync-forward` 92 mm, `sync-height` 95 mm in the take-off), guard plan IoU 0.9076 of 0.92 at 1:50, sweep 10 findings. Next step: body entries for the feather bones (`lbody.py` without `BONES`) and the print profile for forty lofts per wing. Not done in this session; no request.

## 5. Notes

- 2026-11-25 19:09: another worker rewrote `asset.tags` of every catalog document (strings became `{ id, title }`). `rebase.py` spliced that member byte for byte into the backup and the trial copies before every write, so no write of this batch undid it.
- The matrix of an animal takes 25 to 80 minutes on the loaded host (the 1:50 entry alone up to 20 minutes), peak below 350 MB; two lanes ran, one of 800 MB and one of 2500 MB for the suites.
- 21:08: the queue cancelled both lanes of the batch; the matrix of `mallard-duck` had written its last line (0 fail), its summary was made from the output folder afterwards.
- `where.ts` (third fixer) prints the whole body as differing in both directions; it was not used for any decision.

## 6. Files

Updated: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/<id>/🔣️.json` for `domestic-cat`, `mallard-duck`, `herring-gull`, `labrador-dog`, `hedgehog`, `red-fox`, `goat`, `roe-deer`, `pig`, `sheep`, `horse`. Unchanged: `dairy-cow`, `pigeon`.

Created or updated in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️matrix/MA/`: `loft.ts`, `loft/<id>.json` (13), `loft/<id>.all*.json` (full proposals), `loft/<id>.lofts.json` and `loft/<id>.env` (`red-fox`, `goat`, `pig`, `sheep`, `horse`, `dairy-cow`), `lcycle.sh`, `lfinish.sh`, `lsum.sh`, `left.py`, `thin.py`, `edit.py`, `lsettle.py`, `lsettle.sh`, `ltrunk.py`, `lbody.py`, `llimb.py`, `ltrunkbody.py`, `rebase.py`, `crop.sh`, `worker.sh`, `job.sh`, `where.ts`, `where.sh`, `shift.ts`, `shift.sh`; kept from the earlier fixers: `fit.ts`, `nest.ts`, `settle.ts`, `nsettle.ts`, `ops/`, `nest/`, `turns/`, `protect/`, the stage scripts. Removed: `🗑️generated/matrix/MA` (with this report). Report: `multiview-MA.md`.

## 7. Requests

None filed: no finding of this batch needed a change of shared code, tests or fixtures.
