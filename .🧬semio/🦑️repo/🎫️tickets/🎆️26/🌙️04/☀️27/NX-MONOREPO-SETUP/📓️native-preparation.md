# Native Generator Prerequisites

Actual Nx 23 and Cargo passed cold generation-before-compilation, warm reuse, deleted-output restoration without the compiler store, and schema-change invalidation in an isolated workspace using the real plugin and Cargo producer. The test checks actual generator and native producer execution counters and byte-identical output restoration.

| Scenario | Native executions | Generator executions |
| --- | ---: | ---: |
| cold | 1 | 3 |
| warm | 1 | 3 |
| development-source | 1 | 3 |
| unrelated-javascript | 1 | 3 |
| package-metadata | 1 | 3 |
| restore | 1 | 3 |
| schema-change | 2 | 4 |
| toml-tool-change | 3 | 4 |

This fixture qualifies ordering and artifact hashing; the separate Cargo metadata test qualifies production versus test generator selection.
