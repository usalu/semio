# F20 result

## Landed
- Task 3 (source-admission race): the root cause was not a missing retry but an unpaced one: `traverseNxAuthorizerSourcesV1` walked 4 times back to back (milliseconds), so replica churn outlasted it. Now each walk runs under `retryOnConcurrentEditV1("read")` (10/40/160/640/2560 ms pauses, 6 walks, EN+DE report), re-resolving the file list each walk; a path still gone after the last walk refuses with the original ENOENT. Before: 4 walks in ms; after: 6 walks over >=3.4 s (test: source reappears on walk 3, read after a pause).
- Task 1 (slice read speed), partial: rows that a fresh complete verification found holding are taken as holding by the next pass's in-round (per-owner) holds for 60 s; the fresh end-of-pass hold never uses them and refutes edited rows. Test counts: fresh 300 probes, in-round afterwards 300 reused / 0 probed, after an edit 0 reused. Real saving not measured on a slice.
Files/hashes: T/landings.md "Wave 1125 F20".

## NOT done
- Task 2 (labels): no code landed. Findings: the "(N s)." unlabelled window is the slice worker before the `inventory` step starts (registry read, graph-state lease, store recheck/staging readdir, then step opens); wrapping these in new `sliceReadStepV1` steps (`registry-read`, `graph-state`, `store-check`) with EN/DE texts in `🧠️memory/📋️slice-report/🧬️schema` is the fix. The inventory sub-phase label belongs in `describeSliceReadStepV1` (`🟨️.mjs` line ~106) from `diagnostics().active.phase` (+ per-phase EN/DE table); existing `🪜️read-steps` test asserts exact texts, update with it. The graph-service boot (25-111 s) is the service's own `repo` run doing `build:source-admission` source capture (service log shows "walked N files"); the client (`🚪️client/🛰️plan/🟨️.mjs` line 164) can append the last `[repo] ... last phase started:` log line to `host-wait` (new key `host-wait-phase` EN+DE).
- No real before/after slice timing (host at load 30-37, free memory low).
- Tests: since ~17:28 `repo test-file` fails every file with "Unexpected }" (peer break); last two trivial edits unverified. Dangling-import script: exit 1 only for pre-existing test-fixture strings in authorizers tests (mine removed); observation folder clean.

## Round 2
A5's 62/7 on the observation test file did not reproduce: 69/0 on two runs, plus carried-verdicts 1/0 and ancestry 9/0, dangling-imports exit 0. The conflict error comes from the settle/capture path, checked before the in-round loop where the carried verdict applies, so there is no bypass. Likely cause of A5's run: the launcher breakage ("Unexpected }") or load timing. Nothing edited. Task 2 labels stay staged-by-description for after 19:00.
