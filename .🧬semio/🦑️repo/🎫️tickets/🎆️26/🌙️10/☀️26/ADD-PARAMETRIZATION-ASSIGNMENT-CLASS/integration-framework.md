# Course Assignment Integration

The reusable course contract now represents a course-owned PDF with a required explicit producer target. JSON Schema, TypeScript declarations and owned validation, and Rust declarations and owned validation agree on the course namespace and bounded scoped target syntax. Old print-template references are rejected rather than adapted.

Generic assignment vectors now use Sample Course / Sample Topic. The architecture exercise remains only in the teaching domain. The Rust companion suite consumes the same language-agnostic vectors as TypeScript and the independent AJV schema oracle.

## Verification

- Test-first baseline: the new valid course-owned fixture failed against the original print-only contract.
- Focused assignment suite: 10 passed with AJV / owned-validator / projection agreement.
- Full generic course TypeScript suite: 595 passed. The first parallel run hit the existing 5 second exhaustive Unicode timeout; repeating with a 30 second test budget passed without changing the tests or implementation.
- Generic print assignment suite: 19 passed, 74 assertions.
- Generic page chrome: header, joined borders and original logo decoding passed against AJV, mathjs and sharp.
- Rust shared-fixture verification via repo run @semio-tech/course-rs:test --exclude-dependencies -- assignments was blocked before target execution by repository source admission: ENOENT lstat .cache/b/l/fixed.dir. The existing Rust suite consumes the same neutral cases, including projection and invalid-metadata checks.

## Generated Outputs

Removed the three task-created root output/pdf PDFs and the now-empty directory. Added /output/pdf/ to .gitignore. Authored course content is under the Parametric topic; build receipts and final published PDFs belong to ignored managed build storage.

Git index changes are prohibited by the user instructions. Pre-existing staged additions of output PDFs and the previous template paths remain an index hazard even after their files are removed from the working tree; no Git mutation has been performed. See integration-audit.md for the path audit.

The generic Print library no longer selects the university footer globally. Both architecture assignment languages declare the existing optional LUH logo explicitly; the generic header/logo fixtures select it explicitly as well. The page-chrome feature now describes generic exercise/topic labels.
