# Terra Audit — Live Hub Administration, Bilingual and Authenticated

**Scope.** Read-only source audit on 2026-09-03 after the session redesign work. No build, browser, database, Docker service, or runtime launch was run. “Fails” below means the current source’s deterministic control flow reaches the stated rejection; it is not a runtime claim.

## Verdict

The delivered admin SPA is a useful mock-backed dashboard, not a complete live administrator journey. Its auth copy/form still describes a static `OS_HUB_ADMIN_TOKEN` and loopback privilege, whereas the hub now accepts only a valid durable `SessionCapability` whose verified identity matches `OS_HUB_ADMIN_SUBJECTS`. The registered hub launch cannot pass its own fail-closed validation: it defaults to `0.0.0.0` (production), installs neither verifier nor protected local bootstrap, and consequently rejects startup before serving `/admin`.

Even with a test-issued, correctly authorized session, **New space** calls `/admin/api/commands`; that handler constructs `DirectoryActorKind::Admin`, while the decider requires a `User` actor for `create-space`. The action therefore returns a backend error. The SPA also has no authenticated admin connection stream: it constructs a tokenless member `DirectoryClient`, which cannot pass the current directory WebSocket's session check or its membership privacy filter. Its initial connection snapshot is not a durable live view.

The smallest sound repair is not an admin-token compatibility layer. It is a schema-first, session-derived `AdminPrincipal`/operation boundary; server commands derive the real operator identity, durable/event operations go through the directory service, credential operations retain their separate digest-only records plus audit, and live connection kick remains explicitly ephemeral. The admin UI must consume a secure admin session, not a manually pasted static secret.

## Current route and operation census

