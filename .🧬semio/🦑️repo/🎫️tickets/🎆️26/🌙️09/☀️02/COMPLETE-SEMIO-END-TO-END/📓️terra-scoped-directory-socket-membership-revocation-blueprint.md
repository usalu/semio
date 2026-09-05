# Scoped Directory-Socket Membership Revocation — P0 Blueprint

Status: source audit only, 2026-09-05. No build, native test, browser test, or process law was run by this audit.

## Decision

The first honest P0 is a **scope-bound directory subscription**, not a conditional filter on the present global directory stream. A grant must bind one `DocumentScope`; every admission, replay item, live item, lag rebootstrap, periodic check, and outbound text send must use that same scope. A durable `MemberRemoved` must serialize with those sends through a `(userId, spaceId)` membership gate and invalidate the matching live grants before that gate is released. The server then sends close **4401** with no event body. An invisible event from another scope is only `Skip`; it is never revocation.

This preserves the current single-directory-writer law: durable append occurs before broadcast and both occur under `DirectoryService`'s writer guard. It does not expose a raw removal event as a revocation signal.

## Current source evidence

| Boundary | Current behavior | Consequence |
|---|---|---|
| Grant audience | [`🚀️bin.rs:545-548`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:545) has `Directory { auth_session_id, authorization_generation }`, with no scope. | A `?spaceId=&documentId=` query is not cryptographically or ledger-bound to the grant. |
| Grant issue | [`🚀️bin.rs:2226-2234`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2226) authenticates a session and creates that unscoped audience. | No membership or descriptor is checked while issuing a directory grant. |
| Durable authority | [`🚀️bin.rs:571-595`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:571) calls `socket_session_binding(..., None, ...)` for directory grants. The backend exposes `MembershipLost` ([`📇️directory/🦀️.rs:245-253`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:245)), and all three backends return it when a supplied space has no role (SQLite: [`🪶️sqlite/🦀️.rs:1069-1072`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1069)). | The required durable primitive exists but the directory socket bypasses it. |
| Upgrade query | [`🚀️bin.rs:3794-3813`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3794) accepts a paired optional `spaceId`/`documentId`, but consumes a scope-free `pending_directory` grant. | An arbitrary valid directory grant may be paired with any syntactically valid scope. |
| Message visibility | [`🚀️bin.rs:3771-3791`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3771) revalidates unscoped then tests each event's *current* space membership; every non-visible message becomes `Unauthorized`. | The caller cannot distinguish an unrelated invisible message from actual loss of the subscribed scope. |
| Delivery | [`🚀️bin.rs:3821-3834`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3821) obtains only subject gates, checks visibility, then serializes text. Replay/live branches silently ignore `Unauthorized` ([`🚀️bin.rs:3921-3933`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3921), [`3955-3970`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3955)). | A revoked scoped connection remains live and a harmless foreign event shares the same branch. |
| Periodic authority | The one-second tick calls `socket_live_authority` ([`🚀️bin.rs:3935-3947`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3935)); that helper takes only subject gates and unscoped revalidation ([`1328-1342`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1328)). | Removing membership with no later message does not close the socket. |
| Ordered writer | `DirectoryService::append_and_publish_locked` appends, then broadcasts while holding one writer guard ([`📇️directory/🦀️.rs:1596-1610`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1596)); `execute` holds it across decide/append/publish ([`1662-1670`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1662)). | There is a correct ordered durable boundary to compose with; do not move broadcasts to an asynchronous post-commit queue. |
| Removal ingress | Only production generic command callers are REST [`🚀️bin.rs:3607-3618`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3607) and admin intent [`4556-4568`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4556). Both invoke `DirectoryService::execute` directly. The decider's `RemoveMember` emits `MemberRemoved` ([`📇️directory/🦀️.rs:1397-1403`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1397)). | Two ingress sites must share one fenced execution helper; patching only the REST route leaves admin revocation racy. |
| Existing session revocation pattern | The ledger indexes live grants by one subject binding ([`🚀️bin.rs:629-634`, `769-827`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:629)); session revocation holds a binding gate then invalidates grants/plans ([`4027-4038`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4027)). | Reuse this ledger/notify model, extended with a scoped membership binding; do not add a second socket registry. |
| Client close handling | Browser `DirectoryClient.stream` reconnects after every close and ignores the close code ([`🟦️.ts:4333-4440`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:4333)); native reduces `Message::Close(_)` to untyped `DirectoryWsPoll::Closed` and reconnects ([`🔌️client/🦀️.rs:133-139`, `914-955`, `1143-1154`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:133)). | A legitimate 4401 would cause an endless reissue/redial loop instead of a terminal scope-revoked state. |

