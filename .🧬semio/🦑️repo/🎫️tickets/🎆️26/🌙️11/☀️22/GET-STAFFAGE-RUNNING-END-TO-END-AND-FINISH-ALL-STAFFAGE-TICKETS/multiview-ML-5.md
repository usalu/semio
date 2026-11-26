FINAL

# 🧍️ Multiview Fixer ML-5: The Three People MH-4 Left Open (2026-11-26 00:02 to 01:15)

Paths: `R` repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder, `W` = `U/🧰️matrix/ML-5`, `H` = `U/🧰️matrix/MH-4` (its `fit.ts`, `lofta.ts`, `gate.ts` are the chain this round continues; the catalog files MH-4 wrote are the base). Every run went through `repo capacity run` (`W/q.sh`, never more than two requests); a result below is one read in a log that ran to its end.

## Result

3 of 3 clean with every proof; 0 open; 266 failing cells before, 0 after. No engine, test, fixture or manifest was edited. No request is filed: nothing was left that the asset could not cure.

| Asset | Gate (hard) | Scan (hard) | Silhouette | Prints | Check CLI | Matrix (252 cells) | Suites |
|---|---|---|---|---|---|---|---|
| `person-stretching` | 0, was 499 | clean, was 23 | OK | every scale, was 1 refusal (1:1000) | ✓ | 0 fail, was 168 (01:10) | 6/6 exit 0 (01:11) |
| `person-with-white-cane` | 0, was 114 | clean, was 4 | OK | every scale | ✓ | 0 fail, was 56 (00:24) | 6/6 exit 0 (00:27) |
| `person-with-guide-dog` | 0, was 339 | clean, was 8 | OK | every scale, was 2 refusals (1:150) | ✓ | 0 fail, was 42 (01:00) | 6/6 exit 0 (01:01) |

After the last run the three catalog files equal the trial copies that were measured (`H/same.py`): md5 `ddfab074…` (stretching), `1d0b394f…` (white cane), `9b3f0bc1…` (guide dog). Chain per asset, the last step of its chain: `H/lofta.ts` with `W/lofts/<id>.json`, then `H/fit.ts` with `W/after/<id>.json`, then `W/say.ts` with `W/say/<id>.json`; `W/land.sh` runs it twice (second run: file unchanged for all three) and refuses unless the result equals the measured copy.

## Measured at the start (catalog files as MH-4 left them, 00:04 to 00:06)

| Asset | Gate (hard) | Scan (hard) |
|---|---|---|
| `person-stretching` | 499 | 23 |
| `person-with-white-cane` | 114 | 4 |
| `person-with-guide-dog` | 339 | 8 |

The numbers equal the FINAL report of MH-4.

## `person-with-white-cane`: clean, all proofs (landed 00:16)

