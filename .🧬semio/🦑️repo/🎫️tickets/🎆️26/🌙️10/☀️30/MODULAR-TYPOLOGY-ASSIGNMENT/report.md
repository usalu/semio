# Modular Typology Assignment

Source: `temp/Aufgabenstellung_ Parametrischer Entwurf modularer Typologien.docx` (German only, Word styling).

## Result

New course assignment `typology` ("Aufgabe 1: Parametrischer Entwurf einer modularen Typologie" / "Assignment 1: Parametric Design of a Modular Typology") in the Parametric topic of the Architecture course, built on the existing `assignment` Print class and published through the existing course PDF pipeline next to `parametrization`.

| Area | Files |
|---|---|
| Authored leaf | `🎓️teaching/🏛️architecture/🏙️typology/📝️assignment/{en,de}.tex`, `{en,de}.content.tex`, `schema-m-b-1-q.png`, `schema-m-w-2-h.png`, `schema-e-b-2-qh.png` |
| Leaf tests | `…/🏙️typology/📝️assignment/🧪️tests/🥒️.feature`, `🟦️.ts`, `🧫️cases.json`, `🧬️cases.schema.json` |
| Course | `📚️course/🔣️.json` (assignment entry), `🏗️builder/📖️assignments/🟦️.ts` (per-assignment source registry, `buildCourseAssignmentV1(…, id, language)`), `📦️packages/🟦️typescript/📜️script.ts` (`build-typology-en/de` via one script factory), `📋️project.json` (two cached targets, `dev`/`dev-site`/`build` depend on them, leaf in `default` and `assignment` inputs), `🧪️tests/📖️assignment-pdfs/{🟦️.ts,🥒️.feature}` (four publications, typology semantics), `README.md` |

## Content decisions

- Structure: title block (`\maketitle`: course, submission date), section "Aufgabenstellung"/"Brief", then Definitions, Parts A–D, Grading (dMA), Submission. Workflow, definitions, example overview table and individual-submission files are `Table` windows with `SemioTable`, so they carry the standard chips and numbering.
- The three example axonometries from the docx are cropped, downscaled to 720 px and stored as ink-alpha PNGs (multiply look on the cream canvas, light canvas box in dark theme via `\AssignmentContrastImage`); script `prepare-schema-images.py`.
- The facade photo (`image4.jpg`) of the docx is **not** included: its rights are unknown and the course PDFs are public.
- The docx has an **empty** "Bewertung (NGS-Teil)" heading; it is omitted. Add the NGS grading criteria to both `*.content.tex` once available.
- Fixes against the docx: "einer modularer" → "einer modularen", consistent en/de terminology. File names of the submission stay German in the English version (they are fixed by the supervisors); English text explains `Nachname`.
- `\author` is "dMA und NGS" / "dMA and NGS"; institution/course mirror the parametrization assignment.

## Verification

- Local XeLaTeX probe (`compile.ts` with `SEMIO_PROBE_ENGINE=xelatex`, staged like the Print pipeline: compiler names, projected references, library, fonts) → both PDFs 4 A4 pages, PDF title and `/Lang` set, 0 overfull boxes, 0 missing glyphs, no glyph outside the page (`review.py`); all pages inspected visually.
- `run-tests.ts`: `verifyTypologyAssignmentContentV1` passes; all five adapter scenarios (composition, naming, submission, languages, courseOwnership) agree with their mathjs, d3-array and ajv oracles; the parametrization contract still passes; the publication list yields the four expected PDFs.
- `validate-course.ts`: both assignments validate against the course product `Assignment` schema (ajv) and `courseIssues` reports none.
- `tsc` over the course package: no errors in any changed file.

## Not verified

- `repo` (`AppData/Local/Semio/bin/repo.exe`, 2026-10-26) rejects the current launcher registry (`field: restart`), so no `repo run …:build-typology-*`, `test-assignment` or `dev` run happened. The cached tectonic bundle lacks TikZ `patterns`, which `semio-viz-mark.sty` now loads, so the official tectonic route must refresh `@semio-tech/print:deps-tex` first.
- The course vitest config currently fails before tests with "Bun published workspace patterns drift: 🎓️teaching/package.json", caused by other in-flight workspace changes, not by this ticket.
- The site rendering of the second assignment card was not checked in a browser.

## Follow-up 2026-10-30: launcher rebuild

- Rebuilt the launcher with `🚀️launcher/📥️install/🔵️.ps1` (now `%LOCALAPPDATA%\semio\launcher\bin\repo.exe`, contract 1) and the full native command with `repo run repo-mcp:install` (`%LOCALAPPDATA%\Semio\repo\aa03…\repo.exe`, first `repo` on the PATH). `repo --version` works again.
- `repo run @teaching/architecture-course:build-typology-en` then stops before compiling: the Bun locks of `🎓️teaching`, `🌎️hub` and `🏢️semio-tech` still list `…/🌊️flow/🎁️browser-package/🕸️bindings`, which uncommitted edits by another session removed from six `package.json` workspace lists (also the cause of the vitest "workspace patterns drift").
- The remedy the tool names, `repo dependencies lock --scope <scope>`, fails deterministically on this host with `Current lock Source ancestor changed` (`📦️dependencies/💾️storage/🟦️.ts`, ancestor identity stamp incl. `ctimeNs`/`mtimeNs`), although the ancestor directories had not changed for 40 minutes. This is a defect of the lock pipeline on Windows, not of this ticket; the official typology build and `test-assignment` stay unverified until it is fixed.

