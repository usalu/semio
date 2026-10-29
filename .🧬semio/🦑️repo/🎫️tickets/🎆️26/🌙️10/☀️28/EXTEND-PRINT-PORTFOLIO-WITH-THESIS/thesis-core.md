# Academic Thesis Core

The general thesis apparatus lives in semio-thesis.sty. semio-phd.sty selects the doctoral specialization and exposes dissertation-specific entry points. Both use one metadata model and title layout.

Thesis level is explicit: bachelor, master, doctoral, or custom. Custom requires a thesis type. Thesis and doctoral documents require English or German explicitly. University, faculty, institute, awarded degree, discipline, programme, student ID, supervisors, reviewers, dates, place, and ORCID render only when populated. Faculty and institute remain visible even without university. PDF metadata uses document title, author, and keywords.

Frontmatter begins with Roman numbering; the thesis title owns page i and restores body geometry before subsequent content. Existing mainmatter resets Arabic numbering. Backmatter preserves the Arabic page count and disables numbered headings. Academic sections remain unnumbered. English Abstract and German Kurzfassung select their own hyphenation language. Keywords accompany the English abstract. Dedication, acknowledgements, an abbreviations description list, and customizable declaration text/headings are explicit opt-in components.

The source checks for doctoral components now verify shared apparatus and doctoral delegation. Structural brace balance passes for all four affected LaTeX files. Full pipeline source verification is running through the repo tool interface. The parent task runs the provisioned thesis renderer for TeX compilation; no TeX engine exists directly on PATH.

## Files

- semio.cls
- semio-core.sty
- semio-thesis.sty (new)
- semio-phd.sty
- quick-pipeline-verification/🟦️.ts

Class option `bindingoffset` accepts a TeX dimension. Defaults remain 8mm for general bound documents and 15mm for doctoral copies. Isolated academic source assertions pass for both class variants, chapter appendices, doctoral delegation, all five optional environments, Unicode literals, and balanced bilingual scopes. Full source verification currently stops earlier at the document catalog count (89 actual versus 88 expected), which the parent task is updating.

Full `verifyPrintPipelineSourcesV1` now passes after the parent updated the catalog count: 1966/1966 visualization leaves, 1738 kinds, 131/131 public API, and published glyph policy checks. Durable binding-offset source assertions verify the option, 8mm/15mm defaults, and all five bound geometry branches.

## Matter and Contents Audit

The title records entry into frontmatter. Reentering frontmatter preserves the Roman counter instead of reusing page i. Front/backmatter suspend chapter numbering while retaining the configured numbering depth; mainmatter restores it. Doctoral configurations now receive the same custom-type validation as general theses.

KOMA passes empty number text to the shared heading renderer for starred headings and for chapter numbering disabled by matter transitions. The renderer formerly ignored that indication and wrote the current chapter counter into contents rows. It now stores an empty numeric cell for these headings, and the contents collector retains the resulting unnumbered rows. Empty numeric cells do not create reverse navigation anchors. Auxiliary row labels remain unique and preserve title/page links. Numbered headings retain their existing hierarchy values. Brace audits pass after these changes; rendered second-pass assertions are owned by the thesis render harness.

Full source verification passes again after the matter/contents fixes. The final typography audit scopes title-page paragraph spacing and declaration signature fonts, preventing either from affecting later content; isolated scope assertions pass and are retained in the quick pipeline checks. Additional changed LaTeX file: semio-window.sty (shared heading/contents tracking). semio-core.sty now also suppresses empty reverse-link anchors. No comments were added inside definitions.

## Final Source Diff Review

Reviewed all five academic LaTeX files against the tracked baseline and inspected the new thesis file directly. Contents collector, auxiliary loader, sanitizer, blank numeric formatting, unique title/page labels, and reverse-navigation anchor guard agree on unnumbered entries. Numbered contents rows retain their previous path. Academic missing/unsupported language cases reach class errors rather than a fallback; other document kinds retain their established defaults. The title and signature typography groups leave geometry and numbering transitions outside those local groups. Babel language scopes remain balanced. No additional semantic issue was found.

Final evidence: the exact retained academic source assertion block from quick pipeline verification passes with the latest typography changes, explicit academic language guards, and bilingual scope assertions. The complete pipeline source suite passed after the matter/contents changes. No additional build was launched during this final review; actual TeX/PDF verification remains owned by the queued root build and render harness.

## Doctoral Template Regression

Launched `repo run @semio-tech/print:build-phd` once through the normal repository capacity controller (session 71002). The command builds the original doctoral template in light and dark editions. Initially queued behind fourteen requests; no capacity bypass or duplicate thesis/matrix run was used. Terminal compiler result is pending.

