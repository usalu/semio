## Result

Not ready: twelve tries, none reached the setup tasks, the marker `.cache/f/native-bootstrap.done` does not exist. `native_rc` of the last try: 75. Step 0 (`repo tools setup all`) ended with exit code 0 after 1895 s.

## What Differs From The Round Text

- The runner ran unattended from 03:38Z to 08:39Z with the rules of round 17 as read at 03:37Z. Rounds 18 to 27 were posted meanwhile and were first read at 09:10Z; their extra copies (`access-proof` line per try, `ls -laO@`, `mount | grep -c semio`, `ps … service host` right after a failure) were not taken at the time of the failure. What the service logs still hold is copied per try below; the last 40 bootstrap lines of every `native_rc=1` try are in `round-17.bootstrap-tails.txt`.
- Tries 4 and 5 were stopped by my runner at memory pressure level 4 in two samples in a row (rule of round 15). Round 23 (05:44Z) removed that stop: try 4 was stopped at 05:42:54Z before it, try 5 at 06:10:28Z after it, against the then current rule. The round 27 runner has no pressure stop.
- Sample files with a colon in the name were renamed at 09:20Z (`round-13.p12-try-*.sample-…T014525Z.txt`, `round-17.p12-try-*.sample-…Z.txt`), round 27.

## Observations For The Coordinator

- Resident size and `footprint` of the slice worker disagree in both directions (same pid, same second): try 12 at 08:36:50Z 8175 MB resident with footprint 2682 MB; round 27 try 1 at 09:48:42Z 2206 MB resident with footprint 9589 MB. Which of the two the plan guard (`grew to 10035 MB`) reads: not measured.
- Restores seen in this round: generation 222 from a state 106 and 117 min old with 73 projects stale, generation 229 from a state 28 min old with 3146 projects stale (remote cache, 5104 documents, 14 to 22 s). Tries without a restore line inferred here from the start.
- Try 12: `waiting for memory headroom (4453 of 0 MB reserved, 1 projects running)`: the reserve total prints as 0 MB.
- Tries 8 to 12 took `0 projects … from the fleet` in the sampler's last `graph:` lines and inferred here (50, then 150 in try 12) until the guard stopped the plan; closure entries on this machine grew 42033 to 42177 in try 12, so part of the inference is kept for the next try.
- Access: tries 7, 8, 9, 11 and 12 print that the launchd route was refused for this boot and the service runs under the calling app; try 7 ended there (direct service refused too), the later ones went on.
- Other memory users on this machine during the round (30 s samples, three largest processes) are in `round-17.samples.txt`.
