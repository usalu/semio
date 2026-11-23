# 🔀 Replica Merges Stale Local Edits at Apply

## Problem
During the catch-up of g1-m05 (2026-11-22/23), the replica applied hours-old inbound events while agents were editing the same files live. By rule (2) of "Stale views never overwrite newer fleet content" (`conflictWinnerV1`), a local edit written more than `concurrentEditMilliseconds` (30 s) after the inbound event's wall time counts as made on a stale view. The fleet content was installed and the local bytes were moved to `.cache/f/lost/<hlc>/<path>`. The FEM framework agent lost its `NativeReceive` derive edits three times this way, and had to rewrite them.

"Local wins when it is newer than this origin's last publish" was rejected: it would bring back the 2026-11-22 02:13 incident (a stale view overwriting newer fleet work).

## Fix
Rule (3), Merge. Before a remote put replaces a losing local edit, the apply makes a three-way line merge:
- `base` is the content the replica last applied at the path (`known.sha256`), read from its blob and verified by sha256.
- `ours` is the local file.
- `theirs` is the verified inbound bytes already staged for the install.

A clean merge is written over the installed fleet content (event `merged`). The replica's register keeps the inbound event, so the merged file is an ordinary unpublished local edit. It is published once the replica has caught up, through the existing fence. The next inbound put of the same path merges against that put again, so a chain of pending events merges step by step. The local bytes also stay in `lost/`.

A real conflict, binary content, a side over 8 MiB or more than 4096 line edits from the base, a missing or corrupt base blob, or a refused write all keep rule (2): fleet content stays, local bytes go to `lost/`, event `stale-base`. `stale-base` is now emitted after the install finished (it used to be emitted before the blob copy).

## Files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🔀️merge/🟦️.ts`: `mergeTextV1`, `mergeBytesLimitV1`, `mergeEditsLimitV1`. Pure function, no dependencies, byte exact (latin1 line split). Myers diff with common prefix/suffix trimming. diff3 hunk grouping where overlapping or touching changes conflict unless they are identical (git's rule).
- `…/🔀️merge/🧬️schema/🔣️.json`: schema of the language-agnostic vectors.
- `…/🔀️merge/🧪️tests/🧫️fixtures/🔣️.json`: 18 vectors (CRLF, no trailing newline, UTF-8, insert/insert conflict, adjacent lines, delete-vs-edit, identical changes, …).
- `…/🔀️merge/🧪️tests/🟦️.ts`:
  - schema check;
  - the vectors;
  - every vector against `git merge-file -p` (third-party reference);
  - 300 seeded random edit pairs, where every clean merge must equal git's output;
  - limits;
  - performance: 20,000-line file in under 500 ms.
- `…/🔁️replica/🟦️.ts`:
  - `mergeLocal` and `installMerged`;
  - the put branch of `materialiseWork`;
  - `noteStale` moved after the install;
  - event `merged`.
- `…/🔁️replica/🧬️schema/🔣️.json`: `merged` in the event enum.
- `…/🔁️replica/📖️texts/🔣️.json`: `event.merged` (en, de).
- `…/🔁️replica/⌨️command/🟦️.ts`: prints `event.merged`.
- `…/🔁️replica/🧪️tests/🧫️fixtures/🔣️.json`: scenario `stale-edit-of-other-lines-is-merged-onto-the-fleet-content-and-published` (two machines, the Python oracle compares store and worktree).
- `…/🔁️replica/🧪️tests/🟦️.ts`: scenario expectation `mergedOn`.
- `…/🔁️replica/⚖️clash/🧪️tests/🟦️.ts`: fleet test "an edit written on a stale view that changes other lines … is merged onto it and published" (three machines; POSIX only).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/README.md`: rule (3) and the operator note.

## Verification (g1-m05, Windows)
- `🔀️merge` tests: 6 pass. All 18 vectors equal `git merge-file`. Over 50 of the 300 random rounds merged cleanly, each identical to git.
- `⚖️clash` tests: 4 pass, 20 skipped. The fleet harness tests, including the new one, are POSIX only. They were not run here, because there is no Linux bun in WSL and docker is not running.
- Replica suite: 65 pass, 1 skip, 3 fail. The new merge scenario and the three stale and offline tests pass. The 3 failures are Windows environment issues:
  - `sleep` is not on PATH;
  - a test file name with `:` that Windows rejects;
  - the relay-vanish test timed out at 60 s under 100 % CPU from parallel cargo builds.
- Live daemon: at 06:53 it restarted from the worktree code that includes the merge (beat `source: worktree`, lag 149 ms). An earlier code digest (64da6b85, before the integration) was refused because its self-check timed out under host load. The daemon ran sealed code `bd65c594` until then.

## Open
- Run the POSIX fleet tests on a Linux or macOS fleet machine (`⚖️clash/🧪️tests/🟦️.ts`).
- Rule (2)'s "kept" branch (a local edit within 30 s of the remote event wins by mtime and the remote change is superseded) could merge as well; not changed here.
