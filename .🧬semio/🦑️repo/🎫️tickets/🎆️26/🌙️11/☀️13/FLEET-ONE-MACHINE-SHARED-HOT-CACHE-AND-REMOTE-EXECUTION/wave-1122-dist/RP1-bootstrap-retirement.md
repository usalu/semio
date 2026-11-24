# RP1: bootstrap pack retirement

## Findings
- Code: `🔁️replica/🥾️bootstrap/🟦️.ts` (`writeBootstrapArchiveV1`, `refreshBootstrapV1`). The group-store `compaction.json` covers only journal segments, blobs and snapshots, never `bootstrap/`.
- Retirement existed but leaked three ways: (1) the grace was the file mtime touched at supersession by a best-effort `utimes`, and a fixed 1 h constant; (2) the prune ran only after the fold and plan succeeded, and the wait between rounds could far exceed the grace, so a superseded pack lived until a later successful round; (3) the space guard counted neither superseded packs inside the grace nor offered a way to free them, so A (held) + B (current) + C (12-16 GiB staging in v1/tmp) = ~3x a pack. Orphans of a crash between renaming the tar and the manifest were only pruned by mtime.

## Fix
- `retireSupersededV1(storeRoot,{graceMilliseconds,now})`: every worktree-*.tar/.entries the manifest does not name is retired once grace passed since max(file mtime, manifest switch); no touch needed, orphans and crash leftovers covered; a busy file (Windows) is retried later.
- Runs first in every build round (before fold, so a failing fold cannot skip it), after the manifest switch, and at the end of each grace (the refresh loop wakes at the next grace end, min 1 min).
- Readers: grace (default 1 h, schema `Retention.graceMilliseconds`) keeps the old pack for readers mid-copy; on POSIX an open reader survives unlink; later readers follow the manifest. Under space pressure the shorter `pressureGraceMilliseconds` (10 min) applies.
- Space: existing guard (tar + staged pack estimate + 3 % reserve) refuses before staging; refusal is now recorded in `.cache/f/bootstrap.json` and shown as `bootstrap` in `status` (archive, bytes, held, retiredBytes, refusal).
- Schema: new `🥾️bootstrap/🧬️schema/🔣️.json`; status schema extended.

## Tests (scratch stores only)
Bootstrap file 28 pass (6 new: A then B retired after grace, in-flight reader completes and the manifest moves to B, orphan sweep, grace counted from the switch, pressure retirement plus low-space refusal recorded, record + schema default). Python oracle `retention` subcommand cross-checks the directory independently. Whole module (61 files, one at a time): all pass except pre-existing environment failures: launcher-first (2), topology service real launchd (2); race.ts flaked once, passed on rerun.

## Open
- Not restarted: takes effect at the daemon's next natural restart. The live store currently holds one pack (human cleanup done).
- Reader-side lease for shell/PowerShell provisioners was not added; grace plus POSIX unlink semantics cover it.
