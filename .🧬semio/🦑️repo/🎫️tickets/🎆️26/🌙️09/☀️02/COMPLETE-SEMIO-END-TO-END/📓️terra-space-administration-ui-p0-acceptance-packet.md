# 🏛️ First Home/Space Administration UI P0 — Current-Source Acceptance Packet

**Audit mode:** read-only source review, 2026-09-05. I ran no build, browser, native, socket, or process test. “Current” below means the shared tree inspected at the cited source anchors, not a runtime claim.

## Decision

**RED.** A real command path exists from the Home/Space artifact UI to the authenticated hub, and removal now fences membership invalidation correctly. It is not yet an executable two-user UI journey because:

1. the ordinary `DirectoryCommand` wire carries no server-durable idempotency key while both shells retry transport failures;
2. Home and Space consume raw event batches as if each were full history, rather than a bounded authenticated page with a sealed raw-scan frontier; and
3. the member panel depends on that broken Space projection and has English-only dynamic content. A scoped document socket is deliberately **not** the membership feed: it must never serialize `member.upserted` or `member.removed`.

The smallest honest P0 is therefore: **two real local-bootstrap identities; Home creates a private Studio space; Space’s member panel adds, changes, then removes the already-authenticated second identity; both clients derive their visible directory state from authenticated ordered event pages; the second client’s already-open document-scoped socket is revoked with `4401`, without ever receiving the removal event.** It does not claim that creation of a new space also creates a document or a document socket. That remains a separate trusted Space/Flow document-bootstrap prerequisite.

## Current evidence

| Segment | Existing source authority | What it proves / does not prove |
|---|---|---|
| Home create UI | `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-space/🦀️.rs:24-32` emits the exact `os.directory.create-space` effect. The Home manifest retains `createSpace` (`.../✏️editor/🦀️.rs:57-99,347-385`). | The authenticated command intent is real source, but not a confirmed UI outcome. |
| Space member UI | `.../🪐️space/.../📌️panels/👥️members/🦀️.rs:29-89` renders member rows and remove action; `💌invite-member/🦀️.rs:18-20` relays upsert-by-email; `🚪remove-member/🦀️.rs:16-18` relays removal. The manifest declares bilingual dialog/action labels at `.../✏️editor/🦀️.rs:252-328`. | Add and role change share `upsert-member`: submitting an already-existing email with a different role is the role change. There is no row-local “change role” affordance; the invite dialog must be reused deliberately or a named change-role dialog added. |
| Browser funnel | `ShellHost/🟦️.tsx:974-1000,3150-3169` maps the three effect ids to closed `DirectoryCommand` and posts to the worker. `backbone-worker.ts:1422-1469` queues network failures in order and posts accepted/rejected results. | Source retry exists. `ShellHost/🟦️.tsx:1458-1472` only logs rejection and folds raw returned events: no correlated accessible terminal state. |
| Native funnel | `.../🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:382-403,4045-4049` maps the same verbs; `:4096-4135` posts/requeues on transport failure. | Native emits only `eprintln!` for failure and silently drops no-identity commands. It has no user-visible per-command receipt, cancel, or non-lossy request identity. |
| Hub authorization and durable command | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3650-3682` permits create for an authenticated session and upsert/remove only for a current Author/admin; `:3728-3743` reconstructs the actor from the bearer and returns `202`. | The client does not supply actor/role authority. The ordinary command body has no request id. |
| Removal fence and socket revocation | `execute_directory_command_fenced` (`🚀️bin.rs:3709-3726`) holds the affected membership gate through `DirectoryService.execute` and `invalidate_binding`. The scoped issue endpoint requires a live member and actual document descriptor (`:2291-2331`). The live loop closes `4401` on notification/tick (`:4133-4198`). | This supersedes the older claim that REST removal was unfenced. It is source/native-test evidence only unless re-run. |
| Scoped privacy | `directory_message_matches_scope` (`🚀️bin.rs:3926-3946`) returns `false` for all member and space lifecycle events. Its focused socket law asserts a member removal produces `4401` without exposing that event (`:7817-7897`). | Correct boundary: client B observes membership state through its authenticated event page/detail projection, and observes revocation through its scoped socket. It must **not** expect a member-removed socket frame. |
| Ordered writer | `📓️root-directory-ordered-publication.md` records the current append→publish writer packet and registered checks under `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4729-4758`. | Treat it as a prerequisite, not evidence for page/projection/UI success. |
| Current projection failure | Space’s `FoldDirectoryEvents` explicitly requires *full* history and resets from `DirectoryReadModel::default()` (`.../🪐️space/.../📇fold-directory-events/🦀️.rs:1-7,35-46`). Home likewise preserves an incomplete JSON wrapper and silently defaults corrupt JSON (`.../🏠️home/.../🎚️config/🦀️.rs:17-54`); the old Home raw fold is excluded from retained factories (`.../✏️editor/🦀️.rs:57-99,212-239,561-579`). | A stream batch, command result, reconnect replay, or filtered page cannot truthfully update either client’s member view. |

### Two non-negotiable source corrections

`DirectoryClient.command` posts a naked `DirectoryCommand` in TypeScript (`🧰️framework/🛍️products/💻️os/🟦️.ts:4267-4274,4301-4303`) and native Rust (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:672-675`). The browser worker and WGPU retry the same logical operation after an uncertain outcome. The closed command taxonomy itself has no request field (`.../📇️directory/🧬️schema/🦀️.rs:200-218`, `🟦️.ts:134-158`). A lost `202` can therefore produce a second event (and for create, a second space). The admin intent taxonomy is not a substitute: it has `request_id`, but Home/Space do not invoke it.

