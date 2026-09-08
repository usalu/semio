# Native Generator Prerequisites

Actual Nx 23 and Cargo passed cold generation-before-compilation, warm reuse, deleted-output restoration without the compiler store, and schema-change invalidation in an isolated workspace using the real plugin and Cargo producer. The test checks actual generator and native producer execution counters and byte-identical output restoration.

| Scenario | Native executions | Generator executions |
| --- | ---: | ---: |
| cold | 1 | 3 |
| warm | 1 | 3 |
| development-source | 1 | 3 |
| restore | 1 | 3 |
| schema-change | 2 | 4 |

This fixture qualifies ordering and artifact hashing; the separate Cargo metadata test qualifies production versus test generator selection.

The artifact-check target still reports the same eleven unresolved output contracts. It currently delegates to the general target-policy checker and does not constitute a complete artifact ownership audit. Those remaining products need concrete leaf/output refactors rather than empty output declarations or caching of their hidden pipelines.

The actual `@semio-tech/framework-schema:build` passed after deleting its Cargo-to-Bun build dispatcher and assigning code generation to Nx. The default generic Cargo target set now includes build, check and test for authored generator-only metadata as well as inferred projects. Editor coverage for the 409 additional native commands is recorded in `📓️native-editor-targets.md`.

The independent `repo:cache-verify` target passed all 15 non-Cargo scenarios after the discovery optimization. The final five-scenario native probe also passed. The latest full `repo:test` attempt was cancelled after more than 15 minutes of active test execution without completion; phase markers have been added to locate the slowdown. Earlier successful contract runs predate the final argument guard and bounded daemon diagnostics and are not a substitute for this rerun.

The diagnostic daemon-disabled `repo:test` rerun passed the full suite in about one minute of task execution, including the final native argument guard, permanent bounded daemon error test, inferred native target/editor contracts, compiler input oracles and materializer cancellation. Normal shared-daemon requests still need separate qualification. The new phase markers show inventory and later contract phases completing.
