# Result F125: footprint test type errors

- Cause: stale declaration. `🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs` already takes `maximumCharacters` and returns `characters()`, but its `🟨️.d.mts` (03:23) declared only `maximumFiles` and no `characters()`. The test at lines 508-514 was correct.
- Fix: updated `♨️reuse/🟨️.d.mts` (temp + rename, lock `landing.lock.graph`): `createSharedSourceFilesV1` options gain `maximumCharacters?: number`; `SharedSourceFilesV1` gains `characters(): number`. No test or module code changed, no casts.
- Peer check: test file mtime 18:14:16 and reuse module 18:14:12, both older than 20 min at 18:42, so no wait.
- `repo test-file` on footprint `🧪️tests/🟦️.ts`: 54 pass, 0 fail, 414 expects, exit 0.
- `repo typecheck` on that test file and the `.d.mts`: passed, 7 files (2 touched, 5 importers), 0 errors. Dangling-imports check on the reuse folder: exit 0.
