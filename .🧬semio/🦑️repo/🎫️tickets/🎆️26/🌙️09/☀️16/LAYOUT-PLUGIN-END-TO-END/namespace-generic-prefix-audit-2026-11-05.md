# Generic Namespace Prefix Audit

Read-only search of tracked build graph/native-artifact Source plus repository-wide code prefix predicates and anchored JSON regexes. No edits or new gates.

## Demonstrated Causal Residual

`🧰️framework/🔨️modules/🖱️ui/🧬️schema/📦️crate-boundaries/🔣️.json:30` requires `crates[].package` to match `^semio-framework-[a-z0-9-]+$`. Current adjacent fixture `🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` contains canonical renamed packages `semio-ui-surface-identity`, `semio-ui-prepared`, `semio-ui-prepared-staging`, `semio-ui-draw-contract`, etc. These cannot satisfy the old regex.

Actual consumer `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️crate-boundaries/🟦️.ts:14–16` compiles this schema with Ajv and asserts the full corpus validates. Declared owner is `@semio-tech/ui-rs:test-crate-boundaries` in `📦️packages/🦀️rust/📋️project.json:339`; script dispatch52–54 runs the exact file. This is a concrete test/schema-owner namespace defect. Update the schema's canonical family predicate to match the newly authored UI corpus, preserving closed validation. No actual failure invocation was launched by this audit.

## Production Filters

No unqualified old-family prefix predicate found in searched production build-graph/native-artifact consumers. Remaining direct Cargo names in build mode constraints are hash/pack/replication families outside the rename. `library/🎛️commands/✅️verify/🦀️rust-warnings/🟦️.ts:24` has narrower semio-framework-artifact/semio-s-artifact classifiers and role-based plugin/extension admission; these were not renamed families and do not demonstrate exclusion of newly renamed OS/UI/plugin-support/renderer/font names. Explicit unchanged build project IDs are not Cargo package filters.

The observed product1437 input closures versus former65 do not by themselves prove incorrect filtering or a regression. This audit found no production causal explanation for that count change.
