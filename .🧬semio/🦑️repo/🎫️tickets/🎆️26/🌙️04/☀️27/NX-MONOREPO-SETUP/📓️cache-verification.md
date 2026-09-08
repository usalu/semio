# Nx Runtime Cache Verification

Passed on darwin/arm64 with Nx 23.2.0. This isolated fixture verifies the Nx contract; individual product restoration remains separately tracked.

| Scenario | Builds Executed | Tests Executed | Exit | Milliseconds |
| --- | ---: | ---: | ---: | ---: |
| cold | 1 | 0 | 0 | 2562 |
| warm | 1 | 0 | 0 | 1938 |
| restore | 1 | 0 | 0 | 1217 |
| test-cold | 1 | 1 | 0 | 1040 |
| test-only-build | 1 | 1 | 0 | 731 |
| test-only-test | 1 | 2 | 0 | 613 |
| unrelated | 1 | 2 | 0 | 661 |
| source-change | 2 | 2 | 0 | 482 |
| environment-change | 3 | 2 | 0 | 400 |
| toolchain-change | 4 | 2 | 0 | 563 |
| guarded-cold | 4 | 2 | 0 | 581 |
| guarded-warm | 4 | 2 | 0 | 689 |
| guarded-rejected | 4 | 2 | 130 | 1217 |
| failure-first | 5 | 2 | 1 | 941 |
| failure-retry | 6 | 2 | 1 | 895 |

Restored artifact bytes and executable bits matched; its consumer printed 42.
