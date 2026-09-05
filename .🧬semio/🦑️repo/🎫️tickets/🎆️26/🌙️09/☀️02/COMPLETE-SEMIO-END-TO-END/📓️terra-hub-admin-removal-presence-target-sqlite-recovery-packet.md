# Hub Admin Removal, Presence, and Execution-Target SQLite Recovery Packet

Read-only audit, 2026-09-05. No native selector, server, or browser run was started. This packet identifies one missing **composed acceptance proof**, not a demonstrated source defect and not a replacement for the six pending presence laws.

## Bounded gap

There is no current native journey proving that an administrator's durable removal of a member simultaneously:

1. withdraws that member's already-visible, plan-bound normalized presence from another admitted document socket;
2. closes the removed member's live document socket and prevents its reconnect/refresh; and
3. remains authoritative after reopening the same filesystem SQLite directory, so the old valid session cannot retrieve a selected execution target or mint a fresh open plan.

This boundary crosses the actual user/admin authorization, normal document-socket lifecycle, execution-target admission, and the selected SQLite directory. It is outside the six current selected presence laws: those cover normalization, reconnect ownership, expiry, bounds, and an empty fresh in-memory Hub, but no admin-removal plus persistent-directory recovery composition ([`📜️script.ts:7303-7308`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7303>)).

## Why this is the right unverified seam

The current source has the intended pieces:

