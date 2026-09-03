# Local Credential Consumer and Browser Relay Audit

## Result

**First deterministic blocker — critical:** the protected bootstrap channel is owned solely by `os-hub`'s `DevScript`, but every current React, admin, native, and MCP launch is a sibling VS Code process rather than a supervised direct child.  `DevScript` starts the hub with only one `developer` profile (`native`, `mcp`) and then waits; it neither issues a credential nor launches a consumer.  Consequently the existing delivery code is a smoke-only helper, not a production/dev launch path.  The React and native fallbacks still try the deleted public `POST /auth/sessions` email-mint route.

This is a source-only audit.  I made **no source or test edits** and ran **no build, test, socket, browser, or runtime process**.  In particular, the presence of `secure-local-smoke` source is not evidence that the local bootstrap or any consumer works on this machine.

## Current evidence

| Lane | Current carrier and source anchor | Current effect | Severity |
|---|---|---|---|
| Hub bootstrap | `🌎️hub/📦️packages/🦀️rust/📜️script.ts:151-236` creates a private fd 3 duplex, random run key and profiles; `:518-540` starts it for `os-hub:dev`. `🌎️hub/🔐️local-bootstrap/🦀️.rs:16-25,260-308,501-541` validates bounded/HMAC frames, emits one session capability, and zeros its temporary string. | The hub requires this inherited endpoint in development (`📦️bin.rs:601-629,2237-2303`) and `/readyz` reports `local-bootstrap-pipe-v1` (`:560-580`).  The actual dev profile at `📜️script.ts:522-526` allows only `native` and `mcp`; no React/admin profile is configured. | Critical topology gap |
| Existing child seam | `📜️script.ts:308-370` can pass the already-issued envelope once on child fd 3, checks class, and clears the launcher copy.  `🌎️hub/📇️directory/🦀️.rs:699-720` declares `NativeCredentialEnvelopeDelivery`, `McpCredentialEnvelopeDelivery`, and `BrowserCredentialRelay`, but they have no concrete consumer implementation. | It is used only by `runSecureLocalSmoke` (`📜️script.ts:410-472`); neither the WGPU binary nor `semio-os-mcp` reads fd 3.  The transport is intentionally portable: Unix validates fd 3 and Windows duplicates its OS handle (`🔐️local-bootstrap/🦀️.rs:864-906`). | Critical missing consumer |
| Native OS | Native identity reads `S_HUB_URL`, `S_USER`, and `S_DATA_DIR` (`…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:313-319`), calls `mint_or_restore` (`:4018-4052`), and that helper falls through from `/auth/sessions/me` to `POST /auth/sessions {email}` (`…/📇️directory/🪪️identity/🦀️.rs:157-198`).  It writes the full identity including `session_token` to `S_DATA_DIR/os/🪪️identity.json` (`:23-38,95-119`). | The secure router exposes only `GET`/`DELETE /auth/sessions/me` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1724-1740,2074-2105`), so fresh native bootstrap fails rather than becoming authenticated.  The durable JSON cache also retains a bearer without an explicit permission/secret-store contract. | Critical auth failure; high at-rest disclosure |
| React shell | The Vite config bakes `S_HUB_URL`, `S_USER`, and `S_DATA_DIR` into browser code (`…/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts:157-167`). `ShellHost` reads them and either retains a persisted `Identity.sessionToken` or calls `mintSession(email)` (`…/ShellHost/🟦️.tsx:1288-1290,1512-1592`). The identity config itself persists `sessionToken` (`…/🎚️config/…/🪪️sign-in/🟦️.ts:8-39`; worker folder binding `…/🟦️backbone-worker.ts:278-305`). | A fresh browser session attempts the removed public mint endpoint. A recovered bearer is retained in browser-visible app state and forwarded to the worker, document bindings, and plugin persistence bindings (`ShellHost:3370-3390,5438-5444`). | Critical auth failure; critical bearer exposure |
| React sockets | `DirectoryClient` uses `Authorization: Bearer` for REST (`…/💻️os/🟦️.ts:3981-4013`) but puts that bearer in `/directory/ws?token=` (`:4082-4111`). The document worker sends the same bearer inside a `Hello` frame (`…/🟦️backbone-worker.ts:563-601`). | Both carriers are incompatible with a bearer-hidden browser.  The hub currently derives document auth from client-supplied Hello token and actor (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1033-1058`), so this also waits on the authenticated socket-grant migration. | High; socket-grant dependency |
| MCP | The actual CLI accepts a hub bearer in `--hub … --space … --token …` (`…/🌉️mcp/📦️bin.rs:1-120`), stores it in `HubOptions`/`WorkspaceOrigin`, and clones it into a hub persistence binding (`…/🌉️mcp/🦀️.rs:624-696`; `…/🏠️workspace/🦀️.rs:425-450,1216-1230`). HTTP mode additionally requires an MCP transport bearer on argv (`…/🌉️mcp/🦀️.rs:699-743`). | No fd-3 input parser exists.  Even though Debug redacts the hub token, argv and long-lived `String` ownership remain credential carriers.  The default `.vscode` MCP entries have no hub binding at all (`.vscode/launch.json:4381-4400`). | Critical direct-child gap; high argv disclosure |
| Admin browser | `AdminSessionProvider` asks a person to paste a `session.v1` bearer, then stores it in `sessionStorage` and adds it to every request (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:40-67,116-170,179-224`). The Vite server is only an unauthenticated proxy to the hub (`…/⚙️vite.config.ts:13-39`). `ConnectionsPage` then opens a tokenless directory socket (`…/🔴️ConnectionsPage/🟦️.tsx:49-76`). | Session storage is better than local storage but still makes the bearer available to browser JavaScript.  The live connection view cannot authenticate its directory stream.  There is no cookie/session endpoint: hub authorization parses only the `Authorization` header (`📦️bin.rs:403-405`). | Critical admin relay gap |
| Launch ownership | User launchers are generated from `.vscode/🧩️launch.seed.jsonc` by `…/🔌️plugin/📇️registry/🖥️launch.ts:1-24,178-224` and registry `📜️script.ts:1852,2480-2501`; the seed inserts `S_USER`/`S_HUB_URL`/`S_DATA_DIR` at `launch.seed.jsonc:5497-5530`. Generated entries are visible at `.vscode/launch.json:2502-2595`; hub/admin/MCP are independent tasks at `:4342-4400`. | Editing generated `launch.json` would be stale immediately.  The only legitimate owner is the seed/generator plus its existing `📜️script.ts` generation/check route. | High launch drift |

The hub deliberately has no public session mint route: `secure-local-smoke` asserts `POST /auth/sessions` is 404 (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:435-445`).  Current client documentation and tests still describe that route, so this is **production/client drift**, not a reason to restore arbitrary-email minting.

