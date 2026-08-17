# Graph DSL Outside-Alphabet Test Fix Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Graph DSL SHA-256, stable across two coordinator samples: `ecd6515d4827e3026ff85328a247ee1fd99e93d8e549121c56705d3b8ed0f6f2`.
- The only pre-existing working-tree diff is the external `DiagnosticSeverity::Hint` to `Info` rename. Preserve it exactly.

## Diagnosis

The failing test says `$` is outside the shared DSL alphabet and therefore expects `GraphDslError::Lex`. The shared lexer now deliberately promotes `$` to `TokenKind::Dollar`; Jack then correctly rejects that known-but-unsupported token as `GraphDslError::UnexpectedChar('$')`. The parser is correct and the test character is stale. `?` remains outside the shared lexer alphabet and strict lexing returns the required `TextError`, surfaced as `GraphDslError::Lex`.

## Focused Fix

Writable paths:

- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs`
- one unique Terra acceptance Markdown in this ticket

Change only the outside-alphabet test's input, explanatory comment, and expected message from `$` to `?`. Do not change the lexer/parser, the promoted `$` handling, the external severity rename, Cargo/package glue, or other tests.

## Verification

Run the focused Rust test if the package script supports filtering, then:

```text
bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache
```

Require 187/187 graph tests, preservation of the unrelated dirty hunk, and scoped ordinary/cached diff checks.
