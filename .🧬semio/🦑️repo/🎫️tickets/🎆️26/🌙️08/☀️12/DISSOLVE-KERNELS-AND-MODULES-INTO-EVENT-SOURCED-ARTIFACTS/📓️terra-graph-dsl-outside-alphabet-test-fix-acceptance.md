# Terra Graph DSL Outside-Alphabet Test Fix Acceptance

## Scope

- Updated only the two stale outside-alphabet test contracts in `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs`: their explanatory comments, inputs, and expected diagnostics now use `?` rather than `$`.
- Did not change the parser, shared lexer, promoted `$` token handling, package/Cargo glue, or unrelated tests.
- Preserved the pre-existing external `DiagnosticSeverity::Hint` to `Info` hunk exactly.

## Preflight

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Target pre-edit SHA-256: `ecd6515d4827e3026ff85328a247ee1fd99e93d8e549121c56705d3b8ed0f6f2`.
- Target-scoped ordinary diff contained only the external severity hunk; target-scoped cached diff was empty.
- Both stale tests failed before this edit: the wire test received an `Ok` value for its `$` fixture, and the Jack parse test no longer produced `GraphDslError::Lex`.

## Verification

- `bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache -- dag_from_wire_literal_rejects_unexpected_char`: passed, 1/1.
- `bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache -- parse_error_on_char_outside_dsl_core_alphabet_reports_lex_error`: passed, 1/1.
- `bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache`: passed, 187/187.
- The test invocations emitted existing warnings in the OS kernel, but exited successfully.

## Final Integrity

- HEAD remained `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Target post-edit SHA-256: `c1119b07dd5a8b321ca492f1aeccb9786379fe6c1996479b09ae2ae3860aedb9`.
- Target-scoped ordinary diff contains exactly the six `$` to `?` test-contract replacements plus the preserved external `Hint` to `Info` hunk.
- Target-scoped cached diff is empty.
- Ordinary and cached scoped `git diff --check` commands are silent.
