# Published root "dangling record" (OriginalAdmissionClosed on cold MCP launch)

## Finding
No record, index or ledger refers to `published/6d3390d7b4aeb20c3436/prototype`. The key is only sha256(manifest)[0:20]; the root is renamed into place by `executeArtifactPublicationV1` (process/artifacts/publication). A kill between its two renames leaves the root absent, and the next publisher simply republishes (`existsSync(staging)` false), so (a) needs no change: nothing is recorded.

## Root cause
`servePublishedMcpStdioV1` (os/mcp, `serveUsable`) admits the previous publication when the catalog files exist. `withRepositoryPublishedCargoExecutableV1` then lstat'd the absent root and threw ENOENT through a tracked Original operation. `track()` stores the first thrown failure and closes the whole process owner, so the swallowed probe failure made the real build refuse with `OriginalAdmissionClosed ... first failure: ENOENT lstat .../prototype`.

## Fix
- `PublishedArtifactAbsentV1` (process invocation owner), handled like `ResourceLoanDeadlineExhaustedV1`: answers its caller, never closes the owner. Re-exported via the typescript routing facade.
- The reader checks directory and marker first and throws it on ENOENT/ENOTDIR (other errors still close the owner).

## Tests
- `process/routing/invocation/process/tests/retry-after-tracked-failure` plus new `fixture.json` rows (language-agnostic).
- published reader test: new `published-absent-law` case.
- Both pass via `repo test-file`.

## Verification
Journey run: cold launch passes "building the owned native publication" and cargo compiles (compiler-artifact lines streaming). Compile not run to completion (stopped after about 100 s).
