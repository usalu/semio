FINAL

# 🔭️ Multiview: Shared-Code Owner MX-2 (2026-11-25)

Paths: `R` repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` this ticket folder, `K` the multiview ticket of 2026-11-02. Scripts: `U/🧰️matrix/MX-2/`. Every run goes through `repo capacity run` with the pinned bun.

## Status (takeover at 19:00, Sonnet 5.5)

| # | Assignment | State |
|---|---|---|
| 1 | Loft shape, proofs | done: suite `🧊️loft-shape` 29 of 29 (19:52); `tour-guide` matrix 252 cells, 0 fail on the engine of 20:06; the engine suites that the changes touch are green, see "Suites"; the recipe below stands |
| 2 | `steel-column` 1:100 | done in the asset (19 ops of 16:40): front against elevation 0.03012 at 1:50 and 1:100 (90.4 mm of 90 mm, the gusset is drawn, 0.9 mm on paper); matrix 252 ok on the engine of 20:06; the allowlist keeps bands 200 and 500 |
| 3 | Allowlist merge | done: nothing new from MB-1, MB-2, MS, MX; ML-3's three plant entries merged band-limited, the three bandless ones withdrawn (`withdraw`, D61); 15 and 48 entries, each needed in exactly its bands |
| 4 | Pocket rule | prepared and measured (and, new, the body-following rule for request MU), NOT landed: assets are written all the time; see its section |
| 5 | Requests | every `## multiview …` request answered, see "Requests" |
| 6 | Typecheck line 254 | nothing left to fix: both type checks exit 0, see "Final runs" |
| 7 | Font of the engine | done and announced (D60) |
| + | ML-2 print tie, ML-3 plants, MU slender solid | done, done, prepared |

## Font: the engine measures the text with the font it writes (7; engine change 19:50)

Cause: `textWidthMm` summed Adobe Helvetica advances while `writeSvg` and the explanation box name `Anta` first. Anta is wider: measured with PyMuPDF on the Anta files of the repo, digits 656 against 556 per mille (18 %), most small letters 3 to 15 % (`c` 580/500, `s` 583/500, `t` 393/278), narrower for `1` (326), `I`, `J`, `M`, `G`; a caption `2000 2000 …` is 18 % wider than reserved, the 4 % of `TEXT_SLACK` does not cover it.

Change (all under `S/🔨️modules/`): `📐️units`: `textWidthMm(text, sizeMm, bold = false, typeface = "anta")`, tables `ANTA_ASCII` (95), `ANTA_LATIN1` (96) and `ANTA_SYMBOLS` in 2048 units per em, taken from the four Anta subsets (`🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta`), bold has the same advance (the face has no bold), no kerning, an accent takes its base letter, a character outside the face is one em; Helvetica stays under `"helvetica"` for the sheet captions of `🎨️depiction/🖨️preview` (its `FONT` is Helvetica) and nothing else; `DRAWING_FONT_STACK` is one constant now (was copied in `🖼️svg` and `🪧️explanation`); `🪧️explanation` wraps by measured width (`wrapTo`: `textWidthMm` times `TEXT_SLACK` against the frame, at the size and weight the line is written in; before: 0.56 em per character at the label size for every line, also the bold ones); `🧱️canvas`: the docstring of `TEXT_SLACK` names Anta.

Tests first (`🧪️tests/🔤️caption-fit`, reference PyMuPDF measuring the Anta files, the first of the four subsets that has the glyph; `🐍️.py` takes `"font": "anta"` and the directory): (1) the table against PyMuPDF within 0.1 % for ASCII, Latin-1, 12 symbols and five strings, regular and bold; (2) the SVG writer names `Anta` first for every text of a drawn scene, and `2000` is wider in Anta than in Helvetica; (3) every text of the four scene cases stays inside the scene measured with Anta (the scene case measured with Helvetica before); (4) explanation boxes at 50, 60 and 80 mm in both languages keep every line inside the frame. Red before: (1) cannot run on the old signature; (4) with the old wrap `🧰️matrix/MX-2/bench/explain-old.ts` (scratch copy of the modules at HEAD for these files, the live Anta table) finds 2 of 54 lines wider than the frame (`2000 2000 2000 2000 2000 2000 2000 2000 2000`, 48.7 mm against 48), with the live module 0 of 51. Green after: 6 of 6 (19:52). (3) was not shown red: the scene texts are short and the slack covers them; it stays as the guard of the reservation.

