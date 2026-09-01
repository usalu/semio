# Source Index Before and After 65

## Proposed Controller

[The controller](../🧪️source-index-before-after-65/📜️script.ts) is prepared but unexecuted. It invokes exactly the existing synchronous public root `mutationTaxonomySourceIndex(repoRoot, { cancelFile, progress })` twice, serially, with no `explicitTicketDir`. That API owns the canonical shared source admission; the controller adds no roots, collector, schema, or declaration-census logic.

It runs the pair in one owned Bun child. The parent consumes both child pipes concurrently, gives the complete pair 120 seconds to observe the existing cooperative cancellation file, then a fixed 15-second terminal grace. If still live, it kills only that owned child. Timers are cleared at terminal state. There is no retry, no budget extension, and no foreign process, cache, or target action.

The API-call counter is incremented at the actual invocation site and terminally requires exactly two calls. Progress is retained independently for the first and second call. The first call writes its complete row artifacts and a small `before-manifest.json` before the second starts, so a later failure remains interpretable instead of erasing the first observation.

## Proposed Receipt

Before and after the child, the parent lexical Compose/escape-checks then captures the controller, root S, shared N normalization, D discovery classifier, taxonomy schema, and mutation-descriptor schema. Every capture records no-follow root/ancestor/leaf identities plus leaf `lstat`, descriptor `fstat` before/read/after, SHA-256, bytes, device, inode, size, and mtime. A run retains child stdout/stderr, a compact manifest, and exclusive-created JSONL artifacts under this exact new owner. The receipt reports only observation facts:

- each complete admission observation row, including its ordered origins, Git entries, repository boundary, and generator-output root matches, in a separately hashed JSONL artifact; admission status, scope, taxonomy/membership digests, and aggregate kind/origin/repository-boundary counts;
- captured taxonomy and mutation-descriptor schema endpoints;
- source-index roots, raw source path rows plus path digest/count, full source-roster rows plus digest/count, source-tree digest, file-kind counts, roster-role counts, ledger status, and progress counts;
- a separately hashed full drift JSONL of added, removed, and changed admission-observation, source-path, and roster entries, and equality of membership, paths, roster, source-tree, and captured schema endpoints;
- source/receipt identity stability and cancellation/terminal status.

The file-kind counts reuse D’s existing `fileKindIdForSourcePath` against the exact taxonomy bytes already captured by SourceIndex. They do not infer source membership or semantic mutation identity.

The child writes rows incrementally to exclusive-created JSONL artifacts, each with its own byte count, row count, and SHA-256. The compact manifest is the only marker-authenticated JSON object sent through stdout; this prevents a large roster from filling a pipe without truncating the actual returned rows.

At terminal, the parent parses the manifest from the bytes authenticated by its own no-follow descriptor read, not by a second unchecked read. It requires exactly seven in-run JSONL artifacts (three per call plus drift), then independently authenticates each artifact's path, SHA-256, byte count, newline row count, and run ownership. The drift includes whole admission observations keyed by raw source path, so origin, Git-stage, repository-boundary, and generator-output changes are retained even when path/roster rows do not change.

## Boundary

This is a proposed actual admission/source-index pair only. A difference is recorded as observed drift, not a stable census; equality is a bounded pair observation, not semantic completeness, declaration coverage, or an inventory of source contents. Root review is required before `run`; no capture has been started. The earlier single-call controller hash remains historical preparation only.

## Final Read-only Review

Reviewed controller `2badfd4e7b88f7fdbb57310b9f6873ac33f4084b26a28c1b9341580e454f9b53` without execution. The child is serial: it completes the first API call, streams its three artifacts, writes `before-manifest.json`, and only then invokes the second call. `writeAll` uses the Node/Bun buffer overload with an advancing offset, so a short write is retried until the complete buffer is written. The parent starts both pipe readers immediately after spawning the owned child. Baseline, terminal observation, and child stdout/stderr are persisted before post-run validation; a failed second call still retains the first artifacts and baseline/terminal observations.

Release blocker: the stdout parser selects the first marker rather than requiring exactly one, and the parent checks only the marker's `apiCallCount`. It parses the authenticated manifest but does not assert its measured `apiCallCount === 2` (nor bind its terminal semantic status to the marker). A malformed or extraneous marker can therefore reach artifact validation without proving the manifest records the same two actual calls. The parent should require one marker and require the parsed manifest's own call count to equal the marker and `2`; preserve `semanticCompleteness: false` as an explicit manifest boundary. No controller changes were made by this review.
