# Result F52: dead registrations, no-endpoint, live offload (23:30)

## 1. Why 33 workers for 15 origins (cause, fix, live count)
- They were not dead: every one was seen < 6 s ago. `ps` on this machine showed THREE sidecar launcher trees (parent pid 1) for the same names (agent/cache/scheduler launchers from 01:18, 20:43, 21:30), each with its own live agent child registering as `<host>-<pid>`. E76's idempotence per name+origin cannot merge them (the pid is in the name; and the pid+start identity of today did not change the registration key).
- Mechanism: the replica daemon starts a new launcher whenever the record is stale (> 15 s, launcher starved under load 30-100) or the entry hash differs; the old launcher was never ended because all launchers of a sidecar watched ONE shared lease file `<name>.lease`, which the new daemon keeps renewing for whoever is current.
- Fix: lease per launcher pid (`<name>.<pid>.lease`, daemon renews only the current pid, removes the shared file and the lease of an ended pid). Older launchers, including the old-code orphans that only know the shared path, see the missing lease and leave by themselves. In addition a launcher that finds a newer live launcher in the record leaves at once (en/de log). Scheduler expiry by lastSeen (30 s) was already correct; nothing to change there.
- Live: after the daemon here restarted, exactly one launcher per role (15218 agent, 15219 scheduler, 15220 cache, each with one child), orphans gone. Scheduler log shows the old workers `worker-left` in bulk at 22:20-22:24Z. Status at 23:27: 15 workers for 14 origins, one per origin except g1-m05 (Windows, 2; runs old launcher code until its replica restarts, then its old launcher leaves by itself).

## 2. `no-endpoint` 21:56
- Only 3 occurrences (counter `reason/no-endpoint`, last write 21:56), right at the resolver landing. Cause: the local fleet copy `.cache/f/cache-facts.json` written by the previous code did not validate against the new closed schema, so `readLocalFleetCacheFactsV1` returned undefined and the offload saw no endpoint until the first refresh; and a refresh whose scheduler probe missed once (load) wrote `selectedExecution:null`, erasing a good endpoint for the 30 s TTL.
- Fix: a missed probe keeps the last answering scheduler endpoint while the published origin+port are unchanged (test); on no endpoint the offload does one bounded refresh (TTL 5 s) and resolves again; the decline detail now says why, English / German (no checkout known, no readable copy, no scheduler published, no address of <origin> answers on port <n>). Tests: endpoint 3/0, fleet 15/0.

## 3. Live offload proof
- Scheduler restarted 23:18:53 (new code, counters from 0). At 23:24: completed 1, failed 0, g1-m11 offloadedIn 1, requester counter file `out` written 23:20 (6527 bytes); at 23:27: unchanged, nothing queued, 0 refused, 0 rejected. So one fresh offload completed after the rollout, organically (no unit was started by me: builds are quiet because workspace stages fail on other lanes; a deliberate extra unit was skipped under load 25-30). `no-endpoint` did not recur.

## Not done / not verified
- Windows m05 and the Linux/other Macs reach the new launcher code only when their replica restarts (about 1 s after replication on Macs; m05 by its supervisor).
- Sidecar signed-scheduler test 6/1 (F47) and offload `slotQueueMilliseconds` test (expects pid-only ticket files; F42 changed ticket records) fail independently of this lane; the latter fixture needs `{pid,startMs,...}` tickets.