## Security and lifecycle implications

1. `S_USER` is an identity hint, not a credential.  Letting it reach a client that can mint an arbitrary bearer reintroduces the defect the secure-session foundation removed.  `S_*TOKEN`, `--token`, query parameters, `localStorage`, `sessionStorage`, `identity.json`, worker messages, log/error text, and plugin persistence bindings must cease carrying a hub bearer.
2. A browser cookie alone cannot authenticate the current hub: it only recognizes `Authorization`.  A local browser relay must be a bounded backend-for-frontend that keeps the session capability in its own memory and injects that header server-side.  It must not make the hub add ambient cookie authorization.
3. One local profile has one class-specific session and one relay/process lease.  An `admin-relay` session is still administrator only if its verified local subject is in `OS_HUB_ADMIN_SUBJECTS`; a client class, loopback address, browser port, or display name is never authority.  The current admin check correctly resolves a verified session subject (`📦️bin.rs:632-651`).
4. Session revocation is durable and live connections are separately kicked (`📦️bin.rs:1928-1943`); a new carrier must observe both.  Hub restart currently closes the bootstrap endpoint and causes the server to stop (`📦️bin.rs:2300-2325`), so every relay/child must discard its memory-only session and acquire a new envelope through a new supervisor run, never silently reuse old state.
5. Do not rely on CORS as an authentication boundary.  The hub reflects a supplied origin and permits credentials/authorization (`📦️bin.rs:1742-1766`).  The relay must instead expose only loopback, exact-origin, allowlisted BFF paths and reject any origin/host mismatch before proxying.

## Recommended bounded design

### Contract A — direct-child consumer input

Replace the smoke-only raw envelope handoff with a schema-first `semio.local.consumer-credential/v1` frame on inherited fd 3.  It is an internal direct-child contract, not an HTTP API:

