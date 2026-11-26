FINAL

# 🧍️ Multiview Fixer MH-4: Twelve People (final report, 2026-11-26 00:05)

Paths: `R` repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder, `W` = `U/🧰️matrix/MH-4`. Four fixers worked on the batch (2026-11-24 13:39 to 23:25; 2026-11-25 08:50 to 13:10; 16:10 to 18:41; 19:00 to 2026-11-26 00:05). The report of the first is kept as `W/notes/report-of-2026-11-24-2325.md` (ops and recipe of `fit.ts`, still valid). Every run went through `repo capacity run`; a result below is one I read in a log that ran to its end, and "earlier" marks a proof of the second takeover that was not repeated (rule of the second takeover brief: prove once, when the file changes).

## Result

9 of 12 clean with every proof, 3 open with 266 failing cells together (was 6 clean, 6 open with 665 failing cells at 19:00).

| Asset | State | Gate (hard) | Scan (hard) | Silhouette | Check CLI | Matrix (252 cells) | Suites |
|---|---|---|---|---|---|---|---|
| `person-sweeping` | clean, earlier | 0 (13:07) | clean | OK | ✓ | 0 fail (13:23) | 6/6 exit 0 (13:41) |
| `person-taking-selfie` | clean, earlier | 0 (13:26) | clean | OK | ✓ | 0 fail (13:44) | 6/6 (13:42) |
| `person-with-umbrella` | clean, earlier | 0 (13:45) | clean | OK | ✓ | 0 fail (13:53) | 6/6 (13:43) |
| `photographer-with-camera` | clean, earlier | 0 (12:45) | clean | OK | ✓ | 0 fail (13:04) | 6/6 (13:40) |
| `picnicker-sitting-on-ground` | clean, earlier | 0 (13:57) | clean | OK | ✓ | 0 fail (14:26) | 6/6 (13:44) |
| `pregnant-person` | clean, earlier | 0 (12:15) | clean | OK | ✓ | 0 fail (12:42) | 6/6 (13:39) |
| `person-with-arm-sling` | clean, this takeover | 0, was 69 (23:23) | clean, was 4 | OK | ✓ | 0 fail, was 70 (23:32) | 6/6 (23:40) |
| `postal-carrier` | clean, this takeover | 0, was 181 (23:33) | clean, was 13 | OK | ✓, was ✗ | 0 fail, was 91 (23:39) | 6/6 (23:41) |
| `person-with-rollator` | clean, this takeover | 0, was 79 (22:30) | clean, was 8 | OK | ✓ | 0 fail, was 126 (22:45) | 6/6 (23:42) |
| `person-with-guide-dog` | open | 339, was 664 (22:16) | 8, was 17 | OK | ✓ | 42 fail, was 105 (22:26) | 6/6 (22:52) |
| `person-with-white-cane` | open | 114, was 236 (21:34) | 4, was 9 | OK, was 1 failing check | ✓ | 56 fail, was 84 (21:45) | 6/6 (22:48), `🧊️catalog-silhouette` was red |
| `person-stretching` | open | 499, was 1008 (23:49) | 23, was 47 | OK | ✓, was ✗ | 168 fail, was 189 (00:00) | 6/6 (23:56) |

`validateAsset` 0 issues and three-views sweep 0 findings for all twelve. The six suites are `🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation`, `⚡️asset-power` with the name filter (`⚡️asset-power` has no test of these assets: 24 skipped, as before). The six assets marked "earlier" were proven on the engine of 11:10; their catalog files changed once since (2026-11-25 19:09, `asset.tags` rewritten as titled objects for the whole catalog by its owner; compared key by key with the proven copies: nothing else differs).

## What was changed in this takeover

Every repair is the last step of the chain of its asset: `W/ops/<id>.json` with `W/fit.ts` (first fixer), then `W/lofts/<id>.json` with `W/lofta.ts`, then `W/after/<id>.json` with `W/fit.ts`. `W/land.sh` runs the chain on the catalog file, runs the lofts a second time (0 views changed, file unchanged in all six) and refuses unless the result equals the trial copy that was measured.

