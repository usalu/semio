# Wave 1125 audit A3 (landing integrity)

Checked at 16:44-16:55 CET 2026-11-25. Read-only: shasum, stat, git diff --stat, grep, `repo test-file` (6 runs, one at a time, load < 40 each time). Sources: `T/landings.md` sections `## Wave 1125 F1, F3, F5, F9, F4, F10, F4 round 2`.

## 1. Hash check (live file vs landed "after", 12-hex prefix)

| File (under `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/` unless noted) | Landed after | Live | Result |
|---|---|---|---|
| F1 `🧬️protocol/📦️packages/🦀️rust/🦀️.rs` | f3898a07 | f3898a07b244 | match |
| F1 `📦️storage/🦀️.rs` | 0c950a4b | 0c950a4be31c | match |
| F1 `🧳️original/🦀️.rs` | 369368a8 | 369368a86779 | match |
| F1 `🏗️record-machine/🦀️.rs` | 033cbf81 | c62fe1926c8a | MISMATCH (landed value is the one live for enum) |
| F1 `🏗️record-machine/🔀️enum/🦀️.rs` | c62fe192 | 033cbf81a625 | MISMATCH (swapped with record-machine) |
| F1 OP `🛍️products/.../🏭️operation/🫴️record/🦀️.rs` | 01445ad4 | 01445ad4f029 | match |
| F4 `🚦️ready/🟦️.ts` (round 2 after) | f6c8e2463813 | f6c8e2463813 | match |
| F4 `🧪️tests/💽️disk-recovery/🟦️.ts` (round 1 after) | 01922cea4293 | dcc381242161 | MISMATCH, mtime 16:44:00 (edited in the last minutes, likely the round-2 +5 assertions or a live edit) |
| F4 `🧪️tests/💽️disk-recovery/🐍️.py` | f053dde8fcfb | b77c2a2f845f | MISMATCH, mtime 16:37:23 (before the landing record time of 16:45, so the recorded hash does not match any later edit) |
| F4 round 2 `♻️collection/🟦️.ts` | (none recorded) | 6f1cb44f2ed7, mtime 16:42 | cannot verify |
| F10 `🗺️topology/🛠️service/🟦️.ts` | 960c8ac8ba0f29bb | 960c8ac8ba0f | match |
| F10 `🗺️topology/🧪️tests/🛠️service/🟦️.ts` | fbfd33f773d6d115 | fbfd33f773d6 | match |

F3 (`🗄️cas/🧬️schema/🔣️.json`, `⚡️caching/.../💾️storage/🟦️.ts`), F5 (`🌲️process-tree/🧬️schema/🔣️.json`) and F9 (warden `🧬️schema/🔣️.json`) have no recorded hashes. Marker grep: `reflink` present in the CAS schema (3 hits), `unshareJavascriptDependencyPatchedCacheV1` present in storage (2), the F5 reading patterns present in the process-tree schema, F9 identity pattern not separately grepped.

Git diff --stat (read-only, working tree vs HEAD; HEAD is older than the landings, so these are cumulative, not per-landing):
- F4 ready/disk-recovery: 3 files changed, 26 insertions, 11 deletions.
- F1 record-machine/enum/protocol: 3 files changed, 135 insertions, 90 deletions.
- F10 topology service + test: 2 files changed, 13 insertions, 3 deletions.

Reading: the F1 mismatch is almost certainly a transcription swap in the landing record (the record-machine file is the larger diff, 127 lines, matching the description of the close_poll wrapper). No `DBG_INNER`, `debug-ledger` or `dbg::` remnants remain in the owned decode directory. The disk-recovery mismatches come from later edits in the same lane (round 2), except the .py whose mtime predates the landing record; that landing hash should be re-verified by the F4 lane.

## 2. Landing locks (`T/landing.lock.*`)

- `landing.lock.bootstrap`: owner `F14 16:44`, age about 2 min. OK (under 40 min).
- `landing.lock.remote-execution`: owner `F14 16:44`, age about 2 min. OK.

## 3. Test re-runs (`repo test-file`, one at a time, 6 total)

| Lane | Test | Claim | Re-run | Result |
|---|---|---|---|---|
| F10 | `🗺️topology/🧪️tests/🪟️hidden-spawn/🟦️.ts` | 3 pass / 0 fail | 3 pass / 0 fail | match |
| F5 | `🚦️ready/🧪️tests/🕰️stale-stage/🟦️.ts` | 15 / 0 | 15 pass / 0 fail | match |
| F3 | `🗄️cas/🧪️tests/🟦️.ts` | 68 pass | 68 pass / 0 fail | match |
| F9 | `🛡️warden/🧪️tests/🟦️.ts` | 46 pass / 0 fail | 49 pass / 0 fail | count differs (higher); file may have gained tests since the claim |
| F4 | `💽️disk-recovery/🟦️.ts` | 10/10 | 10 pass / 0 fail | match (but the file was being edited at 16:44) |
| F4 r2 | `♻️collection/🧪️tests/🟦️.ts` | 19/19 | 19 pass / 0 fail | match |

Not re-run (over the 6 budget): F4 early-admission 4/4, resume 8/8, disk-unknown 6/6; F10 consolidate 11/0, unit-cache key 8/0, topology service 13/2 (the 2 failures are real-launchd macOS tests); F3 patched-cache 1 pass; F4 r2 early-admission/resume/stage-completion; F9 has no landing-level test beyond warden.

## 4. Findings to act on

1. F1 landing records record-machine and enum hashes swapped (see table). Correct the landing record.
2. F4 disk-recovery `🐍️.py` live hash b77c2a2f845f does not match the landed f053dde8fcfb, and its mtime (16:37) predates the landing record (16:45). Re-check with the F4 lane.
3. F4 disk-recovery `🟦️.ts` changed at 16:44, after its landed hash. Live edit or round-2 work; the F4 lane should record the new hash.
4. F4 round 2 is dated `2026-11-25 17:10`, which is later than the current time (16:44). Timestamp is wrong or the record was written ahead of time.
5. F10 claims the hidden-spawn law test lives under `🤝️shared-worktree/🧪️tests/🫥️hidden-spawn/` (claims-tests.md); the landing and the actual files are under `🤝️shared-worktree/🗺️topology/🧪️tests/🪟️hidden-spawn/`. Claims file is wrong.
6. F10 claims windowsHide on 59 sites across 46 files; this audit only spot-checked (17 files with windowsHide under shared-worktree) and did not count the other roots.
7. F9 test count 49 vs claimed 46 (higher, not a failure).
8. F4 round 2 `♻️collection/🟦️.ts` has no recorded after-hash.

Scratch outputs of the six runs: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/A3/` (not in the ticket, so nothing to delete under `W/🗑️generated`).