```json
{
  "schema": "semio.local.consumer-credential/v1",
  "kind": "session",
  "runId": "32-lower-hex",
  "launchId": "32-lower-hex",
  "audience": "native | mcp | react-relay | admin-relay",
  "hubOrigin": "http://127.0.0.1:<ephemeral-port>",
  "sessionId": "opaque-id",
  "authorizationGeneration": 1,
  "expiresAtMs": 0,
  "capability": "session.v1…"
}
```

The supervisor validates the existing HMAC-protected hub envelope first, then emits exactly one bounded consumer frame to a directly spawned child.  The child must not treat the envelope's HMAC as locally verifiable—it does not possess the bootstrap channel key—and instead relies on the inherited OS handle plus a direct-parent process relationship.  Add `hubOrigin` to the authenticated upstream issuance/consumer contract rather than reading it from a child environment, so a poisoned `S_HUB_URL` cannot redirect the bearer to another server.

Use the existing limits as the hard upper bounds: one 16 KiB frame, 15 s exchange deadline, eight concurrent exchanges, 64 replay slots, and the existing 15-minute local session ceiling (`🌎️hub/🔐️local-bootstrap/🦀️.rs:19-25,829-861`).  Reject wrong `runId`, audience, expiry, duplicate launch id, non-loopback origin, duplicate or trailing frames, EOF-before-frame, and cancellation.  Zero source buffers after parsing; store the capability in a private non-cloneable secret holder with a zeroing `Drop`, and only lend it to HTTP-header construction.

The supervisor retains only `(launchId, sessionId, audience, child handle)`, never the capability, and gains a signed bootstrap-channel `release` message that revokes by `sessionId`.  It releases on child exit, relay shutdown, explicit sign-out, failed child acknowledgment, deadline, and supervisor signal.  The consumer sends an acknowledgment and a non-secret terminal reason over the inherited duplex; the launcher never logs frame payloads.  A crash is bounded by the session TTL, while a clean terminate/revoke is immediate.  This fills the current gap where bootstrap revokes only delivery failure (`🔐️local-bootstrap/🦀️.rs:644-680`) but has no child-lifetime release.

Native takes Contract A and constructs its `DirectoryClient` from the envelope-origin/capability.  Remove `IdentityEnv::from_process_env`, `mint_or_restore`, and the `identity.json` bearer cache from the authenticated native route; retain only non-secret profile/display metadata if needed.  MCP adds a `--local-credential-fd 3` mode that is mutually exclusive with `--hub/--token`, reads Contract A before any JSON-RPC output, derives its hub workspace/principal from `/auth/sessions/me`, and keeps its existing stdin/stdout untouched.  Do not put a credential in argv, environment, audit text, bridge-token file, or MCP protocol messages.

### Contract B — per-profile browser/admin BFF relay

