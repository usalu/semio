# XLSX Natural Draft Fidelity

The three natural spreadsheet editors currently parse every accepted draft again, including unchanged display text. An unchanged numeric-looking inline string becomes a number, an unchanged shared-string reference becomes an inline string, and accepting an unchanged formula removes its cached value. The optimistic revision currently covers the shared-string index but omits the referenced text, so a draft can overwrite a concurrent shared-string edit without a conflict.

The repair preserves the exact typed cell when the submitted text equals the current display, and includes the addressed shared-string content in the revision. These rules apply to base, strict, and transitional editors through shared helpers. A language-neutral fixture covers ambiguous literal strings, cached formulas, shared references, ordinary values, and stale referenced text. Native laws compare saved values and formula source with Calamine. Validation results will be recorded after execution; no native pass is claimed yet.

Full component17 ended before linking on two missing DOCX `OpcPart` imports; its owner has repaired both. Retained-clone native6 has emitted test-source authority/type errors and remains under its owner’s process supervision. These builds do not establish current runtime acceptance.

## Validation Checkpoint

The existing `@semio-tech/stdio-xlsx:test` Bun/Nx target passed in 20.7 seconds. It built the package, tested its public export, validated the windowed-viewer fixture, and validated all ten draft cases plus the shared-string conflict case with Ajv. Its generated log is `xlsx-draft-fixture-typescript-current-1.log`. All edited Rust sources parse through rustfmt, fixture include paths resolve, and the relevant diff check passes. Native3 remains running; the three actual editor behavior laws and independent Calamine value/formula law have not yet reported a runtime result.
