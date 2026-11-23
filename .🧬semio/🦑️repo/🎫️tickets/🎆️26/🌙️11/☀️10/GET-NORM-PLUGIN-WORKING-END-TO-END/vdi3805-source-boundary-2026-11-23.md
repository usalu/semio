# VDI 3805 Examples Source Boundary (2026-11-23)

## Cause
The refusal "33 of 32 files" is the closed Rust source-admission boundary, not a bug. The 32-file / 262144-byte / 4000-active-line values are the ordinary policy in the `[package.metadata.semio.rust-source-boundary]` of the process modules (for example `🧰️framework/🔨️modules/🏃️process/🧭️routing/🏠️workspace/📦️packages/🦀️rust/Cargo.toml`); a unit may only lower them (`effectiveCargoSourceBoundaryV1` takes the minimum). Admission code: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`; texts: `📖️texts/🔣️.json` (`source.crate-refused`). The limit is a deliberate architecture rule, so it was left untouched.

The positive examples crate `semio-s-artifact-norm-vdi3805-examples` mounted 13 example modules, 13 mounted test files, the lib root and the `include_str!` assets, which exceeded 32 once the test mounts (and a demo compliance test) were added.

## Fix
Same pattern the negative examples already use (`examples-failing` + `failing-blatt-4-to-19`): a sibling crate owns part of the example modules, the original crate re-exports them so consumer paths (`semio_s_artifact_norm_vdi3805_examples::blatt_N::source()` in the editor) are unchanged.

- New crate `semio-s-artifact-norm-vdi3805-examples-blatt-3-to-8` mounting `blatt_3..blatt_8` from their existing directories (no file moves).
- `semio-s-artifact-norm-vdi3805-examples` keeps blatt 16, 19, 22, 53, 60, demo, nonconforming and `pub use`s blatt 3..8 from the new crate. Its dev-dependency on `semio-s-plugin-norm-compliance` stays (used by the demo test).
- Workspace member, workspace dependency, `Cargo.lock` entries and the package router crate list were updated. No limit raised, no compatibility layer.

## Changed files (under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/`)
- `Cargo.toml`, `Cargo.lock`
- `📦️packages/🦀️rust/📜️script.ts`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🦀️.rs`
- `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📦️packages/🦀️rust/Cargo.toml`
- new: `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-3-to-8/🦀️.rs`
- new: `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-3-to-8/📦️packages/🦀️rust/Cargo.toml`

## Test result
`repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-vdi3805-examples -p semio-s-artifact-norm-vdi3805-examples-blatt-3-to-8 --offline` passed (exit 0): source admission accepted both crates; 15 tests passed in the examples crate and 6 in the new crate, 0 failed. Compliant examples assert zero failing checks; the demo test additionally asserts passing structure and KVS checks. The failing-example verdict tests (claimed checks fail, every failing check carries remedies) live in the `examples-failing` crates, which were not part of this run. Log: `🗑️generated/examples-vdi3805-fix.log` (delete when closing the ticket).

## Follow-up: failing fixtures blatt-5-fail, blatt-16-fail, blatt-19-fail
Symptom: `test-family-examples.sh 🏭️vdi3805` gave 28 passed, 3 failed ("expected ≥2 fails").

Cause per fixture (all three the same, asset not engine): the engine has no mandatory fields for sheets 5, 7, 8, 9, 10, 16, 19, 22, 29, 53, 60..66 and 100 (`sheet_mandatory_keys` returns an empty list; their sheet records are reported `unverified`, not failed). The three assets differed from their compliant twins only by a wrong header `record-count=0`, which yields exactly one failing check, so they no longer violated what they claim (at least two failing checks, each with an applicable remedy). The engine is unchanged and consistent (blatt-8-fail, blatt-53-fail and blatt-60-fail already use two violations).

Fix: in `blatt-5-fail`, `blatt-16-fail` and `blatt-19-fail` the mandatory identity record 100 was removed from the product (Part 1 structure violation, as in blatt-8-fail) in addition to the wrong `record-count`. The existing tests (at least 2 failing checks, every failing check has an applicable remedy) are unchanged and now pass. No twin or reference snapshots reference these assets.

Changed files (under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/`):
- `blatt-5-fail/🗣️.dsl.semio`
- `blatt-16-fail/🗣️.dsl.semio`
- `blatt-19-fail/🗣️.dsl.semio`

Result: `T/test-family-examples.sh 🏭️vdi3805` exit 0, 31 passed, 0 failed across `examples`, `examples-blatt-3-to-8`, `examples-failing` and `failing-blatt-4-to-19` (log `🗑️generated/examples-vdi3805.log`; delete when closing).
Note: an earlier rerun was blocked by other agents' in-flight edits outside this family (`semio-plugin-decl-app` was 583 bytes over its 262144-byte boundary, then `semio-plugin-guest-contract` failed to compile on a temporary `DbgCause` instrumentation); both resolved on their own, nothing of mine touched them.
