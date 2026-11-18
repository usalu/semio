# Graph load without a fleet builder (11-17)

Follows `graph-crawl.md`. Two decisions, both with tests, under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.

## 1. No fleet builder: infer here, never wait

- `🪪️graph-state/🙋️wanted/📖️read/🟦️.ts` `fleetReadV1` takes a new `builder()` port. When the fleet is usable but no builder is advertised, the read does not publish a request and does not wait: it plans with unlimited local waves (like the machine with serving headroom), so every stale owner is inferred here at the full admitted width; heavy owners keep the solo path. It says `noBuilder` (EN/DE, in the wanted schema texts).
- `💰️cost/🟦️.ts` `fleetBuilderAdvertisedV1`: a builder is advertised while the owner-cost table that serving machines keep in the fleet is fresh (within `ttlMilliseconds`, 20 min); absent, old, unreadable or unreachable means none. A stale table errs towards local inference, the safe side.
- `🕸️graph/🤝️prefetch/🟦️.ts` passes the probe. An unreachable endpoint (client unusable) keeps its existing immediate local path.
- Tests: `🪪️graph-state/🙋️wanted/🧪️tests/📖️read/🟦️.ts` (no builder: outcome local, 4700 owners, no wait, nothing published, both languages; probe fresh/absent/old).

## 2. No-progress watch counts CPU of running owners

- `📥️take/🟨️.mjs`: `takeChildrenCpuV1` (CPU of all descendants of this process; `ps` on macOS/Linux, CIM on Windows), `createCpuProgressV1` (counter grows when the children used at least `cpuProgressPercent` = 10 % of a core since the last tick), policy field `cpuProgressPercent` in the take schema.
- `📦️project-nodes/🟨️.mjs`: the watch sign now includes that counter while owners are active. A slow CPU-bound owner (the 974 s case) no longer aborts the load; owners without CPU progress (hung, waiting) still trigger `graph.no-progress` after 5 min.
- Tests: `📥️take/🧪️tests/🚦️no-progress/🟦️.ts` (counter, watch with busy vs hung owner, ps time parsing, tree sum, real busy child process).
- Known limit: an owner spinning on CPU forever is never stalled by this watch; the per-owner bound is the cost/memory governor.

## Verification

`repo tools exec bun -- test` over the 17 test files of wanted + take: 136 pass, 0 fail. Not verified live against the resident service (it reloads itself on code drift).
