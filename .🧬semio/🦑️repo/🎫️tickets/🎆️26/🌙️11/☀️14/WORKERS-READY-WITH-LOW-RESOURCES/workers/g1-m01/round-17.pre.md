# g1-m01 Round 17

Rounds 14, 15 and 16 were posted while the round 13 runner was still going and the agent session that hosted it had ended (about 03:35Z); they were first read at 03:37Z. This round therefore carries their steps too: the hard stops of round 15, step 0 of round 16 (`repo tools setup all`, once) and the facts of round 17. No separate round 14, 15 or 16 run exists for this machine.

## Before The Run (2026-11-15T03:38:45Z)

- Replica agent: `state = running`; 0 kickstarts. Replica code digest: `97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea`, the same code `97445096cc0a` as g1-server in the replica `status` of 03:45Z.
- Agreement: `status` says `The fleet is not in agreement (10 origins; …)`; g1-server is `ok`, g1-m01 `differs in view.digest at a common checkpoint: #23, #127, #153` (3 of 256 buckets). The first try was not held back for it: the fleet has not reported agreement at any check since 13 November while files keep moving.
- Memory pressure level 1; free disk 82 GiB of 228 GiB.
