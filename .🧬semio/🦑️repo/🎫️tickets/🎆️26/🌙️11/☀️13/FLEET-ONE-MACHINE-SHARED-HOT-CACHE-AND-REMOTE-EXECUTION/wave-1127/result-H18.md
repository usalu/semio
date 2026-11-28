# Result H18: Nx task hashes (backlog R3, R5, R10)

## Root cause of the 0.2 % Nx hit rate
`nx.json` `sharedGlobals` is part of `default`, hence of every cached task. It runs `repo tools exec --light node -- -p ...` and `... bun -- --version` as Nx `runtime` inputs. Nx hashes **stdout and stderr** of a runtime input (proved with the native Nx hasher in a scratch workspace: same stdout, different stderr gives a different task hash; the command text is irrelevant). The front door printed `[repo] still working after N s; last phase started: P` (en, de, or both when no language is set) after 5 s. Measured here on the same command: 4 s run: empty stderr; 9 s run (de): `arbeitet seit 6 s`; 11 s run (en): `still working after 5 s`. So the task hash depended on machine load, seconds waited and language, i.e. every run and every machine produced its own hash.
Server evidence: 65 numeric Nx keys since 11-26 13:54, none put twice, although machines put within seconds of each other (17:01:12 m06, 17:01:15 m13, 17:01:23 m11; 15:06:26 m06 and m01): same task, same replicated tree, different hash. Lifetime counters hit 4 / miss 399 / put 136 / rejected 44; since restart 23:34 hit 1 / miss 24 / put 5.

## Fix (landed, `T/landings.md` "Wave 1127 H18")
`🏗️build/🚪️door`: policy `quiet` (`tools exec --light`) + schema, `buildDoorStaysQuietV1`, `🏃️main` skips the liveness line for it, README en/de. After: stderr empty at 8, 8 and 10 s in de, en, unset.
Tests `🚪️door/🧪️tests/🤫️quiet` 4/0: truth table; law that every `repo ...` runtime command in nx.json is quiet; native Nx hasher cross-check (heartbeat variants differ, silent equal); real command hashed under de/en/unset gives one hash and empty stderr. Door tests 9/0, `repo typecheck` passed, dangling-imports 0.

## R5 (namespace `semio`)
Disk: `semio/ac` last write 11-26 10:04, `semio/nx` 11-25 14:49; no `semio` writes since; all provenance rows of the last 400k are `main` for every origin incl. m06, m07, g1-server. Closed by F16 (topology instance `main`, client precedence); the server 421 guard stays unproven.

## R10
`remote-cache/🛰️fleet/🟦️.ts` already resolves cache and execution endpoints with `resolveFleetEndpointV1` (last-good file, subnet order); no probe-and-failover list left there (code read, not live).

## Before / after hit rate
Before: Nx 4 hits / 403 lookups (1.0 % lifetime, 0.2 % in F11's window). After: NOT measured on the fleet (workers pick the change up on restart; read `repo remote-cache status` nx counters in an hour; expect a hit whenever two machines hash an identical tree).

## NOT verified
- Measured remote hit of a small project: `repo run @semio-tech/repo-workspace-go:generate-source-policy:prototype --exclude-dependencies` took 15 min in the graph read here, then stopped with `Internal error: Actual declared requested Source scope required (7566ad79)` after a stale dependency receipt (`env:BUN_ENABLE_CRASH_REPORTING`); `repo inputs` earlier ended in `tool-closures-refused` (24, `@semio-tech/ui-rs:generate`). Both are graph/dependency state faults outside this lane.
- Other per-machine inputs: plain `node --version` / `bun --version` / `node -p` runtime inputs of engine and test targets resolve from the PATH of each machine (registry pin node 24.15.0 at `~/.local/bin/node`); not compared across workers. Replicated-tree churn still makes hashes time-specific (F16).
- Nx puts rejected 44 (content hash mismatch) are old; session counter 0.