The doctoral regression reached repository graph verification (1154/1920 input closures) but had not started TeX. The parent task diagnosed memory reservation contention between concurrent owned Print commands and explicitly requested sequential execution. After verifying PID186244 was exactly the doctoral build command, SIGTERM cancelled that command only. Session71002 exited143 and reported leases released. Process inspection finds no original parent, direct child, or process group. No unrelated process was signalled. The doctoral regression will be relaunched only after the root thesis build completes.

## Minimal Cover and Babel Probe Review

Read-only review confirms a required-fields-only English master cover omits Faculty, Degree, Supervisor, Reviewers, Place, Submitted, Defense, Student ID, and ORCID. Its retained labels are Title, Academic Work, Author, and the master thesis type. The equivalent German doctoral cover omits Fakultät, Grad, Betreuung, Gutachten, Ort, Eingereicht, Disputation, Matrikelnummer, and ORCID; retained labels are Titel, Abschlussarbeit, Verfasser, and Dissertation. University, institute, discipline, and programme have no independent labels. The cover never reads LaTeX date metadata and therefore emits no implicit today/date/year. No omission defect was found.

Babel probes should observe `english` inside Abstract and `ngerman` inside Kurzfassung. After each environment the document language returns to `english` for English documents and `ngerman` for German documents. These exact expected values and omitted-label sets were sent to the render harness owner; no code or compiler run was added during this review.

## Resumed Source Readiness

Revalidated the current shared LaTeX sources against the retained academic assertions after peer activity. All academic assertions pass, all five files retain balanced braces and literal Unicode, and the shared contents collector still preserves empty numeric cells for unnumbered headings. The original doctoral entry point still selects type phd and language en explicitly. No concrete drift from this audit was found. No heavy job was started. The deferred doctoral regression remains serialized after the sole root thesis build, using `repo --tool=semio run @semio-tech/print:build-phd --parallel=1` once root success is confirmed.

## Pinned Language Runtime Readiness

Read-only syntax/definition-order review against the current pinned bundle manifest found no specific additional defect. The2022.0r0 bundle explicitly contains expl3-code.tex, expl3.sty, xparse.sty, babel.sty, english.ldf, ngerman.ldf and ngermanb.ldf. This verifies required material presence, not actual API execution. The same e-variant and lazy-boolean families are already used in other loaded Print styles. Shared academic metadata exists before use, doctoral default selection precedes begin-document validation, and hyperref is loaded before the metadata hook executes. Both Babel language environments are balanced and declaration prose remains outside ExplSyntax tokenization. No speculative compatibility change was made; actual compiler output remains decisive.

## First Concrete LaTeX Failure

The next actual compiler run recovered the precise error: semio-thesis.sty line366 attempted NewDocumentCommand dedication, which is already owned by KOMA scrreprt. The new academic shorthand is now makededication, matching makedeclaration naming, while the Dedication environment remains unchanged. There is no compatibility alias and no renewal of the class command. README, worked thesis source and customization rendering fixture were hand-updated together. A search confirms no dedication command uses remain in Print-owned source.

Added language-neutral command ownership expectations and Gherkin coverage. The new test failed before the rename, then the complete cheap thesis unit suite passed37tests/121assertions with independent AJV checks. The full rendering fixture covers both the unchanged environment and the new shorthand. This addresses the reported collision; successful compilation after it remains pending the parent-run publication. Changed files: semio-thesis.sty, README.md, thesis.content.tex, thesis rendering test, thesis neutral fixture, thesis Gherkin feature and thesis unit test.

## Final Implementation and Runtime Evidence

The shared thesis apparatus, doctoral specialization and related renderer corrections are complete. Actual composed rendering validates21positive cases (17degree/language/theme includingPhD,2minimal/Babel,2customization) and6required refusals. The PDF adapter preserves only documented absent PDF.js glyph slots, with generic closedguards unchanged. Sharedreader failure propagation preserves primaryidentity without admitting newfinalvalidation after firstfailure; terminalrefusal classification distinguishes only exactprimary fanout from cancellation. All temporaryproductDEBUG removed. Focusedreader/refusal suite14pass208assertions; finalexplicitstrictclosure91162zero.

Final actual canonical native run-many77993 completed terminal0 with all7actions and both registered live thesis/PhD dualtheme publishers. See canonical-build.md for exact command, target roster and lifecycle evidence; relatedreports pdf-adapter.md, artifact-reader-failure.md and refusal-lifetime.md distinguish specific diagnoses/tests. Parent owns final livePDFinspection/preview gate and ticket completion. No further implementation or heavyjob remains owned by this agent.
