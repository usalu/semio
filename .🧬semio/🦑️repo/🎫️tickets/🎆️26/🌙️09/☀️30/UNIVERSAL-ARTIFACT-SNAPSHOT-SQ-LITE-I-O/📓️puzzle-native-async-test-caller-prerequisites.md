# Puzzle Native Async Test Caller Prerequisites

The current Root registered Puzzle3d/Puzzle5d native attempts failed before assertions because authored schema tests called an optional runtime dependency without enabling app assembly. These are authored domain test sources, not generated tests. The existing declared `semio-framework-async-macros` provides a dependency-free test executor. Converted only affected schema tests to `#[semio_framework_async_macros::async_test] async fn` and direct `.await`; all existing assertions, fixtures, async law helpers and explicit standalone-store retirement remain. No runtime dependency, facade, Cargo command or producer change was introduced.

## Changed Sources

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs`: 3 awaited operations in 1 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs`: 3 awaited operations in 1 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`: 2 awaited operations in 1 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: 47 awaited operations in 9 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📍️flat-position/🧪️tests/🔬️unit/🦀️.rs`: 13 awaited operations in 4 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs`: 3 awaited operations in 1 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs`: 3 awaited operations in 1 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`: 2 awaited operations in 1 tests.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: 41 awaited operations in 8 tests.

## Validation

Exact old compiler failures are retained in `🗑️generated/root-native-owner-current.log` (Puzzle3d around 62300–62506; Puzzle5d around 68500–68612). Current source readback has no optional runtime resolver in schema tests. Native execution remains pending Root’s sole Cargo lane; no feature RED or passing assertion receipt is claimed. App-assembly production resolver users retain their existing optional feature dependency.
