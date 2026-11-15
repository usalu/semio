# g1-m01 Round 7

## Before The Run (2026-11-14T20:28:16Z)

Replica code digest (`.cache/f/health.json`, field `code`): `d72ff30ebb826bfabac6ee5f126041ee5f833672897443b9666c557ce85335c1`

`ps -axo pid,ppid,etime,rss,args` of the graph service processes (cut at 260 characters):
```
89646     1       33:35  82608 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo
89655 89646       33:32 445232 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=awa
```

Graph service jobs (`launchctl list`; `launchctl print` lines state, runs, pended nondemand spawn, last exit code):
```
tech.semio.repo.service.c6c0f63aa094
	state = not running
	runs = 1
	last exit code = 4
		state = active
		state = active
tech.semio.repo.service.7a7ed4bee027
	state = running
	runs = 1
	last exit code = (never exited)
		state = active
		state = active
```

Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T20:10:27.433Z worker registered: 9eef2daa-5e7d-4804-b47f-4aec646f9a9c cache http://192.168.178.70:6094
2026-11-14T20:15:26.278Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T20:20:26.925Z agent code changed, restarting
2026-11-14T20:20:26.930Z worker stopped: ueli-workstation-mac-mini-m6-16gb-256gb-01-lan.fritz.box-98089
2026-11-14T20:20:27.808Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T20:20:27.850Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T20:20:27.853Z worker registered: a270fe09-f9cc-4a8a-8f13-cf5c25a6de5f cache http://192.168.178.70:6094
2026-11-14T20:25:25.851Z graph-server: alive, 0 machines seen, 0 requests held, read none
```
