# Round 31 Observation of g1-m02

Written 2026-11-18T09:32:28Z; every 10 minutes. No setup command is started by this session.

## Ready State (`.cache/f/ready`)

```
state.json: {"version":1,"stages":{"native":{"digest":"8d4e90aeb62ef77a55eaf66b71cddbf3c039e58ce0e89aaeb64418cf427c8de2","doneMs":1794770197903},"repo-mcp":{"digest":"cfc00e3a2468de5fae6fcd67ea7a43e9b7f0e4c19785d4f9c0e9f7fb8db2fd8f","doneMs":1794770209401},"dependencies":{"digest":"71e9d68250c5a128a9f8b74af62292acf4c04e9892c05bef2e4ce4a5986fc2be","doneMs":1794770218926},"tools":{"digest":"1d307cfc1e9f353c3daad9f505245b936e81634c30f40ebcc8bb2f55a36952c4","doneMs":1794771472483},"workspace":{"digest":"a17ef0f43bdb651b95f1c3e5f6beb18aa2c2d55f979586821bc74a20b25bc147","doneMs":1794993907570}},"failure":null,"attempts":0,"peakMegabytes":2825,"readyMs":1794993907570,"adopted":false}
running.json: (absent)
lease.json: (absent)
ready-timings.json: {"version":1,"origin":"g1-m02","runs":[{"stage":"workspace","startedMs":1794881990194,"endedMs":1794884658450,"seconds":2668,"exit":1,"outcome":"stage-failed","peakMegabytes":83,"attempts":25,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794885858917,"endedMs":1794889934938,"seconds":4076,"exit":1,"outcome":"stage-failed","peakMegabytes":175,"attempts":26,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794891135390,"endedMs":1794894658266,"seconds":3523,"exit":null,"outcome":"stalled","peakMegabytes":176,"attempts":27,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794894659254,"endedMs":1794898731749,"seconds":4072,"exit":1,"outcome":"stage-failed","peakMegabytes":94,"attempts":27,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794899932223,"endedMs":1794903466988,"seconds":3535,"exit":null,"outcome":"stalled","peakMegabytes":179,"attempts":28,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794903469118,"endedMs":1794904086457,"seconds":617,"exit":1,"outcome":"stage-failed","peakMegabytes":162,"attempts":28,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794905287140,"endedMs":1794908091360,"seconds":2804,"exit":1,"outcome":"stage-failed","peakMegabytes":177,"attempts":29,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794909291918,"endedMs":1794909439284,"seconds":147,"exit":1,"outcome":"stage-failed","peakMegabytes":175,"attempts":30,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794910639961,"endedMs":1794912932202,"seconds":2292,"exit":null,"outcome":"stalled","peakMegabytes":103,"attempts":31,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794912934424,"endedMs":1794931278442,"seconds":18344,"exit":1,"outcome":"stage-failed","peakMegabytes":61,"attempts":31,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794932479859,"endedMs":1794932765202,"seconds":285,"exit":1,"outcome":"stage-failed","peakMegabytes":99,"attempts":32,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794933966743,"endedMs":1794934256570,"seconds":290,"exit":1,"outcome":"stage-failed","peakMegabytes":97,"attempts":33,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794935460291,"endedMs":1794937047910,"seconds":1588,"exit":null,"outcome":"stage-failed","peakMegabytes":125,"attempts":34,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794938303945,"endedMs":1794940494821,"seconds":2191,"exit":null,"outcome":"stage-failed","peakMegabytes":80,"attempts":35,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794941759223,"endedMs":1794942281354,"seconds":522,"exit":null,"outcome":"stage-failed","peakMegabytes":99,"attempts":36,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794944057264,"endedMs":1794945930795,"seconds":1874,"exit":null,"outcome":"stage-failed","peakMegabytes":112,"attempts":37,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794947139835,"endedMs":1794948392152,"seconds":1252,"exit":null,"outcome":"stage-failed","peakMegabytes":54,"attempts":38,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794949603381,"endedMs":1794949651527,"seconds":48,"exit":null,"outcome":"stage-failed","peakMegabytes":59,"attempts":39,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794950868488,"endedMs":1794952685765,"seconds":1817,"exit":null,"outcome":"stage-failed","peakMegabytes":68,"attempts":40,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794953941571,"endedMs":1794954421264,"seconds":480,"exit":null,"outcome":"stage-failed","peakMegabytes":134,"attempts":41,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794955632316,"endedMs":1794956333959,"seconds":702,"exit":null,"outcome":"stage-failed","peakMegabytes":81,"attempts":42,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794957536223,"endedMs":1794958010586,"seconds":474,"exit":null,"outcome":"stage-failed","peakMegabytes":132,"attempts":43,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794959225420,"endedMs":1794960642917,"seconds":1417,"exit":null,"outcome":"stage-failed","peakMegabytes":65,"attempts":44,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794961861594,"endedMs":1794963601404,"seconds":1740,"exit":1,"outcome":"stage-failed","peakMegabytes":66,"attempts":45,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794964802880,"endedMs":1794966304512,"seconds":1502,"exit":null,"outcome":"stage-failed","peakMegabytes":85,"attempts":46,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794967506662,"endedMs":1794969309034,"seconds":1802,"exit":null,"outcome":"stalled","peakMegabytes":135,"attempts":47,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794969309538,"endedMs":1794971057342,"seconds":1748,"exit":null,"outcome":"stage-failed","peakMegabytes":79,"attempts":47,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794972261394,"endedMs":1794972263458,"seconds":2,"exit":1,"outcome":"stage-failed","peakMegabytes":null,"attempts":48,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794974916106,"endedMs":1794975609590,"seconds":693,"exit":null,"outcome":"stage-failed","peakMegabytes":76,"attempts":49,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794976922134,"endedMs":1794977070482,"seconds":148,"exit":null,"outcome":"stage-failed","peakMegabytes":121,"attempts":50,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794978289208,"endedMs":1794978290378,"seconds":1,"exit":1,"outcome":"stage-failed","peakMegabytes":null,"attempts":51,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794979491455,"endedMs":1794979943626,"seconds":452,"exit":null,"outcome":"stage-failed","peakMegabytes":130,"attempts":52,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794981156751,"endedMs":1794981157909,"seconds":1,"exit":1,"outcome":"stage-failed","peakMegabytes":null,"attempts":53,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794982358996,"endedMs":1794982446121,"seconds":87,"exit":1,"outcome":"stage-failed","peakMegabytes":152,"attempts":54,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794983646963,"endedMs":1794984635735,"seconds":989,"exit":1,"outcome":"stage-failed","peakMegabytes":141,"attempts":55,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794985836657,"endedMs":1794986643962,"seconds":807,"exit":null,"outcome":"stalled","peakMegabytes":140,"growthMegabytes":0,"attempts":56,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794986674485,"endedMs":1794989131442,"seconds":2457,"exit":null,"outcome":"stage-failed","peakMegabytes":981,"growthMegabytes":null,"attempts":56,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794990338249,"endedMs":1794990874225,"seconds":536,"exit":null,"outcome":"stage-failed","peakMegabytes":106,"growthMegabytes":2292,"attempts":57,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794992086505,"endedMs":1794992454517,"seconds":368,"exit":1,"outcome":"stage-failed","peakMegabytes":83,"growthMegabytes":1191,"attempts":58,"digest":"a17ef0f43bdb"},{"stage":"workspace","startedMs":1794993654983,"endedMs":1794993907512,"seconds":253,"exit":0,"outcome":"ok","peakMegabytes":139,"growthMegabytes":0,"attempts":58,"digest":"a17ef0f43bdb"}],"updatedMs":1794993907512}
```

