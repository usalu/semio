# Invite Redemption Cross-Backend Transaction Packet

## Verdict

**RED — the current `invite.redeemed` event does not consume its bearer capability.** `DirectoryService::redeem_invite` correctly holds its in-process writer through durable append and publication, but it only reads `InviteRecord.accepted_at`; it never writes it. Every current backend consequently keeps the same valid token eligible after a successful event and membership projection.

This packet is deliberately limited to invite redemption. Presence, scoped socket revocation, and directory event-page work are out of scope.

No build, native test, database integration test, or process test was run for this audit.

## Current-source evidence and corrections

- The outdated claim that redemption releases the service writer before append is false. [`DirectoryService::redeem_invite`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1716>) takes `write`, and [`append_and_publish_locked`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1604>) appends before it publishes. That lock is useful single-process ordering, but is not a cross-process/database claim.
- `InviteRecord` has `accepted_at` but no event linkage at [directory model](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:226>). All three `authenticate_invite` implementations deliberately filter accepted rows, so a field that is never stamped makes redemption replayable: [SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1209>), [PostgreSQL](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1314>), and [Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1127>).
- Current redemption creates a generic `NewDirectoryEvent` from a pre-transaction lookup at [lines 1718–1735](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1718>). Its `InviteRedeemed` projectors only upsert membership: [SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:654>), [PostgreSQL](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:653>), and [Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:395>). A projector must not become the live claim; it has no conditional-change result.
- `HubDirectory` exposes lookup/issue/revoke and generic `append_events`, but no closed claim-and-append operation at [lines 2099–2152](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2099>). The three generic append implementations reject only `ArtifactCheckpointPublished`, not `InviteRedeemed`: [SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1631>), [PostgreSQL](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1792>), [Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1642>). A generic caller could therefore still manufacture a membership-changing redemption without consuming an invite.
- The wire event is already sufficient: [`DirectoryEventBody::InviteRedeemed`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:170>) contains the server-derived space, user, invite id, and role. No public event-schema expansion is warranted.
- The protected HTTP route is correctly thin at [`post_redeem_invite`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3824>): it resolves the bearer user and passes a parsed capability. It can retain its `DirectoryJson<Vec<DirectoryEvent>>` response shape.
- Existing native coverage is one serial happy path, [`invite_create_redeem_revoke_round_trip`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4234>). It does not retry, race, inject failure, reopen, or rebuild.

## Closed common contract

Add one backend-only claim operation next to the invite methods in [`HubDirectory`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2099>), delegated by `HubDirectories` beside its existing invite match arms at [lines 2764–2805](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2764>):

```text
redeem_invite_atomically(
  actor: DirectoryActor,
  authenticated_user_id: &str,
  capability: &InviteCapability,
  hlc: Hlc,
) -> DirectoryResult<InviteRedemptionCommit>

InviteRedemptionCommit =
  NewlyCommitted { event: DirectoryEvent }
| AlreadyCommitted { event: DirectoryEvent }
```

This is a private server authority, not a client command. It accepts neither caller-selected `space_id`, `role`, `invite_id`, `event_id`, nor acceptance timestamp.

### Minimal durable data

Retain the existing `accepted_at`, and add only:

```text
accepted_event_id: Option<String>
```

to `InviteRecord`, `prepare_invite`, the SQLite/PostgreSQL row types and selects, and `SpaceInvite` node mapping. Do **not** duplicate `accepted_user_id`: the linked, verified `InviteRedeemed` event already has the user. The invariant is:

```text
accepted_at is Some(t) <=> accepted_event_id is Some(e)
event(e).recorded_at_ms == t
event(e).body == InviteRedeemed { invite_id: row.id, space_id: row.space_id,
                                  role: row.role, user_id: accepted user }
```

