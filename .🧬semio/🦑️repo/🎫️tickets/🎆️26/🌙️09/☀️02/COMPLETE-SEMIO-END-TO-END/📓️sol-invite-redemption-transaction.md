# Invite Redemption Transaction Repair

## Boundary

This packet implements the invitation-consumption decision only. Presence and the already-staged scoped-directory socket behavior are unchanged.

One valid invitation now has one backend-owned terminal path:

- a fresh claim atomically stamps `accepted_at` and `accepted_event_id`, appends the server-derived `InviteRedeemed` event, and projects membership;
- a same-user retry verifies and returns the immutable linked event without allocating a sequence or broadcasting;
- a different-user retry returns `Conflict`;
- a corrupt marker/event pair returns `Backend`, never a replacement event;
- generic `append_events(InviteRedeemed)` is rejected before sequence allocation;
- revoke is conditional on an unaccepted invitation, so redeem and revoke cannot both commit.

The public redeem route remains thin and has no client-selected space, role, invitation id, event id, or acceptance time.

## Durable implementation

The shared directory model adds the paired event link and the closed `InviteRedemptionCommit::{NewlyCommitted,AlreadyCommitted}` result. `DirectoryService::redeem_invite` holds its writer guard through one HLC tick, the backend transaction, and publication of a newly committed event. The already-committed result is returned without a second live publication.

SQLite uses an immediate transaction. PostgreSQL locks the event head and exact invitation row, and snapshots/restores direct invitations across projection rebuild so the space foreign-key cascade cannot erase credential decision state. Neo4j fences the counter and invitation node in one transaction before marker, event, and projection writes. Every backend derives scope and role from the stored invitation.

The linked event verifier checks marker identity/time, exact invite/space/role/user body, denormalized scope/user fields, and the user actor identity before idempotent return.

## Neutral contract

The schema and corpus live at `🌎️hub/📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1`.

The independent Bun state machine covers 21 traces:

- fresh and response-loss retry;
- concurrent same-user and different-user claims;
- wrong selector/secret, actor mismatch, missing user/space, expiry, and revocation;
- rollback after marker, event, and projection stages;
- corrupt marker/event;
- restart plus rebuild;
- both redeem/revoke orderings;
- forbidden generic append.

AJV rejects six hostile shapes, including raw credentials and client-selected scope, role, and event identity. Six independent source mutations prove that removing the paired marker, retry no-broadcast branch, SQLite immediate transaction, PostgreSQL row locks/rebuild snapshot, or Neo4j write fence is detected.

## Registered ownership

`os-hub` owns these existing-script targets:

- `invite-redemption-transaction-check`
- `invite-redemption-transaction-native-check`
- `invite-redemption-transaction-postgres-check`
- `invite-redemption-transaction-neo4j-check`

The four corresponding launch entries are in the canonical launch seed at orders `411.094` through `411.097`, with native output confined to this ticket's `🗑️generated` directory.

## Evidence

- Session `34698`: `NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run os-hub:invite-redemption-transaction-check --skip-nx-cache` exited 0. AJV 1/1, neutral vectors 21/21, schema hostiles 6/6, source hostiles 6/6; 33 total checks.
- Rust parser boundary: `rustfmt --edition 2021 --check` parsed the shared directory and all three backend files. It exited 1 only because the shared files are intentionally not mechanically reformatted during concurrent edits; there was no parse diagnostic.
- Session `26737`: canonical plugin-registry generation exited 0 and generated all four launch entries exactly once.
- The following freshness run was RED because concurrent plugin catalog changes made `🔌️plugins.json` and `🧩️plugins.ts` stale after generation. The generated launch output itself contains all four entries exactly once. A final shared generation/freshness pass remains required after the catalog settles.

## Native laws staged, not credited

- SQLite: two independent service/backend handles, same-user and different-user races, one linked event, rollback on projection fault, generic append denial, restart, rebuild, and publication order.
- PostgreSQL: real disposable Postgres race, immutable retry, generic append denial, injected projector rollback, and direct-invite rebuild preservation.
- Neo4j: real disposable Neo4j race, immutable retry, generic append denial, explicit transaction rollback, and the documented active-invite rebuild policy.

No native/container/process assertion is credited. The current hub native build still stops before these laws at the externally owned Stdio BRep taxonomy include (`🌎️hub` dependency fan-in, Stdio line 6188: stale `✳️brep/.../io/🦀️.rs` while the live tree is under `🧈️brep`). This packet did not patch or weaken that owner boundary. PostgreSQL and Neo4j remain mandatory runtime RED until their registered container laws execute successfully; no process or OIDC claim is made.
