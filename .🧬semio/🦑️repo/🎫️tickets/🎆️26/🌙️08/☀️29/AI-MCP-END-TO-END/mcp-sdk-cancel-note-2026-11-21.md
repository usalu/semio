# SDK Cancellation During Typed Note Creation

The installed MCP TypeScript SDK 1.32.1 cancelled a real typed Note `artifact_create` after the gateway began paid guest Input. The native gateway returned the terminal job status `CANCELLED`, and a subsequent snapshot of the same artifact returned `NOT_FOUND`. The same SDK session then completed `ping`.

| Time from create | Observed event |
| ---: | --- |
| 0 ms | The SDK sent `tools/call` for `artifact_create`, kind `s.note.note`, request ID 2, and `_meta.progressToken` 2. |
| 8,781 ms | The registered SDK progress handler received `creating: opening-guest`. |
| 10,229 ms | The gateway emitted its Core Input witness for 681 bytes with FNV-1a `17db1cd0d9ba11ad`. With both observations present, the harness triggered the SDK's AbortSignal. The transport recorded its `notifications/cancelled` notification for request ID 2. |
| 10,234 ms | The gateway sent the terminal response on ID 2. Its tool result carried `status: "CANCELLED"`, `pluginId: "note"`, and job ID `job_ef568d902beffd3618ca336c8a`. The successful RPC envelope used `isError: false`, as authored for cancelled jobs. |
| After the terminal response | The SDK's `artifact_snapshot` returned `isError: true` and `code: "NOT_FOUND"` for the same artifact. A further SDK `ping` succeeded. |

Artifact ID `sdk-cancel-27e40235-194b-4c7f-868a-57e79fdaab1d` was allocated in a fresh ticket-local workspace. The host was the 20:12 CET `🗑️generated/native-host/debug/semio-os-mcp` binary with SHA-256 `a1766c3f0e87cfee365c24c8461d1ad8aeec4983bb2a217806e2b22e576e004b`. It used the canonical prepared Note model under `🗑️generated/m-canon`; the verified Note descriptor digest was `d82dd839a4594c7fe9a85dbf440d89161dcc777cc458d362f2c5e03b08990a42`.

The harness observes the official SDK's outgoing JSON-RPC frames and its public transport callback. It uses the assigned IDs from those frames. The SDK rejects the local request when its signal aborts, so the harness independently requires the actual terminal response from the gateway before issuing the snapshot. The first attempt reached the same terminal cancellation but used the wrong assertion for the success envelope. The verified rerun asserted the authored `status`, artifact ID, kind and plugin before checking the follow-up requests. It exited 0 and removed its scratch workspace.

Run `mcp-sdk-cancel-note.ts` with `repo tools exec bun` from the repository root. This result covers one actively cancelled typed Note create. The separate typed success and WFC evidence remains with the full MCP journey.
