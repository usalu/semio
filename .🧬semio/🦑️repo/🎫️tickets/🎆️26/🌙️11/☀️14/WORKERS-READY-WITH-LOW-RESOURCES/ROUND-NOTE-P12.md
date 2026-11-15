# Round note P12: what to run and what to copy so that the next round names the stage that eats the memory

For the coordinator to post to the workers. Nothing here starts a graph command of its own, kills anything or writes into the worktree; the block only reads (`ps`, `footprint`, `sample`, `tail`, `grep`, `find`) and writes its files next to the report of the round.

## 0. What changes for the worker after the P12 landing (nothing to do, only to copy)

The graph plan of a wave now runs in ten named stages (`inference`, `environment`, `tests`, `policies`, `dangling`, `files`, `test-edges`, `edges`, `metadata`, `result`). On a machine that has a plan limit (every machine below 24 000 MB: a 16 GB worker has 4 915 MB of managed growth) the service log shows one line per stage start and per stage end, in English and German:

```
graph: plan stage tests starts (managed 3412 MB, resident 5120 MB)
graph: plan stage tests took 7.5 s (managed 3980 MB, 568 MB more than at its start, resident 5701 MB)
```

A worker that dies inside a stage therefore ends its log with the `starts` line of that stage and no `took` line. When the plan passes its limit, or the whole worker process tree passes the ceiling of its booking (6 553 MB on 16 GB) the read ends with exit 75 and the service log holds one line:

```
graph-plan-too-heavy {"stage":"edges","megabytes":9400,"limit":4915,"worker":{"heapMb":210.5,"extraMb":5300.2,"objectCount":4000000,"objectTypes":[...]}}
```

`worker` is the last heap sample of the slice worker (at most 2 s old): `heapMb` is the JavaScript heap, `extraMb` is what lives outside it (buffers). That one line decides between "many objects" and "one huge buffer".

Optional, existing flags (they must be in the environment BEFORE the first `repo` command of the round; a service started earlier keeps its old environment): `SEMIO_GRAPH_PHASES=1` adds one `[graph-phase] {...,"rssMegabytes":...}` row per engine phase (verification, plan, patch, pack) to the service log; `SEMIO_GRAPH_WAVE_ROWS=1` adds the wave row with the footprint of the process tree after every published wave. `SEMIO_GRAPH_PLAN_LIMIT_MB=off` switches both new limits off (do not use it on a 16 GB worker; it is for a comparison run on a 32 GB machine).

## 1. Ready-to-paste sampler (start it before the try, stop it after)

zsh, copy as one block. `OUT` is the file the round already writes its samples to (a new file is fine). It samples the slice worker (the process whose command line holds `slice-pool` and `worker.ts`) every 5 s: resident size, physical footprint (this includes compressed and swapped pages, which `ps` does not), the newest progress line of the service log, and, once, a 5 s `sample` of the worker when it passes 4 GB resident.

```zsh
OUT="$PWD/p12-samples.txt"; : > "$OUT"; rm -f "$OUT.stop"
LOG=$(ls -t /var/folders/*/*/T/semio-repo/$(id -u)/host-*.log 2>/dev/null | head -1)
echo "service log: $LOG" >> "$OUT"; echo "closure entries before: $(find "$PWD"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' ')" >> "$OUT"
(
  SAMPLED=0
  while [ ! -e "$OUT.stop" ]; do
    ts=$(date -u +%FT%TZ)
    row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | head -1)
    pid=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
    if [ -n "$pid" ]; then
      fp=$(footprint -p "$pid" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
      last=$(tail -n 600 "$LOG" 2>/dev/null | grep -E 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
      echo "$ts pid=$pid rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$OUT"
      if [ "$rssmb" -gt 4096 ] && [ "$SAMPLED" = 0 ]; then SAMPLED=1; sample "$pid" 5 -file "$OUT.sample-$ts.txt" >/dev/null 2>&1 & fi
    else
      echo "$ts no slice worker" >> "$OUT"
    fi
    sleep 5
  done
) &
SAMPLER=$!
```

After the try (whatever its outcome):

