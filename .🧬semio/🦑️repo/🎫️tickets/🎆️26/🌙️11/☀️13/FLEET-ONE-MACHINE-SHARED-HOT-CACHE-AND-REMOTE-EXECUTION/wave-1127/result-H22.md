# H22: fseventsd growth on 16 GB workers

Verdict: the SIGSTOP hypothesis is refuted on this macOS (Darwin 27.2). No product code changed, no layout move landed (nothing to land that the data supports).

## 1. Hypothesis test (own throwaway bun processes, scratch under the Data volume, nice 10, all pids killed afterwards)
fseventsd RSS (KB) via `ps -o rss -p $(pgrep -x fseventsd)`; client = `fs.watch(dir, {recursive:true})` (an FSEvents stream; events counted, so the stream was proven live).
| run | client state | churn | fseventsd RSS |
|---|---|---|---|
| A | SIGSTOP | 36k events (500 paths reused) | 3.5 MB -> 5.5 MB, back to 4.4 MB after 10 s, 5.2 MB after SIGCONT |
| B | SIGSTOP | 240k events, 80k unique long paths | 4.3 MB -> 8.0 MB -> 7.5 MB; SIGCONT: 7.6 MB, 20 s later 4.1 MB |
| C | 8 streams SIGSTOP | 6 x 9000 file create/unlink in 130-char emoji paths | 5.4 MB -> 8.2 MB peak; after SIGCONT 6.3 MB |
| D | SIGSTOP stream on the whole checkout (live replica churn about 150 events/s) | 100 s live, 160 s stopped | 4-6 MB throughout, no trend; resumed stream delivered the events |
A stopped client does not make fseventsd queue unboundedly: growth is bounded by a few MB (the kernel/fseventsd drop or coalesce for a client that does not read). Runs A-B-C-D show no drain needed because nothing accumulated. Note: the first attempt of C failed to stop the watchers (zsh does not word-split `$pids`); the valid C is the array form; no pid leaked (checked).

Consequences: step 2 (warden must not leave stream holders stopped) is NOT justified by this evidence and I did not change the warden (`suspend` on `cargo`/`test`/`slice-worker` kinds stays). m07's `memory.json` at the time of reading shows `suspended: []` with fseventsd at 2390 MB, so the large fseventsd happens with no stopped process at all.

## 2. Event rates per tree (counter `T/🧰️scripts/H22/fsevents-count.mjs`, 90 s window, no sudo, scaled to 10 min)
`.cache/b` (image mounts, history silenced) 25,100; `$TMPDIR` 12,600; `.cache/f` 7,040; `.cache/scratch` 460; `bootstrap/code` 347; `.cache/test-sandbox` 0; `.cache/command` 0; `.🧬semio` 20; whole checkout about 90,000 (run D, replica peers).
The trees H13 proposed to move (`.cache/f`, `.cache/scratch`, `$TMPDIR`) sum to about 20k per 10 min: under a quarter of the checkout's own churn that cannot move, and runs B/C show 240k events cost only a few MB. Moving them into a silenced image volume would change native-build image classes, boot recovery and partition leases and restart every worker supervisor for no measurable fseventsd gain. Not landed; before/after measurement is therefore n/a (before numbers above).

## 3. What the data does say
- The coordinator's fseventsd was 9.6 GB (wave 1122 brief), is 6 MB now with the same uptime (2 days): fseventsd releases the memory itself once the burst ends; RSS is transient. m07/m08 (2.4-3.7 GB) were sampled during or after a burst.
- Remaining candidates, none provable without sudo on the worker: (a) a burst far above 240k events (initial checkout materialisation on a fresh worker: the bootstrap/code publication and replica sync write hundreds of thousands of files in minutes); (b) a client that starts a stream with `sinceWhen` of an old event id (history replay loads the on-disk log); (c) fseventsd pages compressed/swapped, RSS low but footprint high.
- Human with sudo on m08 (30 s): `sudo fs_usage -w -f filesys | head` grouped by process while fseventsd is big; `sudo launchctl kickstart -k system/com.apple.fseventsd` (respawns, frees the memory at once, no data loss: history is rebuilt).

## Not verified
m08 itself (no access without sudo); a multi-million-event burst; bun's recursive watcher on the 16 GB workers' macOS version (same major). Kept script: `T/🧰️scripts/H22/fsevents-count.mjs <seconds> <dir>...`.
