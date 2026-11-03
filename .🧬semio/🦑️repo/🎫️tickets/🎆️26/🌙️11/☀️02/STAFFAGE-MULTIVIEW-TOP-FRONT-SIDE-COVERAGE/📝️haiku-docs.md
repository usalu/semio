# 📝️ haiku-docs, task 1 report (relayed by the coordinator)

Deliverables:
- `📖️view-documentation-inventory.md`
- `📦️handoff.md` (a skeleton)

Findings:
1. **Handedness.** x forward, y up, z left (+) is a left-handed frame. The brief claimed right-handed, which was wrong; it is corrected as D6. The README gives only "x right, y up" and no viewing direction.
2. **"Elevation" means the side view.** The schema, the cast enum and the English prose all use it that way. The `Viewpoint` description also calls the front a "front elevation".
3. **"View" is overloaded.** It names both the 7 render modes and the 3 projections. The German "Ansicht" collides between the two.
4. **German plan wording is split.** The i18n strings and captions say Draufsicht; validator messages and the `front-projection` test say Grundriss.
5. **A third vocabulary.** The uncommitted `🔭️viewpoints/🟦️.ts` uses Seite/Vorne/Oben and "All three"/"Alle drei", which match neither the i18n strings nor the captions.
6. **Side as a section is contradictory.** `Viewpoint` says "side or section", but `sections` admits only front and plan.
7. **Symbol geometry gap.** `SymbolRepresentation.geometry` requires only `elevation`. The validator's "elevation or plan" branch is unreachable.
8. **Silent defaults.** The export CLI and the cast exporter fall back to `elevation`.
9. **Raw ids in user text.** Raw ids such as `dark-line` appear in `detail.export.lede` and in the sheet, PDF and SVG metadata.

Uncertain and unverified: whether the README `heightM` sentence holds for the front and plan views.

## Task 2: terminology proposal (docs only, 2026-11-02)

Deliverable: `📖️terminology.md` in the ticket folder. It is the proposed single en/de vocabulary and is not yet approved.

- Rules: side elevation / Seitenansicht (`elevation`), front elevation / Vorderansicht (`front`), plan (top view) / Draufsicht (`plan`); three views / Drei Ansichten; the seven modes are representation / Darstellung, never Ansicht; Grundriss only for a cut plan section ("Plan section B-B" / "Grundriss-Schnitt B-B"); identifiers stay.
- Deviations listed with file:line, current text and replacement: i18n en and de (E1 to E29, D1 to D26), `🔭️viewpoints` (V1 to V14), captions (C1 to C11), schema and twin descriptions (S1 to S12), validator messages (M1 to M9), other modules (O1 to O8), tests and fixtures (T1 to T11), catalog prose counts, README (R-a to R-i), export prose.
- Open questions: OQ1 front cut label, OQ3 identifier renames, OQ4 "Focus view", OQ5 compare wording. OQ2 is closed by D6.
- Owners apply: viewer (i18n, `🔭️viewpoints`, export panel, downloads), contracts (validator, schema descriptions, captions), exports (SVG, PDF prose), structural (line 530), validation (tests and fixtures), category workers (catalog prose), coordinator (README).
- Noted during the read: the depiction caption omits the viewpoint for the side view (C8), and the `🔭️viewpoints` WORDS block already uses the new names while VIEWPOINT_NAMES and MISSING still do not.
- Not run: no test or check was executed. The check patterns in section 5 of the terminology file are for the owners and are not a gate.

## Task 3: README terminology items R-a to R-i (2026-11-02)

Approved decisions applied: OQ1 front cut is "Section A-A" / "Schnitt A-A"; OQ3 identifiers stay; OQ4 and OQ5 go to the viewer.

File: `🎓️teaching/🏛️architecture/🧍️staffage/README.md`. Edited with Edit only, reread before each edit, minimal changes:
- R-a intro: "seven representations".
- R-b: "with another representation is refused".
- R-c: "(thermal representation only)".
- R-d: front and plan viewpoints named with their labels; "the three views" sheet and "the viewer always offers the three views".
- R-e: "like a side-elevation shape"; "the three views are right at every pose".
- R-f: "the side elevation (`elevation`) is its orthographic projection".
- R-g: "the side elevation, the front elevation and the plan agree"; "in the side elevation".
- R-h: "thermal, forces and moments representations".
- R-i: viewpoint row names the three viewpoints.

