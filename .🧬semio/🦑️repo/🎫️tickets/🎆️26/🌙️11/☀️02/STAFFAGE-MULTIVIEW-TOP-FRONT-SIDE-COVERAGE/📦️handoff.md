# 📦️ Handoff: Staffage Multiview (Top, Front, Side)

Status: CLOSING DRAFT 2026-11-27. Not closed. One FAIL cell is OPEN (section 3). Every number below is taken from the sources named in the section; nothing is claimed beyond them.

Ticket (T): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`.
Library (L): `🎓️teaching/🏛️architecture/🧍️staffage`.
Umbrella (U): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`.

Sources:
- Matrix: `U/🗑️generated/closing/matrix/summary.json` (totals and per-category figures, read with python) and `U/🗑️generated/closing/matrix/matrix.json` (37.5 MB, streamed only to read the failing cell).
- Closing log: `U/🗑️generated/closing/summary.txt`.
- Lane reports: `U/🗑️generated/closing/closing-CL-A.md`, `closing-CL-H.md`, `closing-CL-O.md`, `closing-horn-case.md` exist. `closing-CL-CH.md` does not exist yet.
- Proof sheets: `T/🖨️sheets/`.
- Decisions and conventions: `T/🗂️work-board.md`.
- Dev review items and state log: `U/🤝️handoff.md`.

## 1. Coverage matrix (full run of 2026-11-26)

Run: the closing log records `closing start 2026-11-26 18:45:37` and `STEP matrix exit 0 seconds 33958`. The matrix files are dated 18:58 (`matrix.json`, `summary.json`). The log's duration does not match the 18:45 to 18:58 window; the sources do not reconcile the two. Per the matrix run block: 375 assets done, none failed, none missing. The per-asset seconds sum to 85,174 s (a sum, not wall time).

Column meaning (from the matrix and the work board): `ok` checked and passing; `unavailable` declared unavailable with a cause (D8, D17); `range-end` cells at the end of a `validRange` (D30); `group-drift` reported tier, not a FAIL (D41); `fail` FAIL.

Category table (summary.json `byCategory`):

| Category | Assets | ok | unavailable | range-end | group-drift | fail |
|---|---:|---:|---:|---:|---:|---:|
| humans | 75 | 6,237 | 42 | 1,344 | 11,277 | 0 |
| vegetation | 67 | 12,334 | 105 | 210 | 4,235 | 0 |
| objects | 58 | 11,333 | 273 | 21 | 2,884 | 0 |
| components | 58 | 13,244 | 231 | 0 | 1,162 | 0 |
| furniture | 39 | 7,202 | 63 | 0 | 2,541 | 1 |
| appliances | 39 | 7,252 | 651 | 0 | 1,757 | 0 |
| machinery | 16 | 3,045 | 168 | 147 | 672 | 0 |
| animals | 14 | 2,611 | 189 | 0 | 539 | 0 |
| vehicles | 9 | 1,610 | 42 | 42 | 574 | 0 |
| **total** | **375** | **64,868** | **1,764** | **1,764** | **25,641** | **1** |

Per view (cells are ok / unavailable / range-end / group-drift / fail; summary.json `byView`):

| Category | elevation | front | plan |
|---|---|---|---|
| humans | 1,722 / 14 / 448 / 4,116 / 0 | 2,149 / 14 / 448 / 3,689 / 0 | 2,366 / 14 / 448 / 3,472 / 0 |
| vegetation | 4,004 / 35 / 70 / 1,519 / 0 | 4,165 / 35 / 70 / 1,358 / 0 | 4,165 / 35 / 70 / 1,358 / 0 |
| objects | 3,556 / 91 / 7 / 1,183 / 0 | 3,920 / 91 / 7 / 819 / 0 | 3,857 / 91 / 7 / 882 / 0 |
| components | 4,354 / 77 / 0 / 448 / 0 | 4,431 / 77 / 0 / 371 / 0 | 4,459 / 77 / 0 / 343 / 0 |
| furniture | 2,162 / 21 / 0 / 1,085 / **1** | 2,506 / 21 / 0 / 742 / 0 | 2,534 / 21 / 0 / 714 / 0 |
| appliances | 2,373 / 217 / 0 / 630 / 0 | 2,373 / 217 / 0 / 630 / 0 | 2,506 / 217 / 0 / 497 / 0 |
| machinery | 952 / 56 / 49 / 287 / 0 | 1,043 / 56 / 49 / 196 / 0 | 1,050 / 56 / 49 / 189 / 0 |
| animals | 861 / 63 / 0 / 189 / 0 | 903 / 63 / 0 / 147 / 0 | 847 / 63 / 0 / 203 / 0 |
| vehicles | 504 / 14 / 14 / 224 / 0 | 504 / 14 / 14 / 224 / 0 | 602 / 14 / 14 / 126 / 0 |
| **total** | **20,488 / 588 / 588 / 9,681 / 1** | **21,994 / 588 / 588 / 8,176 / 0** | **22,386 / 588 / 588 / 7,784 / 0** |