Effect on cells, byte for byte (`🧰️matrix/MM/one.ts`, same snapshot of the document, the suite of a scratch tree whose `units`, `canvas`, `preview` are the files of HEAD against the live suite): `finned-heat-exchanger` 52,561 bytes identical, `boxwood-ball` 46,051 identical, `jogger` 209,397 identical (the jogger run took 24 minutes on the loaded host). The scene size (`widthMm`, `heightMm`) of drawings with text grows where Anta is wider; no matrix cell reads it (`sceneBox` takes paths only).

Not done: the React target (`🎯️targets/⚛️react/🔨️modules/🧮️pipeline/🔤️text`) measures the browser's face against `textWidthMm * TEXT_SLACK`; its overhang is smaller now because the reservation is Anta's. Its `🔤️text-reach` suite and the site's label-fit are not run here (Linux lane s5); a browser check of the exported SVG caption is open.

## Print tie: an ink gap of zero is a join (request ML-2, engine change 19:50)

`gapsBetween` (`🖨️representation/📐️measure`) skipped a pair only when `ink <= 0`; the centreline touch had `TOUCH_MM` (0.1 micrometre). It is now `ink <= TOUCH_MM`. Test first in `🧪️tests/📏️print-suitability`: four first-rectangle widths at 14 scales from 1:20 to 1:500 with the centre lines one pen apart (red: `1:30 ink gap 0 at 0.7`), and 0.02 mm farther apart still a gap of 0.02 mm (green before and after); green after 9 of 9; `MC/prints.ts used-hinged-wardrobe`: every authored scale prints (the 1:20 end gave a gap of 1.8e-15 mm). Cells: `finned-heat-exchanger` 52,561, `boxwood-ball` 46,051 and `jogger` 209,397 bytes identical to the engine before both changes.

## `steel-column` 1:100 (2)

The allowlist band 100 stays refused (`BAND_LIMITED` holds 200 and 500). The asset was repaired by the 19 ops of `🧰️matrix/MC/make/steel-column.py` with plates at their true 20 mm (applied 16:40, second run unchanged): the gusset edge stands 90 mm above the plate in the front and the web view has nothing there. Measured 19:08 on the catalog document with the measure of the suite (`MC/derived.ts --far`): front against elevation 0.03012 at 1:50 and at 1:100 (90.4 mm of 90 mm; 0.9 mm of the gusset on paper at 1:100, above the 0.2 mm feature minimum); 1:200 and 1:500 0.00667 (the allowed bands). Scan clean, `validateAsset` 0 issues, every authored scale prints, check CLI `✓`, silhouette guard OK (1:50 front 0.9993/5.3 mm; 1:100 elevation 0.9995/3.8 mm), matrix 252 ok (16:40); matrix 252 ok again on the engine of 20:06 ("Final runs").

## Allowlists (3)

`merge-allowlists.py` at 19:00, 19:55 and at the end: `added 0; derivedViewAllowlist 15, bandCopyAllowlist 48`. All entries of `MB-1`, `MB-2`, `MS` and `MX` are in `S/🧫️fixtures/🔭️multiview-coverage/🔣️.json` and were read: every one names its physical reason (bands where two views cannot differ by the measure; the 15 `derivedViewAllowlist` entries are measured per band by `allow-bands.ts`, seven limited to their bands by D58). No entry weakens a rule without one. The two MB-1 lines for `steel-column` front without bands (the asked band 100) are left out by the script, as the coordinator decided.

## Pocket rule and body-following rule of `🧊️catalog-silhouette` (4; request "multiview MU")

