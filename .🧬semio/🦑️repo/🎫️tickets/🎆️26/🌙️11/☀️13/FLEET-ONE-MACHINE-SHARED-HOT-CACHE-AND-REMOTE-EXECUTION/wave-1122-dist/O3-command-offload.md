# O3: Command offload (G6/G7)

## Design
- `offloadCommandV1` (`LIB/🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts`): same session (endpoint, credentials, decline memo, reachability), recorder (events, counters, progress), shipping (`shipOffloadActionV1`, shared with rustc), execution with ride-through, bounded upload, cancel and headroom/peak placement as the rustc offload. Rustc code was refactored onto the shared pieces (`openOffloadSessionV1`, `offloadRecorderV1`); the old rustc `offloadCommandV1` is now `offloadRustcCommandV1`.
- Action: program `@command:<tool>/<sha256 of tool file>`, args with `@root/<rel>`, inputs content-addressed, outputs `out/<name>` files written atomically to the caller's paths. Any fault, refusal, missing output or non-zero tool exit means "run here" (never throws).
- Allow-list (closed schema `🚀️offload/🧰️command/🧬️schema/🔣️.json`, EN+DE): `wasm-opt`, `node` (resolved: worker finds its own file and offers it by digest, property `semio-command-<tool>=<digest>` so the scheduler places only on an exact match; no shipping, zero-touch via the pinned Binaryen) and `native-transpile`, `build-script` (shipped as input at `tool/<name>`, digest verified by the worker after placement).
- Worker: `commands` table; validator `validateRemoteExecutionCommandActionV1` (only on confining sandbox without network; closed env names; only `@root` relative paths; outputs under `out/`); refusals use the worker-policy prefix so the client classes them `worker-policy`. Agents advertise `semio-offload-format=command-1` plus tool properties.
- Stage wiring: `StageCacheV1.run(..., slot, remote?)`; `sharedStageCacheV1` calls `remote` after winning the fleet claim, falls back to `produce` on false. `optimizeStageRemoteV1` (os plugin commands) wires the wasm-opt stage; honours `SEMIO_OFFLOAD=never`.

## Tests (`repo test-file`)
- command suite 11/0: schema, language-agnostic validator fixture (15 cases), platform, wasm-opt byte-identical to a local Binaryen run then served from action cache, peak placement via load report, non-allow-listed tool refused (client and worker), wrong digest refused, shipped tool digest check, non-zero exit fallback, unreadable/cancel, stage miss produced remotely and shared to a second machine, producer false -> local.
- Others unchanged-green: actions 10, worker 6, command 7, action 3, offload main 39, failure 2, determinism 6, handover 2, stage-fleet 13. Failing but not mine: offload cluster 9/3, wasm-cdylib 6/2 (peers' pin, door, cdylib work).

## Open
- Build-script RUN hook (G6) not wired in `UC/🏗️buildscript` (K2 lock); the `build-script` shipped tool and policy are ready. Needs a hermetic predicate (no network, manifest dir + OUT_DIR + declared env, `RUSTC` mapping) and root-path translation of stdout.
- transpile (native-transpile shipped) and probe (node + module closure) stage specs not wired; only optimize is.
- Incident: I ran `bun test` with bare paths and a repo-wide grep before the 21:55 addendum; the ENFILE may have partly been mine. Stopped; only `repo test-file` since.
