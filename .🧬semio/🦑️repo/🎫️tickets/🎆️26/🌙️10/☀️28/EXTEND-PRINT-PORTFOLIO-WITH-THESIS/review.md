# Independent Thesis Review

Read-only source review completed on 2026-10-28. No confirmed blocking issue was found in the reviewed thesis implementation. No build was launched because the parent task owns build validation.

- The worked thesis uses the declared Table environment with an ordinary tabular body. Table is a window wrapper, not a column-specification environment; this usage is valid.
- The Figure title, height, anchor=center and justify=center keys all exist in semio-window.sty.
- Thesis metadata schema and TypeScript validation agree on required language/theme/title/author/degree, custom thesis titles, string lengths and rejection of unknown properties.
- TypeScript metadata emission maps awardedDegree to the LaTeX degree key, thesisType to thesis-type and all remaining camel-case keys to their declared kebab-case setup keys. Literal TeX special characters are escaped.
- Thesis loads scrreprt with two-sided recto chapters; title/front matter uses Roman page numbers and makemainmatter restores Arabic numbering and page parity. Both Babel languages used by the abstract environments are loaded, with English or German selected from the explicitly required class language.
- The doctoral specialization loads the shared thesis package and selects doctoral level. Both specializations share the same front matter and declaration commands.

The final compiled PDF, reference resolution, fonts, title-page fit and pagination remain the responsibility of the parent build/visual validation.

## Durable Rendering Harness Review

Two blocking source issues were found in the newly added rendering harness and reported immediately to the parent and thesis_contract agent:

1. The generated manuscript invokes `\mainmatter`, while the semio class uses scrreprt and exposes `\makemainmatter`. The declared command must be used to avoid an undefined-control-sequence compiler failure.
2. The missing-language refusal case removes `language=en,`, but thesisPreamble emits language last as `language=en]`. Its mutation therefore leaves the source unchanged and fails assert.notEqual before the refusal probe. Remove `,language=en` instead.

The probe intent uses the accepted finite thesis.tex name and passes the retained PrintExecutionV1 to the existing compiler/refusal interfaces. Shared class/style/font resources are provided through the compiler staging layer; this manuscript needs no extra resource declarations. Imports resolve from the rendering directory to Print modules. Expected English and German thesis type, declaration, dedication and abbreviation headings match semio-thesis.sty, with curly/straight apostrophes and whitespace normalized. All custom cases use the same explicitly authored Independent Study title. Title, institution, supervisors and declaration text expectations match generated metadata. No build or other expensive check was launched during this review.

## Updated Rendering and Navigation Audit

The earlier undefined mainmatter command and missing-language mutation are now corrected. The added doctoral specialization exercises its title command, and footer extraction now checks Roman front matter and Arabic main-matter restart.

Two remaining source semantics need attention:

- The title command starts Roman numbering at i and ships that page. Calling makefrontmatter afterward resets the page counter to i again. Hyperref normally names these page destinations page.i, producing duplicate destinations and potentially routing front-matter page links to the title. The harness currently exercises this duplicate-number sequence without checking anchors. Define one intentional title/front-matter numbering policy and avoid reusing page destinations.
- The begin-document validation hook checks only type=thesis. A type=phd document can override its preset level with thesis-level=custom while omitting thesis-type; this bypasses the custom-heading validation used for general theses. Validate both academic document types, or prohibit contradictory doctoral metadata.

Validation gaps remain: the rendering harness checks visible text but does not inspect PDF title/author/keywords metadata or bookmarks; it checks one front-matter footer and one main footer but does not assert recto chapter placement, blank verso behavior or suppression of chapter numbering in back matter. The phd probe retains an explicit doctoral level, so it does not independently prove the specialization default. These are gaps rather than demonstrated compiler failures. No expensive command was launched.

## Shared ToC Patch Review

Reviewed the updated heading recorder, ToC append routine, auxiliary loader and sanitizer without launching a build. The KOMA renderer now sends an explicit blank number for its empty number argument; the append routine and auxiliary loader preserve that entry, and the sanitizer does not remove it merely for being unnumbered. Numbered chapter/section paths retain their prior counter expression, with the buffered auxiliary record expanding that number at the heading location. No new blocking issue was found in this patch. Existing in-memory number sequences retain unexpanded counter tokens; the compiled multi-pass numbered chapter/section checks should confirm the frozen auxiliary representation used by the rendered ToC.

## Final Documentation, Schema and Wiring Audit

The README names existing test-thesis and test-thesis-render project targets and the actual template/thesis package export. The registered thesis catalog document participates in the existing inferred build target mechanism. The public facade explicitly reexports Thesis, thesisPreamble, escapeThesisText, isThesis and thesisSchema. Its schema properties map to the LaTeX setup keys; custom thesis titles now require a non-whitespace character in both owned validation and the AJV oracle, and LaTeX rejects blank custom titles. README matter-transition descriptions match the revised shared implementation.

Concrete remaining validation gaps: the render matrix always provides declarationText, so neither localized default declaration paragraph is exercised; it does not render the optional makedeclaration argument to verify override precedence; custom declaration/dedication/abbreviation title settings are checked in emitted TeX but not rendered; Acknowledgements is absent from the durable rendered manuscript. The neutral Gherkin scenarios cover the TypeScript API and principal thesis flows but do not individually specify these public LaTeX behaviors. These gaps were reported to the parent. Metadata/bookmark inspection remains outside the current page-text assertions as recorded above. No additional build or expensive check was launched.

## Minimal Cover and Babel Review Pending

At the first focused read, the rendering source and neutral fixtures contained customization probes but the announced minimal-cover and Babel transcript additions had not yet been saved. Requested a completion notification from the contract agent before reviewing their exact expectations. Optional metadata absence checks should inspect the title page alone to avoid false positives from declaration body content or repeated page chrome; the mandatory Academic Work/Abschlussarbeit label remains expected even for a minimal title. No expensive check was launched.

## Minimal Cover and Babel Review Completed

Reviewed the saved two minimal probes and their neutral expectations. Both generate only required metadata through thesisPreamble; EN/master/light and DE/doctoral/dark use the correct localized type headings. Absent optional labels match the actual core label spellings and are checked on the first PDF page only, avoiding false matches from declaration/body content. Neither the mandatory academic-work header nor title/author labels is forbidden. The no-four-digit assertion is safe for the authored minimal titles/authors.

The actual transcript markers run inside each Babel otherlanguage environment and immediately after it closes. Expected english/ngerman names agree with semio.cls Babel loading and explicit main-language selection; the German probe proves restoration after the English abstract, and the English probe proves restoration after Kurzfassung. Probe source uses declared commands and requires no resources beyond staged shared styles/fonts. No concrete blocker or false-failure expectation was found in this focused addition. No build was launched.