| Journey capability | SPA path and behaviour | Hub/projection status | Finding |
|---|---|---|---|
| Admin sign-in | `AdminSessionProvider` stores a manually typed bearer in `sessionStorage` and probes `/admin/api/overview` (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:40-170`); its UI names `OS_HUB_ADMIN_TOKEN` (`:184-224`). | `is_admin` parses a `SessionCapability`, authenticates its durable session and matches provider+subject digest against `OS_HUB_ADMIN_SUBJECTS` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:485-536`). | **Critical stale contract.** There is no admin static-token or loopback policy in the current hub. |
| Auth/session issuance | No SPA flow calls an assertion verifier or protected local bootstrap. | Session record is digest-only, expiry/revocation/generation-bound (`🌎️hub/📇️directory/🦀️.rs:156-199`); verifier and local-bootstrap interfaces exist (`:532-610`). `main` sets both adapters to `None` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2116-2124`). | No production/local-dev issuance path is wired into the real process. Existing `POST /auth/sessions` client comments are stale (`🧰️framework/🛍️products/💻️os/🟦️.ts:4015-4024`). |
| SPA serving/routes | `/admin` and `/admin/{*path}` safely serve static assets/fallback, or 503 when build output is absent (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1918-1978,2004-2005`). Vite proxies hub routes (`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts:13-44`). | UI uses one local `tab` state, not URL routing (`AdminApp:20-96`); direct `/admin/spaces` falls back to the same overview tab. | Deep links/history/refresh do not represent an admin operation or page selection. |
| Users/provisioning | Users page lists users and exposes one “Revoke sessions” button (`🙋️UsersPage:15-45`). | User storage permits password hash/SSO fields (`directory:64-76`), but there is no password credential/admin provisioning REST route. Member upsert creates a projection user by email as a domain event (`directory:1132-1156`). | Do not make email entry a credential issuer. Provisioning must come from verified identity/local bootstrap; current label understates durable revocation. |
| Session revoke vs connection kick | User-page documentation says revoke “kicks every LIVE connection” and cannot handle a login session without a document WS (`🙋️UsersPage:15-17`). Connections page offers “Kick” (`🔴️ConnectionsPage:35-124`). | User revoke first durably increments/revokes all matching auth sessions, then signals matching live sessions (`bin.rs:1840-1855`). Kick only signals one in-memory `Notify` and has no credential effect (`:1827-1838`). | **High misleading control semantics.** These must be two separate labelled commands, confirmations, audit records, and outcome states. |
| Space creation | Dialog sends `create-space` through generic `AdminClient.command` (`🏛️SpacesPage:19-89,283-300`). | `admin_commands` always emits actor `{kind: Admin,id:"admin"}` (`bin.rs:1800-1816`), but `create-space` calls `actor_user_id`, which rejects non-user actors (`directory:1069-1107`). | **High deterministic action failure.** It maps to a backend error, discarded by the UI promise chain. |
| Space/member lifecycle | Rename, visibility, archive/delete, upsert/remove member use generic directory commands (`🏛️SpacesPage:92-299`). | Events/projections enforce owner, atelier, and archive laws (`directory:1080-1164`). Admin route bypasses `authorize_directory_command` and uses anonymous static actor `admin` (`bin.rs:1800-1816`). | Domain event source is present, but actor attribution is lost; optimistic UI does not show typed failure, progress, stale revision or cancellation. Native `window.prompt`/`confirm` controls are not adequate operational dialogs. |
| Invites | A detail panel can issue a fixed seven-day invite, display/copy its once-returned token (`🏛️SpacesPage:152-180`), but does not render existing `detail.invites` nor revoke one. | Invite issue/revoke intentionally persists capability state outside domain events and audits it; redemption is event-sourced (`directory:1080-1085,1165-1174,1460-1467`). Admin detail can read invite metadata (`bin.rs:1729-1738`). | Incomplete lifecycle and no copy success/failure announcement. The raw secret must remain one-display only; list views may expose only selector/id, role, expiry and revocation state. |
| Document shares | No `AdminClient` share method, no tab, no document-detail operation. | Admin-gated issue/revoke routes exist outside `/admin/api`, return a raw one-time token on issuance (`bin.rs:585-611,2006-2010`); durable record is exact-document, digest-only and revocable (`directory:50-62`). | **High missing administrator capability.** Do not fold shares into invites: scope and authority are different. |
| Documents/checkpoints/retention | Documents page is a flat descriptor/frontier/connection-count table (`📄️DocumentsPage:17-84`). No checkpoint, lineage, retention, pair, share or rebootstrap controls/views exist. | Backends project active/lineage/retention and enforce 16,384 lineage limit (`directory:279-324,1438-1449`). Retention is a server-only `ArtifactDirectoryCommand`, validates active lineage/floor and emits `artifact.retention.advanced` (`directory:1189-1228`). No hub admin route exposes checkpoint/lineage/retention. | **High operational gap.** Retention must never be a client state mutation or a raw blob-delete button. |
| Connections/presence | Snapshot from `/admin/api/connections`, then tokenless `new DirectoryClient(window.location.origin).stream(...)` (`🔴️ConnectionsPage:43-76`). It groups by surface purely for display (`:16-32`). | Directory WS requires bearer session in query today (`bin.rs:1514-1542`) and filters live frames to current space members (`:1481-1495`). Admin is not automatically a member; stream will not deliver an all-space admin view. Live connection index is durable, but a failed record write is swallowed, leaving an unkickable socket (`bin.rs:1082-1103`). | **High stale/partial live view.** Surface may be telemetry grouping only; document-wide presence/authority must not be filtered or granted by surface. |
| Connection kick | One `POST /admin/api/connections/{id}/close`, no reason, confirmation, progress or outcome (`AdminSession:105-111`; `bin.rs:1827-1838`). | Notify is process-local; restart clears it and startup marks crash residue closed (`bin.rs:2133-2138`). | Correctly ephemeral in concept, but unaudited and not guaranteed if sync-session persistence failed. It must not be labelled session revocation. |
| Directory/audit events | Events tab lists only `DirectoryEvent` body/actor/time (`📰️EventsPage:15-57`). | `/admin/api/events` reads domain events (`bin.rs:1787-1798`); all three directory backends store bounded `AuthAuditRecord` for session/share/invite lifecycle (`directory:250-264,1451-1457`; SQLite insertion/query `🪶️sqlite/🦀️.rs:236-242,742-751`). No admin route exposes `list_auth_audit`. | The audit page omits credential/revocation/share/invite audit and records admin domain commands as static `admin`, while user-session revocation passes `actor_user_id: None` (`bin.rs:1842-1848`). |
| Rebuild projection | Overview calls `POST /admin/api/directory/rebuild`; message is local state only (`🏠️OverviewPage:37-89`). | Rebuild mutates a derived projection from the log and has a bounded/cancellable control seam (`directory:279-324`), but endpoint invokes uncontrolled rebuild with no operation id, cancel, progress, exclusive lock/error detail or audit (`bin.rs:1819-1825`). | An operational rebuild is valid CQRS projection maintenance, not a CRUD domain edit, but its present request is unobservable and unsafe for a live admin UI. |

