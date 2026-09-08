# Nx Runtime Cache Verification

Passed on darwin/arm64 with Nx 21.6.11. This isolated fixture verifies the Nx contract; individual product restoration remains separately tracked.

| Scenario | Builds Executed | Tests Executed | Exit | Milliseconds |
| --- | ---: | ---: | ---: | ---: |
| cold | 1 | 0 | 0 | 1954 |
| warm | 1 | 0 | 0 | 877 |
| restore | 1 | 0 | 0 | 957 |
| test-cold | 1 | 1 | 0 | 859 |
| test-only-build | 1 | 1 | 0 | 796 |
| test-only-test | 1 | 2 | 0 | 866 |
| unrelated | 1 | 2 | 0 | 630 |
| source-change | 2 | 2 | 0 | 819 |
| environment-change | 3 | 2 | 0 | 1170 |
| toolchain-change | 4 | 2 | 0 | 1285 |
| guarded-cold | 4 | 2 | 0 | 2022 |
| guarded-warm | 4 | 2 | 0 | 1109 |
| guarded-rejected | 4 | 2 | 130 | 860 |
| failure-first | 5 | 2 | 1 | 760 |
| failure-retry | 6 | 2 | 1 | 774 |

Restored artifact bytes and executable bits matched; its consumer printed 42.
