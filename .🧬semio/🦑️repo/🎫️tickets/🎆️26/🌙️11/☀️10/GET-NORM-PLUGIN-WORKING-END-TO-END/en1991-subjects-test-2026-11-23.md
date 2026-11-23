# en1991 subjects test (2026-11-23)

## Cause
`example_subjects_unit::committed_example_assets_decode_to_their_named_subjects` failed on one field: `cases`. The subjects (via `En1991Snapshot::example()`) carry the row `case-e11-04` (E11.04); the committed DSL assets had an empty `cases` block. The assets were stale. A second drift: the `fatigue-models` header still said `assumed-axle-load:NUM assumed-udl:NUM`; the current schema is `QTY`.

## Fix
Regenerated the DSL and pack twin of `de-office-compliant` and `multi-fail-noncompliant` from the named subjects with the family encoders (`encode_en1991_dsl`, `encode_en1991_pack`), driven by a temporary env-gated test (removed afterwards; script `regen-en1991-assets.sh`). Assertion unchanged. The `bridge-showcase` asset is not covered by this test and was not touched.

## Changed files (under `en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/`)
- `🏢de-office-compliant/🏢de-office-compliant/🗣️.dsl.semio`, `🏢de-office-compliant/🎒️.pack.semio`
- `⚠️multi-fail-noncompliant/⚠️multi-fail-noncompliant/🗣️.dsl.semio`, `⚠️multi-fail-noncompliant/🎒️.pack.semio`

## Result
`test-family-examples.sh 🏋️en1991`: 3 passed, 0 failed.

## Rebuild
Yes: these assets are `include_str!`-embedded by the en1991 examples crate, which the en1991 aggregate crate depends on, so `norm-eurocode` needs a rebuild.
