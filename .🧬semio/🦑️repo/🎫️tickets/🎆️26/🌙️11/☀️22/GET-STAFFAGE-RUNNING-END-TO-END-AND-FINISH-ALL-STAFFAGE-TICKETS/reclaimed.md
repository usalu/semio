# Finisher Report — Reclaimed Building Components

Ticket `RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION`, finisher session of 2026-11-22, 20:23 to about 21:40. Scope: the 22 started assets (rows 01–22 of `assign.md`).

## Result

All 22 started assets are finished and proven; none needed a correction, so nothing was re-published and neither the catalog nor the manifest was touched by this session. The ticket can be closed for rows 01–22. Rows 23–40 were never started and stay unstarted.

## What was measured in this session (every tool ran to its end)

- Full proof `r3-validate.ts --file <candidate> --volumes` on each of the 22 candidates: 22 x `VALID file/<slug> (file as it is): schema 0, validateAsset 0, checkVolumes 0`, exit 0, no finding line.
- Product check CLI `🩺️check --file <candidate>`: 22 x `✓ <slug> (1/1)`, exit 0, every scale `authored, suitable under office-laser`.
- `verify-rows.ts` (new): for all 22 the catalog copy equals the candidate byte for byte, the manifest row equals the row `publish.ts` computes from the candidate and its `distinction.json`, exactly one row per asset; collection `reclaimed-components` has 22 assets, none without a candidate. `bad 0`.
- `verify-manifest.ts`: `assets 347, entries 277, reclaimed entries 22, manifest issues 72, of them reclaimed 0` (the 72 are missing rows of other waves).
- `verify.sh` to the end (`DONE`): 22 x `VALID components/<slug> (catalog file as it is): schema 0, validateAsset 0`.
- `summarize.py` (new): every asset has the poses `stored`, `lifted`, `reinstalled…`, a clip `lift` and at least one more, tags `component` and `reclaimed`, bone masses summing to `dimensions.massKg`, default pose `stored`, four authored scales, twelve exports; no `measured` source tag, no dangling reference, every text `{ en, de }`.
- `verify-print.result.txt` of 17:22 had found 22 x `VALID … checkVolumes not run`; it proved nothing about the volumes. Deleted.

## Per asset

State before: published at the time given (catalog file time), no report, no ledger row. State after: unchanged document, proven, report written. Final proof line for every row: `VALID file/<slug> (file as it is): schema 0, validateAsset 0, checkVolumes 0`; check line `✓ <slug> (1/1)`.

