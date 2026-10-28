# Go Build Lease Ownership Audit

The Linux workspace build refused an unbacked lease tree at `.cache/b/l`: the unpublished-generation retention path attempted to observe `fixed.dir`, while only `m/data` existed. The payload is genuine SQLite lease metadata, not a complete unpublished partition. Retention must continue refusing this envelope.

SQLite inspection identified the producer chronology: compiler host-capacity admission, the picker and CPU slots were created at 2026-10-28 15:14:11 UTC; artifact capture and the prototype repo executable destination followed at 15:14:19 UTC. The native repo installation build calls `runCanonicalGoBuild`, which previously passed the planned lease pathname directly to `buildGoArtifactsV1`. The lease implementation recursively creates that path, bypassing the physical partition opener.

The Go build entry now opens the genuine leases partition first, rechecks its physical owner, derives and validates the lease data path beneath the retained image, and holds that owner through toolchain preparation, compilation and artifact publication. It rechecks successful completion and closes on every body outcome. Concurrent body and close failures remain available as an AggregateError. No fixed storage checks were relaxed, no payload adopted, and no generated cache was removed by this change.

The regression is schema first: the Go plan schema and JSON corpus contain six success and failure lifecycles. The test extracts and transpiles the actual Go build entry with controlled bindings, checks the physical opener and exact lease path, and compares event ordering with an independent SQLite JSON oracle. The test failed against the original entry because the opener was absent, then passed after the change: 1 test, 49 assertions.

Validation command: `SEMIO_LANGUAGE=en /home/deck/.local/bin/repo tools exec bun -- test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🧪️tests/🔒️lease/🟦️.ts'`.

The existing invalid cache requires separately owned retirement after confirming no live leases. A real workspace rebuild remains the parent task's integration check. The existing pinned Go environment provisioning also uses the tools partition's planned path directly; that neighboring custody concern was reported to the parent for the complete readiness review.

## Tools Store And Status Audit

The follow-up extended the same build entry to retain the tools Store while provisioning the Go toolchain, compiling and publishing. The exact caller invocation is now required and supplied by all four callers. The already open lease image is shared with the tools Store using command custody. Go build/module caches and temporary paths are constrained to the genuine tools Store. This prevents the neighboring raw tools-directory creation path as well as the original lease-directory failure.

The lifecycle corpus now includes tools Store admission failure, and the harness validates both toolchain root and Go cache roots. The regression is registered in the installer's test command alongside installation and Go layout cases.

The `runLeasesCliV1` status path was inspected through `runLeasesCommandV1`, `readLeaseReportV1` and `readLeaseQueuesV1`. These functions read filesystem registries and queue records without creating, acquiring or writing a lease database. The machine admission sampler uses the separate host-capacity gate. Status therefore remains a read-only observation route that can report an absent or malformed lease tree; opening a generated partition is unnecessary and would add effects to that diagnostic route.

A direct combined invocation through `repo tools exec bun -- test` ran the three suites: 9 tests passed and 1 failed, with 1613 assertions. The sole failure was the existing actual POSIX shell-profile test requiring the parent Script process invocation, which `tools exec` does not transport. The authenticated installer test route is the integration command; the parent task owns running it after cache retirement.

## Native Integration Attempt

After all Source locks converged, the authenticated native install target was started. Its before-state record showed a genuine leases image but an already existing raw tools tree containing only `m/native`, without `fixed.dir` or `image.json`. The target correctly refused this unowned tools envelope during the genuine tools Store opener, before running the repaired Go compiler. The failure is recorded in generated/native-install.log; the independent before-state is generated/native-store-before.json. The parent task owns quiescent retirement of this previously generated raw tree before the clean integration retry.

The parent retired the prior raw tools tree after independent quiescence and archive verification. The authenticated install retry then completed with terminal status 0. It provisioned pinned Go 1.26.9 through the genuine tools Store, completed the native build and installed `/home/deck/.local/bin/repo`. Independent checks confirmed both leases and tools retained a regular single-link `fixed.dir` matching the `image.json` inode and size, with Linux platform receipts. The installed ELF digest exactly matches the build output and installation receipt: `cb60de0ed76c2df96fedc3627f54a7bb595be0f6dafea9a154209f03c3521673`. Evidence was saved under generated/native-store-after.json and generated/native-install-proof.json; no raw tree recurred.

The authenticated installer test target then stopped before running tests at the global JavaScript dependency-budget admission, independently of native Go ownership. Its terminal status was 1. The parent owns that dependency image resize; a final test retry is pending that maintenance, and no repeated command was started against the known refusal.
