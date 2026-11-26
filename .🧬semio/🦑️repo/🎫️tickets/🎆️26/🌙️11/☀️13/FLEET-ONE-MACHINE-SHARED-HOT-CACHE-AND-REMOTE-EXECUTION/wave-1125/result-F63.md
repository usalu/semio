# F63 result: type errors and F56 red tests

Type-check target: `repo run @semio-tech/repo-lib:typecheck` (inferred TypeScript provider; equals `tsc --noEmit -p` of `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json`, which also reaches `🏃️process`). The repo run waited on a whole graph read (>5 min), so the numbers come from the same `tsc --noEmit -p tsconfig.json` run directly (about 3 min, one process).

Errors before: 1462 (product 124, tests 1338). After: 1263 (product 0, tests 1263; list in `T/🧰️scripts/F63/tsc-remaining-errors-after.txt`).

Real runtime bugs found by the type check and fixed: `fleetQueueV1` used but not imported in remote-execution offload (ReferenceError on the beyond-host path); `releaseLease` missing from the remote-cache server's hostStore type; `WorkerRegistration.resumed` typed inside `capacity`; dead `report.code` spread in scheduler fleetLoad; `selected` null in dependency retirement (now an explicit refusal).
Systemic cause: hand-written `.d.mts` beside `.mjs` drift (about 180 undeclared exports, `T/🧰️scripts/F63/dmts-drift.py` lists them). Fixed those product code needs: slice-pool (new d.mts), owner-pool runner (new d.mts), slice limit, host composition, generation load, script-inputs, reload, HOST_POLICY_V1 (now derived from its JSON schema). The test errors are mostly the remaining drift plus fixture typing; not fixed.
Peer-owned, left: warden loop `cpu.known`, native-build `sourceWitness` (both gone in the last run).

F56 red tests: leases heartbeat was a real bug (a failed registry publish disabled the registry for good and no heartbeat existed); now retried by `leaseRegistryV1.heartbeat()`. Sandbox 2/21 was test-only (explicit parent under a long scratch TMPDIR exceeded the 90-char root limit); tests now use a short parent.
Tests: sandbox 21/21, heartbeat 1/1, leases order 79/79, registry-root 2/2, scheduler 29, remote-cache server 10, doctor 17, manifest-closure 5, metrics collector 21. dangling-imports exit 0.

Gate proposal (not implemented): run the library typecheck in `repo doctor`/readiness nightly and fail on any non-test error; add the d.mts drift script as a cheap pre-check (seconds, no compiler).
