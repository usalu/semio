# Thesis Declaration Contract

## Implementation

The schema requires explicit English/German, light/dark, title, author and bachelor/master/doctoral/custom degree. Custom theses require a non-whitespace thesisType. Optional awardedDegree supplies the academic qualification through the LaTeX degree key; the degree enum maps to thesis-level. Academic institution, programme, supervision, review, dates, identification, keywords and section/declaration customization remain plain text.

Inference emits the semio thesis document class and SemioSetup metadata with escaped TeX characters. The public @semio-tech/print/template/thesis interface exposes the type, owned schema, validator and preamble functions without adding a runtime library. The router and package project expose test-thesis and test-thesis-render. The rendering target declares fonts, Tectonic, TeX bundle and generation prerequisites.

## Durable Coverage

Tests were authored and first run before implementation, failing on the missing schema. JSON fixtures provide independent validity and exact TeX expectations; AJV 2020 adjudicates every declaration. Gherkin describes the language-neutral contract. Inferred setup keys are checked against the actual LaTeX interface. The whitespace-only custom heading fixture prevents a schema/LaTeX blank-validation mismatch.

Actual rendering uses the production compiled probe interface with issued resource authority and independent PDF.js snapshots. Fixtures cover all sixteen degree/language/theme combinations and the existing phd specialization. Assertions cover localized headings, academic metadata, abstract languages, authored declaration text, readable body/back matter, Roman front matter footers and Arabic main matter restarting at one.

The source renders an actual tableofcontents. Tests parse thesis.register-toc rows and require empty numbers for generic front/back chapters, abstracts and localized declaration/dedication, with main Research Body numbered one. PDF text must contain each expected title in contents and body. Compiler transcript assertions verify the preserved odd page after the title, unchanged repeated makefrontmatter page, suppressed front/back chapter increments and restored caller section numbering depth. Duplicate page.i anchor diagnostics are rejected.

Six actual compiler-refusal requests cover missing language, missing level, unsupported level, custom level without heading, whitespace-only custom heading and phd overriding its level to custom without heading. The isolated compiler refusal inspector preserves actual compiler diagnostics and caller cancellation.

Two additional language-specific customization probes render the localized default declaration, setup-custom body/title and a later optional body. PDF text counts prove setup and optional bodies each occur exactly once, testing precedence. Custom dedication/abbreviation headings and localized acknowledgements content are also checked.

Two additional required-only probes cover an English master thesis and German doctoral thesis. Independent first-page text checks require title, author and localized academic type, and reject absent institutional, qualification, supervision, date and identity labels. The same complete-document probes record actual Babel languagename inside Abstract and Kurzfassung and immediately after each environment, checking english/ngerman scope and restoration against neutral expected strings. Both minimal declarations also pass the owned validator and independent AJV in the direct suite.

PDF catalog metadata is outside the current snapshot interface; PDF text and keywords are covered. Core review identified and fixed Abstract's unclosed language scope, doctoral custom-validation bypass, front matter restart and stale ToC numbering; those fixes belong to the core agent.

## Current Acceptance

Visual review found contrast defects in the earlier complete private PDFs. The semantic foreground fix is now saved and all57direct thesis/contrast tests pass196assertions. Earlier compilation/reader evidence below predates these foreground changes and does not accept the current source; fresh canonical publication and visual contrast review remain required. All27feature scenarios now have composed actual evidence in composed-acceptance.md. Details and independent colord ratios are recorded in foreground-contrast.md.

- Contract coverage: core reports37thesis tests passed with121assertions after the dedication command-ownership regression. The independent AJV and literal TeX expectations remain part of these tests.
- Final explicit worker/PDF/thesis strict closure72721 passed with zero diagnostics. The permanent Print CheckScript now roots the PDF worker loaded by URL and its colocated tests.
- Actual compilation5888 completed both real compilation entries and published thesis/phd in both themes inside the genuine private workspace.
- Reader replay99370 completed with terminal0:13tests passed,129expectations. The independent pinned PDF.js oracle and production owned worker both ran; all four actual PDFs passed academic text, resolved references/bibliography and first Arabic body-page assertions.
- Complete private publication proof version2 records32pages and physical body page13 for thesis in each theme,24pages and physical body page11 for phd in each theme. Eight cover/body previews were exported at1191×1684, with SHA256 for every PDF and PNG. Proof declares complete=true and canonicalWorkspaceProduct=false.
- Composed actual feature evidence now covers27scenarios: matrix87291 passed17main +2minimal/Babel +2customization; negative-only75383 passed all6refusals, terminal0 in112.62seconds. Current no-DEBUG classifier independently accepts all6retained captures, and current original/consumed snapshot hashes match. This is composed evidence, not a claimed single full-matrix terminal success.
- Ordinary canonical current-source build and new canonical visual/contrast inspection remain pending. These gates are not inferred from the private or composed feature proof.

## Earlier Verification History