Checks counted in the matrix: `language-geometry`: 1 (the one FAIL). No other check fired.

## 2. Coverage: exhaustive and sampled

Source: `matrix.json` field `coverage`.

Exhaustive:
- representation decisions of every preset and declared key
- valid-range band edges of every authored representation
- unavailable declarations: reason, limiting feature and size, alternatives and their targets
- geometry of every pose in the three viewpoints
- cross-view synchrony of every evaluated instant (forward, height, lateral)
- default-pose dimensions against widthM, depthM, heightM
- loop closure of every looping clip
- all seven modes of every pose at every scale entry

Sampled:
- clip instants: start, every keyframe, mid of every key interval, extremes of rig bounds and view extents (48 probes), 8 uniform intermediates (16 for clips longer than 600 s), end; all seven modes at every instant
- transitions: every pose to the rest pose plus three seeded random pairs (mulberry32, seeded by the FNV-1a hash of the asset id) at 0, 0.25, 0.5, 0.75 and 1 of one second; all seven modes
- languages: English at every subject; German additionally at every pose and at the start, key, extreme and end instants (not at uniform, mid or transition instants)
- band edges (validRange ends, up to 1:1000): poses and the start and end instants of every clip
- alternatives: scale alternatives reuse the target cells; symbols render every pose; asset alternatives are checked for an authored target at the scale

Viewpoints: elevation, front, plan.

Modes (7): dark-line, dark-solid, light-line, light-solid, thermal, forces, moments.

## 3. OPEN: the one failing cell

| Field | Value |
|---|---|
| Asset | `used-bentwood-dining-chairs` (furniture) |
| Scale | `500:band-max` (requested 1:1000, served by the 1:500 band) |
| View | elevation |
| Mode | forces |
| Check | `language-geometry`, count 1 |
| Pose | `in-use` |
| Message | "the de drawing has other shape geometry than the English one" |
| Status | **OPEN** |

Owner: lane CL-CH. Its report `U/🗑️generated/closing/closing-CL-CH.md` does not exist yet. The sources do not state a cause or a re-run of this cell. The D53 rule (section labels must match across languages) is in place; whether it bears on this cell is not stated in the sources.

## 4. Proof sheets

`T/🖨️sheets/` holds 45 SVG files, flat, no subfolders.

| Category | contact | print 1:50 | print 1:100 | print 1:200 | print 1:500 | total |
|---|---:|---:|---:|---:|---:|---:|
| animals | 1 | 1 | 1 | 1 | 1 | 5 |
| appliances | 1 | 1 | 1 | 1 | 1 | 5 |
| components | 1 | 1 | 1 | 1 | 1 | 5 |
| furniture | 1 | 1 | 1 | 1 | 1 | 5 |
| humans | 1 | 1 | 1 | 1 | 1 | 5 |
| machinery | 1 | 1 | 1 | 1 | 1 | 5 |
| objects | 1 | 1 | 1 | 1 | 1 | 5 |
| vegetation | 1 | 1 | 1 | 1 | 1 | 5 |
| vehicles | 1 | 1 | 1 | 1 | 1 | 5 |
| **total** | **9** | **9** | **9** | **9** | **9** | **45** |

