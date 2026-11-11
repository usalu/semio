# Source Admission Refusals and Boundary Errors

## En1991 refusal cause

The en1991 inferences source was genuinely unbalanced, so the norm source was wrong and the tokenizer was right. `rustc` and `rustfmt` agreed: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧷️wave3-rows/🦀️.rs` line 58 column 20, the `report.push(` in `check_fatigue_model`, was missing a closing `)`. The file has since been fixed by someone else (modified 08:27). I did not edit norm code.

The tokenizer (`rustTokens`) lexes every tricky lexeme correctly. Fixtures covering braces in strings, chars, byte strings, raw strings, lifetimes, nested block comments, doc comments and format strings all agree with tree-sitter Rust.

## Tokenizer and refusal

- `📚️compiler/📖️syntax/🦀️rust/🟦️.ts`: new `rustDelimiterFaultV1(source)` finds the first delimiter fault in the compiler's way (innermost open delimiter is the culprit; kinds `unexpected`, `mismatched`, `unclosed`; line and column in code points). Exported through the package entry. Schema def `delimiterFault` in `🔣️.json`.
- Fixtures `delimiters` (JSON, language-agnostic) in `🧪️tests/🧫️fixtures/🔣️.json`, checked against the schema and against tree-sitter `hasError`.
- `selectRustSourceV1(source, cfg, file)` now takes the file and throws typed refusals `process.source.delimiter-unclosed|unexpected|mismatched` with file, line and column (and the closer position for mismatches). The matching text exists in en and de.
- Source-test fixtures `refusals` (schema-extended) are checked against `rustc -Zunpretty=normal` opener positions.

## Typed refusals in source admission

All plain `Error` throws in `📥️source/🟦️.ts` and `🎟️admission/🟦️.ts` are now `refuseProcessV1` refusals (`source.*` keys in `🏃️process/📖️texts/🔣️.json`, en and de). Typed refusals pass through the crate wrapper unchanged; other errors become `source.crate-refused`. The source growth breach is `source.boundary-exceeded`.

## Boundary errors

- `reportBoundaryErrorV1` (progress, exported through the package entry): typed refusals render in the resolved language with their exit code; `CargoLockRefused` renders its bilingual text plus the bounded resolver stderr, exit 1; everything else becomes `error.internal` ("Internal error: {message}" / "Interner Fehler: ..."), exit 1. No stack or source excerpt is printed. No debug env var exists in the repo, so none was added.
- Wired into the build script main, the build door main and `reportCommitErrorV1`.
- Tests: progress boundary rows in `📣️progress/🧪️tests/🔣️.json` (cargo lock, error, thrown string).

## Tests (managed Bun)

Passing: rust syntax 8, source 47 of 48, progress 20, error 10, catalog 25, texts 3, cargo-lock 13, native history syntax 2. The one source failure ("shared engine registers its existing canonical slot implementation") reads a shared engine `🦀️.rs` and is unrelated to this change. The admission suite fails at "Original parent Script process invocation required", an environment precondition untouched by this change.