Two changes of the same module are prepared and measured, neither landed. Files: `🧰️matrix/MX-2/pocket/` (one patch of three changes; `patch.py --trial` writes `silhouette.ts`, `--land` patches the real module and refuses unless its anchors are found exactly once; `test.ts`: 14 cases on synthetic masks, 6 pass on the live module and 14 on the trial copy (nine for the pocket rule, five for `drawsBody`); `measure.ts`: both modules on catalog assets). Pocket rule: `agreedPockets` decides an enclosed hole of at most the limit once for both masks (filled in both when the other mask covers the cell or has such a hole there; kept as outline in both where the other is open), and `Mask.sealRings` closes hairlines of drawn rings as the seal closes those of solids. Measured on 17 assets (every scale and view, default pose; `🗑️generated` copy `pocket-measure.txt` is deleted with the folder, numbers here): 0 views change sides at the gate (fail to pass 0, pass to fail 0); `miscanthus-grass` plan 13.3 to 3.8 mm and elevation 9.4 to 1.9 mm; `person-with-white-cane` 1:500 front 80.2 mm unchanged by it.

Body-following rule (request MU, `used-bentwood-dining-chairs` 1:500): `drawsBody` says a slender solid whose projected centre line lies (nine tenths of its points) inside a filled closed drawn shape of kind `loft` of its bone and material (the `Drawn` entry carries `body` for a loft shape; the first version took any filled shape and turned `used-marching-snare-and-bass-drum` 1:500 elevation and plan red, 0.9645 to 0.8894 and 0.9518 to 0.8665, which is why the sample below exists), or within half its true width of the outline, is shown by that shape, and `strokesOf` then returns the projected triangles of the solid instead of one stroke of the width `truth` (twice the 90th percentile of the vertex distances). Cause found by measurement (`pocket/where.ts`, `loft.ts … --out`): the lofts that MU drew from the sections of the solids tapered from 21 to 30 mm while the stroke of the volume side has the constant width of the widest part, so the volume was 1,241 cells (3.7 %) larger than the drawing; the rule needs the lofts `filled` (a body), which the paint rule of `loft.ts` does not give when it takes the paint from an unfilled stroke: `loft.ts` has `"filled": true` in an ops entry now (`ops/bentwood-500.lofts.json`). Trial document (the lofts of back and front legs of the chairs `b` and `d`, filled), front IoU 1:500: live module 0.9118 (limit 0.92, the number MU measured: 0.9050), trial module 0.9464 (drawn 33,444 cells, volume 33,445); plan 0.9953 to 0.9954; elevation 0.9263 to 0.9447. A constant mean width instead (tried first) gives only 0.9202. The original document without lofts is unchanged by the rule (0.9412 in both). Other assets: the sample below.

Sample of the whole patch on the trial module (`pocket/measure.ts`, every authored scale and view in the default pose, gate of `🧊️catalog-silhouette`; "now" is the live module): `used-bentwood-dining-chairs` (document without lofts) 12 views, 4 move, 0 flip; `tower-crane` 12 move, 0 flip; `used-climbing-net-pyramid` 12 move, 0 flip (1:500 front 0.9911 to 1.0000); `used-marching-snare-and-bass-drum` 7 move, 0 flip (with the first version 2 flips to fail); `used-viola` 4 move, `used-baby-grand-piano` 7, `three-seat-sofa` 5, `city-bicycle` 9, `horse` 4: 0 flip each (`horse` 1:50 plan stays red at 18.5 mm, the known `lateralScale` case of MA). Before that, the pocket rule alone on 17 assets (five of them are in this list too): 0 flips. So the sample (21 different assets) found one flip, in the first version of the second rule; that is the reason to measure the whole suite before landing.

Why not landed: the condition is that no fixer writes assets. At 22:00 two documents had changed in the last ten minutes and the chains were still alive. At about 20:00 the ten chains of MA, MH-1, MH-2, MH-4, MH-5, MH-6, MU, MV-3, MM and ML-3 are alive (108 processes under `matrix/`) and about 300 catalog documents changed in the last 45 minutes. A change of the rule decides all 385 assets of `🧊️catalog-silhouette` (385 cases); proving it needs that suite before and after on one frozen catalog. Landing command, for whoever owns the frozen catalog: `python3 🧰️matrix/MX-2/pocket/patch.py --land`, then `🧊️catalog-silhouette` whole (about 75 s for nine cases, longer whole) against its result before.

## Requests (5)

