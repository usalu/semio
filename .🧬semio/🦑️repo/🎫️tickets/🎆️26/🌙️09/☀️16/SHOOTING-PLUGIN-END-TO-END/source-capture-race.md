# Source capture race: "Original Native terminal record changed"

## Root cause
The only source of the message is the native whole-retirement journal in
`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🟦️.ts`
(`continueRetirement.verifyRecord` and `publishIntent`). It pinned the durable
`<capsule>.retire.json` by a metadata stamp (`dev:ino:nlink:size:mtimeMs:ctimeMs`) and
aborted on any difference. On a live replicated worktree any metadata-only touch
(replica daemon, chmod, utimes, indexer) moves ctime/mtime without changing content,
so a resumed retirement aborted the whole command (seen in `source.capture`).
No log was retained and no `.retire.json` exists now, so the incident itself was not
reproduced; the invariant and the race were reproduced at unit level.

## Fix
- Policy in schema: `Protocol.recordRescans = 8`, `recordRescanMilliseconds = 25`
  (`🧹️whole/🧬️schema/🔣️.json`).
- `rescanNativeImageRetirementFileV1` (`📄️retirement-file/🟦️.ts`): re-reads the
  record with bounded rescans; adopts the new stamp only if bytes are identical to the
  held bytes; changed bytes or non-convergence beyond the bound still fail with the
  original message.
- `whole/🟦️.ts`: `verifyRecord` and both `publishIntent` checks use it (held record
  now carries bytes, not just the stamp); progress phase `rescan` reported to the observer.

## Tests
`repo tools exec bun -- test ./…/🧹️whole/🧪️tests/🟦️.ts`: 6 pass (new test mutates
mtime/ctime between reads: adopted; mutates bytes: refused).
