# Portable Build Core

The command owner is `library/🏗️build/📜️script.ts` with `RepoScript`, `runOwnedBuildInvocationV1` and asynchronous `createBuildBootstrapCapabilitiesV1`. Its direct route takes `repo` before normal arguments. Default tool selection is checked into `🏗️build/🔣️policy.json`, overridable with `--tool=nx|bazel` or `SEMIO_BUILD_TOOL`. Literal arguments after `--` remain untouched.

The canonical graph and caller schemas live under `🏗️build/🧬️schema`. The graph loader discovers real workspace authority without importing an Nx runtime and publishes the admitted graph atomically to `.🧬semio/🦑️repo/⚡️cache/build/graph.json`. It includes authored/Cargo projects, test-owner projects and policy-script projects. Collection ownership and generated-directory policies bound discovery. The former primary Nx plugin entry now reexports extracted inference under `🏗️build/🕸️graph/📥️inference`.

The portable command execution contract separates execution from caching/dependencies/products. Configuration resolution merges execution environment explicitly. Native caller grants preserve original cancellation and deadline while issuing exact discovered native commands. Backend-neutral environment variables are projected into Nx only in its bootstrap adapter.

## Verification

Ran the build graph and existing build-owner test suites directly with Bun: 7 tests passed with 494 assertions. The tests use language-neutral JSON fixtures, independent Ajv schema validation and esbuild compilation. The first real graph discovery failed on an authored native-host executor filter still expecting Nx; the parent agent is correcting that adjacent contract. Full graph and live backend verification remain in progress.