Every `## multiview …` request of `requests.md` has an answer under it now (`**Answer MX-2 (…)**` lines, placed with `🧰️matrix/MX-2/answer.py`):

| Request | Verdict | Where |
|---|---|---|
| MH-6 tracer plates; MH-4 opposite gates (seven places); MH-2 walkers | the drawing solid asked for exists (`loft`, D54), recipe and measured effect on `tour-guide`; the work on the 13 other people is the owners' | answers 19:55 |
| MH-4 small-hole rule; MV-2 pocket rule | defect of the guard, repair written and measured on 17 assets (0 verdicts flip, distances fall), not landed | answers 19:55 and 19:58; "Pocket rule" |
| MU slender solid does not follow a `loft` | cause measured (constant stroke width against a tapering loft), repair `drawsBody` prepared: front IoU 0.9118 to 0.9464 on the trial; not landed | answer 21:45; "Pocket rule and body-following rule" |
| MV-3 prism with hundreds of profile points | measured: 271 ms against 13 ms at 764 points since MM's engine; cost no longer a reason for plates | answer 19:55 |
| ML-2 ink gap exactly zero | defect of the measure, repaired (D59 below) | answer 19:55; "Print tie" |
| ML-3 three bandless plant entries | band-limited entries merged, bandless ones withdrawn, scan clean | answer 20:20; "Allowlists" |
| MM engine speed-up, MM queue, MM vitest reservation, MX-2's own notes | landed at 17:48 / for the coordinator / noted (3600 MB is what the coverage suite is asked with here) | none needed |
| site-mac queue | not a multiview request; my part: lanes hold 1000 MB each and end after 150 s idle (the ones named there, 90448, 18860, 11951, 55579, are not mine except as noted in "Incident") | |

Decisions for the dev (also in `K/🗂️work-board.md`): **D59** an ink gap of at most 0.1 micrometre between two shapes is a join, as the centre line touch is (ML-2). **D60** the engine measures text in the face it writes, Anta, with a table checked against PyMuPDF; Helvetica remains the sheet captions' and the PDF's own face. **D61** an allowlist entry can be replaced by a narrower one through `withdraw` in the batch file of the merge script.

## Suites and checks after the changes of 19:30 to 20:20 (1)

Run through the queue, each to its end, logs read (peak memory from `time -l`):

| Suite or check | Result |
|---|---|
| `🔤️caption-fit` | 6 of 6 (19:52, peak 1427 MB) |
| `📏️print-suitability` | 9 of 9 (red 2 before the repair, 19:48) |
| `🧊️loft-shape` | 29 of 29 |
| `🧊️solid-sweep` 21 of 21; `🔺️polygon-union` 27 of 27; `🧊️volume-schema` 115 of 115; `🧬️schema-conformance` 44 of 44 | green |
| `📑️multiview-sheet` 27 of 27 (peak 2379 MB, the preview caption wrap); `🤐️textless-depiction` 13; `📏️print-size` 4; `📐️paper-dimensions` 9; `🎨️seven-modes` 6; `🎨️structure-legend` 5; `📕️pdf-reopen` 5; `🖊️line-hierarchy` 8 | green |
| `🎨️thermal-legend` | 13 of 14: the case "weights the merged regions … of the 1:500 silhouette" reads `shape:sil-torso` of `walking-commuter`, which that asset no longer has (document rewritten 19:09 by its owner, batch MR); asset-caused, not the engine; the test needs an asset with a merged 1:500 silhouette, or the shape id of the asset's new drawing; not changed here |
| `🕳️fill-rule` | 10 of 11: 15 overlaps in catalog documents, none in the engine: `seated-office-worker` 1:200 (0.001 m2), `tram` 1:50 and 1:100 (0.0002, 0.0008 m2, 8 shapes), `park-bench` 1:200 and 1:500 (0.013 to 0.054 m2, 6 shapes); owners MH-5, MU, MB-1 |
| `🧊️chain-state` | 18 of 20: the two reds of the earlier owner (`ranks the right of the asset nearer in the elevation …` 0.0637 against 0.336; `person-on-crutches` rest z, `tripod-stand` elevation minX 0.1486), asset-dependent, unchanged by this pass |
| `derived-view detection` of `🔭️multiview-coverage` | see "Final runs" |
| tsc of the staffage package | exit 0, 0 errors (18:21 by the predecessor; the file `🗂️catalog/🧊️volume/🟦️.ts` last written 16:13) and see "Final runs" |

