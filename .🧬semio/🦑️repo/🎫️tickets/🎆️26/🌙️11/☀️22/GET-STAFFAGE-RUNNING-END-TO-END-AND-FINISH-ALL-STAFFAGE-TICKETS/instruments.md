# 🎻️ Finisher Report: Used Musical Instruments

Ticket `USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION`, finisher session 2026-11-22 20:25–21:30.

## Result

- 21 of the 40 rows of `assign.md` were started; all 21 are finished and proven now. No asset failed a proof and no asset showed an unfinished or broken state on its sheets, so no candidate was changed and nothing was re-published: the catalog and the manifest are untouched by this session.
- The other 19 rows were never started and stay unstarted (dev's decision of 16:29).
- The 21 author reports did not exist (`reports/` was missing); they were written from measurements of this session. The ledger `wave.md` was rebuilt as one row per asset.

## What was run (each to the end, one tool process at a time)

- `r3-validate.ts --file <candidate> --volumes` for all 21: 21 × `VALID … schema 0, validateAsset 0, checkVolumes 0`.
- Product check CLI `🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file <candidate>` for all 21: 21 × `✓`.
- `prove-row.ts` (new, mirrors the row derivation of `publish.ts`), run at 20:26 and 21:20: 21 × catalog copy byte-equal to the candidate, manifest row equal, member of the collection `used-instruments`.
- `r3-validate.ts <category>/<slug> --catalog-only --no-print` for all 21: 21 × `VALID`.
- `measure.ts` (new): bounding boxes of the 1:50 body solids in every non-key pose and solid counts per list and scale; the default pose is within 0.6 % of `dimensions` for every asset (brief: 3 %); bone masses sum to `dimensions.massKg` for every asset.
- Sheets: per asset one 3D contact sheet of four `r3-sheet.ts` sheets (1:50 body in `stored`, `carried` (`rolled` for the xylophone) and `played`, 1:200 body) and one 2D contact sheet of the `r3-2d.ts` drawings of every authored scale. All 42 contact sheets were opened.

## Per asset

Final proof line of every row below: `VALID file/<slug> (file as it is): schema 0, validateAsset 0, checkVolumes 0` and `✓ <slug> (1/1)`; `prove-row.ts`: `OK` (copy equal, row equal, collection member).

| # | slug | category | before (20:23) | after | changed |
|---|---|---|---|---|---|
| 01 | `used-baby-grand-piano` | objects | published, no report | finished and proven, report written | report only |
| 02 | `used-classical-guitar` | objects | published, no report | finished and proven, report written | report only |
| 03 | `used-steel-string-acoustic-guitar` | objects | published, no report | finished and proven, report written | report only |
| 04 | `used-violin` | objects | published, no report | finished and proven, report written | report only |
| 05 | `used-viola` | objects | published, no report | finished and proven, report written | report only |
| 06 | `used-cello` | objects | published, no report | finished and proven, report written | report only |
| 07 | `used-double-bass` | objects | published, no report | finished and proven, report written | report only |
| 08 | `used-concert-harp` | objects | published, no report | finished and proven, report written | report only |
| 09 | `used-electric-guitar-and-combo` | appliances | published, no report | finished and proven, report written | report only |
| 10 | `used-electric-bass-and-cabinet` | appliances | published, no report | finished and proven, report written | report only |
| 11 | `used-stage-piano` | appliances | published, no report | finished and proven, report written | report only |
| 12 | `used-electronic-organ` | appliances | published, no report | finished and proven, report written | report only |
| 13 | `used-harmonium` | objects | published, no report | finished and proven, report written | report only |
| 14 | `used-accordion` | objects | published, no report | finished and proven, report written | report only |
| 15 | `used-drum-kit` | objects | published, ledger said `running`, no report | finished and proven, report written | report only |
| 16 | `used-electronic-drum-kit` | appliances | published, no report | finished and proven, report written | report only |
| 17 | `used-timpani-pair` | objects | published, no report | finished and proven, report written | report only |
| 18 | `used-xylophone` | objects | published, no report | finished and proven, report written | report only |
| 19 | `used-marching-snare-and-bass-drum` | objects | published, no report | finished and proven, report written | report only |
| 20 | `used-djembe-and-cajon` | objects | published, no report | finished and proven, report written | report only |
| 21 | `used-trumpet` | objects | published, no report | finished and proven, report written | report only |

## What stays imperfect

- The review was done on contact sheets at half resolution (1200 × 800 px per sheet). Details below about 5 mm of model size, the thermal and structure solid lists and the clip instants were not inspected by eye; they rest on the measured rules of the full proof and on the check CLI, which walks every pose and clip sample.
- The reports are the finisher's: reference tables are the references and `provenance.notes` the authors wrote into the documents, not an independent literature check. Model-vs-reference is measured against the document's own `dimensions`, not against a second source.
- `used-violin`: the shoulder strap is two rigid straight bands (a flat V on the floor 0.6 m beside the case when stored, a taut triangle when carried); the schema has no flexible band.
- `used-djembe-and-cajon`: the author script is two minutes newer than the document it wrote (16:23:02 against 16:21:09) and was not re-run; a last script edit that never reached the document cannot be excluded. The document passes everything as it is.
- `used-drum-kit`: listed as `running` at the cut-off; nothing unfinished was found, but the author's own closing look at the sheets is undocumented.
- `used-classical-guitar` (1.88 m) and `used-double-bass` (3.21 m) state the width of the whole stored group in `dimensions.widthM` and in the manifest row, not the instrument's own size.
- Coarse-scale dimension texts differ by 10–30 mm from the 1:50 ones on `used-baby-grand-piano`, `used-electric-bass-and-cabinet`, `used-harmonium`, `used-violin`, `used-xylophone` (generalised outlines).
- The catalog test suites and the site build were not run (reserved for the umbrella coordinator).

## Never started (19)

`used-trombone`, `used-french-horn`, `used-tuba`, `used-sousaphone`, `used-saxophone`, `used-clarinet`, `used-flute-and-piccolo`, `used-bassoon`, `used-recorder-set`, `used-alphorn`, `used-bagpipes`, `used-hammered-dulcimer`, `used-zither`, `used-music-stands-and-chairs`, `used-pa-system`, `used-mixing-console`, `used-turntable-dj-setup`, `used-metronome-and-tuner-kit`, `used-instrument-flight-cases`.

## Shared-code requests still open

- `USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION — shapeDepthIn: paint order of sweep "v" / "u" solid shapes in a rolled pose` in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (carried over from the ticket; code re-read and unchanged, not re-measured; worked around inside `used-trumpet`).

## Files

Created:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/reports/<slug>.md` for the 21 slugs of the table above
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/prove.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/prove-row.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/measure.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/sheets.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/make-reports.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/review.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/instruments.md`

Updated:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/wave.md` (rebuilt)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (one section appended)

Removed:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/🗑️generated` (raw proof and check output, measurements, contact sheets of this session)
- the finisher's own sheets under the tool ticket's `🗑️generated/r3-sheet/file/used-*` and `🗑️generated/r3-2d/file/used-*` (21 slugs of this ticket)

No candidate, no catalog file, no manifest row and no shared code was changed.
