# Directory Command Receipt Transport P0

## Decision

The next non-overlapping administration prerequisite is a **bounded, request-idempotent directory-command receipt transport**. It belongs below the forthcoming author administration pane and above the active event-page/Home owner: it changes neither the event-page action nor the space-member UI model.

The present command lane is not a usable authority/receipt channel. It can execute an accepted `create-invite`, then permanently discard the only invite capability; it retries a command whose server may already have committed; and the native shell does not retain either a request identity or a successful `CommandOutcome`.

## Current, verified path

| Boundary | Current source fact | Consequence |
|---|---|---|
| Hub ingress | [`post_directory_commands`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3891) accepts a bare `DirectoryCommand`, authorizes it, then calls [`execute_directory_command_fenced`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3865). There is no request id, dedup record, request size layer, or response-size check. | A lost `202` cannot be distinguished from an unexecuted request. Replaying `CreateInvite` mints another invitation. |
| Hub result | [`DirectoryCommandResponse`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3841) is `{ events, result?: DslValue }`; [`command_result_value`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3856) returns the plaintext `inviteToken`. `CreateInvite` makes the token with fresh OS entropy at [`prepare_invite`](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:868), while storage retains only its digest. | The token is deliberately absent from the log and database. A retry cannot safely reproduce it from current state. |
| Browser client | [`DirectoryClient.command`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:4316) calls generic [`postJson`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:4275), which parses an unbounded JSON body into `unknown`. | No closed result grammar or command-result byte limit exists. |
| Browser worker | [`submitDirectoryCommand`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1457) and [`flushDirectoryQueue`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1481) post only `events`, dropping `result`. Their local `requestId` is never sent to the hub. | The normal invite-link success path loses the token. Retrying can duplicate a completed command. |
| React host | [`FrameworkOsShell` message handler](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1474) folds events and only logs an error. [`applyHostEffects`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3173) creates a random request id but has no receipt owner. | There is no safe recipient for a token, progress, authoritative completion, or an indeterminate outcome. |
| Native client | [`CommandOutcome`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:194) already represents `result: Option<DslValue>`, but [`request_json`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:655) has no general response bound and carries a whole error body. | A forged/oversized response and raw server error text can cross the transport seam. |
| WGPU shell | [`dispatch_directory_command`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4096) ignores successful outcomes. Its queue is `Vec<DirectoryCommand>` ([`ShellState`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1802)), and [`flush_pending_directory_commands`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4123) requeues every error after moving the whole vector out. | It loses result receipts, retries 401/403 as well as transport faults, and can execute later commands after an earlier failed command. |

The existing event-page contract is correctly bounded/canonical ([`DirectoryEventPageV1`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json:139), 128 items/64 KiB) but is not a command receipt and must remain separate.

## P0 contract and ownership

Add a sibling schema-first contract in `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs}`:

```
DirectoryCommandRequestV1 {
  schema: "semio.directory.command-request.v1";
  requestId: lowercase 32-hex nonzero;
  command: DirectoryCommand;
}

DirectoryCommandReceiptV1 {
  schema: "semio.directory.command-receipt.v1";
  requestId: same id;
  commandSha256: lowercase 64-hex;
  outcome: "accepted" | "previously-accepted" | "secret-undeliverable";
  events: DirectoryEvent[];                 // max 4, canonical sequence order
  result: { kind: "none" } | { kind: "invite"; inviteToken: capability };
  receiptSha256: lowercase 64-hex;
}
```

`requestId` is an idempotency correlation, never a capability. The hub derives `commandSha256` from the canonical request command and keys durable completion by `(authenticated user id, requestId)`; equal keys with unequal digests return a generic `409 request-conflict`, never execute again. It performs authentication and the existing [`authorize_directory_command`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3809) **before** returning any stored completion, including a redacted result. Thus an expired/revoked session cannot retrieve an old result just by knowing an id.

Use 8 KiB request and 64 KiB receipt ceilings. The former matches the existing public-admin request ceiling (`ADMIN_INTENT_REQUEST_MAX_BYTES` in [`directory.rs`](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:362)); the latter matches `ADMIN_RESPONSE_MAX_BYTES` and the existing event-page client ceiling. Set the `DefaultBodyLimit` directly on the command route and bound encoded receipt bytes before returning them. Both clients must count raw UTF-8 bytes *before* JSON decoding. A non-2xx error is a closed code only (`unauthorized`, `forbidden`, `stale-session`, `request-conflict`, `invalid`, `overloaded`); neither client should preserve/log a raw response body.

### Secret-result rule

Current `InviteRecord` intentionally contains `secret_digest`, not plaintext ([`InviteRecord`](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:226)). There is no server-side confidential receipt vault or deterministic persisted capability secret. Therefore P0 must **not claim replayable invite-token delivery**:

1. The first successful execution returns the typed invite result to the currently live operation only; it is never appended, broadcast, folded into Home, persisted by browser storage, or put in a WGPU debug log.
2. The durable idempotency record stores only command digest, event range/result *kind*, and `secret-undeliverable` after an ambiguous response loss; it stores no token.
3. A retry of that request returns `secret-undeliverable`, which proves no duplicate invite was minted but cannot recover the link. The later author-admin page can show the issued invite metadata and offer revoke/new invite. A future confidential receipt-vault design may improve this, but must not silently add plaintext capability persistence.