Existing tests prove session revocation and ordinary membership visibility, not scoped membership revocation: [`socket_grant_directory_route_uses_credential_free_hello_and_revokes_live`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7551), [`socket_directory_revoke_after_admission_suppresses_replay_without_deadlock`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7577), and [`socket_directory_visibility_requires_membership_even_for_public_spaces`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7630). The registered existing server gate is `bun nx run os-hub:socket-grant-check --skip-nx-cache`, from [`.vscode/launch.json:4885-4889`](/Users/ueli/Documents/semio/.vscode/launch.json:4885) and [`📜️script.ts:2394-2402`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2394). It has no scoped-removal case today.

## P0 protocol and ownership

### 1. Seal a scope before upgrade

Add a schema-first `DirectoryScopedSocketGrantIntentV1` / receipt route that takes the full `DocumentScope` server-side (prefer a new scoped path such as `POST /directory/spaces/{space}/documents/{document}/socket-grants`; no client-supplied scope body and no raw session token in the WebSocket). At issue time:

1. validate both bounded identifiers;
2. authenticate the session;
3. require `get_document_descriptor(scope)` to be present;
4. call `socket_session_binding(session, user, generation, Some(scope.space_id), now)` and require `Active { role: Some(_), same_expiry }`;
5. mint `SocketAudienceV1::DirectoryScoped(scope)` and a normal credential-free, one-use socket grant.

The upgrade parses the same exact scope from its URL and calls normal `pending(capability, &DirectoryScoped(scope), now)`, not `pending_directory`. Query scope substitution therefore fails before the WebSocket is upgraded. The unscoped directory stream is not evidence for this P0 and must remain explicitly outside its acceptance claim until separately redesigned.

`SocketSubjectV1::revalidate` needs a new `DirectoryScoped(scope)` arm using `socket_session_binding(... Some(scope.space_id) ...)`. It must require current membership but need not freeze an otherwise-authorized member's role: directory raw-event visibility is member-only, not author-only. A missing descriptor or lost membership maps to `Unauthorized`; backend failure/time-out maps to `Unavailable`.

### 2. Make revocation and invisibility distinct

Replace the boolean/three-state result of `socket_directory_message_visible` with a scoped decision owned by the directory socket code:

```text
ScopedDirectoryFrameDecisionV1
  Deliver
  SkipUnrelated
  CloseUnauthorized
  CloseUnavailable
```

The decision order is non-negotiable:

1. acquire the record's complete gates (subject plus scoped membership);
2. revalidate the sealed audience and live ledger record;
3. if membership/session/descriptor is no longer valid, return `CloseUnauthorized` before JSON serialization;
4. if the directory cannot answer, return `CloseUnavailable`;
5. only then apply a total `message_matches_scope` predicate and return `SkipUnrelated` for anything outside the exact `DocumentScope`.

`message_matches_scope` must be body-specific. Space membership alone is insufficient: it would disclose unrelated documents and member mutations in the same space. It must deliver only controls whose scope equals the sealed scope and event bodies that own that scope (for example `DocumentAnnounced`, checkpoint, and retention events with an equal `DocumentScope`). Global user events, other documents, other spaces, connection/presence without exact document scope, and heartbeat remain `SkipUnrelated`. A `MemberRemoved` is never serialized merely to announce revocation. Define the predicate exhaustively over `DirectoryEventBody`; an unknown/new body is fail-closed `SkipUnrelated` until its scope rule is deliberately added.

Apply the same decision to replay, live broadcast, and lag rebootstrap. Replay/live update `last_replayed` only after `Deliver`; a skipped foreign event must not advance the scoped durable cursor. The current query's scope is used only for lag recovery ([`🚀️bin.rs:3972-3991`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3972)); P0 turns it into the authority for the entire connection.

### 3. Linearize member removal with outbound delivery

Periodic checks alone leave an outbound check/send race. Use the existing ledger and gate, not a new queue:

* Add `SocketBindingKeyV1::Membership { user_id, space_id }` and derive all record bindings from the sealed audience. A scope-bound session record has `User`, `Session`, and `Membership`; acquire them in the enum's deterministic sorted order.
* Extend `SocketGrantLedgerV1`'s pending/live indexes, `register_live`, `unregister_live`, `is_live`, and `invalidate_binding` to use all record bindings. `invalidate_binding(Membership { target_user, space })` removes pending grants and wakes every matching live lease.
* Replace direct production `directory_service.execute` calls at `🚀️bin.rs:3617` and `4560` with one hub-local `execute_directory_command_fenced`. For `RemoveMember`, it first acquires the target membership gate with the existing two-second timeout, then invokes `DirectoryService::execute`, then invalidates the membership binding **after durable append/broadcast returns but before releasing that gate**. Other commands pass directly through the same helper without inventing a membership lock.
* `socket_live_authority` must acquire record gates, revalidate the sealed audience, check `is_live`, and return those guards. `send_socket_directory_message` holds them through the matching decision and `sender.send`. If send won the membership gate first, its frame happened before durable removal; if removal won first, the ledger is invalidated and no frame is serialized. This is the required linearization boundary.

