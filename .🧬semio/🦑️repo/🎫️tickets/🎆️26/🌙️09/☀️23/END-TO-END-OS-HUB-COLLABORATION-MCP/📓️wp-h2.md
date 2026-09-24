# WP-H2 — Hub Backend: One Document Socket, Green Suite, DB + Presence Proof

Slice: H2 (session 10). Ports: 7810–7819. Private cargo target: `.tmp-ticket/wp-h2/target`.
Latest good `os-hub` binary: `/Users/ueli/Documents/semio/.tmp-ticket/wp-h2/bin/os-hub` (debug; sqlite,postgres,neo4j,native-artifact-execution; staged 2026-09-24 00:16, includes envelope re-key)

## Status

| Item | Status |
|------|--------|
| 1. Inherit C4d/H1b/O3b/C4c/H1/audit | DONE |
| 2. Reconcile document socket design | DONE — hub route sole owner; framework authority deleted |
| 3. Gates | **GREEN**: check; hub quick 327/327; hub long 335/335; framework server 88+5+3; gateway 26; kernel sync 61/61; TS parity 7 + 9; server TS 14/14; hub TS typecheck 0; root exit code fixed |
| 4. DB | sqlite live e2e **PASS**; Postgres + Neo4j **BLOCKED (root-caused)**: db backends have no WAL writer fence |
| 5. Presence live | **PASS** (sqlite e2e): join with hub label + ui, roster replay to a late joiner, leave drops the peer, 15 s lease expiry strips A to identity while its socket stays open |

## Design decision

**One route, one owner: hub.** `GET /scopes/{space}%2F{doc}/document/ws?surface=` is mounted once, on hub's
own router (`document_ws_v1` → `handle_ws`). The framework `HubDocumentAuthority` is deleted.

Why hub and not the framework port: only hub's handler carries grant admission (open-plan → socket-grant
ledger, consumed once), membership/role → `SecurityGate` (spectators read-only), descriptor schema-hash check,
per-frame live authority revalidation (revoke closes 4401), presence leases (TTL, join replay, leave/expire
fan-out, hub palette colors), admin kick + directory connection records, lag → rebootstrap. The framework
path had none of those: any authenticated session could write any document, presence never left, colors
were `digest % 7`. Moving that machinery behind the byte-frame port would mean moving `HubState` into the lib.

Wire contract (sent to c7, g4):
- Before the upgrade: `POST /spaces/{s}/documents/{d}/open-plan`, then `POST …/socket-grants` (Bearer session or share).
- Upgrade: `Sec-WebSocket-Protocol: semio.session.v1, <session-or-share token>`; server answers `semio.session.v1`.
- Query: only `surface` (`deny_unknown_fields` → `?actor=` is 400).
- Actor: the pending plan grant issued to the credential's binding (session id / share id), oldest first.
  Session actor = `sha256("semio/hub/socket/actor/v1\0" "session" session_digest)` (stable across plans);
  share actor = per-plan random (two viewers of one link are two presences). Receipt `actorId` == `Session.actor`.
- Client-first `SocketHelloV1`; schema + pack hash must equal the durable descriptor.

Framework side: `ServerBuilder` mounts `/scopes/{scope}/document/ws` only when a `DocumentAuthority` is
registered; `HubInstance::Documents = NoDocumentAuthority`; hub's session resolver no longer sets `Resolved.actor`.

## Evidence

