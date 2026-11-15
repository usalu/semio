# Verification rate of the project graph slice (describe15)

Measured on the live run `describe15` (log `🗑️generated/describe15.log`), no code changed, no restart.

## Rate over time (verification N of 3420, seconds since slice start)
| t (s) | done | note |
|---|---|---|
| 875 | 0 | first heartbeat |
| 956 | 31 | cold start, load avg 60 |
| 974 | 194 | rate turns up, load falls |
| 994 | 465 | ~15/s |
| 1028 | 1136 | |
| 1090-1191 | 1667-1785 | ~0.5/s, rederivation waves of 171-185 owners |
| 1247 | 2904 | ~20/s |
| 1255 | 3388 | ~20/s |
| 1255-1712+ | 3388 | the last 32 entries still running |

So the "8/min" reading was the cold start under load average 60 plus one rederivation wave. Over the 380 s from 875 s to 1255 s, 3388 of 3420 records verified, about 9/s on average; a 7 h projection is wrong.

## Where the time goes now
- Records are processed in chunks; each chunk ends with `rederivation`, `flush`. The heartbeat keeps printing the last event (`flush 1 of 1`) until the next chunk reports its first completed entry, so a long `flush 1 of 1` is the next chunk working, not a flush.
- The worker (pid 59539) is mostly off-CPU (2-14 % CPU, RSS 0.6-1.4 GB) with 150 MB/s of disk traffic: it is reading and decompressing the heavy-tail entries (native-target entries of 35-300 MB text, 3295+ layers each; see farmhits.md: median entry is ~0 MB, ~10 % carry almost all the cost).
- The only socket of the worker is the fleet cache server (192.168.178.70:6094); 18 KB in / 71 KB out in 30 min, so it is not network bound and no per-record fleet round trip exists.
- Native frames of the stripped bun could not be symbolised; no JS-level profile was possible without restarting the host (forbidden here).

## Fix status
- No new code: the per-pass row memo, whole-layer parse and verdict sharing from hitcost.md and farmhits.md are already in the running code and explain the fast 3388. The remaining cost is the structural one named in both notes: layers store repeated rows (97 % repeats); the fix is layers that store row ids (store format change, one re-derive of 10.6k entries). That is a multi-hour change with a store migration cost and is not done here.
- Needs a service restart: no. Restart describe15: no, it is past verification and finishing the heavy tail; restarting repays the whole 875 s read.
