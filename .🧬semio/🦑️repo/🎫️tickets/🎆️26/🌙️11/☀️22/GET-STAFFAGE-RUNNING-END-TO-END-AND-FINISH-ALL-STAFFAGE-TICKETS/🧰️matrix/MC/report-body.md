@STATUS@

# 🚜️ Multiview Batch MC: 10 Components, 4 Machines, 6 Vehicles, and Three Leftovers (2026-11-25)

Fixer report, one text for the predecessor's work (2026-11-25 00:45 to 02:43, stopped at 06:36) and the takeover (09:00 to @END@). Batch `🧰️matrix/batches/MC.tsv`, briefs `📋️multiview-brief.md` and `📋️takeover-brief.md`, paths `S`, `T`, `U` as there. Tools: `🧰️matrix/MC/`. Every number below was measured in the takeover session on the catalog documents as they are, unless it says "before".

## 1. Result

- **Batch: @CLEAN20@ of 20 assets clean with all proofs**: fixer's gate over every subject of the matrix 6,742 hard findings before, 0 after; authors' gate `T/🔎️scan.ts` 457 before (0 of 20 clean), 0 after (20 clean); matrix `T/📊️matrix.ts --assets <id>`: @MATRIX_LINE@.
- **Leftovers of the coordinator** (in no unfinished batch: MB-1 and MB-2 ended on 2026-11-24): `storage-crate-stack` repaired in the asset, 28 `language-geometry` cells to 0, matrix 252 ok; `glulam-beam` repaired in the asset, 7 `derived-view-in-band` cells to 0, matrix 252 ok; `steel-column` cannot be drawn so that the rule holds at 1:200 and 1:500 (20 mm against 90 mm), 21 cells stay, the matrix question is in `requests.md` with the numbers (section 4).
- Open: nothing in the batch. Outside it: `steel-column` 21 cells (allowlist line asked). Notes for the coordinator in sections 6 and 7.
- The manifest, the fixtures, the suite and every shared module were not edited. Declared dimensions: none changed.

## 2. Takeover: the state found and what was done

- Found at 09:00: all 20 catalog documents as written at 02:20 (18 byte-identical to the trial copies; `reclaimed-precast-stair` and `used-wall-hung-gas-boiler` were corrected after their trial and differ from it), proofs of 02:21 to 02:40 complete on those files, no matrix: the request of 7,000 MB waited 14,092 s in the queue and ended with "memory waited out". No document was half done; none was put back.
- Proven again (section 5), then the matrix of each asset, one process at a time. Reservations: 1,000 to 1,200 MB for scans, check CLI and matrix, 2,500 MB for the suites (3,200 MB for the last five, see section 7). Every run went through `q.sh` (`repo capacity run`) and inside `capped.sh`, which samples the resident memory of the whole process tree, logs the peak and stops the tree 50 MB below its reservation.
- The chain was proven without writing: `idem.sh` starts from the backup of the untouched document, runs writer and apply on their own result until nothing changes, and compares with the catalog document: **for all 20 one pass from the backup gives the catalog document byte for byte and a further pass leaves it as it is.** (The log line "THE SECOND RUN WROTE OTHER OPS" of 02:20 is harmless: a writer reads the document, so after its repair it writes ops that state the repaired shapes; the file is the same.)
- Sheets: section 5.

## 3. Per asset: findings before and after, matrix, what was changed

Hard findings of the fixer's gate by code before (untouched documents) and after; authors' gate in brackets. Matrix: cells by status, seconds, peak of the process tree. Changes are the predecessor's (`make/<id>.py`, ops in `ops/<id>.json`, applied by `apply.ts`); the takeover changed no document of the batch.

| Asset | Before | After | Matrix | Change (`make/<id>.py`) |
|---|---|---|---|---|
@ROWS@

## 4. The three leftovers

Measured with `derived.ts` (the measure of `T/🪞️derived.ts` without the threshold, with the farthest point of either view), its Python twin `derived.py` (rect and poly views; same numbers), `left-candidates.py` (candidate drawings as trial copies) and `langgeo.ts` (the `language-geometry` comparison of the matrix laid open).