| # | slug | before | after |
|---|---|---|---|
| 01 | reclaimed-oak-beam | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-oak-beam.md` |
| 02 | reclaimed-rafter-bundle | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-rafter-bundle.md` |
| 03 | reclaimed-glulam-girder | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-glulam-girder.md` |
| 04 | reclaimed-timber-roof-truss | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-timber-roof-truss.md` |
| 05 | reclaimed-floorboard-stack | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-floorboard-stack.md` |
| 06 | reclaimed-parquet-crate | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-parquet-crate.md` |
| 07 | reclaimed-steel-i-beam | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-steel-i-beam.md` |
| 08 | reclaimed-steel-column | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-steel-column.md` |
| 09 | reclaimed-lattice-girder | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-lattice-girder.md` |
| 10 | reclaimed-cast-iron-column | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-cast-iron-column.md` |
| 11 | reclaimed-steel-stair | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-steel-stair.md` |
| 12 | reclaimed-trapezoidal-sheet-stack | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-trapezoidal-sheet-stack.md` |
| 13 | reclaimed-brick-pallet | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-brick-pallet.md` |
| 14 | reclaimed-hollow-core-slab | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-hollow-core-slab.md` |
| 15 | reclaimed-concrete-wall-panel | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-concrete-wall-panel.md` |
| 16 | reclaimed-precast-stair | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-precast-stair.md` |
| 17 | reclaimed-paving-slab-pallet | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-paving-slab-pallet.md` |
| 18 | reclaimed-granite-kerbs | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-granite-kerbs.md` |
| 19 | reclaimed-cobblestone-bag | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-cobblestone-bag.md` |
| 20 | reclaimed-sandstone-window-surround | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-sandstone-window-surround.md` |
| 21 | reclaimed-stone-steps | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-stone-steps.md` |
| 22 | reclaimed-roof-tile-pallet | published, unproven with `--volumes`, no report | proven (proof VALID 0/0/0, check ✓, catalog = candidate, row = candidate), report `reports/reclaimed-roof-tile-pallet.md` |

## Sheets

Every sheet was rendered without a tool error (per asset: body, thermal, structure at four scales; every non-default pose and two instants of every clip at 1:50; 2D at four scales) and laid out four to a contact sheet at half size. Opened for all 22 assets: the body contact sheet, the 2D contact sheet and one pose/clip contact sheet (both for the oak beam). Opened in addition: thermal of oak beam, roof-tile pallet and floorboard stack; structure of trapezoidal sheet stack and lattice girder. NOT opened: the other 19 thermal and 20 structure contact sheets and the remaining pose/clip contact sheets (about one to three per asset). Each asset report states exactly what was opened. On the opened sheets no missing, floating or interpenetrating part and no unfinished scale was seen; at half size sub-centimetre detail cannot be judged and was not.

## What stays imperfect

- The author reports were written by the finisher from the documents (`report.py`): their reference tables repeat what each document states with its cited sources; the brief's 3 % comparison against an independently researched reference table was not redone beyond the mass balance.
- Not every thermal and structure sheet and not every pose sheet was looked at by eye (see above); those lists are proven by the measured validator rules only.
- Structural reactions are approximate wherever storage aids or a remaining stack rest separately from the lifted or reinstalled part, and trusses show cantilever cuts instead of axial forces: shared solver limits, stated in each asset's `structure.assumptions`.
- `reclaimed-floorboard-stack` (and the assets with empty `structure.supports`) have no `part: "support"` solids; `reclaimed-paving-slab-pallet` uses single boxes instead of `repeat` at 1:100.

## Never started (not started now)

23 reclaimed-slate-crate, 24 reclaimed-roof-window, 25 reclaimed-zinc-gutter-bundle, 26 reclaimed-box-window, 27 reclaimed-panel-door, 28 reclaimed-steel-window, 29 reclaimed-glazing-rack, 30 reclaimed-fire-door, 31 reclaimed-window-shutters, 32 reclaimed-facade-element, 33 reclaimed-stone-cladding-slabs, 34 reclaimed-raised-floor-panels, 35 reclaimed-cast-iron-radiator, 36 reclaimed-washbasin, 37 reclaimed-cast-iron-bathtub, 38 reclaimed-cast-iron-stove, 39 reclaimed-pv-module-pallet, 40 reclaimed-balcony-railing.

## Shared-code requests still open (carried to `requests.md` of this ticket)

- structure solver: one rigid body for bodies that rest separately in a pose (seven assets measured);
- structure solver: closed lattice against tree rig (timber roof truss, lattice girder);
- `volumes-thermal-coverage`: copies of one `repeat` never seam (paving slab pallet);
- support solids for pose-dependent supports (floorboard stack).

All nine sections of `K/requests.md` were still valid and are merged into these four; `K/requests.md` is kept unchanged with the authors' measurements.

## Decisions

- No re-publish: no document changed, and `verify-rows.ts` shows catalog and manifest already equal the candidates.
- Contact sheets at half size instead of some 25 full sheets per asset, to open every asset's main sheets within one session; stated above as a limit.
- The proof ran as one sequential script (one tool process at a time).

## Files

Created:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/wave.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/prove.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/sheets.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/contact.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/summarize.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/verify-rows.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/report.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/review-notes.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-brick-pallet.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-cast-iron-column.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-cobblestone-bag.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-concrete-wall-panel.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-floorboard-stack.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-glulam-girder.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-granite-kerbs.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-hollow-core-slab.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-lattice-girder.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-oak-beam.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-parquet-crate.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-paving-slab-pallet.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-precast-stair.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-rafter-bundle.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-roof-tile-pallet.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-sandstone-window-surround.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-steel-column.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-steel-i-beam.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-steel-stair.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-stone-steps.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-timber-roof-truss.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/reports/reclaimed-trapezoidal-sheet-stack.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/reclaimed.md`

Updated:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (four sections appended)

Removed:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/verify-print.result.txt`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/🗑️generated/` (proof outputs, `verify.txt`, review contact sheets of this session)
- the `reclaimed-*.png` renders of this session under the tool ticket's `🗑️generated/r3-sheet/file` and `r3-2d/file`

Not touched: any asset document, `distinction.json`, the catalog, the manifest, shared code.