## Last 30 Lines of the `none` Log

```
(no stage running)
```

## Memory

`memory_pressure | tail -1`: System-wide memory free percentage: 80%

Five largest processes by resident size (KiB):

```
70921 1156896       51:31 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts --boo
 2089 1135920    03:19:45 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await imp
 6121 923584       14:25 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes
13000 647264       00:08 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧮️derived-nodes/🧩️closure-k
43836 410016 03-05:14:09 /Users/ueli/Library/Application Support/Claude/claude-code/2.1.325/485fdac4332e/claude.app/Contents/MacOS/claude --output-format stream-json --verbose --input-format stream-json --effort=medium --model=claude-fable-5-2 --permission-pro

```

Permission dialog on screen: not measured (this session cannot see the screen).

## Ready (2026-11-18T09:53:46Z)

`ready.json` has readyMs 1794993907570. `repo preflight --lang en`: exit code 0. Last 20 lines without `[script] {` rows:
```
[capacity] command: sized at 21% of the default from 7 earlier run(s) of its class
[capacity] admitted command: 1 job(s), 512 MB reserved (11018 MB available, 2048 MB floor)
[capacity] command: sized at 21% of the default from 7 earlier run(s) of its class
[script] build:source-admission · Admitting the sources of the build
[script] build:implementation-source · Reading the sources of the build implementation
```