## Incident (mine, to be checked by the owners)

At about 20:02 I stopped two `repo capacity run` wrappers with a pattern on the command line (`--memory=2500` and `MX-2` or `lane`), meant for my own waiting lane. The word `lane` also occurs in the `lane.sh` of other batches (MH-1, MH-4, MH-6 chains are named so). pid 57693 was probably my own waiting request; pid 90448 (a 2500 MB wrapper running since 17:05 with 225 to 430 MB observed, parent 216) I did not identify before or after. The lanes of `MH-4` and `MH-6` were orphans (parent 1) when I looked, which means their wrappers were gone before; I could not establish afterwards whether a process of another batch ended with the one I stopped. If a chain of another batch lost its grant at 20:02, it has to resubmit. The kill rule of 21:10 in `📋️takeover-brief.md` came after this; I follow it since: every later stop named a pid I started myself (`memory=3600` and `MX-2` both in the command line).

## Final runs on the engine of 20:00 to 21:40 (each read to its end)

| Run | Result |
|---|---|
| tsc of the staffage package (`tsc --noEmit -p tsconfig.json`, 877 MB) | exit 0, 0 errors (21:00), after every change of the tests and modules above |
| tsc of the React target (`🎯️targets/⚛️react/📦️packages/🟦️typescript`, 766 MB) | exit 0, 0 errors (21:32). The two errors at line 254 of `🗂️catalog/🧊️volume/🟦️.ts` (TS7006, `p.map((value, i) => …)` after `vector(section.at, 3)`) are gone: the file was last written 16:13 (the helper `coincident` with typed parameters), and both type checks were clean afterwards |
| `🔭️multiview-coverage`, name filter `derived-view detection` | 5 of 5 (21:00, 310 MB): the allowlists of the fixture are well formed (catalog assets, authored bands, reasons in both languages) with the three new band-limited entries |
| `🔭️multiview-coverage`, whole suite | NOT run here: it needs a 3600 MB grant and none was admitted in 80 minutes of waiting (a request was cancelled by the runtime restart of 21:02); the last whole run is the predecessor's 41 of 41 before the changes of this pass, none of which touches `T` or its cases |
| matrix `steel-column` (`📊️matrix.ts --fresh`, engine of 20:06) | 252 cells, 252 ok, 0 fail |
| matrix `tour-guide` (same, 20:06 to 20:14) | 252 cells, 70 ok, 182 `group-drift` (reported, not failing), 0 fail |
| scan of `boxwood-ball`, `norway-spruce`, `windmill-palm` with the band-limited entries | 3 of 3 clean, 0 hard findings |
| `merge-allowlists.py` at the end | `added 0; derivedViewAllowlist 15, bandCopyAllowlist 48` |

Not run: the matrix of the three plants (their owner ML-3 re-runs it), the React suites `🔤️text-reach` and the site's label-fit after the font change (lane s5), a browser look at the exported SVG caption.

## Recipe

How a part that no prism, plate or body of revolution agrees for is drawn as `loft` shapes (README "Loft shapes", D54). Reference: `tour-guide` 1:50, both arms and hands (`🧰️matrix/MX-2/ops/tour-guide.lofts.json`, written 17:13).

### When

The part turns in a pose or clip, its three views agree at the default pose only (`gate.ts --twins` names it in `sync-bone-*`, `group-drift-*` or `twin-*`), and the silhouette guard refuses the prism or the body of revolution. In the volumes the part is a `loft`, an `ellipsoid` or a `capsule` on one bone. A part that is a prism stays `extruded`, a round one `revolved`, a straight bar a `member`.

### Steps (`<id>` the asset, all through the queue with 800 MB; `source U/🧰️matrix/MR/env.sh` with `BATCH=<yours>`)