## Durable versus ephemeral authority

| Operation | Correct lifetime | Required invariant |
|---|---|---|
| Session revoke | Durable credential state: revoked timestamp plus authorization-generation invalidation. | It survives restart; no later revalidation may accept the capability. Revocation remains successful even if immediate socket kick fails. |
| Connection kick | Ephemeral process-local close signal for one recorded sync session. | It terminates only that current connection and does **not** revoke a credential. On restart the connection index is closed as residue. |
| Member/space/document/retention | Event-sourced domain decision and projection. | Server derives operator and authorization, serializes decision, appends immutable event(s), then reads projection; no client-side mutation. |
| Invite/share capability | Durable digest-only capability record plus append-only credential audit; raw secret returned once. | Never write raw bytes into directory events, SPA state beyond one copy panel, URL history, logs, or audit. Share is document-read-only; invite grants a space membership only on redemption. |
| Checkpoint and retention | Verified system publication and server-only authority command. | Private locators remain out of directory DTO/events (`directory:795-855`). Human policy input may request an advance, but the server verifies active lineage, monotonic floor and reference safety before emitting the event. |

Current checkpoint and retention projections are database-backed across the directory backend trait (`directory:1403-1449`), as are sessions/invites/audit. The admin SPA has no corresponding read contract. The `DirectoryClient` stream is not an administrator oracle, because its visibility policy deliberately requires membership.

## Accessibility and language findings

The existing UI has useful foundations: native form labels, semantic tabs, UI-kit dialogs, and tests that check dialog focus/`aria-labelledby` and locale select focus (`🧪️admin.test.tsx:135-165,208-231`). They do not establish a complete accessible operational journey.

- **No-default-language contradiction (high):** I18n claims no default, but `detectAdminLocale()` returns English for non-browser/unsupported locales and `useAdminT()` falls back to English then raw key (`📚️I18n:12-16,249-280`). Tests explicitly expect `EN` initially (`🧪️admin.test.tsx:209-230`). Use an unset locale state with a bilingual, accessible language-choice gate; do not silently choose EN. Both locale catalogs must be complete.
- **Stale/missing keys (high):** existing EN/DE keys cover only nav, session, overview, spaces, users, connections, documents and events (`📚️I18n:18-238`). The messages still describe admin bearer tokens. There are no keys for assertion/bootstrap status, session revoke versus kick, invite/share lifecycle, checkpoint/retention, operation progress/cancel, audit source/outcome, stale revision, typed denied/error, or copy success/failure.
- **Announcements and failure UX (high):** every page commonly converts request rejection to empty data (`DocumentsPage:30-50`, `SpacesPage:198-227`, `UsersPage:21-27`) and actions chain `.then(...)` without `.catch`. Rebuild's result is a plain `span`, not a live region (`OverviewPage:52-87`). Operators cannot distinguish empty, forbidden, stale, offline, cancelled, or failed.
- **Destructive/row controls (medium):** Rename/archive/delete use browser prompt/confirm (`SpacesPage:240-277`), and repeated Remove/Kick/Revoke buttons have no row-specific accessible description (`SpacesPage:100-115`, `UsersPage:29-37`, `ConnectionsPage:107-114`). Use accessible confirm dialogs naming the exact scope/user/session and a server-issued impact preview, retain focus, and announce completion/failure.
- **Locale formatting/telemetry (medium):** users/events call ambient `toLocaleString()` instead of selected Admin locale (`UsersPage:29-37`, `EventsPage:42-47`). Connection `live/offline` is text without a status/live semantic and no recovery explanation (`ConnectionsPage:81-124`).
- **Secret-copy path (medium):** `navigator.clipboard?.writeText` ignores completion/rejection and `inviteCopied` is unused (`SpacesPage:165-178`; i18n keys `:69-72`). Do not leave the secret visible after panel close/reload; add keyboard-confirmable copy, success/failure live announcements and expiry information.

