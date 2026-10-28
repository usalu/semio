# nxlock-g: Nx lock order (result, 2026-10-26 18:58)

## Result
Batch `nxlock` is STAGED (not applied live): 12 files, layered on nxfix-g batch `core` (2 files, the order `policy.json` and order tests, have core bytes as their base; apply `core` first, `stage.py check` reports them `needs-core` until then).

Tools: `nxlock-g-tools/` (`stage.py build|apply|revert|check`, `files-nxlock.txt`, `staged/nxlock/{manifest.json,files/,verify.sh}`, `model.py` independent Python rank model, `mkfixture.py`, `violations.json`, `policy-edit.py`).

## What the batch changes
1. Nx-path acquisition sequences (client run, client start, daemon service, journal reader, cache put) replayed by an independent Python model (`model.py`, shares no TypeScript) against the authored rank table. Found: the start lease `nx:daemon-startup:` (rank 2 tool-singleton) was held while requesting native-storage, graph-pool and graph-namespace (hold-and-wait, out of order), and the daemon service bound the namespace before taking nx-tooling / nx-sdk-snapshot.
2. Policy: `nx:daemon-startup:` becomes class `daemon-startup` (rank 8, taken last, nothing awaited while held); `build-graph:` pins and `nx:daemon-slot:` become two INERT classes (only non-waiting try-locks request them, nobody waits behind them); `cargo-custody` gets the edge to `daemon-startup`.
3. Order library: `inert` class flag (contract + schema), skipped as held lease and as request in `checkLeaseOrderV1` and in the hold-and-wait decision.
4. Startup: `withNxDaemonStartupV1` restructured into explicit steps (receive, admit, store, birth, lease) so images, toolkit use and namespace are open BEFORE the exclusive start lease is taken (lease-first order is now only a negative fixture); service binds after tooling/sdk leases.

## Evidence (single-file runs, graph-free, 18:57, load 24)
Run on a sparse short-root clone with `core` then `nxlock` applied (the real tree was not touched; clone removed):
- order tests 73 pass / 0 fail; component-driver 21 / 0
- graph startup tests 18 / 0; startup phase test 1 / 0
- nx put-slots tests 14 / 0
Re-run after landing with `sh nxlock-g-tools/staged/nxlock/verify.sh` (live root).

## For gate-g
Apply order: `core` (nxfix-g) then `nxlock`. Both change lease policy bytes, which re-keys lease-related tasks; run the product build after. Revert: `python3 -I nxlock-g-tools/stage.py revert`.
Caveat: the repo-wide official Nx proof of the daemon start path (hold-and-wait gone under peer load) is not run; only the model and single-file laws.
