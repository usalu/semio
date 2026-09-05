# Directory Event Page V1 Hub P0

## Boundary

This packet adds the authenticated hub producer for the already-shared `DirectoryEventPageV1` contract. It does not modify the client/bootstrap worker, Home, Flow, Stdio, invite redemption, presence, or the legacy `/directory/events` endpoint.

## Implemented source

- The OS directory schema exports one `validate_directory_event_page_event` primitive for safe sequence, canonical 48 KiB event bytes, and control-character rejection. `DirectoryEventPageV1::validate` delegates its per-event admission to that primitive.
- SQLite validates the complete backend-assigned event inside `persist_event_with_identity`; rejection drops the enclosing transaction. PostgreSQL and Neo4j construct and validate the complete event after their transactional sequence increment and before every event INSERT/CREATE. The generic, verified-checkpoint, and invitation-redemption append seams are covered without a compatibility path.
- `GET /directory/event-page/v1?after=<decimal>` accepts exactly one canonical safe decimal query, authenticates a retained session, reads exactly one raw page of at most 128 rows, then revalidates exact session id/user/generation/expiry-bound session digest before visibility reads or response construction.
- Invisible raw events advance `throughSeqInclusive`. Every visible or invisible cursor advance is trial-sealed into the full canonical page; a 64 KiB overflow stops before that row and leaves it for the continuation. Saturating a raw 128-row scan sets `hasMore` even if every raw row is invisible.
- Receipts are lowercase SHA-256 over the declaration-ordered unsigned page. The session binding hashes length-prefixed session/user ids, authorization generation, and expiry under the dedicated `semio/hub/directory-event-page/session-binding/v1\0` domain; bearer bytes are never hashed or returned.
- The request uses a drop-safe, disarmed-only-after-response-ownership cancellation control and a five-second server deadline. Cancellation is checked after the directory read, after the deterministic test fence, after session/membership reads, and for each raw event.
- Event-page visibility uses the centralized public/member/author access decision but retains a typed backend-error path. Infrastructure failure while resolving a space or role yields 500 rather than an authoritative empty projection.
- Five exact hub Rust laws are staged for raw holes/visibility, post-read revocation and disconnect cancellation, canonical byte-prefix continuation, transactional admission, and body-free pre-read rejection. Three backend-owned exact laws first commit and project one exact-48-KiB event, then exercise rollback of row, sequence, and projection on max+1 for SQLite, PostgreSQL, and Neo4j.
- The process phase now owns a bounded real-binary SQLite journey. It issues two independent local-bootstrap sessions, independently recomputes the session binding and page receipt, creates alternating private-space visible/hidden rows, proves the 64 KiB prefix continuation and a saturated 128-hidden raw scan, rejects a revoked bearer with an empty 401, restarts the hub on the same data directory, and proves the visible event plus raw frontier remain durable under a fresh session binding.

## Neutral oracle

The hostile fixture is in `🌎️hub/🧪️fixtures/📇️directory/event-page-route-v1`. Its independent Bun implementation uses AJV 2020 and Node `createHash`, not the Rust page builder. It covers five page vectors, twelve query/status/read-count cases (including post-read generation rotation and backend failure), five hostile classes, eight Rust/storage source-fence mutations, four real-process source-fence mutations, exact binding preimage, event max/max+1, raw 128-hidden saturation, and byte-prefix continuation.

## Permanent gates

- `os-hub:directory-event-page-v1-source-check`
- `os-hub:directory-event-page-v1-native-check`
- `os-hub:directory-event-page-v1-process-check`

All target bodies route only through the hub `📜️script.ts`. Launch seed orders `411.0996` through `411.0998` use the canonical root Nx router and ticket-local artifact/target directories.

## Evidence

| Session | Command | Result |
|---|---|---|
| `068cfa` | direct `bun ./📜️script.ts directory-event-page-v1-check source` | GREEN, 29 checks before request-lifecycle strengthening |
| `5861cd` | `bun ./📜️script.ts nx run os-hub:directory-event-page-v1-source-check --skip-nx-cache` | GREEN, registered 29-check source boundary |
| `dd4a50` | registered plugin-registry generate then check-generated | GREEN, launch bytes fresh |
| `40a373` | direct strengthened source gate | GREEN, 30 checks |
| `ee6ec8` | registered strengthened source gate | GREEN, 30 checks: AJV 1, vectors 5, queries 10, hostiles 5, source hostiles 8, SHA-256 1 |
| `f019da` | registered final hostile source gate | GREEN, 32 checks: AJV 1, vectors 5, queries 12, hostiles 5, source hostiles 8, SHA-256 1 |
| `17cc9b` | registered source gate after cross-backend law registration | GREEN, 32 checks |
| `8cc771` | registered source gate after real-process scaffolding | GREEN, 32 checks before process-fence mutation coverage |
| `313a0d` / `933eb4` | registered source gate while strengthening self-hosted process-fence mutations | RED on oracle defects only; production/process source was not executed |
| `524eae` | registered final source gate | GREEN, 36 checks: AJV 1, vectors 5, queries 12, hostiles 5, Rust/storage source hostiles 8, process source hostiles 4, SHA-256 1 |
| `6d0a40` | registered source after exact-boundary backend law strengthening plus focused diff check | GREEN, 36 checks; diff clean |
| `f66e74` | registered source after typed membership-backend failure propagation | GREEN, 37 checks: prior 36 plus one fail-closed visibility source hostile; focused diff clean |
| `5ab5df` | registered shared `@semio-tech/framework-os-kernel:directory-event-page-contract-check` | GREEN, 14 clean cross-language contract checks |
| `5c6757` | registered final hub source plus focused packet diff check | GREEN, 37 checks; diff clean after process hidden-identity strengthening |
| `74beca` | focused `git diff --check` | GREEN, no whitespace errors in the packet files |
| `d8b71a` | read-only Rustfmt check | RED on existing broad repository formatting drift; it parsed the touched files and produced formatting diffs, not compiler evidence |

## Pending runtime evidence

Native and process phases are registered but not yet credited. At the current source boundary, multiple foreign WASI/Stdio Rust builds are active, including a two-hour release Stdio compiler, so no competing hub build was launched. PostgreSQL and Neo4j placement is source-proven only; their runtime services were not started. The real process harness is source-green but remains a nonclaim until the exact five-law binary, actual restarted SQLite hub journey, and final all-feature check produce terminal receipts. The deterministic post-read revocation and disconnect fence remains a native-law claim; the standalone process journey proves an already-revoked bearer because production exposes no test fence.
