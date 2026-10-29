# Bootstrap Audit — 2026-10-28

## Scope and Conclusion

Bounded independent source-only review of the dependency publication reexecution and Cargo early mode repair. No concrete defect found in the reviewed changes. No builds, tests, dependency fetches, or peer process changes were performed. This is not evidence of application, release, or E2E completion.

Reviewed dependency-reexec-2026-10-28.md and proctor-native-2026-10-28.md, then the actual dependency helper, sealed execution wrapper, captured kernel entry, invocation resolver and tests, Cargo launcher, and existing format-stamp reexecution supervisor. backend_live confirmed replacement native session 16280 is its canonical build; parent main 4072 and front-end sessions remain independently owned.

## Dependency Publication Boundary

The helper privately issues requests using a module-local WeakSet. The captured kernel catches only after awaited receiveScriptProcessInvocation settles, recognizes locally issued requests, and sets exit status 76. The parent helper translates only OwnedCommandFailure with status 76 and null signal to existing staleImplementationV1. The sealed executeImplementationV1 invokes executeImplementationOnceV1 exactly once: it no longer recaptures under an already failed Original.

The existing external supervisor awaits the entire failed run before replacement, preserves executable arguments using execArgv plus argv.slice(1), and spreads the existing environment into the replacement. It retains its existing bounded restart mechanism and nested exit-75 propagation. The patch introduces no permission grant or new deadline construction. The serialized Original envelope is retained, so replacement does not receive a new creation time merely because dependency publication occurred.

The actual-Original regression extracts the authored sealed wrapper, substitutes its implementation launcher, and launches a real owned child exiting 76. It asserts old invocation and resource-control retirement before its injected replacement callback, retained argv, language, backend and serialized envelope, and nonincreasing remaining time. Its earlier-real-failure branch asserts the original error remains authoritative and replacement does not run. This is useful lifecycle evidence, but the replacement callback is injected; it does not itself run the whole production front door in a fresh OS process. The separate cold-snapshot regression addresses immutable capture versus fresh admission. Private issuance rejection covers copied and newly constructed errors.

Ajv validates neutral fixtures; SQLite supplies a decision oracle, and native Node createRequire independently checks published-package resolution. The report records 26 passing tests and 396 assertions across the related suites; this audit inspected test definitions without rerunning or independently recounting that execution.

## Cargo Mode Admission

resolveBuildModeV1 now admits cargo through the same pre-separator configuration analysis as run, run-many and affected. Its return preserves selection.args, including the entire Cargo tail. Explicit release therefore enters bootstrap authority before Original envelope issuance. Conflicting inherited mode still refuses in the resolver, and readRepositoryCargoLaunchV1 independently retains its explicit configuration versus SEMIO_BUILD_MODE conflict guard. Launcher parsing still slices the opaque Cargo tail after the first separator.

The new test runs the actual generic resolver, createBuildBootstrapCapabilitiesV1 for both Nx and Bazel, and readRepositoryCargoLaunchV1. It asserts mode agreement, unchanged complete argv and unchanged Cargo tail. Its independent yargs-parser oracle receives only the pre-separator head, SQLite decides mode consistency, and Ajv validates neutral vectors. This demonstrates capability issuance and launch parsing; it does not execute either build backend or compile proctor. The report records six focused passing tests and 281 assertions; those totals were not independently rerun here.

## Remaining Evidence Boundary

The first native attempt 71755 is documented terminal before replacement 16280 was started. At the last backend_live message, replacement 16280 remained at capacity admission. No observed Cargo metadata, Rust compiler diagnostic, linked proctor binary, qualified product receipt, or course E2E success follows from these unit and source checks. Continue to use the existing canonical live owners rather than launching duplicate verification.