## Follow-up 2026-10-30: Windows dependency lock fix

Root causes of `repo dependencies lock` refusing on Windows, all fixed:

1. `📦️dependencies/💾️storage/🟦️.ts` captured input identities with non-bigint `lstatSync`. NTFS file IDs exceed 2^53 (`🎓️teaching`: `12947848932114887`, captured as `…888`), so the later exact bigint recheck always failed ("Current lock Source ancestor changed"). Capture and recheck now use `bigint` stats (also the Bun cache device check).
2. `🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` hashed the whole `FILE_STREAM_INFORMATION`, including `StreamAllocationSize`, which NTFS adjusts lazily after close ("Exact named Windows creation after actual close required"). The hash now leaves out the allocation bytes; name and size (pinned to `EndOfFile`) stay.
3. The same descriptor module synthesised Windows modes that differ from Node/libuv (`0o40777` for directories, `0o120777` for links, directory size = allocation), so parent identities never matched `lstat` ("Current lock Source endpoint custody changed"). It now mirrors libuv: `0o444` plus `0o222` unless readonly, for every kind, and size 0 for directories.

Regression test `💾️storage/🧪️tests/🆔️identity` (suite `javascript-input-identity`, all platforms) pins the observed file ID and uses the TypeScript compiler API to require `bigint` on every stat whose identity is read or handed on; it flags all four original sites on the `HEAD` version (`mutation-identity.ts`).

All six Bun locks then regenerated. The official typology builds still could not run: another session keeps changing workspace members (`🕸️bindings` removed, then restored; `🗃️cache` added) in six `package.json` files, so the locks go stale again; once in between, `--all` and one build hit "Current Store group work capacity exceeded".

## Follow-up 2026-10-30: cover page

- New `\makeassignmentcover` in `semio-assignment.sty` after the zwischenbericht cover: windows for course and assignment number, title, cover image, a row of facts (`\SemioAssignmentCoverFact`), partner addresses (`\SemioAssignmentCoverPartner` with `\AddressOrg`/`\AddressPeople`) and the institution logo beside the faculty. New setup keys `number`, `faculty`, `logo`, `cover-image`, `cover-title`; built-in window titles are localized en/de; cover settings are scoped so `\parskip` does not leak. Covered by a print contract test and feature scenario.
- The assignment uses it in both languages instead of `\maketitle`: `cover.jpg` (from `temp/Cover.png`, 1672×941, 371 KB), modules/semester facts, dMA and NGS teams, LUH and the faculty. Typos of the raw info fixed (Architektur, Digitale, Dr.-Ing., Tutor:innen).
- The LUH logo is now owned by the leaf (`logo-luh.png`, `logo-luh-dark.png`, byte-identical to the print and zwischenbericht copies), because document sources only get their own paths projected to compiler names; the library path left the cover and footer logos blank.
- The cover reports one overfull box of the text width from the shared `\SemioImage` cover-fit (image scaled past the window and clipped), as for the zwischenbericht Arbeitsprobe; nothing visible overflows.

## Follow-up 2026-10-30: simulation model axonometries and summer airflow

- Part B (both languages): a third drawing item asks for the same axonometries (base type and variants 1–4, same viewpoint) of the simulation model, its planar surfaces coloured by their U-value from Part C with a legend, carrying the airflow from Part D. The visualization model is now described as the basis of the design axonometries.
- Part D: a third bullet asks for an estimate of the airflow through façade and room for summer with full solar load (window openings, thermal buoyancy), shown as arrows in the simulation axonometries.
- Workflow rows B and D, the CSV row and two new PNG rows of the individual submission (`Nachname_<Modul>_Simulation_Variante0…4.png`) follow suit; the archive grows from 16 to 26 files.
- Fixture `submission.imageModels` (`""`, `"_Simulation"`), `tableRows` 9, schema, adapter (composition: 20 axonometries per member; submission: model × module × variant names, oracle d3 `cross`) and feature (new scenario for the simulation axonometries) updated; all five scenarios agree; both PDFs render with 5 pages, no new overfull boxes.
- U-value colours follow the fixed colour scale of the layout template (no own legend), and each surface of the simulation model carries its U-value colour as the material of its mesh in the GLB (simulation model description and GLB submission row, both languages; feature scenario).
- Cover image replaced by `temp/Typologie.png` (`cover.jpg`, 1672×941, 325 KB), window titled "Typologie"/"Typology". Cover chips shortened to dMA, NGS, LUH and Archland; the latter two through new optional keys `institution-label` and `faculty-label` of `\SemioAssignmentSetup` (fall back to the institution name and the localized "Fakultät"/"Faculty").
- Simulation model, its U-value material colours, calculations, airflow and its axonometries moved from Part B (dMA) to Part D (NGS): Part B keeps the visualization model and its axonometries (noting that the simulation model is later generated from the same parameters); Part D now has the paragraphs Simulationsmodell/Simulation model, Berechnungen/Calculations and Darstellungen/Drawings; workflow rows B and D follow; feature scenario updated.
- Cover rebalanced: image weight 9.4 → 11.4, title 1.6 → 1.1, partners 5.6 → 4.2, logo row 2.2 → 1.5 (logo height 1.1 cm); the partner windows list organisation lines directly instead of `\AddressOrg`, whose fixed five-line box left a gap above the people.
