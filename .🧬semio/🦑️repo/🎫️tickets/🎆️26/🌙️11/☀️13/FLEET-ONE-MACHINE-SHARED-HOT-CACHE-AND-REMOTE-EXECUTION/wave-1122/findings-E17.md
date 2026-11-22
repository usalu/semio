# E17 findings
- The 420 s "nx admission" is the Nx tool derivative being rebuilt (about 210 s) plus source admission/index/storage planning (about 100 s), all rerun after code changes; not the fleet cache (bounded at 1.5 s per request with a shared down breaker). Lever: make the `nx-sdk-derived-namespace` digest independent of unrelated sources or fetch it from the fleet cache. Owner: Nx tool area (E13/E10).
- Graph service start is 158 s when peers land code during start ("2 implementations seen"): the client waits for the service of the current code; an interactive run could use the running older service for a `current`-policy plan when only unrelated files changed (E2/E8).
- `poolWithRoom` waits up to 900 s for room (`capacityWaitMilliseconds`); consider 60 s then refusal with a clear message at high disk usage.
- Closure-first discovery (item 5) untouched; E1 holds `graph`.