Also, a `CreateSpace` event creates no `DocumentDescriptor`. The scoped-grant endpoint correctly returns `404` until an authenticated Author has announced a descriptor (`🚀️bin.rs:2317-2321`). The process phase below requires a pre-existing, authenticatedly announced descriptor in the created space, but that announcement/real trusted Space document creation is **setup owned by the document-bootstrap lane**, not evidence that the Space UI created it.

## P0 contract and ownership

### 1. Idempotent ordinary command envelope — hub/directory slice

Introduce one schema-first, Rust/TS/JSON record, without changing the semantic `DirectoryCommand` enum:

```text
DirectoryCommandRequestV1 {
  requestId: canonical UUID/opaque bounded ID,
  command: DirectoryCommand
}
DirectoryCommandReceiptV1 {
  requestId, actorUserId, authorizationGeneration,
  commandSha256, firstSeq, lastSeq, events, result?
}
```

The server derives `(actorUserId, authorizationGeneration)` from the current durable session. Its database transaction performs: authorize → look up `(actor, generation, requestId)` → reject same key/different command hash → return exact recorded receipt for same hash → decide/append/project → persist receipt → ordered broadcast. The receipt is retained for the same bounded lifetime as directory command replay, not supplied by React/WGPU authority. A request cancelled after durable commit may lose its response, but retry receives the original receipt and does not append again. A request cancelled before commit changes nothing.

Use this envelope in `DirectoryClient`, both action funnels, and the local browser relay. Do not let the renderer fabricate a second request id on retry; the owner is the retained shell command record until receipt/terminal cancellation. The native queue must retain the envelope, not just `DirectoryCommand` (`pending_directory_commands` at `wgpu/🦀️.rs:1798-1802`). The browser queue similarly changes from `QueuedDirectoryCommand { requestId, command }` to the sealed request it first issued (`backbone-worker.ts:1366-1469`).

### 2. Bounded authenticated directory pages — OS directory/Home/Space slice

Implement the event-page P0 in `📓️terra-shell-directory-event-page-p0-acceptance-packet.md`, but extend its *consumer* to the one Space member panel required here. The earlier packet’s **Home-only** scope is intentionally insufficient for this journey.

The page is a session-binding/generation-bound raw scan receipt with `(afterSeqExclusive, throughSeqInclusive, hasMore, ordered visible events, receiptSha256)`, fixed raw/item/byte caps, and no raw session id. Invisible raw sequence holes are legal. Socket heartbeats and raw stream `lastSeq` are dirty hints only, never durable page cursors.

Implement one shared retained `DirectoryProjectionPageOwner` whose terminal payload is a verified page and whose close consumes copied bytes under fixed grants. It retains the current authenticated projection state:

```text
DirectoryProjectionStateV1 {
  bindingDigest, authorizationGeneration,
  rawScanThrough, lastReceiptSha256,
  DirectoryReadModel
}
```

The state may be serialized in the active Home/Space config, but it must be strict and preserve `DirectorySpace.documents`; config corruption is a terminal local recovery state, never `unwrap_or_default()`. It applies each page exactly once, advances only after the single local config replacement is durable, and accepts a different binding only by validated `after=0` reset. The Space adapter derives the *selected space* members/visibility from that state; it never rebuilds from `DirectoryReadModel::default()` on a delta.

The minimal first implementation can reject/ask resync for a page that exceeds the documented cap; it may not silently truncate or discard a continuation. Full continuation is required before claiming general directory browsing, but the P0 process fixture may remain below the first page’s cap.

