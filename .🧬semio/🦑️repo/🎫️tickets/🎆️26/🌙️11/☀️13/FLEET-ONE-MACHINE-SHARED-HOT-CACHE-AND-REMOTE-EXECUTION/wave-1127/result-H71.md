# H71: m04 `Bounded relative file read required`

Cause: regression from H42 (05:00). `policy.maximumReadBytes` (65536) now bounds `file.read` for real; `readProtectedArtifactInputV1` reads staged dependency inputs in chunks of 192 KiB to 6 MiB (`protectedArtifactInputChunkV1`, scaled by free memory), so hashing a staged input group failed on Linux m04 (fresh group, mode `all`). macOS/Windows share the code; their staged groups are mostly stamp-vouched, so they had not hit it yet. Store check: only g1-m04 logs the text (11:17, 11:28); no other origin.

Fix (landed 12:00-12:08, files in landings.md "H71"):
- `withFile(name, body, {maximumReadBytes})`: default 64 KiB stays; declared bound 1..8 MiB (`RelativeProtocol.maximumFileReadBytes`, existing schema); POSIX + Windows. Refusals name path and size, English and German.
- Platform input reader declares `chunk.decoded`; `image.json` receipt (up to 64 MiB) streams in 64 KiB steps.
- Other `withFile` callers audited (dependency storage, retirement, reset, format marker, funding/metadata, envelope, tools marker): all read <= 16 KiB or chunk 65536.
- Failure diagnosis: ready stage env `SEMIO_FAULT_DETAIL=1`; CLI boundary writes `[DATA] Fault detail / Fehlerdetail <ref> {chain,stack}` before the internal error; mirrored whole in `ready-log-<stage>-failed.txt` (long-line + error patterns).
- Tests: read-bound (real FFI, oracle readFileSync) and fault-detail pass; cache suite (artifact-owners) green after making the path lazy (first landing broke 10 extractor tests with `artifactRelativeWindowsPathV1 is not defined`, fixed 12:07); typecheck 0 errors; dangling imports 0.

Unverified: m04 re-run (workers restart on this landing; see watch below). A typecheck run before my edit showed one unrelated error in `🚦️ready/🧪️tests/🛑️stop-sender` (not mine).

## Watch (to 12:12)
m04 picked up the new code digest (bd282afe) at 12:09:30, is in state `waiting` (disk-recovery collectors, 67.6 GiB free), attempts 3; the tools stage has not re-run the dependency sync yet. Next check: when m04 starts attempt 4, `ready-log-tools*.txt` must have no `Bounded relative file read`; if another error appears it now carries a `[DATA] Fault detail` line.