Not touched: the "Conventions" handedness line (sonnet-contracts rewrites it per D6), the uncommitted edits of others outside these lines.

Left unchanged because they are not in R-a to R-i (checked by grep, the lines were read): line 5 "every view" (generic wording for consumers), line 13 `depict(view, scale)` (identifier), line 76 "the front and plan views can read", line 98 "three-views test" (test name), lines 105 and 119 the cast field `view` (identifier, OQ3 keeps it), lines 121 and 122 "thermal view only" in the cast row (proposed: "thermal representation only"), line 128 "front or plan view" (proposed: "front or plan viewpoint"). Coordinator to decide whether these join the terminology rules.

## Task 4: README consumer-audit items (2026-11-02)

Coordinator decisions applied, Edit only, reread before each edit; the Conventions handedness line was not touched.
- Lines 121 and 122: "thermal representation only" (cast row `environment` xor `timeline` and `thermalSpanK`).
- Line 128: "front or plan viewpoint".
- Line 5 ("every view") and the "three-views" test name were left as they are, per the decision.
- Line 27: layout path is now `🗂️catalog/<category>/<slug>/🔣️.json`, "the handcrafted asset `<slug>` of one category".
- Line 120: `viewpoint` is required with no default (`elevation`, `front` or `plan`); `mirror` is allowed only with `elevation`.
- Line 148: `staffage_height(cast)` returns the physical figure height in metres for every viewpoint (landing in the exporter, D13), never the air layer of a transport cast.

Not checked: the Course casts sentence "`heightM` always measures the figure" and the cast `mirror` row were not edited; the exporter behaviour (D13) is not verified here.

Not run: no test or check was executed.

## Task 5: handoff "Conventions and schema changes" section (2026-11-02)

Filled in `📦️handoff.md`, that section only, with Edit. Each item is marked "decided", "landed (per worker report)" or both. Nothing is marked verified: no test result is cited in the sources.

Sources read: `🔗️interfaces.md` (draft 1), `🗂️work-board.md` (decisions D1 to D8). `📐️conventions.md` was not present in the ticket folder when read, so the frame text comes from interfaces §2 and D6.

Gaps and inconsistencies found while filling it:
- D9, D10, D11 and D17 are not in the decisions log, and D13 is cited elsewhere but is not there either. The unavailable-rules and time-model items are taken from interfaces §3 to §5 instead; their wording could not be checked.
- D1 says the frame is "right-handed". D6 corrects it to left-handed. The handoff uses D6.
- The cast `viewpoint` requirement (D5) is decided but not landed: the registry report says 88 of 94 casts lack a viewpoint.
- The validator codes and the resolver API in interfaces §4 and §5 are described as decided, not as landed.

Not run: no test or check was executed.

## Task 6: refresh of the handoff conventions section (2026-11-02)

Refreshed the "Conventions and schema changes" section of `📦️handoff.md` with Edit, after the work board gained D9 to D17.
- D1 now points to D6 (handedness and viewing side). The stale "right-handed" note is gone.
- D11 (time model) and D12 (one environment evaluator) added; D9, D10 and D17 now cited from the log, not from interfaces.
- D5: landed per exports report (schema, exporter, README; cast-export suite 66 pass, 1 skip). The "not landed, 88 of 94" note is replaced: the consumer audit found all 94 rows carry `viewpoint`, so the registry count is stale. The course Python re-run is still pending.
- D13 and D14: landed per exports report (D14 for CLI and preview parts). D15 and D16 added as decided, no landing status.
- Resolver and validator codes: landed per contracts report, in `🔨️modules/🔭️multiview`; the failure backlog has been generated.
- `📐️conventions.md`: still not written; interfaces §2 remains the source.

Not verified: no test result is cited for any of these items, and I ran no test.

## Task 7: "Commands to run and inspect" section of the handoff (2026-11-02)

