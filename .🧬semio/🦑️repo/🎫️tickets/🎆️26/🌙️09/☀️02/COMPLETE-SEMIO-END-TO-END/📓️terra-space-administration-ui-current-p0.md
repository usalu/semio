# Current Space-Administration UI — Non-Optimistic P0 Packet

## Verdict

**RED: the hub can authorize and persist the administration commands, but no ordinary Home/Space UI can authoritatively administer a space.** Transactional invite redemption and scoped socket revocation improve the server boundary; they do not supply an administration surface.

The decisive current UI defect is role blindness: Home turns every hub-origin row into Rename/Share/Delete controls. `HomeSpaceRow` has no caller role and [`row_actions`](../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:71) adds those controls merely because `origin == "hub"`. A spectator can therefore reach a control that the server correctly rejects. No UI fetches the author-only invite list, and no UI consumes the issued invite receipt.

This is a source-only audit on 2026-09-05. No native, browser, or process test was run.

## What is current and usable

| Current element | Exact source | Bounded truth |
|---|---|---|
| Authoritative command authorization | [`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3778`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3778) | Creates are available to a signed-in user; member/invite/role mutations require `Author` (or server admin); deletion/archive require owner/admin. |
| Event-sourced member change | [`🌎️hub/📇️directory/🦀️.rs:1446`](../../../../../../🌎️hub/📇️directory/🦀️.rs:1446) | Upsert/remove emit durable directory events; an owner cannot be removed. |
| Atomic invite redemption | `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1232` and `DirectoryService::redeem_invite` at `🌎️hub/📇️directory/🦀️.rs:1760` | Redemption claim/membership/event are server-side and serialized. It does not create an administration UI receipt. |
| Scoped live revocation | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` scoped socket path and [`🧵️backbone-worker.ts:1403`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1403) | A document-scoped directory stream receives terminal 4401 handling. This is not the global Home administration detail stream. |
| Detail projection | [`🚀️bin.rs:3905`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3905) and [`directory schema TS:473`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:473) | `GET /directory/spaces/{id}` differentiates public/member/author; only author receives `invites`. |
| Command funnel | [`DirectoryClient.command`](../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:4316), [`backbone worker submit`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1457) | The shell can issue existing commands without a plugin networking API. It presently delivers only events and discards `result`. |
| Home projection owner, in progress | `HomeConfig::apply_directory_event_page` and `ApplyDirectoryEventPage` | Sealed event pages can replace the local read projection. Current ShellHost still opens the raw stream from zero at [`ShellHost:1682`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1682), so its active retained page-owner integration is a separate packet. |

## Exact gaps

1. **No capability-shaped Home row.** `home_space_rows` at [`✏️s/…/🪐️space/🦀️.rs:521`](../../../../../../✏️s/🔌️plugins/🪐️space/🦀️.rs:521) copies only id/name/kind/visibility/count/time/origin. It loses current membership role. The Home renderer consequently cannot hide author-only actions.
2. **No administration detail state.** Home’s folded `DirectoryReadModel` deliberately contains members but no pending invites. `CreateInvite`/`RevokeInvite` produce no directory event, so an event-only projection can never truthfully render current invite state.
3. **The current detail endpoint is unbounded and unsigned for retained UI use.** `get_directory_space` calls `list_members` and `list_invites` as full vectors. It returns ordinary JSON, with no session-binding hash, authorization generation, canonical receipt, cursor, or byte/page ceiling.
4. **The command completion is opaque/lost.** Hub `DirectoryCommandResponse` at [`🚀️bin.rs:3810`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3810) turns an issued token into ad-hoc `DslValue`; TypeScript declares `result?: unknown`; the worker posts `events` only. Thus Copy Invite Link cannot copy a real token or announce a receipt.
5. **The offline command queue is not an administration transaction owner.** The browser worker queues `{requestId, command}`, but `requestId` is not transmitted to the hub and is not idempotency authority. Retrying an uncertain `CreateInvite` can mint another invite. Auto-retrying admin mutations must stop until an exact server idempotency receipt exists.
6. **No WGPU parity.** Native has the same action-to-command conversion at [`Shell WGPU:393`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:393), but no authenticated detail-page operation, author panel, typed receipt, clipboard handoff, or cancellation lifecycle.

## Smallest non-overlapping P0

Leave the global `DirectoryEventPageV1` bootstrap/ACK owner to its active Home packet. Add a **space-specific, author-only administration page plus command receipt**; it consumes the Home page projection after that owner has applied it but does not change its scan/ACK/socket-resume semantics.

### 1. Replace the unbounded detail response with a canonical admin page

Under the shared directory schema (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️,🟦️}`), define `DirectorySpaceAdministrationPageV1`:

- schema/version; exact `spaceId`; session-binding SHA-256; authorization generation;
- `access: author | member | public`; the current caller role is server-filled;
- independently paged member and invite windows (maximum 64 rows each, total canonical bytes ≤48 KiB), opaque cursor(s) bound to user/session/generation/space/section; and
- `receiptSha256` over canonical unsigned JSON.

`author` is the only shape containing invites and management capability flags. `member`/`public` never contain empty placeholder invite arrays. A post-read session/auth-generation/role recheck must make a revocation/role downgrade return 401/403, not a stale detail page. Retire the unbounded `DirectorySpaceDetailV1` route shape rather than preserving it as a second surface.

