# Scoped Directory Socket Membership Revocation

## Boundary

This packet implements the P0 document-scoped directory transport from `📓️terra-scoped-directory-socket-membership-revocation-blueprint.md`. It does not claim that the legacy global directory socket is document scoped, and it does not change D1 document-open authority.

The new transport owns one exact `DocumentScope`. Public discoverability does not authorize it. Issuance requires an authenticated live session, exact current space membership and an existing document descriptor. The grant, URL, ledger bindings and every delivered body all retain the same scope.

## Production changes

- `SocketAudienceV1::DirectoryScoped(DocumentScope)` is a distinct audience.
- Session records index `User`, `Session` and exact `(user, space)` membership bindings. Pending and live entries are removed from every index on invalidation.
- `POST /directory/spaces/{space}/documents/{document}/socket-grants` issues an empty-body, bounded, session-only scoped grant.
- `GET /directory/spaces/{space}/documents/{document}/socket/v1?since=…` consumes only a grant with an equal scope.
- Body matching is exhaustive. Exact document announcements, checkpoints, retention, connection, presence and rebootstrap controls may be delivered. Foreign scopes, membership events, raw heartbeats and unknown/unrelated bodies are skipped without cursor advancement.
- Authority failures are total: revoked or absent authority closes with 4401; unavailable authorization closes with 1013; unrelated messages emit nothing.
- REST and typed admin member removal share `execute_directory_command_fenced`. The membership gate remains owned across durable directory append, publication and ledger invalidation.
- Scoped send reauthorization owns the same membership gate through serialization and send. Deterministic test gates prove the only two orders: removal wins and no text escapes, or one already-admitted exact frame is sent before removal and 4401.
- The real route law retains an unrelated second user's private-space stream while removing the target member. It requires both the target's consumed and pending grants to fail afterward, denies reacquisition, and still delivers the unaffected user's exact document event.
- TypeScript and Rust directory clients preserve the close code. Scoped 4401 is a terminal `Revoked(scope)` outcome and never enters reconnect backoff.
- The browser backbone worker owns scoped streams by the full `(space, document)` runtime key. `ShellHost` opens the scoped owner only after the document socket is ready; a typed `directory-scope-revoked` retires that exact worker artifact owner without folding a raw removal event.

## Neutral contract

The schema and hostile corpus live under `🌎️hub/📇️directory/🧪️fixtures/🔌️scoped-socket-revocation-v1`. The Bun oracle uses AJV 2020 plus an independent decision function. It covers 19 delivery/denial/order vectors, three hostile schema mutations and three client-close outcomes.

## Permanent gates

- `os-hub:scoped-directory-socket-source-check`: neutral AJV/oracle, exact relay admission checks, and three focused TypeScript wire/client/worker ownership laws.
- `os-hub:scoped-directory-socket-native-check`: exact Rust client laws through the shared one-build exact-law runner.
- `os-hub:scoped-directory-socket-process-check`: exact hub ledger, message, real route, admin and two-order process laws plus the all-feature hub check.

All three commands are implemented in the existing hub `📜️script.ts`, registered in `📋️project.json`, and represented in `🧩️launch.seed.jsonc` with one ticket-local artifact root, one ticket-local Cargo target and `CARGO_BUILD_JOBS=1` for native work.

## Evidence

- Session `77268`: final current-source canonical registered source gate, exit 0. AJV/oracle `19/19`, hostile schema cases `3/3`, client-close cases `3/3`, exact relay admission/rejection cases `3/3`; Vitest `3/3` passed with `237` unrelated tests skipped by the exact selector. The laws cover the discriminated worker wire, shared client terminal 4401 behavior and the real backbone-worker full-scope owner.
- Session `95801`: earlier canonical registered source gate, exit 0. AJV/oracle `19/19`, hostile schema cases `3/3`, client-close cases `3/3`; Vitest `2/2` passed with `237` unrelated tests skipped. It predates the final worker-wire and relay-admission assertions and is superseded by `77268`.
- Session `67304`: superseded red. The first worker-owner test sampled the fake socket before its bounded promise turn created it. Bounded microtask polling repaired only the test scheduling assumption; session `83246` proved that law directly before the full `95801` gate passed.
- Sessions `30980` and `92956`: plugin-registry generation and registered freshness both exited 0. The three launch entries are present in the generated `.vscode/launch.json`.
- Session `27116`: the broad React-renderer typecheck is environment/source red on existing demonstrator, replication, tutorial, plugin, Flow and type-generation diagnostics. It emitted no diagnostic in the scoped `ShellHost` owner or new worker wire variants and is not counted as acceptance.
- `rustfmt --check` parsed the changed Rust sources; its output contains formatting differences in the concurrently edited hub file and is not compile or runtime evidence.
- Native and process evidence remains pending. Current hub compilation stops before the owned laws in the concurrently moved Stdio crate: its line 6188 still references `✳️brep/.../io/🦀️.rs` while the physical BRep owner is under `🧊️brep`. That taxonomy-owned pre-test failure is not patched or counted here. Membership revocation is therefore source/browser-qualified only until the registered native and real loopback ordering gates reach terminal green from stable current source.

## Nonclaims

- The legacy `/directory/socket/v1` audience remains global and is not advertised as scoped.
- No raw `DirectoryEvent` membership-removal frame is delivered to the removed client.
- No public/nonmember authority, share-token authority, D1 open authority or compatibility route is added.
- No native/process acceptance is inferred from the neutral or TypeScript evidence.