Filled with Edit, from the files only. Nothing was run.
- Repo targets: `@teaching/staffage` test and typecheck; `@teaching/staffage-react` test and typecheck; `@teaching/staffage-site` dev, dev-site, build, test and typecheck. Taken from the three `📋️project.json` files and the README "Run" section.
- Cast exporter command from the README "Course casts" section, plus the export CLI and preview CLI usage from their file headers.
- Course Python tests: `m9_tests.sh` from the previous ticket. It hard-codes `/c/git/semio`, which does not meet the cross-platform rule, so it needs replacing before reuse. Its energy package path and the guard's test module name are marked "to confirm".
- Placeholders marked TO CONFIRM: coverage matrix, proof sheet, multiview sheet export (`sheet-*` formats), and `bun T/🛠️scripts/sonnet-contracts/failures.ts <out.json>`. The failures script exists in the ticket scripts folder, but its arguments were not read.

Not run: no command in this section was executed.

## Task 8: README "Views" draft (2026-11-02)

Written as `📖️readme-views-draft.md` (proposal only; not in the README).
- Sources: work board D1, D5, D6, D7, D8, D9, D10, D11, D17, D18, D19, D23; the cameras file `🔨️modules/🎨️depiction/🔭️views/🟦️.ts` (camera table checked against D6); interfaces §2 to §5; the terminology file.
- Items marked (pending): the D22 text, the course guard allowlist and code audit, the course Python re-run, the structural legend text, the README "joint moments are CCW positive" reconciliation, the i18n and captions terminology, and the viewer sheet.
- Gaps: D22 does not appear in the work board or any ticket file; its content is not used. D21 and D23 are also absent from the work board; D23 is taken from `📝️haiku-analysis-structural-signs.md` ("Coordinator rule D23"). The German IDs D21–D23 in `📖️terminology.md` are local row labels, not decisions, so they collide with the coordinator's numbering; they should be renamed.
- The draft states the sheet rule as one `resolveViews` call per instant. It does not claim exports evaluate once.

Not run: no test or check was executed.

## Task 9: terminology row labels and README "Views" draft with D21–D27 (2026-11-02)

- `📖️terminology.md`: the German row labels are now T1–T26 (were D1–D26), so they no longer collide with decision numbers. The test rows of section 4.8 are now TS1–TS11 (were T1–T11), because the T prefix is taken. The note above the German table was rewritten. Task 2 and Task 3 log entries above use the old labels.
- `📖️readme-views-draft.md`: updated with D21–D27.
  - D22: symbol semantics keep three values; the fidelity table now has a semantics column; semantics are shown to the reader.
  - D27: a true-extent symbol with all three views that passes the print minimum and the extent check satisfies D17.
  - D21, D24, D25 and D26 added under "Shapes and solids". D24 and D25 `lateral-ambiguous` are marked (pending).
  - The sources line lists D21–D27. The old "D22 is not in the work board" note is removed.
- README line "joint moments are CCW positive": marked "Reconcile: applies to mz in the side elevation, as D23 states", in the draft only. The README itself is not edited.
- Still absent from the work board: none of D21–D27 is missing now.

Not run: no test or check was executed.

## Task 10: coverage matrix summary prep (2026-11-02)

- Checked for `T/🗑️generated/validation/matrix.json` and `T/📝️sonnet-validation.md`: neither exists at this check.
- `T/🗑️generated/sonnet-validation/` holds only `full/` and `probe/`; no matrix file was found in them (not opened in depth).
- Per the coordinator's rule for a missing source, the "Coverage matrix summary" section of `📦️handoff.md` now has an empty table template (categories, the 149 total, OK / declared-unavailable / FAIL columns, all TODO), a "Run" TODO, a FAIL-codes TODO, and a "Sampled vs exhaustive" subsection with TODOs. No counts are filled in.

Not run: no test or check was executed.

## Task 11: README update for cast viewpoint, heightM, solids and the sheet export (2026-11-02)

File: `🎓️teaching/🏛️architecture/🧍️staffage/README.md`, Edit only, reread before each edit. The Conventions handedness line and the other sections were not touched.
- Item 1 (cast viewpoint): the "viewpoint" row already said required with no default (done earlier). The `mirror` row now says "only with `viewpoint` `elevation`" (D5), and the D19 allowlist rule.
- Item 2 (heightM): `heightM` is the physical figure height for every viewpoint (D13). `boundsM` holds the paper extents; `figureBoundsM` is written only with `transport`.
- Item 3 (solids): `axisX` and `axisZ` are offsets from the axis bone's rest origin (D24, marked pending for the rigs migration); `axisBone` is the shape's own bone or an ancestor (D21), in the solid table and in the sentence below it.
- Item 4 (sheet export): new paragraph after "Front and plan views": `--format sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg --projection first|third`; third angle is plan above and front to the right, first angle plan below and front to the left; projection required, no default; `--viewpoint` refused; one query, time, representation and scale; file name carries the viewpoint or projection (D14); DWG sheets unverified (`DwgUnavailableError`, pending). Source: `📝️sonnet-exports.md`.

