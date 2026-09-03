# Sol — Admin Backend Correctness Repair

**Packet.** Session-derived administrator authority, closed typed intents, durable operation audit, bounded read projections, asynchronous rebuild ownership, browser-safe relay transport, and the admin SPA cutover. The implementation follows `📓️terra-admin-backend-correctness-implementation-audit.md`; it does not add retention/delete authority, an administrator socket stream, or legacy admin endpoints.

## Stable source boundary

- `AdminPrincipalV1` is reconstructed from one strictly parsed bearer on every request. Durable session generation, revocation, expiry, provider and constant-time subject-digest checks precede dispatch. The server alone derives `user:{userId}#admin-session:{authSessionId}`; neither the JSON body nor a legacy connection actor can nominate identity.
- `AdminIntentV1` is a closed taxonomy. It contains only the implemented bounded directory operations, document-share issue/revoke, durable user-session revoke, ephemeral connection kick, and projection rebuild. Unknown fields and arbitrary embedded `DirectoryCommand` values are rejected. The decoded request ceiling is 8 KiB.
- Accepted and exactly one first-reason terminal operation fact are append-only in SQLite, PostgreSQL and Neo4j. `intent_digest` participates in request collision discrimination. SQLite/PostgreSQL uniqueness conflicts from concurrent first writers are caught and the established receipt is reread; this does not depend on a process mutex.
- Credential-ledger facts for invite and document-share issue/revoke retain the verified principal actor. Durable revoke completes before best-effort socket signalling. A kick is explicitly ephemeral and never changes an authorization generation.
- Create-space uses a request-stable server-derived space id, and both accepted and terminal audit target that exact created resource. The event owner remains the verified principal.
- Rebuild admission uses one atomic semaphore with capacity 64 for distinct asynchronous requests. A task-owned terminal guard releases capacity and appends a first-reason failed/cancelled fact on abort or unwind. Progress is monotonic and status/cancel are exposed through typed operation routes.
- Admin overview counts and spaces, members, users, documents, recorded connections, events, credential audit, and operation audit are source-bounded projections. Page limits are 1–100, backend reads use at most 101 rows, cursors are principal/route/scope MAC-bound, and successful JSON emission is trimmed to at most 65,536 bytes with a continuation. One indivisible oversized row fails with 413.
- Space detail is no longer a full read-model/N+1 response. `GET /admin/api/spaces/{id}?limit=…&cursor=…` returns the bounded `SpaceView` summary and a bounded `AdminPageV1<MemberView>` only. It makes no document or invite-array claim; those are independent projections.
- Connection snapshots expose recorded binding fields only. Legacy caller-asserted actor, label, surface and presence are absent. The SPA does not open the ordinary member-filtered directory stream.
- The SPA calls only `/admin/api/intents` and bounded page/status routes, consumes and accumulates space/member continuations explicitly, preserves the last successful connection snapshot on a failed poll, and labels it stale. Rebuild progress/cancel and durable-revoke versus ephemeral-kick outcomes remain distinct.
- The local admin relay admits only exact typed paths and bounded query grammars. It enforces same-origin cookie/CSRF authority, an 8-KiB intent body ceiling, a 64-KiB API response ceiling, a 1-MiB static-asset response ceiling, one aggregate 64-request admission boundary, downstream-abort propagation, and stop-time cancellation of every owned API or static upstream request.

## Neutral contract and independent oracle

The language-neutral contract is owned under `🌎️hub/📇️directory/🧪️fixtures/🧬️admin-intent-v1/`. Rust, TypeScript and JSON Schema use the same closed intent vocabulary. The independent Bun/Node oracle uses AJV plus its own state-machine checks and redaction assertions.

Fresh result on 2026-09-03:

```text
bun 🌎️hub/📇️directory/🧪️fixtures/🧬️admin-intent-v1/🧪️oracle/🟦️.ts
admin-intent-v1 oracle: 5/5; invalid inventory 22/22
exit 0
```

## Permanent gate ownership

`os-hub:admin-backend-check` is registered in the hub Rust `📋️project.json`, implemented by the existing `📜️script.ts`, and present in `.vscode/🧩️launch.seed.jsonc` as `⚖️gate🛡️admin-backend🌎️hub`. It preflights `cargo test -- --list`, requires exactly one fully qualified match for every Rust law, runs those names with `--exact`, runs the independent oracle and relay proof, runs the focused SPA suite, and ends with the all-feature hub-binary check.

The six portable Rust laws cover:

1. SQLite concurrent first-writer idempotency and first-terminal-wins;
2. bounded overview/space/member/document projection boundaries;
3. closed wire-intent taxonomy;
4. principal/route/scope cursor binding and exact page bounds;
5. 65,536-byte page fitting and indivisible max+1 rejection; and
6. atomic rebuild slots plus abort terminal cleanup.

PostgreSQL absent-request race recovery is a seventh, mandatory container-backed law and runs last. Neo4j has compile-enforced implementation parity and deterministic source laws but no equivalent container concurrency run in this packet; no all-backend runtime-parity claim is made.

## Fresh runtime evidence

```text
bun nx run os-hub:admin-relay-check --skip-nx-cache
Test Files  1 passed (1)
Tests       15 passed (15)
admin-relay-check: ... laws passed
NX Successfully ran target admin-relay-check for project os-hub
exit 0
```

This registered run included the exact bounded spaces, space-detail and document routes; both collection and expanded-member continuation UI laws; 8-KiB request, 64-KiB API-response and 1-MiB static-response exact/max+1 probes; 64 admitted concurrent requests with API and static rejection outside the shared boundary; downstream abort; stop-time cancellation; and the admin component suite. It supersedes the earlier 12- and 14-test SPA evidence.

## Qualified registered boundary

Registered session `5799` ran `bun nx run os-hub:admin-backend-check --skip-nx-cache` on 2026-09-03. It produced the following current-source boundary before the mandatory external fixture:

```text
admin-intent-v1 oracle: 5/5; invalid inventory 22/22
portable exact Rust laws: 6/6 passed
admin relay oracle: passed
admin SPA: 1 file, 15 tests passed
cargo check --all-features --bin os-hub: passed
```

The final PostgreSQL law remained terminally red because its container fixture could not connect to `unix:///Users/ueli/.docker/run/docker.sock`; the daemon socket was absent. The law was neither skipped nor weakened, and no administrator prompt was triggered. This is a qualified SQLite/source boundary, not PostgreSQL runtime acceptance. Direct isolated Cargo runs are diagnostic only; they are not counted as acceptance evidence.

The earlier session `4054` stack overflow in the cursor law was repaired by removing full `HubState` ownership from cursor helpers and the rebuild cleanup guard. Current tests use only the 32-byte cursor key or the directory/operation registries they exercise. The same registered session `5799` proves that superseding repair through all six portable laws.