## First deterministic blocker ordering

1. **Current registered hub launch cannot start.** `.vscode/launch.json:4342-4359` sets port/data but no bind; `main` defaults `OS_HUB_BIND` to `0.0.0.0`, selects production, has `identity_verifier=None`, and `validate_auth_startup` rejects it (`bin.rs:496-524,2116-2124`).
2. **Loopback development is also unavailable until bootstrap wiring lands.** Explicit loopback changes mode to development, but `local_bootstrap=None` is separately rejected (`bin.rs:515-521,2121-2124`).
3. **The SPA describes a removed credential model and cannot establish an admin session.** It asks for `OS_HUB_ADMIN_TOKEN`; the server accepts verified identity-bound session capability only. The verifier/bootstrap interfaces are not exposed/installed in the process.
4. **With a test-issued valid admin session, “New space” still fails.** SPA sends `create-space` via `/admin/api/commands`; handler uses `Admin` actor while `decide` requires a user actor.
5. **With a valid snapshot, live connections remain stale.** The Connections page creates a tokenless, member-filtered directory stream rather than an authenticated administrator stream.
6. **Document/share/checkpoint/retention/audit operations have no SPA/API contract.** The durable backend seams exist but are unreachable; retention must wait for the P2-D CAS/reference-retention implementation before a destructive cleanup control can exist.

## Bounded schema-first implementation packet

### 1. Secure runtime and admin principal — prerequisite

Install a real `IdentityAssertionVerifier` production adapter and a protected, non-network-exposed `LocalBootstrapTransport` development adapter. Register them in `main`; make the hub bind loopback in development and fail closed otherwise. Replace `is_admin -> bool` with one fallible `authenticate_admin` that returns:

```text
AdminPrincipalV1 {
  userId, authSessionId, identityProvider, identitySubjectDigest,
  authorizationGeneration, issuedAt, expiresAt, correlationId
}
```

It must revalidate the durable session on every REST operation and first framed admin WebSocket control. It must never be reconstructed from email, request body, loopback peer, client actor, or static token. `OS_HUB_ADMIN_SUBJECTS` remains identity policy, but the principal retains the matched *session user* for event/audit attribution. A browser admin session derives from the secure assertion/local bootstrap flow; delete the static-token form, wording, storage key and old `DirectoryClient.mintSession` assumptions rather than keeping a compatibility path.

### 2. Admin query/command schemas and audit model

Define strict shared Rust/TS codecs (new directory/admin schema tree), with 8 KiB request and 64 KiB page response caps, opaque IDs, cursor pagination <=100 rows, and fixed 10 s query/30 s operation deadlines:

```text
AdminQueryV1 = Overview | Spaces {cursor} | SpaceDetail {spaceId} |
  Users {cursor} | Documents {spaceId?, cursor} |
  DocumentDetail {scope, lineageLimit<=100} |
  AuthAudit {cursor, limit<=100} | DomainAudit {cursor, limit<=100}

AdminIntentV1 = CreateOwnSpace | RenameSpace | SetSpaceVisibility |
  ArchiveSpace | DeleteSpace | UpsertMember | RemoveMember |
  IssueInvite | RevokeInvite | IssueDocumentShare | RevokeDocumentShare |
  RevokeUserSessions | KickConnection |
  AdvanceRetention {scope, expectedActiveCheckpointId, policyReasonCode} |
  RebuildProjections {confirmationNonce}

AdminOperationV1 { id, intentDigest, state: accepted|running|succeeded|failed|cancelled,
  progress?: {completed,total,labelKey}, resultRef?, typedError?, correlationId }
```

