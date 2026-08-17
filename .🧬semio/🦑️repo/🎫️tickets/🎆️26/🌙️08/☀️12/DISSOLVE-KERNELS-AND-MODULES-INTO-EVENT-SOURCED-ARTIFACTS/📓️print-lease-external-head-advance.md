# Print Lease External Head Advance

## Observation

While the print semantic SCC lease was active, repository `HEAD` advanced externally to `dbcc4fa462` at approximately 03:32 CEST. The new commit includes the lease paths, so the worktree status/diff is clean despite the lease beginning from the monolithic print runner.

## Handling

- The Terra lease holder did not run any modifying Git command.
- No reset, checkout, stash, amend, or restage action is authorized or required.
- Treat this as concurrent external state.
- Before release, rehash every leased source/referrer path and compare the semantic tree, mounts, manifests, tests, and runtime behavior to the packet rather than relying on `git diff`.
- Record the resulting content hashes and validation output in the lease evidence report.

The external commit does not constitute validation or a release verdict.
