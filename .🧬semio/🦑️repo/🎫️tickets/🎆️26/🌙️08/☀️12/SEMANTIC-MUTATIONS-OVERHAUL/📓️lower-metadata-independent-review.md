# Lower Metadata Independent Review

## Accepted Boundary

The canonical fourteen-field descriptor, typed wire enums, const validation, six-field source provenance, and metadata-only `MutationLeaf` now belong to the lower replication contract. OS command and SPR explicitly reexport the same types. This accepts the lower ownership/type boundary only; the base `Mutation` requirements, kind supertraits, aggregate source proof, registry propagation, and production owner conversion remain open.

## Executed Evidence

- `🧪️lower-metadata-build-retry.log`: actual registered `@semio-tech/framework-replication-rs:build --lib --jobs 1`, exit 0, 42.95 seconds. The first build log records a manifest path error; the derive hash dependency path was corrected before the successful retry.
- `🧪️lower-metadata-registered-final.log`: actual registered lower tests after strengthening the borrowed-generic probe, 2 passed, 211 filtered, exit 0. The live test infers metadata from a payload whose generic parameter itself is a non-static borrowed value.
- `🧪️lower-metadata-contract/🧪️root-compiler-replay-retry.log`: explicit workspace-scoped Nx compiler harness, 3 cases, 0 failures. Retained run `🧪️lower-metadata-contract/🧫️run-3m9r5p` includes each source, compiler argv/output, runtime output, and result JSON. The real freshly built protocol pair is `9326ffd3ad988ba0`, serde pair `9726de5488b8f586`; each matching rlib/rmeta pair is supplied together. The positive client ran `insert-page:42`; missing descriptor and missing provenance independently produced completed `E0046` failures.

The first registered test command used the wrong filter and selected zero tests. Its retained `🧪️lower-metadata-registered.log` is not passing test evidence. The corrected intermediate retry passed two tests before the final borrowed-generic strengthening; only the final log supports the current test source.

The first Nx compiler replay did not execute the harness: `bun nx exec` ran through a package lifecycle named `nx`, which Nx interpreted as a missing workspace target. Calling the existing root router directly (`bun ./📜️script.ts nx exec --projects=workspace ...`) executed the intended command once. Both attempts are retained.

## Harness Review

The coordinator required and verified removal of destructive run cleanup, exact matching artifact pairs, retained compiler/runtime evidence, completed process status checks, runtime exit-zero checks, and a genuinely non-static generic parameter. The test fixture is validated through Ajv before rustc executes the actual contract. Manual metadata literals in the compiler fixture are deliberate trait-shape probes, not approved production source provenance.

No full lower suite, OS facade suite after this move, or monorepo-wide acceptance is claimed here.
