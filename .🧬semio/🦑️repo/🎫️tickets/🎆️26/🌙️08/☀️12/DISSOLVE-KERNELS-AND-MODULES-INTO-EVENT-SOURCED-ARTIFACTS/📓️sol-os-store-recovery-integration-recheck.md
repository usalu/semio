# OS Store Recovery Integration Recheck

## Scope

Rechecked the queued Graph tidy-tree, Graph minimum-spanning-tree, Pack field-index, Pack chunk-LRU, and 2D run-blocking semantic leases after the external OS SPR/store compilation migration stabilized. No source or configuration was edited during this recheck.

## Results

- `bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache`: pass. The OS kernel, including both Pack deletions and private DAG layout integration, compiles successfully with ten warnings.
- `bun nx run semio-framework-2d:test --skip-nx-cache`: compilation succeeds, then the fundamental nextest invocation exceeds its 15-second budget before running tests.
- `bun nx run semio-framework-2d:test-quick --skip-nx-cache`: pass, 21/21 tests. This releases the 2D run-blocking source change from its former OS SPR/store quarantine.
- `bun nx run @semio-tech/framework-graph:test-quick --skip-nx-cache`: the graph crate compiles and 94 tests pass before an unrelated graph DSL expectation fails in `parse_error_on_char_outside_dsl_core_alphabet_reports_lex_error`; 92 tests are cancelled by fail-fast. Neither removed graph symbol is referenced or reported.
- `bun nx run @semio-tech/framework-os-kernel:test-quick --skip-nx-cache`: compilation succeeds and 879 tests pass; one unrelated external store test fails, `dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing`.

## Disposition

- 2D run-blocking deletion: integration green through the package quick target.
- Pack field-index and chunk-LRU deletions: compilation green; quick suite remains blocked by one unrelated store behavioral test.
- Graph tidy-tree and minimum-spanning-tree deletions: compilation green; graph quick suite remains blocked by one unrelated DSL behavioral test.
- The former SPR/store API compilation blocker is resolved. Remaining test failures are owner-specific behavioral drift and stay quarantined from these leases.