This is the only honest cancellation rule: cancelling an HTTP wait does not cancel a server command already past its linearization point. The retained client operation keeps `requestId` and becomes **indeterminate**; it may explicitly query/replay the id, but must never silently issue a fresh id.

## Minimal implementation slices

1. **Hub schema and durable idempotency owner.** Replace the bare route input/output in [`bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3841)–[`3905`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3905) with the closed request/receipt. Add a `DirectoryCommandReceiptStore` operation to the existing backend-neutral `HubDirectory` boundary rather than a route-local map. Its atomic `claim_or_read` must cover the existing `DirectoryService` writer/append sequence and publish only after the receipt/event record is durable. The row must carry actor user id, request id, command digest, result kind, event seq range, terminal disposition, and bounded canonical receipt digest—not a capability plaintext.

2. **Browser transport owner.** Replace `QueuedDirectoryCommand` in [`backbone-worker.ts`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1368) with a fixed-capacity (64) `DirectoryCommandTransportOperationV1` carrying the sealed request, an `AbortController`, session/worker epoch, and terminal receipt/error. Add a `directory-command-cancel` request. Capacity rejection and worker close must post a terminal closed error; neither may silently drop the oldest intent. Only transient transport faults retain the same sealed request. 401/403/409/413 and validated server rejection are terminal and never retried. An epoch mismatch suppresses delivery after identity/worker replacement; its request may later be explicitly resolved, not auto-replayed.

3. **Client parsers.** `DirectoryClient.command` in TypeScript and Rust accept the V1 request and parse only a raw-byte-capped `DirectoryCommandReceiptV1`; eliminate `unknown`/open `DslValue` at this public command boundary. Keep `DslValue` internal only while the hub adapts existing `CommandResult`. The Rust `DirectoryClient::command` should use a bounded `request_json_limited` variant and never place raw response text in a UI-facing `DirectoryClientError`.

4. **React/WGPU handoff, not a pane.** Extend `BackboneWorkerResponse` at [`🟦️.ts`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:744) with the typed receipt/closed error. React hands it to a retained, request-id-keyed shell result slot and only then folds accepted events; token text reaches the future administration action’s explicit copy handler, never console/state telemetry. Replace WGPU’s `Vec<DirectoryCommand>` with the same sealed request/receipt operation and route `CommandOutcome` to a matching transient result slot. This transport packet supplies no management widgets.

5. **Cancellation/freshness.** Browser passes operation `AbortSignal` to [`DirectoryClient.command`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:4316). Native creates one child of `directory_cancel` per request, sets a finite deadline (the current [`directory_ctx`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4079) explicitly has none), and preserves FIFO: stop at the first transient failure; terminal auth/conflict failures produce a result and then proceed. Do not change scoped socket revocation; a `401` from this route ends only this command operation.

## Non-vacuous acceptance matrix

| Layer | Required law |
|---|---|
| Neutral + third party | Add request/receipt vectors beside the directory schema: normal `CreateInvite`, prior accepted/redacted retry, equal id/different command, malformed/camel/snake/unknown fields, token over-limit, `64 KiB + 1`, duplicate/out-of-order events, error codes, cancellation/epoch traces. Register Bun + AJV validation and a separate Node canonical SHA-256 receipt oracle; use the Rust parser as another implementation, not the oracle. |
| Hub native | A SQLite route law drives two concurrent identical request IDs and proves one invite row, one receipt disposition, no plaintext token in invite/receipt storage, exactly one response contains the first token, and subsequent same-id result is redacted. Cover cross-user same request id, spectator/admin/removed-author denial, digest substitution, 8 KiB+1 request and 64 KiB+1 response, failure between durable command and reply, restart then same-id resolution, and no event broadcast of the token. Extend the existing command route laws in [`bin.rs`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8346) rather than mocks. |
| Browser native/unit | Extend [`backbone-worker.ts` directory lane tests](../../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2340): receipt reaches the host exactly once; retry uses byte-identical request; cancel/unmount yields indeterminate without late result; terminal 401/403/409/413 never queue; capacity returns a terminal result; queue FIFO stops at a transient head; a token never enters the event fold or logger. Extend ShellHost test coverage from its current action-map-only test at [`index.test.ts`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts:5249). |
| WGPU native | Extend the directory client cancellation tests at [`client.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:2298) and WGPU action-map test at [`wgpu.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:771): cap/reject bodies, preserve receipt once, stop FIFO at transient failure, no retry on forbidden, deadline/cancel result, and terminal operation empty. |
| Process | Add a `📜️script.ts` launch/Nx gate using two real authenticated sessions: author issues invite through the V1 endpoint, exactly one 202 carries the token, simulated client disconnect then same-id resolution is redacted/no duplicate, spectator receives a generic forbidden response, and a revoked author cannot retrieve a previous receipt. This is hub process evidence only; it does not claim a rendered administration panel. |

## Explicit nonclaims

This packet does not implement the author member/invite page, Home event-page retained action, invite redemption, scoped 4401 socket enforcement, persistent offline command queues, or a confidential capability-result vault. It does make the existing browser/native command path safe to consume a real authoritative receipt when those owners arrive.
