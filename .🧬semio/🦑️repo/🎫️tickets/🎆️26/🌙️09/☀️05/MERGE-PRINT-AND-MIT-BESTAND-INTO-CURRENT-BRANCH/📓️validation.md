# Merge Validation

## Runtime Environment

macOS ARM64, Bun 1.3.14, Nx invoked with Node; each Nx target runs its Bun script router. All checks used `--skip-nx-cache`, `NX_DAEMON=false`, and ticket-owned Nx workspace data/output directories. A fresh graph via Bun encountered a Unicode path duplication in the shared workspace; the repository uses Node for the Nx process, which resolved it. Root discovery also encountered an unrelated concurrent taxonomy edit; this is not counted as a passing check.

## Uncached Contract Tests

Command: `node node_modules/nx/bin/nx.js run-many --projects=@semio-tech/print,@semio-tech/mit-bestand-bericht --targets=test --parallel=2 --skip-nx-cache`.

```text
[DEBUG] report zwischenbericht: 20 source documents resolved
[DEBUG] report forschungsbericht: 10 source documents resolved
[DEBUG] report kompaktbericht: 1 source documents resolved
[DEBUG] Akteursnetz valid: 798 nodes, 444 edges, 14 programs
[DEBUG] print: viz coverage 1966/1966 leaves, API 17/17
[DEBUG] print: unit tests passed
 NX   Successfully ran target test for 2 projects
```

Language-neutral JSON fixtures validate staging, extensionless appendix references, collision handling, document families, and taxonomy parsing. The existing third-party MarkdownIt parser independently reproduced all 1,966 terminal taxonomy identifiers. The report test resolves all literal appendix inputs and validates the actor ledger.

## PDF Inspection

PDF.js opened the 12-page standard report, 205-page research report and six-page compact report. Canvas rendering and text extraction succeeded for sampled pages. Reviewed report page 5 and compact page 2: border/window layout, German/English text, and page chrome rendered. Report photo test color blocks are intentional fixture assets. Research report contains incoming draft/template prose, which is preserved.

## Other Checks

Both VS Code launch configuration files parse with jsonc-parser. Scoped `git diff --check` passed. `bun install --lockfile-only --ignore-scripts` completed. The generated lock also reflects the current concurrent 2D workspace rename; unrelated local edits were preserved.

## Imported Defects Repaired

- Added the missing distribution chart package required by the incoming loader; the distribution gallery builds in both themes.
- Corrected the top-level text-kind registration parameter and protected font tokens during TikZ option expansion; the API fixture now exercises cell, module, chip, label, title and value.
- Resolved shared report appendix references and compiler-safe staging for semantic filenames, including extensionless TeX inputs.
- Preserved current font search/token generation and removed obsolete font Path options.
- Report watch mode observes shared appendices and bibliography/assets, excludes generated outputs and serializes rebuilds.
- Applied the existing build-class subprocess budget to large TeX documents after the interim report exceeded the generic command timeout.

## Completion

Final PDF build outcomes and cleanup are recorded in the completion report.

### Visualization Text Output

PDF.js independently extracted all six text kinds from the four-page API document using the expected strings in `🧫️merge-contract.json`. The text-kind page was rendered with Canvas and visually inspected. Light and dark API compilation reached PDF output; final command completion is recorded separately.

### Source Preservation

Compared all 687 incoming research files byte-for-byte against their pending Neo4j intake copies: zero mismatches. The existing graph and canonical `_knowledge` were not modified by this merge. Concurrent actor asset renames were preserved; actor validation now snapshots stored fragments before its long computation, and rendering refuses to write if the source ledger changed during computation.

### Interim Report and Repeatability

The final `build-zwischenbericht` Nx target exited successfully after both print passes, producing a 16.38 MiB PDF. The API PDF rebuilt with the same metadata-normalized content hash: `16c6802bfdbb760c8a038eb3a51669717cd6f58a3fe1e5b0150b4571838c853a`.

PDF.js opened the completed 192-page interim report. Rendered pages 1, 10, 150 and 192; inspected page 150 actor/source tables. Native text extraction confirmed report title, results, actor tables and bibliography content.

### Final Source Contract Rerun

After the validation/write guard changes, both uncached project tests passed again:

```text
[DEBUG] report zwischenbericht: 20 source documents resolved
[DEBUG] report forschungsbericht: 10 source documents resolved
[DEBUG] report kompaktbericht: 1 source documents resolved
[DEBUG] Akteursnetz valid: 798 nodes, 444 edges, 14 programs
[DEBUG] print: viz coverage 1966/1966 leaves, API 17/17
[DEBUG] print: unit tests passed
 NX   Successfully ran target test for 2 projects
```

### Shared Appendix Bibliography Closure

Final PDF log review exposed a missing scaling citation. A new all-report citation closure check failed before the repair and identified two absent bibliography entries (`rakhshan-components-reuse-review-2020`, `sivers-froehlich-fivet-digital-market-solutions-2022`). Both entries already existed in the supplied research bibliography and were copied into the interim bibliography. The uncached report test then passed; the report is rebuilt to verify the BibTeX result. Existing Anta font-shape substitutions and under/overfull box warnings are reported separately from undefined citations.

### Final Research Build and Both-Theme Repeatability

The final `build-forschungsbericht` target completed successfully with shared source staging and actor fragment snapshot validation. Its final log contains no undefined-reference warning. Both light and dark API PDFs have identical metadata-normalized hashes across separate complete builds.

### Completed Report Family

| Report | Pages | PDF bytes |
|---|---:|---:|
| zwischenbericht | 192 | 17735496 |
| forschungsbericht | 205 | 15689099 |
| kompaktbericht | 6 | 235808 |

All three targets completed successfully. The final interim and research logs contain no undefined-reference warning. BibTeX emitted both repaired entries in the interim `.bbl`.

### Completed Standard Template Suite

The uncached `@semio-tech/print:test-long` target exited with status 0 and reported all 12 template PDFs built. PDF.js independently opened all 12:

| PDF | Pages |
|---|---:|
| 🎓️paper-dark.pdf | 4 |
| 🎓️paper.pdf | 4 |
| 📋️report-dark.pdf | 12 |
| 📋️report.pdf | 12 |
| 📑️forschungsbericht-dark.pdf | 28 |
| 📑️forschungsbericht.pdf | 28 |
| 📝️kompaktbericht-dark.pdf | 4 |
| 📝️kompaktbericht.pdf | 4 |
| 📰️flyer-dark.pdf | 2 |
| 📰️flyer.pdf | 2 |
| 🚧️zwischenbericht-dark.pdf | 8 |
| 🚧️zwischenbericht.pdf | 8 |

The closing uncached print/report tests passed. Both project-schema paths resolve, and the final scoped whitespace check passed. All imported destinations in the source manifest exist. No native Windows/Linux/devcontainer run or exhaustive 162-PDF gallery build is claimed.
