# Fleet cache reachability and `--language`

## Findings (Problem 1)

- The fleet is healthy from this machine (g1-m12, 192.168.178.91): cache server `http://192.168.178.70:6094` (facts generation 10, published 02:20), NFS store mounted, ping ~49 ms, signed requests answer in 40-120 ms.
- Probes (`fleet-cache-probe.mjs`, `fleet-cache-probe2.mjs`): 40 sequential signed requests with the 1500 ms client timeout, 0 failures; 60 pointer reads through the real graph fleet client (sequential and 30 concurrent), 60 hits, max 332 ms, client `usable()` true. Unsigned requests get 401 by design; the execution port 8980 accepts connections.
- The text "the fleet cache cannot be reached" is not a protocol probe. `fleetReadV1` (`🪪️graph-state/🙋️wanted/📖️read`) takes ONE decision per read from `client.usable()`, which is only the on-disk circuit breaker `.cache/f/graph-state/down` (15 s window, stamped by ANY process of this machine on any timeout/network error; lookup/claim/head timeout is 1500 ms). On a machine at load ~7.5 with a cache server being restarted (facts republished 02:20), one transient fault makes the read declare the fleet unreachable and infer all projects locally for the whole run.
- Evidence in `graph-try2.txt`: `owner pool refusal ... stage pointer-read, code unavailable, status null, transient` and `breaker-open`, followed later by `owners published to the fleet`, and the run now reports `fetched from the fleet` and `wanted from the fleet`. No stale-fleet code or contract mismatch was found; no service had to be started.

## Fix

`fleetReadV1` waits out an open breaker (at most 20 s, `BREAKER_PATIENCE_MILLISECONDS`, injectable `sleep`) before it decides the fleet is unreachable. Regression test: `🪪️graph-state/🙋️wanted/🧪️tests/📖️read/🟦️.ts` (breaker of 9 s is waited, a breaker of 60 s is not); the dead-fleet test now injects its clock. 24/24 pass.

## Restart answer

The running graph run already pulls from the fleet where owners exist (a few fetched, the rest published as wanted to the fleet); it does not need a restart for reachability. Pulling is limited by what other machines have published (no machine held a complete generation).

## Problem 2

`--language` is not a flag of `repo`; the global flag is `--lang <en|de>` / `--lang=<en|de>` (the MCP surface maps `language` to `--lang`; `repo fleet status --lang=en` already works). `fleetArgumentsV1` now also consumes `--lang` itself (it is passed to the language resolution of the same arguments), `--language` is still refused. Test in `🚢️fleet-status/🧪️tests/🟦️.ts` (the one pre-existing unrelated failure there is the cache ledger reader: `/retirements/0/within is not allowed`).
