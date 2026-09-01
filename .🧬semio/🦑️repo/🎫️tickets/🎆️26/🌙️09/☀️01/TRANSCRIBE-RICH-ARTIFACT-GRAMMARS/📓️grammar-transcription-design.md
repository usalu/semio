# Rich Artifact Grammar Transcription

## Scope

Transcribe the twenty-seven hand-crafted `📖️component.grammar.semio` grammars identified by
`STUBS-AND-PLACEHOLDERS-COMPLETION` into their sibling ANTLR4 `🅰️component.g4` and ISO-14977
`🔤️component.ebnf` representations. Preserve each existing grammar name and `schema` identity
literal.

## Method

For every leaf, retain the common `schema` envelope and translate each normative Semio production
into equivalent parser productions and lexical terminals. ANTLR grammars use parser rules for
structural productions and uppercase lexer rules for tokens. EBNF grammars use ISO-14977
concatenation, alternation, option, and repetition forms. Both representations remain
self-contained and describe the same accepted document language as their Semio sibling.

## Validation

The existing M5 fixture sweep validates only `component.grammar.semio`. This ticket will inspect
whether the repository has a distinct ANTLR/EBNF check. If none exists, it will add a focused,
language-agnostic structural conformance test that detects remaining opaque-envelope stubs and
checks grammar identity plus production coverage across all twenty-seven leaves.

## Findings

The requested fifty-four transcriptions are already present in the checkout (commit
`67fb4216b2c1991518be79a1e032408f5bcb9327`, 2026-09-01). They are rich mirrors rather than the
opaque envelope described in the originating ticket: every Semio production has a camel-cased
ANTLR parser rule and a space-cased ISO-14977 production. The supplied layout-diff path used
`🏅️标准`; the actual leaf is under `🏅️standards` and was included in the audit.

No repository ANTLR4 or ISO-14977 parser, compiler, or test target exists. M5 deliberately
recognizes only the normative Semio grammar against real fixtures; extending it to parse inert
interoperability mirrors would couple runtime fixture conformance to a second pair of parser
implementations and is the wrong boundary. A separate mirror-level check is appropriate whenever
ANTLR/EBNF tooling is introduced.

## Results

The ticket-local language-agnostic audit checked all 27 normative source files and both sibling
mirrors (54 files total). It verified production coverage, the preserved grammar identity across
the ANTLR `DOCUMENT` and EBNF header/comment forms, and the absence of the old opaque-envelope
body/payload shape: **0 failures**.

`bun nx run @semio-tech/framework-os-kernel:test --args='--features
dsl-fixture-sweep-full m5_handcrafted_grammar_conformance'` was invoked for the normative side,
but Cargo remained blocked on the shared package-cache and build-directory locks before compilation
or test execution. The process was cancelled after four minutes to avoid adding another blocked
job. No M5 pass or failure is claimed.
