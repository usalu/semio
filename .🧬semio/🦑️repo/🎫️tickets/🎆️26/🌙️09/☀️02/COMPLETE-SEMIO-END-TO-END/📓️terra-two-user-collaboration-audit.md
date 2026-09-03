# Real Two-User Collaboration Journey — Current Audit

Observed 2026-09-03 from the shared tree. This is a read-only audit: no Cargo build, Bun/Nx test, Docker command, hub process, or external backend was run. Source was re-read immediately before this report, so it distinguishes currently composed behavior from fixture-only and test-only claims.

## Verdict

The opt-in TypeScript E2E is not a current proof of a real two-user collaboration journey.

Its first deterministic failure, once `HUB_E2E=1` is enabled and a hub is actually booted, is stale test expectation: it asserts that a viewer-surface peer is absent from editor rosters. Production is intentionally document-wide and broadcasts that peer to every connection on the document. More seriously, the public session-mint endpoint accepts an arbitrary email and returns a session for that existing user, so the E2E's two “authenticated” clients do not model production authentication at all. Trusted checkpoint publishing is implemented as an adapter but is not composed into the hub process; therefore a forced lag can close with 1013 yet cannot be shown to yield a recoverable verified checkpoint.

Do not declare the two-user journey complete until real identity binding, per-session actor binding, authoritative checkpoint publication, and end-to-end recovery are present.

## Actual path and coverage matrix

