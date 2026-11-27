# Component model build: findings

## Root causes
1. The launcher IS wired: `readMissingMcpModelV1` -> `WarmMcpGateway.noteMissingModel` -> `servePublishedMcpStdioV1.models` -> `buildMcpComponentModelV1` -> republish `SEMIO_MCP_COMPONENT_MODELS` -> swap in a fresh native child. The shape matches the schema. The log line `building the component model of plugin stdio` appears on the first refusal.
2. The build then FAILED and the failure was only written to the server stderr (`component model failed for plugin ...`). Every later tool reply kept the native child's promise "the launcher is preparing it, retry in a moment", so the client waited forever. Each refusal started another failed build (one at a time).
3. Failures seen while reproducing, all in the build, none in the wiring:
   - `process.source.crate-refused ... Actual single-link Cargo Source file required` for crate `semio-framework-actor-vocabulary`: `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🦀️.rs` and `.../🩹️patch/🦀️.rs` had link count 2, because a stray hardlinked copy `lb/` lived in the old ticket `🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/`. Removed that tool-output directory `lb` (the originals are untouched). The source admission requires nlink == 1.
   - A transient `declareHostCapacityNeedV1` export missing in `🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts` (another agent mid-edit; fixed by itself).
   - `[warden] cancelled local-unit ... the host was critically short of memory`: rustc units killed while many agents compile. The build is retried by the next refusal.

## Fix (truthful reporting)
`WarmMcpGateway` remembers the reason of a failed model build per plugin. Until a build succeeds, every relayed missing-model refusal of that plugin is rewritten (tool error and JSON-RPC error form) to cause `model-build-failed` with `reason` and an en/de summary: "its last build failed (reason); the launcher is building it again". The next refusal still retries the build. Texts live in `📡️stdio/🚪️gateway/🧬️schema/🔣️.json` (`Models`).
Live check: after the failed build the probe received `Plugin stdio has no component model: its last build failed (...Cargo artifact build failed...)`.

## Not done
- Eager prebuild of the catalog at start: not implemented. It would start a cargo build per plugin at every server start on a host that is already memory-starved (see warden); the build-on-demand with truthful errors is the safer default. A follow-up could prebuild only the plugins of the kinds in the first calls.
- Full success of `artifact_create` for `s.stdio.txt` / `s.space.space` within one session could not be shown: the native publication plus the model build need minutes of cargo and were cancelled by the memory warden during the probe run. Rerun `python3 🐍️probe.py` with sleeps (`{"sleep":60}` lines are now supported; `PROBE_STDERR=<path>` selects the stderr file) on a quieter host.
- A build that finishes with "unchanged" while the child still reports the model missing is still answered with "preparing".

## Tests
`📡️stdio/🚪️gateway/🧪️tests/🟦️.ts` journey `failed-build-is-logged-reported-truthfully-and-retried-by-the-next-refusal` (corpus `🧪️tests/🔣️.json`) asserts the rewritten refusal; 63 tests pass via `repo test-file`; typecheck passes.
