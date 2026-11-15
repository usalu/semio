# g1-m01 Round 3

## Before The Run (2026-11-14T17:27:21Z)

Replica code digest (`.cache/f/health.json`): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`

`ps -axo pid,ppid,etime,rss,args` for `show projects` and `service host` (non-ASCII path characters removed):
```
 1517     1       36:01 162976 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo
 1519  1517       35:59 402608 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=awa
```
Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T16:59:46.036Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T16:59:46.112Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T16:59:46.115Z worker registered: 0a50c9ce-a5cb-4c12-a88c-f90dba69b33d cache http://192.168.178.70:6094
2026-11-14T17:23:45.897Z agent code changed, restarting
2026-11-14T17:23:45.900Z worker stopped: ueli-workstation-mac-mini-m6-16gb-256gb-01-lan.fritz.box-6181
2026-11-14T17:23:48.250Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:23:48.300Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:23:48.303Z worker registered: 2c610894-7783-449d-b950-ce9b0d6c8904 cache http://192.168.178.70:6094
```