Naming: `contact-<category>.svg` and `print-<scale>-<category>.svg`. Summary log: `STEP proofs exit 0` in the closing run. The sheets were not opened for this draft; the same-instant, same-scale and same-ground checks are not recorded here.

## 5. Conventions and decisions (D1 to D70)

Status vocabulary, taken from the board and the handoff: "decided" (board entry); "landed" (the board or a worker report says so, not verified here); "open" (the board or a later source names it open). Where no source gives a landing status, the row says so.

| D | Decision (short) | Status |
|---|---|---|
| D1 | Side view = existing `elevation`; view names `elevation / front / plan` | decided; handedness by D6 |
| D2 | Unified 3D state contract (Vec3, BoneFrame, anchors.z, WorldForce and AppliedLoad fields, `momentNm` to `mzNm`) | decided; types landed (worker report); evaluate, worldForces and solver follow-ups open |
| D3 | Structural solver becomes 3D | decided (approved); 3D solver in structural report |
| D4 | Transport, emitters, condensation as Vec3; `directionRad` dropped; `windDirectionDeg` a bearing from +x to +z | decided; follow-up open |
| D5 | Every cast row requires `viewpoint`; `mirror` only with elevation | decided; landed (exports report); course Python re-run pending |
| D6 | Left-handed frame; elevation from -z, front from +x, plan from +y; Seitenansicht, Vorderansicht, Draufsicht | decided |
| D7 | Three-view layout: plan above elevation (shares x), front beside (shares y) | decided |
| D8 | Unavailable scales need a `cause`; symbol backlog (53 front, 7 plan on 2026-11-02) | decided; landed (worker report); 2026-11-23 state: 152 symbols with three views, no gap |
| D9 | `asset` alternative must be drawable and of the same kind; no chains | decided |
| D10 | `validRange` is a support range; coverage includes its ends to 1:1000 | decided; applied (1,764 range-end cells) |
| D11 | `timeS` absolute; `clipTimeS` the wrapped clip phase | decided |
| D12 | One environment evaluator; wind blended as a vector | decided |
| D13 | `heightM` is the physical height; paper extents in `boundsM` | decided; landed (exports report) |
| D14 | Every export, download and preview filename includes the viewpoint | decided; landed for CLI and preview (exports report) |
| D15 | Missing views never tolerated; no default viewpoint | decided; landing not reported |
| D16 | Revolve axis absolute rest-world | superseded by D24 |
| D17 | Asset at least 3 mm on paper must be drawable; sixteen bands to author | decided |
| D18 | `dimensions` is the physical envelope; tolerance max(2 %, 15 mm) | decided; used by the matrix and validator (see D50) |
| D19 | Course casts: mirror of asymmetric asset needs an allowlist reason; sized from real dimensions; plan rotation only as documented yaw | decided; course audit: sampled cases need a hand check (open) |
| D20 | Manifest fixes: three missing poses, three 1:500 bands flipped to authored | decided (approved) |
| D21 | `axisBone` is the shape's own bone or an ancestor | decided |
| D22 | Symbol semantics (true-extent, true-position, nominal) shown to the reader | decided; display status not reported |
| D23 | Moment vector as a two-headed arrow; left-hand rule; curl sense in legend | decided |
| D24 | Revolve axis local to its bone (supersedes D16) | decided; rigs pass reported finished (worker report, relayed) |
| D25 | `view-twin-missing` removed; `lateral-ambiguous` added | decided |
| D26 | Solids allowed in every view | decided |
| D27 | True-extent symbol with three views and print minimum satisfies D17 | decided; symbol print-minimum part reported not implemented |
| D28 | `lateral-ambiguous` threshold: flattening error above 0.2 mm on paper | decided |
| D29 | Vegetation crosswind: no exemption; all 13 trees reverted to along-wind sway | decided; outcome per board (worker report) |
| D30 | Nominal-scale cells reported apart from `validRange` end cells | decided; applied (range-end column) |
| D31 | Revolved solid declares bone-local axis (`u`, `v` or `w`) | decided; not landed per the last source naming it (2026-11-02) |
| D32 | `--projection first|third` required, no default | decided; landed (exports report) |
| D33 | 1:500 `representation-copied` band counts as FAIL until redrawn | decided |
| D34 | One D18 tolerance everywhere; reused view across bands is `derived-view` FAIL | decided; landing not reported |
| D35 | Thin parts leaving the drawing plane become solids with physical thickness | decided; landing not stated |
| D36 | `dimensions` is the single source of truth; manifest within 1 mm | decided; closed (four rows confirmed) |
| D37 | `member` solid for slender diagonals | decided; landed (rigs and contracts reports) |
| D38 | Playground placements in 3D; catalog friezes labelled as presentation layout | decided; landing not stated |
| D39 | View-neutral symbol ids; `*-plan` renamed, no aliases | decided; applied to machines (9 symbols), registry applies the rest |
| D40 | True orthographic, hidden-line removal, no dashed hidden lines; cut plan only as named section | decided |
| D41 | Two-tier sync: hard per-bone FAIL; group-drift reported | decided; refined by D48 |
| D42 | Optional linear taper toward bone tip | open (field name to be agreed) |
| D43 | Air-borne region may have no shape under two conditions | decided |
| D44 | Physical root yaw (`yawDeg`) rotates the evaluated state before projection | decided; rigs owns, viewer and course consume |
| D45 | One anchor per physical contact or support | decided; applied across batches |
| D46 | `lateralScale` for elliptic revolved cross-section | decided; rigs and contracts own it |
| D47 | Extruded solids declare `sweep: "v" \| "w"` | decided; migration in one pass (status not stated) |
| D48 | Hard per-bone tier only for the same shape id | decided; landed (landmarks.ts) |
| D49 | Band-copy and derived-view allowlists with physical reason | decided; implemented (`derivedFindings`) |
| D50 | Dimension and extent comparisons in whole micrometres | landed (board); for the dev's review |
| D51 | Derived-view allowlist may name bands | landed (board); for the dev's review |
| D52 | Heater case states per band what the body resolves | decided; test changed, not asset; for the dev's review |
| D53 | Section label identical in EN and DE (`section-label-language`) | landed (catalog validator and tests); for the dev's review |
| D54 | `loft` shape (seventh geometry) with `LoftSection` | decided; for the dev's review; `loft-shape` 29 of 29 (handoff) |
| D55 | Derived-view measure in whole millionths; a tie differs | decided; for the dev's review |
| D56 | `lateralScale` honoured; five assets lose the member at that scale | decided; for the dev's review; owners remove the member |
| D57 | Drawn width of a loft is its least dimension | landed (`loftExtents`) |
| D58 | Derived allowlist entries name their bands; `BAND_LIMITED` | decided; `steel-column` front merged for 200 and 500 only |
| D59 | Ink gap at most `TOUCH_MM` is a join | decided; for the dev's review |
| D60 | Text measured in Anta by default | decided; for the dev's review; no matrix cell changed |
| D61 | `withdraw` list narrows an allowlist entry | decided; used for three plants |
| D62 | 33 `model-bounds` band cases state per band what the body resolves | decided; `model-bounds` 82 of 82 (handoff) |
| D63 | Solids measured in the model frame | decided |
| D64 | Compressor mounts by role; street lamp reads 1:50 catches only | decided |
| D65 | `sagittalRoot` for limb roots in chain-state | decided; `chain-state` 20 of 20 (handoff) |
| D66 | Loft as depth twin of its bone | decided; `loft-depth` 5 cases; five assets move; owners decide |
| D67 | Paper box of lofts 15 mm shorter at 1:1000; left as is | decided; open follow-up: dimension check margin |
| D68 | Sync-bone rule kept; hand asset repaired | decided; 3.5 mm left, owner ML-5 (open) |
| D69 | `unionPolygons` loses no body; guard on 24 bead masses | decided; no engine change |
| D70 | Sub-pen hole in silhouette guard; pocket rule | decided; pocket rule live 2026-11-26 08:34 (handoff) |

