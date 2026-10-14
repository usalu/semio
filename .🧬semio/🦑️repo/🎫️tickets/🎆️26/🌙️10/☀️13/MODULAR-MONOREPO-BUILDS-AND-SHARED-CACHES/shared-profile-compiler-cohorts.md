# Shared Profile Compiler Cohorts

The compiler previously acquired an exclusive profile lease despite Cargo's configured fine-grained unit locking. Every same-profile builder waited for the entire preceding invocation. The schema and actual lease now admit a shared compiler cohort at both store and profile levels. Cargo retains responsibility for each physical compilation unit. The explicit `acquireCargoProfileMaintenanceLeaseV1` acquires shared store access followed by exclusive profile access; incremental compaction uses that method. Whole-store maintenance remains exclusive.

The language-neutral fixture covers same-profile overlap, independent profiles, exclusive maintenance, original cancellation, crashed holders, and profile/store writer barriers. A queued writer prevents later compiler cohorts from overtaking it. Actual Bun and Node processes remain held simultaneously, and an independent Node SQLite transaction confirms exclusive maintenance cannot enter an occupied profile. The existing Python SQLite and filesystem oracles also run. The dedicated case schema is validated through Ajv. The physical compiler directory contract now enforces 256 UTF-16 code units.

Actual TDD and qualification:

- `cargo-cohort-red.log` rejects the previous exclusive compiler contract against the new shared schema.
- `cargo-cohort-green2.log` passes the initial cross-process cohort and generic resource tests in 6.8 seconds.
- `cargo-build-lease-cohort-prototype-final1.log` and `cargo-build-lease-cohort-release-final1.log` pass actual same-profile overlap, independent Node SQLite exclusion, cancellation/crash recovery, and both writer barriers.
- `pruning-cohort-{prototype,release}-final1.log` pass all five cooperative pruning laws, 57 Bun assertions plus existing Node/Python assertions, and strict TypeScript. An earlier strict check exposed two schema-string narrowing errors; the actual guards were repaired before these green checks.
- `cargo-test-leases-cohort-prototype.log` passes three Cargo test-driver custody laws and 57 assertions, including held admission through listing/assertion termination, queued cancellation without spawn, timeout termination and terminal reacquisition. Its fake Node process is an assertion oracle; no actual Cargo compilation is claimed by that gate. `cargo-test-leases-cohort-release.log` also passes the same three custody laws and 57 assertions through actual Nx in 1.2 seconds.

Changed owners: native-build lease source, schema, cases schema, neutral fixture and actual process law; narrow TypeScript lease facade; incremental compactor; intentional blocking fixtures in Cargo test-driver, mutation inventory and fresh-component tests; generic queue boundary fixture; retained private reclamation inputs; headroom script/project routes and launch entries. Reclamation inputs were updated to explicit maintenance admission and were not rerun.

Independent audit findings are retained in `build-cohort-audit.md`. Hub namespace and release-before-capture defects are repaired separately by the native executor, as recorded in `hub-build-custody.md`. Target-directory ownership for general maintenance remains unresolved. Native real-Cargo overlap/reuse is a separate bounded qualification and is not inferred from process/SQLite tests. Full prototype/release graph completion and accepted warm ordinary-launch timing remain outstanding.
