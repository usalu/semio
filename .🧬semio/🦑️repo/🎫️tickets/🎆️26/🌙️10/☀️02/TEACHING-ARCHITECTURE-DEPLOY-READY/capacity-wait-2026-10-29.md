# Current Capacity Wait

The official read-only repo capacity_status query sampled the host at1793294449560. The live dev grant reserved5946MB and observed2677MB. The normal affected course test grant (PID1334222, handle85976) reserved6802MB and observed754MB. Available memory was8950MB; after the1981MB floor and6048MB of unrealized reservations, admission headroom was921MB.

Seventeen requests were waiting, including full browser87054 and caller40304. The earliest request needed2048MB and was waiting on memory; later requests waited on queue order. No reservation, floor, estimate, queue, process or policy was changed.

A bounded read-only /proc parent-chain check found none of those waiting PIDs descended from the current test grant. The evidence therefore does not establish a nested capacity deadlock. Several waiters belong to independent check scripts and other agents; their work must be preserved. Existing exact job handles are retained rather than duplicated or restarted. The authoritative host sample can change, so these figures are a point-in-time observation.
