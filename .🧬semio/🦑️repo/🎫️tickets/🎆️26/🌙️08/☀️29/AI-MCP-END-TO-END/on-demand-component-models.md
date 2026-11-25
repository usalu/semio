# On-demand component models and native server supervision

Module: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`

## Component models on demand

- Schema (`📡️stdio/📦️component-models/🔣️.json`): `MissingModel` (details key, cause `model-missing`, en + de summary) and `MissingModelDetails`.
- Rust (`🏠️workspace/🧭️plugin-paths-and-descriptor/🦀️.rs`): `missing_plugin_model_error` replaces the prose-only refusal. Retryable `PLUGIN_UNAVAILABLE` with `details {cause, missingModel{pluginId,cratePath}, summary{en,de}}`; it reaches `structuredContent` (tool error) unchanged.
- TS reader `📡️stdio/📦️component-models/🕳️missing/🟦️.ts` (`readMissingMcpModelV1`), tool error or JSON-RPC error `data`.
- Gateway (`📡️stdio/🚪️gateway/🟦️.ts`): relayed responses of the current server are inspected; per plugin and session once (failure removes the mark so the next refusal retries), builds serialized, `AbortSignal` aborted at client close, progress/logs from schema `Models`; changed model set triggers `swap(true)` (re-swap after a running candidate).
- Launcher (`🟦️.ts` `servePublishedMcpStdioV1`): `models` waits for the preparation, builds through `buildMcpComponentModelV1` (`buildComponentClosureV1`, facet `model`), re-admits the component environment with a new reader group (held to the end), updates the child env, reports whether the manifest changed. `changed()` also compares the model manifest.
- Lease custody: the new reader group is added next to the held ones (shared leases coexist); the writer only needs the exclusive lease of the missing plugin's own model path, which no reader holds.
- README sections "Component models on demand" and "Native server supervision".

## Supervision of the native child

- `serverExited(code, port, signal)` returns whether the gateway restarts: unexpected non-zero/signal exit while ready: held requests answered `-32603` with `data{cause:"server-exited", retryable, exit, summary{en,de}}`, replacement started through swap/handshake/replay (or the running candidate promoted), staging untouched, requests queue, list_changed announced. Budget `Supervision` (3 per 60 s, schema), also for failed replacement handshakes; beyond it `abandon` (stop 1). Clean exit 0 unchanged.
- The launcher skips `stop` when the gateway restarted.

## Tests (bun, bundled)

- `📡️stdio/📦️component-models/🧪️tests` (9): fixture shared with Rust, Ajv + SQLite oracles.
- `📡️stdio/🚪️gateway/🧪️tests` (54): model journeys (once, retry after failure, serial, abort on close, swap after candidate), crash journeys (pending request, crash loop over budget, crash during a candidate swap, client close during restart, clean exit). The live-process test of the old "exit ends session" was rewritten to the new semantics.
- `📡️stdio/🏗️staging/🧪️tests/🧾️publication` (24): launcher models/fingerprint cases from the shared fixture.

## Not verified

- Rust compile and the Rust unit test `missing_model_refusal_names_the_plugin_as_the_launcher_schema_declares` (cargo check via the ingress was waiting for memory; see summary).
- A real `buildComponentClosureV1` run from the launcher (capabilities of the MCP invocation, lease behavior when a dependency model is stale and held by readers) and a live hot swap.
- The wall-clock shutdown test `native peer shutdown is bounded...` is flaky under machine load (failed once at 2.8 s, passed on rerun).