An `AdminIntent` carries an expected target revision/active checkpoint when state could have moved. The server derives `AdminPrincipal`, validates scope and role, serializes through the appropriate service, and returns an operation/reference—not a mutable client model. For `CreateOwnSpace`, derive a **User** event actor from the authenticated administrator's actual user id while recording the admin principal in the operation/audit; do not use the static actor `admin`. If creation on behalf of another user is desired, introduce a distinct server-decided ownership intent and event invariant; do not accept arbitrary owner/email identity.

Keep domain events and capability records deliberately distinct. Add an append-only `AdminAuditRecord` for every accepted/denied/cancelled admin operation, plus connection kick and projection rebuild, with principal user/session, target-kind/id, reason/outcome/correlation and no capability bytes/private locator. Continue `AuthAuditRecord` for issue/revoke of session/share/invite, but pass the derived administrator user as actor (current revoke passes `None`). The Events UI becomes a cursor-paged unified view with domain versus credential/operational source labels and redacted fields.

### 3. Server surfaces and projections

Replace generic `/admin/api/commands` with typed admin intent route(s), and add read-only document detail, checkpoint lineage/active retention, share metadata and auth-audit queries. It is valid for admin data to be projected from the durable directory/event/capability stores; it is not valid for it to reach private blob locators.

Expose an administrator-only live endpoint such as `/admin/api/connections/ws`, using the same first-frame secure session control planned for directory/open-plan work—never a query capability. The hub authenticates `AdminPrincipal` and streams bounded `ConnectionOpened/Closed` records; subscriber visibility is administrator policy, not membership policy. Record a live connection before admitting it, or fail the document mount closed: swallowing the recording error currently creates an unobservable/unkickable connection. Kick returns an ephemeral result and emits `AdminAuditRecord`; durable revoke first changes session generation then requests matching kicks, reporting separately `revokedCount` and `connectionsSignalledCount`.

Keep retention server authority-only: the admin intent is a reasoned request, the server reads current active lineage, enforces the monotonic floor exactly as `decide_artifact_authority` already does, writes `ArtifactRetentionAdvanced`, then schedules P2-D's reference-aware sweep. Never expose a “delete checkpoint/blob” operation before chunk-CAS reference accounting/retention races are solved.

### 4. SPA and bilingual semantic UI

Refactor `AdminSessionProvider` around a secure session credential provider and `AdminPrincipal` summary; no manual secret input. Add URL-backed pages for Overview, Spaces, Users, Connections, Documents, Checkpoints/Retention, Shares/Invites, and Audit. The operator chooses a language before entering the application when browser preference does not exactly select an available locale; formats use that selection with `Intl.DateTimeFormat`.

Each operation uses server status/typed error, cancellation where applicable, focus-safe confirmation for destructive/revoke actions, `role="status"`/appropriate live region, labelled progress, keyboard-operable controls, and row-specific accessible names (for example, “Revoke sessions for Ada”). “Revoke all sessions” says it invalidates credentials everywhere and then attempts to disconnect live sessions. “Kick connection” says it closes only this connection and leaves credentials valid. Shares/invites show raw capability only in a short-lived one-time reveal panel; lists never reconstruct it.

Add EN and DE keys in lockstep for auth/bootstrap, all operation names/states/error codes, scope-aware confirmations, checkpoint/retention/rebootstrap, share/invite privacy, audit source/outcome, progress/cancel, connection recovery and copy feedback. Do not let translation function fall back to English/raw keys in normal UI operation.

### 5. Operations and launch

