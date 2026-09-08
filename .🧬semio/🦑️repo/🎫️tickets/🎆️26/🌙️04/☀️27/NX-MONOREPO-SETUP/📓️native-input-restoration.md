# Native Input Invalidation and Restoration

The actual repository plugin and native producer, copied into an isolated ticket workspace, passed 12 Nx scenarios on darwin/arm64. A fixture-only completion counter verified real producer executions. Unrelated frontend, separate test, audit/test implementation and variant environment changes reused the cached build. Native source, embedded asset, compiler flags, compiler contract and producer implementation changes executed the producer again. Deleted outputs restored byte-identically while the compiler store was absent; rustc independently linked a consumer against the restored library and its executable printed shared-data.

| Scenario | Executions |
| --- | ---: |
| cold | 1 |
| warm | 1 |
| frontend | 1 |
| test-source | 1 |
| tooling-test | 1 |
| variant-env | 1 |
| restore-without-compiler-store | 1 |
| native-source | 2 |
| embedded-asset | 3 |
| compiler-flags | 4 |
| toolchain-contract | 5 |
| producer-implementation | 6 |

This qualifies the isolated generic Cargo producer. Conservative real-package source fallbacks and browser-specific commands remain separately tracked.
