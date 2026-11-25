# Warm gateway: serve usable, revalidate behind it

## Design
- `WarmMcpLaunch.stage(progress, signal, serving)` may call `serving()` once a previously published server can already run (`command()` valid). The gateway then starts the native child at once (handshake, queue replay, three `list_changed`).
- After a successful stage, optional `changed()` says whether the published binary or catalog differ (absent means changed). Gateway states: `stagingFinished(status, changed)`.
  - Cold (never usable): unchanged behavior, including `changed=false` still starting the server.
  - Usable + failed refresh: logs `[semio-os-mcp] refresh failed: ...`, keeps serving.
  - Usable + unchanged: nothing.
  - Usable + changed: candidate child handshakes with the client's original initialize params (modern clients: swapped at once); new client requests go to it only after the handshake; the old child keeps its pending request ids, its responses are relayed, its other frames are dropped with a logged line, a cancel goes to the owner of the request id, an early exit of the old child fails its held ids with -32603; when its pending set empties it is stopped (stdin end then the Shutdown SIGTERM/SIGKILL budget); then the three `list_changed` are emitted. A refresh finishing during the first handshake swaps right after it. A refused or exited candidate logs `refresh failed` and is stopped.
- Cancellation: client EOF/signals stop staging and every child (current, candidate, retiring), finishing when all closed, bounded by the Shutdown budget. Wording lives in the schema definition `Refresh`.
- `servePublishedMcpStdioV1(..., probe?)` with `createMcpPublicationProbeV1(env)`: usable = configured catalog files exist, then a short admission attempt of executable + components succeeds; the fingerprint is the executable stamp (size, mtime, inode) plus sha256 of both catalog files. The attempt admission is released right after `serving()` because the publication writer takes `artifact:<dir>` exclusively and a held reader would block the refresh; after the refresh the publication is admitted for the whole lifetime (existing hold semantics) and `changed` compares fingerprints. Caveat: the first child runs from the previous publication unleased during the refresh (safe on POSIX where the publisher replaces files; not verified on Windows). `🛠️dev` `runMcpOs` passes the probe.
- Existing publication source-compiling tests (`🏗️staging/🧪️tests/🧾️publication`, not edited) keep passing: probe is an injected parameter, no new identifiers in the function body.

## Files
- `🌉️mcp/📡️stdio/🚪️gateway/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🔣️.json`, `🧪️tests/🟦️.ts`
- `🌉️mcp/🟦️.ts`, `🌉️mcp/README.md`
- `🦑️repo/.../🎛️commands/🛠️dev/🟦️.ts` (probe wiring)
- new `🌉️mcp/🧪️tests/🔭️publication-revalidate/{🔣️.json,🟦️.ts}`

## Tests (repo test-file)
- gateway test file: 36 pass (journeys usable-unchanged, usable-changed with in-flight request on old child, refresh-failed, candidate-refuses-handshake, changed-before-first-handshake, cold-unchanged; SDK schema validation; SQLite pending-set check; Ajv on `Refresh`; real-process swap and unchanged/failed tests).
- publication staging test file: 20 pass. New revalidate test file: 6 pass.

## Review fixes
- Lease: the writer's exclusive `artifact:<dir>` lease rules out a held reader during the refresh, so the first child runs from a private executable copy (`probe.snapshot`, removed by `probe.discard`) and its component readers stay held until the refreshed publication is admitted (or to the end when the refresh fails).
- Gateway: server-initiated requests of replaced/new servers get gateway-unique ids (`semio-gateway-request-N`) with a routing map; retiring-server notifications are forwarded; cancel clears pending and releases a retiring server; stdin failure of a non-current child is a refresh failure; logging level, subscriptions and roots change are replayed to the candidate; candidate handshake timeout from schema `Handshake` (60000 ms); a current-server exit runs the full stop path (stops candidate and retiring servers); closed children skip reaper timers.
