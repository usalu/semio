# result-rev-g (reviewer rev-g, wave 1026g)

Findings, ranked, with file:line, failing scenario and fix: `C/review-1026g.md`.

## Status
- 09:55 DONE. Read the landed code of ram-g, lease-g, bzsrv-g, chain-g, disk-g, infer-f and exec-f myself; graph-free, single-file `nice -n 10 bun test` only, no sub-agents, no ticket tools, no git write, no process left running.
- HIGH (1): ram-g reservation counts only 60 s (ramp 300 -> 60 s), the 16 GB law proves admission counts, never memory; real store shows `run` peaks of 45-167 % of default over 36-1418 s. Fix proposal: per-class time-to-peak ramp, ramp-slot gate, cold-graph class, memory-aware law.
- MED (7): failed/cancelled runs are learned; lease "stuck" ignores the watchdog's tree activity (exit 1 on yield in default mode); terminate mode (legacy age includes own queue wait, SIGKILL orphans children); stale yield trusts the claim; reaper reads ledger headroom (spurious reaping of warm servers); quota eviction has no liveness signal for tool caches; `store-custody` prefix covers fact derivation/encoding in `🕸️graph/🕰️generations/` (a scheduling edit there re-keys nothing, generation digest is stamp-only).
- OK: chain-g loader change (FOR chain-g: hash `loaderIdentity()` once; law for double evaluation).

## Fixed by me (finished lanes, law failed first)
- F1 disk-g `🧹️cache-gc/👯️dedupe/🟦️.ts`: canonical file re-checked against its census stamp (ino, mtime, size) before and after the clone; law added in `👯️dedupe/🧪️tests/🟦️.ts` (8 pass).
- F2 lease-g `📈️standing/🤝️yield/🟦️.ts`: a head holder without a registry entry is not a terminate candidate; law added in `📈️standing/🧪️tests/🟦️.ts` (19 pass).

## Not run
Official `bun repo` routes, typecheck (`tsc` not available graph-free), the live lease test (needs the envelope runner), Windows/Linux.