The database row is direct credential decision state, like issue/revoke today; the event is durable outcome evidence. Use an application-enforced invariant rather than a cross-backend foreign key: the claim must stamp its marker before it inserts the event in its one transaction, and Neo4j has no equivalent relational constraint. A corrupted marker with no matching event is `DirectoryError::Backend`, never an opportunity to mint a replacement event.

### One authoritative sequence

While the service writer is held, `redeem_invite` must first require `actor_user_id(&actor) == user_id` (the parser exists at [directory lines 1318–1322](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1318>)), take exactly one HLC tick, and call the new operation. Remove its `authenticate_invite`, expiry/revocation, and `get_user` preflight: each is a read/check/use race and `authenticate_invite` cannot observe a same-user idempotent retry.

The operation runs one short backend transaction:

1. Read the selector row **inside that transaction** and compare the stored and candidate digest with the existing constant-time helper. A missing selector or mismatched digest is `Unauthorized`; no detail is leaked.
2. Validate the marker pair. If it is present, load the event by `accepted_event_id` within the same transaction; verify every invariant above. Return `AlreadyCommitted` only if the event user equals the authenticated user. A different authenticated user gets the generic `Conflict` result. This check deliberately precedes expiry: a lost response remains idempotent even if the capability expires later.
3. If not accepted, ensure the invite is unrevoked, unexpired against one backend-captured server timestamp, and its user and space still exist. The backend must derive `id`, `space_id`, and `role` only from this row.
4. Generate one event id and one `recorded_at_ms` inside the transaction. Conditionally set both acceptance fields only while the record remains unaccepted, unrevoked, and unexpired; require exactly one changed row. `accepted_at` is this exact `recorded_at_ms`.
5. Insert one `DirectoryEvent` with the passed HLC and server-derived body, then run the existing `InviteRedeemed` membership projector in the **same transaction**. The persist helper needs an internal explicit `(id, recorded_at_ms)` variant so it cannot call `now_ms()` again ([SQLite currently mints both internally at lines 432–442](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:432>)).
6. Commit and return `NewlyCommitted`. Any error/cancellation before commit rolls back marker, event, membership, and audit state together. There is no progress stream for this short transaction. A lost client response after commit retries to `AlreadyCommitted`.

`DirectoryService` publishes only `NewlyCommitted.event` through its existing locked publisher. It returns the same one-event vector for `AlreadyCommitted` without appending or broadcasting again; that preserves the current HTTP response shape and gives a retry the original immutable result.

Update all three `append_events` guards to reject `InviteRedeemed` alongside `ArtifactCheckpointPublished`. The special operation is then the only write path that can create this event. Add a generic-append denial law; no compatibility path remains.

`revoke_invite_as` must likewise condition on both `revoked_at IS NULL` **and** `accepted_at IS NULL` in all backends. If redemption wins, later revoke returns `Conflict` and writes no successful revocation audit; if revoke wins, redemption sees the revoked state and emits nothing. This is necessary for a coherent one-way decision even across service processes.

## Backend packets