Cause (measured by MH-4, request "the small-hole rule turns a corrected drawing red"): the left thigh of the 1:500 front stood 84 to 93 mm above its place; with the thigh at its place the slit between the legs became a closed pocket of the drawn mask, because the 1:500 band drew the cane as a strip of 75 mm across the 74 mm between the boots, and the guard read 167.0 mm against 150. The repair was held back for the pocket rule of the guard. It does not need that rule: the strip was 75 mm wide only so that it prints at 1:1000 with its 0.13 mm outline (0.075 + 0.13 = 0.205 mm). With a strip of 40 mm (the width the band's own `thinElements` text states) and a 0.17 mm outline it prints (0.21 mm), the slit stays open as in the body, and the guard reads 47.7 mm in the front (was 80.2 with the misplaced thigh), 33.8 mm in the side elevation (was 55.2), 20.7 mm in the plan (was 25.5), under the rule that is live today.

Changed (`W/lofts`, `W/after`, `W/say`; chain `H/lofta.ts` → `H/fit.ts` → `W/say.ts`, second run: file unchanged): `thigh-l` and `thigh-r` at 1:500 as lofts of their solids (the two ops MH-4 named); `cane-shaft`, `cane-band-tip`, `cane-grip` at 1:500 in the three views 40 mm wide with `penMm` 0.17; the two decision texts of the band that state the strip and the pens (`silhouette`, `thinElements`, en and de; the stated pen of the fist follows its 0.17 mm too).

Measured variants (trial copies): thighs alone 167.0 mm (as MH-4 measured); strip 40 mm with the 0.13 mm outline: guard 47.7 mm, but 0.176 to 0.186 mm at 1:1000; strip 50 mm: guard 51.5 mm, 0.186 to 0.198 mm at 1:1000.

| Proof | Result |
|---|---|
| Gate (every subject, twins) | 0 hard, was 114 (00:17) |
| Scan | clean, was 4 hard |
| Silhouette guard | OK: 1:50 9.4 / 15.0 / 14.6 mm, 1:100 8.4 / 15.0 / 28.6, 1:200 20.0 / 28.3 / 28.7, 1:500 33.8 / 47.7 / 20.7 (elevation / front / plan) |
| `validateAsset`, three-views sweep | 0 issues, 0 findings |
| Prints (`MC/prints.ts`, every band at its key and both ends) | every authored scale prints |
| Check CLI | ✓ |
| Matrix | 252 cells: 175 ok, 77 group-drift, 0 unavailable, 0 fail, was 56 fail (00:24) |
| Suites | `🧊️catalog-silhouette` 1 passed, `🧊️three-views` 2, `🌡️catalog-physics` 3, `🫁️breathing-catalog` 3, `✅️asset-validation` 10, `⚡️asset-power` 24 skipped (no test of this asset), all exit 0 (00:27) |

Sheets opened: 1:500 before and after (the cane reads as before, a thinner stroke).

## `person-with-guide-dog`: clean, all proofs (landed 00:51)

Cause (the 339 hard gate findings, `sync-bone-forward` 77 mm and `sync-bone-height` 81 mm on `hand-l` and `hand-r` at 1:50): the side elevation drew each hand as ONE prism on the hand bone (palm, fingers and thumb merged), the front and the plan draw the palm on the hand bone and every phalanx as a prism on its own finger bone. MX-3's answer of 00:50 to MH-4's request says the same (D68). Repair at the cause, one body in three views:

1. The elevation takes the palm prism and the 15 phalanx prisms per hand that front and plan already draw (`copy` front → elevation, `W/after`); they are prisms swept along the bone, so the same solid stands in the three views. Gate 0 hard on this alone, but the guard read 17.7 mm in the elevation (limit 15; catalog plate 8.0).
2. The last 2.7 mm were three body solids that no shape draws and that the old plate covered: `hand-l-web-2` (on `middle-proximal-l`), `hand-l-grip-pad` (on `middle-distal-l`) and the thenar ellipsoids `hand-l-thenar`, `hand-r-thenar`. Found with `W/twin.ts` (every drawn shape beside the solid of its id; the solids without a shape). They are drawn as lofts of their solids in the three views (`W/lofts`): elevation 12.0 mm, front 15.0 and plan 13.5 unchanged. The palm as a loft changes nothing (measured: 17.7 mm in three variants), it stays the prism.
3. Print, band 200 at 1:150 (the fine end of its range; `MC/prints.ts` refused it on the catalog file MH-4 left, the check CLI does not ask the ends): `sleeve-l` showed a sliver of 0.190 mm with its outline in the side elevation of two poses, the two hind shanks of the dog 0.1995 mm in the front of `waiting-dog-sitting` (`W/prints-at.ts`). `sleeve-l` carries a 0.15 mm outline, the hind shanks 0.19 mm (was 0.13 and 0.18); the `styles` text of the band says so (en, de).

| Proof | Result |
|---|---|
| Gate (every subject, twins) | 0 hard, was 339 (00:52) |
| Scan | clean, was 8 hard |
| Silhouette guard | OK: 1:50 12.0 / 15.0 / 13.5 mm, 1:100 25.2 / 22.5 / 13.5, 1:200 33.4 / 47.2 / 45.4, 1:500 47.2 / 110.1 / 50.2 |
| `validateAsset`, three-views sweep | 0 issues, 0 findings |
| Prints | every authored scale prints (was 2 refusals at 1:150) |
| Check CLI | ✓ |
| Matrix | 252 cells: 252 group-drift, 0 unavailable, 0 fail, was 42 fail (01:00; 371 s, peak 325 MB) |
| Suites | `🧊️catalog-silhouette` 1 passed, `🧊️three-views` 2, `🌡️catalog-physics` 3, `🫁️breathing-catalog` 3, `✅️asset-validation` 10, `⚡️asset-power` 24 skipped, all exit 0 (01:01) |

Sheets opened: 1:50 before and after in the default pose (the hands of the side elevation show their phalanges as the front does; nothing else moves). Rendered and not opened: 1:50 `walking-with-guide-dog`, 1:200.

## `person-stretching`: clean, all proofs (landed 01:00)

Five causes, each measured on trial copies (`W/trials/person-stretching.v3` to `.v11`, ablation `.x*`) and repaired where it lies:

1. **Arms** (`sync-bone-*` on `arm-l`, `arm-r`, `forearm-r`, up to 183 mm at 1:50 and 1:100): sleeves, forearms and cuff ribs at 1:50, sleeves and forearms at 1:100 as lofts of their solids, the whole limb on both sides as the recipe asks. MH-4's trial stopped at the guard (16.0 mm plan at 1:50, 30.2 mm front at 1:100). Located by leaving one loft out per variant: both come with the right sleeve. At 1:100 the 30.2 mm are not the sleeve: the front already stood at 30.0 mm of 30 because the band names socks, its body has them, and the drawing left a slit between shoe and trouser leg (`H/solids-at.ts`: `sock-l` at the far place, no shape). `sock-l` and `sock-r` are drawn at 1:100 as lofts of their solids in the tone and order of the 1:50 socks: front 11.9 mm. At 1:50 the plan needs the right sleeve swollen by 2 mm (`swell`, 0.04 mm on paper): 13.3 mm. Stated plainly: what the 2 mm close was not located; it is not the pocket rule (16.0 mm under MX-2's prepared rule as well), and the plain raster shows no place above 9 mm that the variant without the sleeve does not show.
2. **Hem rib** (`sync-forward` 50 mm, `sync-lateral` 55 mm at 1:50): the body carries the hem rib on the pelvis, the drawing had it on the torso, so it swung out behind the body when the trunk folds. The loft of the pelvis solid replaces the torso shapes at 1:50 and 1:100.
3. **Training top** (`sync-lateral` 45 mm at 1:100, `sync-forward` 37 mm at 1:200): front and plan prisms of the skinned `hoodie` do not fold with the spine. The skinned loft of the solid in the three views at 1:50, 1:100 and 1:200 (the op `loft` of `fit.ts`, as MH-4 used it for `sil-body` at 1:500). With it the hood no longer hid behind the prism (32.3 mm elevation at 1:100): `hood` with its two rolls as lofts at 1:50 and 1:100 (elevation 12.6 mm at 1:50, 30.0 mm at 1:100, as before the batch).
4. **Head** (`sync-height` 38 to 42 mm, `sync-forward` 47 to 50 mm in the hamstring fold, all four bands): the prisms of the hair overshoot the ellipsoid when the head hangs. `hair` as the loft of its ellipsoid at 1:50 and 1:100, `head` and `hair` at 1:200, at 1:500 `sil-head` of front and plan as one loft fitted to head and hair and `head`, `hair` of the side elevation singly. The fringe (a front-only shape) ran through the whole depth of the head; it is as deep as the front half of the hair (v −0.07 to 0) at 1:50, 1:100 and 1:200.
5. **Right foot at 1:500** (`sync-bone-height` 140 mm, anchors `heel-r` 97 mm and `ball-r` 41 mm, and the print refusal at 1:1000, feature 0.199 mm): `sil-shoe-r` was flat in the three views; front and plan take the side outline swept over the width the shape had (`extrude`, the op MH-4's first fixer had listed; the loft was refused for the declared width, the prism keeps it: bbox 0.7147 m as before).

The `silhouette` text of 1:50 names what is drawn from sections and what stays a prism (en, de).

| Proof | Result |
|---|---|
| Gate (every subject, twins) | 0 hard, was 499 (01:01) |
| Scan | clean, was 23 hard |
| Silhouette guard | OK: 1:50 12.6 / 12.0 / 13.3 mm (IoU 0.986 / 0.979 / 0.983), 1:100 30.0 / 11.9 / 11.3, 1:200 20.1 / 33.6 / 17.0, 1:500 26.7 / 33.5 / 13.7 |
| `validateAsset`, three-views sweep | 0 issues, 0 findings |
| Prints | every authored scale prints (was: 1:1000 refused) |
| Check CLI | ✓ |
| Matrix | 252 cells: 252 group-drift, 0 unavailable, 0 fail, was 168 fail (01:10; 467 s, peak 389 MB) |
| Suites | `🧊️catalog-silhouette` 1 passed, `🧊️three-views` 2, `🌡️catalog-physics` 3, `🫁️breathing-catalog` 3, `✅️asset-validation` 10, `⚡️asset-power` 24 skipped, all exit 0 (01:11) |

Sheets opened: 1:50 default pose before and after (no visible change but the hem rib), 1:50 `hamstring-stretch` before and after (before: the hem rib stands out behind the back as a slab and the plan is a set of prism ends; after: the trunk folds as one body in the three views), 1:100 and 1:200 after. Rendered and not opened: 1:500 before and after, 1:200 before, 1:50 `side-bend` after.

Tight, stated plainly: the 1:100 side elevation reads 30.0 mm of 30 (as on the file MH-4 left; not moved by this round); the 1:50 front of the guide dog and of the white cane read 15.0 mm of 15 (unchanged).

## Requests

None filed. Appended to `U/requests.md` as a note for MX-3 and the coordinator: the two open requests of MH-4 ("hands whose fingers hang on bones of their own", "the small-hole rule turns a corrected drawing red") need nothing more for these three assets; the pocket rule of MX-2 is no longer awaited by `person-with-white-cane`.

## Queue

Grants of 800 to 1200 MB for trials, landings and proofs (the matrix inside the 1000 MB landing grant), 2500 MB for the six suites of one asset; never more than two requests, no wait above a few seconds (10 running, 0 waiting at 00:02). Peaks (`/usr/bin/time -l`): gate 139 MB, scan 220 MB, `measure.ts` 484 to 678 MB, check CLI 255 MB, matrix 325 to 389 MB (312 to 467 s), vitest worker 1454 MB. `H/where.ts` did not end in 14 minutes on the 1:50 guide dog (own run stopped by its PIDs); `W/far.ts` is the same measure with a scanline raster (seconds).

## Files

Updated (catalog, repo-relative): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-stretching/🔣️.json`, `…/person-with-white-cane/🔣️.json`, `…/person-with-guide-dog/🔣️.json`.

Created in `U/🧰️matrix/`: `batches/ML-5.tsv`; `ML-5/env.sh`, `q.sh` (one script under one grant), `try.sh`, `trials.sh` (a trial on a copy: chain, prints, guard, gate, scan), `land.sh`, `prove.sh`, `suites.sh`, `measure.sh`, `where.sh`, `probes.sh`, `prints.sh`, `sheet.sh`; tools `say.ts` (decision texts follow a repair), `far.ts` (where a view leaves the guard), `twin.ts` (shapes beside their solids, undrawn solids), `prints-at.ts` (print violations of a band at a denominator of its range, per pose and view); the ops in the catalog `lofts/<id>.json`, `after/<id>.json`, `say/<id>.json` for the three assets; `trials/` (every variant that was measured); the job scripts `c2.sh`, `d2.sh` to `d8.sh`, `p1.sh`, `s4.sh` to `s9.sh` (what ran under each grant).

Updated: `U/requests.md` (one note appended), this report. Removed: `U/🗑️generated/matrix/ML-5` (trial copies, logs, sheets).
