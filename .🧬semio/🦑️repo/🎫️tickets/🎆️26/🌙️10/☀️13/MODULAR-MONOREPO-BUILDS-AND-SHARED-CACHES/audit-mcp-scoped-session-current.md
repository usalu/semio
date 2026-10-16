# Independent Scoped MCP Session Source Audit

Read-only Source inspection on 2026-10-16. No runtime, test, compiler, Native, network, Hub, browser, cache, publication, or Git mutation was executed. This audit writes only this report. Existing ticket custody was supplied by the parent; repository MCP tools are unavailable in this agent tool inventory. Findings below are static control-flow findings, not reproduced runtime claims.

## Actionable Findings

### P1 — Delegation revocation does not enclose publication, credential creation, launch, or terminal drainage

The remote delegation is minted before `withDelegatedMcpClientSessionV1` in all three GIS consumers. Durability (`🤝️hub-edit-durability/🟦️.ts:121-144`) and coverage (`🧩️plugin-coverage/🟦️.ts:304-395`) place revocation inside the session body. That body cannot run when published-executable admission, Diagnostics reservation/scratch, credential creation, or child launch fails. Their revocation also runs before `withMcpClientSessionV1` drains accepted work, joins its writer/pump, and closes its child. Participant (`🤖️hub-agent-participant/🟦️.ts:159-256`) revokes only on the successful fall-through after both sessions; any first/second-session throw bypasses it entirely. Delegations live for 900 seconds in participant/durability and 3600 seconds in coverage.

Place a failure-preserving `try/finally` immediately after successful delegation minting around the entire awaited delegated owner. Revoke after that owner has returned/failed and joined terminal drainage; retain primary and revocation failures in order. Continue to use genuine carried cancellation authority: this change cannot promise remote cleanup succeeds after cancellation and must not mint replacement cleanup authority. Add Source failure-path coverage for owner refusal before body entry and for body/terminal failure. Ordinary peers can test the enclosure without claiming a real Hub revocation proof.

### P2 — A later callback throw replaces the recorded first session failure

The session owner (`session/🟦️.ts:22,34`) retains only the first `fail(scope,error)` in `scope.failures`, but its terminal selection uses `failed ? failure : scope.failures[0]`. If a real request fails first with A, a caller catches A and later throws B (including `undefined`), the body catch sets `failed=true`/`failure=B`; final selection discards A and throws B. A second form is an unobserved accepted operation failing A before the callback subsequently throws B. This contradicts the checkpoint's first-failure/ordered-terminal claim. Existing Error/undefined rows cover only immediate body throws, not competing failures.

Choose the already recorded first failure by explicit presence (array length, never truthiness); record the distinct body failure as a secondary failure where appropriate. Add actual ordinary-peer Source rows for first A/later B and first undefined/later Error, retaining exact identity and ordered aggregates. The canonical ledger might independently remain fatal, but that does not establish that this session's returned failure preserves its first failure.

### P2 — Pump rejects can be suppressed once another failure has begun close

`pump` (`session/🟦️.ts:32`) catches a read/parser/send rejection and, when `scope.closing` and `scope.failures.length` are both true, returns successfully. The outer terminal collector then cannot observe a distinct pump failure. It correctly independently awaits writer, close, and pump, but this inner suppression prevents the advertised distinct terminal-failure aggregation. Classify expected closure completion explicitly and retain an unexpected distinct pump error instead of broadly swallowing all errors during failed close. Add a Source peer/transport failure row where the body failure and pump terminal failure differ. This is a static suppression finding; this audit did not reproduce a platform-specific terminal error.

## Verified Source Structure And Limits

All six former constructor positions in the four consumer files use callback custody (two generic journeys, two participant sessions, one durability lane, one coverage lane); no raw public constructor remains in those sites. The private frozen session is authenticated by WeakMap and actual invocation/control checks. Input DATA is snapshotted before deferred effects, writes serialize on `writeTail`, and requests use the actual requested child `signal`. No fresh request clock was found. Normal and failed exit independently await the writer, close, and output pump and retire the token afterward; the failure-suppression findings above limit the failure-preservation claim.

