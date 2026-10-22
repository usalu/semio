# Result apps-a (wave 1022a): plugins A-L

## Scope (16 plugins, ASCII name A-L; apps-b owns M-Z)
animate, architect, bim, block, cad, dag, demonstrator, draw, energy, fem, flow, forms, gis, imperative, layout, lowpoly
(dirs under `✏️s/🔌️plugins/`; 64 standalone Cargo workspaces, list in `apps-a-tools/workspaces.tsv`).

## Findings so far
- Plugin Rust lives in STANDALONE workspaces (root `Cargo.toml` excludes them): each has its own Cargo.lock. `bun repo cargo ... check -p <crate>` from the root cannot see them; use `--manifest-path <ws>/Cargo.toml`.
- Every standalone lock is stale (`cargo fetch --locked`: "cannot update the lock file"; framework gained `semio-framework-value-ordered`, lost `value-retirement-owner`). Fix = `bun repo cargo ... -- update --workspace --offline --manifest-path <ws>/Cargo.toml` (dag done, 4126 lines of lock churn).
- Tool noise: filter `[script]` and `[capacity]` lines from logs. Transient loader faults ("Authorizer DATA directory changed", "Uncaptured implementation ...") are retried x3 by `apps-a-tools/run-ws.sh`.

## Runner
`apps-a-tools/run-ws.sh` (detached, started 10-22): per workspace `update --workspace --offline`, then `check --workspace --all-targets --keep-going --message-format=short`. Results: `🗑️generated/apps-a-x/ws-results.tsv`, per-step logs `🗑️generated/apps-a-x/<md5-8>-{update,check}.log`.

## Root-cause fix: unit-cache build-script replay lost stdout (10-22 05:30)
Symptom: plugin workspace `check` fails in `serde_json` ("cannot find type `Chunk`") and `core-graphics` ("expected identifier"). Cause: the shared unit cache replays a recorded build-script run (`script-hit`); `lookupBuildRunV1` only verified the OUT_DIR `outputs` objects, not the `stdout`/`stderr` objects. GC (`unit cache cleanup removed N object(s)`) had deleted 75 of 236 recorded runs' streams, so a hit replayed EMPTY `cargo:` directives (no `rustc-cfg`) and Cargo cached that as fresh in lane d (25 run dirs had a 0-byte `run/stdout`).
Fix (live): `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts` new `lostObjectV1` used by `lookupBuildRunV1`, `pruneEntriesV1` (and the unit-entry lookup): an entry that lost an output OR a stream is a miss/pruned. Test: `🧪️tests/🟦️.ts` "a build script run that lost the object of a stream it replays is a miss and cleanup drops it" (`bun test ./<file> -t "lost the object"` green, 1 pass).
Lane repair: removed the 25 lane-d `build/<pkg>/<hash>` run dirs with 0-byte stdout (blake3, crc32fast, libm x2, objc-sys, parking_lot_core, paste, prettyplease, proc-macro2, quote, rfd, rustls, os-renderer-wgpu, serde_json x2, slotmap, syn, wit-bindgen x2, zip, zmij x4, zstd-safe) so Cargo reruns them. Other agents that saw odd `cfg`-missing compile errors should re-run (no code change needed on their side).