## 6. For the dev's review

From the "For the dev's review" section of `U/🤝️handoff.md`:
- Rule decisions taken by the coordinator: D48, D49, excavator house radius 20 mm, sheet stack as one pack at 1:50, `norway-spruce` and `windmill-palm` fronts on the allowlist.
- Shared test edits: silhouette oracle (no 10 MB input limit; polygon reference on an integer grid with retries); plan-projection reference chain in 3D with one re-pinned shape (`small-reds.md`).
- Debt stated at closing: `bamboo-clump` passes the silhouette guard through raster-fitted squares; leaves of agave, hosta-clump and date-palm are slab stacks; several people pass exactly at the limit; `weeping-willow` file is 9.5 MB.
- Incidents in `requests.md`: the dependencies image did not mount (a leftover `semio-trim` volume, repaired by hand; a same-named scratch volume was deleted by mistake); four catalog files reverted to older content at 13:22 on 2026-11-24, writer unknown.

Also from the state log of `U/🤝️handoff.md` (2026-11-26 entry, for the dev):
- `person-stretching` 1:50 plan passes the guard only with the right sleeve 2 mm fuller; cause not found (ML-5).
- `sunbather-on-towel` front 1:100 at 0.9207 against 0.92 (MU).
- Four framework type errors in `🛂️descriptor-admission` reached through the site project (another lane's edit).
- `steel-column` allowlist and the D50 to D61 decisions (section 5).

## 7. Multiview suites

Source: `U/🗑️generated/closing/summary.txt`. Runs are labelled; the log is an append of several closing runs.

| Suite | Result | Run |
|---|---|---|
| 🔭️multiview-coverage | exit 0; 10 files; 42 passed, 0 failed, 0 skipped | 09:32 and rerun 17:27 (42 passed) |
| 🧊️three-views | exit 0; 6 files; 775 passed, 0 failed | 09:32 and rerun 17:27 (775 passed) |
| catalog-silhouette sil-a | exit 0; 379 files; 105 passed, 0 failed, 280 skipped | 12:31 |
| catalog-silhouette sil-b | exit 0; 379 files; 280 passed, 0 failed, 105 skipped | 12:31 (105 + 280 = 385 cases in the two shards) |
| catalog-silhouette (rerun line) | exit 0; 379 files; 11 passed, 0 failed, 374 skipped | 17:27; scope of this rerun not stated in the log |
| 📑️multiview-sheet | exit 0; 3 files; 27 passed | 09:32 |
| 🌐️thermal-multiview | exit 0; 7 files; 23 passed | 09:32 |
| 🎨️view-synchrony | exit 0; 3 files; 15 passed | 09:32 |
| 🎨️seven-modes | exit 0; 2 files; 6 passed | 09:32 |
| 🧊️lateral-spread | exit 0; 7 files; 27 passed | 09:32 |
| 🧊️axial-twist | exit 0; 5 files; 33 passed | 09:32 |
| 🏗️structure-projection | exit 0; 10 files; 54 passed | 09:32 |
| 🧍️front-projection / 🧍️plan-projection | exit 0; 9 and 13 passed (plan also 13 on rerun 17:27) | 09:32 |
| tsc-🧍️staffage | exit 0; 0 errors | 09:32 |
| tsc-⚛️react | exit 0; 0 errors | 09:32 |
| tsc-🌐️site | exit 0; 0 errors | 09:32 |

Superseded lines, not counted: the first `sil-b` (exit 143, killed, no JSON, 09:32 run) is replaced by the 12:31 pair above.

Check CLI: the 12:31 run recorded `STEP check exit 1`. The per-category check runs (`check-<category>`, 16:05 and 17:27) are all exit 0.

Outside the multiview scope, for the record: `🧊️model-catalog` is not green in the latest log (shards 00 to 11 exit 1, 5 to 11 failed each in the 19:16 chain). The earlier non-green lines of other suites have later rerun lines that are green.

## 8. Limits of this draft

- The matrix duration in the log (33,958 s from 18:45:37) and the file times (18:58) are not reconciled by the sources.
- The failing cell has no recorded cause and no re-run; it stays OPEN until CL-CH reports.
- The proof sheets were counted, not opened.
- D31 and several landing statements in section 5 rest on the last source that mentions them; they are not checked here.
- No test was run for this draft.