- A document subject and target read acquire the same deterministic session/membership gates. A document audience adds `Membership { user_id, space_id }` to a session record's bindings ([`🚀️bin.rs:732-745`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:732>)); target selection acquires those gates and revalidates before and after reading its selection ([`🚀️bin.rs:2437-2446`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2437>), [`:2513-2521`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2513>)).
- Both ordinary author commands and administrator `RemoveSpaceMember` reduce to `DirectoryCommand::RemoveMember` ([`🚀️bin.rs:3984-4016`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3984>), [`:5380-5391`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5380>). Its fenced execution holds that exact membership gate through durable directory execution, then invalidates pending/live socket records ([`🚀️bin.rs:4024-4068`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4024>)).
- An invalidated live grant wakes the document socket; handler cleanup conditionally removes the matching presence slot and publishes an empty roster when that row was visible ([`🚀️bin.rs:3738-3755`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3738>). The normalizer stamps the peer from the admitted plan/socket rather than the client frame ([`🚀️bin.rs:1591-1607`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1591>)).
- Production selects a durable file-backed SQLite directory by default and reseeds it on process open ([`🚀️bin.rs:6535-6556`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6535>). In contrast, the normal native `test_state` creates `SqliteDirectory::connect(":memory:")` ([`🚀️bin.rs:7068-7074`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7068>)). The only current restart proof nearby is command-receipt persistence, not Hub membership/target admission ([`🚀️bin.rs:10237-10308`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10237>)).

Existing tests deliberately stop short: the scoped-directory removal law closes a **directory** socket and rejects its old grants ([`🚀️bin.rs:9024-9093`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9024>)); the admin counterpart only observes its close ([`🚀️bin.rs:9097-9124`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9097>)); target-route laws use an in-memory `TestDocumentOpenCatalog` and do not remove a live member ([`🚀️bin.rs:8222-8353`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8222>)). None proves the cross-seam terminal state.

One precise current distinction matters to the expected result: the membership-removal fence invalidates `socket_grants` but does not eagerly call `document_open_plans.invalidate_binding`; session/share revocation paths do ([`🚀️bin.rs:4045-4068`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4045>), [`:5745-5773`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5745>)). This is not an authority bypass in the inspected exchange route: it re-authenticates B's current membership before consulting the receipt and later fences the directory revision ([`🚀️bin.rs:2363-2389`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2363>). The composite law must therefore expect an unauthorized exchange/no socket, not require the old plan record's internal state to be `Invalidated`. It does leave an issued record until expiry/replacement; treat any eager memory cleanup as a separate bounded policy choice, not as the claimed recovery result.

## Minimal native law

Add one `#[test]` adjacent to the scoped membership-removal and presence socket laws, for example:

`admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen`

It needs a small test-state constructor accepting a supplied `Arc<HubDirectories>` and fixture root; reuse the existing `test_state_with_capacity` initialization rather than creating a parallel Hub configuration. The constructor opens a file-backed `SqliteDirectory` below `tempdir("admin-presence-target-recovery")`, calls `seed`, and constructs the ordinary DB/CAS/state around it. It must preserve the current `test_state` in-memory helper for unit laws.

The current `spawn_server` intentionally returns only an address and leaves its task alive ([`🚀️bin.rs:7287-7295`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7287>)); it cannot model a clean process replacement. Add a test-only sibling that returns `(SocketAddr, shutdown_sender, JoinHandle)` and uses Axum graceful shutdown. The law must close its clients, signal shutdown, await the task, and drop the first `HubState`/directory handle **before** reopening the same SQLite path. Starting a second state while the first server still owns the first state is a concurrent-handle test, not a restart proof.

### Exact sequence

1. Seed owner/administrator A and member B in `default`; announce one document. Configure the existing verified test catalog used by execution-target/native plan tests and enable `open_plan` readiness. Give observer C independent author membership.
2. Issue and exchange real `DocumentOpenPlanV1` grants for B and C, using the same selected `surface_id`. Connect both document sockets, complete `Hello`/`Session`, and have B submit a canonical fixture peer. C must receive a decoded peer whose actor/user/role/color/surface are server-derived; this reuses the normalizer boundary, not a raw `refresh_presence` helper.
3. Before removal, issue one repeated `DocumentOpenIntentV1` target request for B and assert the manifest succeeds. This pins the journey to the protected target route, not merely a socket grant.
4. Call the real `admin_intents` `RemoveSpaceMember` route as A. Do not mutate the directory directly. Assert the canonical accepted admin receipt.
5. Drain to terminal observations, with explicit bounds:
   - B receives close code `4401`.
   - C receives exactly the empty document `ServerFrame::Presence`; decoding/`presence_snapshot` has no B row. If a directory observer is included, its member-only `DirectoryStreamMessage::Presence` contains no B actor.
   - B's existing target `manifest`, `component`, and `descriptor` requests with the same strict repeated intent each return no body and an authorization failure. B cannot issue a new plan or reconnect with the consumed grant. C remains able to refresh/publish and retrieve its own target, proving the removal is scoped to B's membership rather than the document/space globally.
6. Use the cancellable test server to close/drop the first state/server, then reopen a new `SqliteDirectory` at the exact same path. Construct a fresh Hub state with the same verified catalog/target readiness. Its RAM presence, socket-grant ledger, and plan ledger must be empty by design. The durable removal must still make B's old session return unauthorized for all target routes and new open-plan issue, while C still obtains a plan/manifest. This separates desired ephemeral restart loss from forbidden durable membership resurrection.

The law should never assert that a plan receipt or socket grant survives restart: both are explicitly server-local ledgers. The persistent assertion is only directory membership/session authority and target admission.

## Fixture and selector

No new generic identity, presence, or target schema is needed. Add one named composite vector to the existing neutral Hub fixture family (or a compact `🧪️fixtures/🛂️admin-removal-target-presence-v1` schema) containing only stable facts: scope, B/C roles, selected surface, expected close code `4401`, target denial status/code, and `sqliteReopen: true`. It must not carry capabilities, raw peer identity fields, runtime socket ids, or an expected receipt secret.

Register the exact native law in the existing selected-SQLite execution-target runner, rather than an all-features fleet command:

- [execution-target native runner](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2690>) already uses `semio-hub --bin os-hub --features sqlite`.
- Keep it separate from `presence-normalization-check native` so its receipt honestly names the composite SQLite recovery journey. A dedicated `hub-admin-presence-target-sqlite-recovery-check` target is clearer if adding it does not broaden the execution target gate.

## Qualification boundary

Passing this one native law would qualify the Hub's selected SQLite, live socket, user/admin removal, target-read and restart composition. It would not qualify browser footer projection, browser component activation/rendering, PostgreSQL/Neo4j, WGPU, durable artifact replay, or the six pending presence laws themselves. Conversely, the present source supports the expected result but no fresh run in this audit establishes it.

## Review of the landed composed law (current source, 2026-09-05)

The requested journey is now present as `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` ([`🚀️bin.rs:9154`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9154>)), with a cancellable Axum sibling and explicit first-state retirement ([`:7301`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7301>), [`:9140`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9140>)).  The lifecycle approach is sound in principle: it closes both WebSockets, awaits Axum graceful shutdown, waits for zero `HubDirectories` owners and sole database ownership, calls `Database::shutdown`, then opens a new `SqliteDirectory` at the same file path.  That is a real restart of the **file SQLite directory authority**.  `Database::open_at` is filesystem-backed, not SQLite, so the acceptance/result description must not imply that every Hub store is SQLite.

### P0: fixture space does not exist in this test state

The new fixture fixes `scope.spaceId` to `"studio"` ([`🔣️.json:3`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json:3>)), but `test_state_with_directory` calls `SqliteDirectory::seed`, whose seeded space id is `"default"` ([`🚀️bin.rs:7074`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7074>), [`sqlite/🦀️.rs:658`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:658>)).  The Hub test constant explicitly confirms `STUDIO == "default"` ([`🚀️bin.rs:6865`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6865>)).  Therefore the first `upsert_member_for_test` requires a nonexistent `studio` space and aborts before the composed journey.

Fix the fixture and its JSON-schema `const` to `"default"`, or explicitly create a `studio` space before member upsert.  The former is the bounded repair.  Also make the source oracle reject a fixture scope other than the seeded one; currently it validates the self-consistent but wrong schema and only searches for helper strings ([`📜️script.ts:7324`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7324>)), so this failure survives the inexpensive source gate and appears only after the costly native compile.

### P1: the current close assertion permits a binary authority leak

The composed law calls `next_close_code(&mut b, false)` ([`🚀️bin.rs:9215`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9215>)).  That helper rejects unexpected text when `false`, but silently skips every binary frame before the Close ([`:7705-7718`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7705>)).  Consequently a stale `Session`, `Presence`, command, or rebootstrap binary frame can leak after removal and the law still reaches 4401.

Use the existing `next_close_without_authority(&mut b)` instead ([`:7722-7733`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7722>)); it accepts only the close and rejects both binary and text.  This is the correct test-order/synchronization primitive for a terminal revocation claim.

### P1: strengthen actual membership and asset observations

The route implementation itself does the correct ordering: target selection authenticates and revalidates membership before resolving target assets ([`🚀️bin.rs:2437-2446`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2437>)), and only produces component/descriptor bytes after successful selection ([`:2540-2545`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2540>).  The law's negative loop currently parses into unconstrained `serde_json::Value` and checks only `code` ([`:9223-9227`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9223>).  Parse instead as `DocumentOpenPlanErrorV1` (which has `deny_unknown_fields`) and require the expected schema/code; for component/descriptor also require JSON content rather than `application/octet-stream`.  That makes the no-asset-byte claim observable rather than inferred from status.

After restart, add direct durable authority checks before HTTP assertions:

- B's `authenticate_session` remains `Some` (the test expects removal, not incidental token expiry/revocation), but `get_role(scope.space, B) == None`.
- C's same session remains active and `get_role(...) == Some(Author)`.
- Decode C's post-removal presence and assert its server-derived `user_id`, `role`, and surface, not only the actor.
- Assert C's reopened component and descriptor bodies equal the existing `TEST_EXECUTION_TARGET_*_BYTES`, not only HTTP 200.  This proves a current authorized selection returned real selected target bytes while B's exact paths did not.

The 3,600-second test session TTL ([`🚀️bin.rs:7762-7773`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7762>)) means expiry is unlikely during the law, but the direct checks distinguish expiry/session loss from durable membership removal and role persistence.

### P2: reconnect assertion uses an already consumed grant

`grant_b` is consumed to establish B's initial socket, then the law expects its reconnect to fail ([`🚀️bin.rs:9183`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9183>), [`:9222`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9222>)).  It would fail even if member-removal invalidation were absent because grants are single-use.  If the composed claim includes pending-grant revocation, mint a second B plan/grant before removal, leave it unused, and prove that exact capability cannot connect after removal; otherwise describe the assertion as consumed-grant replay only.

No build or source edit was performed for this review.