Run rebuild under an exclusive bounded operation control using the existing projection control seam, publish monotonic progress and permit cancellation before each replay unit. Replace `dir_size`/full `list_users(i64::MAX)`/full-space scans on page load with bounded projections and cursors. Keep the admin SPA build prerequisite in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:5-47`; make startup report a localized, non-secret configuration diagnosis only to local operator logs/UI status.

Update the existing `🛠️dev🗄️os-hub` launch configuration after adapters exist: explicit loopback development mode, dedicated protected local-bootstrap configuration and isolated data directory. The current profile is not an E2E prerequisite. Retain `🛠️dev🗄️os-hub🛡️admin` only as the proxy UI profile after its auth/preflight wording is corrected. Add one ordered compound for hub + admin + two user clients after the production/open-plan lane is usable; the existing two-user compound (`.vscode/launch.json:7590-7596`) lacks admin and currently inherits the broken hub profile.

## Test and independent-oracle packet

1. **Neutral fixtures:** versioned JSON schema fixtures for every `AdminIntentV1`, `AdminOperationV1`, principal redaction, checkpoint/retention details, share/invite metadata and bilingual message key set. Include forged operator, stale revision, cross-space target, expired/revoked session, cancelled rebuild, lineage rollback, raw-capability/private-locator leakage, and kick-vs-revoke cases.
2. **Independent oracle:** validate the neutral schemas with AJV and recompute session/share/invite and checkpoint SHA-256 vectors with Node `crypto`; this mirrors existing independent capability/checkpoint tests in `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:56-75,156-202`. Rust/TS codec tests must consume the exact same fixtures.
3. **Backend transaction tests:** across SQLite, PostgreSQL and Neo4j, verify a principal-derived admin operation retains operator attribution; auth revoke survives restart and invalidates every capability; one kick closes only its recorded socket; connection recording failure rejects admission; share/revoke cannot cross document/space; retention cannot move backward or delete referenced chunk; rebuild progress/cancel leaves a valid old projection or atomically installed rebuilt one.
4. **Real two-user bilingual E2E:** start an explicitly configured loopback hub, authenticate a configured admin and two distinct verified users, create own space, add/revoke member, issue/revoke invite and share, create/open approved document, publish checkpoint, force rebootstrap, inspect/check retention state, observe two live document-wide roster connections, kick one then revoke the other user's credentials, and assert reconnect denial. Repeat the operator UI in EN and DE with keyboard-only actions, focus restoration, live status, translated dates, and no secret in DOM after reveal closure.
5. **Third-party browser/socket oracle:** use an actual browser automation driver plus an independent WebSocket client confined to tests, not mocked `fetch`/`FakeDirectoryWebSocket`. Current SPA tests mock both boundaries (`🧪️admin.test.tsx:2-5,47-99`) and hub tests call handlers/in-process spawned server (`📦️bin.rs:3301-3357`); neither proves the shipped launch, secure browser auth, proxy, or end-to-end semantics.

Focused commands to run **after** implementation and only when concurrent jobs permit (none were run for this audit):

```sh
bun nx run os-hub-admin:test
bun nx run os-hub:test-quick -- admin
bun nx run os-hub-ts:test-quick
```

Use the existing launch entries `🛠️dev🗄️os-hub` and `🛠️dev🗄️os-hub🛡️admin` only after their secure configuration repair. SQLite directory plus filesystem DB are the default zero-touch local substrates (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2040-2113`); PostgreSQL tests require a live Docker daemon according to the hub test script (`📜️script.ts:27-35`). No existing registered launch configuration can presently prove the bilingual two-user admin journey.

## Authority/privacy exit criteria

- Every admin API/stream action has a current verified admin session and exact principal; no static token, arbitrary email, loopback proximity, query capability or client actor grants authority.
- The server derives target scope, actor and authorization; all domain modifications are decisions/events, all credential records are digest-only durable state with audit, and all operational actions have attributable immutable audit.
- Session revoke and connection kick are visibly and mechanically different. A failed kick cannot undo durable revoke; an unrecorded connection is never admitted.
- Share is exact-document read-only; invite is space membership; neither exposes topology, raw secret after one reveal, private storage keys, or cross-space data.
- Checkpoint/retention UI is read-only until server authority and P2-D chunk-CAS retention permit a verified bounded advance/sweep.
- EN and DE have equal complete keys; unsupported/no preference requires explicit selection; all expensive operations expose cancellation/progress and accessible outcome semantics.