Replica status:
```
Background tiers, last pass (cost, duty cycle of one core): hot 5 ms 1.6%, directory sweep 327 ms 10%, file sweep 3808 ms 3.7%, tree walk 27647 ms 1.5%, git view 224411 ms 5%.
Latency over the last 19 publishes and 100 applies: publish p50 198 ms, p95 1563 ms; apply p50 191 ms, p95 811 ms.
g1-m01: 0 events (0 B) behind, last event 27s ago
g1-m02: 0 events (0 B) behind, last event 506s ago
g1-m03: 0 events (0 B) behind, last event 548s ago
g1-m04: 0 events (0 B) behind, last event 129272s ago
g1-m05: 0 events (0 B) behind, last event 3539s ago
g1-m06: 0 events (0 B) behind, last event 174104s ago
g1-m07: 0 events (0 B) behind, last event 769s ago
g1-m08: 0 events (0 B) behind, last event 1081s ago
g1-m10: 0 events (0 B) behind, last event 5572s ago
g1-server: 0 events (0 B) behind, last event 737s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 1s 0/0 cp:1s on ca097b4fae32 3829/44/341 140/273|227/850 ok
  g1-m02 macos 2s 0/0 cp:2s on ca097b4fae32 3829/44/341 198/1563|191/811 ok
  g1-m03 macos 1s 0/0 cp:1s on ca097b4fae32 3829/44/341 488/709|193/820 ok
  g1-m04 linux 128806s 8/0 cp:128806s on b4c6513b87ef 452/11/1504 190/16614|302/142793 stale (attestation too old)
  g1-m05 windows 5s 0/228 cp:- on ca097b4fae32 3829/44/341 115501/115501|7619/149929 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 173518s 9/0 cp:173518s on b4c6513b87ef 365/11/1142 139/508|307/841 stale (attestation too old)
  g1-m07 macos 1s 0/0 cp:1s on ca097b4fae32 3829/44/341 288/1315|195/1122 ok
  g1-m08 macos 2s 0/0 cp:2s on ca097b4fae32 3829/44/341 188/278|661/2180 ok
  g1-m10 macos 1s 0/0 cp:1s on ca097b4fae32 3829/44/341 -/-|338/1130 ok
  g1-server macos 2s 0/0 cp:2s on ca097b4fae32 3829/44/341 678/1683|689/4268 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  warning: g1-m09 is registered in the store (fleet-auth.json) but has never attested: start the replica on that machine (run the provisioning script there, then `status`), or take it out of the fleet with `retire g1-m09`
The fleet is not in agreement (10 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```