Implement route work in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`, with bounded backend page primitives on `HubDirectory` (`🌎️hub/📇️directory/🦀️.rs`) and all SQLite/Postgres/Neo4j implementations. This is read projection work only; it does not overlap invite claim storage.

### 2. Close the command/result wire and make retry exact

Replace raw `DirectoryCommand` POST bodies with a schema-first `DirectoryCommandRequestV1 { requestId, command, expectedAdministrationReceipt }` and a closed `DirectoryCommandReceiptV1`:

- request id is bounded/canonical and keyed durably by actor/session authorization generation, space, canonical command digest, and request id;
- a same tuple returns the original receipt; same id with a different canonical command is conflict; different actor/scope cannot discover it;
- receipt contains command kind, durable event ids/seq range when applicable, resulting administration receipt/frontier, and a one-shot invite capability only for `create-invite`;
- the capability is held only by the operation until copy/display acknowledgement, never folded into Home config, event page, Redux-like state, logs, URL, or replay queue.

The server still performs fresh `authorize_directory_command` immediately before execution. `expectedAdministrationReceipt` gives a deterministic stale result (refresh required) for member/invite actions; it is not an authorization substitute. This prevents retry from silently issuing two invites and gives the UI a real receipt without optimistic mutation.

### 3. One shell-owned retained administration operation

Add a fixed-capacity `DirectoryAdministrationOperation` to the existing worker/native Shell directory lane—not a second socket or a generic queue. It owns:

1. canonical admin-page request/response bytes and an abort/deadline;
2. one closed command request/receipt; and
3. at most one invite capability until an explicit clipboard/display acknowledgement, followed by terminal close.

States: `Loading → Ready(author|member|public) → Submitting → Receipt → Refreshing → Ready`, with `Cancelled`, `Denied`, `Stale`, and `Failed` terminal. On unmount, identity/session generation change, 401/403, or scoped 4401, cancel and erase page/receipt/capability before notifying the renderer. The old global `directoryCommandQueue` may continue for commands that acquire exact server receipts; until then this P0 must make administration mutations fail **unknown outcome / refresh required**, not auto-retry.

For browser, extend `🧵️backbone-worker.ts` and the closed request/response types in `🧰️framework/🛍️products/💻️os/🟦️.ts`; ShellHost receives typed administration updates and never receives raw session identity. For WGPU, add the same finite operation/turn driver beside `ShellDirectoryRunner` in `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`; its native transport is the only HTTP owner.

### 4. Home/Space UI, not a client-side CRUD mirror

Add one `manageSpace` action on Home only when the authoritative current caller capability is `author`; `openSpace` stays available for nonauthors. Its host effect opens a Shell-owned Administration pane for the selected exact `spaceId`.

The pane renders solely from the canonical page:

- members with role selector and remove affordance; owner removal omitted/disabled before dispatch;
- current pending/accepted/revoked invite rows with issue/revoke control; and
- a receipt/status region, which changes only on an accepted server receipt followed by a page refresh.

The existing S Space panel may later share the view primitive, but it currently renders English-only tree text and accepts a local `SpaceIndexConfig`; it must not be treated as the authoritative admin pane. Its direct network-free action relays are reusable only after the closed receipt return path exists.

All labels/actions require explicit EN/DE values and semantic buttons, form labels, error/status live region, focus restoration after dialog/receipt, and keyboard operability. The browser and WGPU implementations render the same capability-derived action set; neither derives authority from a locally stored role.

## Acceptance matrix

1. **Neutral fixture + independent Bun/AJV oracle** for page/cursor/receipt canonical JSON: author positive; member/public omission; malformed/oversize/noncanonical; stale expected receipt; duplicate same request; same id/different command; foreign scope; revoked or generation-changed response; invite secret never present in page/event fixture.
2. **Hub native laws**: exact author results for add/change-role/remove/issue/revoke; spectator cannot get an author page or mutate; atomic invite redemption remains a separate existing law; retry under transport-uncertain command yields one receipt/invite; restart preserves receipt/idempotency; remove then immediate page/reconnect returns denied and no leaked invite/member rows.
3. **Browser worker/ShellHost law**: real `DirectoryClient` parser and fake authenticated transport drive Load → submit → receipt → refresh; no state change before receipt; cancellation/unmount preserves no pending secret; 401/403 clears pane; duplicate/reconnect does not dispatch twice. Use no bypass token.
4. **WGPU native law**: actual `DirectoryClient<NativeDirectoryTransport>` turn progression, bounded page/receipt close, capability-shaped controls, same stale/denied cancellation. This is UI transport proof, not rendering performance proof.
5. **Two-user process law**: owner creates a private space, opens the admin pane, adds/changes/removes a second identity and issues/revokes one invite; the second identity observes only ordered authorized page/socket changes, loses access after removal (4401/re-fetch denial), and cannot reuse a stale receipt. Verify restart and no duplicate invite on retry.

Register source/oracle and native gates through each package’s existing `📜️script.ts` and `project.json` target only; add the final two-user launch entry from the launch seed, not generated `launch.json`.

## Explicit nonclaims and stale assumptions

- **Fixed/stale:** the earlier assumption that invite redemption itself lacked transactional claim is obsolete; redemption now has a backend atomic path. This packet does not modify it.
- **Fixed/stale:** scoped document socket 4401 is real boundary work, but it is not global Home administration hydration.
- **Still RED:** Home’s current raw socket/fold path is not the new page-ACK bootstrap; that active owner remains separate.
- **Still RED:** neither Home nor S Space currently provides an authoritative member/invite screen or receipt status. Existing unit tests prove action mapping/render snippets only.
- This P0 does not create a cloud-admin SPA, grant admins implicit space-author powers, store secrets in a document, or claim WGPU/browser process success before the above laws run.