Policy provides finite pending/total requests, pending operations, server request IDs, notification count/bytes, launch args/env/paths, credential bytes, and combined Core output limits. Core input additionally enforces cumulative frames/bytes. Numeric pending response correlation and direction-separated server IDs are present; unknown/duplicate response and repeated server ID refusal exist. Server roots receive an empty list, unsupported methods -32601. Client initialization capabilities/version and successful server version are pinned; this does not prove full MCP lifecycle negotiation or real server compatibility.

The three delegated call sites hold an actual published Cargo executable reader around the protocol callback. Credential creation is exclusive within genuine paid Diagnostics scratch, bounds bytes, checks single-link regular-file identity/size and POSIX mode, closes the file before launch, and the enclosing scratch owner outlives actual session return/drain. This inspection found no credential-path or environment-selected Hub-root adoption in this migrated slice. Durability encloses its lifecycle in the actual published Hub Host and materialized trusted catalog callbacks and restarts that owned Host. The selected user folder stays domain DATA. These are Source lifetime observations, not Native/publication/Host/runtime proof. Windows ACL behavior remains unqualified.

## Evidence Qualification And Moving Peers

Eleven of the twelve supplied Source hashes match this inspection. The ticket `typescript-artifact/📜️script.ts` moved from checkpoint `4f12b1a40137b009c0e8e9f2545f03ac209cc5c2eb7e10e4905967dcb8e3c6b7` to the hash in the table. This is identified as a moved shared wrapper, not attributed to this work or treated as a test result. No claim that the whole transitive source/provider epoch is frozen follows from the eleven matching leaves.

The supplied checkpoint records Source BOTH 4 tests/99 assertions and exact caller prototype strict exit 0. This audit did not rerun them. The final release strict check remains unqualified after the unchanged 120-second parallel deadline failure; there is no strict BOTH-green conclusion. Actual ordinary Bun peers are Source transport evidence only. Native/network/Hub/browser/compiler/cache/default publication remains HOLD.

## Exact Inspected Twelve-File Hashes

| Source | SHA-256 | Relation to supplied checkpoint |
|---|---|---|
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `676376e650e761e5c4526ae3762497b83c45d7adb11f844b0922579caa1bf6e1` | moved since checkpoint |
| `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️hub-agent-participant/🟦️.ts` | `ac96b78764affd225a7ca3aa33f6c5aa25c8e77c487a8d32291ce67e7212402c` | same |
| `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤝️hub-edit-durability/🟦️.ts` | `f3f3c0842103ab0ae0c84ffbf8ea8e2ca09a216e89543f38be5b88ca552b39f7` | same |
| `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts` | `66aee2b0a43667599a8884c30dae765391181ce7faf1f8bd1bab36155bbf567c` | same |
| `✏️s/🧑‍💻dev/📜️script.ts` | `ad4df336789c24ad7749b14a4969c70cea47603a5528cc4daf592f20306f2a93` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/📋️data/🟦️.ts` | `0189893928bf81cae910e7963860c39ab77fdd0ce7542ab785dfc4599f519d66` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/📦️published/🟦️.ts` | `6a2e2071768d71be57f650103e8da0095e04e10b2958602c02ddba83e9614207` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🟦️.ts` | `b22fb247640828a08408e01daee28359bf4041fd4c91dda5a007e1636f480e08` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🧪️tests/🔣️.json` | `9822297ce30363025b33c45b518679403ec10a0d196e7f56b71163b718e086d3` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🧪️tests/🟦️.ts` | `971801ad8f674b3a2f120bffb54a6125389e0d5d62f2a847dbda138f9825f099` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🎫️session/🧬️schema/🔣️.json` | `d747bd3792566f728816cf2481c3896f0f01186ceb71afdd3777f7b70590b57c` | same |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts` | `e76166255349e1ee3d2246a459b3085765e3f3208ecb285d213e5fd2baa6f63a` | same |
