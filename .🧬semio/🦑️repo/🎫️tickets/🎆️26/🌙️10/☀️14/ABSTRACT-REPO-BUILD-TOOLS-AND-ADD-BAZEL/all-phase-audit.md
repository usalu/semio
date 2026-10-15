# Retained Complete Producer Phase Intervals

Read-only analysis of final-all-current5.log, 31351 recorded phase events. The original deadline marker is present. These are elapsed intervals between progress phase transitions, not CPU profiling or per-owner attribution. Active waits, asynchronous work, source settlement and progress emission can share an interval. Final incomplete phase time is excluded from aggregate completed intervals. No Source, leases, caches or peer processes were changed.

| Phase | Completed Intervals | Total ms | Maximum ms |
| --- | --- | --- | --- |
| build:command-source-imports:running | 1794 | 839949 | 197026 |
| cache:leases-store:running | 1 | 112066 | 112066 |
| build:cargo-owners:running | 3 | 97031 | 36896 |
| build:cargo-targets:running | 3 | 37991 | 16044 |
| build:playground-native:running | 114 | 35161 | 3262 |
| cache:dependencies-store:running | 1 | 31449 | 31449 |
| build:discover:running | 1 | 8434 | 8434 |
| build:commands:complete | 3 | 7838 | 3493 |
| cache:graph-store:running | 1 | 7165 | 7165 |
| build:playground-components:running | 3 | 5228 | 2801 |
| build:command-source-packages:running | 1186 | 3148 | 806 |
| build:settle:running | 2 | 2648 | 1449 |
| build:playground-variants:running | 117 | 2575 | 194 |
| build:discover:complete | 1 | 2379 | 2379 |
| start | 1 | 1642 | 1642 |

Last recorded phase: build:discover:running; elapsed 1199118ms; duration 2113ms. Recorded observations: {"loadObservations":136,"projectImages":596,"originalInferencePublications":596,"projectObservationRows":1115255,"captureObservations":0,"recordStores":571,"analysisEntries":14017,"analysisBytes":6492611}. Counts: {"build:dependencies:running":1,"cache:dependencies-store:running":1145,"cache:leases-store:running":176,"cache:graph-store:running":116,"build:discover:running":608,"build:discover:complete":1,"build:cargo-owners:running":330,"build:commands:running":3,"build:command-source-imports:running":755376,"build:commands:complete":3,"build:playground-components:running":465,"build:playground-variants:running":468,"build:playground-native:running":210,"build:cargo-targets:running":330,"build:project-assembly:running":586,"build:command-source-packages:running":1226,"build:projects:running":37,"build:source-contracts:running":6,"build:settle:running":2}. The exact inner source-conflict identity is unavailable because authority tracing was omitted. No accepted complete publication follows from these diagnostic records.
