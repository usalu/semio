# Task Preparation for Current Graph Queries

The coordinator's earlier explicit current project-list query took about 80 seconds before reporting that no whole graph was sealed. Its profile included more than 178,816 parser-source listing operations and 106,496 verification operations in each dependency pass. The native command entered the runtime's dependency preparation and task-store maintenance before the existing current-only loader refused. The same runtime already had a direct route for an explicit current single-project read without a bound generation pin or a reused graph.

The runtime now uses that route for explicit current project and target lists and for explicit current full graph queries. The direct route runs under the existing invocation and diagnostics control and enters the normal graph query loader, including its parser bootstrap and graph store custody. Whole queries retain complete compatible peer restoration, their ordinary local seal checks and the existing progress-bearing refusal. Lists keep the lean catalog path. Full graph verification still enters the ordinary dependency path when it can run baseline inference. Pinned or reused queries, implicit policy, and fresh requests keep ordinary preparation.

Before the fix, a private runtime fixture recorded dependency setup, a dependency-store open and maintenance for the explicit current project list. It now observes none of those phases for the newly eligible queries. The language-neutral cases and independent SQLite routing check cover explicit current flags and environment, list queries during verify mode, full graph baseline mode, implicit current, fresh, pins, and reused graphs. The private filesystem producer verifies the project and target outputs and the whole-reader refusal with builder progress. The real whole-gate fixtures still restore a complete peer head and refuse an incomplete one. The combined focused run passed 16 tests with 292 assertions.

The before profile was taken on the coordinator. The after measurements in this audit are from private fixtures. No production graph pointer, cache or process was changed manually.


## Native Coordinator Check

After the query routing and Bun producer changes, the same native command returned the existing no-sealed-generation diagnostic with exit 1. It emitted zero parser-source scans, zero dependency-generation verification passes, and no stale installer-configuration event. The earlier invocation exceeded 178,816 parser source listing operations and reached 106,496 verification operations under each custody mode. Host admission dominated the 85.9-second elapsed time. The phase counters independently show the avoided work. The graph still requires a compatible complete seal.

[Admission observations](audit-current-admission-pulse.md) · [Bun producer provenance](audit-dependency-bun-launcher-provenance.md)