| Command | Result | Capture |
|---|---|---|
| `cargo check -p semio-hub --all-targets [--features integration-fixtures]` | EXIT 0 (hub warnings present = typechecked) | `check1..4.txt` |
| hub `test quick` (first, before fixes) | 282/287 run, 5 FAIL, EXIT 100 (exit code propagates) | `test-quick-1.txt` |
| hub `test quick --no-fail-fast` (after presence fixture regen) | **327/327 passed**, 9 skipped, EXIT 0 | `test-quick-2.txt` |
| hub `test quick` after envelope re-key + fixture law | 326/327 (fixture law expectation) → after fix **327/327** EXIT 0 | `build-and-quick-3.txt`, `test-quick-4.txt` |
| `presence-normalization-check source` (TS independent oracle) | 19 vectors PASS | `presence-normalization-source.txt` |
| `cargo build -p semio-hub --bin os-hub --features sqlite,postgres,neo4j,native-artifact-execution` | EXIT 0 | `build-os-hub.txt` |
| `docker version` | 29.5.3 (daemon answers) | `docker-version.txt` |
| two-client e2e `OS_HUB_STORAGE_BACKEND=sqlite` port 7811 | **2/2 PASS** — open, Commands→Ack Accepted + relay to B, presence A→B (label+ui), late joiner Welcome ≥ ack frontier + roster replay of A, leave drops joiner, 15 s lease expiry strips A to identity (socket open), B rejoin w/ resume, hub restart keeps artifact+frontier | `two-client-sqlite-1.txt` |
| same, `postgres` (docker postgres:16, 7815) | FAIL: `unavailable: Postgres has no mounted session-scoped WAL writer fence` | `two-client-postgres-1.txt` |
| same, `neo4j` (docker neo4j:5, 7816) | FAIL: `unavailable: Neo4j has no mounted session-scoped WAL writer fence` | `two-client-neo4j-1.txt` |
| hub `test long --no-fail-fast` | **335/335 passed**, 1 skipped, EXIT 0 | `test-long-1.txt` |
| final chain: rebuild + hub `test quick` | build EXIT 0; **327/327**, EXIT 0 | `final-chain.txt` |
| `cargo test -p semio-framework-server` (lib + closed_ports + wire) | **88 + 5 + 3** passed | `framework-server-all.txt` |
| `cargo test -p semio-framework-server --lib gateway::` | **26** passed | (console) |
| `cargo test -p semio-framework-os-kernel --lib --features sync,ureq sync` | 60/61 at 23:5x (g4 root-caused: law never registered `demo/v1` codec, fixed by g4) → **61/61** | `kernel-sync.txt`, `kernel-sync-2.txt` |
| OS TS `test long -t "backbone parity"` | **7** passed | `ts-parity.txt` |
| OS TS `test long -t "handleHubFrame\|rebootstrap\|outbox\|socket actor"` | **9** passed | `ts-neighbour.txt` |
| server TS `test` | 13/14 (route-order law) → **14/14** | `ts-server.txt` |
| OS TS `test-quick -t "browser document open"` | 4/5 (stale grant-protocol expectation) → **5/5** | `os-browser-document-open.txt` |
| hub `open-plan-server-check` | PASS | `open-plan-server-check.summary.txt` |
| hub `os-hub-ts typecheck` | 4 errors (undefined `spaceId`/`documentId`, `Uint8Array.toString(arg)`) → **EXIT 0** | `hub-ts-typecheck.txt` |
| sqlite two-client e2e on the final binary | **2/2 PASS** | `two-client-sqlite-final.txt` |
| root `bun ./📜️script.ts test quick` (direct) | now EXIT 1 with "run `bun nx run workspace:test-quick`" (was a vacuous EXIT 0) | `root-test-quick-direct.txt` |

### Root causes fixed along the way
1. **Presence fixture drift** — a peer added `PresenceWindowView.ray_origin` (+ `active_tool`, bit 12) to the Rust+TS presence codecs and regenerated replication fixtures, but not `🌎️hub/🧫️fixtures/🪪️presence-normalization-v1`; 5 quick laws (incl. the H1-era `admin_removal_revokes…`) failed decoding vector 0. Re-encoded the 14 hex vectors (one `00` per view) and taught the hub TS independent oracle `rayOrigin` + `activeTool`.
2. **Hub socket never re-keyed envelopes** — `handle_client_frame` passed wire `document_id` to the db, which rejected every real client batch (`envelope targets document artifact-… but this actor owns v1:…`); bin-unit laws had masked it by sending the internal key. Now: foreign document → Rejected ack; wire id → db key on the way in; relayed `Commands` envelopes projected back to the wire id. Laws send wire ids and assert the relay's wire id.
3. **Root `test quick` exit code** — invoked directly it runs no project (tests are Nx prerequisites of `workspace:test-quick`) and printed success. Now refuses outside Nx (`NX_TASK_TARGET_PROJECT`); `os-hub:test-quick` itself exits 100 on cargo failure (measured).
5. **Stale contracts on the old grant-on-socket design**: the browser document-open fixture and schema (`semio.socket.v1` + grant, `?actor=`), the matching hub TS oracles and the OS worker law, the framework `closed_ports` test (C4d port arity) and the server route-table fixture. All now say `semio.session.v1` + credential, with the conditional document route listed last.
4. **Two-client harness** never ran under vitest: `pick` matched `node_modules`, `Bun.sleep` under node, wrong HLC shape, `Applied` matched rejections, `?actor=`, `[DEBUG]` leftovers.

### Blocker (user/owner decision)
Postgres + Neo4j document storage: `🛢️db/🗄️storage/🐘️postgres/🦀️.rs:702` and `🌐️neo4j/🦀️.rs:861` answer `WalWriterAcquire` with `Unavailable` unconditionally — no document can be opened on those backends. Needs a cross-process writer fence (advisory lock / lease node) modelled on sqlite's `WalWriterTable`; filed as a task chip.

