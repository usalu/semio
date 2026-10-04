# DSL Positioned Refusal Caller Prerequisites

The measured prerequisite input is `🗑️generated/root-dsl-text-error-current-diagnostics.txt` (116 diagnostic blocks). Scope is canonical framework DSL Lexer, Grammar and Idiom, including their necessary unit callers. No Cargo or native assertions were executed in this lane.

The actual diagnostic owner exposes `TextError::new(kind, message, span)`, `expected(kind, message, span, expected)` and `from_value_error(error, span)`. Current readback already contained the production constructor ports throughout the three measured modules, plus `TextError { span: rebase(error.span), ..error }` in grammar segment lexing. Those concurrent changes were preserved and are not attributed to this repair.

## Authored Changes

- `🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🦀️.rs`: exact repetition zero remains `InvalidValue`; a positive count exceeding the authored 1,024 expansion ceiling, including an integer too wide for `usize`, is `WorkLimit`. An absent explicitly requested fragment registry owner is `UnsupportedOwner`. Syntax and numerical protocol-domain refusals retain `InvalidValue`.
- `🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🧪️tests/🔬️unit/🦀️.rs`: added `positioned_refusals_preserve_syntax_work_and_owner_categories` before changing these categories. It asserts a delegated lexer error after an alternation retains line 3 / column 14, zero repetition is malformed, 1,025 and an oversized integer hit the work ceiling at line 3 / column 7, and missing fragment registration preserves owner refusal.
- `🧰️framework/🔨️modules/🗣️dsl/🗣️idiom/🧪️tests/🔬️unit/🦀️.rs`: repaired the remaining old two-argument greeting parser fixture constructor to explicit `InvalidValue` with its original source position.

No message classifier, conversion shim, old constructor overload or changed syntax admission was added. Span rebasing forwards the complete child error, including its category and expected token.

## Verification

`rustfmt --edition 2021 --config skip_children=true --emit stdout` parsed all three production modules and all three associated unit modules successfully (six exit-zero receipts). Parsed output is retained under `🗑️generated/dsl-positioned-refusal-{0..5}-parsed.rs`; source files were not reformatted by this check. This is parser-only verification, not type checking or runtime proof. Root retains the sole native execution lane and must execute the staged category law and current compiler prerequisites.