### 3. Shell/UI terminal state — Browser and WGPU slice

The shell must carry a correlated command request id plus exactly one terminal state: `pending`, `succeeded(receipt range)`, `denied`, `unavailable/retryable`, or `cancelled`. Only the final committed page makes a created space/member row visible. It must not optimistically mutate either app config.

Browser changes land at the existing `BackboneWorkerRequest/Response` protocol (`🧰️framework/🛍️products/💻️os/🟦️.ts:705-744`), worker, and `ShellHost`; native changes land at `ShellState` rather than a second renderer queue. WGPU must reuse its existing typed operation-result exchange/ACK (identified in the prior page packet), while its current raw `dispatch_directory_event_batch` (`wgpu/🦀️.rs:4138-4149`) is removed from this ingestion path. A stream event merely sets one bounded dirty bit. `4401` clears the scoped viewer state and disallows reconnect with the stale grant; `1013` schedules page revalidation from the last committed raw scan frontier.

The members panel must receive explicit resolved locale/terminology through its view context. Current panel/tab/dialog metadata is bilingual, but dynamic `TreeItem` labels are documented English-only (`📌️panels/👥️members/🦀️.rs:45-54`). Add localized label keys for member role, empty state, invite/change/remove, pending, succeeded, denied, unavailable, revoked and recovery. Use accessible action names, `role=status`/live result announcement, visible keyboard focus, and a confirm dialog for removal. Never encode a per-user locale in the shared `SSpaceSnapshot`.

## Exact executable journey

### Identities and setup

Use the existing protected local bootstrap—not `S_USER`, `OS_HUB_ADMIN_TOKEN`, hand-made bearer strings, direct DB writes, or `DirectoryService::execute` helpers. Bootstrap two distinct persistent local profiles through the same browser relay/session path that the hub accepts. Before upsert, client B must already have a real authenticated session and known email: the service otherwise resolves an unknown email as a newly created user (`🚀️bin.rs:6098-6103`).

Provision a descriptor only through an authenticated Author `AnnounceDocument` command and the trusted document-bootstrap owner. This creates the scope required to demonstrate `4401`; it is setup and must be visibly recorded as such. The user-visible actions under test remain Home create and Space add/change/remove.

### Positive flow

1. Client A signs in, Home opens in EN, invokes the real create-space dialog, and sends one sealed `create-space` request. Its `succeeded` receipt has one durable `space.created`; the verified page makes exactly one localized Home row visible.
2. A opens the preprovisioned Space index for that space. The authenticated page populates the member panel. A submits invite/upsert with B’s existing email as `spectator`; B’s own authenticated page shows only permitted member state. A resubmits the same member with `author`; both derived rows show the changed role and one ordered transition.
3. B acquires a genuine scoped grant for the provisioned descriptor, opens its exact scope socket, sends protocol Hello, and receives only allowed document-scope frames. It receives no `member.*` frame.
4. A removes B using the rendered member row and confirmed dialog. The directory receipt/page derives B’s removal; the scoped socket receives terminal `4401`; the old grant cannot reconnect or mint another scoped grant. A’s state remains authorized.

### Hostile cases

- Cross-space request or descriptor substitution: `401/403`, no receipt/event/projection change and no foreign frame.
- Spectator B tries upsert/remove: `403`, no head advance, no local state mutation, terminal localized denied status.
- Duplicate same envelope after response loss: byte-identical receipt/range, no second event. Same request id with altered command: terminal conflict, no append.
- Stale authorization generation/session and B’s old scoped grant: deny issue/replay; a live B socket closes `4401` without serializing `MemberRemoved`.
- Browser/native cancellation before send, after durable commit/before response, during page scan, after prepare/before ACK, and after ACK: exact owner close; only the durable-receipt retry may surface success.
- Ordered A add/change/remove under concurrent unrelated-space writer: visible page raw frontier is monotone, visible membership rows follow A’s durable order, and no cross-space data reaches B.
- Disconnect/`1013` then reconnect: dirty hint causes page fetch from committed raw frontier; no heartbeat-based cursor advance or duplicated change. Restart the SQLite hub: recreate sessions/grants, retain durable events/projections/command receipt as policy permits, and do not reuse an old grant.

## Non-overlapping implementation slices