This is compatible with `DirectoryService`'s current append→broadcast order: a removal gets persisted and published under the writer lock, then its outer membership fence wakes/closes consumers before another scope-protected send can pass. It does not make `DirectoryService` depend on a binary-only socket ledger.

### 4. Close and reconnect semantics

At admission, replay, periodic revalidation, live message delivery, and lag recovery:

* `CloseUnauthorized` sends exactly `Close(4401, "unauthorized")`, emits no `DirectoryStreamMessage`, exits, and lets `SocketLiveLeaseV1::drop` unregister it.
* `CloseUnavailable` sends 1013 and may retry later.
* `SkipUnrelated` does neither.

Expose the 4401 distinction through both clients. Browser `onclose` must inspect the `CloseEvent.code`; native `DirectoryWsPoll::Closed` must preserve `Option<u16>` from tungstenite/browser transport. A scoped stream transitions to a terminal `Revoked`/`Unauthorized` turn on 4401, closes its transport, clears only that scope's retained event-page/projection owner, and never reacquires a grant automatically. Network/1013 remains bounded reconnect with the exact sealed scope and last delivered scoped sequence. No close reason is trusted as application data.

## Minimal files and ordered packet

1. **Hub grant and fence authority** — [`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs): scoped audience, multi-binding ledger/gates, scoped issue/consume/upgrade, total delivery decision, and the shared fenced command executor. Add no new database table and no raw-event redactor.
2. **Schema and neutral oracle** — new sibling schema/fixture under the hub package's existing directory/socket test taxonomy, consumed by [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts). The oracle frames scope, event/control class, durable binding result, gate winner, expected outbound bytes (`none` or close code), and cursor effect. It must independently enforce exact object keys and byte bounds.
3. **Shared client contract** — [`🧰️framework/🛍️products/💻️os/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts): a scoped grant/stream request and code-aware close handling. [`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs): exact scoped URL path/query encoder, scoped grant path, typed close code, and terminal turn. Update its URL unit law at [`1916-1920`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1916), not a stringly test-only branch.
4. **Shell owner** — keep [`ShellHost`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx) as renderer only. Its backbone-worker directory owner receives the typed terminal state and clears its retained scope state; it must not fold or display a raw `MemberRemoved` body.

## Non-vacuous acceptance matrix

| Law | Required observable result |
|---|---|
| Neutral scope authority | Same session/grant paired with a substituted URL scope is denied before upgrade. One differing scope field, a missing descriptor, malformed pair, stale generation, and unknown event body all deny/skip without raw bytes. |
| Neutral distinction | An event visible in another member space produces `SkipUnrelated`, keeps the scoped lease/live connection, and does not advance scoped `lastSeq`. Actual membership loss produces `CloseUnauthorized`/4401. |
| Native hub: winning removal | Pause a sender before membership-gate acquisition; remove target via each ingress (REST and admin), release sender, assert durable `MemberRemoved` precedes notification in the writer order, socket closes 4401, and reads no raw text. |
| Native hub: winning send | Hold sender's scoped membership gate first, assert one authorized scope event arrives, then removal completes and the next observable result is 4401. This proves the permitted happens-before alternative, not a timing heuristic. |
| Native hub: periodic / replay / reconnect | Remove membership with no live event and close within the bounded tick; remove after grant issue before Hello/replay and deny without replay; reconnect with the old receipt is denied; a new receipt after re-admission is allowed only for that scope. |
| Native hub: privacy and availability | Cross-space and same-space/different-document messages never serialize; public discovery does not change raw socket policy; directory failure gives 1013, not 4401; cancellation/peer close unregisters the live lease exactly once. |
| Process/browser/native | Start the actual hub, two authenticated users, and two scopes. Remove one member through actual admin and REST routes. Browser and WGPU/native see a typed terminal revocation, no reconnect storm, no stale scope projection, no raw removal payload; the other user's and unrelated scope's subscription continue. Restart/replay still requires a freshly scoped grant. |

Register the selected native laws in the existing `SocketGrantCheckScript` rather than creating an unregistered build path, then use its already registered launch target: `bun nx run os-hub:socket-grant-check --skip-nx-cache`. Add a separate bounded browser/process exercise only after the native fixture passes; the present gate has no scope-close proof.

## Explicit nonclaims

This P0 does not make the existing global directory stream safe for document-scoped use, does not deliver/redact arbitrary raw events, does not alter document-sync socket authorization, and does not make presence lease expiry real. It also does not claim tests or a process journey have passed; this audit ran neither.
