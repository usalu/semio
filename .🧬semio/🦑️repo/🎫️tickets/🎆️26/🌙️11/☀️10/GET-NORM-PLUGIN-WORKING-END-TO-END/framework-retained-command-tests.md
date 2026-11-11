# semio-plugin-retained-command test migration

Crate: `🔌️plugin/🧵️retained-command/📦️packages/🦀️rust` (root workspace member). Test targets: lib test and `mounted-ui-copy`.

## Baseline
`check -p semio-plugin-retained-command --tests --keep-going`: 11 errors, all in the lib test and `mounted-ui-copy`.

## Changes
- `🔄️full-operation/🦀️.rs` `publication_failure_tests`: `PublicationFailureCustody` drains through `OriginalJobFixtureCaller` + `close_original_effect_step` (a bare `EffectBank` turn lacks the original producer invocation and `ControlledRetirement::step` refuses); each grant is `plugin_demand_grant(close_demands(BODY))`; the denied turn is the quote with `maximum_items:0`; decoder and caller retired at the end.
- `🔄️full-operation/📃️mounted/🖼️ui/🧪️tests/🦀️.rs`: removed its own `#[global_allocator]` (conflicted with `semio_framework_value_testing_allocator`, linked through the job testkit); closes use `close_original_effect_step` with `RetainedAllocationEffect`; `retire` drives `ControlledRetirement::step(body, effect)`; the transfer closure pins `ValueError`.
- `🧵️retained-command-job/🦀️.rs`: `source_construction` is `pub(super)`; `🧯️raw-allocation-close-oracle` sets it.
- `Cargo.toml`: dev-dependency `semio-framework-value-testing-admission`.

- `mounted-ui-copy` denied-grant check: `advance` re-quotes its demand with the grant's copy bytes as body, so a grant one byte short legitimately re-quotes a smaller funded step. Denied grants are now those whose own re-quote is unfunded (plus zero items). Test assumption fix, not a production bug.

## Status
- `check -p semio-plugin-retained-command --tests`: 0 errors.
- `test -p semio-plugin-retained-command`: lib 7 passed, `mounted-ui-copy` 2 passed, doc-tests 0; all green. The command then exited 1 with "Internal error: Original resource loan deadline exhausted" after doc-tests (infra, 822 s; unrelated to the crate).
- No production bugs found in this crate.
- Logs: `🗑️generated/rc-tests-check.log`, `rc-tests-run.log`.