| Slice / owner | Bounded change | Deliberately excluded |
|---|---|---|
| **Sol Hub directory** | `DirectoryCommandRequestV1` receipt table + route wrapper at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3709-3743`; authenticated event-page endpoint and append-time event cap. Reuse the current ordered writer; do not reimplement its lock. | Admin intent route, invite redemption, document mutation, presence lease, public raw stream. |
| **Sol OS directory** | Shared Rust/TS page/command schema, retained byte scanner/close, strict projection state and event-page client. | A generic directory snapshot or any unauthenticated page. |
| **Sol Space UI** | Home and Space config adapters plus retained config preparation for the real page footprint; page-derived rows; localized dynamic members panel and confirmed removal. | Shared snapshot mutation, direct guest network access, role authority. |
| **Sol Browser + WGPU** | One envelope owner/terminal response and one dirty→page operation; browser broker whitelist; native result-exchange ACK reuse. | A global raw event relay, second UI queue, raw heartbeat cursor. |
| **Sol journey gate** | Isolated real SQLite hub + two bootstrap identities + browser UI; a separate native loopback shell law. | Existing obsolete `collab-e2e`, mocks, test-only user/session injection. |

## Acceptance registration

### Language-neutral source fixture

Add `space-admin-ui-journey-v1` next to the directory fixtures with Rust/TS schemas and Bun/AJV plus independent Node SHA-256 oracle. Rows own only public fixture identity metadata—not capabilities—and pin:

- two distinct bootstrap identities/email bindings; create/upsert(role spectator)/upsert(role author)/remove intent hashes and the one request-id retry pair;
- resulting ordered event bodies/ranges, page scan frontiers including invisible holes, receipt hashes, page byte/item caps, authorized client projections and localized EN/DE message keys;
- document scope, no-visible-member-frame assertion, `4401` after removal, grant replay denial, cross-space/spectator/stale-generation/changed-request denial; and
- cancellation, `1013` re-page, duplicate delivery, restart and malformed/corrupt local projection rows.

### Focused native laws (proposed registrations; none currently pass this P0)

Register through each owner’s `📜️script.ts`, project target, and `.vscode/🧩️launch.seed.jsonc`, then regenerate launch JSON:

```sh
# Existing prerequisite, not re-run by this audit.
bun nx run os-hub:directory-ordered-publication-check --skip-nx-cache
bun nx run os-hub:directory-ordered-publication-native-check --skip-nx-cache

# Proposed after the slices above exist.
bun nx run os-hub:space-admin-ui-journey-check --skip-nx-cache
bun nx run @semio-tech/framework-os:directory-event-page-native-check --skip-nx-cache
bun nx run @semio-tech/framework-os-dev:space-admin-ui-browser-check --skip-nx-cache
```

The hub native law starts the real SQLite server, creates two bootstrap-issued sessions, exercises REST/WS grants and decodes exact frames. The framework native law runs one real retained page operation at grants `1`, `64`, and cap through prepare → durable config commit → terminal result → ACK → terminal-empty, and closes every denied/cancelled owner. The browser process law drives the real Home/Space UI, not a mocked `DirectoryClient`. Native WGPU UI evidence is a separate loopback renderer/process law and remains RED until it renders the localized terminal state.

The existing `bun nx run os-hub:admin-live-journey-check --skip-nx-cache` is credible narrower SQLite/admin evidence recorded in prior reports, not proof of this Home/Space journey. `bun nx run @semio-tech/framework-os-dev:collab-e2e` remains unsuitable because its current bootstrap/document-target assumptions are stale.

## Reconciled stale assumptions

1. `📓️terra-space-administration-user-journey-current-audit.md` and `📓️terra-two-user-space-admin-runtime-audit.md` predate the current ordered writer and fenced removal work. Do **not** repeat “REST removal releases its fence” or “append broadcasts outside the writer” as current REDs.
2. `📓️terra-scoped-directory-socket-membership-revocation-blueprint.md` is now source-realized at the scoped grant, binding, notification and one-second revalidation anchors above. It is still not a full two-client process result.
3. `📓️terra-shell-directory-event-page-p0-acceptance-packet.md` correctly declares raw ingestion RED and WGPU’s reusable result/ACK bridge. Its Home-only consumer boundary is stale for this task: the Space member panel needs the same strict page/projection adapter. Its warning that a scoped socket cannot carry member events remains correct and is strengthened here.
4. A just-created space is not a document scope. Any report saying that create-space alone permits a scoped socket is false; descriptor bootstrap is an explicit prerequisite.

## Honest nonclaims

This packet does not prove trusted Space component installation, Flow child/member open, WGPU rendering, document mutation/presence, long offline operation, generic paginated directory browsing beyond the declared page cap, production OIDC, PostgreSQL/Neo4j, invite redemption ordering, or presence lease expiry. Current code/source fixtures and prior recorded gates are prerequisites only; no P0 runtime result is claimed here.