Start one direct-child `LocalBrowserRelayV1` for each `(runId, profileId, audience, browser-origin)`.  It receives Contract A as `react-relay` or `admin-relay`; only the relay retains the bearer, in memory, until release.  It binds one fresh IPv4 loopback port (`127.0.0.1`) and binds a single canonical UI origin such as `http://127.0.0.1:<vite-port>`.  The existing allocator already uses that family (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:125-131`), which avoids Windows/macOS/Linux hostname-resolution divergence.  Do not bind `0.0.0.0`, reuse ports across profiles, accept `localhost` aliases, or persist endpoint/key files.

The UI server's existing Vite proxy forwards only an allowlisted `/hub/*` BFF path (and the later socket-grant path) to that relay.  The Vite process knows only relay URL and a public profile label; the relay knows the hub bearer; browser code knows neither.  This permits retaining Vite/HMR without turning it into a bearer holder.  Built admin must likewise be served through its profile relay/BFF in local-secure mode, not through direct hub `/admin` requests.

Bootstrap exchange:

1. Relay generates a 256-bit single-use nonce, stores its hash in a fixed-capacity in-memory table with exact UI origin, audience, profile, expiry (at most 60 s), and no bearer.
2. The supervisor prints/opens the UI URL with the nonce **only in the fragment**: `http://127.0.0.1:<ui-port>/#local-bootstrap=<nonce>`.  The small bootstrap module reads it, immediately calls `history.replaceState`, then `POST`s only the nonce to `/_semio/local-auth/redeem` through Vite to the relay.  A fragment is never sent in HTTP requests, referrers, or server logs.
3. Relay requires exact `Host`, exact `Origin`, loopback peer, `Sec-Fetch-Site: same-origin`, POST JSON size bound, and an unconsumed/non-expired nonce before it marks the nonce consumed.  It replies `204` with a random opaque host-only cookie (`HttpOnly; SameSite=Strict; Path=/; Max-Age` at or below session expiry; no `Domain`).  Do not use `Secure` on cleartext loopback unless the development topology supplies HTTPS; instead retain the exact-origin/loopback checks and never allow a non-loopback bind.  Clear the fragment and cookie on every refusal.
4. Each BFF request must carry that opaque cookie, match its in-memory profile/audience/generation, be same-origin, and match a finite route/method/body-size table.  Relay injects `Authorization: Bearer` only for the upstream request, strips authorization/cookie/set-cookie and sensitive redirects from either direction, applies deadline/cancellation, and returns redacted errors.  It never implements a generic URL proxy, blob/hash probe, raw descriptor locator, or cache that survives relay process lifetime.
5. Logout removes the relay mapping/cookie, uses its private bearer to call `DELETE /auth/sessions/me` where possible, and tells the supervisor to release.  Hub `401`, revocation generation change, kick, restart, deadline, or upstream unavailability clears the mapping and sends a terminal signed-out/restart state; reconnect must begin a fresh bootstrap exchange.

React changes from `VITE_S_*` identity bootstrap to an anonymous BFF `GET /_semio/local-auth/me` that returns only user/display/session-expiry metadata.  Its `Identity` and worker/document `PersistenceBinding` no longer contain `sessionToken`; direct `/directory` and direct hub document endpoints are forbidden.  Admin removes its paste-token form and `sessionStorage` key, makes `AdminClient` BFF-cookie based, and opens its directory observation only after the relay provides the later socket grant.  It must retain EN/DE `role=status` progress/error text and accessible Cancel/Sign-out controls; add corresponding keys in `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx` rather than embedding English strings.  Native and React should expose the same non-secret states: connecting, waiting for local authorization, ready, cancelled, revoked, expired, restart required, and unavailable.

### WebSocket boundary

The relay packet can land before an authoritative open plan, but **not** before the authenticated socket-grant carrier for real directory/document live traffic.  REST BFF migration is independent.  Once `socket.v1` grants exist, relay requests an exact-audience, exact channel/scope, one-use, short-lived grant on behalf of its cookie session; browser gets only that grant and sends it in the credential-free first frame.  It must not put it in a query string and must not send a bearer or caller actor.  The hub derives user/role/actor/session/generation and closes/rejects on grant/session revocation.  This replaces the token query and Hello token/actor shown above.

Open-plan/catalog/P2-C then layer on top of the already authenticated BFF/session.  P2-D chunk CAS and final catalog loading do not block Contract A/B; they do block later full document/materialization verification.  No relay may claim a document is open or a catalog is trusted before those authorities respond.

## Dependency-ordered implementation packets

| Order | Bounded packet and owned files | Completion rule |
|---|---|---|
| P0 | **Local supervisor topology.** Extend `🌎️hub/📦️packages/🦀️rust/📜️script.ts` to keep the only bootstrap channel and supervise named direct children; add source-owned profile/class plans for `native`, `mcp`, `react-relay`, `admin-relay`.  Add authenticated release/ack semantics in `🌎️hub/🔐️local-bootstrap/🦀️.rs` and the directory port/schema in `🌎️hub/📇️directory/🦀️.rs`. | `os-hub:dev` secure mode can start hub plus two isolated profiles without `S_USER`, a static token, or a public mint route.  It waits for readiness and tears down every child/session deterministically. |
| P1 | **Native and MCP envelope consumers.** Add fd-3 reader/secret ownership to `…/📇️directory/🪪️identity/🦀️.rs`, WGPU shell target, MCP `📦️bin.rs`, `🦀️.rs`, and hub launch script.  Delete authenticated `S_USER` mint and bearer persistence paths rather than adapting them. | Native and MCP obtain `me`, reconnect, sign out, revoke, and restart only via Contract A; their command lines, environment, disk cache, and JSON-RPC stream do not contain the hub bearer. |
| P2 | **Relay service and React BFF migration.** Put relay lifecycle in the existing hub `📜️script.ts` (no extra launcher script), use the existing OS dev `📜️script.ts`/`⚙️vite.config.ts` only for non-secret proxy coordinates, and change `ShellHost`, `🟦️backbone-worker.ts`, and `💻️os/🟦️.ts` to BFF metadata/no-token bindings. | Two React profiles receive distinct HttpOnly opaque cookies from fragment nonce redemption; no browser-visible bearer remains after a full reload/logout/restart. |
| P3 | **Admin relay migration.** Change admin `⚙️vite.config.ts`, `📜️script.ts`, `AdminSession`, `ConnectionsPage`, and EN/DE i18n.  Built admin local launch must use its per-profile relay as well. | No paste-bearer form or `sessionStorage` bearer remains; administrator policy is verified upstream and revocation/kick returns an accessible translated state. |
| P4 | **Generated launch registration.** Update only `.vscode/🧩️launch.seed.jsonc`, `…/🔌️plugin/📇️registry/🖥️launch.ts`, and its owning `📜️script.ts`; regenerate/check `.vscode/launch.json` as output.  Replace the separate user/hub/admin/MCP secure entries with one supervised local-secure compound/suite. | Each registered secure launcher is a child plan, uses profile-specific loopback/UI port, prints a nonce fragment only after readiness, and stop-all releases sessions. |
| P5 | **Socket grant client migration.** Land the separate authenticated socket-grant backend first; then change directory and document socket clients/relay to first-frame grants. | No token query, Hello bearer, or caller-selected actor remains.  Revocation/kick/restart causes terminal close and bounded rebootstrap. |
| P6 | **Open-plan/P4 pair/P2-C work.** Bind the authenticated carrier to server-selected descriptor/catalog/app/surface and later P2-C recovery. | Browser/native never choose authoritative plugin/package/schema identities; document access continues to fail closed on stale/revoked scope. |

## Required fixtures and independent oracles

Add neutral JSON fixtures for Contract A and Contract B covering the exact valid case plus wrong HMAC before forwarding, wrong class, wrong run/profile/origin, expired/replayed nonce, duplicated cookie, cross-profile cookie, oversized body/frame, fd EOF, child abort, relay crash, hub restart, session expiry, durable revoke, live kick, and absent public mint.  Fixtures contain fake token-shaped bytes only and assert all diagnostics redact them.

The independent oracle must be a real Chromium/Playwright two-profile process, not a mocked `fetch`/`WebSocket`: it observes that no bearer appears in browser `localStorage`, `sessionStorage`, IndexedDB/config documents, DOM, page URL, request URL, console, Vite environment bundle, or rendered error; it independently checks the cookie is HttpOnly/SameSite/host-only, a second nonce redemption fails, cross-port/profile reuse fails, and an upstream request still reaches the hub with the authorized subject.  A separate raw fd-3 child oracle (Node/Bun is sufficient) must prove native and MCP receive one bounded envelope while MCP stdin/stdout stays byte-identical JSON-RPC.  The existing secure bootstrap fixture/TS test (`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:53-84`) is useful input but does not replace either oracle.

Focused commands to run only after implementation (none were run for this audit):

```sh
bun nx run os-hub:secure-local-smoke
bun nx run @semio-tech/framework-os-mcp-rs:test quick
bun nx run os-hub-admin:test
bun nx run @semio-tech/framework-os-dev:collab-e2e
```

The last command must be extended to launch the supervised secure suite and Playwright profiles rather than injecting `S_USER`; its current helper still spawns Vite with `S_USER` (`…/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:2853-2875`).  Do not call the existing broad hub or Cargo suites as proof of this packet until their runtime assertions include the new carrier laws.

## Blocker ordering

1. **Critical — no supervised direct-child topology:** the parent of fd 3 launches no native/MCP/relay consumers and the registered user/admin/MCP tasks are siblings.
2. **Critical — stale arbitrary-email client bootstrap:** native and React still rely on `S_USER` plus deleted `POST /auth/sessions`; existing stored identities retain bearer strings.
3. **Critical — browser/admin bearer ownership:** React passes bearer through state/worker/bindings; admin prompts/stores one; neither has a cookie/BFF relay.
4. **High — MCP argv and lifetime ownership:** hub bearer is a CLI token and no inherited-envelope reader exists.
5. **High — live socket carrier:** query bearer and Hello token/actor must wait for `socket.v1` grants/server-derived identity.
6. **Medium — launch generator and UX:** move the secure flow into the seed/generator-owned suite, add EN/DE accessible status/cancel/logout, and remove stale per-user environment launchers.