1. Back the document up and list the body solids of the bones the gate names (`volumes.<scale>.body`, member `bone`): kind, `id`, `weights`, `repeat`. Take the whole limb (upper arm, forearm, hand), on both sides, not only the one shape the gate names: neighbours drawn by the same sections join without steps, and the drawing of the limb becomes the body (`tour-guide` plan 11.9 mm before, 4.2 mm after).
2. Write `🧰️matrix/<BATCH>/ops/<id>.lofts.json`, one entry per scale and bone:

   ```json
   [
     { "scale": "50", "bone": "upper-arm-l", "paint": "sleeve-l" },
     { "scale": "50", "bone": "forearm-l", "paint": "forearm-l" },
     { "scale": "50", "bone": "hand-l", "paint": "hand-l" },
     { "scale": "50", "bone": "hand-r", "paint": "hand-r", "swell": { "hand-r-thumb": 0.0025 } }
   ]
   ```

   `paint` is the drawn shape whose role, z, tone, region and pen a new loft takes where the view has no shape of the solid's own id. Old shapes with other ids on the bone are named in `"replace": [ids]` (a traced hand with the id `palm`, say); a shape that is not replaced and not named stays, which is right for pocket lines and wrong for the old outline. `"solids": [ids]` takes some of the solids only, `"views": […]` some of the views only (use it only when the other view must stay for a stated reason: three views are the point).
3. Dry run and measure, in this order (each refuses for another reason, cheapest first):

   ```sh
   cd "$S/📦️packages/🟦️typescript"; "$B" "$U/🧰️matrix/MX-2/loft.ts" <id> --ops <ops> --out <trial.json>   # 0 refused
   cd "$T"; "$B" "$U/🧰️matrix/MC/prints.ts" <trial.json>                                                   # every authored scale prints
   cd "$S/📦️packages/🟦️typescript"; "$B" "$U/🧰️matrix/MX-2/guard.ts" <trial.json>; "$B" "$U/🧰️matrix/MC/validate.ts" <trial.json>
   cd "$T"; "$B" "$U/🧰️matrix/MR/gate.ts" <trial.json> --twins                                              # hard findings: 0
   U/🧰️matrix/MX-2/sheets.sh after <trial.json> <scale> <pose>…                                             # open them
   ```

   `U/🧰️matrix/MX-2/tour-try.sh` runs the middle three for several variants of the ops.
4. `loft.ts <id> --ops <ops> --write`, and once more: it must say `0 views changed` and `file unchanged`. It is the last step of the asset's chain, after `fit.ts` and `bodies.ts`.
5. The proof list of the multiview brief.

### What the tool refuses, and the answer

| Refusal | Answer |
|---|---|
| prints: `gap` between two lofts (fingers 0.4 to 7 mm apart at 1:50, less than 0.15 mm on paper) | `swell` one of the two by 2 to 4 mm, so that they touch (`tour-guide`: thumb 2.5 mm; swelling thumb AND index 2 mm each only moved the gap to the thenar). When several fingers stand apart, `fit` them into one loft: `"fit": [{ "id": "hand-r-fingers", "solids": ["hand-r-index", "hand-r-middle", "hand-r-ring", "hand-r-little"], "stations": 6 }]` |
| prints: `feature` (a part thinner than the scale prints) | the same, or leave the part out of the drawing when the band's decisions say so |
| `a box / extrude / revolve is not a loft` | name the kind that fits: a box or extrude is an `extruded` prism (`MH-6/bodies.ts`), a revolve a `revolved` solid; or `fit` it into the loft beside it |
| `has skin weights or a repeat` | a skinned body solid (a coat, a trouser leg) is not taken by the tool; the shape itself may state `weights` (README), written by hand with the sections of the solid |
| validate `loft-fold` | the path bends tighter than the section is wide: use fewer stations, or split the part at the bend (one loft per bone) |
| guard: a view is worse than before | the old plate drew a neighbour as well (a cuff, a thumb): add the neighbour's bone to the ops |
| gate: an anchor leaves its shape | the anchor was bound to the place of the old plate; bind it to the bone that draws its place (D45) |
| the loft is drawn as an outline only (the shape it took the paint from is an unfilled stroke) | an entry may say `"filled": true` (tool change of 21:30): a loft is a body, and `drawsBody` of the guard (prepared) needs it |
| one loft through several separate bodies (the 12 ellipsoids of a foliage mass on one thread) | one loft per body, as the hand taught (31 mm in one loft); see the answer to "multiview ML-3 — loft silhouette fol-6" |

