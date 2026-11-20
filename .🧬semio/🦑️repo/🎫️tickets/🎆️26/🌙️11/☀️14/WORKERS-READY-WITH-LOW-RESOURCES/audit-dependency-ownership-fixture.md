# Dependency Ownership Fixture

A fresh run of the existing ownership index test failed in the initial cold child while computing ownership from the fixture directory. That child only imports the workspace facade. The test reaches the incremental decoder after the child succeeds.

The private fixture originally declared `pkg/*` and published `pkg/a` and `pkg/b`. A direct read of that fresh tree with the current Bun workspace facade returned `[]` for the derived native patterns. The workspace schema declares `pkg` as an opaque output directory, and the production candidate walk skips that path before reading manifests below it. The `package.json` named in the refusal is the temporary fixture manifest.

The fixture now places authored packages under `packages/` and includes a generated manifest under `pkg/output/`. Python independently expands the declared globs and filters output directories to derive the initial and changed membership. The test compares that result with the published fixture and the cold, seeded, and changed reads. It still rejects a seed that hides a real package and compares ledger entries with the Python filesystem projection. The test keeps its generated child and seed files in its own disposable case directory.

The focused test reproduced the original failure and passed after the correction with 25 assertions. Production workspace rules and checkout package manifests were outside the change.

The same four-file focused selection passed 18 tests and 273 assertions after the correction. Thirteen tests were outside the selection.