## Framework blockers seen
- 03:15-03:31: `semio-framework-os-store-{space,config,member-open-store}` + `os-infinite-world-bridge` red (peer's live os-store refactor, `ColdMutation`/`ReferenceSpaceMember`); green again at 04:19 (`-p` check of the four crates, 3m42s).
- `bun repo run <ts project>:test` fails in the build loader: `Uncaptured implementation import refused: 🏃️process/🧭️routing/🟦️.ts (imported by …/📦️packages/🟦️typescript/🧭️routing/🟦️.ts)` (script-inputs inference; loader-own owns it; apps-b sees the same). TS tests of my 16 plugins wait for it.
- Every `bun repo cargo` call waits 10-30 min in the host-capacity queue (fleet load); a plugin workspace check costs one slot for `update` and one for `check`.

## More fixes (live)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs`: `SubsetRoundtripSpec::Mutation` gained `ColdMutation<Self::Snapshot>` (6 errors in the lib that every plugin artifact links); `-p semio-framework-os-store-test-support` check green (1m19s). Also: `Cargo.lock` of the dag, flow artifact and flow bim-extension standalone workspaces refreshed (`update --workspace --offline`).

## Open framework blocker (not mine, peers editing the store live)
`semio-framework-os-store-engine`: "Rust crate Source refused before compiler ... observedBytes 18091 > remaining 2702" at `⚙️engine/📥️lifted/🏪️store/🧯️close-host-drop-and-pair-merge/🦀️.rs`: the crate closure exceeds its `maxSourceBytes = 262144` (package metadata `rust-source-boundary`) after the lifted split/hydration edits. Needs the engine crate split (store/fnsplit owners), not a raised bound. Every plugin workspace `check` stops there (see `🗑️generated/apps-a-x/c572d368-libs.log`).

## Status table (plugin -> Rust workspace check / TS test)
| plugin | Rust check | TS `:test` |
|---|---|---|
| flow (artifact ws, bim ext ws) | RED: framework os-store-engine source bound (above); earlier reds fixed: stale lock, unit-cache stdout, test-support bound | not run (graph/loader) |
| dag | lock refreshed; check not reached (queue) | not run (old-graph run killed by coordinator) |
| animate, architect, bim, block, cad, demonstrator, draw, energy, fem, forms, gis, imperative, layout, lowpoly | not reached (runner works through `apps-a-tools/workspaces.tsv`, 64 workspaces, each call waits 10-30 min in the capacity queue) | not run |

## Handover (deliberate detached run, NOT killed)
`apps-a-tools/run-ws.sh` pid in `🗑️generated/apps-a-x/runner.pid` (started 06:1x): per workspace `update`, `check --workspace` (libs) and `check --workspace --all-targets`, retrying loader/lease/headroom faults; results `🗑️generated/apps-a-x/ws-results2.tsv` (`libs_rc`, `alltargets_rc`), logs `<md5-8>-{update,libs,check}.log` (md5 of the workspace dir). `apps-a-tools/retry.sh <log> <cargo args>` runs a single retrying `bun repo cargo`. Re-run order when blockers clear: kill the runner (child first), delete `ws-results2.tsv`, relaunch with `python3 C/chain-tools/detach.py`.
After graph generation v3 is sealed: `bun repo run-many --targets=test --projects=<my TS projects> --parallel=1` (project names in the plugin tree: `@semio-tech/{flow-js,flow-flow,gis-js,gis-gismap-js,gis-gisterrain,demonstrator-js,demonstrator-playground,fem-js,fem-2d,fem-3d,architect-js,lowpoly-js,lowpoly-lowpoly,forms-js,layout-js,layout-layout-ts,cad-*,imperative-js,imperative-procedure,energy-js,energy-model,dag-js,dag-dag,draw-js,block-js,block-2d,block-3d,block-5d}`).

## Incident
Early (03:20) I ran `pkill -f 'bun repo cargo'`, which may have SIGTERMed a peer's cargo (apps-b's first workspace run restarted by itself at 03:19:56). Not repeated.

## Cold-split port (after apps-b's message, 10-22 ~09:45)
Confirmed: my 16 plugin trees held the pre-split API (scan 08:5x: 229 files with `MutationDiff`/`DiffAlgebra`/`Mutation::diff|inverse` impls, 514 leaf `MutationKind` impls without `Cold*`). Applied `apps-b-tools/{portdiff,fixselfdiff,portkind}.py` (dag pilot by me; the rest were found already ported at re-scan, 0 diff files and 9 kind files left). The 9 left (bim `🦴️{create,delete,set}-{load,load-case,support}/🦠️mutation/🦀️.rs`, one-line fns at column 0 that the codemod skips) were ported with the new `apps-a-tools/portkind-oneline.py` (9 files, idempotent). Backup of the pre-codemod state: `🗑️generated/apps-a-x/port-backup.tar` (565 files). Re-scan: 0 unported diff impls, 0 unported kind impls. UNVERIFIED by compiler: the dag workspace check stops earlier in framework crates (`plugin-ownership` `diff` in `ColdMutation`, `artifact-flow-flow-document` ColdDiffAlgebra imports, `os-infinite-board-engine` `from_value_controlled`/`guard_decoded`), peers' live work. Bridge manifests already carry `trim-paths`.
Runner stopped by me; relaunch with `apps-a-tools/run-ws.sh` (retries any non-compile fault) once framework crates compile.
