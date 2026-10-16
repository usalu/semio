# Artifact Descriptor Failure and Final-Component Link Boundary

## Implemented Source

Both actual common inspection readers (`readArtifactManifestV1` and `artifactDigestV1`) now call the focused first-party `inspection/📁️descriptor/🟦️.ts` owner. Its generic descriptor interface requires only asynchronous close; it issues no control, store, root or Native authority. The owner awaits the body and then always awaits close. An explicit tagged outcome preserves the exact earlier Error or thrown undefined; successful body plus failed close rejects with the exact close reason, including undefined. No truthiness sentinel or catch-and-acknowledge exists. Bounded buffers/hash objects are now allocated inside this owned descriptor scope, so an allocation exception after open also reaches close.

Actual opens use `O_RDONLY | (O_NOFOLLOW ?? 0)`. The platform flag refuses a symlink at the final opened component where exposed by Node; existing pre-open named-path checks, exact descriptor pre/post identity, final named-path identity, full SHA, extent checks, per-pass aggregates, Native root checks and final roster recensus remain. This is **not** atomic beneath-root resolution: ancestor substitution and arbitrary raw filesystem ABA still require the existing custody/recensus assumptions. No cross-platform kernel no-follow guarantee is claimed where the flag is unavailable.

## TDD and Independent Oracles

The first missing-helper/authoring attempts were not behavioral proof (one test schema syntax error was corrected). Genuine functional RED19469 tested the old try/finally precedence in the focused owner:110 pass/2 fail, including exact primary-undefined mismatch and actual caller integration absent. Log `🗑️generated/artifact-close-precedence-red2.log` preserves this result.

The closed language-neutral JSON corpus covers successful value, Error and undefined body failures with successful/Error/undefined close outcomes. Ajv validates it; independent SQLite CASE precedence chooses the expected winner. Deferred close gates prove the returned operation does not settle before close. An independent TypeScript AST checks both actual caller closures use the owner and no-follow flags while retaining real stat/read/lstat calls. Group aggregate tests still check charge placement and full descriptor SHA integration.

A cheap real ticket file is opened/read/statted/closed through the helper. Node crypto SHA256 equals Bun CryptoHasher. The closed handle refuses later stat. Actual macOS final-component symlink open with the exact flags refuses; Windows symlink privilege behavior was not exercised. Every temporary file/directory is removed by its test finally. No Native images, publications, listener, compiler or canonical store was opened.

Final prototype80878 and release22043 are both terminal exit0:113 tests/1,582 assertions in7.34/7.18s. Expanded strict ES2022/noUnchecked64221/66157 are both terminal exit0. Logs are `artifact-close-final-vite-environment{-types}-{prototype,release}.log` under generated. Existing editor-registered `sdk-derivative/📜️script.ts vite-environment|vite-environment-types <mode>` includes these new tests; no new executable route or script was created.

## Remaining Qualification

The descriptor-specific failure precedence limitation in `artifact-group-reader-current.md` is superseded by this executed repair. This does not replace that report’s pending actual group SH/FD/partial-acquisition/runtime proofs, nor establish immutable consumed-byte custody. All own handles are closed and Source is frozen for handback.