German: the README is English-only, with no German text in it, so no German was added.
Coordination: sonnet-contracts' Conventions edit was not seen in the sections I touched, and I did not edit Conventions.

Not run: no test or check was executed.

## Task 12: README stale note and handoff sections (2026-11-02)

- README solids row: "pending: rigs migrates every revolved solid" dropped, per the coordinator (rigs finished the D24 pass: 411 revolved solids in 18 assets, 0.000 mm rest difference, relayed). D31 not added.
- `📦️handoff.md` filled, from `🗂️work-board.md` and the worker reports in T; each claim is tagged worker-reported, coordinator-checked or not verified:
  - Fleet record: all 20 rows with type, agent id, configured model and effort (Sonnet 5.5 or Haiku 5.5, medium, from the launch note; not observed), assignment, board state, and the report and its verification.
  - Concurrency: 20 live at launch against a runtime cap of 20; no per-agent model reported.
  - Decisions D1–D33, one line each. D16 is marked superseded by D24; D31 is marked not yet landed.
  - Schema changes: interfaces, conventions, exports (cast schema and CLI) and structural, each tagged.
  - Consumer migration: viewpoint required, `heightM` physical, `mzNm` rename with open consumers (React detail model, animation), fidelity, `--projection`. i18n and URL keys stay TODO.
  - Matrix and proofs left as TODO.
- Discrepancy: `📐️conventions.md` exists in T (contracts, about 8 KB, derived from code), but the coordinator's last note said it was not written. The handoff marks it "to confirm".
- Board gap: the work board gives no landing status for D1–D33; landing status in the handoff comes from the worker reports.
- Open items not verified: the course Python re-run; the site and React download caches' viewpoint keys; the viewer display of fidelity.

Not run: no test or check was executed.

## Task 13: evidence index and handoff source status (2026-11-02)

- `📦️handoff.md`: the conventions bullet now says `📐️conventions.md` exists, "worker-reported, exists", not coordinator-checked.
- `📦️handoff.md`: the coverage-matrix source status is re-checked. `🗑️generated/validation/matrix.json` now exists (10.1 MB, 17:34, with `summary.json`); it was not read into the handoff. `T/📝️sonnet-validation.md` still does not exist. The failures-script placeholder now names its argument (`<out.json>`, read from `process.argv[2]`) and the absolute-path problem. The course `tests.sh` is listed as a candidate replacement, not run.
- `📑️evidence-index.md` written: every file in T excluding `🗑️generated`, with owner, purpose and status; every script under `🛠️scripts` (about 300, by folder) with the run form by extension; and the citations that do not exist.
- Missing citation: `T/📝️sonnet-validation.md`. Stale citation fixed: the matrix file.
- Stray: an empty directory `true` at the T root.

Not run: no script, test or check was executed.

## Task 15: handoff update for D34–D36, W1, matrix v2 and consumer audits (2026-11-02)

- Decisions: D34, D35 and D36 added after D33, from the board, each marked "board, decided" with landing not reported; D35's D29 outcome is worker-reported by the board.
- Coverage matrix summary: matrix v2 note added (board, worker-reported, not verified; 37,233 cells, 28,052 ok, 1,113 declared unavailable, 8,068 FAIL; v3 pending, open). The table stays an empty template.
- New "Consumer audits" section, each item with its routed owner and status "open":
  - modes × views (`📝️haiku-consumer-modes.md`): six items to viewer, contracts, structural, thermal, rigs and validation;
  - React and site (`📝️haiku-consumer-react.md`): eight items, all to sonnet-viewer;
  - course call sites (`📝️haiku-consumer-callsites.md`): two items to sonnet-course;
  - export paths: absent. `📝️sonnet-exports.md` has no export-path audit section; owner sonnet-exports, open.
- W1 (wrap-up) recorded in the decisions list and in "Remaining work", with per-owner script counts; status open.
- Correction: the W1 line says its routing is in "Remaining work" and the index.

Not run: no script, test or check was executed.
