# Print Macro Staging Execution Packet

Date: 2026-09-12

## Scope

Implement the native binding established by 📓️terra-latex-implementation-binding-audit-2026-09-12.md, then extract the graph macro implementation with full production consumer closure. Read that audit, the earlier 📓️latex-package-implementation-followup-packet-2026-09-12.md, and applicable AGENTS instructions. This is the first bounded print macro lane; the other 96 contracts remain separately reviewed debt.

The canonical repository leaf is 📐️.tex under a semantic graph owner. Compiler staging maps that exact anonymous leaf to _.tex. The ASCII name exists only in disposable compiler staging, never as a general repository exception. Keep native named package/class declarations as minimal actual loader glue. Do not change authored graph styles, dimensions, opacity, vocabulary or behavior.

## Actual Production Gap Found by Coordinator

The native audit proves the loader mechanism through controlled staging, but the current production pipeline does not stage the macro library:

- source-staging/🟦️.ts strips the first emoji grapheme of every component, so 📐️.tex currently becomes .tex. It rewrites all matching component text only in .tex/.bib; .sty/.cls are copied.
- tectonic-template-compilation/🟦️.ts: publishPrintArtifact stages only document.sources from each sourceRoot. The print document catalog lists templates, content, bibliography and assets, not the shared macro library.
- compilePrintDocument sets searchPaths to the live print/🖋️latex, document work directory, output and fonts.
- tectonicEnvironment similarly injects the live latex directory into TEXINPUTS.
- Mit report documents pass a different sourceRoot to the same public publishPrintArtifact; simply adding a print-relative directory to those source lists would be wrong.

Therefore moving a macro into a canonical emoji leaf and changing only stagePrintSources will break real builds: native compilation would still load the live shim and never see its staged implementation. The production lane must materialize the shared print macro library plus the actual semantic implementation owners into an isolated compiler library root for every document invocation, and pass that staged root through the existing light/dark/panel compilation flow. Preserve caller document source roots, per-document temporary and output ownership, fonts, deterministic epoch, cancellation and timeout. Never rely on a direct emoji path, accidental extension-only lookup, live PDF publication, or a compatibility copy of the implementation.

Use the existing source-staging owner and compilation seam. If a shared library manifest is needed, make it the precise schema-first producer input authority; no guessed directory scan over unrelated TS, tests or all framework code. Declare Nx actual source inputs for print and Mit builds so moved macro bytes invalidate ordinary builds. Existing named package identities can be materialized from their required shims along with explicit implementation owners.

## Portable and Native Acceptance

Start with a portable schema/fixture covering root document → named shim → semantic graph/📐️.tex, expected stage graph/_.tex and rewritten references in .tex/.bib/.sty/.cls. Keep unchanged unrelated bytes and collision rejection before output mutation; include collision between an existing _.tex and the canonical source in the same stage directory and extensionless references. Probe actual consumed relative/full paths rather than assuming substring replacement is a TeX parser. Repeated source roots and directory/file collisions need deliberate semantics, not silent overwrites.

Use installed independent parsers/compilers only behind test interfaces. The native audit controls are retained in 📋️latex-binding-controls. Prepared Tectonic 0.16.9 and the locked cached bundle are available. Native controls must run through the actual changed production library materialization/compile seam into ticket scratch, exercise SemioGraphState and SemioGraphEdgeKind, verify an expected PDF text marker with the existing independent reader, and obtain two deterministic outputs. Preserve timeout/cancellation. Do not claim native graph drawing if only registry macros executed.

Add the precise anonymous macro source contract/context and minimal fixed glue body contract so the previous graph inventory bypass becomes an actual enforced distinction. The catalog currently treats the generic tex kind as documentation and exact fixed filenames have null kind; do not change all document prose to implementation or introduce a blanket new exception. Coordinate shared schema/discovery/normalization edits with active Sol/root workflow and Terra fixed-script audit. The existing explicit fixed-source decision only admits a contract that declares its grammar; a TeX declaration grammar must be separately constrained and tested, not mislabeled TypeScript.

Use Bun/Nx, mandatory 📜️script.ts command routing, package/project/seed/derived launch conventions. Reuse an existing route if it exposes every necessary operation; register a genuinely new executable route in launch seed and derive normally. Keep source-as-data tests pointed at their real semantic owner. No permanent source-body SHA snapshots.

## Verification and Reporting

Scope inventories must validate every new owner and its full ancestor chain. Run direct and registered focused portable tests, current print pipeline unit tests, exact native macro test and relevant declared consumer input checks. Do not compile all published documents unnecessarily or overwrite live report outputs.

Retain report 📓️sol-print-macro-staging-2026-09-12.md with exact changed-file attribution, first-red evidence, native inputs/results, generated-output semantics, and remaining macro domains. Keep authored controls. Scratch is only 🗑️generated/sol-print-macro-staging; delete only that disposable output at lane completion. Do not edit AGENTS, use modifying Git, close ticket or change goal lifecycle.

