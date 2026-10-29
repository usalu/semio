# Print Thesis Audit

## Existing Dissertation Support

The existing phd kind already provides a two-sided chapter document, a binding gutter, examination metadata, bilingual abstract headings, acknowledgements, statutory declaration and signature, roman frontmatter, appendices, bibliography and registers. Its worked example has introduction, method, results and conclusion chapters. The new thesis feature should provide an explicit thesis declaration and full worked document rather than rename or delete this dissertation.

## Integration Boundaries

- Documents are registered in Print 🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json. Each declares its complete source roster. The build-graph inference derives build-ID targets from this catalog; there is no handwritten build-phd target in the Print project.
- The canonical root TeX source must explicitly specify theme=light and language=en or de. Production derives a dark source and publishes only ID.pdf and ID-dark.pdf into the mode-owned Print document slot. A German variant requires its own catalog entry or an explicit test probe; dark generation never translates content.
- Template compilation uses pinned Tectonic and TeX bundle, staged local fonts, deterministic mode and SOURCE_DATE_EPOCH. Resources and source maps are staged with collision/path-length checks. Do not invoke system LaTeX tools directly.
- Source staging rechecks every source and retained compiler input, so ongoing peer edits can produce a typed concurrent-edit refusal; retry the outer build after edits settle.
- The aggregate template consumer hardcodes templates=14 in 🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️pdf-consumption.json. One additional bilingual-theme document means 16; two language-specific documents mean 18. Update this alongside the catalog.
- Existing quick verification checks literal phd class and macro definitions. Preserve their behavior, and add independent thesis contract and rendering coverage.

## Missing Coverage to Add

A language-neutral thesis feature should exercise accepted/rejected metadata, explicit languages, degree/kind customization and safe TeX inference against AJV. PDF inspection should use the existing independent PDF.js interface, asserting thesis title, abstracts, acknowledgements, declaration, chapter order, references and appendix content in actual compiled PDFs. It should distinguish unnumbered frontmatter from chapter one, verify both theme outputs, and cover the German labels through an actual German root source or compiled probe.

A full worked template should demonstrate figures, tables, equations, citations, cross references, abbreviations/glossary where supported, research chapters and supplementary material; a successful compile alone does not prove these appear. Existing generic PDF verification only demands one page, readable text and valid glyphs for non-gallery templates.

## Verification Commands

Use owner targets through repo so compiler materials and artifact authority are issued correctly:

- repo run @semio-tech/print:test-thesis (after adding the dedicated contract target).
- repo run @semio-tech/print:build-thesis (catalog-derived document producer).
- repo run @semio-tech/print:test -- long (existing quick checks and PDF.js consumption of every published template).
- repo run @semio-tech/print:check (TypeScript owner check).

The build target publishes retained thesis.pdf and thesis-dark.pdf under the selected mode products slot, documents/print/thesis. The artifact layout is computed by 📦️artifacts/🟦️.ts via repository cache partition data; do not infer authority from an arbitrary absolute cache path. Build completion validates both PDF signatures; the long test independently reads all pages. No build or test was executed during this read-only audit.

## Rendered Verification Script

The portable ticket verify.ts consumes retained thesis.pdf and thesis-dark.pdf through the existing owned PDF.js inspection interface, checks visible metadata and academic components, unresolved references, valid glyphs, roman frontmatter and reset arabic page one, and writes rendered.md on success. It intentionally does not forge invocation authority. Direct repo script execution refused Original parent Script process invocation required. The owner-scoped command is repo exec --projects=@semio-tech/print -- repo <absolute-ticket-path>/verify.ts; the attempt entered capacity waiting behind 11 requests (2544 MB spare versus 3584 MB required), and was cancelled so the main build could proceed. Re-run after build completion. The current inspection interface exposes text/geometry/operators only, so PDF metadata and bookmarks cannot be inspected through it. Only English root documents are currently registered; the German abstract can be checked, while German apparatus requires a separate German compile probe.

## Inspector Readiness After Continuation

Read latest acceptance, progress, inventory and contract notes. The main producer remains owned by root; no duplicate build or inspector queue was launched. Confirmed the owner-scoped script command from repository run-script command contract and test: SEMIO_LANGUAGE=en repo exec --projects=@semio-tech/print -- repo /home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/EXTEND-PRINT-PORTFOLIO-WITH-THESIS/verify.ts. This is the actual repository-authored-script ingress, unlike direct repo script execution which lacks a parent invocation. Successful execution is still pending.

The ticket inspector now also calls the existing owned rasterPrintPdfV1 interface to render the cover and the first Arabic-numbered main page in each theme at scale one. PNG previews will appear under ticket 🗑️generated as thesis.pdf-1.png, thesis-dark.pdf-1.png and corresponding body-page files. Inspect with view_image, then remove these generated outputs before ticket close. The durable rendered.md records readable page evidence. All relative module imports were independently resolved successfully. Actual metadata/bookmarks remain beyond the existing snapshot API; the durable sixteen-case rendering target covers German language and degree variants separately.

## Final Pre-Publication Revalidation

Cheap source reads reconfirmed all current imports and the root run-script manifest ingress. Corrected the ticket inspector to consume the published artifact through the pipeline scratch execution, ensuring PDF readers and raster jobs inherit the same genuine resource-control lineage; the former outer execution would have made the artifact control a sibling of the verification scratch control. Bibliography text assertions now explicitly require Knuth, Wilson and their cited work titles, in addition to rejecting unresolved references. No expensive process or duplicate producer was launched. Ready for the successful root producer notification before running owned inspection and visual review.