| Journey step | Current composed path | E2E coverage | Classification |
| --- | --- | --- | --- |
| Authenticate two people | `POST /auth/sessions` finds or creates a user solely from request `email` and mints a 30-day bearer (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:496-509`). `DirectoryClient::mint_session` calls that endpoint (`📇️directory/🔌️client/🦀️.rs:339-341`). | Calls `mintSession("user1…")` and `mintSession("user2…")` at `🧪️index.test.ts:363-371`. | **Critical production authority defect.** This is explicitly dev-mode behavior, publicly routed at `bin.rs:1849`; knowing a member's email permits session impersonation. The E2E proves only this dev fixture mechanism. |
| Create/private open and membership | A bearer session may create; an author may upsert/remove members and announce descriptors (`bin.rs:1185-1250`). Private listing/detail filters by membership (`:1253-1328`). | Creates private studio, adds user 2, observes directory event, announces `index` (`test:374-401`). | Partially covered, conditional on the insecure session mint. |
| Open document | Document WS resolves session role/share/public, requires a durable descriptor, and verifies declared schema/hash (`bin.rs:347-365,816-849`). | Both users Hello successfully. | Covered for happy path only; no negative identity/role cases in this E2E. |
| Document-wide presence | Presence map/fanout key is document scope, not surface; `Presence` emits the whole document roster and directory telemetry (`bin.rs:179-192,789-797`). `surface` is telemetry inside peer data. | Test title/header still says “presence-per-surface”; after viewer C publishes it requires A/B *not* to see C (`test:427-452`). | **Stale test expectation, not production bug.** Production Rust test already asserts document-wide visibility (`bin.rs:2767-2807`). |
| Concurrent edit | Every accepted batch is Fsync-submitted, then relayed on document fanout (`bin.rs:689-744,765-783`). The DB batch checks one target document and engine checks it again (`db/📄️artifact/🦀️.rs:86-103`). | Only A submits one mutation; B/C merely observe it (`test:454-466`). | Not tested for simultaneous authors, causal conflict/order, duplicate retry, or convergence after reconnect. |
| Checkpoint | `HubVerifiedCheckpointPublisher` atomically projects a verified private/public checkpoint when invoked (`hub/📇️directory/🦀️.rs:871-884,942-963`). `CheckpointPublicationOrchestrator` exists (`hub/🗿️artifact-authority/🦀️.rs:415-463`). The hub `main` instantiates only `VerifiedRebootstrapSource`, not this publisher/orchestrator (`bin.rs:1985-2019`). | Validates an AJV/Node-crypto fixture only (`test:49-100`); it performs no hub checkpoint publication. | **High production composition blocker.** There is no live path from edit to trusted artifact checkpoint. |
| Short disconnect/reconnect | A fresh Hello passes its frontier to `db.hello` (`bin.rs:816,888`); disconnect removes ephemeral presence and records session close (`:1023-1032`). The hello `resume_token` is not consumed in this handler. | None. No reconnect with frontier, missed command, or roster repair assertion. | Unproven; a new Hello may have DB catch-up semantics, but the journey has no runtime evidence. |
| Forced lag/rebootstrap | Per-document fanout capacity is 256 (`bin.rs:213-216`). A lagged receiver rechecks authorization, sends verified control only if available, then closes 1013 (`:616-637,1011-1015`). Directory live fanout is 1024 and follows an analogous close (`:1464-1468`). | Fixture encoding only (`test:118-169`); neither socket is made lagged nor reconnected. | **High recovery blocker.** With no composed publisher/authority, `VerifiedRebootstrapSource::control` can be unavailable, leaving only close 1013 and no usable checkpoint control. |
| Restart | Startup reopens DB/directory under the same `OS_HUB_DATA`, marks stale sync sessions closed, and recreates ephemeral fanout/presence (`bin.rs:1985-2018`). | Stops/restarts and checks descriptor plus numerical status only (`test:487-510`). | Partial. It does not reopen a document with an old frontier, verify replay/catch-up, checkpoint recovery, privacy, or presence reset. |
| Share/revoke | Share create/revoke are admin-only; document auth reevaluates share authorization every one-second WS tick (`bin.rs:453-478,821,966-974`). Existing Rust test exercises a revoked share socket. | None. | Implementation exists for live document access, but two-user E2E lacks share/revoke assertions. |
| Membership revoke | `member.removed` deletes membership projection (`hub/📇️directory/🪶️sqlite/🦀️.rs:335-337`); document WS reevaluates authorization every second. Directory telemetry is filtered per current membership (`bin.rs:1331-1367`). | None. | Partial/untimed. Add a live socket and post-revocation no-read/no-write test. |
| Admin kick | A configured admin bearer signals a live socket by `syncSessionId` and the WS loop exits (`bin.rs:1694-1704,1009-1017`). | Asserts C closes (`test:469-484`). | Covered as a connection kick only. It does not revoke C's bearer, so C can reconnect. |
| Admin “revoke user sessions” | Route filters **live sync sessions** and notifies them; it does not call `revoke_auth_session`, and its own comment describes the missing enumeration (`bin.rs:1707-1723`). | None. | **High production defect/name-contract gap.** Offline or later connections keep working; live user can reconnect with the same bearer. |
| Delete space | Owner/admin command deletes directory rows/projections including shares and checkpoint projections (`directory/🦀️.rs:686-688,492-495`; SQLite removes private checkpoint rows at `sqlite/🦀️.rs:317-320`). Later document Hello is denied. | None; Rust test only checks a post-delete *new* Hello (`bin.rs:2996-3010`). | Medium lifecycle gap. Existing connections only observe the one-second auth tick; no evidence of physical DB-WAL/blob erasure or no-recovery after delete. |

## First deterministic E2E failure chain

1. Default `bun nx run os-hub-ts:test` intentionally skips the real journey: `HUB_E2E !== "1"` at `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:349`.
2. With `HUB_E2E=1`, the script builds the default-feature `os-hub` binary before Vitest (`📜️script.ts:13-28`), the harness spawns it using a temporary data directory and static test admin token (`🟦️.ts:90-140`), and the scenario reaches the third socket if the local build/runtime is usable.
3. C connects with a viewer `surface` and sends presence (`test:427-431`). The hub stores C under the same `(space, document, actor)` key and fans out a roster containing A, B, and C (`bin.rs:789-797`).
4. The drain at `test:441-452` deterministically asserts `actors.has(actorC) === false` and size `<= 2`. It therefore fails before the first mutation, admin kick, or restart.

Update that test to assert all three actors at both surfaces, while retaining the invariant that `surface` is represented inside the opaque peer/telemetry rather than a fanout access boundary. Do not “fix” production back to surface-scoped presence.

## Authority and privacy invariants

- A remote identity provider, not the email string supplied to `/auth/sessions`, must bind each bearer to a principal. Until then no private-space authorization result is trustworthy.
- The server must bind `Hello.actor` and every envelope actor to the authenticated subject/device identity. At present the client supplies `actor` at `bin.rs:816`, it becomes the security principal at `:854-875`, and `admit_writes` receives each independently supplied envelope actor (`:746-783`). A member can claim another actor's ID, corrupting presence/color/replay attribution. Require a server-issued actor or validate a subject-owned actor namespace; reject envelope actors that differ.
- Authorization has to be rechecked before durable write, every sensitive read/stream delivery, checkpoint transfer, and after reconnection. The one-second document tick is a bounded delayed close, not an atomic revocation barrier around an already accepted command.
- Presence, connection telemetry, member identities, and checkpoint controls are visible only to current members. `directory_message_visible` applies that distinction for telemetry even when metadata is public (`bin.rs:1352-1367`); retain it.
- Use structural `DocumentScope { space_id, document_id }` everywhere. DB uses a scope-qualified artifact ID; no cache, event, or checkpoint may key on `document_id` alone.
- A share token/public visibility admits only a read-only spectator. It must neither confer member telemetry, writes, generic blob access, nor access after share deletion/revocation.
- Admin kick terminates a connection; session revocation must delete/invalidate the auth credential as well. Both outcomes need distinct externally observable contracts.
- Space deletion must specify retention/erasure of WAL, blob/CAS, private checkpoint locator, and backup data. Deleting the directory projection alone is access revocation, not demonstrated data destruction.

## Dependency-ordered implementation packet

### 1. Make two identities real and session/actor authority explicit

In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` and the directory identity boundary, remove the production route that mints an existing user's bearer from an email. Put dev identity only behind an explicit test-only/injected identity adapter; production must validate a provider assertion and derive `user_id` server-side. Require a configured admin token outside an explicit local-dev profile.

Create a bounded server-issued actor/session binding at document Hello. Validate actor and envelope actor ownership before `SecurityGate::admit_command`; include the authenticated subject and a per-device/session identifier in recorded sync metadata. Add immediate revoke notification or a bounded authorization generation checked before write/relay, rather than relying solely on a timer.

This is independent of the loader and P2-C work and must precede meaningful external E2E claims.

### 2. Repair the opt-in E2E into a neutral two-user oracle

In `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`, change the stale surface assertion to document-wide presence. Replace `mintSession(email)` in the scenario with a test identity adapter that produces two non-forgeable principals. Split the monolithic scenario into bounded cases so failures identify one state transition.

Add a language-neutral scenario fixture defining actor identities, scopes, commands, expected frontier/checkpoint IDs, close codes, visibility, and postcondition per step. Use Node `crypto` and AJV as independent identity/checkpoint/schema oracles; use a second raw WebSocket client implementation for the peer that races the first. Do not make the hub serializer its own only oracle.

### 3. Compose trusted checkpoint publication before recovery testing

At hub startup, construct the validated canonical artifact authority, `DbImmutableArtifactBlobStore`, `HubVerifiedCheckpointPublisher`, and `CheckpointPublicationOrchestrator`; expose a bounded, cancellable system job that creates a checkpoint after a stable document frontier. Publish only after exact blob readback and descriptor digest verification. Ensure the live directory event contains the public shape only, never private locators.

This depends on the loader/catalog authority and its byte/hash association. It also remains blocked for the advertised 64 MiB pair envelope until the 496 KiB durable blob ceiling is coherently solved by chunk-manifest CAS or a cross-backend payload redesign; do not let a small fixture imply general checkpoint support.

### 4. Finish reconnect and forced-lag recovery

Define the client rebootstrap protocol: receive verified control, fetch/verify the scope-bound pair via an authenticated endpoint, apply the required tail frontier, and issue a new Hello. Preserve cancellation, 15-second rebootstrap deadline, 4 KiB chunks, 16,384 chunks, and 64 MiB aggregate budgets from `🌎️hub/🛰️lag-rebootstrap/🦀️.rs:11-12,219-270`.

Add deterministic hub-test fanout controls rather than relying on network timing: overflow the 256 document channel and 1024 directory channel, assert control then close 1013, reconnect through the verified checkpoint, and prove no private locator/other-space data is emitted. This layer can progress beside loader work but cannot make a passing recovery claim until step 3 produces a real checkpoint.

### 5. Close revocation, kick, and deletion lifecycle gaps

Add a directory port operation that enumerates/revokes all active and stored auth sessions for a user, atomically invalidates them, and wakes every associated live connection. Keep connection kick as a separate non-revocation action. For share/member removal/delete, test both already-open sockets and subsequent REST/WS attempts; guarantee the client sees a clear authorization close/error and cannot continue writes/telemetry.

Specify and implement space-delete retention across directory projections, DB WAL/document data, authority blobs/chunk manifests, and backups. Until an erasure policy is landed, call the operation access revocation/tombstoning rather than secure deletion.

## Runtime prerequisites and focused verification

The default E2E is local and does **not** require Docker: default Rust features select bundled SQLite for the directory and FS storage for DB (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:21-29`; `bin.rs:1908-1963`). It needs Bun dependencies, a usable Rust/Cargo toolchain, a successfully built `target/debug/os-hub`, loopback TCP, and writable temporary/`OS_HUB_DATA` storage. The harness uses a newly selected port, so a bind race remains possible (`🟦️.ts:20-37`).

Postgres requires a build with the `postgres` feature plus reachable `OS_HUB_DATABASE_URL` and `OS_HUB_DIRECTORY_DATABASE_URL`; Neo4j similarly requires its feature plus `OS_HUB_NEO4J_URI`, `OS_HUB_DIRECTORY_NEO4J_URI`, user, and password (`bin.rs:1924-1936,1965-1979`). No hub Docker Compose definition was found in the current source census. Docker/container availability was not probed in this audit, so external-backend E2E remains an operator-provisioned prerequisite, not a verified path.

The only current focused Nx commands are the TypeScript harness commands below. Run the real-process command only when shared Cargo work is idle: it does a default-feature Cargo build before Vitest.

```sh
# Fast fixture/schema contract; real E2E stays skipped.
bun nx run os-hub-ts:test --skip-nx-cache --verbose

# Current default local FS + SQLite real-process scenario; builds default-feature hub first.
HUB_E2E=1 bun nx run os-hub-ts:test --skip-nx-cache --verbose --testNamePattern='boots the real hub'
```

Do **not** use `bun nx run os-hub:test` as a focused Rust command today: the real project name is `os-hub`, but its current `📜️script.ts:20-28` unconditionally requests `--all-features`, and its own comment says PostgreSQL tests need a live Docker daemon. Add a default-feature selected-test mode to that existing script before proposing Rust commands such as `bun nx run os-hub:test-quick --skip-nx-cache --verbose -- document_wide_presence`. No command in this report was executed here.

## Honest exit criteria

The journey is complete only when two independently authenticated, non-forgeable users in one private space can both open a descriptor-matched document; see a document-wide roster; submit concurrent accepted/rejected operations with a single convergent durable frontier; publish a verified checkpoint; reconnect after a short disconnect; recover from deterministic lag using that checkpoint and tail; survive restart; observe share/member/session revocation and admin kick with correct distinct semantics; and delete the space under an explicit access-retention/erasure contract. The neutral oracle must show that a third user, a revoked user, a share holder, and a same-document-ID user in another space receive neither content nor telemetry beyond their authorized contract.
