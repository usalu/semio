# Rust Companion Course Repair

The five assigned Rust companion/test-support source files now use the actual Cargo library `course_schema`. The shared lifecycle builder is `course() -> Course`, emits `semio.course/v1`, supplies `LearnerContext.course`, and reads the neutral lifecycle fixture's `course` key. The companion view import is `CourseView`. No compatibility aliases were added. Child quiz documents retain `semio.quiz/v1`.

Actual schema declarations were inspected before editing. The repair also supplies the optional fields that are required when constructing Rust values: `Course.topics: None`, `ClassificationTask.pool: None`, `SortingTask.pool: None`, `MatchingTask.pool: None`, and `Command::StartRun.origin: None`.

## Changed Files

- `🧰️framework/🛍️products/📚️course/🧪️tests/🧷️companion-support/🧾️lifecycle/🦀️.rs`
- `🧰️framework/🛍️products/📚️course/🧪️tests/🧷️companion-support/🃏️sheet/🦀️.rs`
- `🧰️framework/🛍️products/📚️course/🧪️tests/🎞️companion-runs/🧾️lifecycle/🦀️.rs`
- `🧰️framework/🛍️products/📚️course/🧪️tests/🎞️companion-runs/👁️views/🦀️.rs`
- `🧰️framework/🛍️products/📚️course/🧪️tests/🥣️companion/🃏️sheet/🦀️.rs`
- This report.

## Verification

A source contract diagnostic ran before editing and reported 11 violations: five stale schema-library references; missing Course helper, schema string, fixture key, topics field and origin field; and missing pool fields. The same checks ran after editing and reported zero violations. The green diagnostic also parsed the existing language-neutral learner lifecycle fixture through PowerShell's JSON parser and confirmed the independent Course and child Quiz schema discriminators. A search across Course Rust test sources found no remaining `quiz_schema`, `Catalog`, `CatalogView`, `catalog()` or `vectors["catalog"]` matches. Read-only `git diff --check` passed for all five files.

Edits used PowerShell with `ErrorActionPreference = 'Stop'`, UTF-8 without BOM, and a per-file before-image concurrency guard. One initial mutation attempt stopped after updating the lifecycle helper because the sheet file uses LF line endings; the assertion prevented a partial sheet write. The remaining four files were then edited with line-ending-aware replacement, and the lifecycle helper was normalized back to LF.

No Rust compiler or Cargo build was executed by this repair agent. Source diagnostics do not establish compilation or runtime readiness. The shared ticket records the existing canonical native-build Windows/capacity blocker. The domain unit-test agent is independently checking imported symbols and struct fields with a repository tokenizer and third-party tree-sitter/TOML tooling. An additional independent Ajv 8.20.0 diagnostic compiled the actual normative JSON Schema and validated the neutral lifecycle fixture: one Course and both child Quiz documents passed (3 cases). This verifies the fixture contract, not execution of the Rust helper.

The independent expanded source diagnostic subsequently passed across 26 current Rust files and 285 closed struct/enum literals, including these five companion files. It confirmed actual schema imports, grouped/direct support imports, current factory imports and required literal fields against the repository tokenizer and third-party tree-sitter/TOML tooling. The domain agent records this in Rustunitrepair.md and the temporary rust-unit-final-green.json. This is source-only evidence, without native compilation.
