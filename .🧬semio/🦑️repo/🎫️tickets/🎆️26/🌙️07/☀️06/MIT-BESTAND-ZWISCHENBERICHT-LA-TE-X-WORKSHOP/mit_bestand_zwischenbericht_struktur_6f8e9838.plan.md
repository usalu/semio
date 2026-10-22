---
name: Mit Bestand Zwischenbericht Struktur
overview: Restructure the mit-bestand Zwischenbericht into the requested 4-section outline (pure heading skeleton, no body prose) using section→subsection→subsubsection→paragraph headings plus nested lists for the deepest leaves, rename AP1–4 to AP-Erfahrung/AP-Plattform/AP-Tool/AP-Validierung, and refresh the cover page with the real project metadata you provided (LUH/UdK Berlin, subtitle, abstract, updated Förderzeitraum).
todos:
 - id: core-kurzfassung
   content: Add \kurzfassung field to print/tex/semio-core.sty
   status: completed
 - id: components-cover
   content: Render author/subtitle/kurzfassung in \makecoverpages in print/tex/semio-components.sty
   status: completed
 - id: rewrite-tex
   content: "Rewrite mit-bestand/bericht/zwischenbericht/zwischenbericht.tex: cover metadata, renamed AP workpackage table, full 4-section heading skeleton (section/subsection/subsubsection/paragraph + nested itemize), secnumdepth=4, remove old prose/appendix"
   status: completed
 - id: build-verify
   content: Build with bun ./📜️script.ts build and visually verify light+dark PDF output
   status: completed
 - id: ticket-close
   content: Reopen MIT-BESTAND-ZWISCHENBERICHT-LA-TE-X-WORKSHOP ticket, then close with summary
   status: completed
isProject: false
---

## Scope

Edit the real report source (not the -dark variant, which is auto-derived at build time by `print/script.ts`'s `compileLightAndDark`):

- [mit-bestand/bericht/zwischenbericht/zwischenbericht.tex](mit-bestand/bericht/zwischenbericht/zwischenbericht.tex)

Two small, shared-template additions are needed because the current `\makecoverpages` macro never renders `\author` or `\subtitle`, and there is no field for a lead abstract:

- [print/tex/semio-core.sty](print/tex/semio-core.sty): add a `\kurzfassung{...}` field (mirrors `\aktenzeichen`/`\foerderzeitraum`), stored in a new `\l_semio_kurzfassung_tl`.
- [print/tex/semio-components.sty](print/tex/semio-components.sty): extend `\makecoverpages` (lines 28-44) to render `\@author`, the existing-but-unused subtitle, and the new `\kurzfassung`, guarded by `\tl_if_empty:NTF` like the existing `aktenzeichen`/`foerderzeitraum`/`doi` lines. This is backward compatible — `forschungsbericht.content.tex` doesn't set these fields, so nothing changes for it.

## Heading depth strategy

Your outline goes up to 7 levels deep (e.g. `1.2.1.1.1.1.1 Depotshop`). `scrartcl` (which `zwischenbericht` type loads) supports `\section → \subsection → \subsubsection → \paragraph → \subparagraph`, but `semio-window.sty`'s `\semio@heading@installall` only styles `\part/\chapter/\section/\subsection/\subsubsection/\paragraph` with the window-chip look — **`\subparagraph` has no chip styling at all**, so using it would look visually inconsistent.

Decision: cap real numbered headings at `\paragraph` (4 levels) and render everything deeper as nested `itemize` lists (up to 3 levels, well within LaTeX's default nesting limit). Add `\setcounter{secnumdepth}{4}` near the top of the document so `\paragraph` gets numbered (default `secnumdepth` only numbers down to `\subsubsection`).

Mapping:

- `\section` — Ergebnisse / Projektstand / Mittelverwendung / Ergebnisverwertung
- `\subsection` — Arbeitspakete, Personalkosten, Leistungen Dritter, Repositorien, …
- `\subsubsection` — AP-Erfahrung/AP-Plattform/AP-Tool/AP-Validierung, Leibniz Universität Hannover, …
- `\paragraph` — Recherche, Interviews, User Stories, Generatoren, Test-Case, …
- nested `itemize` (2-3 levels) — Bauteilbörsen▸️Archetypen▸️Depotshop, Geplante Interviews▸️I-Usability▸️I-U-Plattform, etc.

I'll rebuild the tree from your **indentation** (source of truth) rather than the literal printed numbers, since several are inconsistent/duplicated in the pasted outline (e.g. duplicate `2.2.1.1.1.1` for both I-U-Plattform/I-U-Tool, `2.2.4.1` reused for both Test-Case and Workshop, `4.2`/`4.3` reused). LaTeX will auto-number everything correctly regardless. I'll also fix obvious typos while transcribing: Harvestkatalog, KI-Modelle, Entwurfsgrammatik, AP-Validierung, Leibniz Universität (Hannover).

Per your answer, headings get **no body text** — pure skeleton. This means removing all of the current prose: the Recherche/Entwurfswerkzeug/Bauteilportal `Blockquote`s, the Plan/Ist comparison table, the Mittelverwendung/Ergebnisverwertung bullet lists, and the `\appendix`/`Anlagen` section (none of it maps onto the new 4-section tree you specified).

## Cover page updates

Per your answers:

- `\makeworkpackages` table: rename AP1–4 to **AP-Erfahrung / AP-Plattform / AP-Tool / AP-Validierung**, with a short "Schwerpunkt" phrase per AP matching the new outline branches, and partner column using **Leibniz Universität Hannover** / **Universität der Künste Berlin** (best-guess mapping — AP-Tool's Tragwerksanalyse clearly sits with UdK Berlin/Gengnagel's chair; flag for your review since the exact per-AP partner split wasn't specified).
- `\title{Entwerfen mit Bestand}` — unchanged.
- Add `\subtitle{Eine offene Plattform für einen KI-unterstützten, performance-optimierten und integrativen Entwurfsprozess mit wiederverwendeten Baukomponenten}`.
- Add `\kurzfassung{...}` with the abstract paragraph you provided (Entwerfen mit inhomogenen Baukomponenten…).
- `\author{...}` — replace ETH Zürich/Semio Tech GmbH placeholder with the real project partners: Leibniz Universität Hannover (Fakultät für Architektur und Landschaft, IEK, Abt. Nachhaltige Gebäudesysteme; Projektleitung: Ueli Saluz; weitere Bearbeitung: Nikolaus Möllenhof) and Universität der Künste Berlin (Konstruktives Entwerfen und Tragwerksplanung; Kinan Sarakbi).
- `\aktenzeichen{10.08.18.7-25.06}` — unchanged, already matches.
- `\foerderzeitraum{11/2025 - 05/2027}` — updated from the current placeholder `01/2024 - 12/2026` per your data (Projektbeginn 11.2025 / Projektende 05.2027).

## Verification

- Build with `bun ./📜️script.ts build` in `mit-bestand/bericht` (or the `latex` nx target) to compile via Tectonic, confirming the new secnumdepth/itemize nesting and cover fields compile cleanly for both light and derived dark variants.
- Visually spot-check a rendered page (convert PDF page to image) to confirm the `\paragraph` chip styling still reads well with back-to-back empty headings and that nested nested lists render legibly.

## Ticket workflow

Reopen the existing closed ticket `MIT-BESTAND-ZWISCHENBERICHT-LA-TE-X-WORKSHOP` (goal `🎯️r2603`) since it covers this exact file, rather than opening a new one; close it again when done with a summary of the files touched.