| Backend | Current transaction authority | Exact claim implementation | Required persistent/rebuild work |
| --- | --- | --- | --- |
| SQLite | [`append_events`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1631>) uses `TransactionBehavior::Immediate`. | Add a private transaction helper in the invite region. Read/constant-time check, conditional `UPDATE hub_space_invite ... accepted_at, accepted_event_id ... RETURNING id, space_id, role`, persist with a supplied id/time, project, commit. The immediate writer linearizes separate SQLite handles. | Extend schema at [lines 157–168](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:157>), issue/select/list/row mapper, and the temporary credential snapshot/restore at [lines 1671–1760](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1671>). |
| PostgreSQL | [`append_events`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1792>) begins one SQL transaction, advances the dense head, inserts, projects, then commits. | In that transaction, fetch the selector row with a lock for constant-time digest validation; on a fresh row use conditional `UPDATE ... RETURNING` for the acceptance marker, then the existing dense-head/event/project sequence. For retry, fetch event by the stored id and verify it. | Extend table/issue/query/`InviteRow` at [schema](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:156>) and [row mapper](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2003>). **Also add a temporary invite snapshot and restore to `rebuild_projections_controlled`: current rebuild deletes `hub_space` at [lines 1872–1886](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1872>), whose FK cascade removes all direct invite state, and never restores it.** |
| Neo4j | [`append_events`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1642>) uses one `Txn` for counter, event node, and projection. | In the same `Txn`, read `SpaceInvite` by selector for constant-time digest check. For fresh redemption use a conditional write match requiring `acceptedAt/acceptedEventId` absent, `revokedAt` absent, valid expiry, and matching live `:User`/`:Space`; set both markers and return the invite fields. Then create event node and project before `Txn::commit`. Query the exact event node by marker for retry verification. | Add `acceptedEventId` in issue/node mapping at [issue/auth](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1106>) and [mapper](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1853>). Active-space invite nodes survive the current rebuild; historic `SpaceDeleted` already deletes their nodes at [lines 367–375](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:367>). Test that actual policy rather than claiming full issue/revoke replay. |

The application must handcraft all three schemas and queries together; this greenfield repository has no migration/compatibility obligation.

## Source ownership and failure boundaries

1. **Shared directory authority — `🌎️hub/📇️directory/🦀️.rs`:** `InviteRecord`, `prepare_invite`, new closed request/outcome, `HubDirectory`, enum delegation, and `DirectoryService::redeem_invite`. The existing event contract remains unchanged.
2. **Each backend — SQLite/PostgreSQL/Neo4j files above:** schema, explicit claim helper, event-by-id verification helper, revocation predicate, row/node conversion, and generic append denial. Do not factor driver transactions into a fake common transaction abstraction.
3. **Hub route — `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3824`:** no new bearer, token, idempotency, or event wire field. It maps `Unauthorized`/`Conflict` with the existing [status mapping](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3459>) and returns the original event on a same-user retry.
4. **Projection/replay:** leave `InviteRedeemed` projector membership-only. It replays a committed event; it must not stamp an acceptance marker or create an invite. Preserve direct issue/revoke/accept decision records across rebuilds as noted above. This is the current bounded hybrid model, not a claim that invite issue/revoke are fully event sourced.

There are no awaits after the conditional mutation except the backend's own database calls within its transaction. No service broadcast, socket send, directory watch, or HTTP response occurs before commit. The writer lock only surrounds the whole call/publish to retain local event order; correctness must hold with two service instances pointed at the same backend.

## Language-neutral contract

Create a schema-first `invite-redemption-transaction-v1` fixture and JSON schema under the existing hub directory fixture taxonomy, with a first-party TypeScript oracle in [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts). It models rows, one append log, and one membership projection. It must use selectors and digests only—never output a raw capability.

Each row records logical server time, actor/user, pre-state, injected failure boundary, calls, and expected `(outcome, event identity, marker, membership, broadcast count)`.

Required rows:

1. normal claim: one marker, one event, one member; `accepted_at == event.recorded_at`;
2. same-user retry after committed response loss: identical immutable event, no new seq/id/marker or broadcast;
3. two concurrent authenticated users on one capability: exactly one winner/event/member; the other is conflict;
4. two concurrent same-user calls: one new and one existing result, exactly one event/broadcast;
5. wrong selector or digest: unauthorized and no observation of marker/user/role;
6. expired and revoked capability: no marker/event/member;
7. revoked-versus-redeem race in both orderings: exactly one terminal decision, neither dual audit nor event after revocation;
8. actor/user mismatch, missing user, and deleted/missing space: denial with no write;
9. fault after marker C.A.S., after event insert, and after projection but before commit: rollback of all state;
10. corrupt marker/event mismatch: backend error, never a repaired second event;
11. restart then same-user retry: existing original event; rebuild then the same result and membership;
12. direct generic `append_events(InviteRedeemed)` rejection.