- **`person-with-arm-sling`** (`after/`): `sync-forward` 37 to 38 mm against 36 at 1:50 and 1:100 came from the plan prism of the left hand, which ended at the knuckles (u 0.14) while the side outline runs to the finger tips (u 0.178). Its finger end is drawn to u 0.16. The whole side outline as the prism leaves the plan guard (40.6 mm against 15), so does every loft of the hand (below). Silhouette numbers unchanged (plan 8.4 and 9.4 mm).
- **`postal-carrier`** (`after/`): (1) print: in `passing-right`, `contact-left` and `reading-address` coat and shank left a stub of the front thigh 0.190 to 0.194 mm high at 1:500 (limit 0.2 mm, check ✗); the knee end of the two front thighs is 12 mm longer, under the shank (`prints-where.ts`: no violation; three other variants measured and dropped). (2) `sync-bone-forward` 72 to 73 mm on `hand-l` at 1:50 and 1:100: the plan had the flat plate only; the prism `hand-l-body` of palm and fingers stands beside it, as the arm sling has it (same rig, same plates).
- **`person-with-rollator`** (`lofts/`, `after/`): both hands at 1:50 as a finger loft and a thumb loft, the coat and its shoulders at 1:50, 1:100 and 1:200 from the sections of their own solids (a fit of the three solids left the 1:50 front guard at 15.1 mm, their own sections give 8.4, was 11.4). The last 14 failing cells (`sync-forward` 55 mm in `seated-rest>stoop-to-basket`) came from the frame turning about the vertical (`aid-turn`, twist 180 in `seated-rest`): the side elevation drew the wheels flat in the middle plane; tyres, rims and hubs get the lateral span of their track there (12 shapes; the ring outlines stay, a disc copied from the plan filled the wheel openings and left the guard at 84.8 mm).
- **`person-with-white-cane`** (`lofts/`): both hands at 1:50 as palm, finger loft and thumb loft (fingers and thumb cut across their own bones), forearms, wrists and sleeves from their solids; 24 views. The 1:50 front guard is 15.0 mm (was 16.9, the `lateralScale` case MX-2 named for this lane: the forearm shapes that carried the member are replaced).
- **`person-with-guide-dog`** (`lofts/`): the dog's thighs and shanks at 1:50, thighs and neck at 1:100, forearms, jaw and the handler's right thigh at 1:200, from the sections of their solids. The hands are not changed (below).
- **`person-stretching`** (`lofts/`, `after/`): right shoe, sole and neck at 1:50 and 1:100, shoe, thighs, right sleeve and seat at 1:200, thighs, left sleeve, neck, right shank and seat at 1:500, and the skinned hoodie as `sil-body` at 1:500 (the op `loft` of `fit.ts`); 49 views. The 1:500 band prints now (the seat stub of 0.192 mm is gone).

Sheets opened: `person-with-white-cane` 1:50 after; `person-with-rollator` 1:50 before and after (no visible change); `person-with-guide-dog` 1:50 after; `person-with-arm-sling` 1:50 after; `person-stretching` 1:50 and 1:500 after. Rendered and not opened: rollator 1:100 and 1:200, guide dog 1:100 and 1:200, arm sling 1:100, postal carrier 1:50, 1:100 and 1:500 (its second step changes a hidden prism and a hidden knee end; the guard numbers did not move).

## Open