- **`storage-crate-stack`** (objects; 28 `language-geometry` cells in MB-2's matrix): cause in the asset. Its front section was named "Longitudinal crate section" / "Kistenlängsschnitt" where the 19 other assets with a section name it by letters. The side elevation of 1:50 and 1:100 writes that name beside the arrows of the cut line, so the English scene was 4.18 mm wider on paper and every shape lay 4.18 mm further right than in the German one (42 of 252 scale, pose, view, mode renders differed, all in the side elevation of 1:50 and 1:100); the caption read "Section Longitudinal crate section". Repair (`make/storage-crate-stack.py`, 4 ops): label `A-A` in both languages in the four bands. After: 0 renders differ; gate 0, scan clean, validate 0, prints, check `✓`, six suites as far as they name it pass (silhouette 1, three-views 2, physics 2, validation 10; no run on the untouched document, the label is no geometry); **matrix 252 ok**. Sheet 1:50 opened: cut line A, caption "Section A-A".
- **`glulam-beam`** (components; plan against side elevation at 1:500: 0.02875 = 230 mm, limit 0.03 = 240 mm of the 8 m span; 7 cells): the side elevation was one rectangle 8.00 x 0.62 m with the two 20 mm bearing plates merged into the bar, the plan 8.00 x 0.16 m. Repair (`make/glulam-beam.py`, 2 ops): the side elevation is the true outline of the body, the 0.60 m beam with the two bearing plates of 120 x 20 mm as steps under its ends, overall 0.62 m as before; the thin-element text of the band says so. Measured after: 0.03243 = 259 mm, not flagged; front against side 0.09278; every band prints at key and range ends; gate 0, scan clean, validate 0, check `✓`; silhouette 1, three-views 2, physics 2, validation 10 passed before and after; **matrix 252 ok** (11 s). Sheet 1:500 opened before and after: the same bar. Plainly: the steps are 0.24 x 0.04 mm on paper, below the 0.13 mm pen; the drawing is truer, the print is the same, and what the measure sees is the 120 mm length of the plate (section 6).
- **`steel-column`** (components; front against side elevation: 1:100 0.02522 = 75.7 mm, 1:200 and 1:500 0.00667 = 20 mm, limit 90 mm; 21 cells): **cannot be drawn so that the rule holds; not changed.** An HEB 200 is 200 mm in both views; base plate 300 x 260 mm and head plate 240 x 220 mm differ by 20 and 10 mm per side. The one part that tells the views apart is the gusset edge at the middle of the flange face, up to 0.11 m, and it counts for 0.11 m minus the drawn plate thickness. 1:500: nothing to add, 20 mm. 1:200: the gusset edge of the front is 0.1 mm (below the 0.2 mm feature minimum); with gussets drawn in the side elevation 0.02007 = 60.2 mm. 1:100: plates at 30 mm give 80.6 mm and make the front a band copy of the 1:50 front (0.00422); plates at their true 20 mm give exactly 0.03000 = 90.0 mm, a tie that passes by rounding (1:50 passes with 90.4 mm). The request `## multiview MC — steel-column front …` in `requests.md` carries the numbers and the allowlist line with its reason in both languages.

## 5. Proof

Each run to the end in the takeover session on the catalog documents (logs were in `🗑️generated/matrix/MC`).

| Check | Result |
|---|---|
| Fixer's gate, all subjects (`../MR/gate.ts`) | 0 hard findings for all 20 (before 6,742), 0 for the two repaired leftovers |
| `T/🔎️scan.ts --language en` | 20 clean, 0 hard findings (before 457, 0 clean); `storage-crate-stack` and `glulam-beam` clean |
| `validateAsset` | 0 issues for all 22 |
| check CLI `--asset <id>` | `✓` for all 22 |
| every authored scale prints, key and both range ends | 21 of 22; `forklift` 1:20 (gap `cyl-seat-part` / `pivot-pin` 0.13 against 0.15 mm) is refused exactly as before the batch |
| matrix per asset | section 3 and 4: @MATRIX_LINE@; `storage-crate-stack` and `glulam-beam` 252 ok each |
| `🧊️catalog-silhouette` | 20 passed (before 20); the two leftovers 1 each |
| `🧊️three-views`, `🌡️catalog-physics` | 40 and 40 passed (before 40 and 40) |
| `✅️asset-validation` (ids, manifest cases, sample figure) | 10 passed (before 10) |
| `🫁️breathing-catalog`, `⚡️asset-power`, `🧊️thermal-scenarios`, `🎨️thermal-legend` | no case matches the name filter (290, 24, 14, 14 skipped), before and after |
| `✂️section-views`, `🌐️thermal-multiview`, `🏗️structure-projection`, `📑️multiview-sheet`, `🧍️plan-projection`, `🧊️model-roundness` | 2, 3, 3, 2, 2, 1 passed, same as before |
| `🧊️model-bounds`, `🧊️model-skeleton` (name filter) | 3 passed 1 failed and 1 passed 1 failed, before AND after, the same two tests ("has an open airflow passage through each generator intake at 1:500", "laces both bicycle wheels through open nipple seats"); they failed on the untouched documents (01:50), not from this batch |
| every test line, untouched documents against now (`results.py before now`) | no test changed its result in any of the 16 suites |
| chain (`idem.sh`) | 20 of 20: one pass from the backup gives the catalog document, a further pass changes nothing; the two leftover writers: second run "same ops, file unchanged" |
| silhouette guard values (`../MR/measure.ts`) | the predecessor's run of 02:30 on the same files: OK for all 20; changed values: stair plan 5.6 to 3.8 mm (1:50), 7.5 to 3.8 mm (1:100), front 1:200 5.5 to 0.0 mm; towel radiator plan 5.6 to 3.8 mm, 7.0 to 3.5 mm; generator front 1:100 18.8 to 15.9 mm. Not repeated today: the run was stopped by the cap at 956 MB of 1,000 after 14 assets; the real suite above is the proof |

Sheets (product drawings of `r3-2d`, side, front and plan, line and solid): one sheet per changed scale of every asset whose drawing changed, in a pose that turns the changed parts (`sheets.txt`, 45 sheets), rendered from the catalog and from the backups, plus the two leftovers. **Opened and looked at: 34** (city-bus 50 and 100; city-car 50; diesel-generator 50 and 100; forklift 50 and 100; motor-scooter 50; column 50 reinstalled and lifted, 100 slung, 500 lifted; stair 50 stored and lifted, 100 reinstalled; sheet stack 50 lifted and slings-tensioned, 200; heat pump 500; oil burner 50 reinstalled and lifted, 100 lifted; pellet boiler 50, 100; towel radiator 50 upright and reinstalled, 200; gas boiler 50, 100, 200; vintage car 50 parked and braking; crate stack 50; glulam beam 500). Nothing wrong or uglier was found; every view of a turned part agrees with the side elevation. **Rendered, not opened: 17** (city-bus 200, 500; city-car 100, 200, 500; forklift 200; column 200; stair 200, 500; sheet stack 100, 500; oil burner 200; pellet boiler 200, 500; vintage car 100, 200, 500); for those the evidence is gate, scan, matrix and the silhouette suite.

## 6. Decisions worth a second look

@DECISIONS@
- **`glulam-beam` 1:500** (takeover): the rule now holds through a true but sub-printable step (0.04 mm). If the coordinator prefers the merged bar, the alternative is an allowlist line for the plan; `make/glulam-beam.py` states both shapes.
- **`reclaimed-precast-stair`, front of the lifted and reinstalled flight**: a plain face under the nib without riser lines; seen from ahead the flight shows its soffit side, so this is what the body gives, but it reads bare.

## 7. Open, and notes

- Batch: no hard finding, no `fail` cell.
- `steel-column`: 21 `derived-view-in-band` cells until the allowlist line or another measure (request filed).
- `used-towel-radiator`: small round parts and surface marks on the radiator bone stay flat sheets, 35 shapes, mostly at 1:50 (the radiator turns by 10 degrees only; parts ride up to about 3 mm off); no finding.
- `reclaimed-precast-stair`: the lifting sockets are discs parallel to the waist slab, 6 mm narrow on a 40 mm socket once the flight stands at its pitch; no finding.
- Reported tier (`group-drift`) rose for `forklift` (75 to 109), `city-bus` (29 to 42), `motor-scooter` (0 to 22) with the wheel rebinding (section 6); not a failure.
- `forklift` does not print at 1:20, as before the batch. The two failing tests of `model-bounds` and `model-skeleton` are older than the batch and were not worked on.
- Memory, measured (`capped.sh`, sum of the resident memory of the process tree): matrix of one asset @PEAKS@; check CLI and scans below 1,000 MB; `../MR/measure.ts` above 950 MB; the vitest chain of ten suites reached 2,451 MB once (two of my grants ran side by side then) and 1,427 to 2,220 MB otherwise.
- Sheets not opened: 17, all at 1:100 or coarser (section 5).

## 8. Files

Updated, catalog documents (repo-relative under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`): `🧱️components/<id>/🔣️.json` (reclaimed-cast-iron-column, reclaimed-precast-stair, reclaimed-trapezoidal-sheet-stack, sectional-garage-door, sliding-door, used-air-water-heat-pump-monobloc, used-forced-draught-oil-burner, used-pellet-boiler, used-towel-radiator, used-wall-hung-gas-boiler, and in the takeover glulam-beam), `🏗️machinery/<id>/🔣️.json` (diesel-generator, evaporative-humidifier, forklift, scroll-compressor), `🚜️vehicles/<id>/🔣️.json` (cargo-bike, city-bicycle, city-bus, city-car, motor-scooter, vintage-car-1929), and in the takeover `🪑️objects/storage-crate-stack/🔣️.json`.

Under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`:

- Updated: `multiview-MC.md` (this report), `requests.md` (one section appended: `## multiview MC — steel-column front …`).
- Predecessor, kept: `🧰️matrix/MC/` with `make/<id>.py` and `ops/<id>.json` (20 each), `mc.py`, `apply.ts`, `files.ts`, `validate.ts`, `print.ts`, `probe.ts`, `dump.py`, `rig.py`, `boxes.ts`, `prints.ts`, `flat.py`, `survey.py`, `why.py`, `restdiff.py`, `gate-table.py`, `env.sh`, `chain.sh`, `measure.sh`, `try.sh`, `write.sh`, `prove.sh`, `suites.sh`, `look.sh`, `before/gate.txt`, `before/scan.json`, `after/gate.txt`, `after/scan.json`.
- Takeover, updated in `🧰️matrix/MC/`: `q.sh` (runs inside `capped.sh`), `matrix.sh` (peak and cap per asset), `lib.py` (`MC_SOURCE`), `results.py` (a missing log is named), `report.py` (rewritten).
- Takeover, created in `🧰️matrix/MC/`: `capped.sh`, `idem.sh`, `small.sh`, `check.sh`, `sheets.sh`, `sheets.txt`, `rest.sh`, `crate.sh`, `beam.sh`, `left.sh`, `left-after.sh`, `left-candidates.py`, `derived.ts`, `derived.py`, `langgeo.ts`, `matrix-table.py`, `make/storage-crate-stack.py`, `make/glulam-beam.py`, `ops/storage-crate-stack.json`, `ops/glulam-beam.json`, `report-body.md`, `report-rows.md`, `report-decisions.md`, `matrix.txt`, `peaks.txt`, `idem.txt` (the matrix lines, the measured memory peaks and the chain proof of this session).
- Removed: `🧰️matrix/MC/report-head.md`, `report-tail.md` (replaced by `report-body.md`); @REMOVED@
