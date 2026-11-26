# F79: owner pool refused 507 capacity, then no-answer, then 503

## Cause (real numbers, read-only)
Server: the fleet cache server (pid 17090, port 6094, role cache-host). Its host CAS store `~/.cache/semio/cas` answers every blob PUT through `CasCapRefusalV1` / `disk.low-space` as 507 (`remote-cache/💾️store`: `hostPutFailureV1` -> "capacity", `🖥️server` -> 507).
- Volume `/System/Volumes/Data`: 926 GiB, 859 GiB used (96 %), 36.7 GiB free.
- Hard free-space reserve for cache-host: max(8 % = 74.08 GiB, 72 GiB floor) = 74.08 GiB. Free (36.7) is 37 GiB BELOW the reserve, so the cap in force = store + free - reserve = 52.67 + 36.7 - 74.08 = ~15.3 GiB, while the store holds 52.67 GiB (census: 48368 objects; 51.65 GiB "held" by two live unit-cache holders, 25.98 + 25.68 GiB; 1.0 GiB pinned; 0 unreferenced / expired / leased). Nothing is collectable: unreferenced = 0 and held entries are only evictable against the proportional cap (about 62 GiB), by design. So the 507 is legitimate: the volume, not the store, is exhausted, and the reserve refuses all writes (including the few-KB owner pool manifest). Server counters: cas puts 0, rejected 1,131,336.
- Not a leak: both holders are live (renewed, roots exist), role record is live, usage.json and census agree (56.55e9 bytes, census 15 min old).

## Why the cycle 507 -> no-answer -> 503 (code, not luck)
1. The guard (`🗄️cas/🚦️guard`) waited up to `admissionBudgetMs` = 30 s per attempt, up to 3 attempts (and the critical-partition and other-scope waits were unbounded: they awaited the whole collection scan of 48k objects).
2. The graph exchange client abandons a blob PUT after `blobMilliseconds` = 20 s -> "no-answer / cas-put / 20 s timeout" although the server was still working.
3. The server keeps the request slot of the abandoned put. Slots are capped (`maximumConcurrentRequests`, at least 16); when they are full every request, even cheap GET/HEAD, is answered `503 Busy`. So one blocked put starves the rest: yes, by this slot accounting.
4. Host health: process alive, RSS 1.19 GiB, 47 % CPU, counters.json rewritten within 4 s (event loop turns); but the machine has load average 33, swap 5.9 of 7.2 GiB used, which slows every turn. I ran no probe request (seal window).

## Change
- `🗄️cas/🚦️guard/🟦️.ts`: one admission deadline for the whole `admit` (all attempts, the sweep, the collection wait and the other-scope wait are raced against the remaining budget); when it is spent a refusal is thrown at once, so the answer (507) always arrives.
- `🗄️cas/🧬️schema/🔣️.json`: `admissionBudgetMs` 30000 -> 15000 (below the 20 s put deadline of the graph client).
- `🪪️graph-state/🛰️exchange/🧯️refusal/🧬️schema/🔣️.json`: `bodyCharacters` 160 -> 640, so the server's bilingual capacity detail (bytes, cap, free, reserve, holders) is not cut off.
- `🕸️graph/.../owner-pool/🟦️.ts`: the logged `owner pool refusal` of code `capacity` now carries `detail`, the server's own English / German text with the exhausted resource and numbers (only when the body starts with the server's `Insufficient storage: `; any other response text stays out of the log).
- Not changed: the hard reserve and the held-entry protection; evicting a live unit cache to refill a volume that other data fills would trade the build cache for a refusal that is correct. The volume needs 37 GiB freed outside the store (a human or the machine's disk guard), then writes resume by themselves.

## Tests (repo test-file, one file at a time)
- `🗄️cas/🧪️tests/🟦️.ts`: +2 (critical partition with hanging collection refused within one budget, failed before the fix: waited 3 s; admission budget < client put deadline): 72 pass, 0 fail.
- `🪪️graph-state/.../🧯️refusal/🧪️tests/🟦️.ts`: 5 pass (long-body assertion made to exceed 640 chars).
- `owner-pool/🧪️tests/🌊️early-wave/🟦️.ts`: +1 (507 detail in both languages with the reserve numbers): 8 pass.
- `🌐️remote-execution/🚀️offload/🧪️tests/⏱️timeouts/🟦️.ts`: 16 pass.
- `repo typecheck` on the changed paths: no new errors in the guard, the owner-pool module or my added tests; the reported errors sit in untouched older lines of those test files and other tests. `dangling-imports.py`: no output.
