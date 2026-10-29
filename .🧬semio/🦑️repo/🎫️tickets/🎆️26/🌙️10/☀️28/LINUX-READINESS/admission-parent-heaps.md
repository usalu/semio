# Parent Heap and Child Admission Observation

Read-only live observation, no process changes or guard bypass. Largest Bun processes retain captured build/graph heaps; this supports memory pressure but does not establish a deadlock.

| PID | Relationship/state | Approx RSS MiB | CPU tick increase over6.332s |
|---:|---|---:|---:|
|194452|sleeping parent,child212277|2079|160|
|212277|running graph child|1427|609|
|211237|running graph process|2240|598|
|242964|running container graph/test process|1327|2052|
|229702|sleeping captured build process|1657|49|
|249683|sleeping captured process|854|38|

Clock ticks are raw /proc utime+stime deltas, not normalized percentages; multicore threads may consume more than one CPU per wall second. Every sampled process made CPU progress. Parent sleep state was epoll waiting while its graph worker ran.

Authoritative admission snapshot:425MB spare above1981MB floor; five granted/seven waiting. Parent194452 requests additional exec3584MB while outer194086 retains a grant whose actual tree use4578MB exceeds reserved1638MB. This is pressure and underforecasting evidence. Graph activity advanced197/568 from earlier122/568, though slowly; other graph0/128 processes consumed CPU, so completion counters alone hide expensive first-project work.

Source project-farm/🟨️.mjs56–68 starts a worker pool from the existing grant, then requests additional capacity asynchronously through capacity.granted.then. prepare73–87 uses whichever funded pool is ready and returns false for no pool, allowing parent inference. Thus queued supplementary capacity is not a mandatory child required for current progress. close89–96 releases pending capacity when all pools use own grant before awaiting opening, preventing that completion wait from requiring ungranted supplementary work.

No strict circular admission deadlock was proven. Retained source-capture heaps plus long first-project CPU work make severe contention plausible. Safe next evidence is bounded per-phase CPU/bytes/heap instrumentation in a future controlled invocation, and correct parent/child peak forecasting. Do not free memory by killing external peers, dropping source custody or bypassing admission. Tiny verification requests occupy relatively little RSS but still need their proper reserved capacity.