```zsh
touch "$OUT.stop"; wait $SAMPLER 2>/dev/null
echo "closure entries after: $(find "$PWD"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' ')" >> "$OUT"
```

(`$PWD` must be the worktree root of the worker for the `find` lines; the number of files in `ce/e` before and after tells how many closure entries this machine derived itself during the try: the server has 40 675.)

## 2. What to copy into the report of the round

1. The whole `p12-samples.txt` (first 3 lines, then every line from 4 samples before the first `footprint` above 6 000 MB until the end), plus `p12-samples.txt.sample-*.txt` as far as its section `Sort by top of stack` (about 40 lines).
2. From the service log (`LOG`): `grep -h -E 'graph: plan stage|Graph: Plan-Stufe|graph-plan-too-heavy|graph: refreshing|graph: [0-9]+ stale projects|taken from the fleet|\[graph-phase\]' "$LOG"` (verbatim, in order).
3. The last 40 lines of the bootstrap log (as before) and the exit code (75 is the typed refusal; any other code with a hard stop is the old failure).
4. The two `closure entries` numbers of the sampler.

## 3. What the lines decide (for the coordinator)

| You see | Meaning | Next |
|---|---|---|
| the last `starts` line is `inference` and `closure entries` grew by thousands | hit owners derive their closures (the records carry questions, not answers) | package "closure entries travel with the project records" (report P12, section 4) |
| the last `starts` line is `tests` or `policies` | the whole test discovery ran because a `.feature` or `script.ts` changed or the environment moved | retain the tagged owners across a moved environment / carry the shared walk |
| the last `starts` line is `edges` or `metadata` | the edges of every kept owner were refreshed because no edge consumers were known | carry the edges of owners whose reads did not change |
| `extraMb` above `heapMb` in the `worker` sample | a buffer, not objects (an index, a row log or a brotli output) | read the `sample` file: top of stack names the allocator |
| `heapMb` above 3 000 and `objectTypes` led by one type | retained objects of that type | the type names the structure |
| exit 75 `host-graph-plan-too-heavy` and no hard stop of the runner | the guard worked: the machine did not reach pressure level 4 | the stage in the message is the work item |

## 4. P12b (after round 13): what the next worker try must show for the `environment` stage

Cause found (details in `result-P12.md`, section "P12b"): the stage name was right, the function was not. `deriveEnvironmentV1` costs 0.4 to 0.9 s and 131 MB on the live tree (it reproduces the digest of generation 201 exactly). What grew 5 GB during the stage is the SWEEP OF THE CLOSURE STORE (`collectClosureStoreV1`, started by the first closure entry a process saves, at most every 10 minutes): it read and kept every parsed entry with all its observation rows (41 429 entries, 3.8 GB of rows on the server: 5.9 GB resident in a scratch replay). It is now streamed and holds a few hundred megabytes.

The next try (g1-m02 or any 16 GB worker) must show:
1. NO `graph-plan-too-heavy` and no exit 75 from `environment`; the lines `graph: plan stage environment starts (managed A MB …)` and `graph: plan stage environment took S s (managed B MB, D MB more …)` with S below 10 s and D below 700 MB (before: 4 953 MB and no `took` line).
2. Every following stage line (`tests`, `policies`, `dangling`, `files`, `test-edges`, `edges`, `metadata`, `result`) with its seconds and growth; the sum of the growths is what a wave of 48 owners costs the plan. The first stage with a growth above 1 500 MB is the next work item.
3. If the guard still stops: the log holds `graph: plan stop detail {...}` (and the German twin) written at the moment of the stop: `memory` (rss, heapTotal, heapUsed, external, arrayBuffers in MB: this decides heap against buffers with fresh numbers, the `worker` sample of the refusal line is up to minutes old), `heap.objectTypes` (fresh), `resources` (pending native requests by kind) and `recent` (the last 40 samples every 250 ms with the operation that was running). The refusal text now names the operation: `the plan stage <stage> (during <operation>) grew to <n> MB`.
4. The sampler's `ce/e` count after the try is unchanged or higher, never lower than before by more than the number of superseded entries (the sweep may remove entries older than one hour that a newer one supersedes).