## Files changed

- `🌎️hub/🏗️bootstrap/🦀️.rs` — route mounted; `document_ws_v1` credential (session|share) admission; `authenticate_document_credential`; `socket_actor_id` formula owned here; `pending_document_binding`; `issue_socket_grant(actor_id)`; `compose_hub_server(data_dir, directory)`.
- `🌎️hub/📄️documents/` — deleted (authority, unit tests, fixture).
- `🌎️hub/📦️packages/🦀️rust/🦀️.rs` — module removed.
- `🌎️hub/🗄️stores/🦀️.rs` + unit test — `Documents = NoDocumentAuthority`, resolver `actor: None`.
- `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs` — conditional document route + docs.
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — document sockets on session protocol; `?actor=` 400 + ungranted-session 401 asserts; new share-holder socket law.
- `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts` + `🦀️.rs` + fixture + schema `two-client-document-v1` — presence/late-joiner/leave/expiry steps, harness fixes.
- `🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json` — re-encoded for `ray_origin`.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — presence independent oracle `rayOrigin`/`activeTool`.
- `📜️script.ts` (root) — `TestScript` refuses direct level runs outside Nx.
- `🧰️framework/🛍️products/🖥️server/🧪️tests/🔒️closed-ports/🦀️.rs`, `🧫️fixtures/🔌️wire/🔣️.json`, `🟦️.ts` (`SERVER_ROUTES`) — port arity; conditional document route last.
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json`, `🔨️modules/📇️directory/🧬️schema/🔣️.json` (`BrowserDocumentOpenTransportV1.expected.protocol`), `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` — session protocol, no `?actor=`.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — browser document-open oracles on session protocol; typecheck fixes.

## Peers told
- c7, g4: final wire contract + binary path. w1: latest good binary (no release build needed). r1: kernel nextest failures likely codec-registration isolation (g4 owns sync).


## Late fix (after g4 report, 01:00–01:40)

- **g4's symptom:** on :7830 the agent's edits were relayed to the human, but head_seq stayed at 1.
- **Root cause (measured):** the agent re-sends `mutation_id` `edit-2bb425fa800a3b7f`, which is the document's first committed edit (`wp-h2/generated/agent-edit-probe.txt`). `db_artifact::submit` dedupes whole batches by command_id and returns the cached receipt, so the Ack is Accepted but nothing is written.
- **Agent side (g4):** the guest mints the same edit id in every process (`Store::edit_id` hashes a constant actor with a non-advancing in-guest clock). g4 wrote it up for main to route.
- **Hub side (mine):** `submit_commands` reads commit_seq under the write gate before submitting. A receipt that doesn't advance it is still acked but never relayed.
- **Law:** in `socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live`, commit then resend after reconnect gives an Accepted ack, no relay to the peer (checked with a Preview fence) and commit_seq stays 1. The same-socket resend is refused earlier by the per-socket ReplayGuard. The reconnect and late socket first receive the catch-up Commands before Session. Law run on its own: **1 passed** (`idempotent-law.txt`).
- **Full hub quick on the tree at 01:40:** 326/327. The one failure is `document_open_plan_issue_route…` expecting `semio.hub.socket-grant/v1` but getting `semio.hub.document-socket-grant/v1`. That comes from H3's in-progress receipt-schema split, not from me (`idempotent-chain.txt`). Handoff: semio-hub now belongs to H3.
- **Binary:** the published "latest good" binary stays the 00:16 one. The 01:40 build in `wp-h2/target` includes H3's partial tree, so I did not stage it.

## Gaps

1. **Postgres/Neo4j document storage**: blocked, root cause above (no WAL writer fence); task chip filed. Docker answers (29.5.3). The containers I started (`h2-pg`, `h2-neo4j`) are stopped.
2. **`browser-document-open-check` runtime step** fails before the hub: Vite bundles `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` into `🧰️framework/node_modules/.vite-temp/`. Its runtime-joined relative `import(['../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite','🟦️.ts'].join('/'))` then resolves from the temp dir ("Cannot find module"). This is os-dev builder code, not changed by me. Its neutral oracle, OS law and server laws pass separately.
3. **Document socket-grant receipt still mints an unused secret**: `SocketGrantReceiptV1.grant` and `protocol: semio.socket.v1` are only meaningful for directory sockets. Document sockets are admitted by credential + binding. A later cleanup could give document exchanges a receipt without a secret; this is a schema change across hub, OS and the TS parser.
4. **`[DEBUG]`-prefixed receipts** are load-bearing in hub `📜️script.ts` (e.g. `[DEBUG] native-catalog-payload=`) and in bin-unit law output. This is pre-existing debt and I left it alone.