AJV validates the schema; the TypeScript state machine independently evaluates the outcome. It must not call the Rust backend or reuse the backend query strings.

## Native and process proof matrix

| Layer | Proposed exact law / target | Required observation |
| --- | --- | --- |
| Shared SQLite native | `directory::tests::invite_redemption_claim_is_single_use_and_same_user_idempotent` | two independent service instances/handles, barrier race, one event and exact original retry result; actor mismatch and generic append denial. |
| SQLite transaction/replay native | `directory::sqlite::tests::invite_redemption_claim_rolls_back_and_rebuild_preserves_marker` | inject each pre-commit failure; reopen persistent SQLite; `rebuild_projections`; marker/event/membership invariants stay exact. |
| PostgreSQL native integration | `directory::postgres::tests::invite_redemption_claim_matches_neutral_contract` | configured live/testcontainer Postgres race, rollback, same-user retry, and the newly required direct-invite rebuild preservation. It may not be replaced by a source assertion. |
| Neo4j native integration | `directory::neo4j::tests::invite_redemption_claim_matches_neutral_contract` | configured Neo4j transaction race, existing-node retry verification, missing live `User`/`Space`, rollback, restart/rebuild policy. The module currently documents that it has no in-memory mode at [lines 1989–1995](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1989>). |
| Service publication native | extend `directory::tests::directory_append_and_live_broadcast_share_one_writer_guard_and_projection_order` | new result publishes after durable membership; existing result returns event but sends no second live event. |
| SQLite process | new protected-hub two-user fixture | two real authenticated sessions concurrently POST one token, restart against same SQLite file, then recover one ordered event; assert one 2xx winner/new membership and an idempotent result for the same-user retry. |

For the new generic-append law, prove all three backends reject it before they allocate a sequence. For the race laws, use a deterministic test-only barrier at the database-transaction seam—not a service mutex—so the test would fail if two hub processes race.

## Nx and launch registration

Extend the existing pattern, not a new runner:

- Add `InviteRedemptionTransactionCheckScript` to [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts) beside `DirectoryOrderedPublicationCheckScript` at [lines 4792–4824](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4792>). Its source mode runs AJV plus the independent model; native mode runs the exact SQLite laws; configured `--postgres` and `--neo4j` modes run the non-skipped backend integration laws; process mode owns the local persistent SQLite hub lifecycle.
- Add corresponding `project.json` targets beside the ordered-publication target at [lines 127–141](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:127>), each only invoking `bun ./📜️script.ts ...`.
- Register executable commands in [`.vscode/🧩️launch.seed.jsonc`](</Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc:3681>) and regenerate launch output. Never edit generated `launch.json` directly.

Expected commands after implementation:

```sh
bun nx run os-hub:invite-redemption-transaction-check --skip-nx-cache
bun nx run os-hub:invite-redemption-transaction-native-check --skip-nx-cache
bun nx run os-hub:invite-redemption-transaction-postgres-check --skip-nx-cache
bun nx run os-hub:invite-redemption-transaction-neo4j-check --skip-nx-cache
bun nx run os-hub:invite-redemption-transaction-process-check --skip-nx-cache
```

## Dependency order and nonclaims

1. Land shared marker/outcome/append-denial contract and all three physical schema/query packets atomically.
2. Add the neutral oracle and SQLite transaction/failure/rebuild laws; port the same rows to PostgreSQL and Neo4j, including PostgreSQL's missing direct-invite rebuild preservation.
3. Add service publication and route/process proof.

This does not claim that PostgreSQL or Neo4j has executed, that invite issue/revoke became event-sourced, that invite events are visible to unrelated socket scopes (they are already filtered at [`bin.rs:3965`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3965>)), or that presence/revocation work is complete. The P0 only closes one capability consumption decision: one valid invite can commit one server-derived redemption event, one membership projection, and one immutable retry result.