- Latest thesis and assignment suites: 54 passed, 0 failed, 186 assertions. Thesis alone: 35 passed, 112 assertions.
- Latest strict TypeScript full import closure, including minimal-cover and Babel-scope probes: zero diagnostics, terminal exit0 (session64594). Options: strict, noEmit, skipLibCheck, resolveJsonModule, allowImportingTsExtensions, Preserve/Bundler modules and ES2023. An earlier concurrent native-compiler machine-union diagnostic is absent from the latest check; that unrelated file was not edited here.
- Actual contract target PID 175147/session 71495 was deliberately canceled for capacity relief after reaching 1187/1920 input closures; no contract test had started. Verified exact command identity before sending SIGTERM. Terminal exit143 explicitly reported cancellation and released leases. Follow-up process inspection found zero remaining matching contract commands and zero direct children of the canceled parent. Root requested this cancellation after repo lease/doctor evidence showed the parent holding approximately2872MB against2574MB reserved while the original thesis build required a3584MB child execution reservation. Direct52 tests already passed; the regular target will be rerun sequentially after the original root thesis build.
- Actual rendering target PID177409/session50782 was also deliberately canceled for capacity relief, after exact command identity verification. It was admitted just before cancellation and had reached2/1920 input closures; no compiler had started. SIGTERM produced terminal exit143 and explicitly released leases. Follow-up process inspection found zero matching rendering commands and zero direct children.
- Targets use repo run with --exclude-dependencies --output=stream. The original root thesis build remains the sole expensive thesis validation job. Root requested serialization because the older rendering request would otherwise take freed memory before the original build’s3584MB child execution. The complete unchanged rendering matrix and regular target remain required after the original build succeeds. No rendering pass is claimed; no duplicate job was launched.

## Resumed Validation Readiness

After the user resumed work, the root native build session52427/PID214878 was admitted and began graph planning while waiting for its graph claim. Reinspection confirmed that the rendering harness still imports current exported interfaces for compiled probes, isolated compiler refusals, PrintExecutionV1 and thesis inference. No interface disappearance was found and no duplicate expensive target was launched. The full rendering matrix, including minimal declarations and language restoration probes, remains pending sequential execution after root native build success.

## Files

- print/🧾️template/🎓️thesis/🧬️schema/🔣️.json
- print/🧾️template/🎓️thesis/🟦️.ts
- print/🧪️tests/🎓️thesis/{🔣️.json,🥒️.feature,🟦️.ts}
- print/🧪️tests/🎓️thesis/🖨️rendering/🟦️.ts
- print/📦️packages/🟦️typescript/🧾️template/🎓️thesis/🟦️.ts
- print/📦️packages/🟦️typescript/package.json
- print/📦️packages/🟦️typescript/📋️project.json
- print/🧪️tests/🔬️script-commands/🟦️.ts

### Class command collision review after publication 24563

The actual captured compiler refusal established that KOMA owns the lowercase dedication metadata command. Core is renaming the new rendering shorthand to makededication while retaining the distinct uppercase Dedication environment; no alias is retained. An independent read-only definition scan covered every tracked Print LaTeX class/package and the shared thesis public metadata setters, academic title commands, front/backmatter commands and environments. No additional new-command collision was found in the tracked Print sources. The intentional maketitle renewal and the existing makemainmatter hook remain separate. This source audit does not replace compilation against the actual KOMA/base packages; the next unchanged publication is the authoritative external-definition check.

The next fixture successfully reaped the earlier failed managed root r1bEEdD through the official private repository factory, as confirmed by root; only the current failed fixture's managed root remains for the same subsequent lifecycle. No raw deletion was used.

## PDF Adapter And Check Inventory

The following five PDF adapter files were changed to support the actual pinned PDF.js Glyph facts while retaining strict generic data capture:

- `🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/🧬️schema/🔣️.json`: language-neutral narrow operator-normalization declaration.
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/🟦️.ts`: operator-aware capture; only the declared absent optional Glyph slots are omitted before strict bounded capture.
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/📜️script.ts`: owned PDF worker invokes the operator-aware factory.
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/🧪️tests/🔣️.json`: language-neutral accepted/refused operator facts.
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/🧪️tests/🟦️.ts`: neutral behavior and independent actual pinned PDF.js oracle coverage.

The additional permanent source-check edit is `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️script-commands/🟦️.ts`, whose CheckScript now explicitly roots both the PDF worker and the PDF suite. The worker is loaded by URL and therefore requires explicit inclusion beyond ordinary imports. No runtime third-party API is exposed by the thesis contract.

Durable acceptance scripts retained at the ticket root are recover-publication.ts, inspect-publication.ts and pdf-oracle.ts. Generated evidence currently resides under 🗑️generated/recovery and 🗑️generated/inspection, including complete publication-proof.json, four PDF text reports, four private PDFs and eight PNGs; this is temporary ticket output and must be retired at final ticket cleanup. Runtime-private products remain distinguished from canonical live-workspace artifacts.

The new shared reader lifecycle helper/schema/six-vector fixtures/tests inventory and exact six refusal request hashes are retained in composed-acceptance.md. The current reader/refusal focused checks are14pass208assertions after temporary DEBUG removal.
