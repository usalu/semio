# Replica Stage Publication and Durable Hints

## Observed on 21 November 2026 UTC

- At 17:06 all nine reporting worker journals had advanced past exact publication events for the nine recent dependency readiness and graph-recovery files. The preceding stage and replica implementation had also reached all nine reporting workers.
- The coordinator then observed stage `00e8597ed4e08f28efd176ba008711b951bbb801`, up from `66b378bec2e6c8b8407588b2018e152207de9763`. Git reported 1,703 changed paths and 1,301 distinct nonzero object IDs in the diff. The journal advanced from `184:59295` through 1,048 Git prerequisite PUTs and 15 ordinary PUTs before publishing the stage registers. The first prerequisite HLC was 17:10:14.099 UTC. The 1,694 stage links and nine deletions then committed in about 60 ms at 17:11:38 UTC. The journal was moving throughout the prerequisite work.
- At the observed `184:596591` cursor, 729 of the 1,703 stage records had arrived. Workers `g1-m02` and `g1-m11` attested the intermediate tree `dd221e872ce89e8a9f67049a9a1615b0ca70eab6`. Another worker briefly reported no certified tree. All nine reporting workers subsequently reached `184:956031` and attested the exact completed stage by 17:18 UTC. The coordinator had already observed a newer local tree, `8302e0daa30e4b66af4b9c12861286b2e23f66b3`, at the older published cursor.
- A separate restart hazard was present in the source hint reader. It unlinked hints before publication and held the paths in process memory. A natural restart could lose the notification. The live delay above also had a directly measured Git prerequisite bottleneck.

## Changes

- The author publishes every record of one index transition with a single journal head update, including transitions spanning segments. The journal encoder retains one segment of encoded events and bounded lookahead. Readers continue to see the previous committed head until that update.
- Followers pin the author head sampled at the start of intake and read only up to that committed position across bounded passes. They continue applying ordinary work and required objects while keeping the prior stage during a partial author read. New author writes cannot move the pinned target. Existing Git object and lock checks still apply before stage materialization.
- Store guards renew on an advancing, successfully committed outbound position, including puts, deletes and stage-register appends. Empty passes and failed journal writes leave the deadline unchanged.
- Exact change hints remain on disk until the replica has settled their paths. Partial receipts, retryable I/O, unresolved state, and active incoming paths remain pending. An author index hint also requires the published index signature. The reader bounds file count and bytes per polling pass, visits later files even if earlier files are unreadable, and checks file identity before acknowledgement. One blocked receipt cannot delay successful acknowledgements for the rest of the batch. Ordinary hints run ahead of maintenance and an explicitly hinted stage operation.

## Verification

- The real-Git replica regression previously installed a synthetic tree after the first 1,024 events. It now applies the ordinary ignore-file update while preserving the old index, installs the exact committed stage at the pinned position, and applies a later ordinary author event on the next follow. Git constructs the independent expected tree.
- The final real-Git replica module passed 40 tests and 542 assertions, including follower handoff, the pinned author position, the greater-than-5,000-entry stage, and stage priority under unrelated work.
- The hint filesystem suite passed 64 tests, including the Python oracle, denied-unlink retry, a replaced receipt, more than 64 inaccessible inputs, and continued processing across byte and path bounds.
- Replica filesystem tests passed for both exact-path and directory receipts surviving a restart after partial publication. A separate test retained an exact path and its directory while an incoming descendant was missing from disk. SQLite supplies the independent path and event expectations.
- The native Git suite passed 66 tests with a 60-second runner limit under current host load. The original five-second run hit deadline limits in nine cases. After the author append change, three focused Git tests passed, including the 6,000-entry single-batch case.
- The final replica race suite passed 36 tests and 262 assertions. The outbound guard test failed against the old code with two committed batches and zero heartbeats. It now checks that a denied write and a no-op add no heartbeats while the committed PUT and DELETE renew the deadline.
- The journal suite passed 131 tests and 310 assertions. Its multi-segment fault fixture left the original canonical and flat heads visible, and native Python decoded exactly the expected JSONL order after retry. The replica torn-short-segment regression also passed.
- A focused hint test initially used a source directory as its temporary root. Cleanup completed and a bounded check found no remaining test directories there. Subsequent checks used the ticket-generated directory.
- The native touch command accepted the ten changed source and test paths for normal publication. Live observations used the existing replica and journals.

## Live rollout at 17:43 UTC

The coordinator restarted through the existing supervisor and its native self-check verified replica digest `b1ec6c3804c482a7f6aa10c134de4116c8abdf72e0549282a04ad6439644a277`. It loaded 34 modules, compiled 23 schemas and validated three multilingual text catalogues. The coordinator reported a healthy store and current loop status.

The author committed a later 248-entry stage at `185:151337` on 21 November at 17:42:08 UTC. By 17:43:20 UTC all nine fresh workers attested the exact coordinator tree `7d8c0b059bcaed05be881279fe59b4fa8305d514`. Their author cursors were at or beyond `185:157955`. Four of the nine were running the new replica digest at that sampling point. All nine worker cursors covered exact published PUTs for all ten changed source and test paths.

The original isolated launcher process and its native runner were no longer present. Its last log entry recorded a wait at the physical disk gate and no later machine receipt was present. A fresh preflight admitted the path envelope, with a 249-unit maximum clone path. It measured 41,136,533,504 free bytes against the 60,129,542,144-byte floor and therefore deferred execution. The live graph reader remained queued for its rebuild reservation and no worker had a current Ready receipt.

Eight workers still had fresh stage records at 17:53 UTC, each with the exact coordinator tree. The ninth match recorded at 17:43, `g1-m02`, subsequently went stale. The cooldown follow-up passed all 36 native race tests and the native self-check, and the coordinator started its new digest `5833bd94f16502c16334f2093620201382811632f28abe01fca301731285a7c9`. Five of the eight fresh workers reported that digest at the next sample.

The signed graph-adoption verifier returned `receipt-missing` for all twelve enrolled worker destinations. The coordinator's live graph reader still needed a 14,336 MiB rebuild grant. At 17:54 UTC the repository doctor measured 4,414 MiB of headroom above the 4,096 MiB host floor.

## Open acceptance

Workers `g1-m02`, `g1-m04` and `g1-m06` were stale in the latest sample, and `g1-m09` remained absent. Current authenticated Ready on the enrolled fleet, physical replay in the isolated 8,192 MiB lab, and authenticated whole-graph adoption by a compatible second physical worker are pending. The isolated host remains below its 60,129,542,144-byte disk admission floor. The local graph pointer is still quarantined for an incomplete closure, and its live reader is waiting for the 14,336 MiB rebuild reservation.

[Targeted closure and prior stage verification](verification-targeted-closure-stage-2026-11-21.md) · [Dependency readiness evidence](audit-ready-dependency-inputs.md)