1. **`person-with-white-cane`, 56 cells**: all `sync-bone-height` of `thigh-l` at 1:500 (91 to 93 mm, the flat front thigh stands above its place). The repair is known and is held back on the order of MX (11:41) and MX-2 (19:58): with the thigh at its place the pocket rule of `🧊️catalog-silhouette` reads 167.0 mm against 150 (request "multiview MH-4 — the small-hole rule turns a corrected drawing red"). The rule is repaired in `U/🧰️matrix/MX-2/pocket` and not landed. When it lands: add `{ "scale": "500", "bone": "thigh-l", "solids": ["thigh-l"], "paint": "thigh-l" }` and the same for `thigh-r` to `W/lofts/person-with-white-cane.json` and run `land.sh` (measured at 20:37 with these two: gate 0 hard, scan 0 hard, 1:500 front 167.0 mm under the rule of today).
2. **`person-with-guide-dog`, 42 cells**: `sync-bone-forward` (77 mm) and `sync-bone-height` (81 mm) of `hand-l` and `hand-r` at 1:50. The side elevation draws the whole hand on the hand bone, front and plan draw the palm there and the fingers on their bones. Tried, each clears the gate (0 hard) and leaves the guard: hands as lofts in three variants (front 28.2 mm, plan 16.9 mm against 15), the prism of the side elevation copied behind the hands in front and plan (60.0 and 35.8 mm), the same with its finger half shortened (52.5 and 31.9 mm). Request filed (below).
3. **`person-stretching`, 168 cells**: `sync-bone-forward` 84, `sync-bone-height` 70, `anchor-landmark` 28 (per finding code, cells overlap). Left: the arms (`arm-r` 57 mm forward, 183 mm height, 137 mm lateral at 1:50 and 1:100), the extents in the stretch poses (`sync-forward` 50 mm, `sync-height` 39 mm, `sync-lateral` 55 mm: hem rib, hair and head, the left sole), the right foot at 1:500 (`sync-bone-height` 140 mm, anchors `heel-r` 97 mm and `ball-r` 41 mm). Curable in the asset and not reached: with the two sleeves and the right forearm as lofts at 1:50 and 1:100 the gate falls to 184 hard findings, but the guard reads 16.0 mm against 15 (1:50 plan) and 30.2 against 30 (1:100 front), so they are not written (`W/lofts-trials/person-stretching.v3.json`); the right shoe as a loft at 1:500 draws the band 0.717 m wide against the declared 0.701 m (`extent-width`, 1 mm over the tolerance) and is left out too.

## Requests in `U/requests.md`

- Filed in this takeover: "multiview MH-4 — hands whose fingers hang on bones of their own: no loft drawing of the hand passes the multiview gate and the silhouette guard together in three of four people" (table of five variants on the arm sling, postal carrier and guide dog; offers `W/lofta.ts`). Since then the arm sling and the postal carrier are cured with a plan prism instead; it stands for the guide dog.
- Of the earlier fixers, still open: "multiview MH-4 — `🧊️catalog-silhouette`: the small-hole rule turns a corrected drawing red" (answered, repair not landed: item 1 above).

## Queue

One lane of 1000 MB served every run of the takeover in name order (`W/lane.sh`, `W/jobs.sh`), the matrix inside it at 800 MB (figures of MM, 20:05), the suites under a second request of 2500 MB. Waits for the lane: 19:05 to 20:10 withdrawn unserved at 1200 MB on the coordinator's word, 20:10 to 20:32, 21:09 to 21:29. The lane of 20:32 ended at 21:03 while a job ran, cause not found (no line in its log); the job was queued again. Peaks read from `rss.log`: matrix of a person 292 MB, `measure.ts` 914 MB, gate 153 MB, scan 235 MB, check CLI 341 MB.

## Files

Updated (catalog, repo-relative): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/<id>/🔣️.json` for `person-with-arm-sling`, `postal-carrier`, `person-with-rollator`, `person-with-guide-dog`, `person-with-white-cane`, `person-stretching` (this takeover); for `person-sweeping`, `person-taking-selfie`, `person-with-umbrella`, `photographer-with-camera`, `picnicker-sitting-on-ground`, `pregnant-person` by the earlier fixers. No engine, test, fixture, manifest or README was edited.

Created in `U/🧰️matrix/MH-4/` (this takeover): `lofta.ts` (the loft tool of MX-2 with `"across"` in a `fit`), `land.sh`, `trial.sh`, `probes.sh`, `solids-at.ts`, `same.py`, `after/person-with-arm-sling.json`, `after/person-with-rollator.json`, `after/postal-carrier.json`, `lofts-trials/` (the hand variants and the stretching variants that were measured and not written). Updated: `lofts/*.json` (the ops that are in the catalog; `[]` for the arm sling and the postal carrier), `loft-try.sh` (takes `lofta.ts`), `look.sh` (`BEFORE_DIR`), `final.sh` and `prove.sh` (matrix at 800 MB). Kept from the earlier fixers: `ops/`, `ops-trials/`, `after/person-stretching.json`, `notes/`, the tools.

Updated: `U/requests.md` (one request appended), this report. Removed: `U/🗑️generated/matrix/MH-4` (trial copies, logs, sheets).