Limits measured: the drawing cost is at most sections − 1 convex hulls per loft and view (a hand of seven lofts evaluates like the seven prisms before); a hand with thumb beside the palm needs one loft per part, never one for the hand (31 mm, D54). `volumes-feature` reads the width of the body of a loft, not its tapered tip (D57).

### What it did for `tour-guide` (1:50, both arms: 18 views redrawn, 0 refused)

Guard: elevation 0.9814/11.9 mm → 0.9811/11.9, front 0.9710/12.0 → 0.9733/12.0, plan 0.9798/11.9 → 0.9907/4.2 (limit 15 mm). Gate over 117 subjects: 0 hard before and after; `group-drift-height` ×47 worst 92 mm (forearm-r) → none; `twin-height` ×91 worst 97 mm → none; `twin-forward` ×59 worst 81 mm (hand-r) → ×8 of the jacket. Sheets opened before and after in `standing-pointing`, `wave-swing-a`, `gesturing-to-group`, `holding-flag`: the flat hand boxes and the clipped forearm of the waving pose are gone, cuffs, wrists and fingers read in all three views.

### The 13 other people (section 5 of `multiview-MX.md`)

`tennis-player` (plan shirt: skinned or on the torso, see `fit`), `waving-person` (1:500 left arm: clean since MH-6, nothing to do unless its gate says so), `worker-with-jackhammer` (left hand 1:50), `worker-carrying-box` (upper arms and hands), `traveller-with-rolling-suitcase` (hand and grip) of MH-6; `person-with-arm-sling`, `postal-carrier`, `person-with-rollator` (trunk: one loft of 17 sections is 14.6 mm of 15 at 1:50, `fit` with 16 stations), `person-with-guide-dog`, `person-stretching` of MH-4; `nurse-walking`, `office-worker-carrying-laptop`, `patient-with-iv-pole` (arms, hands and shoes; foot-l one loft 4.1 mm) of MH-2. A shoe is `foot-*` and `toe-*`, two lofts with their soles.

## Files (repo-relative; `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` and `K` as above)

Updated, shared code and tests: `S/🔨️modules/📐️units/🟦️.ts` (Anta table, `typeface`, `DRAWING_FONT_STACK`), `S/🔨️modules/🎨️depiction/🪧️explanation/🟦️.ts` (wrap by measured width, shared stack), `S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts` (Helvetica explicit), `S/🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts` (docstring), `S/🔨️modules/📤️export/🖼️svg/🟦️.ts` (shared stack), `S/🔨️modules/🖨️representation/📐️measure/🟦️.ts` (`TOUCH_MM` for the ink gap), `S/🧪️tests/🔤️caption-fit/🟦️.ts`, `🐍️.py`, `S/🧪️tests/📏️print-suitability/🟦️.ts`, `🥒️.feature`, `S/🧫️fixtures/🔭️multiview-coverage/🔣️.json` (three plant entries band-limited).

Updated, tickets: `U/multiview-MX-2.md` (this report), `U/requests.md` (nine answers, one note), `K/🗂️work-board.md` (D59, D60, D61), `U/🧰️matrix/merge-allowlists.py` (`withdraw`), `U/🧰️matrix/MX-2/loft.ts` (`"filled"`), `U/🧰️matrix/MX-2/pocket/patch.py`, `pocket/test.ts`.

Created: `U/🧰️matrix/MX-2/lane.sh` (one grant, many jobs), `answer.py`, `answers/1.json` to `5.json`, `bench/prism-cost.ts`, `bench/explain.ts`, `bench/explain-old.ts`, `ops/bentwood-500.lofts.json`, `pocket/where.ts`; the trial copy `pocket/silhouette.ts` is rewritten by `patch.py --trial`.

Removed: `U/🗑️generated/matrix/MX-2` (tool output, trial copies, the scratch tree of HEAD files); `pocket/silhouette-truth.ts` (an experiment).
